//! 🎭️ The quiz lifecycle as framework actors: one handle actor per handle key (it registers a
//! pseudonym or name exactly once) and one learner actor per learner (its registration, runs,
//! answers, submissions and badges). Both wrap the quiz core's pure `decide_*`/`evolve_*` without
//! adding domain rules; the actor state bytes are the serde JSON of the core's
//! `HandleState`/`LearnerState`.
//!
//! **No roster.** A handle key is its own stream (`quiz-handle/<hex of the key's UTF-8 bytes>`), so
//! claiming a handle is exactly-once by the turn of that one small actor: its state is the holder, a
//! lookup is one indexed read, and nothing deserializes a map of every handle. A claimed handle is
//! *recalled* by the `quiz.handle` read (`🔭️projections`), which writes nothing — a recall appends
//! no event anywhere. An anonymous learner registers in its own learner stream.
//!
//! **Enrollment.** A learner actor only knows a learner once `learner-registered` is in its own
//! stream. [`EnrollmentSaga`] relays the one fact of a handle stream to the learner it registered as
//! a proctor-internal `quiz.enroll-learner` command, issued as the `proctor` service account under a
//! key that is a function of the fact's stream (`enroll:<catalog>:<handle actor id>`, which names
//! no learner); the learner actor appends it unless it is already registered. [`unrelayed`] finds
//! the registrations whose relay a crash lost.
//!
//! **Admission.** [`Admission`] is what a command must pass before the bus reads or places
//! anything: the envelope agrees with the quiz command inside it (design §9a), every id has its
//! shape, the target is the actor the command belongs to, and a registration finds the proctor below
//! its cap of learners. It reads no store — the learner count is a gauge the projector keeps, one
//! per anonymous learner and per claimed handle, so the cap bounds the learner streams and the
//! handle streams alike — and the deciders ask it again, so no path around the hook reaches a
//! stream. The gauge trails the log by the commands in flight, so the cap is a quota, not a ledger:
//! a burst may pass it by as many registrations as were decided before the fold caught up.
//!
//! **One registration per learner.** A handle cannot see whether its claimant is already
//! registered, so a learner that claims a second handle holds both, while its own stream keeps the
//! identity it registered first; recalling either handle answers that learner.
//!
//! **Undecodable facts are loud.** A committed event or a stored state these deciders cannot decode
//! poisons the actor: its state records the failure, every later command answers
//! `actorUnavailable` with it and the cause goes to stderr. Nothing is skipped silently.
//!
//! **Wire mapping** (design §9a): command kind `quiz.<type>`, version [`WIRE_VERSION`] of the contract, command id and
//! idempotency key = the quiz command id, tenant and scope = the catalog id, target
//! `quiz-learner/<learner>` for every command but a pseudonym or name `identify-learner`, which
//! targets `quiz-handle/<handle actor id>`; event kind `quiz.<type>` with the quiz event JSON as
//! payload; a quiz rejection is the framework `invalid` rejection whose `detail` is the rejection
//! string.
//!
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🔨️modules/🧾️lifecycle/🦀️.rs — decide/evolve
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs — `Decider`, `Saga`

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use quiz::{Command, Event, HandleState, Identity, LearnerContext, LearnerState, Limits, WIRE_VERSION};
use semio_framework_dispatch_macros::dyn_enum_close;
use serde::{Deserialize, Serialize};
use server::authority::{ActorState, Decider, Decision, DecisionContext, Saga};
use server::contract::{ActorKey, CommandEnvelope, CommandId, EventRecord, IdempotencyKey, Principal, Rejection, Scope, TenantId, TraceContext};
use server::storage::{AuthorityStore, StorageError};
use server::__semio_dispatch_Decider;

use crate::catalog::LoadedCatalog;
use crate::storage::SqliteAuthorityStore;

