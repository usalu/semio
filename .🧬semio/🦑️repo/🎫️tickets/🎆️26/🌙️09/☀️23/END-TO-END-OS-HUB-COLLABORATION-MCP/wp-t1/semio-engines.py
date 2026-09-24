"""🔁️ Writes the four single-kind semio@v1 carrier engines on json-rust (seed, edit and projection kept from the committed serde_json ones)."""
import re
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
ENGINES = {
    "📐️cad": dict(crate="semio_cad_json", subset="cad", kind="add-layer", directory="🗂️add-layer-applied", array="layers",
        seed='{"schema": "stdio.semio.cad", "layers": [{"name": "walls", "colorIndex": 7, "lineType": "CONTINUOUS", "visible": true}], "blocks": [], "entities": []}',
        item='{"name": "dimensions", "colorIndex": 3, "lineType": "DASHED", "visible": true}', keys='["schema", "layers", "blocks", "entities"]'),
    "🔺️mesh": dict(crate="semio_mesh_json", subset="mesh", kind="create-material", directory="🎨️create-material-applied", array="materials",
        seed='{"schema": "stdio.semio.mesh", "meshes": [], "materials": [{"id": "mat1", "baseColor": {"r": 0.8, "g": 0.8, "b": 0.8, "a": 1.0}, "metallic": 0.0, "roughness": 0.5}], "textures": []}',
        item='{"id": "mat2", "baseColor": {"r": 0.2, "g": 0.4, "b": 0.9, "a": 1.0}, "metallic": 0.2, "roughness": 0.8}', keys='["schema", "meshes", "materials", "textures"]'),
    "🖊️drawing": dict(crate="semio_drawing_json", subset="drawing", kind="create-layer", directory="🗂️create-layer-applied", array="layers",
        seed='{"schema": "stdio.semio.drawing", "canvas": {"width": 100.0, "height": 100.0}, "styles": [], "layers": [{"id": "layer1", "name": "Background", "visible": true, "root": {"kind": "path", "segments": []}}]}',
        item='{"id": "layer2", "name": "Foreground", "visible": true, "root": {"kind": "path", "segments": []}}', keys='["schema", "canvas", "styles", "layers"]'),
    "🧊️brep": dict(crate="semio_brep_json", subset="brep", kind="create-vertex", directory="➕️create-vertex-applied", array="vertices",
        seed='{"schema": "stdio.semio.brep", "vertices": [{"id": "v1", "point": {"x": 0.0, "y": 0.0, "z": 0.0}}], "edges": [], "loops": [], "faces": [], "shells": [], "solids": []}',
        item='{"id": "v2", "point": {"x": 1.0, "y": 1.0, "z": 1.0}}', keys='["schema", "vertices", "edges", "loops", "faces", "shells", "solids"]'),
}
TAIL = '''
/// 🔤️ Orders every object's keys, the committed carrier spelling; arrays keep their order.
pub fn canonical(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Object(object) => {
            let mut entries: Vec<(&str, &JsonValue)> = object.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let mut sorted = json::object::Object::with_capacity(entries.len());
            for (key, member) in entries {
                sorted.insert(key, canonical(member));
            }
            JsonValue::Object(sorted)
        }
        JsonValue::Array(items) => JsonValue::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

/// 🖨️ The committed file bytes of one carrier.
pub fn render(value: &JsonValue) -> String {
    format!("{}\\n", canonical(value).pretty(2))
}
'''
for owner, e in ENGINES.items():
    base = f"{ROOT}/{owner}/🏭️generator/🧩️json"
    old = open(f"{base}/🦀️.rs", encoding="utf-8").read()
    header = old.split("use serde_json")[0].replace("read back through `serde_json` — a third-party JSON implementation and nothing of ours.", "read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.")
    seed_doc = re.search(r"((?:/// .*\n)+)pub fn build_seed", old).group(1)
    apply_doc = re.search(r"((?:/// .*\n)+)pub fn apply", old).group(1)
    project_doc = re.search(r"((?:/// .*\n)+)pub fn project", old).group(1)
    lib = f'''{header}use json::JsonValue;

pub const KINDS: &[&str] = &["{e['kind']}"];

/// 🗂️ The reviewed fixture directory each kind's pair is committed under.
pub const FIXTURE_DIRECTORY_BY_KIND: &[(&str, &str)] = &[("{e['kind']}", "{e['directory']}")];

fn literal(text: &str) -> JsonValue {{
    json::parse(text).expect("a carrier literal is valid JSON")
}}

{seed_doc}pub fn build_seed() -> JsonValue {{
    literal(r#"{e['seed']}"#)
}}

{apply_doc}pub fn apply(kind: &str, doc: &JsonValue) -> Result<JsonValue, String> {{
    let mut doc = doc.clone();
    match kind {{
        "{e['kind']}" => match &mut doc["{e['array']}"] {{
            JsonValue::Array(items) => items.push(literal(r#"{e['item']}"#)),
            _ => return Err("the seed declares a {e['array']} array".to_string()),
        }},
        other => return Err(format!("unknown kind {{other}}")),
    }}
    Ok(doc)
}}
{TAIL}
{project_doc}pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {{
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in {e['keys']} {{
        out.insert(key, canonical(&parsed[key]));
    }}
    Ok(JsonValue::Object(out))
}}
'''
    open(f"{base}/🦀️.rs", "w", encoding="utf-8").write(lib)
    gen = open("/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/🏗️generate/🦀️.rs", encoding="utf-8").read()
    gen = gen.replace("`equation@1` JSON-carrier", f"`semio@v1/{e['subset']}` JSON-carrier").replace("use equation_json::", f"use {e['crate']}::")
    open(f"{base}/🏗️generate/🦀️.rs", "w", encoding="utf-8").write(gen)
    cargo = open(f"{base}/📦️packages/🦀️rust/Cargo.toml", encoding="utf-8").read()
    cargo = cargo.replace("# 🚫️ DEPENDS ON `serde_json` AND NOTHING ELSE.", "# 🚫️ DEPENDS ON `json` (json-rust) AND NOTHING ELSE — never `serde_json`, which the subject crate links in\n# production.")
    cargo = cargo.replace('serde_json = "1"', 'json = "0.12"')
    assert "serde_json =" not in cargo
    open(f"{base}/📦️packages/🦀️rust/Cargo.toml", "w", encoding="utf-8").write(cargo)
    print(owner, "ok")
