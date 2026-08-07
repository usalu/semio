import { readFileSync, writeFileSync } from "fs";

const file =
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs";
const dsl =
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs";
const protocolGrammar =
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📖️protocol.grammar.semio";

let src = readFileSync(file, "utf8");

const fromStart = src.indexOf("//#region 🔖️FromRecordSpec");
const fromEnd = src.indexOf("//#endregion 🔖️FromRecordSpec");
if (fromStart < 0 || fromEnd < 0) throw new Error("FromRecordSpec region missing");
src = src.slice(0, fromStart) + src.slice(fromEnd + "//#endregion 🔖️FromRecordSpec".length).replace(/^\n+/, "\n");

const testsStart = src.indexOf("    //#region 🔖️FromRecordSpecTests");
const testsEnd = src.indexOf("    //#endregion 🔖️FromRecordSpecTests");
if (testsStart < 0 || testsEnd < 0) throw new Error("FromRecordSpecTests region missing");
src =
  src.slice(0, testsStart) +
  src.slice(testsEnd + "    //#endregion 🔖️FromRecordSpecTests".length).replace(/^\n+/, "\n");

src = src.replace(
  `            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                (token_kind_name(token.kind) == name.to_uppercase()).then_some(pos + 1)
            }`,
  `            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                terminal_matches(name, token).then_some(pos + 1)
            }`,
);

const oldMacros = `fn token_kind_name(kind: CoreKind) -> String {
    format!("{kind:?}").to_uppercase()
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![MacroMatcher { name: "edge", try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok() }]
}
//#endregion 🔖️Recognizer`;

const newMacros = `fn token_kind_name(kind: CoreKind) -> String {
    format!("{kind:?}").to_uppercase()
}

/// @emoji 🏷️ Maps grammar terminal class names onto \`dsl_core\` token kinds, with BOOL/QUANTITY and
/// arrow/equals sugar that are not 1:1 Debug names of [\`CoreKind\`].
fn terminal_matches(name: &str, token: &crate::os_dsl::core::SpannedToken) -> bool {
    match name.to_uppercase().as_str() {
        "BOOL" => token.kind == CoreKind::Ident && matches!(token.text.as_str().as_ref(), "true" | "false"),
        "ARROW" => token.kind == CoreKind::Arrow,
        "DASHARROW" => token.kind == CoreKind::DashArrow,
        "BACKARROW" => token.kind == CoreKind::BackArrow,
        "EDGEARROW" => token.kind == CoreKind::EdgeArrow,
        "EQUALS" => token.kind == CoreKind::Equals,
        "QUANTITY" => matches!(token.kind, CoreKind::Float | CoreKind::Int),
        other => token_kind_name(token.kind) == other,
    }
}

fn significant_tokens(text: &str) -> Option<Vec<crate::os_dsl::core::SpannedToken>> {
    let raw = core_lex(text, &Limits::default(), false).ok()?;
    Some(raw.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof).collect())
}

fn match_quantity_macro(text: &str) -> bool {
    let Some(tokens) = significant_tokens(text) else {
        return false;
    };
    match tokens.as_slice() {
        [number] if matches!(number.kind, CoreKind::Float | CoreKind::Int) => true,
        [number, unit] if matches!(number.kind, CoreKind::Float | CoreKind::Int) && unit.kind == CoreKind::Ident => {
            crate::os_dsl::core::unit_by_symbol(unit.text.as_str()).is_some()
        }
        _ => false,
    }
}

fn match_props_macro(text: &str) -> bool {
    let Some(tokens) = significant_tokens(text) else {
        return false;
    };
    if tokens.len() < 2 || tokens[0].kind != CoreKind::LBrace || tokens[tokens.len() - 1].kind != CoreKind::RBrace {
        return false;
    }
    let mut i = 1usize;
    let end = tokens.len() - 1;
    while i < end {
        if tokens[i].kind != CoreKind::Ident {
            return false;
        }
        i += 1;
        if i >= end || tokens[i].kind != CoreKind::Equals {
            return false;
        }
        i += 1;
        if i >= end {
            return false;
        }
        match tokens[i].kind {
            CoreKind::Ident | CoreKind::Int | CoreKind::Float | CoreKind::Text => i += 1,
            _ => return false,
        }
    }
    true
}

fn match_table_macro(text: &str) -> bool {
    let Some(tokens) = significant_tokens(text) else {
        return false;
    };
    let mut saw_schema = false;
    let mut saw_rows = false;
    let mut depth_bracket = 0i32;
    let mut depth_brace = 0i32;
    for token in &tokens {
        match token.kind {
            CoreKind::LBracket => depth_bracket += 1,
            CoreKind::RBracket => {
                depth_bracket -= 1;
                if depth_bracket == 0 {
                    saw_schema = true;
                }
            }
            CoreKind::LBrace => depth_brace += 1,
            CoreKind::RBrace => {
                depth_brace -= 1;
                if depth_brace == 0 && saw_schema {
                    saw_rows = true;
                }
            }
            _ => {}
        }
        if depth_bracket < 0 || depth_brace < 0 {
            return false;
        }
    }
    saw_schema && saw_rows && depth_bracket == 0 && depth_brace == 0
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![
        MacroMatcher { name: "edge", try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok() },
        MacroMatcher { name: "quantity", try_match: match_quantity_macro },
        MacroMatcher { name: "props", try_match: match_props_macro },
        MacroMatcher { name: "table", try_match: match_table_macro },
    ]
}
//#endregion 🔖️Recognizer`;