/// ✒️ The actor kind of a handle key.
pub const HANDLE: &str = "quiz-handle";
/// 🧑‍🎓️ The actor kind of a learner.
pub const LEARNER: &str = "quiz-learner";
/// 🛎️ The command kind of a registration, anonymous or under a handle: the one kind a client sends
/// that adds to the proctor's count of registrations.
pub const IDENTIFY: &str = "quiz.identify-learner";
/// 📨️ The proctor-internal command relaying a registration to its learner.
pub const ENROLL: &str = "quiz.enroll-learner";
/// 🏁️ The event kind of a submitted run: what tells a learner that played from one that only
/// registered.
pub const RUN_SUBMITTED: &str = "quiz.run-submitted";
/// 🤖️ The service account the proctor's own workflows act as.
pub const PROCTOR_SERVICE: &str = "proctor";
/// 🧬️ The revision of the deciders' folds and of the encoding of their state bytes; a snapshot of
/// another revision is ignored and the stream replayed.
pub const STATE_FORMAT: u32 = 2;

/// 🏷️ `quiz.<type>` of a quiz command.
pub fn command_kind(command: &Command) -> String {
    format!("quiz.{}", command.type_name())
}

/// 🔖️ `quiz.<type>` of a quiz event.
pub fn event_kind(event: &Event) -> String {
    format!("quiz.{}", event.type_name())
}

/// 🗝️ The actor of one handle key of a catalog.
pub fn handle_key(tenant: &str, key: &str) -> ActorKey {
    ActorKey { tenant: TenantId(tenant.to_string()), kind: HANDLE.to_string(), id: quiz::handle_actor_id(key) }
}

/// 🎓️ The actor of one learner of a catalog.
pub fn learner_key(tenant: &str, learner: &str) -> ActorKey {
    ActorKey { tenant: TenantId(tenant.to_string()), kind: LEARNER.to_string(), id: learner.to_string() }
}

/// 🎯️ The actor a quiz command is serialized through: the handle of a pseudonym or name being
/// registered, the learner otherwise; `handle-invalid` for a handle outside the policy.
pub fn command_target(command: &Command, tenant: &str) -> Result<ActorKey, quiz::Rejection> {
    match command {
        Command::IdentifyLearner { identity: Identity::Pseudonym { handle } | Identity::Name { handle }, .. } => quiz::normalize_handle(handle).map(|normalized| handle_key(tenant, &normalized.key)).ok_or(quiz::Rejection::HandleInvalid),
        _ => Ok(learner_key(tenant, command.learner())),
    }
}

/// 🚫️ The framework rejection carrying a quiz rejection.
pub fn rejection(rejection: quiz::Rejection) -> Rejection {
    Rejection::Invalid { detail: rejection.as_str().to_string() }
}

//#region 🔖️Admission
/// 🛃️ What a command passes before anything is read or placed, and the caps the deciders decide
/// with. Cheap and store-free: envelope checks, one decode of the payload, two atomic reads.
pub struct Admission {
    tenant: String,
    learners: Arc<AtomicU64>,
    cap_learners: AtomicU64,
    cap_runs_per_quiz: AtomicU64,
    cap_runs: AtomicU64,
    cap_answers_per_run: AtomicU64,
}

impl Admission {
    /// 🌱️ The admission of one catalog under the default caps, reading the learner count from
    /// `learners` (the projector's gauge).
    pub fn new(tenant: &str, learners: Arc<AtomicU64>) -> Self {
        let caps = quiz::DEFAULT_LIMITS;
        Self { tenant: tenant.to_string(), learners, cap_learners: AtomicU64::new(caps.learners), cap_runs_per_quiz: AtomicU64::new(caps.runs_per_quiz), cap_runs: AtomicU64::new(caps.runs), cap_answers_per_run: AtomicU64::new(caps.answers_per_run) }
    }

    /// 📚️ The catalog id every envelope must name as tenant and scope.
    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    /// 🧱️ The caps in force.
    pub fn caps(&self) -> Limits {
        Limits { learners: self.cap_learners.load(Ordering::Acquire), runs_per_quiz: self.cap_runs_per_quiz.load(Ordering::Acquire), runs: self.cap_runs.load(Ordering::Acquire), answers_per_run: self.cap_answers_per_run.load(Ordering::Acquire) }
    }

