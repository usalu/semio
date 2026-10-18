#!/usr/bin/env python3
"""🧩 BD12 item 3: stages the decomposition of `poll_kernel_turn` (reactor turn crate) from the pre-edit snapshot `orig/rt/🌀️poll-kernel-turn/🦀️.rs`.
Every phase body is a verbatim line range of the snapshot plus declared mechanical substitutions; nothing is written to the repo here, only to `stage/`.
usage: gen_rt.py   (writes bd12-tools/stage/**)"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SNAP = HERE + "/orig/rt"
STAGE = HERE + "/stage"
O = open(SNAP + "/🌀️poll-kernel-turn/🦀️.rs", encoding="utf-8").read().split("\n")


def L(a, b, indent=0, sub=(), loose=False):
    text = "\n".join(O[a - 1:b])
    for old, new in sub:
        assert loose or old in text, ("missing substitution anchor", old, a, b)
        text = text.replace(old, new)
    if indent:
        text = "\n".join((" " * indent + l) if l.strip() else l for l in text.split("\n"))
    return text


def anchor(a, expect):
    assert O[a - 1].lstrip().startswith(expect), (a, O[a - 1][:100], expect)


STRING = re.compile(r'b?"(?:\\.|[^"\\])*"')


def words(text, table):
    out, last = [], 0
    pieces = []
    for m in STRING.finditer(text):
        pieces.append((text[last:m.start()], False))
        pieces.append((m.group(0), True))
        last = m.end()
    pieces.append((text[last:], False))
    for piece, literal in pieces:
        if not literal:
            for old, new in table:
                piece = re.sub(r"(?<![A-Za-z0-9_])" + re.escape(old) + r"(?![A-Za-z0-9_])", new, piece)
        out.append(piece)
    return "".join(out)


def put(rel, text):
    path = os.path.join(STAGE, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text if text.endswith("\n") else text + "\n")


OF = "semio_framework_job::OriginalTurnFailure"
CX = "&mut semio_framework_job::StepContext<'_>"
IDENT = "&mut EntityIdentityAuthority<'_>"
BASE_IMPORTS = """use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
"""
HEAD_IMPORTS = BASE_IMPORTS + """use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
"""

# ---------------------------------------------------------------- 🪜️ entry, lifecycle, retirement
anchor(186, "budget.retained.validate()?;")
anchor(187, "if budget.retained.operation!=")
anchor(189, "if REFUSED_PUBLICATION.with")
anchor(190, 'trace_turn_phase_retention("enter");')
anchor(193, "let mut focus = None;")
anchor(235, "let mut close_instances")
anchor(239, "retire_while_progress_fallible")
anchor(271, "for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {")
anchor(279, "let close_cleanup_work=")
anchor(288, "semio_framework_plugin_runtime_instances::inst_opening::plugin_resume_actor_openings")
assert O[277].strip() == "}" and O[270].lstrip().startswith("for _ in 0..PATCH_CLOSE_UNITS_PER_TURN")
assert L(271, 278) == L(777, 784), "the two patch close loops must be identical"

entry = f"""//! 🪜️ Entry, lifecycle focus and retirement phases of the reactor turn: the borrowed retained input admission, the one lifecycle command per turn and the bounded close ladders that run before the turn's event. See [`super`].

{BASE_IMPORTS}
/// 🚪️ Admits the borrowed retained input of one poll: it must equal the context's own operation, generation and grant, and no refused publication may still own its output.
pub(super) fn admit_turn_entry(budget: semio_framework::kernel::Budget, cx: &semio_framework_job::StepContext<'_>) -> Result<(), {OF}> {{
{L(186, 187)}
{L(189, 190)}
    Ok(())
}}

/// 🎯️ Admits the one lifecycle command this turn serves (close request, acknowledgement staging, admission advance) and returns the instance whose lifecycle the turn advances.
pub(super) fn admit_lifecycle_focus<PA: PluginApp>(runtime: &PluginRuntime<PA>, events: &Option<Event>) -> Result<Option<u32>, {OF}> {{
{L(193, 234)}
    Ok(focus)
}}

