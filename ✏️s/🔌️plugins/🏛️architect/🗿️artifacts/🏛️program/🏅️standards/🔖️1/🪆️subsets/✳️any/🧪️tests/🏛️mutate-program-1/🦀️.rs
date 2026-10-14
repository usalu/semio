//! 🦀️ Architect program exhaustive mutation case — Rust adapter. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! The subject applies the 14 program mutation kinds over 266 committed vectors and exports each resulting document through the
//! production ZIP serializer. The oracle reads those exact bytes through the approved `zip` crate;
//! it never links the subject implementation. The older Python second implementation remains
//! supplemental evidence, while this carrier reader is the qualifying third-party oracle. The laws
//! this half claims are asserted inside the subject handlers through the shared
//! `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs` module, whose helpers are dependency-free
//! and format-neutral by their own doc comment; the Python half restates them by hand in its own
//! `🔖️Laws` region, because the Python host exposes no `law` module.
//!
//! The evidence rests on the committed `(before, mutation, after, outcome)` specification vector
//! under each of the 266 `🧫️fixtures/🧬️mutations/<domain>/<op>/<case>/` vectors. Those files are read HERE
//! through `asset://`, so the plan pins all 1,064 of their digests and a silently edited vector
//! changes the plan rather than the result. At this vocabulary size that matters more than anywhere
//! else in the repository: the mapping from kind to vector is DATA in the feature's own `Examples`
//! table rather than a 266-arm `match` in this file, which is what keeps the adapter a page long
//! instead of a thousand lines of `include_str!` that no reviewer would read.
//!
//! This subset's mutation facet is dispatch-only — it never carried an apply helper, because every
//! in-crate caller goes through `store::ArtifactStore`, which an external host cannot construct.
//! `apply_program_mutation_outcome`/`inverse_program_mutation_steps` beside the enum are the seam
//! this case needed and the only production surface it added.

use semio_repo_test_host::Adapter;

