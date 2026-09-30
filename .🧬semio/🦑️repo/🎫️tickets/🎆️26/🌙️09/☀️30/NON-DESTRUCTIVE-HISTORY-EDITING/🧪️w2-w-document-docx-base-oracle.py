"""📨️ W2-W-document: the DOCX ECMA-376 base reference (`🧱️base/🔮️oracles/🦀️.rs`) reads the leaf wire (design §11, F10):
run edits resolve the wire `DocxXmlAddress` in the oracle's own quick-xml tree, `set-style-based-on` reads `based_on`,
`set-part` reads `content_type` + `bytes`, the computed undo is a list of wire specs (no internal `no-mutation`), and
`set-snapshot` — a whole package, no table cell — becomes `oracle_replace_package`. Idempotent per anchor."""
import pathlib

PATH = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs")

ADDRESS = '''    /// 🧭️ The leaf wire's `DocxXmlAddress` (`{partPath, nodePath, expectedName, revision}`) of one run, resolved in this
    /// oracle's OWN `quick-xml` tree of the main part: `nodePath` walks child indices from the root, and every step is
    /// translated into this module's block model — `w:p`/`w:tbl` ordinal in a block list, `w:tr` and `w:tc` ordinals in
    /// a table, `w:r` ordinal in a paragraph. Only runs this model carries (direct `w:r` children of a paragraph) are
    /// addressable; the `revision` lineage guard is the subject's own and is not re-derived here.
    fn resolve_run_address(pkg: &WPackage, address: &Json) -> Result<(DPath, usize), String> {
        let part_path = address.str("partPath");
        if part_path != pkg.main_path {
            return Err(format!("run address names part {part_path:?}, not the main document part {:?}", pkg.main_path));
        }
        let main = pkg.opc.part(&pkg.main_path).ok_or("run address: the main document part is absent")?;
        let mut node = parse_xml(&main.bytes)?;
        let mut path = DPath::default();
        let mut table: Option<DPathSegment> = None;
        let mut run = None;
        for step in address.array("nodePath") {
            let index = match step {
                Json::Number(value) if value >= 0.0 && value.fract() == 0.0 => value as usize,
                other => return Err(format!("run address nodePath step {other:?} is not an index")),
            };
            let XNode::Element { name: parent, children, .. } = node else { return Err("run address descends through a text node".to_string()) };
            let child = children.get(index).cloned().ok_or_else(|| format!("run address step {index} is outside <{parent}>'s {} children", children.len()))?;
            let XNode::Element { name, .. } = &child else { return Err(format!("run address step {index} of <{parent}> is not an element")) };
            let ordinal = |names: &[&str]| children[..index].iter().filter(|sibling| matches!(sibling, XNode::Element { name, .. } if names.contains(&name.as_str()))).count();
            match (parent.as_str(), name.as_str()) {
                ("w:document", "w:body") => {}
                ("w:body" | "w:tc", "w:p" | "w:tbl") => {
                    if let Some(segment) = table.take() {
                        path.segments.push(segment);
                    }
                    path.index = ordinal(&["w:p", "w:tbl"]);
                }
                ("w:tbl", "w:tr") => table = Some(DPathSegment { block_index: path.index, row: ordinal(&["w:tr"]), cell: 0 }),
                ("w:tr", "w:tc") => table.as_mut().ok_or("run address enters a cell outside a table")?.cell = ordinal(&["w:tc"]),
                ("w:p", "w:r") => run = Some(ordinal(&["w:r"])),
                (outer, inner) => return Err(format!("run address steps from <{outer}> into <{inner}>, outside this oracle's paragraph/table/run model")),
            }
            node = child;
        }
        let expected = address.str("expectedName");
        let XNode::Element { name, .. } = &node else { return Err("run address resolves to a text node".to_string()) };
        if name != "w:r" || !expected.ends_with("}r") {
            return Err(format!("run address resolves <{name}>, expected {expected:?}"));
        }
        Ok((path, run.ok_or("run address names no run")?))
    }
    //#endregion 🔖️PathAddressing
'''

BYTES = '''    /// 🧮️ A wire byte array (`[0..=255, …]`).
    fn byte_array(value: Option<&Json>) -> Result<Vec<u8>, String> {
        match value {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(byte) if (0.0..=255.0).contains(byte) && byte.fract() == 0.0 => Ok(*byte as u8),
                    other => Err(format!("{other:?} is not a byte")),
                })
                .collect(),
            other => Err(format!("{other:?} is not a byte array")),
        }
    }

    fn bytes_to_json(bytes: &[u8]) -> Json {
        Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {'''

ROUTING = '''        let base = read_package(input)?;
        let mut current = apply_mutation(input, kind, params)?;
        for undo in inverse_spec(&base, kind, params)? {
            current = apply_mutation(&current, &undo.str("kind"), undo.get("params").unwrap_or(&Json::Null))?;
        }
        Ok(current)
    }

    /// 📸️ The whole package replaced by `replacement`: both are read through this oracle's own OPC + WordprocessingML
    /// reader, and the replacement is written back through its own writer — the reference side of `set-snapshot`, whose
    /// payload is an entire package rather than a table cell.
    pub fn replace_package(input: &[u8], replacement: &[u8]) -> Result<Vec<u8>, String> {
        read_package(input)?;
        write_package(&read_package(replacement)?)
    }

    /// 🔄️ The package read and re-written through this oracle's own reader and writer alone.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        write_package(&read_package(input)?)
    }'''

DISABLED = 'Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())'