    /// 🎚️ Put other caps in force, from the next decision on.
    pub fn cap(&self, caps: Limits) {
        self.cap_learners.store(caps.learners, Ordering::Release);
        self.cap_runs_per_quiz.store(caps.runs_per_quiz, Ordering::Release);
        self.cap_runs.store(caps.runs, Ordering::Release);
        self.cap_answers_per_run.store(caps.answers_per_run, Ordering::Release);
    }

    /// 🙋️ How many registrations the projections count.
    pub fn learners(&self) -> u64 {
        self.learners.load(Ordering::Acquire)
    }

    /// 🚦️ Admit an envelope, or say why not: an enrollment only from the proctor's own service
    /// account and only as the relay it claims to be ([`enrolled`]), any other command as
    /// [`Admission::command`] admits it.
    pub fn admit(&self, envelope: &CommandEnvelope) -> Result<(), Rejection> {
        match envelope.kind == ENROLL {
            true => enrolled(envelope, &self.tenant).map(drop),
            false => self.command(envelope).map(drop),
        }
    }

    /// 📬️ The quiz command of an admitted envelope: it agrees with its envelope ([`admitted`]) and,
    /// when it registers a learner, finds the proctor below its cap of learners (`roster-full`).
    pub fn command(&self, envelope: &CommandEnvelope) -> Result<Command, Rejection> {
        let command = admitted(envelope, &self.tenant)?;
        match (&command, quiz::registration_rejection(self.learners(), &self.caps())) {
            (Command::IdentifyLearner { .. }, Some(full)) => Err(rejection(full)),
            _ => Ok(command),
        }
    }
}

fn mismatch(detail: impl std::fmt::Display) -> Rejection {
    Rejection::Invalid { detail: format!("envelope-mismatch: {detail}") }
}

/// 🪪️ Whether `target` has the shape of a quiz actor: a learner id, or the hex of a handle key (at
/// most three UTF-8 bytes for each of its code points).
pub fn addressable(target: &ActorKey) -> bool {
    match target.kind.as_str() {
        LEARNER => quiz::is_id(&target.id),
        HANDLE => target.id.len() <= 6 * quiz::HANDLE_MAX && quiz::handle_key_of(&target.id).is_some(),
        _ => false,
    }
}

/// 🛂️ The quiz command inside an envelope, once the envelope agrees with it (design §9a): version,
/// tenant and scope, a target of the shape of a quiz actor (`id-invalid`, before the payload is
/// even decoded), a payload that decodes, the kind, ids of their shapes (`id-invalid`), the command
/// id as command id and idempotency key, and the target the command belongs to.
pub fn admitted(envelope: &CommandEnvelope, tenant: &str) -> Result<Command, Rejection> {
    if envelope.version != WIRE_VERSION {
        return Err(mismatch(format!("version {} is not {WIRE_VERSION}", envelope.version)));
    }
    if envelope.target.tenant.0 != tenant || envelope.scope.0 != tenant {
        return Err(mismatch(format!("tenant and scope must both be the catalog id {tenant:?}")));
    }
    if !addressable(&envelope.target) {
        return Err(rejection(quiz::Rejection::IdInvalid));
    }
    let command: Command = serde_json::from_slice(&envelope.payload).map_err(|error| Rejection::Invalid { detail: format!("command-malformed: {error}") })?;
    if envelope.kind != command_kind(&command) {
        return Err(mismatch(format!("kind {:?} does not carry a {} command", envelope.kind, command.type_name())));
    }
    if let Some(malformed) = quiz::command_rejection(&command) {
        return Err(rejection(malformed));
    }
    if envelope.command_id.0 != *command.id() || envelope.idempotency_key.as_ref().map(|key| &key.0) != Some(command.id()) {
        return Err(mismatch("commandId and idempotencyKey must both be the command id"));
    }
    let target = command_target(&command, tenant).map_err(rejection)?;
    if envelope.target != target {
        return Err(mismatch(format!("target must be {}/{}/{}", target.tenant.0, target.kind, target.id)));
    }
    Ok(command)
}

