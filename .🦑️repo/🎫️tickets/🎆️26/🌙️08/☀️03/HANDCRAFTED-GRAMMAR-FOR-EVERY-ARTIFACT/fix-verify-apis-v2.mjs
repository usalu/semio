import { readFileSync, writeFileSync } from "fs";
import path from "path";

const root = "/Users/ueli/Documents/semio";
const fw = require("fs").readdirSync(root).find((n) => n.endsWith("framework"));
const file = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl", "📖️grammar", "🦀️component.rs");
const dslFile = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl", "🦀️component.rs");

let src = readFileSync(file, "utf8");
let changed = [];

// 1) walk_protocol: named-record body-as-rest for Framing::Record
const oldWalkInner = `    let skip_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);

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
            Block::Record { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,`;

const newWalkInner = `    let skip_named_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);
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
            }`;

if (src.includes(oldWalkInner)) {
  src = src.replace(oldWalkInner, newWalkInner);
  changed.push("walk body-as-rest");
} else if (src.includes("record_body_as_rest") && src.includes("skip_named_records")) {
  changed.push("walk already ok");
} else {
  console.log("WARN: walk form unexpected");
  const i = src.indexOf("let skip_records");
  console.log(src.slice(i, i + 700));
}

// 2) replace verify_protocol_bytes / verify_protocol_source / remove envelope
const shallow = `/// @emoji 📡️ Shallow [\`GrammarFile\`] back-compat check: pack requires leading \`0x89\` magic
/// (any family) and ≥32 bytes; spr requires non-empty bytes. Deep walks use [\`verify_protocol_source\`].
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    let id = spec.id.to_ascii_lowercase();
    let start = spec.start.to_ascii_lowercase();
    let is_spr = start == "record" || id.contains("spr");
    let is_pack = start == "frame" || id.contains("pack") || matches!(spec.dialect, SemioDialect::Protocol) && !is_spr;
    if is_spr {
        if bytes.is_empty() {
            return Err("spr envelope rejects empty bytes".into());
        }
        return Ok(());
    }
    if is_pack || bytes.first() == Some(&0x89) {
        if bytes.len() < 32 {
            return Err(format!("pack envelope requires ≥32 bytes, got {}", bytes.len()));
        }
        if bytes[0] != 0x89 {
            return Err("pack magic must start with 0x89".into());
        }
        return Ok(());
    }
    Err(format!(
        "verify_protocol_bytes: cannot classify protocol id='{}' start='{}'",
        spec.id, spec.start
    ))
}

/// @emoji 📡️ Parses handcrafted \`.protocol.semio\` source then deep-walks bytes via [\`walk_protocol\`].
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    walk_protocol(&spec, bytes)
        .map(|_| ())
        .map_err(|e| format!("offset {}: {}", e.offset, e.message))
}
`;

const verifyStart = src.indexOf("pub fn verify_protocol_bytes");
if (verifyStart < 0) throw new Error("verify_protocol_bytes missing");
let docStart = verifyStart;
const preceding = src.lastIndexOf("/// @emoji", verifyStart);
if (preceding >= 0 && verifyStart - preceding < 400) docStart = preceding;

// cut through verify_protocol_source and optional envelope until //#endregion 📡️ProtocolWalk
const regionEnd = src.indexOf("//#endregion 📡️ProtocolWalk", docStart);
if (regionEnd < 0) throw new Error("ProtocolWalk end missing");
src = src.slice(0, docStart) + shallow + "\n" + src.slice(regionEnd);
changed.push("verify APIs");

// 3) Fix tests calling verify_protocol_bytes with ProtocolFile
if (src.includes("verify_protocol_bytes(&spec, &bytes)")) {
  src = src.replace(
    `        let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_bytes(&spec, &bytes).expect("verify");`,
    `        let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_bytes(&project_protocol(spec.clone()), &bytes).expect("shallow verify");
        verify_protocol_source(source, &bytes).expect("deep verify");`,
  );
  changed.push("shape-a test");
}

// 4) Add shallow any-0x89 test if missing
if (!src.includes("verify_protocol_bytes_accepts_any_0x89_magic")) {
  const insertAt = src.lastIndexOf("\n}\n//#endregion 🔖️Tests");
  if (insertAt < 0) throw new Error("tests end missing");
  const test = `
    #[test]
    fn verify_protocol_bytes_accepts_any_0x89_magic() {
        let g = GrammarFile {
            dialect: SemioDialect::Protocol,
            id: "demo.pack".into(),
            extension: None,
            uses: vec![],
            start: "frame".into(),
            productions: vec![],
        };
        let mut bytes = vec![0x89];
        bytes.extend(std::iter::repeat(0u8).take(31));
        verify_protocol_bytes(&g, &bytes).expect("any 0x89");
        bytes[0] = 0x00;
        assert!(verify_protocol_bytes(&g, &bytes).is_err());
        let spr = GrammarFile {
            dialect: SemioDialect::Protocol,
            id: "demo.spr".into(),
            extension: None,
            uses: vec![],
            start: "record".into(),
            productions: vec![],
        };
        verify_protocol_bytes(&spr, &[1u8]).expect("spr non-empty");
        assert!(verify_protocol_bytes(&spr, &[]).is_err());
    }
`;
  src = src.slice(0, insertAt) + test + src.slice(insertAt);
  changed.push("shallow test");
}

writeFileSync(file, src);

// 5) DSL re-exports: drop verify_protocol_envelope
let dsl = readFileSync(dslFile, "utf8");
const oldUse = `pub use crate::os_dsl::grammar::{
    parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_envelope,
    verify_protocol_source, walk_protocol, Block, Count, Field, FragmentRegistry, Framing, GrammarFile, Prim,
    ProtocolFile, ProtocolMismatch, ProtocolTrace, Recognizer, SemioDialect,
};`;
const newUse = `pub use crate::os_dsl::grammar::{
    parse_grammar, parse_protocol, print_grammar, print_protocol, verify_protocol_bytes, verify_protocol_source,
    walk_protocol, Block, Count, Field, FragmentRegistry, Framing, GrammarFile, Prim, ProtocolFile,
    ProtocolMismatch, ProtocolTrace, Recognizer, SemioDialect,
};`;
if (dsl.includes(oldUse)) {
  dsl = dsl.replace(oldUse, newUse);
  writeFileSync(dslFile, dsl);
  changed.push("dsl exports");
} else if (dsl.includes("verify_protocol_envelope")) {
  dsl = dsl.replace(/,\s*verify_protocol_envelope/, "");
  dsl = dsl.replace(/verify_protocol_envelope,\s*/, "");
  writeFileSync(dslFile, dsl);
  changed.push("dsl exports (loose)");
} else {
  changed.push("dsl exports already clean");
}

console.log("changed:", changed.join(", "));
console.log("sig", /pub fn verify_protocol_bytes\([^)]+\)/.exec(src)?.[0]);
console.log("envelope", src.includes("verify_protocol_envelope"));
console.log("body-as-rest", src.includes("record_body_as_rest"));
console.log("source walks", src.includes("walk_protocol(&spec, bytes)"));