/// 🧯️ Drives the bounded close ladders of one turn (reactor close, cold pair close, patch and document retirement, close cleanup, live cleanup pressure steps) and reports whether close cleanup still has work.
pub(super) fn drive_turn_retirement<PA: PluginApp>(runtime: &PluginRuntime<PA>, cx: {CX}) -> Result<bool, {OF}> {{
{L(239, 270)}
    step_pending_patch_close(cx)?;
{L(279, 287)}
    Ok(close_cleanup_work)
}}

/// 🩹️ Spends the turn's patch close units on the pending patch owner until one unit completes or the turn must yield.
pub(super) fn step_pending_patch_close(cx: {CX}) -> Result<(), {OF}> {{
{L(271, 278)}
    Ok(())
}}
"""
assert "dirty" not in L(239, 287) and "identity" not in L(239, 287)
put("🪜️entry-lifecycle-and-retirement/🦀️.rs", entry)

# ---------------------------------------------------------------- 📨️ event dispatch
anchor(291, "if matches!(events.as_ref(),Some(Event::Completed")
anchor(303, "} else { None };")
anchor(304, "for event in selected_event {")
anchor(325, "Event::UiIntent")
anchor(342, "Event::SurfaceVisible")
anchor(387, "Event::JobCompleted")
anchor(405, "Event::Message { source: MessageEndpoint::Shell")
anchor(441, "}")
DIRTY_SUB = (("&mut dirty", "&mut *dirty"), ("&mut document_backbone_effects", "&mut *document_backbone_effects"))


def arm_body(a, b, sub=DIRTY_SUB):
    # arm lines a..b are `Event::X {..} => {` .. `}`; returns the body between them dedented by 12
    inner = L(a + 1, b - 1, sub=())
    for old, new in sub:
        inner = inner.replace(old, new)
    return "\n".join(l[12:] if l.startswith(" " * 12) else l for l in inner.split("\n"))


ui_intent_body = arm_body(325, 341)
job_done_body = arm_body(387, 404)
assert "continue" not in ui_intent_body + job_done_body

dispatch_main = L(304, 441, sub=DIRTY_SUB)
# replace the two extracted arms by calls
ui_arm = L(325, 341, sub=DIRTY_SUB, loose=True)
job_arm = L(387, 404, sub=DIRTY_SUB, loose=True)
assert dispatch_main.count(ui_arm) == 1 and dispatch_main.count(job_arm) == 1
dispatch_main = dispatch_main.replace(ui_arm, "            Event::UiIntent { instance, intent } => ui_intent_event(runtime, instance, intent, dirty)?,")
dispatch_main = dispatch_main.replace(job_arm, "            Event::JobCompleted { job, result } => job_completed_event(runtime, job, result, dirty, document_backbone_effects, identity, cx).await?,")

events = f"""//! 📨️ Event dispatch phases of the reactor turn: the selection of the one event a turn serves, its routing, and the original receiving advance that follows it. See [`super`].

{HEAD_IMPORTS}
/// 🎟️ Retains a completed donation and selects the one event this turn may serve under its retained grant, if any.
pub(super) fn select_turn_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, events: &mut Option<Event>, cx: {CX}) -> Result<Option<Event>, {OF}> {{
{L(291, 303)}
    Ok(selected_event)
}}

/// 📨️ Routes the selected event to the owner it names; dirty surfaces and document effects accumulate in the caller's owners.
pub(super) async fn dispatch_turn_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, selected_event: Option<Event>, dirty: &mut DirtyPollOwners, document_backbone_effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
{dispatch_main}
    Ok(())
}}

/// 🎯️ Decodes one UI intent, drops it when its tree is stale and batches it per instance for the dispatch pass.
fn ui_intent_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance: semio_framework::kernel::PluginInstanceId, intent: Vec<u8>, dirty: &mut DirtyPollOwners) -> Result<(), {OF}> {{
{ui_intent_body}
    Ok(())
}}

/// 🧬️ Completes a spawned job: resolves its render binding, hands a reserved output to its instance and resolves the parked request.
async fn job_completed_event<PA: PluginApp>(runtime: &PluginRuntime<PA>, job: u64, result: semio_framework::kernel::RequestOutcome, dirty: &mut DirtyPollOwners, document_backbone_effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
{job_done_body}
    Ok(())
}}

