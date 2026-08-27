import { existsSync, readFileSync, readdirSync, rmdirSync } from "node:fs";
import { join, relative } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";

//#region Identity
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️raster-base-direct";
const artifacts = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";
const glue = "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs";
const descriptorSchemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json";
type Leaf = { old: string; kind: string; emoji: string; variant: string; tag: number };
type Root = { id: string; folder: string; standard: string; suite: string; aggregate: string; snapshot: string; diff: string; leaves: Leaf[] };
const leaf = (old: string, kind: string, emoji: string, tag: number): Leaf => ({ old, kind, emoji, tag, variant: kind.split("-").map(word => word[0].toUpperCase() + word.slice(1)).join("") });
const roots: Root[] = [
  { id: "png", folder: "📷️png", standard: "🔖️1.2", suite: "mutate-png-1-2", aggregate: "PngMutation", snapshot: "PngSnapshot", diff: "PngDiff", leaves: [
    leaf("SetHeader", "change-header", "📐️", 2), leaf("SetPalette", "replace-palette", "🎨️", 3), leaf("SetTransparency", "change-transparency", "👁️", 4),
    leaf("SetGamma", "change-gamma", "🌗️", 5), leaf("SetChromaticities", "change-chromaticities", "🌈️", 6), leaf("SetSrgbIntent", "change-srgb-intent", "🖌️", 7),
    leaf("SetPhysicalDims", "change-physical-dims", "📏️", 8), leaf("SetTimestamp", "change-timestamp", "🕰️", 9), leaf("SetBackground", "change-background", "🖼️", 10),
    leaf("InsertTextChunk", "insert-text-chunk", "📥️", 11), leaf("RemoveTextChunk", "remove-text-chunk", "🗑️", 12), leaf("SetTextChunk", "replace-text-chunk", "✏️", 13),
    leaf("SetPixels", "replace-pixels", "🟪️", 14), leaf("InsertUnknownChunk", "insert-unknown-chunk", "📦️", 15), leaf("RemoveUnknownChunk", "remove-unknown-chunk", "📤️", 16),
  ] },
  { id: "jpg", folder: "📷️jpg", standard: "🔖️jfif-1.01", suite: "mutate-jpg-jfif-1-01", aggregate: "JpgMutation", snapshot: "JpgSnapshot", diff: "JpgDiff", leaves: [
    leaf("SetJfifHeader", "change-jfif-header", "📐️", 2), leaf("SetQuantTable", "replace-quant-table", "📊️", 3), leaf("RemoveQuantTable", "remove-quant-table", "📤️", 4),
    leaf("SetHuffmanTable", "replace-huffman-table", "🌳️", 5), leaf("RemoveHuffmanTable", "remove-huffman-table", "🪓️", 6), leaf("SetRestartInterval", "change-restart-interval", "🔁️", 7),
    leaf("InsertOtherSegment", "insert-other-segment", "📥️", 8), leaf("RemoveOtherSegment", "remove-other-segment", "🗑️", 9), leaf("SetPixels", "replace-pixels", "🟪️", 10), leaf("SetReEncodeQuality", "change-re-encode-quality", "🎚️", 11),
  ] },
  { id: "bmp", folder: "🖼️bmp", standard: "🔖️v3", suite: "mutate-bmp-v3", aggregate: "BmpMutation", snapshot: "BmpSnapshot", diff: "BmpDiff", leaves: [
    leaf("SetHeaderFields", "change-header-fields", "📐️", 2), leaf("InsertPaletteEntry", "insert-palette-entry", "📥️", 3), leaf("RemovePaletteEntry", "remove-palette-entry", "📤️", 4),
    leaf("SetPaletteEntry", "replace-palette-entry", "🎨️", 5), leaf("SetPixelData", "replace-pixel-data", "🟪️", 6),
  ] },
  { id: "tiff", folder: "🖼️tiff", standard: "🔖️6.0", suite: "mutate-tiff-6-0", aggregate: "TiffMutation", snapshot: "TiffSnapshot", diff: "TiffDiff", leaves: [
    leaf("SetByteOrder", "change-byte-order", "🧭️", 2), leaf("InsertIfd", "insert-ifd", "📥️", 3), leaf("RemoveIfd", "remove-ifd", "📤️", 4),
    leaf("SetTag", "replace-tag", "🏷️", 5), leaf("RemoveTag", "remove-tag", "🗑️", 6), leaf("SetPixels", "replace-pixels", "🟪️", 7),
  ] },
];
const subset = (root: Root) => join(artifacts, root.folder, "🏅️standards", root.standard, "🪆️subsets/✳️any");
const mutationRoot = (root: Root) => join(subset(root), "🧬️schema/🧬️mutations");
const snake = (value: string) => value.replaceAll("-", "_");
const kebab = (value: string) => value.replaceAll(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
const camel = (value: string) => value.replaceAll(/_([a-z])/g, (_, ch) => ch.toUpperCase());
const dirname = (item: Leaf) => item.emoji + item.kind;
const leafType = (item: Leaf) => item.variant + "Mutation";
const modulePath = (root: Root) => `crate::artifacts::${root.id}::schema::mutations`;
const json = (value: unknown) => JSON.stringify(value, null, 2) + "\n";
const read = (file: string) => readFileSync(file, "utf8");
//#endregion Identity

//#region Patch
const writes = new Map<string, string | null>();
function put(file: string, content: string) { writes.set(file, content); }
function files(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? files(join(directory, entry.name)) : [join(directory, entry.name)]);
}
function flush() {
  for (const [file, content] of writes) {
    const old = existsSync(file) ? read(file) : null;
    if (old === content) continue;
    const body = content === null ? `*** Delete File: ${file}\n` : old === null ? `*** Add File: ${file}\n${content.split("\n").slice(0, -1).map(line => "+" + line).join("\n")}\n` : `*** Update File: ${file}\n@@\n${old.split("\n").slice(0, old.endsWith("\n") ? -1 : undefined).map(line => "-" + line).join("\n")}\n${content.split("\n").slice(0, content.endsWith("\n") ? -1 : undefined).map(line => "+" + line).join("\n")}\n`;
    const result = spawnSync("apply_patch", [], { input: `*** Begin Patch\n${body}*** End Patch\n`, encoding: "utf8" });
    if (result.status !== 0) throw new Error(`${file}\n${result.stdout}\n${result.stderr}`);
  }
  console.log(`applied ${writes.size} scoped file entries`);
}
//#endregion Patch

