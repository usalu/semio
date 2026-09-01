//! 🔬️ Emits the COMPOSED `semantic-obj-3-0-v1` projection for `pattern-shell.obj` and for every one
//! of the 22 declared `obj-3-0-any` mutation kinds applied to it.
//!
//! The composition is exactly what the registered comparison profile describes: the MESH half is
//! `mesh::project_obj`, the genuine third-party `tobj` 4 reading; the DOCUMENT half is this
//! subset's own `oracle_document_projection`, the independently-parsed surface a triangle-soup
//! reader cannot see. Nothing here computes a verdict — it prints JSON, and the framework's own
//! `compareProjections` decides.
//!
//! Usage: `verify <pattern-shell.obj>`  → one JSON document on stdout.

use semio_repo_test_host::Json;
use semio_s_plugin_stdio_test_oracle::artifacts::obj::standards::v3_0::subsets::any as obj;
use semio_s_plugin_stdio_test_oracle::mesh;

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn number(value: f64) -> Json {
    Json::Number(value)
}

fn text(value: &str) -> Json {
    Json::String(value.to_string())
}

fn spec(kind: &str, params: Vec<(&str, Json)>) -> Json {
    object(vec![("kind", text(kind)), ("params", object(params))])
}

fn vertex(x: f64, y: f64, z: f64) -> Json {
    object(vec![("x", number(x)), ("y", number(y)), ("z", number(z))])
}

fn corner(index: f64) -> Json {
    object(vec![("vertex", number(index))])
}

fn face(indices: [f64; 3]) -> Json {
    object(vec![("vertices", Json::Array(indices.iter().map(|index| corner(*index)).collect()))])
}

/// ⚖️ The composed projection: the `tobj` mesh reading and the independent document reading, side
/// by side under one root, which is the surface `semantic-obj-3-0-v1` names.
fn project(bytes: &[u8]) -> Result<Json, String> {
    Ok(object(vec![("mesh", mesh::project_obj(bytes)?), ("document", obj::oracle_document_projection(bytes)?)]))
}

