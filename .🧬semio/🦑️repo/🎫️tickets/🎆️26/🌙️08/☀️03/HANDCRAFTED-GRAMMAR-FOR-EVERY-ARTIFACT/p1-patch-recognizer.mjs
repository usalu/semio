import fs from "fs";
import path from "path";

import { fileURLToPath } from "url";
const ticket = path.dirname(fileURLToPath(import.meta.url));
const gram = fs.readFileSync(path.join(ticket, "gram.path"), "utf8").trim();
const facade = fs.readFileSync(path.join(ticket, "facade.path"), "utf8").trim();
const recognizer = fs.readFileSync(path.join(ticket, "recognizer-fragment.rs"), "utf8");

let src = fs.readFileSync(gram, "utf8");

{
  const a = src.indexOf("//#region 🔖️FromRecordSpec");
  const b = src.indexOf("//#endregion 🔖️FromRecordSpec");
  if (a < 0 || b < 0) throw new Error("FromRecordSpec missing");
  src =
    src.slice(0, a) +
    "//#region 🔖️FromRecordSpec\n// Deleted: from_record_spec / terminal_for_shape (P1/M3b).\n//#endregion 🔖️FromRecordSpec\n" +
    src.slice(b + "//#endregion 🔖️FromRecordSpec".length);
}

{
  const a = src.indexOf("//#region 🔖️Recognizer");
  const b = src.indexOf("//#endregion 🔖️Recognizer");
  if (a < 0 || b < 0) throw new Error("Recognizer missing");
  src = src.slice(0, a) + recognizer + "\n" + src.slice(b + "//#endregion 🔖️Recognizer".length);
}

if (!src.includes("pub fn verify_protocol_envelope")) {
  const endRegion = src.indexOf("//#endregion 📡️ProtocolWalk");
  if (endRegion < 0) throw new Error("ProtocolWalk end missing");
  const insert = `
/// @emoji 🛟 Shallow envelope check — any 0x89 magic (pack) or non-empty (spr).
pub fn verify_protocol_envelope(framing_hint: &str, bytes: &[u8]) -> Result<(), String> {
    if framing_hint.contains("record") || framing_hint.ends_with(".spr") {
        if bytes.is_empty() {
            return Err("spr envelope rejects empty bytes".into());
        }
        return Ok(());
    }
    if bytes.len() < 8 || bytes[0] != 0x89 {
        return Err(format!("pack envelope requires 0x89 magic prefix, got {} bytes", bytes.len()));
    }
    Ok(())
}
`;
  src = src.slice(0, endRegion) + insert + src.slice(endRegion);
}

if (!src.includes("pub fn verify_protocol_source")) {
  const marker = "pub fn verify_protocol_bytes";
  const idx = src.indexOf(marker);
  if (idx < 0) throw new Error("verify_protocol_bytes missing");
  const afterFn = src.indexOf("\n}", idx);
  const insertAt = afterFn + 2;
  const insert = `

/// @emoji 📡️ Parse protocol source then walk_protocol.
pub fn verify_protocol_source(text: &str, bytes: &[u8]) -> Result<ProtocolTrace, String> {
    let spec = parse_protocol(text).map_err(|e| e.to_string())?;
    walk_protocol(&spec, bytes).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}
`;
  src = src.slice(0, insertAt) + insert + src.slice(insertAt);
}