if (!src.includes(oldMacros)) throw new Error("default_macros block not found");
src = src.replace(oldMacros, newMacros);

src = src.replace(
  `plus macros that have a registered matcher. Only
/// the \`edge\` macro has one so far (backed by \`crate::os_dsl::notation::parse_edge_text\`) — every other
/// macro name (\`table\`, \`quantity\`, \`props\`, …) is accepted syntactically by the parser above but
/// has NO recognizer support yet, because the shared \`dsl_notation\` piece-parser library those
/// macros are supposed to delegate to doesn't exist yet (tracked in the ticket's progress.md).`,
  `plus macros that have a registered matcher (\`edge\`, \`quantity\`, \`props\`, \`table\`).`,
);

const oldWalk = `/// @emoji 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly \`bytes.len()\`, else returns [\`ProtocolMismatch\`] with the failing offset.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked => {}
    }

    let skip_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        if skip_records && matches!(block, Block::Record { .. }) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Segment { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Record { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => {
                let mut env = std::collections::HashMap::new();
                walk_prim(prim, bytes, &mut pos, &mut env, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// @emoji 📡️ Byte-level protocol conformance via [\`walk_protocol\`].
pub fn verify_protocol_bytes(spec: &ProtocolFile, bytes: &[u8]) -> Result<(), String> {
    walk_protocol(spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}

/// @emoji 📡️ Parses handcrafted \`.protocol.semio\` source then verifies bytes (M5 pack/spr law).
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    verify_protocol_bytes(&spec, bytes)
}
//#endregion 📡️ProtocolWalk`;

const newWalk = `/// @emoji 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly \`bytes.len()\`, else returns [\`ProtocolMismatch\`] with the failing offset. Pack framing
/// walks header/segments/footer exactly; spr \`framing record\` walks preamble fields then treats the
/// tagged-record body as the remaining bytes.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked => {}
    }

    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
    let record_body_as_rest = matches!(spec.framing, Framing::Record);
    let mut consumed_record_body = false;

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Segment { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Record { name, fields, .. } => {
                if skip_named_records && !name.is_empty() {
                    continue;
                }
                if record_body_as_rest && !name.is_empty() {
                    if !consumed_record_body {
                        pos = bytes.len();
                        consumed_record_body = true;
                    }
                    continue;
                }
                walk_fields(fields, bytes, &mut pos, reserved)?;
            }
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => {
                let mut env = std::collections::HashMap::new();
                walk_prim(prim, bytes, &mut pos, &mut env, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if record_body_as_rest && !consumed_record_body && pos < bytes.len() {
        pos = bytes.len();
    }

    if pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// @emoji 📡️ Shallow GrammarFile back-compat check: pack requires a leading \`0x89\` magic (any
/// 8-byte family) and ≥32 bytes; spr requires non-empty bytes. Deep walks use [\`verify_protocol_source\`].
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    if spec.dialect != SemioDialect::Protocol {
        return Err("verify_protocol_bytes requires dialect protocol".to_string());
    }
    let is_pack = spec.start == "frame" || spec.id.contains("pack");
    let is_spr = spec.start == "record" || spec.id.contains("spr");
    if is_pack {
        if bytes.len() < 8 {
            return Err("pack bytes shorter than magic".to_string());
        }
        if bytes[0] != 0x89 {
            return Err("pack magic must start with 0x89".to_string());
        }
        if bytes.len() < 32 {
            return Err("pack header requires 32 bytes".to_string());
        }
        return Ok(());
    }
    if is_spr {
        if bytes.is_empty() {
            return Err("spr bytes empty".to_string());
        }
        return Ok(());
    }
    Err(format!("protocol spec start '{}' is neither frame nor record", spec.start))
}

/// @emoji 📡️ Parses handcrafted \`.protocol.semio\` source then deep-walks bytes via [\`walk_protocol\`].
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    walk_protocol(&spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}
//#endregion 📡️ProtocolWalk`;