fn archive_projection(mut entries: Vec<(String, Vec<u8>)>) -> Result<semio_repo_test_host::Json, String> {
    use semio_repo_test_host::{parse_json, Json};
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let projected = entries
        .into_iter()
        .map(|(name, bytes)| {
            let text = String::from_utf8(bytes).map_err(|error| format!("{name} is not UTF-8 JSON: {error}"))?;
            Ok(Json::Object(vec![("name".into(), Json::String(name)), ("rows".into(), parse_json(&text)?)]))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Json::Object(vec![("format".into(), Json::String("architect-program-registers-zip-v1".into())), ("entries".into(), Json::Array(projected))]))
}

fn archive_oracle(ctx: &semio_repo_test_host::Context) -> Result<semio_repo_test_host::Outcome, String> {
    let raw = ctx.subject_raw_bytes("rust")?;
    let entries = semio_s_plugin_stdio_archive_test_oracle::read_zip_entries(&raw)?.into_iter().map(|entry| (entry.name, entry.bytes)).collect();
    Ok(semio_repo_test_host::Outcome::with_raw(raw, archive_projection(entries)?))
}

//#region 🔖️Kinds
/// 🏷️ One scenario id per committed vector the feature's `Examples` tables address: `<kind>-<entity>`
/// for the four generic register-row kinds (one vector per register) and the kind itself for the edge
/// and facet kinds of `ProgramMutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`). Duplicated, not
/// imported, because the oracle-only build must not link the subject crate; the contract gate keeps
/// this list honest against the feature.
const KINDS: &[&str] = &[
    "create-register-row-information-requirement",
    "delete-register-row-information-requirement",
    "rename-register-row-information-requirement",
    "replace-register-row-information-requirement",
    "create-register-row-sustainability-requirement",
    "delete-register-row-sustainability-requirement",
    "rename-register-row-sustainability-requirement",
    "replace-register-row-sustainability-requirement",
    "create-register-row-accessibility-requirement",
    "delete-register-row-accessibility-requirement",
    "rename-register-row-accessibility-requirement",
    "replace-register-row-accessibility-requirement",
    "create-register-row-conflict",
    "delete-register-row-conflict",
    "rename-register-row-conflict",
    "replace-register-row-conflict",
    "create-register-row-option-evaluation",
    "delete-register-row-option-evaluation",
    "rename-register-row-option-evaluation",
    "replace-register-row-option-evaluation",
    "create-register-row-function",
    "delete-register-row-function",
    "rename-register-row-function",
    "replace-register-row-function",
    "create-register-row-risk",
    "delete-register-row-risk",
    "rename-register-row-risk",
    "replace-register-row-risk",
    "create-register-row-decision",
    "delete-register-row-decision",
    "rename-register-row-decision",
    "replace-register-row-decision",
    "create-register-row-validation-record",
    "delete-register-row-validation-record",
    "rename-register-row-validation-record",
    "replace-register-row-validation-record",
    "create-register-row-priority-record",
    "delete-register-row-priority-record",
    "rename-register-row-priority-record",
    "replace-register-row-priority-record",
    "create-register-row-flow-requirement",
    "delete-register-row-flow-requirement",
    "rename-register-row-flow-requirement",
    "replace-register-row-flow-requirement",
    "create-register-row-environmental-requirement",
    "delete-register-row-environmental-requirement",
    "rename-register-row-environmental-requirement",
    "replace-register-row-environmental-requirement",
    "create-register-row-workshop",
    "delete-register-row-workshop",
    "rename-register-row-workshop",
    "replace-register-row-workshop",
    "create-register-row-scenario",
    "delete-register-row-scenario",
    "rename-register-row-scenario",
    "replace-register-row-scenario",
    "create-register-row-benchmark-record",
    "delete-register-row-benchmark-record",
    "rename-register-row-benchmark-record",
    "replace-register-row-benchmark-record",
    "create-register-row-activity",
    "delete-register-row-activity",
    "rename-register-row-activity",
    "replace-register-row-activity",
    "create-register-row-infrastructure-requirement",
    "delete-register-row-infrastructure-requirement",
    "rename-register-row-infrastructure-requirement",
    "replace-register-row-infrastructure-requirement",
    "create-register-row-organizational-requirement",
    "delete-register-row-organizational-requirement",
    "rename-register-row-organizational-requirement",
    "replace-register-row-organizational-requirement",
    "create-register-row-issue",
    "delete-register-row-issue",
    "rename-register-row-issue",
    "replace-register-row-issue",
    "create-register-row-approval-record",
    "delete-register-row-approval-record",
    "rename-register-row-approval-record",
    "replace-register-row-approval-record",
    "create-register-row-stakeholder",
    "delete-register-row-stakeholder",
    "rename-register-row-stakeholder",
    "replace-register-row-stakeholder",
    "create-register-row-quality-record",
    "delete-register-row-quality-record",
    "rename-register-row-quality-record",
    "replace-register-row-quality-record",
    "create-register-row-resilience-requirement",
    "delete-register-row-resilience-requirement",
    "rename-register-row-resilience-requirement",
    "replace-register-row-resilience-requirement",
    "create-register-row-assumption",
    "delete-register-row-assumption",
    "rename-register-row-assumption",
    "replace-register-row-assumption",
    "create-register-row-cost-requirement",
    "delete-register-row-cost-requirement",
    "rename-register-row-cost-requirement",
    "replace-register-row-cost-requirement",
    "create-register-row-document",
    "delete-register-row-document",
    "rename-register-row-document",
    "replace-register-row-document",
    "create-register-row-schedule-requirement",
    "delete-register-row-schedule-requirement",
    "rename-register-row-schedule-requirement",
    "replace-register-row-schedule-requirement",
    "create-register-row-growth-plan",
    "delete-register-row-growth-plan",
    "rename-register-row-growth-plan",
    "replace-register-row-growth-plan",
    "create-register-row-performance-criterion",
    "delete-register-row-performance-criterion",
    "rename-register-row-performance-criterion",
    "replace-register-row-performance-criterion",
    "create-register-row-operational-requirement",
    "delete-register-row-operational-requirement",
    "rename-register-row-operational-requirement",
    "replace-register-row-operational-requirement",
    "create-register-row-requirement",
    "delete-register-row-requirement",
    "rename-register-row-requirement",
    "replace-register-row-requirement",
    "create-register-row-site-context",
    "delete-register-row-site-context",
    "rename-register-row-site-context",
    "replace-register-row-site-context",
    "create-register-row-template-record",
    "delete-register-row-template-record",
    "rename-register-row-template-record",
    "replace-register-row-template-record",
    "create-register-row-report-record",
    "delete-register-row-report-record",
    "rename-register-row-report-record",
    "replace-register-row-report-record",
    "create-register-row-audit-event",
    "delete-register-row-audit-event",
    "rename-register-row-audit-event",
    "replace-register-row-audit-event",
    "create-register-row-knowledge-record",
    "delete-register-row-knowledge-record",
    "rename-register-row-knowledge-record",
    "replace-register-row-knowledge-record",
    "create-register-row-regulatory-requirement",
    "delete-register-row-regulatory-requirement",
    "rename-register-row-regulatory-requirement",
    "replace-register-row-regulatory-requirement",
    "create-register-row-communication-requirement",
    "delete-register-row-communication-requirement",
    "rename-register-row-communication-requirement",
    "replace-register-row-communication-requirement",
    "create-register-row-resource",
    "delete-register-row-resource",
    "rename-register-row-resource",
    "replace-register-row-resource",
    "create-register-row-status-record",
    "delete-register-row-status-record",
    "rename-register-row-status-record",
    "replace-register-row-status-record",
    "create-register-row-change-record",
    "delete-register-row-change-record",
    "rename-register-row-change-record",
    "replace-register-row-change-record",
    "create-register-row-process",
    "delete-register-row-process",
    "rename-register-row-process",
    "replace-register-row-process",
    "create-register-row-search-filter",
    "delete-register-row-search-filter",
    "rename-register-row-search-filter",
    "replace-register-row-search-filter",
    "create-register-row-access-rule",
    "delete-register-row-access-rule",
    "rename-register-row-access-rule",
    "replace-register-row-access-rule",
    "create-register-row-privacy-requirement",
    "delete-register-row-privacy-requirement",
    "rename-register-row-privacy-requirement",
    "replace-register-row-privacy-requirement",
    "create-register-row-quantity-requirement",
    "delete-register-row-quantity-requirement",
    "rename-register-row-quantity-requirement",
    "replace-register-row-quantity-requirement",
    "create-register-row-analysis-record",
    "delete-register-row-analysis-record",
    "rename-register-row-analysis-record",
    "replace-register-row-analysis-record",
    "create-register-row-relationship",
    "delete-register-row-relationship",
    "rename-register-row-relationship",
    "replace-register-row-relationship",
    "create-register-row-storage-requirement",
    "delete-register-row-storage-requirement",
    "rename-register-row-storage-requirement",
    "replace-register-row-storage-requirement",
    "create-register-row-meeting-record",
    "delete-register-row-meeting-record",
    "rename-register-row-meeting-record",
    "replace-register-row-meeting-record",
    "create-register-row-survey",
    "delete-register-row-survey",
    "rename-register-row-survey",
    "replace-register-row-survey",
    "create-register-row-delivery-constraint",
    "delete-register-row-delivery-constraint",
    "rename-register-row-delivery-constraint",
    "replace-register-row-delivery-constraint",
    "create-register-row-constraint-record",
    "delete-register-row-constraint-record",
    "rename-register-row-constraint-record",
    "replace-register-row-constraint-record",
    "create-register-row-compliance-record",
    "delete-register-row-compliance-record",
    "rename-register-row-compliance-record",
    "replace-register-row-compliance-record",
    "create-register-row-service-requirement",
    "delete-register-row-service-requirement",
    "rename-register-row-service-requirement",
    "replace-register-row-service-requirement",
    "create-register-row-equipment",
    "delete-register-row-equipment",
    "rename-register-row-equipment",
    "replace-register-row-equipment",
    "create-register-row-security-requirement",
    "delete-register-row-security-requirement",
    "rename-register-row-security-requirement",
    "replace-register-row-security-requirement",
    "create-register-row-collaboration-record",
    "delete-register-row-collaboration-record",
    "rename-register-row-collaboration-record",
    "replace-register-row-collaboration-record",
    "create-register-row-safety-requirement",
    "delete-register-row-safety-requirement",
    "rename-register-row-safety-requirement",
    "replace-register-row-safety-requirement",
    "create-register-row-user-profile",
    "delete-register-row-user-profile",
    "rename-register-row-user-profile",
    "replace-register-row-user-profile",
    "create-register-row-human-factor-requirement",
    "delete-register-row-human-factor-requirement",
    "rename-register-row-human-factor-requirement",
    "replace-register-row-human-factor-requirement",
    "create-register-row-flexibility-requirement",
    "delete-register-row-flexibility-requirement",
    "rename-register-row-flexibility-requirement",
    "replace-register-row-flexibility-requirement",
    "create-register-row-wayfinding-requirement",
    "delete-register-row-wayfinding-requirement",
    "rename-register-row-wayfinding-requirement",
    "replace-register-row-wayfinding-requirement",
    "create-register-row-program-element",
    "delete-register-row-program-element",
    "rename-register-row-program-element",
    "replace-register-row-program-element",
    "connect-adjacency",
    "disconnect-adjacency",
    "connect-trace",
    "disconnect-trace",
    "rename-meta",
    "replace-meta",
    "rename-project",
    "replace-project",
    "rename-governance",
    "replace-governance",
];

#[cfg(feature = "sut")]
/// 👁️ The six scenarios whose committed vector pins a REJECTION rather than an effect, so
/// `before` and `after` are the same document and the observability law cannot hold.
/// `knowledge` and `benchmarks` are the only two of this subset's 66 registers that are composed
/// `s.stdio.semio.table` CHILD handles: their rows live in a working-scene cache that a fresh
/// process has never populated, so every id is absent and the only reachable branch is
/// `mutation.target-missing`. Naming them here is a claim the reader can check against the vectors;
/// the feature description states the same, and their `mutate` scenarios still assert the declared
/// status and diagnostic code.
const GUARD_VECTORS: &[&str] = &["delete-register-row-benchmark-record", "rename-register-row-benchmark-record", "replace-register-row-benchmark-record", "delete-register-row-knowledge-record", "rename-register-row-knowledge-record", "replace-register-row-knowledge-record"];

#[cfg(feature = "sut")]
/// 🧫️ Where a `<vector>` cell from the feature's `Examples` tables is rooted, relative to this
/// case's owner — the artifact root, which is what `asset://` resolves against.
const VECTORS: &str = "shared://🧬️mutations";

#[cfg(feature = "sut")]
/// 📄️ The real committed example document, in this subset's own `.dsl.semio` text envelope.
const EXAMPLE_ASSET: &str = "asset://🎬️demo/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::export::serializers::artifacts::zip::v2_0::any as export_zip;
    use semio_s_artifact_architect_program::standards::v1::subsets::any::schema::mutations::{inverse_program_mutation_steps,ProgramMutation};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::apply_program_mutation_outcome;

    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::text::snapshot::{encode_program_snapshot_json};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::text::snapshot::{decode_program_snapshot_json};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::text::mutations::{decode_program_mutation_json};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::text::snapshot::{print_program_dsl};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::io::text::snapshot::{parse_program_dsl};
    use semio_s_artifact_architect_program::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;
    use semio_repo_test_host::law;

    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        loop {
            match future.as_mut().poll(&mut context) {
                std::task::Poll::Ready(output) => return output,
                std::task::Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn carrier(snapshot: &ProgramSnapshot) -> Result<(Vec<u8>, Json), String> {
        let archive = block_on(export_zip::serialize(snapshot)).map_err(|error| format!("program ZIP export failed: {error}"))?;
        let projection = super::archive_projection(archive.entries.iter().map(|entry| (entry.name.clone(), entry.data.clone())).collect())?;
        let raw = block_on(export_zip::serialize_raw_bytes(snapshot)).map_err(|error| format!("program ZIP encoding failed: {error}"))?;
        Ok((raw, projection))
    }

    //#region 🔖️VectorReading
    /// 🧫️ The scenario's own doc string, which carries the kind and the committed vector directory
    /// the row addresses. The mapping from kind to vector is DATA in the feature's `Examples` table,
    /// so this adapter holds no table of its own that could drift from it.
    fn addressed(ctx: &Context) -> Result<(String, String, Json), String> {
        let spec = ctx.doc_json()?;
        let (kind, vector) = (spec.str("kind"), spec.str("vector"));
        if kind.is_empty() || vector.is_empty() {
            return Err(format!("the scenario doc string must carry both a \"kind\" and a \"vector\", got {}", spec.to_string()));
        }
        Ok((kind, vector, spec))
    }

    fn text_at(ctx: &Context, vector: &str, leaf: &str) -> Result<String, String> {
        let uri = format!("{}/{vector}/{leaf}", super::VECTORS);
        String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed vector {uri} is not UTF-8: {error}"))
    }

    fn snapshot_at(ctx: &Context, vector: &str, leaf: &str, kind: &str) -> Result<ProgramSnapshot, String> {
        decode_program_snapshot_json(&text_at(ctx, vector, leaf)?).map_err(|error| format!("the committed {leaf} vector for {kind:?} must decode: {error}"))
    }

    fn mutation_at(ctx: &Context, vector: &str, kind: &str) -> Result<ProgramMutation, String> {
        decode_program_mutation_json(&text_at(ctx, vector, "🦠️mutation/🔣️.json")?).map_err(|error| format!("the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &ProgramSnapshot) -> Result<Json, String> {
        parse_json(&encode_program_snapshot_json(snapshot))
    }

    /// 🚨️ The `mutation.*` codes a committed `🎯️outcome` vector declares. Two shapes are in use and
    /// both are read here rather than one being assumed: an APPLIED outcome carries its diagnostics
    /// in a `messages` array, a REJECTED one carries the single `code` that refused it.
    fn declared_codes(outcome: &Json) -> Vec<String> {
        let listed: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).filter(|code| !code.is_empty()).collect();
        if !listed.is_empty() {
            return listed;
        }
        match outcome.str("code").as_str() {
            "" => Vec::new(),
            single => vec![single.to_string()],
        }
    }
    //#endregion 🔖️VectorReading

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts three things in role: the
    /// result IS the committed after-snapshot, the raised diagnostic codes ARE the ones the
    /// committed `🎯️outcome` vector declares, and — for every kind not named in `GUARD_VECTORS` —
    /// the mutation actually moved the compared projection.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, _spec) = addressed(ctx)?;
        let base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let expected = snapshot_at(ctx, &vector, "📸️snapshot/➡️after/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let declared = parse_json(&text_at(ctx, &vector, "🎯️outcome/🔣️.json")?)?;
        let (current, outcome) = apply_program_mutation_outcome(&base, &mutation);
        let raised: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
        if raised != declared_codes(&declared) {
            return Err(format!("mutate-{kind}: raised {raised:?}, the committed 🎯️outcome vector declares {:?}", declared_codes(&declared)));
        }
        let (produced, wanted, before) = (projection(&current)?, projection(&expected)?, projection(&base)?);
        if let Some(first) = law::divergence(&produced, &wanted) {
            return Err(format!("mutate-{kind}: the applied snapshot is not the committed after-snapshot — {first}"));
        }
        law::mutation_is_observable(&kind, &produced, &before, super::GUARD_VECTORS)?;
        let (raw, carrier_after) = carrier(&current)?;
        let (_, carrier_before) = carrier(&base)?;
        law::mutation_is_observable(&kind, &carrier_after, &carrier_before, super::GUARD_VECTORS)?;
        Ok(Outcome::with_raw(raw, carrier_after))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse
    /// must land back on the committed before-snapshot's projection, field for field, with no
    /// tolerance and no ignored key.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, _spec) = addressed(ctx)?;
        let base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let original = projection(&base)?;
        let (mut current, _) = apply_program_mutation_outcome(&base, &mutation);
        for step in inverse_program_mutation_steps(&mutation, &base).expect("valid retained mutation inverse fixture") {
            current = apply_program_mutation_outcome(&current, &step).0;
        }
        let restored = projection(&current)?;
        law::inverse_restores(&kind, &restored, &original)?;
        let (raw, carrier_restored) = carrier(&current)?;
        let (_, carrier_original) = carrier(&base)?;
        law::inverse_restores(&kind, &carrier_restored, &carrier_original)?;
        Ok(Outcome::with_raw(raw, carrier_restored))
    }

    /// 🔁️ The real committed artifact, parsed and printed back. Two laws, both in role: the
    /// reparsed document must carry the same projection, and the printed text must reproduce the
    /// committed bytes EXACTLY. The exact-bytes half is `carrier_is_exact` rather than the wave's
    /// usual no-byte-pass-through tripwire because the committed `🗣️.dsl.semio` is this
    /// subset's OWN printer's output — the repository generated it from this very codec — so
    /// reproducing it is the correct answer and any drift between the committed artifact and the
    /// printer is the defect this scenario exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.input_bytes(super::EXAMPLE_ASSET)?;
        let text = String::from_utf8(committed.clone()).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_program_dsl(&text)?;
        let printed = print_program_dsl(&parsed);
        let reparsed = parse_program_dsl(&printed)?;
        let (before, after) = (projection(&parsed)?, projection(&reparsed)?);
        law::round_trip_preserves(&after, &before)?;
        law::carrier_is_exact(printed.as_bytes(), &committed)?;
        let (raw, carrier_after) = carrier(&reparsed)?;
        let (_, carrier_before) = carrier(&parsed)?;
        law::round_trip_preserves(&carrier_after, &carrier_before)?;
        Ok(Outcome::with_raw(raw, carrier_after))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's two `Examples` tables exactly.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), archive_oracle).oracle(&format!("inverse-{kind}"), archive_oracle);
    }
    built = built.oracle("identity-round-trip", archive_oracle);
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