/// 🦠️ One concrete, deliberately non-degenerate spec per declared kind — every one chosen to touch
/// something the base fixture actually carries, so a kind that fails to move the projection is a
/// real finding rather than a badly-aimed parameter.
fn specs(base: &[u8]) -> Vec<(&'static str, Json)> {
    let shortened = obj::oracle_apply_mutation(base, &spec("remove-vertex", vec![("index", number(5.0))])).expect("remove-vertex on the unreferenced v6");
    let alternative_snapshot = obj::oracle_snapshot_json(&shortened).expect("snapshot of the shortened document");
    vec![
        ("no-mutation", spec("no-mutation", vec![])),
        ("set-snapshot", spec("set-snapshot", vec![("snapshot", alternative_snapshot)])),
        ("insert-vertex", spec("insert-vertex", vec![("index", number(6.0)), ("vertex", vertex(7.0, 8.0, 9.0))])),
        ("remove-vertex", spec("remove-vertex", vec![("index", number(5.0))])),
        ("set-vertex", spec("set-vertex", vec![("index", number(5.0)), ("vertex", vertex(-4.0, -4.0, -4.0))])),
        ("insert-texcoord", spec("insert-texcoord", vec![("index", number(6.0)), ("texcoord", object(vec![("u", number(0.33)), ("v", number(0.44))]))])),
        ("remove-texcoord", spec("remove-texcoord", vec![("index", number(5.0))])),
        ("set-texcoord", spec("set-texcoord", vec![("index", number(5.0)), ("texcoord", object(vec![("u", number(0.11)), ("v", number(0.22))]))])),
        ("insert-normal", spec("insert-normal", vec![("index", number(4.0)), ("normal", vertex(0.0, -1.0, 0.0))])),
        ("remove-normal", spec("remove-normal", vec![("index", number(3.0))])),
        ("set-normal", spec("set-normal", vec![("index", number(3.0)), ("normal", vertex(-1.0, -1.0, 0.0))])),
        ("insert-face", spec("insert-face", vec![("index", number(1.0)), ("face", face([0.0, 2.0, 4.0]))])),
        ("remove-face", spec("remove-face", vec![("index", number(1.0))])),
        ("set-face", spec("set-face", vec![("index", number(0.0)), ("face", face([2.0, 1.0, 0.0]))])),
        ("set-group", spec("set-group", vec![("name", text("base")), ("faces", Json::Array(vec![number(0.0), number(1.0), number(2.0)]))])),
        ("remove-group", spec("remove-group", vec![("name", text("base"))])),
        ("set-object", spec("set-object", vec![("name", text("trim")), ("faces", Json::Array(vec![number(3.0), number(4.0)]))])),
        ("remove-object", spec("remove-object", vec![("name", text("shell"))])),
        ("set-mtllib", spec("set-mtllib", vec![("mtllib", text("replaced.mtl"))])),
        ("set-usemtl", spec("set-usemtl", vec![("usemtl", Json::Array(vec![object(vec![("faceIndexFrom", number(0.0)), ("material", text("only"))])]))])),
        ("set-smoothing-groups", spec("set-smoothing-groups", vec![("smoothingGroups", Json::Array(vec![object(vec![("faceIndexFrom", number(1.0)), ("group", number(7.0))])]))])),
        ("set-unknown-statements", spec("set-unknown-statements", vec![("unknownStatements", Json::Array(vec![object(vec![("raw", text("# replaced comment"))])]))])),
    ]
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: verify <pattern-shell.obj>");
    let base = std::fs::read(&path).unwrap_or_else(|error| panic!("reading {path}: {error}"));

    let base_projection = project(&base).expect("base projection");
    // ♻️ The identity round trip: the oracle's own parse→render of the untouched document. Its
    // projection must equal the base's, or every "did this kind move it?" answer below is noise.
    let round_tripped = obj::oracle_apply_mutation(&base, &spec("no-mutation", vec![])).expect("no-mutation");
    let round_trip_projection = project(&round_tripped).expect("round-trip projection");

    let mut kinds: Vec<Json> = Vec::new();
    for (kind, mutation) in specs(&base) {
        let mutated = obj::oracle_apply_mutation(&base, &mutation).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let projection = project(&mutated).unwrap_or_else(|error| panic!("{kind} projection: {error}"));
        kinds.push(object(vec![
            ("kind", text(kind)),
            ("spec", mutation),
            ("bytes", number(mutated.len() as f64)),
            ("mutatedDocument", text(&String::from_utf8_lossy(&mutated))),
            ("projection", projection),
        ]));
    }

    // 📏️ Two hand-corrupted copies that differ from the fixture ONLY in one vertex coordinate, one
    // below and one above the registered 1e-5 tolerance. They calibrate the gate: a profile that
    // rejects both is not a tolerance, and one that accepts both is not a gate.
    let corruptions: Vec<Json> = [("sub-tolerance", "v 0.5 0.5 1", "v 0.500001 0.5 1"), ("supra-tolerance", "v 0.5 0.5 1", "v 0.501 0.5 1")]
        .iter()
        .map(|(label, from, to)| {
            let text_form = String::from_utf8(base.clone()).expect("the fixture is UTF-8");
            assert!(text_form.contains(from), "the fixture must carry {from:?} for the {label} corruption to mean anything");
            let corrupted = text_form.replace(from, to);
            object(vec![("label", text(label)), ("replaced", text(from)), ("with", text(to)), ("projection", project(corrupted.as_bytes()).expect("corrupted projection"))])
        })
        .collect();

    let document = object(vec![
        ("fixture", text(&path)),
        ("baseBytes", number(base.len() as f64)),
        ("baseProjection", base_projection),
        ("identityRoundTripProjection", round_trip_projection),
        ("corruptions", Json::Array(corruptions)),
        ("kinds", Json::Array(kinds)),
    ]);
    println!("{}", document.to_string());
}