/// 📥️ Advances the original receiving owners (intents retained, responses, intents) with the turn's context.
pub(super) async fn advance_original_receiving<PA: PluginApp>(runtime: &PluginRuntime<PA>, dirty: &mut DirtyPollOwners, effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
{L(771, 775, sub=(("&mut effects", "&mut *effects"),))}
    Ok(())
}}
"""
put("📨️event-dispatch/🦀️.rs", events)

# ---------------------------------------------------------------- ⌨️ command ingress
anchor(486, "let mut command_ingress")
anchor(497, "retained = match retained.take() {")
anchor(533, "retained = match retained.take() {")
anchor(562, "if matches!(command_ingress")
anchor(594, "if let Some(owner) = retained.take() {")
anchor(618, "if let Some((cursor, page)) = command_page {")
anchor(622, "if !runtime.guest_lifetimes.borrow().get(cursor.instance)")
anchor(640, "{")
anchor(649, "} else if cursor.kind == 28 {")
anchor(694, "} else if cursor.page_index == 0 && retained.is_none() {")
anchor(723, "} else if let Some(CommandIngressOwner::GenericAssembly")
anchor(729, "if cursor.page_index as usize != pages.len() {")
anchor(761, "} else {")
anchor(765, "command_ingress = command_ingress_while_owned")

SELF = (("retained_key", "self.key"), ("retained", "self.retained"), ("command_ingress", "self.status"), ("close_instances", "self.close_instances"), ("&mut effects", "&mut *effects"))


def method(a, b, indent=8):
    return words(L(a, b, sub=()), SELF)


def reindent(text, shift):
    return "\n".join((" " * shift + l) if l.strip() else l for l in text.split("\n"))


def dedent(text, n):
    return "\n".join(l[n:] if l.startswith(" " * n) else l for l in text.split("\n"))


valid_cond = L(622, 639)
valid_cond = valid_cond.replace("    if !runtime.guest_lifetimes", "!runtime.guest_lifetimes", 1)
valid_cond = valid_cond.lstrip()
assert valid_cond.startswith("!runtime.guest_lifetimes")
valid_cond = "\n".join([valid_cond.split("\n")[0]] + [l[4:] for l in valid_cond.split("\n")[1:]])

presence_body = dedent(words(L(650, 693), SELF), 4)
first_body = dedent(words(L(695, 722), SELF), 4)
assembly_body = dedent(words(L(729, 760), SELF), 4)
assert "continue" not in presence_body + first_body + assembly_body
resume_head = words(L(486, 496), (("command_ingress", "command_ingress"),))

cancel_body = words(L(497, 532), SELF)
readmit_body = words(L(533, 561), SELF)
presence_exchange_body = words(L(562, 593), SELF)
generic_exchange_body = words(L(594, 617), SELF)

command = f"""//! ⌨️ Command ingress phase of the reactor turn: the retained ingress owner, its cancellation by instance close, its resumption and the admission of one inbound command page. See [`super`].

{HEAD_IMPORTS}use semio_framework::kernel::{{CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage}};

/// ⌨️ The command ingress status, retained owner and exact lifetime one turn carries between its ingress steps.
pub(super) struct CommandIngressTurn {{
    status: CommandIngressStatus,
    retained: Option<CommandIngressOwner>,
    key: Option<instance_lifetime::NativeCloseKey>,
    close_instances: Vec<u32>,
}}

/// 🧾️ Whether a command page violates the fixed page, command or presence limits, its instance is not live or its instance is closing.
fn command_page_invalid<PA: PluginApp>(runtime: &PluginRuntime<PA>, cursor: &CommandPageCursor, page: &FixedCommandPage, close_instances: &[u32]) -> bool {{
    {valid_cond}
}}

