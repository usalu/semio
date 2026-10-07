//! 🦀️ EN 1996 exhaustive mutation case — the Rust SUBJECT half. `s.norm.en1996` is a semio-native artifact with no
//! third-party reader or writer, so its reference is the independent Python implementation registered as the oracle
//! `en1996-1-python-independent`; this adapter drives this repository's own production dispatch over the whole
//! `En1996Mutation` vocabulary — a masonry building: walls with their openings, load cases and concentrated loads, addressed by wall, load-case and member position.
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
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::mutations::{apply_en1996_mutation,inverse_en1996_mutation,En1996Mutation};

    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::text::mutations::{decode_en1996_mutation_json};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::snapshot::{En1996Snapshot};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::binary::snapshot::{encode_en1996_pack};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::binary::snapshot::{decode_en1996_pack};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::text::snapshot::{encode_en1996_dsl};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::text::snapshot::{decode_en1996_dsl};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::text::snapshot::{decode_en1996_snapshot_json};
    use semio_s_artifact_norm_en1996::standards::v1::subsets::any::io::text::snapshot::{encode_en1996_snapshot_json};
    use semio_repo_test_host::law;

    /// 🗣️ The real committed EN 1996 document, read where the domain already keeps it.
    const DSL_ASSET: &str = "asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio";

    /// 🧫️ The committed `(before, mutation, after, outcome)` texts this scenario's steps name, in that role order.
    fn vector(ctx: &Context) -> Result<[String; 4], String> {
        let uris = ctx.step_input_uris();
        let text = |suffix: &str| -> Result<String, String> {
            let uri = uris.iter().find(|uri| uri.ends_with(suffix)).ok_or_else(|| format!("the scenario names no committed …{suffix}"))?;
            String::from_utf8(ctx.input_bytes(uri)?).map_err(|error| format!("{uri}: {error}"))
        };
        Ok([text("⬅️before/🔣️.json")?, text("🦠️mutation/🔣️.json")?, text("➡️after/🔣️.json")?, text("🎯️outcome/🔣️.json")?])
    }

    fn snapshot_of(text: &str, label: &str) -> Result<En1996Snapshot, String> {
        decode_en1996_snapshot_json(text).map_err(|error| format!("the committed {label}-snapshot must decode: {error}"))
    }

    fn mutation_of(text: &str) -> Result<En1996Mutation, String> {
        decode_en1996_mutation_json(text).map_err(|error| format!("the committed mutation payload must decode: {error}"))
    }

    fn projection(snapshot: &En1996Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1996_snapshot_json(snapshot))
    }

    /// 🚨️ A failure that names WHAT disagreed, in the JSON the vectors are written in.
    fn disagreement(what: &str, got: &En1996Snapshot, expected: &En1996Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1996_snapshot_json(got), encode_en1996_snapshot_json(expected))
    }

    /// 🎯️ Applies the committed mutation to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot under exactly the committed outcome: production raises exactly the committed messages, an `applied`
    /// vector moves the document, and a `no-op` or `rejected` one leaves it bit-identical.
    pub fn mutate(row: String) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let [before, mutation, after, outcome] = vector(ctx)?;
            let (base, expected, mutation) = (snapshot_of(&before, "before")?, snapshot_of(&after, "after")?, mutation_of(&mutation)?);
            let outcome = parse_json(&outcome)?;
            let status = outcome.str("status");
            if !["applied", "no-op", "rejected"].contains(&status.as_str()) {
                return Err(format!("mutate-{row}: unknown committed outcome status {status:?}"));
            }
            let spelled = |message: &Json| {
                let level = message.str("level");
                format!("{}{}:{}", level.get(..1).unwrap_or_default().to_uppercase(), level.get(1..).unwrap_or_default(), message.str("code"))
            };
            let promised: Vec<String> = outcome.array("messages").iter().map(spelled).collect();
            let (current, messages) = apply_en1996_mutation(&base, &mutation).map_err(|error| format!("mutate-{row}: production dispatch failed: {error}"))?;
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

    /// ↩️ The metamorphic inverse law: the mutation followed by its OWN computed inverse restores the committed
    /// before-snapshot, position included; an applied kind must produce a non-empty inverse. The projection carries
    /// BOTH the mutated and the restored document so the differential never compares one constant.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let [before, mutation, _after, outcome] = vector(ctx)?;
            let (base, mutation) = (snapshot_of(&before, "before")?, mutation_of(&mutation)?);
            let original = projection(&base)?;
            let (mut current, _messages) = apply_en1996_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation failed: {error}"))?;
            let mutated = projection(&current)?;
            let steps = inverse_en1996_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
            if parse_json(&outcome)?.str("status") == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1996_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step failed: {error}"))?.0;
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
        let parsed = decode_en1996_dsl(&text)?;
        let reprinted = encode_en1996_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        if decode_en1996_dsl(&reprinted)? != parsed {
            return Err("identity-round-trip: printing the document back to DSL and reparsing it lost content".to_string());
        }
        let repacked = decode_en1996_pack(&encode_en1996_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: the pack codec lost content", &repacked, &parsed));
        }
        let rejson = decode_en1996_snapshot_json(&encode_en1996_snapshot_json(&parsed))?;
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
/// plus the carrier identity and one `mutate-` row per further catalog vector. SUBJECT role only: the reference answer comes from the independent Python oracle.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in semio_s_artifact_norm_en1996::standards::v1::subsets::any::schema::mutations::KINDS {
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
