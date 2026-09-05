//! 🦀️ Architect program exhaustive mutation case — Rust adapter. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! The subject applies the 266 program mutations and exports each resulting document through the
//! production ZIP serializer. The oracle reads those exact bytes through the approved `zip` crate;
//! it never links the subject implementation. The older Python second implementation remains
//! supplemental evidence, while this carrier reader is the qualifying third-party oracle. The laws
//! this half claims are asserted inside the subject handlers through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module, whose helpers are dependency-free
//! and format-neutral by their own doc comment; the Python half restates them by hand in its own
//! `🔖️Laws` region, because the Python host exposes no `law` module.
//!
//! The evidence rests on the committed `(before, mutation, after, outcome)` specification vector
//! under each of the 266 `🧬️mutations/<slug>/🧪️tests/<fixture>/` leaves. Those files are read HERE
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
    let entries = semio_s_plugin_stdio_test_oracle::archive::read_zip_entries(&raw)?.into_iter().map(|entry| (entry.name, entry.bytes)).collect();
    Ok(semio_repo_test_host::Outcome::with_raw(raw, archive_projection(entries)?))
}

//#region 🔖️Kinds
/// 🏷️ Mirrors `ProgramMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The production module's
/// own `kinds_match_the_enum_and_the_catalog` keeps that list honest against the enum and the
/// catalog; the contract gate keeps this case honest against the catalog.
const KINDS: &[&str] = &[
    "create-information-requirement",
    "delete-information-requirement",
    "rename-information-requirement",
    "replace-information-requirement",
    "create-sustainability-requirement",
    "delete-sustainability-requirement",
    "rename-sustainability-requirement",
    "replace-sustainability-requirement",
    "create-accessibility-requirement",
    "delete-accessibility-requirement",
    "rename-accessibility-requirement",
    "replace-accessibility-requirement",
    "create-conflict",
    "delete-conflict",
    "rename-conflict",
    "replace-conflict",
    "create-option-evaluation",
    "delete-option-evaluation",
    "rename-option-evaluation",
    "replace-option-evaluation",
    "create-function",
    "delete-function",
    "rename-function",
    "replace-function",
    "create-risk",
    "delete-risk",
    "rename-risk",
    "replace-risk",
    "create-decision",
    "delete-decision",
    "rename-decision",
    "replace-decision",
    "create-validation-record",
    "delete-validation-record",
    "rename-validation-record",
    "replace-validation-record",
    "create-priority-record",
    "delete-priority-record",
    "rename-priority-record",
    "replace-priority-record",
    "create-flow-requirement",
    "delete-flow-requirement",
    "rename-flow-requirement",
    "replace-flow-requirement",
    "create-environmental-requirement",
    "delete-environmental-requirement",
    "rename-environmental-requirement",
    "replace-environmental-requirement",
    "create-workshop",
    "delete-workshop",
    "rename-workshop",
    "replace-workshop",
    "create-scenario",
    "delete-scenario",
    "rename-scenario",
    "replace-scenario",
    "create-benchmark-record",
    "delete-benchmark-record",
    "rename-benchmark-record",
    "replace-benchmark-record",
    "create-activity",
    "delete-activity",
    "rename-activity",
    "replace-activity",
    "create-infrastructure-requirement",
    "delete-infrastructure-requirement",
    "rename-infrastructure-requirement",
    "replace-infrastructure-requirement",
    "create-organizational-requirement",
    "delete-organizational-requirement",
    "rename-organizational-requirement",
    "replace-organizational-requirement",
    "create-issue",
    "delete-issue",
    "rename-issue",
    "replace-issue",
    "create-approval-record",
    "delete-approval-record",
    "rename-approval-record",
    "replace-approval-record",
    "create-stakeholder",
    "delete-stakeholder",
    "rename-stakeholder",
    "replace-stakeholder",
    "create-quality-record",
    "delete-quality-record",
    "rename-quality-record",
    "replace-quality-record",
    "create-resilience-requirement",
    "delete-resilience-requirement",
    "rename-resilience-requirement",
    "replace-resilience-requirement",
    "create-assumption",
    "delete-assumption",
    "rename-assumption",
    "replace-assumption",
    "create-cost-requirement",
    "delete-cost-requirement",
    "rename-cost-requirement",
    "replace-cost-requirement",
    "create-document",
    "delete-document",
    "rename-document",
    "replace-document",
    "create-schedule-requirement",
    "delete-schedule-requirement",
    "rename-schedule-requirement",
    "replace-schedule-requirement",
    "create-growth-plan",
    "delete-growth-plan",
    "rename-growth-plan",
    "replace-growth-plan",
    "create-performance-criterion",
    "delete-performance-criterion",
    "rename-performance-criterion",
    "replace-performance-criterion",
    "create-operational-requirement",
    "delete-operational-requirement",
    "rename-operational-requirement",
    "replace-operational-requirement",
    "create-requirement",
    "delete-requirement",
    "rename-requirement",
    "replace-requirement",
    "create-site-context",
    "delete-site-context",
    "rename-site-context",
    "replace-site-context",
    "create-template-record",
    "delete-template-record",
    "rename-template-record",
    "replace-template-record",
    "create-report-record",
    "delete-report-record",
    "rename-report-record",
    "replace-report-record",
    "create-audit-event",
    "delete-audit-event",
    "rename-audit-event",
    "replace-audit-event",
    "create-knowledge-record",
    "delete-knowledge-record",
    "rename-knowledge-record",
    "replace-knowledge-record",
    "create-regulatory-requirement",
    "delete-regulatory-requirement",
    "rename-regulatory-requirement",
    "replace-regulatory-requirement",
    "create-change-record",
    "delete-change-record",
    "rename-change-record",
    "replace-change-record",
    "create-communication-requirement",
    "delete-communication-requirement",
    "rename-communication-requirement",
    "replace-communication-requirement",
    "create-resource",
    "delete-resource",
    "rename-resource",
    "replace-resource",
    "create-status-record",
    "delete-status-record",
    "rename-status-record",
    "replace-status-record",
    "create-process",
    "delete-process",
    "rename-process",
    "replace-process",
    "create-search-filter",
    "delete-search-filter",
    "rename-search-filter",
    "replace-search-filter",
    "create-access-rule",
    "delete-access-rule",
    "rename-access-rule",
    "replace-access-rule",
    "create-privacy-requirement",
    "delete-privacy-requirement",
    "rename-privacy-requirement",
    "replace-privacy-requirement",
    "create-relationship",
    "delete-relationship",
    "rename-relationship",
    "replace-relationship",
    "create-quantity-requirement",
    "delete-quantity-requirement",
    "rename-quantity-requirement",
    "replace-quantity-requirement",
    "create-analysis-record",
    "delete-analysis-record",
    "rename-analysis-record",
    "replace-analysis-record",
    "create-storage-requirement",
    "delete-storage-requirement",
    "rename-storage-requirement",
    "replace-storage-requirement",
    "create-meeting-record",
    "delete-meeting-record",
    "rename-meeting-record",
    "replace-meeting-record",
    "create-survey",
    "delete-survey",
    "rename-survey",
    "replace-survey",
    "create-delivery-constraint",
    "delete-delivery-constraint",
    "rename-delivery-constraint",
    "replace-delivery-constraint",
    "create-constraint-record",
    "delete-constraint-record",
    "rename-constraint-record",
    "replace-constraint-record",
    "create-compliance-record",
    "delete-compliance-record",
    "rename-compliance-record",
    "replace-compliance-record",
    "create-service-requirement",
    "delete-service-requirement",
    "rename-service-requirement",
    "replace-service-requirement",
    "create-equipment",
    "delete-equipment",
    "rename-equipment",
    "replace-equipment",
    "create-security-requirement",
    "delete-security-requirement",
    "rename-security-requirement",
    "replace-security-requirement",
    "create-collaboration-record",
    "delete-collaboration-record",
    "rename-collaboration-record",
    "replace-collaboration-record",
    "create-safety-requirement",
    "delete-safety-requirement",
    "rename-safety-requirement",
    "replace-safety-requirement",
    "create-user-profile",
    "delete-user-profile",
    "rename-user-profile",
    "replace-user-profile",
    "create-human-factor-requirement",
    "delete-human-factor-requirement",
    "rename-human-factor-requirement",
    "replace-human-factor-requirement",
    "create-flexibility-requirement",
    "delete-flexibility-requirement",
    "rename-flexibility-requirement",
    "replace-flexibility-requirement",
    "create-wayfinding-requirement",
    "delete-wayfinding-requirement",
    "rename-wayfinding-requirement",
    "replace-wayfinding-requirement",
    "create-program-element",
    "delete-program-element",
    "rename-program-element",
    "replace-program-element",
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
/// 👁️ The six kinds whose committed vector pins a REJECTION rather than an effect, so
/// `before` and `after` are the same document and the observability law cannot hold.
/// `knowledge` and `benchmarks` are the only two of this subset's 66 registers that are composed
/// `s.stdio.semio.table` CHILD handles: their rows live in a working-scene cache that a fresh
/// process has never populated, so every id is absent and the only reachable branch is
/// `mutation.target-missing`. Naming them here is a claim the reader can check against the vectors;
/// the feature description states the same, and their `mutate` scenarios still assert the declared
/// status and diagnostic code.
const GUARD_VECTORS: &[&str] = &["delete-benchmark-record", "rename-benchmark-record", "replace-benchmark-record", "delete-knowledge-record", "rename-knowledge-record", "replace-knowledge-record"];

#[cfg(feature = "sut")]
/// 🧫️ Where a `<vector>` cell from the feature's `Examples` tables is rooted, relative to this
/// case's owner — the artifact root, which is what `asset://` resolves against.
const VECTORS: &str = "asset://🧬️schema/🧬️mutations";

#[cfg(feature = "sut")]
/// 📄️ The real committed example document, in this subset's own `.dsl.semio` text envelope.
const EXAMPLE_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_architect::artifacts::program::io::export::serializers::artifacts::zip::v2_0::any as export_zip;
    use semio_s_plugin_architect::artifacts::program::standards::v1::subsets::any::schema::mutations::{
        apply_program_mutation_outcome, decode_program_mutation_json, decode_program_snapshot_json, encode_program_snapshot_json, inverse_program_mutation_steps, ProgramMutation,
    };
    use semio_s_plugin_architect::artifacts::program::standards::v1::subsets::any::schema::snapshot::{parse_program_dsl, print_program_dsl};
    use semio_s_plugin_architect::artifacts::program::ProgramSnapshot;
    use semio_s_plugin_stdio_test_oracle::law;

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
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the committed vector {uri} is not UTF-8: {error}"))
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
        let mut current = base.clone();
        let outcome = apply_program_mutation_outcome(&mut current, &mutation);
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
        let mut current = base.clone();
        apply_program_mutation_outcome(&mut current, &mutation);
        for step in inverse_program_mutation_steps(&mutation, &base) {
            apply_program_mutation_outcome(&mut current, &step);
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
        let committed = ctx.fixture_bytes(super::EXAMPLE_ASSET)?;
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
