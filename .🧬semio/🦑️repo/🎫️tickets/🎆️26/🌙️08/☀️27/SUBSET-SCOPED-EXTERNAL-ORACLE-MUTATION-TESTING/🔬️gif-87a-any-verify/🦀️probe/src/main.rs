//! 🔬️ Scratch verification harness for the `s.stdio.gif@87a/✳️any` fixture corpus.
//!
//! Calls ONLY the committed oracle's public surface — `oracle_apply_mutation`, `oracle_inverse_spec`
//! and `project_gif_87a` — and dumps every projection it produced as JSON. It decides nothing: the
//! accept/reject judgement is made by the framework's OWN `compareProjections` under the real
//! `semantic-raster-v1` profile, driven by `../📜️script.ts`.
//!
//! Usage: `probe <fixture.gif> <out.json>`.

use semio_repo_test_host::Json;
use semio_s_plugin_stdio_test_oracle::artifacts::gif::standards::v87a::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, project_gif_87a};

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn num(value: f64) -> Json {
    Json::Number(value)
}

fn indices(values: &[u8]) -> Json {
    Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
}

fn rgb(r: u8, g: u8, b: u8) -> Json {
    obj(vec![("r", num(r as f64)), ("g", num(g as f64)), ("b", num(b as f64))])
}

fn palette(colors: Vec<Json>) -> Json {
    obj(vec![("sorted", Json::Bool(false)), ("colors", Json::Array(colors))])
}

/// 📸️ The oracle's own reading of the ORIGINAL document, as a snapshot payload — obtained from the
/// oracle's own `set-snapshot` inverse rather than hand-written, so the `set-snapshot` case starts
/// from a document the oracle itself produced.
fn original_snapshot(bytes: &[u8]) -> Json {
    let inverse = oracle_inverse_spec(bytes, "set-snapshot", &obj(vec![])).expect("oracle inverse of set-snapshot");
    inverse.get("params").and_then(|params| params.get("snapshot")).cloned().expect("set-snapshot inverse carries the original snapshot")
}

/// 📸️ That snapshot with its logical screen and background index deliberately moved, so
/// `set-snapshot` is a real replacement rather than an identity restatement.
fn altered_snapshot(bytes: &[u8]) -> Json {
    let snapshot = original_snapshot(bytes);
    let Json::Object(entries) = snapshot else { panic!("the oracle's snapshot is not an object") };
    Json::Object(
        entries
            .into_iter()
            .map(|(key, value)| match key.as_str() {
                "width" => (key, num(9.0)),
                "height" => (key, num(7.0)),
                "backgroundColorIndex" => (key, num(1.0)),
                _ => (key, value),
            })
            .collect(),
    )
}

fn cases(bytes: &[u8]) -> Vec<(&'static str, Json)> {
    let inserted_image = obj(vec![
        ("left", num(4.0)),
        ("top", num(2.0)),
        ("width", num(2.0)),
        ("height", num(2.0)),
        ("interlace", Json::Bool(false)),
        ("lct", palette(vec![rgb(200, 0, 0), rgb(0, 200, 0), rgb(0, 0, 200), rgb(200, 200, 200)])),
        ("indices", indices(&[3, 2, 1, 0])),
    ]);
    vec![
        ("no-mutation", obj(vec![])),
        ("set-snapshot", obj(vec![("snapshot", altered_snapshot(bytes))])),
        ("set-screen-size", obj(vec![("width", num(9.0)), ("height", num(7.0))])),
        ("set-global-color-table", obj(vec![("gct", palette(vec![rgb(1, 2, 3), rgb(4, 5, 6), rgb(7, 8, 9), rgb(10, 11, 12)]))])),
        ("set-background-color-index", obj(vec![("index", num(6.0))])),
        ("set-pixel-aspect-ratio", obj(vec![("ratio", num(17.0))])),
        ("insert-image", obj(vec![("index", num(1.0)), ("image", inserted_image)])),
        ("remove-image", obj(vec![("index", num(1.0))])),
        ("move-image", obj(vec![("from", num(0.0)), ("to", num(2.0))])),
        ("set-image-geometry", obj(vec![("index", num(2.0)), ("left", num(3.0)), ("top", num(2.0)), ("width", num(2.0)), ("height", num(2.0))])),
        ("set-image-pixels", obj(vec![("index", num(0.0)), ("indices", indices(&[7, 6, 5, 4, 3, 2, 1, 0, 7, 6, 5, 4, 3, 2, 1, 0]))])),
        ("set-image-interlace", obj(vec![("index", num(0.0)), ("interlace", Json::Bool(true))])),
    ]
}