/// 🤝️ The registration an enrollment envelope relays, once it is what [`enrollment`] issues: from
/// the proctor's service account, to the learner the fact registers, under the key of that fact.
pub fn enrolled(envelope: &CommandEnvelope, tenant: &str) -> Result<Event, Rejection> {
    if envelope.principal != (Principal::ServiceAccount { id: PROCTOR_SERVICE.to_string() }) {
        return Err(Rejection::Unauthorized { detail: format!("{ENROLL} is issued by the proctor only") });
    }
    if envelope.version != WIRE_VERSION || envelope.target.tenant.0 != tenant || envelope.scope.0 != tenant || envelope.target.kind != LEARNER {
        return Err(mismatch(format!("{ENROLL} must address a learner of catalog {tenant:?}")));
    }
    let fact: Event = serde_json::from_slice(&envelope.payload).map_err(|error| Rejection::Invalid { detail: format!("{ENROLL} carries no quiz event: {error}") })?;
    let Event::LearnerRegistered { learner, identity, .. } = &fact else {
        return Err(mismatch(format!("{ENROLL} relays a registration")));
    };
    let key = match identity {
        Identity::Anonymous => None,
        Identity::Pseudonym { handle } | Identity::Name { handle } => quiz::normalize_handle(handle).filter(|normalized| normalized.display == *handle).map(|normalized| enrollment_key(tenant, &quiz::handle_actor_id(&normalized.key))),
    };
    if !quiz::is_id(learner) || key.is_none() || *learner != envelope.target.id || Some(&envelope.command_id.0) != key.as_ref() || envelope.idempotency_key.as_ref().map(|idempotency| &idempotency.0) != key.as_ref() {
        return Err(mismatch(format!("{ENROLL} relays the registration of {} under the key of its handle", envelope.target.id)));
    }
    Ok(fact)
}
//#endregion 🔖️Admission

//#region 🔖️Deciders
/// 🖋️ One handle key: registers its first claim, refuses every later one.
pub struct HandleDecider {
    pub admission: Arc<Admission>,
}

/// 🎒️ One learner's registration, runs, answers, submissions and badges, decided against the loaded
/// catalog and the caps in force.
pub struct LearnerDecider {
    pub catalog: Arc<LoadedCatalog>,
    pub admission: Arc<Admission>,
}

/// ☣️ What an actor's state bytes say once a fact or a state could not be decoded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Poison {
    pub corrupt: String,
}

/// 🧠️ An actor's stored state: nothing yet, the state, or the reason it can no longer be trusted.
#[derive(Clone, Debug, PartialEq)]
pub enum Stored<T> {
    Fresh,
    State(T),
    Corrupt(String),
}

/// 📖️ Read an actor's state bytes: empty bytes are a fresh actor, anything that is neither the
/// state nor a [`Poison`] is corrupt.
pub fn stored<T: serde::de::DeserializeOwned>(state: &ActorState) -> Stored<T> {
    if state.bytes.is_empty() {
        return Stored::Fresh;
    }
    match serde_json::from_slice::<T>(&state.bytes) {
        Ok(value) => Stored::State(value),
        Err(error) => Stored::Corrupt(serde_json::from_slice::<Poison>(&state.bytes).map_or_else(|_| format!("the stored state does not decode: {error}"), |poison| poison.corrupt)),
    }
}

fn encoded<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

fn unavailable(actor: &ActorKey, corrupt: &str) -> Decision {
    eprintln!("[ERROR] proctor actor {}/{} is unavailable: {corrupt}", actor.kind, actor.id);
    Decision::Reject(Rejection::ActorUnavailable { detail: format!("actor-corrupt: {corrupt}") })
}

fn poison(state: &mut ActorState, event: &EventRecord, cause: impl std::fmt::Display) {
    let corrupt = format!("event {} of {}/{} ({}) cannot be folded: {cause}", event.seq, event.stream.kind, event.stream.id, event.kind);
    eprintln!("[ERROR] proctor actor {}/{} is poisoned: {corrupt}", event.stream.kind, event.stream.id);
    state.bytes = encoded(&Poison { corrupt });
}