def run_fix(kind):
    return (f'''            "{kind}" => {{
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks_mut''', f'''            "{kind}" => {{
                let (path, run_index) = resolve_run_address(pkg, params.get("address").unwrap_or(&Json::Null))?;
                let list = resolve_blocks_mut''')


def inverse_run(kind, extra_json):
    old_extra = {"set-run-text": '("text", Json::String(run.text.clone()))', "set-run-formatting": '("bold", Json::Bool(run.bold)), ("italic", Json::Bool(run.italic)), ("underline", Json::Bool(run.underline))'}[kind]
    old = f'''            "{kind}" => {{
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks(&base.body'''
    new = f'''            "{kind}" => {{
                let address = params.get("address").cloned().unwrap_or(Json::Null);
                let (path, run_index) = resolve_run_address(base, &address)?;
                let list = resolve_blocks(&base.body'''
    return [(old, new), (f'spec("{kind}", obj(vec![("path", path_to_json(&path)), ("runIndex", Json::Number(run_index as f64)), {old_extra}]))', f'spec("{kind}", obj(vec![("address", address), {old_extra}]))')]


EDITS = [
    ("    //#endregion 🔖️PathAddressing\n", ADDRESS),
    ('''            "no-mutation" => {}
            "set-snapshot" => {
                pkg.body = params.array("body").iter().map(json_to_block).collect::<Result<_, _>>()?;
                pkg.styles = params.array("styles").iter().map(|s| Ok::<_, String>(json_to_style(s))).collect::<Result<_, _>>()?;
            }
''', ""),
    run_fix("set-run-text"),
    run_fix("set-run-formatting"),
    ('style.based_on = non_empty(params, "basedOn");', 'style.based_on = non_empty(params, "based_on");'),
    ('''                let content_type = params.str("contentType");
                pkg.opc.set_part(&path, &content_type, params.str("content").into_bytes());''', '''                let content_type = params.str("content_type");
                pkg.opc.set_part(&path, &content_type, byte_array(params.get("bytes"))?);'''),
    ('''    /// ↩️ Reads `base` (the CURRENT, pre-mutation package) to build the spec that undoes `{kind,
    /// params}` — same law `DocxMutation::inverse` proves at the Rust-model level, computed here
    /// against the reference libraries instead.
    fn inverse_spec(base: &WPackage, kind: &str, params: &Json) -> Result<Json, String> {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);''', '''    /// ↩️ Reads `base` (the CURRENT, pre-mutation package) to build the wire specs that undo `{kind, params}` — none when
    /// there is nothing to undo — the same law `DocxMutation::inverse` proves at the Rust-model level, computed here
    /// against the reference libraries instead.
    fn inverse_spec(base: &WPackage, kind: &str, params: &Json) -> Result<Vec<Json>, String> {
        let spec = |inverse_kind: &str, inverse_params: Json| vec![Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)])];'''),
    ('''            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", obj(vec![("body", Json::Array(base.body.iter().map(block_to_json).collect())), ("styles", Json::Array(base.styles.iter().map(style_to_json).collect()))])),
''', ""),
    *inverse_run("set-run-text", None),
    *inverse_run("set-run-formatting", None),
    ('''                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-style-name" => {''', '''                    None => Vec::new(),
                }
            }
            "set-style-name" => {'''),
    ('''                            "basedOn",
                            match &style.based_on {''', '''                            "based_on",
                            match &style.based_on {'''),
    ('''                    None => spec("no-mutation", obj(vec![])),
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),''', '''                    None => Vec::new(),
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),'''),
    ('''        let base = read_package(input)?;
        let inverse = inverse_spec(&base, kind, params)?;
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))
    }''', ROUTING),
    ("    fn non_empty(value: &Json, key: &str) -> Option<String> {", BYTES),
    ('''pub fn project_docx_ecma_376(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}
''', '''pub fn project_docx_ecma_376(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 📸️ The whole package replaced by `replacement`, through the reference reader and writer. @see [`oracles::replace_package`].
#[cfg(feature = "oracles")]
pub fn oracle_replace_package(input: &[u8], replacement: &[u8]) -> Result<Vec<u8>, String> {
    oracles::replace_package(input, replacement)
}

/// 🔄️ The package read and re-written by the reference alone — its side of the identity law.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}
'''),
    (f'''pub fn project_docx_ecma_376(_bytes: &[u8]) -> Result<Json, String> {{
    {DISABLED}
}}
''', f'''pub fn project_docx_ecma_376(_bytes: &[u8]) -> Result<Json, String> {{
    {DISABLED}
}}

#[cfg(not(feature = "oracles"))]
pub fn oracle_replace_package(_input: &[u8], _replacement: &[u8]) -> Result<Vec<u8>, String> {{
    {DISABLED}
}}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {{
    {DISABLED}
}}
'''),
]

text = PATH.read_text()
if "fn resolve_run_address(" in text:
    raise SystemExit("already converted")
for old, new in EDITS:
    count = text.count(old)
    if count == 2 and old.startswith('                    Some(part) => spec("set-part"'):
        pass
    assert count == 1, (old[:100], count)
    text = text.replace(old, new)
old_part = 'Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("contentType", Json::String(part.content_type.clone())), ("content", Json::String(String::from_utf8_lossy(&part.bytes).into_owned()))])),'
assert text.count(old_part) == 2
text = text.replace(old_part, 'Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("content_type", Json::String(part.content_type.clone())), ("bytes", bytes_to_json(&part.bytes))])),')
PATH.write_text(text)
print("converted")
