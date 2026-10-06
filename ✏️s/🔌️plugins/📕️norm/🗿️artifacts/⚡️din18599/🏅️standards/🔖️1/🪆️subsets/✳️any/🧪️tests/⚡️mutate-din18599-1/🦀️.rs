//! 🦀️ DIN V 18599 exhaustive mutation case — the Rust SUBJECT half. `s.norm.din18599` is a semio-native artifact with no
//! third-party reader or writer, so its reference is the independent Python implementation registered as the oracle
//! `din18599-1-python-independent`; this adapter drives this repository's own production dispatch over the whole
//! `Din18599Mutation` vocabulary — a energy-balance building: document scalars, whole-facet system specifications, zone and element lists and the parent-owned monthly climate with its derived climate table.
//!
//! ⚖️ Every law is asserted IN ROLE through the shared `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law` module, reached
//! through the `oracleHostPackages` entry of `✏️s/🔌️plugins/📕️norm/🔮️oracles/🔣️.json`. Both implementations read
//! the SAME committed bytes: the feature is the single place a vector path is written down, and each handler resolves
//! exactly the `(before, mutation, after, outcome)` URIs its own scenario's steps name.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::mutations::{apply_din18599_mutation, inverse_din18599_mutation, Din18599Mutation};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::text::mutations::{decode_din18599_mutation_json};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::snapshot::{Din18599Snapshot};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::binary::snapshot::{encode_din18599_pack};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::binary::snapshot::{decode_din18599_pack};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::text::snapshot::{encode_din18599_dsl};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::text::snapshot::{decode_din18599_dsl};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::text::snapshot::{decode_din18599_snapshot_json};
    use semio_s_artifact_norm_din18599::standards::v1::subsets::any::io::text::snapshot::{encode_din18599_snapshot_json};
    use semio_repo_test_host::law;

    /// 🗣️ The real committed DIN V 18599 document, read where the domain already keeps it.
    const DSL_ASSET: &str = "asset://🎬️demo/🗣️.dsl.semio";

    /// 🧫️ The committed `(before, mutation, after, outcome)` texts this scenario's steps name, in that role order.
    fn vector(ctx: &Context) -> Result<[String; 4], String> {
        let uris = ctx.step_input_uris();
        let text = |suffix: &str| -> Result<String, String> {
            let uri = uris.iter().find(|uri| uri.ends_with(suffix)).ok_or_else(|| format!("the scenario names no committed …{suffix}"))?;
            String::from_utf8(ctx.input_bytes(uri)?).map_err(|error| format!("{uri}: {error}"))
        };
        Ok([text("⬅️before/🔣️.json")?, text("🦠️mutation/🔣️.json")?, text("➡️after/🔣️.json")?, text("🎯️outcome/🔣️.json")?])
    }

    fn snapshot_of(text: &str, label: &str) -> Result<Din18599Snapshot, String> {
        decode_din18599_snapshot_json(text).map_err(|error| format!("the committed {label}-snapshot must decode: {error}"))
    }

    fn mutation_of(text: &str) -> Result<Din18599Mutation, String> {
        decode_din18599_mutation_json(text).map_err(|error| format!("the committed mutation payload must decode: {error}"))
    }

    fn projection(snapshot: &Din18599Snapshot) -> Result<Json, String> {
        parse_json(&encode_din18599_snapshot_json(snapshot))
    }

    /// 🚨️ A failure that names WHAT disagreed, in the JSON the vectors are written in.
    fn disagreement(what: &str, got: &Din18599Snapshot, expected: &Din18599Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_din18599_snapshot_json(got), encode_din18599_snapshot_json(expected))
    }

    /// 🎯️ Applies the committed mutation to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot under the committed outcome: an `applied` vector moves the document without a diagnostic, a
    /// `rejected` one raises one and leaves the document bit-identical.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let [before, mutation, after, outcome] = vector(ctx)?;
            let (base, expected, mutation) = (snapshot_of(&before, "before")?, snapshot_of(&after, "after")?, mutation_of(&mutation)?);
            let status = parse_json(&outcome)?.str("status");
            let current = match (status.as_str(), apply_din18599_mutation(&base, &mutation)) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("rejected", Ok((snapshot, messages))) if !messages.is_empty() => snapshot,
                ("rejected", Ok(_)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic")),
                (_, Err(error)) => return Err(format!("mutate-{kind}: production dispatch failed: {error}")),
                (other, _) => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
            };
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(kind, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{kind}: a rejected mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// ↩️ The metamorphic inverse law: the mutation followed by its OWN computed inverse restores the committed
    /// before-snapshot, position included; an applied kind must produce a non-empty inverse. The projection carries
    /// BOTH the mutated and the restored document so the differential never compares one constant.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let [before, mutation, _after, outcome] = vector(ctx)?;
            let (base, mutation) = (snapshot_of(&before, "before")?, mutation_of(&mutation)?);
            let original = projection(&base)?;
            let (mut current, _messages) = apply_din18599_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation failed: {error}"))?;
            let mutated = projection(&current)?;
            let steps = inverse_din18599_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if parse_json(&outcome)?.str("status") == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_din18599_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step failed: {error}"))?.0;
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

    /// 🧵️ The canonical carrier bytes as a comparable projection: preamble, body lines as written, and the digest and
    /// length of what was emitted — the identical shape the Python implementation builds from ITS re-emission.
    fn carrier_projection(text: &str) -> Json {
        let (preamble, body) = text.split_once('\n').unwrap_or((text, ""));
        let body = body.strip_suffix('\n').unwrap_or(body);
        let lines = if body.is_empty() { Vec::new() } else { body.split('\n').map(|line| Json::String(line.to_string())).collect() };
        Json::Object(vec![
            ("preamble".to_string(), Json::String(preamble.to_string())),
            ("lines".to_string(), Json::Array(lines)),
            ("dslDigest".to_string(), Json::String(digest(text.as_bytes()))),
            ("dslLength".to_string(), Json::Number(text.len() as f64)),
        ])
    }

    /// 🔁️ The real committed document through every encoding it has: the DSL carrier re-emits byte for byte, and the
    /// pack and JSON codecs reproduce the parsed document, so a shortcut that handed back its input could not pass.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.input_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = decode_din18599_dsl(&text)?;
        let reprinted = encode_din18599_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        if decode_din18599_dsl(&reprinted)? != parsed {
            return Err("identity-round-trip: printing the document back to DSL and reparsing it lost content".to_string());
        }
        let repacked = decode_din18599_pack(&encode_din18599_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: the pack codec lost content", &repacked, &parsed));
        }
        let rejson = decode_din18599_snapshot_json(&encode_din18599_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: the JSON codec lost content", &rejson, &parsed));
        }
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), carrier_projection(&reprinted)))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by FULL expanded scenario id, one `mutate-`/`inverse-` pair per kind of the production vocabulary
/// plus the carrier identity. SUBJECT role only: the reference answer comes from the independent Python oracle.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in semio_s_artifact_norm_din18599::standards::v1::subsets::any::schema::mutations::KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