/// ⌨️ Runs the whole command ingress step of one turn and returns the status the turn reports.
pub(super) async fn drive_command_ingress<PA: PluginApp>(runtime: &PluginRuntime<PA>, command_page: Option<(CommandPageCursor, FixedCommandPage)>, close_instances: Vec<u32>, effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<CommandIngressStatus, {OF}> {{
    let mut turn = CommandIngressTurn::resume(runtime, close_instances, cx)?;
    turn.cancel_for_close();
    turn.readmit_reserved(runtime).await;
    turn.exchange_presence(runtime, effects, identity, cx).await?;
    turn.exchange_generic(runtime, effects, identity, cx).await?;
    if let Some(incoming) = command_page {{
        turn.admit_page(runtime, incoming, effects, identity, cx).await?;
    }}
    Ok(turn.finish())
}}

impl CommandIngressTurn {{
    /// 🔁️ Takes the retained owner of the live slot and marks its instance for cancellation when its lifetime is no longer the exact retained one.
    fn resume<PA: PluginApp>(runtime: &PluginRuntime<PA>, mut close_instances: Vec<u32>, cx: {CX}) -> Result<Self, {OF}> {{
        let status = CommandIngressStatus::Idle;
        step_retiring_command_ingress(cx)?;
{reindent(L(488, 496, sub=(("let (mut retained, mut retained_key)", "let (retained, retained_key)"),)), 4)}
        Ok(Self {{ status, retained, key: retained_key, close_instances }})
    }}

    /// 🚫️ Cancels the retained owner of every instance closing this turn and steps an owner that is already closing.
    fn cancel_for_close(&mut self) {{
{reindent(cancel_body, 4)}
    }}

    /// 🎟️ Offers a reserved presence admission or a pending presence page to its owner once more.
    async fn readmit_reserved<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>) {{
{reindent(readmit_body, 4)}
    }}

    /// 👥️ Exchanges one presence page of the retained presence owner when the turn has no other status.
    async fn exchange_presence<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
{reindent(presence_exchange_body, 4)}
        Ok(())
    }}

    /// 🔁️ Retries the retained generic command with the instance that owns it.
    async fn exchange_generic<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
{reindent(generic_exchange_body, 4)}
        Ok(())
    }}

    /// 📄️ Admits one inbound command page against the retained owner: invalid, superseded, presence, first generic page or assembly continuation.
    async fn admit_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
        let (cursor, page) = incoming;
{words(L(619, 621), SELF)}
        if command_page_invalid(runtime, &cursor, &page, &self.close_instances) {{
{words(L(641, 641), SELF)}
{words(L(642, 648), SELF)}
        }} else if cursor.kind == 28 {{
            self.admit_presence_page(runtime, (cursor, page)).await;
        }} else if cursor.page_index == 0 && self.retained.is_none() {{
            self.admit_first_generic_page(runtime, (cursor, page), effects, identity, cx).await?;
        }} else if let Some(CommandIngressOwner::GenericAssembly {{ cursor: active, pages }}) = {{
            match self.retained {{
                Some(CommandIngressOwner::GenericAssembly {{ .. }}) => self.retained.take(),
                _ => None,
            }}
        }} {{
            self.continue_generic_assembly(runtime, (cursor, page), (active, pages), effects, identity, cx).await?;
        }} else {{
{words(L(762, 762), SELF)}
        }}
        Ok(())
    }}

    /// 👥️ Admits one presence ingress page: the first reserves the instance's presence admission, later pages push into the published generation.
    async fn admit_presence_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage)) {{
        let (cursor, page) = incoming;
{presence_body}
    }}

    /// 🧱️ Admits the first page of a generic command and exchanges it when it is the only page.
    async fn admit_first_generic_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
        let (cursor, page) = incoming;
{first_body}
        Ok(())
    }}

    /// 🧩️ Appends a later page to the retained assembly and exchanges the command once its last page arrives.
    async fn continue_generic_assembly<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), assembly: (CommandPageCursor, CommandPageSet), effects: &mut Vec<Effect>, identity: {IDENT}, cx: {CX}) -> Result<(), {OF}> {{
        let (cursor, page) = incoming;
        let (active, mut pages) = assembly;
{assembly_body}
        Ok(())
    }}

    /// 🏁️ Reports the status while an owner is retained and parks the owner in the live slot with its exact lifetime.
    fn finish(mut self) -> CommandIngressStatus {{
        self.status = command_ingress_while_owned(self.status, self.retained.as_ref().map(command_ingress_owner_cursor));
        if let Some(retained) = self.retained {{
            COMMAND_INGRESS.with(|ingress| {{
                ingress.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT] = Some(RetainedCommandIngress {{ key: self.key.expect("admitted command retains its exact lifetime"), state: retained }});
            }});
        }}
        self.status
    }}
}}
"""
put("⌨️command-ingress-turn/🦀️.rs", command)

# ---------------------------------------------------------------- 🎨️ dirty surface render + reconcile
anchor(797, "for (instance, surface) in dirty.surfaces {")
anchor(850, "}")
anchor(851, "let reconcile_work = PATCHES")
anchor(859, "}")
render = f"""//! 🎨️ Dirty surface render and reconcile phases of the reactor turn: renders every dirty surface under its reconcile reservation and drives the reconcile opportunities of the turn. See [`super`].

