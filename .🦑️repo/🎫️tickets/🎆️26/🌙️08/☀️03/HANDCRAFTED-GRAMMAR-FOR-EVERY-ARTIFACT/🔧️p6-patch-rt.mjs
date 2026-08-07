#!/usr/bin/env bun
/** [DEBUG] P6: gut __rt document/inline codecs and delete op_rt. */
import { readFileSync, writeFileSync, existsSync, readdirSync } from "fs";
import { join, relative } from "path";

const repo = "/Users/ueli/Documents/semio";
const DSL_DIR = (() => {
  for (const ent of readdirSync(repo)) {
    const p = join(repo, ent, "🛍️products", "💻️os", "🔨️modules", "🗣️dsl");
    if (existsSync(p)) return p;
  }
  throw new Error("dsl dir not found");
})();
const file = join(DSL_DIR, "🦀️component.rs");

function replaceOnce(hay, needle, repl, label) {
  const i = hay.indexOf(needle);
  if (i < 0) throw new Error("missing needle: " + label);
  const j = hay.indexOf(needle, i + 1);
  if (j >= 0) throw new Error("needle not unique: " + label);
  return hay.slice(0, i) + repl + hay.slice(i + needle.length);
}

let t = readFileSync(file, "utf8");

const rtOld = `pub mod __rt {
    use super::*;

    pub fn parse_document_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Document })
    }

    pub fn print_document_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Document)
    }

    pub fn parse_inline_record(text: &str, spec: &RecordSpec) -> Result<RecordValue, TextError> {
        parse(text, spec, &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })
    }

    pub fn print_inline_record(value: &RecordValue, spec: &RecordSpec) -> String {
        print(value, spec, JoinMode::Inline)
    }

    pub fn field_error(message: impl Into<String>) -> TextError {
`;

const rtNew = `pub mod __rt {
    use super::*;

    pub fn field_error(message: impl Into<String>) -> TextError {
`;
t = replaceOnce(t, rtOld, rtNew, "__rt codec wrappers");

// Update doc comment above __rt
t = t.replace(
  `/// @emoji ⚙️ Thin wrappers the derive-generated \`impl crate::os_store::DocumentDsl\`/\`impl crate::os_spr::OpText\`
/// bodies call into — kept as free functions (not methods) so generated code never has to name
/// this crate's internal types, only \`crate::os_dsl::__rt::*\`.`,
  `/// @emoji ⚙️ Helpers remaining after P6 flag day — DslField/DslVariants derive bodies only (codec paths deleted).`,
);

const opRtStart = t.indexOf("//#region 🔖️OpRt");
const opRtEnd = t.indexOf("//#endregion 🔖️OpRt");
if (opRtStart < 0 || opRtEnd < 0) throw new Error("op_rt region markers missing");
const opRtEndClose = opRtEnd + "//#endregion 🔖️OpRt".length;
t = t.slice(0, opRtStart) +
  `//#region 🔖️OpRt
// P6: generic OpBinary runtime deleted — artifacts handcraft OpBinary against their spr protocol.
//#endregion 🔖️OpRt` +
  t.slice(opRtEndClose);

for (const bad of ["parse_document_record", "print_document_record", "parse_inline_record", "print_inline_record", "pub mod op_rt", "fn encode_op<T: DslVariants>"]) {
  if (t.includes(bad)) throw new Error("still present: " + bad);
}
if (!t.includes("pub fn field_error")) throw new Error("field_error missing");
if (!t.includes("pub fn unit_for_derive")) throw new Error("unit_for_derive missing");

writeFileSync(file, t);
console.log("patched", relative(repo, file), t.length);