src = src.replace(/\n\s*\/\/#region 🔖️FromRecordSpecTests[\s\S]*?\/\/#endregion 🔖️FromRecordSpecTests\n/, "\n");

src = src.replace(
  'let p = parse_grammar("dialect protocol\\nprotocol demo.pack\\nstart frame\\n").expect("protocol");',
  'let p = parse_grammar("dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo\\nstart frame\\nframing record\\n").expect("protocol");',
);

// Append new tests before endregion if not present
if (!src.includes("fn recognizer_matches_bool_terminal")) {
  const endTests = src.indexOf("//#endregion 🔖️Tests");
  const tests = `
    #[test]
    fn parse_protocol_roundtrips_magic_pack() {
        let source = "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo.v1\\nstart frame\\nframing magic 0x8953454D0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n";
        let parsed = parse_protocol(source).expect("parse_protocol");
        let printed = print_protocol(&parsed);
        let reparsed = parse_protocol(&printed).expect("reparse");
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn walk_protocol_consumes_magic_and_header() {
        let source = "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo.v1\\nstart frame\\nframing magic 0x8953454D0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n";
        let spec = parse_protocol(source).expect("parse");
        let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&7u32.to_le_bytes());
        walk_protocol(&spec, &bytes).expect("walk");
        assert!(walk_protocol(&spec, &bytes[..8]).is_err());
    }

    #[test]
    fn walk_protocol_spr_record_body_as_rest() {
        let source = "dialect protocol\\nprotocol demo.spr\\nversion 1\\nschema demo.op\\nstart record\\nframing record\\nfield format u8\\nfield body bytes\\n";
        let spec = parse_protocol(source).expect("parse");
        walk_protocol(&spec, &[1u8, 9, 9, 9]).expect("spr walk");
    }

    #[test]
    fn recognizer_matches_bool_terminal() {
        let grammar = parse_grammar("grammar demo\\nstart doc\\ndoc = BOOL\\n").expect("grammar");
        let rec = Recognizer::compile(&grammar);
        assert_eq!(rec.recognize("true").unwrap(), true);
        assert_eq!(rec.recognize("false").unwrap(), true);
        assert_eq!(rec.recognize("maybe").unwrap(), false);
    }

    #[test]
    fn verify_protocol_source_ok() {
        let source = "dialect protocol\\nprotocol demo.pack\\nversion 1\\nschema demo.v1\\nstart frame\\nframing magic 0x8953454D0D0A1A0A\\nheader fixed 4\\nfield flags u32\\n";
        let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&0u32.to_le_bytes());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
    }

`;
  src = src.slice(0, endTests) + tests + src.slice(endTests);
}

fs.writeFileSync(gram, src);
console.log("grammar ok", src.split("\n").length);

const protoPath = path.join(path.dirname(gram), "📖️protocol.grammar.semio");
if (!fs.existsSync(protoPath)) {
  fs.writeFileSync(
    protoPath,
    `dialect grammar
grammar protocol
extension protocol.semio
start protocol-file

protocol-file = dialect-line? header-line directive*

dialect-line = "dialect" "protocol"
header-line = "protocol" IDENT
directive = version-dir | schema-dir | use-dir | start-dir | framing-dir | header-dir | field-dir | segment-dir | record-dir | struct-dir | enum-dir | footer-dir | chain-dir

version-dir = "version" INT
schema-dir = "schema" IDENT
use-dir = "use" IDENT
start-dir = "start" IDENT
framing-dir = "framing" framing-mode
framing-mode = "magic" INT | "record" | "chunked"
header-dir = "header" "fixed" INT
field-dir = "field" IDENT prim
segment-dir = "segment" IDENT segment-tail
segment-tail = "kind" "=" INT field-block? | field-block | prim
record-dir = "record" IDENT tag-clause? field-block?
tag-clause = "tag" "=" INT
struct-dir = "struct" IDENT field-block
enum-dir = "enum" IDENT enum-block
footer-dir = "footer" "fixed" INT
chain-dir = "chain" IDENT? prim
field-block = "{" field-pair* "}"
field-pair = IDENT prim
enum-block = "{" enum-variant* "}"
enum-variant = IDENT "=" INT
prim = "u8" | "u16" | "u32" | "u64" | "i32" | "i64" | "f32" | "f64" | "varint" | "zigzag" | "bytes" | "utf8" | "tag" | fixed-prim | array-prim | ref-prim
fixed-prim = "fixed" INT
array-prim = "Array" "(" prim "," count ")"
ref-prim = "Ref" "(" IDENT ")"
count = INT | "varint" | IDENT
`,
  );
  console.log("wrote", protoPath);
} else {
  console.log("protocol.grammar.semio exists");
}

let fac = fs.readFileSync(facade, "utf8");
if (!fac.includes("FragmentRegistry")) {
  if (/pub use crate::os_dsl::grammar::\{/.test(fac)) {
    fac = fac.replace(/pub use crate::os_dsl::grammar::\{([^}]+)\}/, (m, inner) => {
      const add = [
        "verify_protocol_envelope",
        "FragmentRegistry",
        "Framing",
        "Block",
        "Field",
        "Prim",
        "Count",
        "ProtocolTrace",
        "ProtocolMismatch",
        "parse_protocol",
        "print_protocol",
        "walk_protocol",
        "verify_protocol_source",
        "ProtocolFile",
      ].filter((n) => !inner.includes(n));
      return `pub use crate::os_dsl::grammar::{${inner.trim().replace(/,$/, "")}, ${add.join(", ")}}`;
    });
  } else {
    fac += `\n//#region 📡️ProtocolExports\npub use crate::os_dsl::grammar::{parse_protocol, print_protocol, walk_protocol, verify_protocol_source, verify_protocol_envelope, ProtocolFile, Framing, Block, Field, Prim, Count, ProtocolTrace, ProtocolMismatch, FragmentRegistry};\n//#endregion 📡️ProtocolExports\n`;
  }
}
fs.writeFileSync(facade, fac);

fs.appendFileSync(
  path.join(ticket, "progress-v2.md"),
  `\n\n## P1 orchestrator patch\n- FragmentRegistry + terminal_matches + macros\n- Deleted from_record_spec\n- protocol.grammar.semio + verify_protocol_envelope\n`,
);
console.log("done");
