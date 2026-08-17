import fs from "fs";
import path from "path";

function findTicket() {
  function walk(dir) {
    for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, ent.name);
      if (ent.isDirectory()) {
        if (ent.name.includes("HANDCRAFTED-GRAMMAR")) return p;
        const f = walk(p);
        if (f) return f;
      }
    }
    return null;
  }
  return walk("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets");
}

function sliceRegionFromText(src, name) {
  const lines = src.split("\n");
  let start = -1, end = -1;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i] === "//#region " + name) start = i;
    else if (lines[i] === "//#endregion " + name && start >= 0 && end < 0) {
      end = i;
      break;
    }
  }
  if (start < 0 || end < 0) throw new Error("missing region " + name + " start=" + start + " end=" + end);
  return lines.slice(start, end + 1).join("\n");
}

const ticket = findTicket();
const target = fs.readFileSync(path.join(ticket, "gram.path"), "utf8").trim();
const src = fs.readFileSync(target, "utf8");

const fixedDocs = `//! @emoji 📖️ \`dsl_grammar\` — the self-hosted \`.grammar\` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own \`📖️grammar/📖️grammar.grammar\` is written in it and
//! parses cleanly under its own parser — see the \`self_hosting\` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see the Recognizer region's doc comment for exactly what that
//! covers today and what it does not yet).
//!
//! Depends on \`dsl_core\` only, following the same "own pre-scan lexer delegating the shared
//! alphabet to \`crate::os_dsl::core::lex\`" pattern \`math::graph::dsl\` (Jack) established — \`?\` and \`|\`
//! aren't in the shared token alphabet (a structural-DSL alphabet has no need for them), so this
//! crate's lexer pre-scans those two characters itself and hands every other run of characters to
//! \`crate::os_dsl::core::lex\` unchanged.

use crate::os_dsl::core::{lex as core_lex, Limits, TextError, TextSpan, TokenKind as CoreKind};
`;

const model = sliceRegionFromText(src, "🔖️Model");
const protocolModel = sliceRegionFromText(src, "📡️ProtocolModel");
const lexer = sliceRegionFromText(src, "🔖️Lexer");
let parser = sliceRegionFromText(src, "🔖️Parser");
let writer = sliceRegionFromText(src, "🔖️Writer");

for (const [label, text, needles] of [
  ["parser", parser, ["pub fn parse_protocol", "fn project_protocol", "fn is_protocol_source", "fn expect_ident_or_int"]],
  ["writer", writer, ["pub fn print_protocol", "pub fn canonicalize"]],
]) {
  for (const n of needles) {
    if (!text.includes(n)) throw new Error(`${label} missing ${n}`);
  }
}
if (parser.includes("fn skip_line") || parser.includes("is_protocol_directive_line")) {
  throw new Error("parser still has skip_line/directive");
}
if (!/pub fn canonicalize[\s\S]*is_protocol_source/.test(writer)) {
  throw new Error("canonicalize must route protocol via is_protocol_source");
}

const primFixed = `fn prim_fixed_width(prim: &Prim) -> Option<usize> {
    match prim {
        Prim::U8 => Some(1),
        Prim::U16 => Some(2),
        Prim::U32 | Prim::I32 | Prim::F32 => Some(4),
        Prim::U64 | Prim::I64 | Prim::F64 => Some(8),
        Prim::Fixed(n) => Some(*n),
        Prim::Varint | Prim::Zigzag | Prim::Tag | Prim::Bytes | Prim::Utf8 | Prim::Array(_, _) | Prim::Ref(_) => None,
    }
}`;

// Remove any prim_fixed_width from writer (keep single copy before walk)
writer = writer.replace(/\nfn prim_fixed_width\(prim: &Prim\) -> Option<usize> \{[\s\S]*?\n\}/g, "");
parser = parser.replace(/\nfn prim_fixed_width\(prim: &Prim\) -> Option<usize> \{[\s\S]*?\n\}/g, "");

const fromRecordStub = `//#region 🔖️FromRecordSpec
// 🗑️ \`from_record_spec\` hatch deleted — handcrafted \`.grammar.semio\` / \`.protocol.semio\` are normative.
//#endregion 🔖️FromRecordSpec`;

const recognizer = fs.readFileSync(path.join(ticket, "recognizer-fragment.rs"), "utf8").trim();