{BASE_IMPORTS}
/// 🎨️ Renders each dirty surface of the turn under a reserved reconcile grant, defers a surface that was refused its reservation and records presence updates.
pub(super) async fn render_dirty_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, dirty: DirtyPollOwners, now_ms: u64, effects: &mut Vec<Effect>, cx: {CX}) -> Result<(), {OF}> {{
{L(797, 850, sub=(("effects.push(", "effects.push("),))}
    Ok(())
}}

/// 🩹️ Drives the reconcile opportunities of the turn, reports whether reconcile arms another turn and answers a render fault with a shell fault effect.
pub(super) fn drive_turn_reconcile(effects: &mut Vec<Effect>, cx: {CX}) -> Result<bool, {OF}> {{
{L(851, 859)}
    Ok(reconcile_work)
}}
"""
put("🎨️dirty-surface-render/🦀️.rs", render)

# ---------------------------------------------------------------- 🔚️ finish and publish
anchor(926, "with_pending_patches(|pending| {")
anchor(965, "})")
anchor(966, "}")
finish = f"""//! 🔚️ Publication phase of the reactor turn: validates and stages the patch receipt, prepares the caller's output and either publishes the turn result or hands every transferred patch back. See [`super`].

{BASE_IMPORTS}
/// 🔚️ Stages the patch emission of one built turn result, finishes the focused lifecycle turn and publishes the result; a failed preparation returns every transferred patch to its pending slot.
pub(super) fn publish_turn_result<PA: PluginApp, T, Prepared>(
    runtime: &PluginRuntime<PA>,
    mut result: semio_framework::kernel::TurnResult,
    focus: Option<u32>,
    retryable_lifecycle: bool,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
) -> Result<T, {OF}> {{
{L(926, 965)}
}}
"""
put("🔚️turn-finish/🦀️.rs", finish)

# ---------------------------------------------------------------- appended phases of existing children
anchor(443, "if let Some(effect)=advance_retained_extension_invocation")
anchor(447, "let (extension_retirement_work, _)")
ext_append = f"""
/// 🔌️ Advances the retained extension invocation and the extension retirement under the mounted close policy; reports whether extension retirement still has work.
pub(super) async fn drive_extension_turn<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, cx: {CX}) -> Result<bool, {OF}> {{
{L(443, 447, sub=(("inbound_request_effects.push(effect)", "effects.push(effect)"),))}
    Ok(extension_retirement_work)
}}
"""
put("🔌️extension-invocation-and-command-ingress/append.rs", ext_append)

anchor(451, "let mut cold_pair_ingress")
anchor(483, "}")
cold_append = f"""
/// 🧊️ Accepts the inbound cold document pair page, begins its document load and steps the pending load; returns the ingress status this turn answers.
pub(super) async fn drive_cold_pair_ingress<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: {CX}) -> semio_framework::kernel::ColdPairIngressStatus {{
{L(451, 483)}
    cold_pair_ingress
}}
"""
put("🧊️cold-pair-loads-and-process-pool/append.rs", cold_append)

anchor(861, "let executor_deadline_work")
anchor(867, "}")
more_append = f"""
/// 🔦️ The work the executors reported on one turn.
#[derive(Clone, Copy)]
pub(super) struct TurnExecutorWork {{
    deadline: bool,
    task: bool,
    pool: bool,
}}

/// 🧮️ Every source the turn read before deciding whether another turn is needed.
pub(super) struct TurnWorkInputs {{
    pub(super) events_pending: bool,
    pub(super) executors: TurnExecutorWork,
    pub(super) close_cleanup_work: bool,
    pub(super) typed_operation_runnable: bool,
    pub(super) typed_operation_contended: bool,
    pub(super) reconcile_work: bool,
    pub(super) resumes_remain: bool,
    pub(super) document_load_work: bool,
}}

