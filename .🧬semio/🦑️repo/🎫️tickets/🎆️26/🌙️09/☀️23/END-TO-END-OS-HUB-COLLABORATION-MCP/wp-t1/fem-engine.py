"""🔁️ Emits the json-rust fem engine lib from the committed pack-based one (reviewed by diff, not kept as a codemod)."""
import re, subprocess, sys
path, dimension = sys.argv[1], sys.argv[2]
source = subprocess.run(["python3", "json-literals.py", path], capture_output=True, text=True, check=True).stdout
head, rest = source.split("pub const KINDS", 1)
head = re.sub(r"//! carrier and read back through the first-party JSON value tree\.", "//! carrier and read back through `json` (json-rust) — a third-party JSON implementation and nothing of ours.", head)
head = head.replace("use pack::json::{Object, Value};\nuse pack::json;\n", "use json::JsonValue;\n")
rest = "pub const KINDS" + rest
rest = rest.split("/// 📄️ Canonicalises for comparison")[0]
rest = rest.replace("-> Value", "-> JsonValue").replace("&Value", "&JsonValue")
rest = re.sub(r'fn array<.a>\(doc: &.a mut Value, key: &str\) -> &.a mut Vec<Value> \{\n    doc.get_mut\(key\).and_then\(Value::as_array_mut\).expect\("the seed declares every collection"\)\n\}',
 'fn literal(text: &str) -> JsonValue {\n    json::parse(text).expect("a carrier literal is valid JSON")\n}\n\nfn array<\'a>(value: &\'a mut JsonValue, key: &str) -> Result<&\'a mut Vec<JsonValue>, String> {\n    match &mut value[key] {\n        JsonValue::Array(items) => Ok(items),\n        _ => Err(format!("{key} is not an array")),\n    }\n}', rest)
rest = re.sub(r"array\(&mut doc, (\"\w+\")\)", r"array(&mut doc, \1)?", rest)
rest = re.sub(r'\.retain\(\|(\w)\| \1\.get\("id"\)\.and_then\(Value::as_str\) != Some\(("[\w-]+")\)\)', r'.retain(|\1| \1["id"] != \2)', rest)
rest = re.sub(r'(cases\[\d\])\.get_mut\("loads"\)\.and_then\(Value::as_array_mut\)\.ok_or\("lc1 has no loads array"\)\?', r'array(&mut \1, "loads")?', rest)
rest = re.sub(r'(cases\[\d\])\.as_object_mut\(\)\.ok_or\("lc\d is not an object"\)\?\.insert\("(\w+)"\.to_string\(\), Value::Bool\((\w+)\)\)', r'\1["\2"] = \3.into()', rest)
rest = re.sub(r'(cases\[\d\])\.as_object_mut\(\)\.ok_or\("lc\d is not an object"\)\?\.insert\("(\w+)"\.to_string\(\), Value::String\(("[\w ]+")\.to_string\(\)\)\)', r'\1["\2"] = \3.into()', rest)
rest = re.sub(r'doc\.as_object_mut\(\)\.ok_or\("document is not an object"\)\?\.insert\("(\w+)"\.to_string\(\), (literal\(r#".*?"#\))\)', r'doc["\1"] = \2', rest)
keys = re.search(r'for key in (\[[^\]]+\])', source).group(1)
tail = f'''/// 🔤️ Orders every object's keys, the committed carrier spelling; arrays keep their ORDER, so a
/// reordering is a difference, not a tie. Numbers keep their lexeme — no rounding, because a
/// tolerance here would silently accept a changed stiffness.
pub fn canonical(value: &JsonValue) -> JsonValue {{
    match value {{
        JsonValue::Object(object) => {{
            let mut entries: Vec<(&str, &JsonValue)> = object.iter().collect();
            entries.sort_by(|left, right| left.0.cmp(right.0));
            let mut sorted = json::object::Object::with_capacity(entries.len());
            for (key, member) in entries {{
                sorted.insert(key, canonical(member));
            }}
            JsonValue::Object(sorted)
        }}
        JsonValue::Array(items) => JsonValue::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }}
}}

/// 🖨️ The committed file bytes of one carrier.
pub fn render(value: &JsonValue) -> String {{
    format!("{{}}\\n", canonical(value).pretty(2))
}}

/// 📄️ The projection: the nine collections the kinds touch, canonicalised.
pub fn project(bytes: &[u8]) -> Result<JsonValue, String> {{
    let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    let parsed = json::parse(text).map_err(|error| error.to_string())?;
    let mut out = json::object::Object::new();
    for key in {keys} {{
        out.insert(key, canonical(&parsed[key]));
    }}
    Ok(JsonValue::Object(out))
}}
'''
print(head + rest + tail, end="")
