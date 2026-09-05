//! 🦀️ Forms document exhaustive mutation case — Rust adapter. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `forms-document-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): `s.forms.form` is a
//! semio-NATIVE artifact with no third-party reader or writer, so this case registers SUBJECT
//! handlers only. That is not an omission — the runner resolves an oracle implementation from the
//! feature's `@oracle-` tag, this feature carries `@no-oracle-` instead, and the oracle role is
//! therefore never dispatched for it. Registering an oracle handler here would be dead code that
//! reads as coverage in every listing, so there is none; every law this case claims is asserted
//! inside the subject handlers, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module.
//!
//! ⚠️ What this case can and cannot prove is set by the vectors it has. Nine of the ten pin a
//! rejection or a no-op branch, because `FormsSnapshot` keeps its steps tree behind
//! content-addressed child handles whose successful re-mint is a `DefaultHasher` digest no one can
//! hand-author. So the forward assertion here is the committed `🎯️outcome`'s status AND code, not
//! a moved projection, and `GUARD_VECTORS` names all nine. The inverse law is unexempted for all
//! ten: a refused mutation's inverse is EMPTY, and applying nothing must still land on the
//! before-snapshot, which is a real check that a spuriously non-empty inverse would fail.
//!
//! The `scene` cell each row carries is the working-scene half of that row's before-state,
//! transcribed from the same leaf's own `🧪️tests/<fixture>/🦀️.rs::before()`. It is the one
//! thing in this adapter that is not read from a digest-pinned committed file, and it is in the
//! feature as visible data rather than a per-kind `match` here for exactly that reason.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
#[cfg(feature = "sut")]
/// 🏷️ Mirrors `FormMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The production module's
/// own `kinds_match_the_enum_and_the_catalog` keeps that list honest against the enum and the
/// catalog; the contract gate keeps this case honest against the catalog.
const KINDS: &[&str] = &[
    "create-step",
    "delete-step",
    "reorder-step",
    "rename-step",
    "change-step-description",
    "create-block",
    "delete-block",
    "move-block-to-step",
    "replace-block",
    "change-form-title",
];

#[cfg(feature = "sut")]
/// 👁️ The nine kinds whose committed vector pins a REJECTION or NO-OP branch rather than
/// an effect, so `before` and `after` are the same document and the observability law cannot hold.
/// The reason is structural and is stated in the leaves themselves: a successful forms mutation
/// re-mints the content-addressed `structure`/`results` child handles from a `DefaultHasher` digest,
/// and that value cannot be hand-authored into a committed `➡️after`. Naming them here is a claim
/// the reader can check against the vectors, and the feature description states the same. Only
/// `change-form-title`, the one kind that touches a persisted scalar, carries an effect vector.
const GUARD_VECTORS: &[&str] = &[
    "create-step",
    "delete-step",
    "reorder-step",
    "rename-step",
    "change-step-description",
    "create-block",
    "delete-block",
    "move-block-to-step",
    "replace-block",
];

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
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_forms::artifacts::forms::standards::v1::subsets::any::io::snapshot::text::{parse_form_dsl, print_form_dsl};
    use semio_s_plugin_forms::artifacts::forms::standards::v1::subsets::any::schema::mutations::{apply_form_mutation_outcome, decode_form_mutation_json, decode_form_snapshot_json, encode_form_snapshot_json, inverse_form_mutation_steps, seed_form_scene_json, FormMutation};
    use semio_s_plugin_forms::artifacts::forms::FormsSnapshot;

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

    fn snapshot_at(ctx: &Context, vector: &str, leaf: &str, kind: &str) -> Result<FormsSnapshot, String> {
        decode_form_snapshot_json(&text_at(ctx, vector, leaf)?).map_err(|error| format!("the committed {leaf} vector for {kind:?} must decode: {error}"))
    }

    fn mutation_at(ctx: &Context, vector: &str, kind: &str) -> Result<FormMutation, String> {
        decode_form_mutation_json(&text_at(ctx, vector, "🦠️mutation/🔣️.json")?).map_err(|error| format!("the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &FormsSnapshot) -> Result<Json, String> {
        parse_json(&encode_form_snapshot_json(snapshot))
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

    //#region 🔖️WorkingScene
    /// 🌱 Seeds the composed child's working scene from the row's own `scene` cell before the
    /// mutation runs. The persisted `⬅️before` vector carries only the child HANDLE, so without this
    /// half the scene is empty, every addressed id is absent, and every kind would collapse onto the
    /// same `mutation.target-missing` branch — which would make the case look green while testing
    /// one code path nine times. The cell is DATA in the feature's `Examples` table, quoted there
    /// with the leaf test it was read from, rather than a per-kind `match` hidden in this file.
    fn seed(snapshot: &mut FormsSnapshot, spec: &Json) -> Result<(), String> {
        let scene = match spec.get("scene") {
            Some(value) => value.to_string(),
            None => return Err("the scenario doc string must carry a \"scene\" array for this subset".to_string()),
        };
        seed_form_scene_json(snapshot, &scene).map(|_| ())
    }
    //#endregion 🔖️WorkingScene

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts three things in role: the
    /// result IS the committed after-snapshot, the raised diagnostic codes ARE the ones the
    /// committed `🎯️outcome` vector declares, and — for every kind not named in `GUARD_VECTORS` —
    /// the mutation actually moved the compared projection.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, spec) = addressed(ctx)?;
        let mut base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let expected = snapshot_at(ctx, &vector, "📸️snapshot/➡️after/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let declared = parse_json(&text_at(ctx, &vector, "🎯️outcome/🔣️.json")?)?;
        seed(&mut base, &spec)?;
        let mut current = base.clone();
        let outcome = apply_form_mutation_outcome(&mut current, &mutation);
        let raised: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
        if raised != declared_codes(&declared) {
            return Err(format!("mutate-{kind}: raised {raised:?}, the committed 🎯️outcome vector declares {:?}", declared_codes(&declared)));
        }
        let (produced, wanted, before) = (projection(&current)?, projection(&expected)?, projection(&base)?);
        if let Some(first) = law::divergence(&produced, &wanted) {
            return Err(format!("mutate-{kind}: the applied snapshot is not the committed after-snapshot — {first}"));
        }
        law::mutation_is_observable(&kind, &produced, &before, super::GUARD_VECTORS)?;
        Ok(Outcome::with_raw(produced.to_string().into_bytes(), produced))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse
    /// must land back on the committed before-snapshot's projection, field for field, with no
    /// tolerance and no ignored key.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, spec) = addressed(ctx)?;
        let mut base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        seed(&mut base, &spec)?;
        let original = projection(&base)?;
        let mut current = base.clone();
        apply_form_mutation_outcome(&mut current, &mutation);
        for step in inverse_form_mutation_steps(&mutation, &base) {
            apply_form_mutation_outcome(&mut current, &step);
        }
        let restored = projection(&current)?;
        law::inverse_restores(&kind, &restored, &original)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
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
        let parsed = parse_form_dsl(&text)?;
        let printed = print_form_dsl(&parsed);
        let reparsed = parse_form_dsl(&printed)?;
        let (before, after) = (projection(&parsed)?, projection(&reparsed)?);
        law::round_trip_preserves(&after, &before)?;
        law::carrier_is_exact(printed.as_bytes(), &committed)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
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