/// 🧮️ Fold one committed event into an actor's state bytes with `fold`, starting from `fresh`; an
/// event or a state that does not decode poisons the actor instead of being skipped.
fn evolved<T: Serialize + serde::de::DeserializeOwned>(state: &mut ActorState, event: &EventRecord, fresh: impl FnOnce() -> Option<T>, fold: impl FnOnce(&mut T, &Event)) {
    let mut value = match stored::<T>(state) {
        Stored::State(value) => value,
        Stored::Corrupt(_) => return,
        Stored::Fresh => match fresh() {
            Some(value) => value,
            None => return poison(state, event, "the stream id names no actor of this kind"),
        },
    };
    match serde_json::from_slice::<Event>(&event.payload) {
        Ok(fact) => {
            fold(&mut value, &fact);
            state.bytes = encoded(&value);
        }
        Err(error) => poison(state, event, error),
    }
}

impl Decider for HandleDecider {
    async fn actor_kind(&self) -> &str {
        HANDLE
    }

    async fn decide(&self, state: &ActorState, envelope: &CommandEnvelope, context: &DecisionContext) -> Decision {
        let handle = match stored::<HandleState>(state) {
            Stored::Corrupt(corrupt) => return unavailable(&envelope.target, &corrupt),
            Stored::State(handle) => Some(handle),
            Stored::Fresh => None,
        };
        match (self.admission.command(envelope), handle.or_else(|| quiz::handle_key_of(&envelope.target.id).map(|key| quiz::empty_handle_state(&key)))) {
            (Ok(command), Some(handle)) => emitted(envelope, context, quiz::decide_handle(&handle, &command, context.now.millis)),
            (Err(refusal), _) => Decision::Reject(refusal),
            (_, None) => Decision::Reject(rejection(quiz::Rejection::IdInvalid)),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        evolved(state, event, || quiz::handle_key_of(&event.stream.id).map(|key| quiz::empty_handle_state(&key)), quiz::evolve_handle);
    }

    async fn state_format(&self) -> u32 {
        STATE_FORMAT
    }
}

impl Decider for LearnerDecider {
    async fn actor_kind(&self) -> &str {
        LEARNER
    }