/// 🏃️ Runs the reactor and task executors to their deadline, pumps the process worker pool and admits the registry's effects.
pub(super) fn drive_turn_executors(effects: &mut Vec<Effect>) -> TurnExecutorWork {{
{L(861, 863)}
    for effect in REGISTRY.with(|registry| registry.drain()) {{
        push_admitted_effect(&mut *effects, 0, effect);
    }}
    TurnExecutorWork {{ deadline: executor_deadline_work, task: task_executor_work, pool: process_pool_work }}
}}

/// 🔦️ Decides whether the turn answers `MoreWork`, records which sources armed it and traces the more-work streak.
pub(super) fn settle_turn_work<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, inputs: TurnWorkInputs, effects: &[Effect]) -> Result<bool, {OF}> {{
    let TurnWorkInputs {{ events_pending, executors, close_cleanup_work, typed_operation_runnable, typed_operation_contended, reconcile_work, resumes_remain, document_load_work }} = inputs;
    let (executor_deadline_work, task_executor_work, process_pool_work) = (executors.deadline, executors.task, executors.pool);
    let more_work = executor_deadline_work || task_executor_work || process_pool_work;
{L(871, 913, sub=(("events.is_some()", "events_pending"), ("typed_operation_scan.runnable", "typed_operation_runnable")))}
    Ok(more_work)
}}
"""
put("🔦️more-work-sources-and-tracing/append.rs", more_append)

# ---------------------------------------------------------------- the slimmed poll_kernel_turn
head = "\n".join(O[:185])
assert O[174].startswith("pub(super) async fn poll_kernel_turn"), O[174][:60]
anchor(191, "let retryable_lifecycle")
anchor(235, "let mut close_instances: Vec<u32>")
close_instances_line = L(235, 235).replace("let mut close_instances", "let close_instances")
assert close_instances_line != L(235, 235)
body = f"""    admit_turn_entry(budget, cx)?;
    let original_progress=cx.retained_progress();let original_work=cx.original_work_units_spent();
{L(191, 191)}
    let mut dirty = DirtyPollOwners::new();
    let focus = admit_lifecycle_focus(runtime, events)?;
{close_instances_line}
    let mut effects: Vec<Effect> = Vec::new();
    trace_turn_phase_retention("lifecycle");
    let close_cleanup_work = drive_turn_retirement(runtime, cx)?;
{L(288, 290)}
    let selected_event = select_turn_event(runtime, events, cx)?;
    dispatch_turn_event(runtime, selected_event, &mut dirty, &mut effects, identity, cx).await?;
    let extension_retirement_work = drive_extension_turn(runtime, &mut effects, cx).await?;
    let close_cleanup_work = close_cleanup_work || extension_retirement_work;
    let cold_pair_ingress = drive_cold_pair_ingress(runtime, cold_pair_page, identity, cx).await;
{L(484, 485, sub=(("&mut effects", "&mut effects"),))}
    let command_ingress = drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?;
    advance_original_receiving(runtime, &mut dirty, &mut effects, identity, cx).await?;
    step_pending_patch_close(cx)?;

{L(786, 796)}
    render_dirty_surfaces(runtime, dirty, now_ms, &mut effects, cx).await?;
    let reconcile_work = drive_turn_reconcile(&mut effects, cx)?;
    let executors = drive_turn_executors(&mut effects);
{L(869, 870)}
    let more_work = settle_turn_work(runtime, TurnWorkInputs {{ events_pending: events.is_some(), executors, close_cleanup_work, typed_operation_runnable: typed_operation_scan.runnable, typed_operation_contended, reconcile_work, resumes_remain, document_load_work }}, &effects)?;

{L(915, 925, sub=(("let mut result =", "let result ="),))}
    publish_turn_result(runtime, result, focus, retryable_lifecycle, prepare, publish)
}}
"""
put("🌀️poll-kernel-turn/🦀️.rs", head + "\n" + body)
print("staged", sorted(os.path.relpath(os.path.join(dp, f), STAGE) for dp, dn, fn in os.walk(STAGE) for f in fn))