if (!src.includes(oldWalk)) throw new Error("walk/verify block not found");
src = src.replace(oldWalk, newWalk);

src = src.replace(
  `        let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_bytes(&spec, &bytes).expect("verify");`,
  `        let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
        let shallow = parse_grammar(source).expect("grammar projection");
        verify_protocol_bytes(&shallow, &bytes).expect("shallow verify");`,
);

if (!src.includes("recognizer_matches_bool_and_arrow_terminals")) {
  const anchor = "fn self_hosting_protocol_grammar_semio_parses_as_grammar()";
  const idx = src.indexOf(anchor);
  if (idx < 0) throw new Error("self_hosting_protocol test missing");
  const insert = `fn recognizer_matches_bool_and_arrow_terminals() {
        let grammar = parse_grammar(
            "grammar demo\\nstart doc\\ndoc = BOOL EQUALS QUANTITY ARROW DASHARROW BACKARROW\\n",
        )
        .expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("true = 12 -> -- <-").expect("recognize"));
        assert!(recognizer.recognize("false = 3.5 -> -- <-").expect("recognize"));
        assert!(!recognizer.recognize("yes = 12 -> -- <-").expect("recognize"));
    }

    #[test]
    fn walk_protocol_spr_tagged_record_body_as_rest() {
        let source = r#"dialect protocol
protocol demo.spr
version 1
schema demo.operation
start record
framing record
field format u8
field ordinal varint
record ObjectsAdd tag 1
field index varint
field item Object
record ObjectsRemove tag 2
field id utf8
"#;
        let spec = parse_protocol(source).expect("parse spr");
        let bytes = vec![1u8, 0x07, 0xAA, 0xBB, 0xCC];
        let trace = walk_protocol(&spec, &bytes).expect("body-as-rest");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
    }

    #[test]
    fn verify_protocol_bytes_accepts_any_0x89_pack_magic() {
        let pack = parse_grammar(
            "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo\\nstart frame\\nframing magic 0x8953504B0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n",
        )
        .expect("pack");
        let spr = parse_grammar(
            "dialect protocol\\nprotocol demo.spr\\nversion 1\\nschema demo\\nstart record\\nframing record\\nfield format u8\\n",
        )
        .expect("spr");
        let mut spk = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
        spk.extend(std::iter::repeat(0u8).take(24));
        verify_protocol_bytes(&pack, &spk).expect("SPK");
        let mut lwpl = vec![0x89, b'L', b'W', b'P', b'L', 0x0D, 0x0A, 0x1A];
        lwpl.extend(std::iter::repeat(0u8).take(24));
        verify_protocol_bytes(&pack, &lwpl).expect("any 0x89 magic");
        assert!(verify_protocol_bytes(&pack, &[0x00u8; 32]).is_err());
        assert!(verify_protocol_bytes(&spr, &[]).is_err());
        verify_protocol_bytes(&spr, &[1u8]).expect("spr non-empty");
    }

    #[test]
    `;
  src = src.slice(0, idx) + insert + src.slice(idx);
}

writeFileSync(file, src);

let dslSrc = readFileSync(dsl, "utf8");
const oldVerify = `    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {
        let Some(file) = self.parsed_protocol().map_err(|e| e.message.clone())? else {
            return Ok(());
        };
        verify_protocol_bytes(&file, bytes)
    }`;
const newVerify = `    pub fn verify_protocol(&self, bytes: &[u8]) -> Result<(), String> {
        let Some(text) = self.protocol else {
            return Ok(());
        };
        verify_protocol_source(text, bytes)
    }`;
if (dslSrc.includes(oldVerify)) {
  dslSrc = dslSrc.replace(oldVerify, newVerify);
} else if (!dslSrc.includes("verify_protocol_source(text, bytes)")) {
  const m = dslSrc.match(/pub fn verify_protocol\(&self, bytes: &\[u8\]\) -> Result<\(\), String> \{[\s\S]*?\n    \}/);
  if (!m) throw new Error("LanguageSpec::verify_protocol not found");
  dslSrc = dslSrc.replace(m[0], newVerify.trim());
}
writeFileSync(dsl, dslSrc);

let pg = readFileSync(protocolGrammar, "utf8");
if (!pg.includes('"array"')) {
  pg = pg.replace(
    `array-prim = "Array" "(" prim "," count ")"`,
    `array-prim = "array" prim | "Array" "(" prim "," count ")"`,
  );
  writeFileSync(protocolGrammar, pg);
}

console.log("patched ok");