// Prefer salvage ProtocolWalk (ProtocolFile verify + envelope)
const salvage = fs.readFileSync(path.join(ticket, "salvage-walk-tests.rs"), "utf8");
let walk = sliceRegionFromText(salvage, "📡️ProtocolWalk");
if (!walk.includes("pub fn walk_protocol")) throw new Error("salvage walk missing walk_protocol");
if (!walk.includes("verify_protocol_envelope")) throw new Error("salvage walk missing envelope");
if (!walk.includes("pub fn verify_protocol_bytes(spec: &ProtocolFile")) {
  // tolerate formatting
  if (!/pub fn verify_protocol_bytes\(\s*spec:\s*&ProtocolFile/.test(walk)) {
    throw new Error("verify_protocol_bytes must take &ProtocolFile");
  }
}

// Inject prim_fixed_width at top of walk if missing
if (!walk.includes("fn prim_fixed_width")) {
  walk = walk.replace(
    "//#region 📡️ProtocolWalk",
    "//#region 📡️ProtocolWalk\n" + primFixed + "\n"
  );
}

// Build tests: salvage tests region, surgically fixed
let testsRegion = sliceRegionFromText(salvage, "🔖️Tests");
// Fix orphaned tests outside mod
if (testsRegion.includes("}\n\n    #[test]\n    fn parse_protocol_roundtrips_magic_pack")) {
  testsRegion = testsRegion.replace(
    "}\n\n    #[test]\n    fn parse_protocol_roundtrips_magic_pack",
    "\n    #[test]\n    fn parse_protocol_roundtrips_magic_pack"
  );
  if (!/\n\}\n\/\/#endregion 🔖️Tests\s*$/.test(testsRegion)) {
    testsRegion = testsRegion.replace(/\n\/\/#endregion 🔖️Tests\s*$/, "\n}\n//#endregion 🔖️Tests\n");
  }
}

const parts = [
  fixedDocs.trimEnd(),
  "",
  model,
  "",
  protocolModel,
  "",
  lexer,
  "",
  parser,
  "",
  writer,
  "",
  fromRecordStub,
  "",
  recognizer,
  "",
  walk,
  "",
  testsRegion.trimEnd(),
  "",
];

let out = parts.join("\n");

// Dedupe prim_fixed_width
{
  let count = 0;
  out = out.replace(/\nfn prim_fixed_width\(prim: &Prim\) -> Option<usize> \{[\s\S]*?\n\}/g, (m) => {
    count++;
    return count === 1 ? m : "";
  });
}

const outLines = out.split("\n");
const docPart = outLines.slice(0, outLines.findIndex((l) => l === "//#region 🔖️Model")).join("\n");
if (docPart.includes("//#region") || docPart.includes("//#endregion")) {
  throw new Error("docs still contain region markers: " + docPart);
}

const regionOrder = [
  "🔖️Model",
  "📡️ProtocolModel",
  "🔖️Lexer",
  "🔖️Parser",
  "🔖️Writer",
  "🔖️FromRecordSpec",
  "🔖️Recognizer",
  "📡️ProtocolWalk",
  "🔖️Tests",
];
const found = [];
for (let i = 0; i < outLines.length; i++) {
  const m = outLines[i].match(/^\/\/#region\s+(.+)$/);
  if (m) found.push(m[1]);
}
if (JSON.stringify(found) !== JSON.stringify(regionOrder)) {
  throw new Error("region order mismatch: " + JSON.stringify(found));
}

for (const r of [
  "pub fn parse_grammar",
  "pub fn parse_protocol",
  "pub fn print_protocol",
  "pub fn walk_protocol",
  "pub struct FragmentRegistry",
  "pub fn verify_protocol_bytes",
  "pub fn verify_protocol_source",
  "pub fn verify_protocol_envelope",
  "pub fn compile(grammar: &GrammarFile)",
]) {
  if (!out.includes(r)) throw new Error("missing " + r);
}
if (out.includes("fn skip_line") || out.includes("is_protocol_directive_line")) {
  throw new Error("still has skip_line/directive");
}
if (out.includes("pub fn from_record_spec")) {
  throw new Error("from_record_spec still present");
}
if (out.includes("FromRecordSpecTests")) {
  throw new Error("FromRecordSpecTests still present");
}
if (/pub fn verify_protocol_bytes\(\s*spec:\s*&GrammarFile/.test(out)) {
  throw new Error("verify_protocol_bytes still takes GrammarFile");
}

fs.writeFileSync(path.join(ticket, "grammar-reconstructed.rs"), out);
fs.writeFileSync(target, out);

const progressPath = path.join(ticket, "progress-v2.md");
const stamp = new Date().toISOString();
fs.appendFileSync(
  progressPath,
  `\n\n## CRITICAL RECOVERY — grammar component reconstruction (${stamp})\n\n` +
    `- Rebuilt \`🦀️component.rs\` from live Parser/Writer/ProtocolModel + \`recognizer-fragment.rs\` + salvage \`ProtocolWalk\`/Tests.\n` +
    `- Fixed module docs (no literal region-marker substring).\n` +
    `- FromRecordSpec → deleted stub only.\n` +
    `- \`verify_protocol_bytes(&ProtocolFile)\` + envelope from salvage.\n` +
    `- Wrote ticket copy \`grammar-reconstructed.rs\` (${outLines.length} lines).\n` +
    `- Confirmed symbols: parse_grammar, parse_protocol, walk_protocol, Recognizer::compile.\n`
);

console.log(JSON.stringify({
  lines: outLines.length,
  regions: found,
  target,
  copy: path.join(ticket, "grammar-reconstructed.rs"),
}, null, 2));