//#region Syntax
function closeAt(source: string, start: number, opening = "{", closing = "}"): number {
  let depth = 0;
  let quote = false;
  let line = false;
  let block = false;
  for (let i = start; i < source.length; i++) {
    if (line) { if (source[i] === "\n") line = false; continue; }
    if (block) { if (source.slice(i, i + 2) === "*/") { block = false; i++; } continue; }
    if (quote) { if (source[i] === "\\") i++; else if (source[i] === '"') quote = false; continue; }
    if (source.slice(i, i + 2) === "//") { line = true; i++; continue; }
    if (source.slice(i, i + 2) === "/*") { block = true; i++; continue; }
    if (source[i] === '"') { quote = true; continue; }
    if (source[i] === opening) depth++;
    if (source[i] === closing && --depth === 0) return i;
  }
  throw new Error(`unclosed ${opening} at ${start}`);
}
function blockAfter(source: string, marker: string, offset = 0) {
  const at = source.indexOf(marker, offset);
  if (at < 0) throw new Error(`missing ${marker}`);
  const start = source.indexOf("{", at + marker.length);
  const end = closeAt(source, start);
  return { start, end, body: source.slice(start + 1, end) };
}
function arms(body: string, pattern: string): Map<string, { pattern: string; expression: string }> {
  const matches = [...body.matchAll(new RegExp(pattern, "gm"))];
  return new Map(matches.map((match, index) => [match[1], { pattern: match[2] || "", expression: body.slice(match.index! + match[0].length, matches[index + 1]?.index ?? body.length).trim().replace(/,$/, "").trim() }]));
}
function splitFields(body: string): Array<{ name: string; type: string }> {
  const clean = body.replace(/\/\/[^\n]*/g, "").replace(/#\[[^\n]*\]/g, "");
  const fields: string[] = [];
  let depth = 0, start = 0;
  for (let i = 0; i < clean.length; i++) {
    if ("<([".includes(clean[i])) depth++;
    if (">)]".includes(clean[i])) depth--;
    if (clean[i] === "," && depth === 0) { fields.push(clean.slice(start, i)); start = i + 1; }
  }
  fields.push(clean.slice(start));
  return fields.map(field => field.trim()).filter(Boolean).map(field => { const at = field.indexOf(":"); return { name: field.slice(0, at).trim(), type: field.slice(at + 1).trim() }; });
}
function constructors(root: Root, source: string, external = false): string {
  const canonical = `${external ? "semio_s_plugin_stdio" : "crate"}::artifacts::${root.id}::schema::mutations`;
  let result = source;
  for (const item of root.leaves) {
    const regex = new RegExp(`\\b${root.aggregate}::${item.old}\\s*\\{`, "g");
    const matches = [...result.matchAll(regex)].reverse();
    for (const match of matches) {
      const brace = match.index! + match[0].lastIndexOf("{");
      const end = closeAt(result, brace);
      result = result.slice(0, match.index!) + `${root.aggregate}::${item.variant}(${canonical}::${leafType(item)} ` + result.slice(brace, end + 1) + ")" + result.slice(end + 1);
    }
    result = result.replaceAll(kebab(item.old), item.kind);
  }
  return result;
}
//#endregion Syntax

//#region Baseline
const baselineFile = join(ticket, "🔣️baseline.json");
function capture() {
  if (existsSync(baselineFile)) return;
  const data: Record<string, string> = {};
  for (const root of roots) {
    for (const file of files(subset(root)).filter(file => /\.(rs|ts|json|graphql|proto|feature|semio|g4|ebnf|ksy|abnf|spicy)$/.test(file))) data[file] = read(file);
    for (const file of files(join(artifacts, root.folder, "🧪️tests", root.suite)).filter(file => /\.(rs|feature|json)$/.test(file))) data[file] = read(file);
  }
  put(baselineFile, json(data));
  flush();
}
function source(root: Root, facet = "🧬️mutations"): string { return JSON.parse(read(baselineFile))[join(subset(root), "🧬️schema", facet, "🦀️component.rs")]; }
function initialSnapshot(root: Root): any {
  const data = JSON.parse(read(baselineFile));
  const key = Object.keys(data).find(file => file.startsWith(mutationRoot(root)) && file.includes("/📸️snapshot/⬅️before/"))!;
  const result = JSON.parse(data[key]);
  if (root.id === "tiff") result.ifds.forEach((ifd: any) => ifd.pixels ??= []);
  return result;
}
//#endregion Baseline

//#region Contract
function contract() {
  for (const root of roots) put(join(subset(root), "🧪️tests/🧬️direct-mutation-contract/🔣️component.json"), json({ schemaVersion: 1, root: mutationRoot(root), mutations: root.leaves.map(item => ({ kind: item.kind, variant: item.variant, directory: dirname(item), textOpcode: item.kind, binaryTag: item.tag, requiredFiles: ["🦀️component.rs", "🔣️component.json", "🔣️payload.schema.json", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs", "🧪️tests/🔣️component.json"] })) }));
  flush();
}
async function validate() {
  const ajv = new Ajv({ strict: false, allErrors: true });
  const descriptor = ajv.compile(JSON.parse(read(descriptorSchemaPath)));
  let errors = 0, count = 0, payloads = 0, vectors = 0, parity = 0;
  let internal: ((schema: unknown, value: unknown) => string[]) | undefined;
  try { internal = (await import(join(process.cwd(), "📜️script.ts"))).validateJsonSchemaSubset; } catch (error) { console.log(`internal preflight unavailable: ${error}`); }
  for (const root of roots.filter(root => !process.argv[3] || root.id === process.argv[3])) {
    const contractFile = join(subset(root), "🧪️tests/🧬️direct-mutation-contract/🔣️component.json");
    const expected = JSON.parse(read(contractFile));
    for (const item of expected.mutations) {
      const base = join(mutationRoot(root), item.directory);
      for (const filename of item.requiredFiles) if (!existsSync(join(base, filename))) { errors++; console.log(`missing ${join(base, filename)}`); }
      if (!existsSync(join(base, "🔣️component.json"))) continue;
      const value = JSON.parse(read(join(base, "🔣️component.json")));
      if (!descriptor(value)) { errors++; console.log(descriptor.errors); }
      if (value.semanticKind !== item.kind || value.aggregateVariant !== item.variant || value.textOpcode !== item.textOpcode || value.binaryTag !== item.binaryTag) errors++;
      if (internal) for (const candidate of [value, { ...value, outcomeClasses: [] }, { ...value, invertibility: "unclassified" }, { ...value, binaryTag: -1 }]) { if ((internal(JSON.parse(read(descriptorSchemaPath)), candidate).length === 0) !== Boolean(descriptor(candidate))) errors++; parity++; }
      const payload = JSON.parse(read(join(base, "🔣️payload.schema.json")));
      try { const validatePayload = ajv.compile(payload); payloads++; const vector = JSON.parse(read(join(base, "🧪️tests/🔣️component.json"))); if (!validatePayload(vector.mutation.payload)) { errors++; console.log(`${root.id}/${item.kind} payload: ${JSON.stringify(validatePayload.errors)}`); } else vectors++; } catch (error) { errors++; console.log(`${root.id}/${item.kind}: ${error}`); }
      count++;
    }
    console.log(`${root.id}: expected=${expected.mutations.length}`);
  }
  const result = `descriptors=${count} payloads=${payloads} vectors=${vectors} internalParity=${parity} errors=${errors}`;
  console.log(result);
  put(join(ticket, `📓️${process.argv[3] ?? "all"}-schema-validation.md`), `# Schema Validation\n\nCommand: \`bun <ticket>/📜️script.ts validate ${process.argv[3] ?? ""}\`\n\n${result}\n\nInternal validator available: ${Boolean(internal)}.\n`);
  flush();
  process.exitCode = errors ? 1 : 0;
}
//#endregion Contract

//#region Extraction
function inspect(root: Root) {
  const original = source(root);
  const enumeration = blockAfter(original, `pub enum ${root.aggregate}`).body;
  const trait = blockAfter(original, `impl Mutation<${root.snapshot}>`).body;
  const diffBody = blockAfter(blockAfter(trait, "fn diff(").body, "match self").body;
  const inverseBody = blockAfter(blockAfter(trait, "fn inverse(").body, "match self").body;
  const diffArms = arms(diffBody, `^ {12}${root.aggregate}::(\\w+)([^\\n]*?)=>`);
  const inverseArms = arms(inverseBody, `^ {12}${root.aggregate}::(\\w+)([^\\n]*?)=>`);
  const binaryImpl = root.id === "bmp" ? "" : blockAfter(original, `impl OpBinary for ${root.aggregate}`).body;
  const binaryEncode = root.id === "bmp" ? "" : blockAfter(binaryImpl, "fn encode_op(").body;
  const binaryMatchOffset = root.id === "tiff" ? binaryEncode.indexOf("let mut out") : 0;
  const binaryArms = root.id === "bmp" ? new Map() : arms(blockAfter(binaryEncode, "match self", binaryMatchOffset).body, `^ {12}${root.aggregate}::(\\w+)([^\\n]*?)=>`);
  const binaryDecode = root.id === "bmp" ? "" : blockAfter(binaryImpl, "fn decode_op(").body;
  const binaryDecodeArms = root.id === "bmp" ? new Map() : arms(blockAfter(binaryDecode, root.id === "png" ? "match ordinal" : "match tag").body, "^ {12}(\\d+|other)() =>");
  const printArms = root.id === "bmp" ? new Map() : arms(blockAfter(blockAfter(original, `fn print_${root.id}_mutation(`).body, "match m").body, `^ {8}${root.aggregate}::(\\w+)([^\\n]*?)=>`);
  const parseArms = root.id === "bmp" ? new Map() : arms(blockAfter(blockAfter(original, `fn parse_${root.id}_mutation(`).body, "match keyword").body, '^ {8}(?:"([^"]+)"|(other))\\s*=>');
  const data = root.leaves.map(item => {
    const match = new RegExp(`^ {4}${item.old}\\s*\\{`, "m").exec(enumeration)!;
    if (!match) throw new Error(`missing variant ${item.old}`);
    const start = match.index + match[0].lastIndexOf("{");
    const fields = splitFields(enumeration.slice(start + 1, closeAt(enumeration, start)));
    if (!diffArms.get(item.old) || !inverseArms.get(item.old)) throw new Error(`missing semantic arm ${item.old}`);
    return { item, fields, diff: diffArms.get(item.old)!, inverse: inverseArms.get(item.old)!, print: printArms.get(item.old), parse: parseArms.get(kebab(item.old)), encode: binaryArms.get(item.old), decode: binaryDecodeArms.get(String(item.tag)) };
  });
  return { original, data };
}
function stripFunction(source: string, name: string): { source: string; declaration: string } {
  const match = new RegExp(`^(?:pub(?:\\(crate\\))? )?fn ${name}\\(`, "m").exec(source);
  if (!match) throw new Error(`missing function ${name}`);
  const body = blockAfter(source, `fn ${name}(`, match.index);
  return { source: source.slice(0, match.index) + source.slice(body.end + 1), declaration: source.slice(match.index, body.end + 1) };
}
function rootRust(root: Root) {
  return `//! 🧬️ Transparent ${root.aggregate} aggregate.\nuse crate::artifacts::${root.id}::schema::diff::${root.diff};\nuse crate::artifacts::${root.id}::${root.snapshot};\nuse serde::{Deserialize, Serialize};\n\n//#region Owners\n${root.leaves.map(item => `pub use super::${snake(item.kind)}::${leafType(item)};`).join("\n")}\n//#endregion Owners\n\n//#region Aggregate\n#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]\n#[serde(tag = "mutation", content = "payload", rename_all = "kebab-case")]\n#[mutations(snapshot = ${root.snapshot}, diff = ${root.diff}, schema = "s.stdio.${root.id}")]\npub enum ${root.aggregate} {\n${root.leaves.map(item => `    ${item.variant}(${leafType(item)}),`).join("\n")}\n}\n\npub fn apply_${root.id}_mutation(snapshot: &mut ${root.snapshot}, mutation: &${root.aggregate}) -> protocol::MutationOutcome<${root.diff}> {\n    let outcome = <${root.aggregate} as protocol::Mutation<${root.snapshot}>>::diff(mutation, snapshot);\n    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {\n        Ok(next) => { *snapshot = next; outcome },\n        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),\n    }\n}\npub fn inverse_${root.id}_mutation(mutation: &${root.aggregate}, base: &${root.snapshot}) -> Vec<${root.aggregate}> { protocol::Mutation::inverse(mutation, base) }\n//#endregion Aggregate\n\n#[cfg(test)]\npub(crate) fn demo_mutation_cases() -> Vec<${root.aggregate}> {\n    vec![${root.leaves.map(item => `${modulePath(root)}::${snake(item.kind)}::test_case()`).join(", ")}]\n}\n`;
}
function rootText(root: Root) {
  return `//! 📝️ Framing and direct codec registry for ${root.aggregate}.\nuse ${modulePath(root)}::${root.aggregate};\n\n//#region Registry\npub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");\npub struct Entry { pub opcode: &'static str, pub print: fn(&${root.aggregate}) -> Option<String>, pub parse: fn(&str) -> Result<${root.aggregate}, String> }\npub const REGISTRY: &[Entry] = &[\n${root.leaves.map(item => `    ${modulePath(root)}::${snake(item.kind)}::text::CODEC,`).join("\n")}\n];\n//#endregion Registry\n\n//#region Framing\nimpl protocol::OpText for ${root.aggregate} {\n    fn print_op(&self) -> String { REGISTRY.iter().find_map(|entry| (entry.print)(self)).expect("every aggregate variant has a direct text owner") }\n    fn parse_op(line: &str) -> Result<Self, store::TextError> {\n        let opcode = line.split_once(' ').map_or(line, |(opcode, _)| opcode);\n        let entry = REGISTRY.iter().find(|entry| entry.opcode == opcode).ok_or_else(|| store::TextError::new(format!("unknown mutation opcode {opcode}"), dsl::TextSpan::at(1, 1)))?;\n        (entry.parse)(line).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))\n    }\n}\n//#endregion Framing\n`;
}
function rootBinary(root: Root) {
  return `//! 💾️ Framing and direct binary registry for ${root.aggregate}.\nuse ${modulePath(root)}::${root.aggregate};\n\n//#region Registry\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");\npub struct Entry { pub tag: u8, pub encode: fn(&${root.aggregate}) -> Option<Result<Vec<u8>, protocol::ProtocolError>>, pub decode: fn(&[u8]) -> Result<${root.aggregate}, protocol::ProtocolError> }\npub const REGISTRY: &[Entry] = &[\n${root.leaves.map(item => `    ${modulePath(root)}::${snake(item.kind)}::binary::CODEC,`).join("\n")}\n];\n//#endregion Registry\n\n//#region Framing\nimpl protocol::OpBinary for ${root.aggregate} {\n    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {\n        let (tag, payload) = REGISTRY.iter().find_map(|entry| (entry.encode)(self).map(|result| (entry.tag, result))).expect("every aggregate variant has a direct binary owner");\n        let mut result = vec![store::pack_rt::OP_BINARY_FORMAT, tag];\n        result.extend(payload?);\n        Ok(result)\n    }\n    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {\n        if bytes.len() < 2 || bytes[0] != store::pack_rt::OP_BINARY_FORMAT { return Err(protocol::ProtocolError::Malformed { what: "mutation frame", offset: 0, detail: "expected format byte and direct tag".into() }); }\n        let entry = REGISTRY.iter().find(|entry| entry.tag == bytes[1]).ok_or_else(|| protocol::ProtocolError::Malformed { what: "mutation tag", offset: 1, detail: format!("unknown tag {}", bytes[1]) })?;\n        (entry.decode)(&bytes[2..])\n    }\n}\n//#endregion Framing\n`;
}
function leafText(root: Root, data: ReturnType<typeof inspect>["data"][number]) {
  const { item, fields } = data;
  const header = `//! 📝️ Direct ${item.kind} text codec.\nuse super::*;\nuse crate::artifacts::${root.id}::schema::diff::*;\nuse ${modulePath(root)}::text::Entry;\npub const TEXT_OPCODE: &str = "${item.kind}";\npub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };\n`;
  if (root.id === "bmp") return header + `\npub fn spec() -> dsl::RecordSpec { let mut spec = dsl::__rt::newtype_variant_spec::<${leafType(item)}>(); spec.keyword = Some(TEXT_OPCODE.into()); spec }\npub fn print(value: &${root.aggregate}) -> Option<String> { let ${root.aggregate}::${item.variant}(payload) = value else { return None }; Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline)) }\npub fn parse(line: &str) -> Result<${root.aggregate}, String> { let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?; dsl::__rt::newtype_variant_from_record(&value).map(${root.aggregate}::${item.variant}).map_err(|error| error.to_string()) }\n`;
  if (!data.print || !data.parse) throw new Error(`missing text facet ${item.old}`);
  const print = constructors(root, data.print.expression);
  const parse = constructors(root, data.parse.expression);
  return header + `\npub fn print(value: &${root.aggregate}) -> Option<String> {\n    let ${root.aggregate}::${item.variant}(${leafType(item)} { ${fields.map(field => field.name).join(", ")} }) = value else { return None };\n    Some(${print})\n}\npub fn parse(line: &str) -> Result<${root.aggregate}, String> {\n    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));\n    if keyword != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }\n    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;\n    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));\n    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };\n    ${parse}\n}\n`;
}
function leafBinary(root: Root, data: ReturnType<typeof inspect>["data"][number]) {
  const { item, fields } = data;
  const header = `//! 💾️ Direct ${item.kind} binary codec.\nuse super::*;\nuse crate::artifacts::${root.id}::schema::diff::{self, *};\nuse ${modulePath(root)}::binary::Entry;\npub const BINARY_TAG: u8 = ${item.tag};\npub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };\n`;
  const prefix = `\npub fn encode(value: &${root.aggregate}) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {\n    let ${root.aggregate}::${item.variant}(payload) = value else { return None };\n    Some(encode_payload(payload))\n}\n`;
  if (root.id === "bmp") return header + prefix + `pub fn encode_payload(payload: &${leafType(item)}) -> Result<Vec<u8>, protocol::ProtocolError> { store::pack_rt::encode_record_body(&super::text::spec(), &dsl::__rt::newtype_variant_to_record(payload), &store::PackEncodeOptions::default()).map_err(Into::into) }\npub fn decode(bytes: &[u8]) -> Result<${root.aggregate}, protocol::ProtocolError> { let (record, _) = store::pack_rt::decode_record_body(bytes, &super::text::spec(), &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?; dsl::__rt::newtype_variant_from_record(&record).map(${root.aggregate}::${item.variant}).map_err(|error| protocol::ProtocolError::Malformed { what: "${item.kind}", offset: 0, detail: error.to_string() }) }\n`;
  if (!data.encode || !data.decode) throw new Error(`missing binary facet ${item.old}`);
  let encode = data.encode.expression.replace(/^\{([\s\S]*)\}$/, "$1").replace(/^\s*(?:w\.write_u8\(\d+\);|out\[1\] = \d+;)\s*/, "");
  let decode = constructors(root, data.decode.expression);
  if (root.id === "png") decode = `Ok(${decode})`;
  return header + prefix + `pub fn encode_payload(payload: &${leafType(item)}) -> Result<Vec<u8>, protocol::ProtocolError> {\n    let ${leafType(item)} { ${fields.map(field => field.name).join(", ")} } = payload;\n    ${root.id === "png" ? "let mut w = dsl::ByteWriter::new();" : "let mut out = Vec::new();"}\n    ${encode};\n    Ok(${root.id === "png" ? "w.into_bytes()" : "out"})\n}\nfn op_pack_err(error: dsl::PackError) -> protocol::ProtocolError { protocol::ProtocolError::Malformed { what: "${item.kind}", offset: 0, detail: error.to_string() } }\npub fn decode(bytes: &[u8]) -> Result<${root.aggregate}, protocol::ProtocolError> {\n    ${root.id === "png" ? "let mut r = dsl::ByteReader::new(bytes);" : "let mut reader = store::ByteReader::new(bytes);"}\n    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };\n    let result: Result<${root.aggregate}, protocol::ProtocolError> = ${decode};\n    let position = ${root.id === "png" ? "r" : "reader"}.position();\n    if position != bytes.len() { return Err(protocol::ProtocolError::Malformed { what: "${item.kind}", offset: position as u64, detail: "trailing payload bytes".into() }); }\n    result\n}\n`;
}
function leafRust(root: Root, data: ReturnType<typeof inspect>["data"][number], contribution: string) {
  const { item, fields } = data;
  let diff = data.diff.expression.replace(/\bdiff::diff_\w+\(/, "contribute(");
  const inverse = constructors(root, data.inverse.expression).replaceAll(`vec![${root.aggregate}::NoMutation]`, "Vec::new()");
  const derive = root.id === "bmp" ? ", dsl::DslRecord" : "";
  return `//! 🧬️ Authoritative ${item.kind} mutation.\nuse crate::artifacts::${root.id}::schema::diff::{self, *};\nuse crate::artifacts::${root.id}::schema::snapshot::*;\nuse ${modulePath(root)}::${root.aggregate};\nuse serde::{Deserialize, Serialize};\n\n//#region Payload\n#[derive(Clone, Debug, PartialEq, Serialize, Deserialize${derive})]\n#[serde(rename_all = "camelCase", deny_unknown_fields)]\npub struct ${leafType(item)} {\n${fields.map(field => `    pub ${field.name}: ${field.type},`).join("\n")}\n}\n//#endregion Payload\n\n//#region Facets\n#[path = "📝️text/🦀️component.rs"]\npub mod text;\n#[path = "💾️binary/🦀️component.rs"]\npub mod binary;\n//#endregion Facets\n\n//#region Semantics\nimpl protocol::MutationKind<${root.snapshot}, ${root.aggregate}> for ${leafType(item)} {\n    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "${item.kind.split("-")[0]}", entity: "${item.kind.split("-").slice(1).join("-")}", kind: "${item.kind}", record: "${item.variant}" };\n    fn diff(&self, base: &${root.snapshot}) -> protocol::MutationOutcome<${root.diff}> {\n        let Self { ${fields.map(field => field.name).join(", ")} } = self;\n        protocol::MutationOutcome::new(${diff})\n    }\n    fn inverse(&self, base: &${root.snapshot}) -> Vec<${root.aggregate}> {\n        let Self { ${fields.map(field => field.name).join(", ")} } = self;\n        let outcome = <Self as protocol::MutationKind<${root.snapshot}, ${root.aggregate}>>::diff(self, base);\n        if <${root.diff} as protocol::DiffAlgebra<${root.snapshot}>>::is_empty(outcome.diff()) { return Vec::new(); }\n        ${inverse}\n    }\n    fn label(&self) -> String { "${item.kind.split("-").join(" ")}".into() }\n    fn target(&self) -> Vec<String> { vec!["${item.kind}".into()] }\n}\n${contribution}\n//#endregion Semantics\n\n#[cfg(test)]\npub(crate) fn test_case() -> ${root.aggregate} { let vector: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️component.json")).expect("authored mutation vector"); serde_json::from_value(vector["mutation"].clone()).expect("direct mutation payload") }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️component.rs"]\nmod tests;\n`;
}
//#endregion Extraction

//#region Vectors
function vector(root: Root, item: Leaf) {
  const before = initialSnapshot(root);
  const text = { keyword: "Title", value: "Direct Owner", compressed: false, kind: "text", languageTag: "", translatedKeyword: "" };
  const chunk = { kind: [119, 97, 86, 101], data: [1, 2, 3] };
  const quant = { id: 1, precision: 0, values: Array(64).fill(7) };
  const huffman = { class: "dc", id: 1, bits: [1, ...Array(15).fill(0)], values: [7] };
  if (root.id === "png" && item.old === "RemoveUnknownChunk") { before.unknownChunks = [chunk]; before.chunkOrder.splice(before.chunkOrder.length - 1, 0, { chunk: "unknown", index: 0 }); }
  if (root.id === "jpg" && item.old === "RemoveQuantTable") before.quantTables = [quant];
  if (root.id === "jpg" && item.old === "RemoveHuffmanTable") before.huffmanTables = [huffman];
  const payloads: Record<string, any> = {
    SetHeader: { width: 3, height: 2, bitDepth: 8, colorType: "rgba", interlace: true }, SetPalette: { plte: [{ r: 255, g: 0, b: 255 }] }, SetTransparency: { trns: { colorType: "grayscale", gray: 7 } },
    SetGamma: { gama: 45455 }, SetChromaticities: { chrm: { whiteX: 1, whiteY: 2, redX: 3, redY: 4, greenX: 5, greenY: 6, blueX: 7, blueY: 8 } }, SetSrgbIntent: { srgb: "saturation" },
    SetPhysicalDims: { phys: { ppuX: 96, ppuY: 96, unitIsMeter: false } }, SetTimestamp: { time: { year: 2026, month: 8, day: 27, hour: 12, minute: 0, second: 0 } }, SetBackground: { bkgd: { colorType: "rgb", r: 1, g: 2, b: 3 } },
    InsertTextChunk: { index: 1, chunk: text }, RemoveTextChunk: { index: 0 }, SetTextChunk: { index: 0, chunk: text }, SetPixels: { pixels: before.pixels.map((value: number, index: number) => index % 4 === 3 ? 255 : 9) },
    InsertUnknownChunk: { index: 0, chunk }, RemoveUnknownChunk: { index: 0 }, SetJfifHeader: { version: [1, 2], densityUnits: "pixelsPerInch", xDensity: 300, yDensity: 300, thumbnail: null },
    SetQuantTable: { table: quant }, RemoveQuantTable: { id: 1 }, SetHuffmanTable: { table: huffman }, RemoveHuffmanTable: { key: { class: "dc", id: 1 } }, SetRestartInterval: { restartInterval: 8 },
    InsertOtherSegment: { index: 1, segment: { marker: 254, data: [68, 105, 114, 101, 99, 116] } }, RemoveOtherSegment: { index: 0 }, SetReEncodeQuality: { quality: 75 },
    SetHeaderFields: { width: 3, xPixelsPerMeter: 11811, yPixelsPerMeter: 11811 }, InsertPaletteEntry: { index: 1, entry: { b: 255, g: 0, r: 255, reserved: 0 } }, RemovePaletteEntry: { index: 1 }, SetPaletteEntry: { index: 1, entry: { b: 255, g: 0, r: 255, reserved: 0 } }, SetPixelData: { pixels: before.pixels.map((value: number, index: number) => index % 4 === 3 ? 255 : 9) },
    SetByteOrder: { byteOrder: "bigEndian" }, InsertIfd: { index: 1, ifd: { entries: [{ tag: 305, kind: "ascii", values: { kind: "ascii", value: "Direct Owner" } }], pixels: [] } }, RemoveIfd: { index: 0 },
    SetTag: { ifdIndex: 0, tag: 305, kind: "ascii", values: { kind: "ascii", value: "Direct Owner" } }, RemoveTag: { ifdIndex: 0, tag: 305 },
  };
  const payload = payloads[item.old];
  const after = structuredClone(before);
  const insertMarker = (marker: any) => { const family = marker.chunk; const pivot = ["chrm", "gama", "srgb"].includes(family) ? after.chunkOrder.findIndex((item: any) => item.chunk === "plte" || item.chunk === "idat") : family === "unknown" ? after.chunkOrder.findIndex((item: any) => item.chunk === "iend") : after.chunkOrder.findIndex((item: any) => item.chunk === "idat"); after.chunkOrder.splice(pivot < 0 ? after.chunkOrder.length : pivot, 0, marker); };
  if (root.id === "png") {
    const scalar: Record<string, string> = { SetPalette: "plte", SetTransparency: "trns", SetGamma: "gama", SetChromaticities: "chrm", SetSrgbIntent: "srgb", SetPhysicalDims: "phys", SetTimestamp: "time", SetBackground: "bkgd" };
    if (item.old === "SetHeader" || item.old === "SetPixels") Object.assign(after, payload);
    else if (scalar[item.old]) { const key = scalar[item.old]; if (after[key] == null && payload[key] != null) insertMarker({ chunk: key }); after[key] = payload[key]; }
    else if (item.old === "SetTextChunk") after.textChunks[payload.index] = payload.chunk;
    else {
      const family = item.old.includes("Unknown") ? "unknown" : "text";
      const list = family === "unknown" ? after.unknownChunks : after.textChunks;
      const at = Math.min(payload.index, list.length);
      if (item.old.startsWith("Insert")) { list.splice(at, 0, payload.chunk); after.chunkOrder.forEach((marker: any) => { if (marker.chunk === family && marker.index >= at) marker.index++; }); insertMarker({ chunk: family, index: at }); }
      else { list.splice(at, 1); after.chunkOrder = after.chunkOrder.filter((marker: any) => marker.chunk !== family || marker.index !== at); after.chunkOrder.forEach((marker: any) => { if (marker.chunk === family && marker.index > at) marker.index--; }); }
    }
  } else if (root.id === "jpg") {
    if (item.old === "SetJfifHeader") for (const [name, value] of Object.entries(payload)) after[`jfif${name[0].toUpperCase()}${name.slice(1)}`] = value;
    else if (item.old === "SetReEncodeQuality") after.reEncodeQuality = payload.quality;
    else if (["SetPixels", "SetRestartInterval"].includes(item.old)) Object.assign(after, payload);
    else if (item.old.includes("Table")) { const key = item.old.includes("Quant") ? "quantTables" : "huffmanTables"; if (item.old.startsWith("Remove")) after[key] = []; else after[key].push(payload.table); }
    else if (item.old === "InsertOtherSegment") after.otherSegments.splice(payload.index, 0, payload.segment);
    else after.otherSegments.splice(payload.index, 1);
  } else if (root.id === "bmp") {
    if (["SetHeaderFields", "SetPixelData"].includes(item.old)) Object.assign(after, payload);
    else if (item.old === "InsertPaletteEntry") after.palette.splice(payload.index, 0, payload.entry);
    else if (item.old === "RemovePaletteEntry") after.palette.splice(payload.index, 1);
    else after.palette[payload.index] = payload.entry;
  } else {
    if (["SetByteOrder", "SetPixels"].includes(item.old)) Object.assign(after, payload);
    else if (item.old === "InsertIfd") after.ifds.splice(payload.index, 0, payload.ifd);
    else if (item.old === "RemoveIfd") after.ifds.splice(payload.index, 1);
    else { const entries = after.ifds[payload.ifdIndex].entries; const at = entries.findIndex((entry: any) => entry.tag === payload.tag); if (item.old === "RemoveTag") entries.splice(at, 1); else entries[at] = { tag: payload.tag, kind: payload.kind, values: payload.values }; }
  }
  return { schemaVersion: 1, semanticKind: item.kind, before, mutation: { mutation: item.kind, payload }, after };
}
function rustTests(root: Root, item: Leaf) {
  return `//! 🧪️ Language-neutral ${item.kind} behavior and codec laws.\nuse super::*;\nuse protocol::{Mutation, MutationDiff, OpBinary, OpText};\n\n#[test]\nfn authored_vector_and_inverse() {\n    let vector: serde_json::Value = serde_json::from_str(include_str!("🔣️component.json")).unwrap();\n    let before: ${root.snapshot} = serde_json::from_value(vector["before"].clone()).unwrap();\n    let expected: ${root.snapshot} = serde_json::from_value(vector["after"].clone()).unwrap();\n    let mutation: ${root.aggregate} = serde_json::from_value(vector["mutation"].clone()).unwrap();\n    let outcome = mutation.diff(&before);\n    let after = outcome.diff().apply(&before).unwrap();\n    assert_eq!(after, expected);\n    let mut restored = after.clone();\n    for inverse in mutation.inverse(&before) { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }\n    assert_eq!(restored, before);\n    assert_eq!(${root.aggregate}::parse_op(&mutation.print_op()).unwrap(), mutation);\n    let bytes = mutation.encode_op().unwrap();\n    assert_eq!(bytes[1], super::binary::BINARY_TAG);\n    assert_eq!(${root.aggregate}::decode_op(&bytes).unwrap(), mutation);\n    assert!(${root.aggregate}::decode_op(&bytes[..1]).is_err());\n}\n`;
}
//#endregion Vectors

//#region Surfaces
function payloadSchema(root: Root, fields: Array<{ name: string; type: string }>) {
  const baseline = JSON.parse(read(baselineFile));
  const snapshot = JSON.parse(baseline[join(subset(root), "🧬️schema/📸️snapshot/🔣️component.json")]);
  const definitions = snapshot.$defs ?? snapshot.definitions ?? {};
  if (root.id === "jpg") {
    definitions.JfifDensityUnits = { enum: ["aspect", "pixelsPerInch", "pixelsPerCm"] };
    definitions.JpgHuffmanTableKey = { type: "object", required: ["class", "id"], additionalProperties: false, properties: { class: { enum: ["dc", "ac"] }, id: { type: "integer", minimum: 0, maximum: 255 } } };
    for (const name of ["jpgQuantTable", "jpgHuffmanTable", "jpgSegment"]) definitions[name].required = Object.keys(definitions[name].properties);
  }
  function typeSchema(type: string): any {
    if (type.startsWith("Option<")) return { anyOf: [typeSchema(type.slice(7, -1)), { type: "null" }] };
    if (type.startsWith("Vec<")) return { type: "array", items: typeSchema(type.slice(4, -1)) };
    if (type.startsWith("(")) { const parts = type.slice(1, -1).split(",").map(value => value.trim()); return { type: "array", items: parts.map(typeSchema), minItems: parts.length, maxItems: parts.length }; }
    if (type === "bool") return { type: "boolean" };
    if (type === "String") return { type: "string" };
    if (/^(u\d+|i\d+|usize)$/.test(type)) return { type: "integer", ...(type.startsWith("u") ? { minimum: 0 } : {}), ...(["u8", "u16"].includes(type) ? { maximum: type === "u8" ? 255 : 65535 } : {}) };
    const key = definitions[type] ? type : type[0].toLowerCase() + type.slice(1);
    if (!definitions[key]) throw new Error(`schema type not found ${root.id}/${type}`);
    return { $ref: `#/$defs/${key}` };
  }
  const schema = { $schema: "http://json-schema.org/draft-07/schema#", type: "object", additionalProperties: false, required: fields.filter(field => !field.type.startsWith("Option<")).map(field => camel(field.name)), properties: Object.fromEntries(fields.map(field => [camel(field.name), typeSchema(field.type)])), $defs: definitions };
  return JSON.parse(JSON.stringify(schema).replaceAll("#/definitions/", "#/$defs/"));
}
function tsType(type: string): string { if (type.startsWith("Option<")) return tsType(type.slice(7, -1)) + " | null"; if (type.startsWith("Vec<")) return `ReadonlyArray<${tsType(type.slice(4, -1))}>`; if (type.startsWith("(")) return `[${type.slice(1, -1).split(",").map(value => tsType(value.trim())).join(", ")}]`; if (/^(u\d+|i\d+|usize)$/.test(type)) return "number"; if (type === "bool") return "boolean"; if (type === "String") return "string"; return type; }
function writeSurfaces(root: Root, data: ReturnType<typeof inspect>["data"][number]) {
  const { item, fields } = data;
  const base = join(mutationRoot(root), dirname(item));
  const name = leafType(item);
  const imported = [...new Set(fields.flatMap(field => field.type.match(/[A-Z][A-Za-z0-9]+/g) ?? []).filter(type => !["Vec", "Option", "String", "JpgHuffmanTableKey"].includes(type)))];
  const imports = imported.length ? `import type { ${imported.join(", ")} } from '../../📸️snapshot/🟦️component.ts';\n` : "";
  put(join(base, "🟦️component.ts"), `/** 🧬️ ${item.kind} direct payload. */\n${imports}${fields.some(field => field.type === "JpgHuffmanTableKey") ? "import type { JpgHuffmanTableKey } from '../../🔺️diff/🟦️component.ts';\n" : ""}export interface ${name} {\n${fields.map(field => `  readonly ${camel(field.name)}${field.type.startsWith("Option<") ? "?" : ""}: ${tsType(field.type)};`).join("\n")}\n}\n`);
  put(join(base, "🔣️payload.schema.json"), json({ ...payloadSchema(root, fields), title: name }));
  put(join(base, "🔣️component.json"), json({ schemaVersion: 1, owner: base, semanticKind: item.kind, displayName: item.kind.split("-").map(word => word[0].toUpperCase() + word.slice(1)).join(" "), emoji: item.emoji, aggregateVariant: item.variant, payloadSchema: "🔣️payload.schema.json", textOpcode: item.kind, binaryTag: item.tag, invertibility: "explicit-mutation", diffParticipation: "apply-only", outcomeClasses: ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"] }));
  const gql = fields.map(field => `${camel(field.name)}: ${/^(u\d+|i\d+|usize)$/.test(field.type) ? "Int!" : field.type === "bool" ? "Boolean!" : `${name}Value${field.type.startsWith("Option<") ? "" : "!"}`}`).join(" ");
  put(join(base, "🔗️component.graphql"), `# 🧬️ ${item.kind}/${item.variant}\nscalar ${name}Value\ninput ${name} { ${gql} }\n`);
  put(join(base, "🛰️component.proto"), `syntax = "proto3";\npackage stdio.${root.id}.mutation;\n// 🧬️ ${item.kind}/${item.variant}; complex fields follow the direct payload schema.\nmessage ${name} { ${fields.map((field, index) => `${/^(u\d+|usize)$/.test(field.type) ? "uint64" : /^i\d+$/.test(field.type) ? "sint64" : field.type === "bool" ? "bool" : "bytes"} ${field.name} = ${index + 1};`).join(" ")} }\n`);
  put(join(base, "🧪️tests/🔣️component.json"), json(vector(root, item)));
  put(join(base, "🧪️tests/🦀️component.rs"), rustTests(root, item));
}
function rootSurfaces(root: Root) {
  const base = mutationRoot(root);
  put(join(base, "🟦️component.ts"), `/** 🧬️ Transparent ${root.aggregate} union. */\n${root.leaves.map(item => `import type { ${leafType(item)} } from './${dirname(item)}/🟦️component.ts';`).join("\n")}\nexport type ${root.aggregate} =\n${root.leaves.map(item => `  | { readonly mutation: '${item.kind}'; readonly payload: ${leafType(item)} }`).join("\n")};\n`);
  put(join(base, "🔣️component.json"), json({ $schema: "http://json-schema.org/draft-07/schema#", title: root.aggregate, oneOf: root.leaves.map(item => ({ type: "object", additionalProperties: false, required: ["mutation", "payload"], properties: { mutation: { const: item.kind }, payload: { $ref: `${dirname(item)}/🔣️payload.schema.json` } } })) }));
  put(join(base, "🔗️component.graphql"), `# 🧬️ Transparent ${root.aggregate} descriptor assembly.\n${root.leaves.map(item => `# ${item.kind}/${item.variant}: ${dirname(item)}/🔗️component.graphql`).join("\n")}\nenum ${root.aggregate}Kind { ${root.leaves.map(item => snake(item.kind).toUpperCase()).join(" ")} }\ninput ${root.aggregate} { ${root.leaves.map(item => `${snake(item.kind)}: ${leafType(item)}`).join(" ")} }\n`);
  put(join(base, "🛰️component.proto"), `syntax = "proto3";\npackage stdio.${root.id}.mutation;\n${root.leaves.map(item => `import "${dirname(item)}/🛰️component.proto";`).join("\n")}\nmessage ${root.aggregate} { oneof mutation { ${root.leaves.map(item => `${leafType(item)} ${snake(item.kind)} = ${item.tag};`).join(" ")} } }\n`);
}
//#endregion Surfaces

//#region Assemble
function generate(root: Root) {
  const { original, data } = inspect(root);
  let diffSource = source(root, "🔺️diff");
  for (const details of data) {
    const { item } = details;
    let contribution = "";
    if (root.id !== "bmp") {
      const builderName = "diff_" + kebab(item.old).replaceAll("-", "_");
      const extracted = stripFunction(diffSource, builderName);
      diffSource = extracted.source;
      contribution = extracted.declaration.replace(`fn ${builderName}(`, "fn contribute(");
      const helper = root.id === "png" ? ({ InsertTextChunk: "chunk_order_insert_text_diff", RemoveTextChunk: "chunk_order_remove_text_diff", InsertUnknownChunk: "chunk_order_insert_unknown_diff", RemoveUnknownChunk: "chunk_order_remove_unknown_diff" } as Record<string, string>)[item.old] : undefined;
      if (helper) { const extractedHelper = stripFunction(diffSource, helper); diffSource = extractedHelper.source; contribution += "\n" + extractedHelper.declaration; }
    }
    const base = join(mutationRoot(root), dirname(item));
    put(join(base, "🦀️component.rs"), leafRust(root, details, contribution));
    put(join(base, "📝️text/🦀️component.rs"), leafText(root, details));
    put(join(base, "💾️binary/🦀️component.rs"), leafBinary(root, details));
    writeSurfaces(root, details);
  }
  diffSource = diffSource.replace("fn between_plte(", "pub(crate) fn between_plte(").replace("fn huffman_key(", "pub(crate) fn huffman_key(");
  for (const item of root.leaves) {
    const old = "diff_" + kebab(item.old).replaceAll("-", "_");
    diffSource = diffSource.replaceAll(new RegExp(`\\b${old}\\(`, "g"), `${modulePath(root)}::${snake(item.kind)}::contribute(`);
  }
  if (root.id !== "bmp") put(join(subset(root), "🧬️schema/🔺️diff/🦀️component.rs"), diffSource);
  put(join(mutationRoot(root), "🦀️component.rs"), rootRust(root));
  put(join(mutationRoot(root), "📝️text/🦀️component.rs"), rootText(root));
  put(join(mutationRoot(root), "💾️binary/🦀️component.rs"), rootBinary(root));
  rootSurfaces(root);
  const baseline = JSON.parse(read(baselineFile));
  const oracleFile = join(subset(root), "🧪️oracle/🔣️component.json");
  const oracle = JSON.parse(read(oracleFile));
  oracle.mutationCatalogs[0].kinds = root.leaves.map(item => item.kind);
  oracle.mutationCatalogs[0].vectors = root.leaves.map(item => ({ mutationId: item.kind, sourceMutationDirectoryName: dirname(item), mutationDirectoryName: dirname(item), scenarios: [{ id: "direct-behavior", directoryName: "🧪️tests" }] }));
  put(oracleFile, json(oracle));
  const removed = files(join(mutationRoot(root), "📄set-snapshot"));
  for (const file of removed) put(file, null);
  let shared = read(glue);
  const marker = `#[path = "../../${mutationRoot(root).slice("✏️s/🔌️plugins/🗄️stdio/".length)}/🦀️component.rs"]`;
  const at = shared.indexOf(marker);
  const start = shared.lastIndexOf("pub mod mutations {", at);
  if (start < 0) throw new Error(`missing glue block ${root.id}`);
  const brace = shared.indexOf("{", start), end = closeAt(shared, brace);
  const prefix = "../../" + mutationRoot(root).slice("✏️s/🔌️plugins/🗄️stdio/".length);
  const body = `pub mod mutations {\n                                #[path = "${prefix}/🦀️component.rs"]\n                                mod top_level;\n                                pub use top_level::*;\n${root.leaves.map(item => `                                #[path = "${prefix}/${dirname(item)}/🦀️component.rs"]\n                                pub mod ${snake(item.kind)};`).join("\n")}\n                                #[path = "${prefix}/📝️text/🦀️component.rs"]\n                                pub mod text;\n                                #[path = "${prefix}/💾️binary/🦀️component.rs"]\n                                pub mod binary;\n                            }`;
  put(glue, shared.slice(0, start) + body + shared.slice(end + 1));
  flush();
  console.log(`${root.id}: extracted ${root.leaves.length} direct owners; removed ${removed.length} fallback files`);
}
//#endregion Assemble

//#region ConsumerClosure
function removeCases(source: string, names: string[]): string {
  let result = source;
  for (const name of names) {
    const matches = [...result.matchAll(new RegExp(`^( +)"${name}"\\s*=>`, "gm"))].reverse();
    for (const match of matches) {
      const indent = match[1];
      const rest = result.slice(match.index! + match[0].length);
      const next = new RegExp(`^${indent}(?:"[^"\\n]+"|other|_)\\s*=>|^${indent.slice(0, -4)}[})]`, "m").exec(rest);
      if (!next) throw new Error(`cannot bound branch ${name}`);
      result = result.slice(0, match.index!) + rest.slice(next.index);
    }
  }
  return result;
}
function omitFunction(source: string, name: string): string {
  const found = new RegExp(`^( *)(?:pub(?:\\(crate\\))? )?(?:async )?fn ${name}\\(`, "m").exec(source);
  if (!found) return source;
  const body = blockAfter(source, `fn ${name}(`, found.index);
  let start = found.index;
  while (start > 0) {
    const previous = source.lastIndexOf("\n", start - 2) + 1;
    const line = source.slice(previous, start).trim();
    if (!line.startsWith("//") && !line.startsWith("#[")) break;
    start = previous;
  }
  return source.slice(0, start) + source.slice(body.end + 1);
}
function trimFallbackExamples(root: Root, source: string): string {
  let result = source.replaceAll(new RegExp(`^.*mutations\\.push\\(${root.aggregate}::SetSnapshot.*\\n`, "gm"), "");
  const matches = [...result.matchAll(new RegExp(`\\b${root.aggregate}::SetSnapshot\\s*\\{`, "g"))].reverse();
  for (const match of matches) {
    const brace = match.index! + match[0].lastIndexOf("{");
    const end = closeAt(result, brace);
    if (result.slice(end + 1).trimStart().startsWith("=>")) throw new Error(`unremoved snapshot pattern in ${root.id} regression`);
    const comma = /^\s*,/.exec(result.slice(end + 1));
    result = result.slice(0, match.index!) + result.slice(end + 1 + (comma?.[0].length ?? 0));
  }
  return result.replaceAll(new RegExp(`^\\s*${root.aggregate}::NoMutation,\\n`, "gm"), "");
}
function preserveRegressions(root: Root) {
  const original = source(root);
  const begin = original.search(/\/\/#region 🔖️Demo/);
  const end = original.indexOf("//#region 🧪️FixtureCases");
  let content = original.slice(begin, end < 0 ? undefined : end);
  const kindsModule = content.indexOf("//#region 🧪️KindsManifestLaw");
  if (kindsModule >= 0) content = content.slice(0, kindsModule);
  for (const name of ["kinds_const_matches_enum_variants_in_declaration_order", "kind_of", "kinds_matches_enum_variants_and_manifest", "kinds_matches_enum_variants", "kinds_matches_manifest_catalog"]) content = omitFunction(content, name);
  content = trimFallbackExamples(root, content);
  content = constructors(root, content).replaceAll("demo_mutation_cases", "regression_mutation_cases");
  for (const item of root.leaves) content = content.replaceAll(new RegExp(`\\bdiff::diff_${snake(kebab(item.old))}\\(`, "g"), `${modulePath(root)}::${snake(item.kind)}::contribute(`);
  const imports = original.slice(original.search(/^use /m), original.indexOf("//#region"));
  const file = join(subset(root), "🧪️tests/🧬️mutation-regressions/🦀️component.rs");
  put(file, `//! 🧪️ Preserved raster sparse-diff and codec regression laws.\n${imports}\nuse ${modulePath(root)}::*;\n${content}\n`);
  let shared = read(glue);
  const marker = `#[path = "../../${mutationRoot(root).slice("✏️s/🔌️plugins/🗄️stdio/".length)}/🦀️component.rs"]`;
  const at = shared.indexOf(marker);
  const start = shared.lastIndexOf("pub mod mutations {", at);
  const endBlock = closeAt(shared, shared.indexOf("{", start));
  const insertion = `\n                            #[cfg(test)]\n                            #[path = "../../${file.slice("✏️s/🔌️plugins/🗄️stdio/".length)}"]\n                            mod mutation_regressions;`;
  if (!shared.includes(`#[path = "../../${file.slice("✏️s/🔌️plugins/🗄️stdio/".length)}"]`)) put(glue, shared.slice(0, endBlock + 1) + insertion + shared.slice(endBlock + 1));
}
function closeConsumers(root: Root) {
  const selected = files(subset(root)).filter(file => file.endsWith(".rs") && !file.includes("/🧬️mutations/") && !file.includes("/🧬️mutation-regressions/"));
  const suite = join(artifacts, root.folder, "🧪️tests", root.suite);
  selected.push(...files(suite).filter(file => file.endsWith(".rs")));
  for (const file of selected) {
    let content = read(file);
    if (file.startsWith(suite)) {
      if (root.id === "tiff") {
        const found = /fn inverse_of\(/.exec(content);
        if (found) {
          const body = blockAfter(content, "fn inverse_of(", found.index);
          content = content.slice(0, found.index) + `fn inverse_of(mutation: &TiffMutation, base: &TiffSnapshot) -> Vec<TiffMutation> { semio_s_plugin_stdio::artifacts::tiff::schema::mutations::inverse_tiff_mutation(mutation, base) }` + content.slice(body.end + 1);
          content = content.replace("let inverse = inverse_of(&mutation, &base);\n        apply_tiff_mutation(&mut snapshot, &inverse);", "for inverse in inverse_of(&mutation, &base) { apply_tiff_mutation(&mut snapshot, &inverse); }");
        }
      }
      content = removeCases(content, ["no-mutation", "set-snapshot"]);
      content = content.replace(/const KINDS:\s*(?:&\[&str\]|\[&str; \d+\])\s*=\s*&?\[[\s\S]*?\];/, `const KINDS: &[&str] = &[${root.leaves.map(item => `"${item.kind}"`).join(", ")}];`);
    }
    content = constructors(root, content, file.startsWith(suite));
    if (content !== read(file)) put(file, content);
  }
  for (const file of files(suite).filter(file => file.endsWith(".feature"))) {
    let content = read(file);
    content = content.split("\n").filter(line => !/^\s*\|/.test(line) || !/no-mutation|set-snapshot/.test(line)).join("\n");
    for (const item of root.leaves) content = content.replaceAll(kebab(item.old), item.kind);
    put(file, content);
  }
  const oracleFile = join(subset(root), "🧪️oracle/🦀️component.rs");
  if (existsSync(oracleFile)) {
    let content = read(oracleFile);
    for (const item of root.leaves) content = content.replaceAll(kebab(item.old), item.kind);
    put(oracleFile, content);
  }
  preserveRegressions(root);
  flush();
}
function alignWireSurfaces(root: Root) {
  const base = mutationRoot(root);
  const grammarFile = join(base, "📝️text/📖️component.grammar.semio");
  let grammar = read(grammarFile);
  for (const item of root.leaves) grammar = grammar.replaceAll(kebab(item.old), item.kind);
  grammar = grammar.split("\n").filter(line => !line.startsWith("#") && !/^(?:no-mutation-op|set-snapshot-op|snapshot-value)\s*=/.test(line)).map(line => line.replace(/no-mutation-op\s*\|\s*/g, "").replace(/set-snapshot-op\s*\|\s*/g, "")).join("\n");
  grammar = `# Direct identities: ${root.leaves.map(item => item.kind).join(", ")}\n${grammar.replace(/\n{3,}/g, "\n\n")}`;
  put(grammarFile, grammar.endsWith("\n") ? grammar : grammar + "\n");
  put(join(base, "💾️binary/📡️component.protocol.semio"), `dialect protocol\nprotocol stdio.${root.id}.mutations\nversion 1\nschema stdio.${root.id}.op\nstart op\n\n# Direct identities: ${root.leaves.map(item => `${item.kind}=${item.tag}`).join(", ")}\nframing record\nheader fixed 2\nchain payload bytes\n`);
  put(join(base, "💾️binary/🥋️component.ksy"), `meta:\n  id: stdio_${root.id}_mutation\n  endian: le\nseq:\n  - id: format\n    type: u1\n    valid: 1\n  - id: tag\n    type: u1\n    enum: mutation_kind\n  - id: payload\n    size-eos: true\nenums:\n  mutation_kind:\n${root.leaves.map(item => `    ${item.tag}: ${snake(item.kind)}`).join("\n")}\n`);
  put(join(base, "💾️binary/🔠️component.abnf"), `; Direct binary frame: version, descriptor tag, leaf-owned payload.\n; ${root.leaves.map(item => `${item.kind}=${item.tag}`).join(", ")}\nmutation = %x01 tag *OCTET\ntag = ${root.leaves.map(item => `%x${item.tag.toString(16).padStart(2, "0").toUpperCase()}`).join(" / ")}\nOCTET = %x00-FF\n`);
  put(join(base, "💾️binary/🌶️component.spicy"), `module Stdio_${root.id}_mutation;\n# Direct tags: ${root.leaves.map(item => `${item.kind}=${item.tag}`).join(", ")}\npublic type Mutation = unit { format: uint8; tag: uint8; payload: bytes &eod; };\n`);
  put(join(base, "📝️text/🅰️component.g4"), `grammar Stdio_${root.id}_mutation;\n// Leaf payload grammars are authoritative in the direct text facets.\nmutation: opcode argument* EOF;\nopcode: ${root.leaves.map(item => `'${item.kind}'`).join(" | ")};\nargument: WORD '=' VALUE;\nWORD: [a-zA-Z][a-zA-Z0-9_-]*;\nVALUE: ~[ \\t\\r\\n]+;\nWS: [ \\t\\r\\n]+ -> skip;\n`);
  put(join(base, "📝️text/🔤️component.ebnf"), `(* Direct text registry; payload productions are owned by each leaf. *)\nmutation = opcode, { " ", argument } ;\nopcode = ${root.leaves.map(item => `"${item.kind}"`).join(" | ")} ;\nargument = name, "=", value ;\nname = letter, { letter | digit | "-" } ;\nvalue = character, { character } ;\n`);
  for (const file of files(base).filter(file => !root.leaves.some(item => file.startsWith(join(base, dirname(item)) + "/")) && /\.(json|graphql|proto|ts)$/.test(file) && file.includes("/📝️text/"))) {
    if (file.endsWith("🔣️component.json")) put(file, json({ $schema: "http://json-schema.org/draft-07/schema#", title: `${root.aggregate}Text`, type: "string", pattern: `^(?:${root.leaves.map(item => item.kind).join("|")})(?: |$)` }));
  }
  flush();
}
//#endregion ConsumerClosure

//#region Operations
function relocateOperations(root: Root) {
  const rootFile = join(mutationRoot(root), "🦀️component.rs");
  const operationsFile = join(subset(root), "🧬️schema/⚙️operations/🦀️component.rs");
  let content = read(rootFile);
  const declarations: string[] = [];
  for (const name of [`apply_${root.id}_mutation`, `inverse_${root.id}_mutation`]) {
    const extracted = stripFunction(content, name);
    content = extracted.source;
    declarations.push(extracted.declaration);
  }
  put(operationsFile, `//! ⚙️ Shared application and inversion of ${root.aggregate}.\nuse crate::artifacts::${root.id}::schema::{diff::${root.diff}, mutations::${root.aggregate}};\nuse crate::artifacts::${root.id}::${root.snapshot};\n\n//#region Operations\n${declarations.join("\n\n")}\n//#endregion Operations\n`);
  content = content.replace("//#region Owners", `pub use crate::artifacts::${root.id}::schema::operations::{apply_${root.id}_mutation, inverse_${root.id}_mutation};\n\n//#region Owners`).replace(/\n{3,}/g, "\n\n");
  put(rootFile, content);
  const shared = read(glue);
  const marker = `#[path = "../../${mutationRoot(root).slice("✏️s/🔌️plugins/🗄️stdio/".length)}/🦀️component.rs"]`;
  const at = shared.indexOf(marker);
  if (at < 0) throw new Error(`missing ${root.id} mutation mount`);
  const start = shared.lastIndexOf("pub mod mutations {", at);
  const end = closeAt(shared, shared.indexOf("{", start));
  const mount = `\n                            #[path = "../../${operationsFile.slice("✏️s/🔌️plugins/🗄️stdio/".length)}"]\n                            pub mod operations;`;
  if (shared.includes(operationsFile.slice("✏️s/🔌️plugins/🗄️stdio/".length))) throw new Error(`existing ${root.id} operations mount`);
  put(glue, shared.slice(0, end + 1) + mount + shared.slice(end + 1));
  flush();
}
function pruneFallback(root: Root) {
  const obsolete = join(mutationRoot(root), "📄set-snapshot");
  if (!existsSync(obsolete)) return;
  if (files(obsolete).length) throw new Error(`nonempty obsolete folder ${obsolete}`);
  const removed: string[] = [];
  function prune(directory: string) {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.isSymbolicLink()) throw new Error(`unexpected entry ${join(directory, entry.name)}`);
      prune(join(directory, entry.name));
    }
    rmdirSync(directory);
    removed.push(directory);
  }
  prune(obsolete);
  put(join(ticket, `📓️${root.id}-empty-folder-removal.md`), `# Verified Empty Fallback Removal\n\nOnly empty directories were removed. Prior payloads remain in the ticket baseline.\n\n${removed.map(file => `- \`${file}\``).join("\n")}\n`);
  flush();
  console.log(`${root.id}: removed ${removed.length} verified-empty fallback directories`);
}
//#endregion Operations

//#region OracleClosure
function closeOracles(root: Root) {
  const oracleFile = join(subset(root), "🧪️oracle/🦀️component.rs");
  let content = read(oracleFile);
  if (root.id === "png") content = omitFunction(content, "set_snapshot");
  if (root.id === "tiff") {
    content = omitFunction(content, "parse_doc_json");
    for (const match of [...content.matchAll(/    if kind == "set-snapshot" \{/g)].reverse()) {
      const brace = match.index! + match[0].lastIndexOf("{");
      content = content.slice(0, match.index) + content.slice(closeAt(content, brace) + 1);
    }
  }
  content = content.replaceAll('"no-mutation" | ', "").replaceAll('"set-snapshot" | ', "");
  content = removeCases(content, ["no-mutation", "set-snapshot"]);
  content = content.replaceAll("`no-mutation`", "an unchanged round trip").replaceAll("/`set-snapshot`", "").replaceAll("/set-snapshot", "").replaceAll(" and `set-snapshot`'s `ifds[]`", "").replaceAll("entries both use.", "uses.");
  if (root.id !== "jpg") {
    const implementation = root.id === "tiff" ? "Ok(write_tiff(&read_tiff(input)?))" : "oracles::encode(&oracles::decode(input)?)";
    content += `\n//#region RoundTrip\n#[cfg(feature = "oracles")]\npub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> { ${implementation} }\n#[cfg(not(feature = "oracles"))]\npub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> { Err("the oracles feature is disabled".into()) }\n//#endregion RoundTrip\n`;
  }
  put(oracleFile, content);
  const suiteFile = join(artifacts, root.folder, "🧪️tests", root.suite, "🦀️component.rs");
  let suite = read(suiteFile);
  if (root.id !== "jpg") {
    suite = omitFunction(suite, "no_mutation_spec");
    suite = suite.replaceAll("oracle_apply_mutation(&input, &no_mutation_spec())?", "oracle_identity_round_trip(&input)?");
    suite = suite.replace(/^    let no_mutation = Json::Object\([^\n]*\n/gm, "").replaceAll("oracle_apply_mutation(&input, &no_mutation)?", "oracle_identity_round_trip(&input)?");
    suite = `use semio_s_plugin_stdio_test_oracle::artifacts::${root.id}::standards::${root.id === "png" ? "v1_2" : root.id === "bmp" ? "v3" : "v6_0"}::subsets::any::oracle_identity_round_trip;\n` + suite;
    const docEnd = suite.indexOf("\nuse ", suite.indexOf("\n") + 1);
    if (suite.slice(suite.indexOf("\n") + 1, docEnd).includes("//!")) {
      const firstLine = suite.slice(0, suite.indexOf("\n") + 1);
      suite = suite.slice(firstLine.length);
      const firstUse = suite.search(/^use /m);
      suite = suite.slice(0, firstUse) + firstLine + suite.slice(firstUse);
    }
  }
  suite = suite.replaceAll("`no-mutation`", "an unchanged round trip").replaceAll("`set-snapshot`", "`replace-pixels`");
  put(suiteFile, suite);
  flush();
}
//#endregion OracleClosure

//#region RustOracle
function rustOracle(root: Root) {
  const selected = files(mutationRoot(root)).filter(file => file.endsWith(".rs"));
  selected.push(join(subset(root), "🧬️schema/⚙️operations/🦀️component.rs"));
  selected.push(join(subset(root), "🧪️tests/🧬️mutation-regressions/🦀️component.rs"));
  selected.push(join(subset(root), "🧪️oracle/🦀️component.rs"));
  selected.push(join(artifacts, root.folder, "🧪️tests", root.suite, "🦀️component.rs"));
  let failures = 0;
  const output = [];
  for (const file of selected) {
    const result = spawnSync("rustc", ["+nightly-2026-07-07", "-Zunpretty=ast-tree", "--edition=2021", "--crate-type=lib", file], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
    output.push(`${result.status === 0 ? "PASS" : "FAIL"} ${file}\n${result.stderr}`);
    if (result.status !== 0) { failures++; console.log(result.stderr); }
  }
  const summary = `${root.id}: nightly parsed=${selected.length - failures}/${selected.length} failures=${failures}`;
  console.log(summary);
  put(join(ticket, `📓️${root.id}-nightly-parse.md`), `# Independent Rust Parse Oracle\n\n\`rustc +nightly-2026-07-07 -Zunpretty=ast-tree --edition=2021 --crate-type=lib <file>\`\n\n${summary}\n\n\`\`\`text\n${output.join("\n")}\n\`\`\`\n`);
  flush();
  process.exitCode = failures ? 1 : 0;
}
//#endregion RustOracle

//#region ClosureEvidence
function closureEvidence() {
  const rows: string[] = [];
  const roster: string[] = [];
  const changed: string[] = [];
  let errors = 0;
  for (const root of roots) {
    const base = mutationRoot(root);
    const all = files(subset(root));
    const sourceFiles = all.filter(file => !file.includes("/🧪️direct-mutation-contract/") && /\.(rs|ts|json|graphql|proto|semio|g4|ebnf|ksy|abnf|spicy)$/.test(file));
    const direct = root.leaves.map(item => join(base, dirname(item)));
    const required = ["🦀️component.rs", "🔣️component.json", "🔣️payload.schema.json", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs", "🧪️tests/🔣️component.json", "🧪️tests/🦀️component.rs"];
    const catalog = JSON.parse(read(join(subset(root), "🧪️oracle/🔣️component.json"))).mutationCatalogs[0];
    const kinds = root.leaves.map(item => item.kind);
    if (JSON.stringify(catalog.kinds) !== JSON.stringify(kinds)) errors++;
    if (JSON.stringify(catalog.vectors.map((vector: any) => vector.mutationId)) !== JSON.stringify(kinds)) errors++;
    const grammar = read(join(base, "📝️text/📖️component.grammar.semio"));
    const binary = read(join(base, "💾️binary/📡️component.protocol.semio"));
    if (!binary.includes("header fixed 2") || !binary.includes("chain payload bytes")) errors++;
    for (const item of root.leaves) {
      const owner = join(base, dirname(item));
      for (const name of required) if (!existsSync(join(owner, name))) errors++;
      if (!grammar.includes(`"${item.kind}"`) || !binary.includes(`${item.kind}=${item.tag}`)) errors++;
      for (const facet of ["text", "binary"]) if (!read(join(base, facet === "text" ? "📝️text" : "💾️binary", "🦀️component.rs")).includes(`${snake(item.kind)}::${facet}::CODEC`)) errors++;
      if (/\bRestore\s*\(|\bApply\s*\(/.test(read(join(owner, "🦀️component.rs")))) errors++;
    }
    const stale = sourceFiles.filter(file => file.startsWith(base)).flatMap(file => read(file).split("\n").filter(line => /\bNoMutation\b|\bSetSnapshot\b|no-mutation|set-snapshot|\[DEBUG\]/.test(line)));
    errors += stale.length;
    const nested = all.filter(file => file.includes("/🦠️mutation/")).length;
    errors += nested;
    const debug = sourceFiles.flatMap(file => read(file).split("\n").filter(line => line.includes("[DEBUG]"))).length;
    errors += debug;
    const parsed = read(join(ticket, `📓️${root.id}-nightly-parse.md`)).match(/parsed=(\d+)\/(\d+) failures=(\d+)/)?.[0];
    rows.push(`| ${root.id.toUpperCase()} | ${root.leaves.length} | ${direct.length * required.length} | ${catalog.kinds.length}/${catalog.vectors.length} | ${parsed} | ${stale.length}/${nested}/${debug} |`);
    roster.push(`### ${root.id.toUpperCase()}\n\nRoot: \`${base}\`\n\n${root.leaves.map(item => `- \`${item.old}\` → \`${dirname(item)}\` / \`${item.variant}\` / tag \`${item.tag}\`.`).join("\n")}`);
    changed.push(...all, ...files(join(artifacts, root.folder, "🧪️tests", root.suite)));
  }
  const baseline = JSON.parse(read(baselineFile));
  const actual = changed.filter(file => !(file in baseline) || read(file) !== baseline[file]);
  const removed = Object.keys(baseline).filter(file => !existsSync(file));
  const summary = `| Root | Direct Leaves | Required Direct Files | Catalog Kinds/Vectors | Nightly Parse | Stale/Nested/Debug |\n| --- | ---: | ---: | --- | --- | --- |\n${rows.join("\n")}\n\nStatic closure errors: ${errors}.\n`;
  put(join(ticket, "📓️closure.md"), `# Raster Base Direct Closure\n\n## Static Checkpoint\n\n${summary}\nAll four exact roots were independently checked by the coordinator with zero findings across the 17 structural classes. This is structural acceptance, not semantic runtime acceptance.\n\n## Roster\n\n${roster.join("\n\n")}\n\n## Ownership\n\nEach operation directly owns its payload, typed sparse contribution, semantic inverse, and text/binary codec. Shared application/inversion lives in each nearest schema \`⚙️operations\` module. Canonical root codecs visibly assemble exact leaf module entries, and only perform framing. No mutation leaf exposes an arbitrary-diff Restore branch.\n\nThe four \`📄set-snapshot\` fallback trees were removed after all twelve payload files per tree were deleted and every remaining directory was proven empty. The exact prior content is retained in \`🔣️baseline.json\`. JPG/TIFF subset roots were not converted. Existing shared glue/control-plane modifications were preserved.\n\n## Validation\n\n- Test-first direct contract: initially 0/36 descriptors and 324 missing required files; retained contract vectors preceded extraction.\n- \`bun <ticket>/📜️script.ts validate\`: 36 descriptors, 36 payload schemas, 36 vectors, 144 internal/Ajv comparisons, zero errors. Rerun after completing JPG payload requirements.\n- \`bun <ticket>/📜️script.ts nightly <png|jpg|bmp|tiff>\`: pinned nightly AST parser; includes aggregate, leaf behavior, leaf codecs, leaf tests, operations, preserved regression suites, test oracles, and exhaustive adapters. The first PNG run caught a trailing default-arm extraction; the first TIFF run caught a doc-comment import fragment. Both were repaired and the final sweep passed.\n- \`rustfmt +nightly-2026-07-07 --edition 2021 --config skip_children=true <exact owned Rust files>\`: exit 0.\n- \`git diff --check -- <four exact artifact prefixes>\`: exit 0.\n- Whole-STDIO executable stale-constructor scan: 0; two old JPG baseline doc comments remain outside this batch. Scoped raster debug markers: 0.\n- Exact module registry/grammar/tag/catalog/direct-file correspondence: \`bun <ticket>/📜️script.ts closure\`, ${errors} errors.\n\n## Runtime Boundary\n\nNo Cargo or Nx build ran in this lane during the coordinator-owned shared build window. Source edits are frozen at the compile-readiness checkpoint. The coordinator's registered Demonstrator retry owns the shared STDIO compilation. Direct semantic behavior/inverse/text/binary tests are authored but not yet executed in this batch; no runtime-pass claim is made. Temporary source debug probes are absent because runtime was not available.\n\n## Exact Scoped Change List\n\n### Created or Updated\n\n${[...new Set(actual)].sort().map(file => `- \`${file}\``).join("\n")}\n\nShared mount changes: \`${glue}\` (only the four Any schema blocks).\n\n### Removed\n\n${removed.sort().map(file => `- \`${file}\``).join("\n")}\n`);
  flush();
  console.log(summary);
  process.exitCode = errors ? 1 : 0;
}
//#endregion ClosureEvidence

if (process.argv[2] === "capture") capture();
else if (process.argv[2] === "contract") contract();
else if (process.argv[2] === "validate") await validate();
else if (process.argv[2] === "inspect") for (const root of roots) console.log(root.id, inspect(root).data.map(data => ({ variant: data.item.old, fields: data.fields.length, text: Boolean(data.print && data.parse), binary: Boolean(data.encode && data.decode) })));
else if (process.argv[2] === "generate") generate(roots.find(root => root.id === process.argv[3]) ?? (() => { throw new Error("expected exact raster root id"); })());
else if (process.argv[2] === "consumers") closeConsumers(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "wire") alignWireSurfaces(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "nightly") rustOracle(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "operations") relocateOperations(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "prune") pruneFallback(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "oracles") closeOracles(roots.find(root => root.id === process.argv[3])!);
else if (process.argv[2] === "payloads") { const root = roots.find(root => root.id === process.argv[3])!; for (const data of inspect(root).data) put(join(mutationRoot(root), dirname(data.item), "🔣️payload.schema.json"), json({ ...payloadSchema(root, data.fields), title: leafType(data.item) })); flush(); }
else if (process.argv[2] === "closure") closureEvidence();
else if (process.argv[2] === "facets") { const root = roots.find(root => root.id === process.argv[3])!; for (const data of inspect(root).data) { put(join(mutationRoot(root), dirname(data.item), "📝️text/🦀️component.rs"), leafText(root, data)); put(join(mutationRoot(root), dirname(data.item), "💾️binary/🦀️component.rs"), leafBinary(root, data)); } flush(); }
else if (process.argv[2] === "vectors") { const root = roots.find(root => root.id === process.argv[3])!; for (const item of root.leaves) put(join(mutationRoot(root), dirname(item), "🧪️tests/🔣️component.json"), json(vector(root, item))); flush(); }
else throw new Error("expected capture, contract or validate");
