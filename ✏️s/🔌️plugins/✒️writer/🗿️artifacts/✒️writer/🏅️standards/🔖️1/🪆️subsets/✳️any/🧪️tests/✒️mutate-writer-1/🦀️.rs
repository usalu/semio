//! 🦀️ Writer document exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `writer-document-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): `s.writer.writer` is a
//! semio-NATIVE artifact with no third-party reader or writer, so this case registers SUBJECT
//! handlers only. That is not an omission — the runner resolves an oracle implementation from the
//! feature's `@oracle-` tag, this feature carries `@no-oracle-` instead, and the oracle role is
//! therefore never dispatched for it. Registering an oracle handler here would be dead code that
//! reads as coverage in every listing, so there is none; every law this case claims is asserted
//! inside the subject handlers, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module, whose helpers are dependency-free
//! and format-neutral by their own doc comment.
//!
//! What the evidence rests on is the committed `(before, mutation, after, outcome)` specification
//! vector under each of the four `🧬️mutations/<slug>/🧪️tests/<fixture>/` leaves. Those files are
//! read HERE through `asset://`, so the plan pins their digests and a silently edited vector
//! changes the plan rather than the result. Nothing is transcribed into this file: the only literal
//! this adapter carries is the kind list, and the production module's own
//! `kinds_match_the_enum_and_the_catalog` keeps that honest.
//!
//! **Why `edit-text` needs one extra step and the other three do not.** `WriterSnapshot::document`
//! is a composed `s.stdio.semio.document` child HANDLE, and its live body is local-only content
//! owned by that exact handle. `edit-text`'s diff oracle reads the current body to decide whether
//! the edit is a no-op, so the handler materializes it before applying. The body is the committed payload's OWN `text` — that is
//! sound for exactly this vector and no other, because `⚠️warns-that-the-brief-body-is-unchanged` is
//! by construction the case where the payload repeats the body already behind the handle. It is
//! derived from the committed bytes, never transcribed beside them.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
#[cfg(feature = "sut")]
/// 🏷️ Mirrors `WriterMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because the oracle-only build must not link the subject crate.
const KINDS: &[&str] = &["rename-writer", "change-uri", "change-language", "edit-text"];

#[cfg(feature = "sut")]
/// 👁️ The kinds whose committed vector is deliberately a GUARD vector, so `before` and `after` are
/// the same document and the observability law cannot hold. `edit-text`'s vector pins that a save
/// with no keystrokes behind it must not re-mint the content-addressed handle; naming it here is a
/// claim the reader can check against the vector, and the feature description states the same. In
/// exchange, its `mutate` handler asserts the committed `🎯️outcome` and the handle's stability,
/// which the other three kinds have no equivalent of.
const GUARD_VECTORS: &[&str] = &["edit-text"];

#[cfg(feature = "sut")]
/// 🧫️ Where a `<vector>` cell from the feature's `Examples` table is rooted, relative to this
/// case's owner — the artifact root, which is what `asset://` resolves against.
const VECTORS: &str = "asset://🧬️schema/🧬️mutations";

