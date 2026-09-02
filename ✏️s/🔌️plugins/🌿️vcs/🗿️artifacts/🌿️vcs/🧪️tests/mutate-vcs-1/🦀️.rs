//! 🦀️ VCS checkpoint exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `vcs-1-checkpoint-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): `s.vcs.vcs` is a
//! semio-NATIVE checkpoint document with no third-party reader or writer, so `oracle` here reads
//! the committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_vcs_mutation_reporting` over the full six-kind `VcsDemoMutation`
//! vocabulary.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law this case claims is asserted INSIDE the subject handler. `mutate-<kind>` checks
//! three things at once: the applied snapshot IS the committed after-snapshot, the ONE projection
//! member the feature's `moves` column names is the only member that moved, and the diagnostic
//! codes the implementation raised are the ones the committed `🎯️outcome/🔣️.json`
//! declares. `inverse-<kind>` checks that the mutation's own computed inverse restores the
//! committed before-snapshot exactly. A handler that merely returned `Ok` would report a pass
//! having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the vcs artifact would make one plugin's
//! test tree a build dependency of another's. The three laws are therefore stated inline, in the
//! same words and with the same strictness: inverse restores the ORIGINAL projection,
//! `mutate-<kind>` must MOVE the projection, and the identity carrier is asserted byte-exact
//! because it genuinely is one (see `round_trip`).
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`🦀️.rs`), so neither
//! `protocol::Mutation` nor a `serde` derive is nameable from here. The subset's own production
//! code exports the bridges instead, whose signatures name only reachable types:
//! `decode_vcs_snapshot_json`/`encode_vcs_snapshot_json`/`parse_vcs_dsl`/`print_vcs_dsl`
//! (`…/🧬️schema/📸️snapshot/🦀️.rs`) and `decode_vcs_mutation_json`/
//! `apply_vcs_mutation_reporting`/`inverse_vcs_mutation_steps` (`…/🧬️schema/🧬️mutations/🦀️.rs`).
//! Both roles read the SAME committed bytes — the oracle role via `include_str!`, the subject role
//! by decoding that same text. The subject half is gated behind the generated host's `sut` feature
//! so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `VcsDemoMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum. No `no-mutation`, no `set-snapshot`: whole-document replace is banned vocabulary here.
const KINDS: &[&str] = &["rename-vcs", "change-counter", "change-notes", "change-status", "add-tag", "remove-tag"];

