//! 🦀️ EN 1990 exhaustive mutation case — Rust adapter, the SUBJECT half. `s.norm.en1990` is a semio-native
//! artifact with no third-party reader or writer, so its reference is a second IMPLEMENTATION: the norm plugin's
//! independent Python engine, which `🐍️.py` beside this file feeds with this subset's catalog, registered as the
//! oracle `en1990-1-python-independent`. This adapter drives this repository's own production dispatch over the
//! whole `En1990Mutation` vocabulary — every kind the aggregate's own `KINDS` declares, one committed vector each.
//!
//! ⚖️ WHERE THE ASSERTIONS LIVE. Every law this case claims is asserted IN ROLE inside the subject handlers as well
//! as being compared against the oracle's answer, through the shared `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law`
//! module (`law::mutation_is_observable`, `law::inverse_restores`, `law::round_trip_preserves`,
//! `law::carrier_is_exact`), reached through the `oracleHostPackages` entry this plugin declares.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only `semio-repo-test-host`, the stdio law
//! crate and — behind `sut` — this subset's own crate, so the crate exports the bridges
//! (`decode_en1990_snapshot_json`/`encode_en1990_snapshot_json`, `decode_en1990_dsl`/`encode_en1990_dsl`,
//! `decode_en1990_pack`/`encode_en1990_pack`, `decode_en1990_mutation_json`, `apply_en1990_mutation`,
//! `inverse_en1990_mutation`) whose signatures name only reachable types. The mutation JSON is decoded generically,
//! through the aggregate's own `FromValue`; the committed files are read through the URIs the scenario's own steps
//! declare, so the feature is the one place a vector path is written down and both languages read the same bytes.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::mutations::{apply_en1990_mutation, decode_en1990_mutation_json, inverse_en1990_mutation};
    use semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::snapshot::{decode_en1990_dsl, decode_en1990_pack, decode_en1990_snapshot_json, encode_en1990_dsl, encode_en1990_pack, encode_en1990_snapshot_json, En1990Snapshot};
    use semio_repo_test_host::law;

    /// 🗣️ The real committed EN 1990 document the identity scenario declares.
    const DSL_ASSET: &str = "asset://🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio";
    /// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
    const PACK_ASSET: &str = "asset://🏢️high-consequence-office/🎒️.pack.semio";

    //#region 🔖️FixtureDecode
    /// 🧫️ One committed vector file the scenario's steps declare, found by its bundle-relative suffix.
    fn committed(ctx: &Context, kind: &str, suffix: &str) -> Result<String, String> {
        let uri = ctx.step_fixture_uris().into_iter().find(|uri| uri.ends_with(suffix)).ok_or_else(|| format!("mutate-en1990-1: the {kind:?} scenario declares no committed …/{suffix}"))?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("mutate-en1990-1: {uri} is not UTF-8: {error}"))
    }

    fn snapshot(ctx: &Context, kind: &str, suffix: &str) -> Result<En1990Snapshot, String> {
        decode_en1990_snapshot_json(&committed(ctx, kind, suffix)?).map_err(|error| format!("mutate-en1990-1: the committed {suffix} for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1990Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1990_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in.
    fn disagreement(what: &str, got: &En1990Snapshot, expected: &En1990Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1990_snapshot_json(got), encode_en1990_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the row's committed mutation to its committed before-snapshot and asserts the result IS the committed
    /// after-snapshot under exactly the committed outcome: production raises exactly the committed messages, an `applied`
    /// vector moves the projection, and a `no-op` or `rejected` one leaves the document bit-identical.
    pub fn mutate(row: String) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot(ctx, &row, "📸️snapshot/⬅️before/🔣️.json")?;
            let expected = snapshot(ctx, &row, "📸️snapshot/➡️after/🔣️.json")?;
            let mutation = decode_en1990_mutation_json(&committed(ctx, &row, "🦠️mutation/🔣️.json")?)?;
            let outcome = parse_json(&committed(ctx, &row, "🎯️outcome/🔣️.json")?)?;
            let status = outcome.str("status");
            if !["applied", "no-op", "rejected"].contains(&status.as_str()) {
                return Err(format!("mutate-{row}: unknown committed outcome status {status:?}"));
            }
            let spelled = |message: &Json| {
                let level = message.str("level");
                format!("{}{}:{}", level.get(..1).unwrap_or_default().to_uppercase(), level.get(1..).unwrap_or_default(), message.str("code"))
            };
            let promised: Vec<String> = outcome.array("messages").iter().map(spelled).collect();
            let (current, messages) = apply_en1990_mutation(&base, &mutation).map_err(|error| format!("mutate-{row}: production dispatch failed: {error}"))?;
            if messages != promised {
                return Err(format!("mutate-{row}: production dispatch raised {messages:?}, the committed outcome {promised:?}"));
            }
            if current != expected {
                return Err(disagreement(&format!("mutate-{row}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(&row, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{row}: a {status} mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// 🧫️ The refusal, no-op and clamp rows the subset's committed catalog registers beside each kind's canonical vector.
    pub fn rows() -> Vec<String> {
        let manifest = parse_json(include_str!("../../🔮️oracles/🔣️.json")).expect("the committed oracle manifest is JSON");
        manifest.array("mutationCatalogs").iter().flat_map(|catalog| catalog.array("vectors")).flat_map(|vector| vector.array("scenarios").into_iter().skip(1)).map(|scenario| scenario.str("id")).collect()
    }

    /// ↩️ The metamorphic inverse law in role: the kind and then its OWN computed inverse must restore the committed
    /// before-snapshot exactly, collection position included; an `applied` kind must compute a non-empty inverse. The
    /// projection carries BOTH the mutated and the restored document, so every row projects its own value.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot(ctx, kind, "📸️snapshot/⬅️before/🔣️.json")?;
            let mutation = decode_en1990_mutation_json(&committed(ctx, kind, "🦠️mutation/🔣️.json")?)?;
            let applied = parse_json(&committed(ctx, kind, "🎯️outcome/🔣️.json")?)?.str("status") == "applied";
            let original = projection(&base)?;
            let mut current = apply_en1990_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}"))?.0;
            let mutated = projection(&current)?;
            let steps = inverse_en1990_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if applied && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1990_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), restored)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🧵️ The canonical carrier bytes as a comparable projection: the envelope preamble, every body line as written,
    /// and the digest and length of what was emitted — the identical shape the Python reference builds from ITS
    /// re-emission, since `.dsl.semio` publishes no grammar to map carrier tokens onto snapshot spellings with.
    fn carrier_projection(text: &str) -> Json {
        let (preamble, body) = text.split_once('\n').unwrap_or((text, ""));
        let body = body.strip_suffix('\n').unwrap_or(body);
        let lines = if body.is_empty() { Vec::new() } else { body.split('\n').map(|line| Json::String(line.to_string())).collect::<Vec<Json>>() };
        Json::Object(vec![
            ("preamble".to_string(), Json::String(preamble.to_string())),
            ("lines".to_string(), Json::Array(lines)),
            ("dslDigest".to_string(), Json::String(digest(text.as_bytes()))),
            ("dslLength".to_string(), Json::Number(text.as_bytes().len() as f64)),
        ])
    }

    /// 🔁️ The real committed document through every encoding it has: the DSL carrier re-emits byte for byte, and the
    /// hand-written DSL grammar, the binary pack protocol and the JSON projection all agree on what it parsed to — a
    /// shortcut that handed back its input bytes could not survive the pack leg.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1990 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1990_dsl(&text)?;
        let reprinted = encode_en1990_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        if decode_en1990_dsl(&reprinted)? != parsed {
            return Err("identity-round-trip: printing the document back to DSL and reparsing it lost content".to_string());
        }
        let repacked = decode_en1990_pack(&encode_en1990_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1990_snapshot_json(&encode_en1990_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1990_pack(&ctx.fixture_bytes(PACK_ASSET)?)?;
        if twin != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact", &twin, &parsed));
        }
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), carrier_projection(&reprinted)))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id: one `mutate-`/`inverse-` pair
/// per kind the production aggregate declares, so the loop and the feature's `Examples` tables name the same set.
/// SUBJECT role only — the reference answer comes from the independent Python implementation.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in semio_s_artifact_norm_en1990::standards::v1::subsets::any::schema::mutations::KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind.to_string())).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }
        for row in subject::rows() {
            built = built.subject(&format!("mutate-{row}"), subject::mutate(row));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