/// 📄️ The real committed `jack` document, in this subset's own `.dsl.semio` text envelope.
#[cfg(feature = "sut")]
const EXAMPLE_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_writer::artifacts::writer::attach_writer_document_text;
    use semio_s_plugin_writer::artifacts::writer::standards::v1::subsets::any::io::snapshot::text::{parse_writer_dsl, print_writer_dsl};
    use semio_s_plugin_writer::artifacts::writer::standards::v1::subsets::any::schema::mutations::{
        apply_writer_mutation_outcome, decode_writer_mutation_json, decode_writer_snapshot_json, encode_writer_snapshot_json, inverse_writer_mutation_steps, WriterMutation,
    };
    use semio_s_plugin_writer::artifacts::writer::WriterSnapshot;

    //#region 🔖️VectorReading
    /// 🧫️ The `(kind, vector)` pair the scenario's doc string carries. The vector directory is data
    /// in the feature's own `Examples` table, so this adapter holds no map from kind to fixture.
    fn addressed(ctx: &Context) -> Result<(String, String), String> {
        let spec = ctx.doc_json()?;
        let (kind, vector) = (spec.str("kind"), spec.str("vector"));
        if kind.is_empty() || vector.is_empty() {
            return Err(format!("the scenario doc string must carry both a \"kind\" and a \"vector\", got {}", spec.to_string()));
        }
        Ok((kind, vector))
    }

    fn text_at(ctx: &Context, vector: &str, leaf: &str) -> Result<String, String> {
        let uri = format!("{}/{vector}/{leaf}", super::VECTORS);
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the committed vector {uri} is not UTF-8: {error}"))
    }

    fn snapshot_at(ctx: &Context, vector: &str, leaf: &str, kind: &str) -> Result<WriterSnapshot, String> {
        decode_writer_snapshot_json(&text_at(ctx, vector, leaf)?).map_err(|error| format!("the committed {leaf} vector for {kind:?} must decode: {error}"))
    }

    fn mutation_at(ctx: &Context, vector: &str, kind: &str) -> Result<WriterMutation, String> {
        decode_writer_mutation_json(&text_at(ctx, vector, "🦠️mutation/🔣️.json")?).map_err(|error| format!("the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &WriterSnapshot) -> Result<Json, String> {
        parse_json(&encode_writer_snapshot_json(snapshot))
    }

    /// 🌱 Materializes the before-snapshot's exact document handle with the committed payload's
    /// body for the one kind whose diff oracle reads it.
    fn seed_working_scene(snapshot: &mut WriterSnapshot, mutation: &WriterMutation) {
        if let WriterMutation::EditText(payload) = mutation {
            attach_writer_document_text(&mut snapshot.document, &payload.text);
        }
    }

    /// 🚨️ The `mutation.*` codes the committed `🎯️outcome` vector declares, in declared order.
    fn declared_codes(outcome: &Json) -> Vec<String> {
        outcome.array("messages").iter().map(|message| message.str("code")).filter(|code| !code.is_empty()).collect()
    }
    //#endregion 🔖️VectorReading

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts three things: the result IS
    /// the committed after-snapshot, the raised diagnostic codes ARE the committed outcome's, and —
    /// for every kind not named in `GUARD_VECTORS` — the mutation actually moved the projection.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector) = addressed(ctx)?;
        let mut base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let expected = snapshot_at(ctx, &vector, "📸️snapshot/➡️after/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let declared = parse_json(&text_at(ctx, &vector, "🎯️outcome/🔣️.json")?)?;
        seed_working_scene(&mut base, &mutation);
        let mut current = base.clone();
        let outcome = apply_writer_mutation_outcome(&mut current, &mutation);
        let raised: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
        if raised != declared_codes(&declared) {
            return Err(format!("mutate-{kind}: raised {raised:?}, the committed 🎯️outcome vector declares {:?}", declared_codes(&declared)));
        }
        let (produced, wanted, before) = (projection(&current)?, projection(&expected)?, projection(&base)?);
        if let Some(first) = law::divergence(&produced, &wanted) {
            return Err(format!("mutate-{kind}: the applied snapshot is not the committed after-snapshot — {first}"));
        }
        if kind == "edit-text" && current.document.child_id != base.document.child_id {
            return Err(format!("mutate-{kind}: an unchanged body re-minted the content-addressed handle ({} → {})", base.document.child_id, current.document.child_id));
        }
        law::mutation_is_observable(&kind, &produced, &before, super::GUARD_VECTORS)?;
        Ok(Outcome::with_raw(produced.to_string().into_bytes(), produced))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse
    /// must land back on the committed before-snapshot's projection, field for field, with no
    /// tolerance and no ignored key.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector) = addressed(ctx)?;
        let mut base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        seed_working_scene(&mut base, &mutation);
        let original = projection(&base)?;
        let mut current = base.clone();
        apply_writer_mutation_outcome(&mut current, &mutation);
        for step in inverse_writer_mutation_steps(&mutation, &base) {
            apply_writer_mutation_outcome(&mut current, &step);
        }
        let restored = projection(&current)?;
        law::inverse_restores(&kind, &restored, &original)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The real committed `jack` artifact, parsed and printed back. Two laws, both in role: the
    /// reparsed document must carry the same projection, and the printed text must reproduce the
    /// committed bytes EXACTLY. The exact-bytes half is `carrier_is_exact` rather than the wave's
    /// usual no-byte-pass-through tripwire because the committed `🗣️.dsl.semio` is this
    /// subset's OWN printer's output — the repository generated it from this very codec — so
    /// reproducing it is the correct answer and any drift between the committed artifact and the
    /// printer is the defect the scenario exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_bytes(super::EXAMPLE_ASSET)?;
        let text = String::from_utf8(committed.clone()).map_err(|error| format!("identity-round-trip: the committed jack artifact is not UTF-8: {error}"))?;
        let parsed = parse_writer_dsl(&text)?;
        let printed = print_writer_dsl(&parsed);
        let reparsed = parse_writer_dsl(&printed)?;
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