/// 🗣️ The real committed checkpoint — "VCS Demo" at counter 2, status `draft`, tags `alpha` then `beta`.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` fixture TEXT for one kind, read literally
/// via `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. One `include_str!` per file for the whole adapter: `oracle`
/// answers with `before`/`after`, `subject` decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "rename-vcs" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs/🧪️tests/retitles-the-document/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs/🧪️tests/retitles-the-document/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs/🧪️tests/retitles-the-document/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-vcs/🧪️tests/retitles-the-document/🎯️outcome/🔣️.json"),
        ),
        "change-counter" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-counter/🧪️tests/sets-counter-to-seven/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-counter/🧪️tests/sets-counter-to-seven/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-counter/🧪️tests/sets-counter-to-seven/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-counter/🧪️tests/sets-counter-to-seven/🎯️outcome/🔣️.json"),
        ),
        "change-notes" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-notes/🧪️tests/rewrites-the-notes/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-notes/🧪️tests/rewrites-the-notes/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-notes/🧪️tests/rewrites-the-notes/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-notes/🧪️tests/rewrites-the-notes/🎯️outcome/🔣️.json"),
        ),
        "change-status" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦change-status/🧪️tests/draft-to-review/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦change-status/🧪️tests/draft-to-review/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦change-status/🧪️tests/draft-to-review/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦change-status/🧪️tests/draft-to-review/🎯️outcome/🔣️.json"),
        ),
        "add-tag" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️add-tag/🧪️tests/appends-urgent-tag/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️add-tag/🧪️tests/appends-urgent-tag/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️add-tag/🧪️tests/appends-urgent-tag/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️add-tag/🧪️tests/appends-urgent-tag/🎯️outcome/🔣️.json"),
        ),
        "remove-tag" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-tag/🧪️tests/detaches-the-review-tag/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-tag/🧪️tests/detaches-the-review-tag/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-tag/🧪️tests/detaches-the-review-tag/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-tag/🧪️tests/detaches-the-review-tag/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-vcs-1: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_vcs::artifacts::vcs::standards::v1::subsets::any::schema::mutations::{apply_vcs_mutation_reporting, decode_vcs_mutation_json, inverse_vcs_mutation_steps, VcsDemoMutation};
    use semio_s_plugin_vcs::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::{decode_vcs_snapshot_json, encode_vcs_snapshot_json, parse_vcs_dsl, print_vcs_dsl, VcsSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes,
    /// never a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<VcsSnapshot, String> {
        decode_vcs_snapshot_json(text).map_err(|error| format!("mutate-vcs-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<VcsDemoMutation, String> {
        decode_vcs_mutation_json(text).map_err(|error| format!("mutate-vcs-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &VcsSnapshot) -> Result<Json, String> {
        parse_json(&encode_vcs_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &VcsSnapshot, expected: &VcsSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_vcs_snapshot_json(got), encode_vcs_snapshot_json(expected))
    }

    /// 🔢️ The projection's members as `(key, rendered value)` pairs — the granularity the feature's
    /// `moves` column is stated at.
    fn members(projection: &Json) -> Vec<(String, String)> {
        match projection {
            Json::Object(entries) => entries.iter().map(|(key, value)| (key.clone(), value.to_string())).collect(),
            other => vec![(String::new(), other.to_string())],
        }
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law, in the exact form this subset can state it: the ONE member the
    /// feature's `moves` column names must have moved, and every other member must be untouched. A
    /// kind whose forward effect never reached the projection would pass `mutate-<kind>` and
    /// `inverse-<kind>` identically to doing nothing; a kind that reached a NEIGHBOURING member
    /// would pass a whole-document comparison against a fixture written from the same bug.
    fn only_named_member_moved(kind: &str, moves: &str, base: &Json, mutated: &Json) -> Result<(), String> {
        let (before, after) = (members(base), members(mutated));
        if before.len() != after.len() {
            return Err(format!("mutate-{kind}: the mutation changed the projection's member SET ({} members in, {} out) — a checkpoint mutation edits fields, it never adds or drops them", before.len(), after.len()));
        }
        let mut moved = Vec::new();
        for ((key, was), (other, now)) in before.iter().zip(after.iter()) {
            if key != other {
                return Err(format!("mutate-{kind}: the mutation reordered the projection ({key:?} became {other:?})"));
            }
            if was != now {
                moved.push(format!("{key} {was} -> {now}"));
            }
        }
        if !moved.iter().any(|entry| entry.starts_with(&format!("{moves} "))) {
            return Err(format!("mutate-{kind}: the feature declares this kind moves {moves:?}, but that member is unchanged — the scenario would report a pass for a mutation it never observed (moved: {moved:?})"));
        }
        if moved.len() != 1 {
            return Err(format!("mutate-{kind}: the feature declares this kind moves ONLY {moves:?}, but {} member(s) moved: {moved:?}", moved.len()));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: `applied` means the implementation raised nothing, and any
    /// other status means it raised exactly the diagnostic code the fixture names. Checking the
    /// snapshot alone would let a rejection that silently succeeded pass.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[String]) -> Result<(), String> {
        let status = declared.str("status");
        let code = declared.str("code");
        match status.as_str() {
            "applied" => {
                if !raised.is_empty() {
                    return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
                }
                Ok(())
            }
            "" => Err(format!("mutate-{kind}: the committed 🎯️outcome fixture declares no status")),
            _ => {
                if !raised.iter().any(|entry| entry == &code) {
                    return Err(format!("mutate-{kind}: the committed outcome declares {status:?} with code {code:?}, but the implementation raised {raised:?}"));
                }
                Ok(())
            }
        }
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts, in role, that the result
    /// IS the committed after-snapshot, that only the declared member moved, and that the reported
    /// diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_vcs_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            let (was, now) = (projection(&base)?, projection(&current)?);
            only_named_member_moved(kind, &ctx.doc_json()?.str("moves"), &was, &now)?;
            let projection = now;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly — collection POSITION included, not merely
    /// membership, which is what an `add-tag`/`remove-tag` pair has to rebuild rather than
    /// re-append.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_vcs_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"));
            }
            for step in inverse_vcs_mutation_steps(&mutation, &base) {
                apply_vcs_mutation_reporting(&mut current, &step);
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed artifact through its own DSL carrier. `.vcs.dsl.semio` is a
    /// FIXED-LAYOUT record grammar — `dsl::print` emits one line per declared field in schema order
    /// under the `semio vcs.vcs.dsl v1` preamble, with no writer freedom at all — and the committed
    /// example is this codec's own output, committed as such. The wave's usual "output must not
    /// equal input" tripwire therefore does not apply and would be the wrong law here; the
    /// byte-exact law is asserted instead, together with a content check that a parser returning
    /// `VcsSnapshot::default()` (counter 0, status `new`, no tags) cannot satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed checkpoint artifact is not UTF-8: {error}"))?;
        let parsed = parse_vcs_dsl(&text)?;
        if parsed.title != "VCS Demo" || parsed.counter != 2 || parsed.status != "draft" || parsed.tags != vec!["alpha".to_string(), "beta".to_string()] {
            return Err(format!("identity-round-trip: the committed checkpoint is \"VCS Demo\" at counter 2, status draft, tags [alpha, beta], but parsed {}", encode_vcs_snapshot_json(&parsed)));
        }
        let printed = print_vcs_dsl(&parsed);
        let reparsed = parse_vcs_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.vcs.dsl.semio` is a fixed-layout record grammar and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
                printed.len(),
                text.len(),
                match at {
                    Some(offset) => format!(" (first at byte {offset})"),
                    None => String::new(),
                }
            ));
        }
        let projection = projection(&parsed)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// snapshot the oracle role can read literally, but the real artifact is committed as `.dsl.semio`
/// text ONLY, and turning that into a snapshot needs this subset's own codec — which the
/// oracle-only build must not link. Transcribing a JSON copy of it here would be a second, drifting
/// copy of the artifact, so that scenario asserts entirely in-role instead.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
