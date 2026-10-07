import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const ticket = dirname(dirname(import.meta.path));
const root = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets";
const schemaFile = root + "/✳️any/🧬️schema/🔺️diff/🦀️.rs";
const ioFile = root + "/✳️any/🚪️io/📝️text/🔺️diff/🦀️.rs";
const source = readFileSync(ioFile, "utf8");
const body = (name: string) => {
  const start = source.indexOf(`pub fn ${name}(`);
  if (start < 0) throw new Error(`Missing ${name}`);
  const brace = source.indexOf("{", start);
  let depth = 1, end = brace + 1;
  for (; depth; end++) {
    if (source[end] === "{") depth++;
    if (source[end] === "}") depth--;
  }
  return source.slice(start, end);
};
const builder = body("note_block_patch_diff").replace("block_json: Some(semio_framework_pack_json::to_json_string(block))", "block: Some(block.clone())");
const apply = body("apply_blocks_delta").replace(/entry\n\s*\.patch\n\s*\.block_json[\s\S]+?\.transpose\(\)/, "Ok(entry.patch.block.clone())");
let schema = readFileSync(schemaFile, "utf8");
schema = schema.replace("pub block_json: Option<String>", "pub block: Option<NoteBlockNode>");
schema = schema.replace("/// 🩹 Sparse block field patch (JSON blob for whole-block replacement).", "/// 🩹 Sparse whole-block replacement with a canonical block value.");
schema = schema.replace(/#\[derive\(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue\)\]\n#\[serde\(rename_all = "camelCase"\)\]\n(#\[value\(rename_all = "camelCase"\)\]\npub struct NoteBlockPatchEntry)/, "#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]\n$1");
schema = schema.replace(/#\[derive\(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue\)\]\n#\[serde\(rename_all = "camelCase", default\)\]\n(#\[value\(rename_all = "camelCase", default\)\]\npub struct NoteBlockPatch)/, "#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]\n$1");
writeFileSync(schemaFile, schema + "\n/// 🩹 Builds a sparse identified block replacement.\n" + builder + "\n\n/// 🧩 Applies validated identified block collection changes.\n" + apply + "\n");
writeFileSync(ioFile, "//! 🔺️ Physical Note diff representation.\n" + source.slice(source.indexOf("//#region 📖️SemioGrammar"), source.indexOf("#[allow(unused_imports)]")) + "semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::NoteDiff);\n");
const dragSchema = root + "/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🔺️diff/🦀️.rs";
const dragIo = root + "/🧱️block/🚪️io/📝️text/🧬️mutations/🦀️.rs";
const dragSource = readFileSync(dragIo, "utf8");
const dragFn = dragSource.slice(dragSource.indexOf("pub fn diff("), dragSource.lastIndexOf("\n}\n}") + 2).replace("block_json: Some(semio_framework_pack_json::to_json_string(&moved))", "block: Some(moved)");
writeFileSync(dragSchema, readFileSync(dragSchema, "utf8") + "\n" + dragFn + "\n");
writeFileSync(dragIo, "//! 🚪️ Block mutation physical representation assembly.\n");
writeFileSync(join(ticket, "note-semantic-block-patches.md"), "# Note Semantic Block Patches\n\n`NoteBlockPatch.block` retains a typed canonical `NoteBlockNode`; it no longer carries JSON text. Whole-block patch construction, block collection validation/application, and drag mutation diff construction now live in canonical schema owners. The text/binary facets encode only at IO. TypeScript declarations and shared neutral fixtures are coordinated with the TypeScript owner. Native compilation is pending.\n\n" + [schemaFile, ioFile, dragSchema, dragIo].map(file => `- ${file}`).join("\n") + "\n");
console.log("[DEBUG] Note typed block patch helpers restored to schema");
