import { readFileSync, writeFileSync } from "fs";

function regionBounds(src, name) {
  const open = `//#region ${name}`;
  const close = `//#endregion ${name}`;
  // Prefer the last occurrence so module-doc mentions of the marker don't win.
  const start = src.lastIndexOf(open);
  const end = src.lastIndexOf(close);
  if (start < 0 || end < 0 || end < start) {
    throw new Error(`region ${name} missing (${start},${end})`);
  }
  return { start, end: end + close.length };
}

const head = readFileSync("/tmp/grammar-head.rs", "utf8");
const walkRaw = readFileSync("/tmp/protocol-walk.rs", "utf8");
let recImproved = readFileSync("/tmp/recognizer-improved.rs", "utf8");

const protocolModel = readFileSync("/tmp/protocol-model-snippet.rs", "utf8");
const parserTail = readFileSync("/tmp/protocol-parser-snippet.rs", "utf8");
const printProtocol = readFileSync("/tmp/protocol-print-snippet.rs", "utf8");
const testsTail = readFileSync("/tmp/protocol-tests-snippet.rs", "utf8");

let src = head;

// Insert ProtocolModel after Model (unique end marker)
{
  const marker = "//#endregion 🔖️Model";
  const idx = src.indexOf(marker);
  if (idx < 0) throw new Error("Model end missing");
  src = src.slice(0, idx + marker.length) + "\n" + protocolModel + src.slice(idx + marker.length);
}

// Replace skip_line with expect_ident_or_int
{
  const skipBlock = `    fn skip_line(&mut self) {
        while self.peek().kind != GKind::Newline && self.peek().kind != GKind::Eof {
            self.advance();
        }
        if self.peek().kind == GKind::Newline {
            self.advance();
        }
    }`;
  const orInt = `    fn expect_ident_or_int(&mut self) -> Result<GToken, TextError> {
        match self.peek().kind {
            GKind::Ident | GKind::Int => Ok(self.advance()),
            other => Err(TextError::new(format!("expected ident or int, found {other:?}"), self.peek().span.clone())),
        }
    }`;
  if (!src.includes(skipBlock)) throw new Error("skip_line missing");
  src = src.replace(skipBlock, orInt);
}

// Drop is_protocol_directive_line helper
src = src.replace(/fn is_protocol_directive_line\([\s\S]*?\n\}\n\n/, "");

// Replace from parse_grammar doc through Parser endregion
{
  const parseDoc = "/// @emoji 📖️ Parses one `.grammar` file. v1 requires every header directive and every production";
  const start = src.indexOf(parseDoc);
  const endMarker = "//#endregion 🔖️Parser";
  const end = src.indexOf(endMarker, start);
  if (start < 0 || end < 0) throw new Error(`parser replace bounds ${start} ${end}`);
  src = src.slice(0, start) + parserTail + "\n" + src.slice(end + endMarker.length);
}

// Insert print_protocol before canonicalize
{
  const canon = "/// @emoji ♻️ `canonicalize";
  const idx = src.indexOf(canon);
  if (idx < 0) throw new Error("canonicalize missing");
  src = src.slice(0, idx) + printProtocol + "\n" + src.slice(idx);
}

src = src.replace(
  `pub fn canonicalize(text: &str) -> Result<String, TextError> {
    Ok(print_grammar(&parse_grammar(text)?))
}`,
  `pub fn canonicalize(text: &str) -> Result<String, TextError> {
    if is_protocol_source(text) {
        Ok(print_protocol(&parse_protocol(text)?))
    } else {
        Ok(print_grammar(&parse_grammar(text)?))
    }
}`,
);

// Delete FromRecordSpec
{
  const b = regionBounds(src, "🔖️FromRecordSpec");
  src = src.slice(0, b.start) + src.slice(b.end).replace(/^\n+/, "\n");
}

// Fix arrow terminal matching in improved recognizer
recImproved = recImproved.replace(
  `        "EQUALS" | "EQ" => matches!(token.kind, CoreKind::Equals) || text == "=",
        "ARROW" => text == "->" || text == "→",
        "DASHARROW" => text == "-->" || text == "⟶",
        "BACKARROW" => text == "<-" || text == "←",
        "EDGEARROW" => text == "<->" || text == "<-->" || text == "↔",`,
  `        "EQUALS" | "EQ" => matches!(token.kind, CoreKind::Equals),
        "ARROW" => matches!(token.kind, CoreKind::Arrow),
        "DASHARROW" => matches!(token.kind, CoreKind::DashArrow),
        "BACKARROW" => matches!(token.kind, CoreKind::BackArrow),
        "EDGEARROW" => matches!(token.kind, CoreKind::EdgeArrow),`,
);

{
  const b = regionBounds(src, "🔖️Recognizer");
  src = src.slice(0, b.start) + recImproved + "\n" + src.slice(b.end);
}

// Build fixed walk/verify
let walk = walkRaw;
const oldWalkBody = `    let skip_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);

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
}`;

const newWalkBody = `    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
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

/// @emoji 📡️ Shallow GrammarFile back-compat check: pack requires leading 0x89 magic (any family)
/// and ≥32 bytes; spr requires non-empty bytes. Deep walks use [\`verify_protocol_source\`].
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
}`;

if (!walk.includes(oldWalkBody)) throw new Error("old walk body missing");
walk = walk.replace(oldWalkBody, newWalkBody);
// Drop envelope helper if present
{
  const envStart = walk.indexOf("/// @emoji 🛟 Shallow envelope");
  if (envStart >= 0) {
    const envEnd = walk.indexOf("\n}", envStart);
    walk = walk.slice(0, envStart) + walk.slice(envEnd + 2);
  }
}

{
  const b = regionBounds(src, "🔖️ProtocolVerify");
  src = src.slice(0, b.start) + walk + "\n" + src.slice(b.end);
}

// Delete FromRecordSpecTests
{
  const open = "    //#region 🔖️FromRecordSpecTests";
  const close = "    //#endregion 🔖️FromRecordSpecTests";
  const start = src.indexOf(open);
  const end = src.indexOf(close);
  if (start >= 0 && end >= 0) {
    src = src.slice(0, start) + src.slice(end + close.length).replace(/^\n+/, "\n");
  }
}

// Replace dialect/protocol tests
{
  const dialect = "    #[test]\n    fn parse_grammar_sets_dialect_grammar_vs_protocol()";
  const start = src.indexOf(dialect);
  const end = src.lastIndexOf("//#endregion 🔖️Tests");
  if (start < 0 || end < 0) throw new Error("tests bounds missing");
  src = src.slice(0, start) + testsTail + "\n";
}

writeFileSync("/tmp/grammar-rebuilt.rs", src);

const checks = [
  "pub fn parse_protocol",
  "pub fn print_protocol",
  "pub fn walk_protocol",
  "pub fn verify_protocol_source",
  "pub fn verify_protocol_bytes",
  "terminal_matches",
  "ProtocolFile",
  "from_record_spec",
  "//#region 🔖️Model",
  "//#region 🔖️Lexer",
  "//#region 🔖️Parser",
  "//#region 🔖️Writer",
];
for (const c of checks) console.log(c, src.includes(c));
console.log("lines", src.split("\n").length);
// Ensure file doesn't start with recognizer mid-doc
console.log("startsOk", src.startsWith("//! @emoji 📖️"));
console.log("hasSkipLine", src.includes("fn skip_line"));