fn spec(kind: &str, params: &Json) -> Json {
    obj(vec![("kind", Json::String(kind.to_string())), ("params", params.clone())])
}

/// 💥️ A deliberately WRONG document, not a plausible alternative: the same pixel mutation aimed at
/// image 2 instead of image 0, plus one Global Color Table byte flipped in the committed file
/// itself. Both are genuine content differences a raster comparison must refuse.
fn wrong_documents(bytes: &[u8]) -> Vec<(&'static str, Vec<u8>)> {
    let wrong_target = oracle_apply_mutation(bytes, &spec("set-image-pixels", &obj(vec![("index", num(2.0)), ("indices", indices(&[7, 6, 5, 4]))]))).expect("apply set-image-pixels to the wrong image");
    let mut flipped = bytes.to_vec();
    flipped[13] ^= 0xFF;
    vec![("set-image-pixels-aimed-at-image-2", wrong_target), ("global-color-table-byte-13-flipped", flipped)]
}

fn main() {
    let mut args = std::env::args().skip(1);
    let fixture_path = args.next().expect("usage: probe <fixture.gif> <out.json>");
    let out_path = args.next().expect("usage: probe <fixture.gif> <out.json>");
    let bytes = std::fs::read(&fixture_path).unwrap_or_else(|error| panic!("reading {fixture_path}: {error}"));

    let base = project_gif_87a(&bytes).expect("project the committed fixture");
    let mut rows = Vec::new();
    for (kind, params) in cases(&bytes) {
        let mutated = oracle_apply_mutation(&bytes, &spec(kind, &params)).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let mutated_projection = project_gif_87a(&mutated).unwrap_or_else(|error| panic!("{kind} projection: {error}"));
        let inverse = oracle_inverse_spec(&bytes, kind, &params).unwrap_or_else(|error| panic!("{kind} inverse: {error}"));
        let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse apply: {error}"));
        let restored_projection = project_gif_87a(&restored).unwrap_or_else(|error| panic!("{kind} inverse projection: {error}"));
        rows.push(obj(vec![
            ("kind", Json::String(kind.to_string())),
            ("params", params),
            ("mutatedBytes", num(mutated.len() as f64)),
            ("byteIdenticalToFixture", Json::Bool(mutated == bytes)),
            ("restoredBytesIdenticalToFixture", Json::Bool(restored == bytes)),
            ("projection", mutated_projection),
            ("restoredProjection", restored_projection),
            ("inverseSpec", inverse),
        ]));
    }

    let wrong = wrong_documents(&bytes)
        .into_iter()
        .map(|(label, document)| {
            obj(vec![
                ("label", Json::String(label.to_string())),
                ("projection", project_gif_87a(&document).unwrap_or_else(|error| panic!("{label} projection: {error}"))),
            ])
        })
        .collect();

    let report = obj(vec![
        ("fixture", Json::String(fixture_path.clone())),
        ("fixtureBytes", num(bytes.len() as f64)),
        ("magic", Json::String(String::from_utf8_lossy(&bytes[0..6]).to_string())),
        ("base", base),
        ("cases", Json::Array(rows)),
        ("wrongDocuments", Json::Array(wrong)),
    ]);
    std::fs::write(&out_path, report.to_string()).unwrap_or_else(|error| panic!("writing {out_path}: {error}"));
    eprintln!("wrote {out_path}");
}