    async fn decide(&self, state: &ActorState, envelope: &CommandEnvelope, context: &DecisionContext) -> Decision {
        let learner = match stored::<LearnerState>(state) {
            Stored::Corrupt(corrupt) => return unavailable(&envelope.target, &corrupt),
            Stored::State(learner) => learner,
            Stored::Fresh => quiz::empty_learner_state(&envelope.target.id),
        };
        if envelope.kind == ENROLL {
            return match enrolled(envelope, self.admission.tenant()) {
                Ok(fact) => Decision::Emit { events: if learner.identity.is_some() { Vec::new() } else { vec![record(&envelope.target, context, &fact)] }, effects: Vec::new() },
                Err(refusal) => Decision::Reject(refusal),
            };
        }
        match self.admission.command(envelope) {
            Ok(command) => {
                let caps = self.admission.caps();
                let lifecycle = LearnerContext { now: context.now.millis, catalog: &self.catalog.catalog, quizzes: self.catalog.current(), limits: &caps };
                emitted(envelope, context, quiz::decide_learner(&learner, &command, &lifecycle))
            }
            Err(refusal) => Decision::Reject(refusal),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        evolved(state, event, || quiz::is_id(&event.stream.id).then(|| quiz::empty_learner_state(&event.stream.id)), quiz::evolve_learner);
    }

    async fn state_format(&self) -> u32 {
        STATE_FORMAT
    }
}

dyn_enum_close! {
    /// ⚖️ The proctor's closed decider set.
    pub enum ProctorDeciders: Decider {
        Handle(HandleDecider),
        Learner(LearnerDecider),
    }
}

/// 🧰️ The deciders of one catalog: the handle and the learner, sharing one admission.
pub fn deciders(catalog: &Arc<LoadedCatalog>, admission: &Arc<Admission>) -> Vec<ProctorDeciders> {
    vec![ProctorDeciders::Handle(HandleDecider { admission: Arc::clone(admission) }), ProctorDeciders::Learner(LearnerDecider { catalog: Arc::clone(catalog), admission: Arc::clone(admission) })]
}

fn emitted(envelope: &CommandEnvelope, context: &DecisionContext, decision: quiz::Decision) -> Decision {
    match decision {
        quiz::Decision::Events(events) => Decision::Emit { events: events.iter().map(|fact| record(&envelope.target, context, fact)).collect(), effects: Vec::new() },
        quiz::Decision::Rejection(refused) => Decision::Reject(rejection(refused)),
    }
}

fn record(target: &ActorKey, context: &DecisionContext, fact: &Event) -> EventRecord {
    EventRecord { stream: target.clone(), seq: 0, hlc: context.now, kind: event_kind(fact), payload: encoded(fact) }
}
//#endregion 🔖️Deciders

//#region 🔖️Saga
/// 🧵️ Relays the registration a handle stream committed to the learner it registered.
pub struct EnrollmentSaga;

impl Saga for EnrollmentSaga {
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope> {
        enrollment(event).into_iter().collect()
    }
}

/// 🔑️ The command id and idempotency key of the enrollment one handle stream owes: a function of
/// the stream whose one fact it relays, naming the handle actor and saying nothing of the learner.
pub fn enrollment_key(tenant: &str, handle_actor: &str) -> String {
    format!("enroll:{tenant}:{handle_actor}")
}

/// 🧾️ The key the receipt of one handle stream's enrollment is stored under — the proctor's own key
/// space of the authority's receipts. The receipt belongs to the learner the claim was relayed to,
/// so whoever removes a handle stream and leaves that learner removes this receipt with it: the
/// next claim of the handle is relayed under the same key, to somebody else.
pub fn enrollment_receipt(tenant: &str, handle_actor: &str) -> IdempotencyKey {
    server::authority::receipt_key(&Principal::ServiceAccount { id: PROCTOR_SERVICE.to_string() }, &IdempotencyKey(enrollment_key(tenant, handle_actor)))
}

/// 📮️ The enrollment command relaying one committed event, if it is the registration of a handle
/// stream.
pub fn enrollment(event: &EventRecord) -> Option<CommandEnvelope> {
    if event.stream.kind != HANDLE {
        return None;
    }
    let Event::LearnerRegistered { learner, .. } = serde_json::from_slice::<Event>(&event.payload).ok()? else { return None };
    let tenant = &event.stream.tenant.0;
    let key = enrollment_key(tenant, &event.stream.id);
    Some(CommandEnvelope {
        command_id: CommandId(key.clone()),
        kind: ENROLL.to_string(),
        version: WIRE_VERSION,
        target: learner_key(tenant, &learner),
        scope: Scope(tenant.clone()),
        principal: Principal::ServiceAccount { id: PROCTOR_SERVICE.to_string() },
        session: None,
        device: None,
        payload: event.payload.clone(),
        causal_frontier: None,
        client_hlc: event.hlc,
        expected_revision: None,
        idempotency_key: Some(IdempotencyKey(key)),
        capability_proof: None,
        trace: TraceContext::default(),
    })
}

/// 🔁️ The enrollments still owed: one per committed handle registration whose learner stream is
/// empty — what a crash between marking an outbox row delivered and submitting its relay leaves
/// behind. One indexed read per handle, no bus.
pub async fn unrelayed(log: &SqliteAuthorityStore, tenant: &str) -> Result<Vec<CommandEnvelope>, StorageError> {
    let mut owed = Vec::new();
    for event in log.stream_events(tenant, HANDLE)? {
        let Some(command) = enrollment(&event) else {
            return Err(StorageError::Backend(format!("event {} of {}/{} ({}) is no registration this proctor can relay", event.seq, event.stream.kind, event.stream.id, event.kind)));
        };
        if log.last_seq(&command.target).await? == 0 {
            owed.push(command);
        }
    }
    Ok(owed)
}
//#endregion 🔖️Saga

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
