import Parser from "web-tree-sitter";
import { dirname, join, relative, resolve } from "node:path";
import { existsSync, mkdirSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { execFileSync } from "node:child_process";
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
const files = execFileSync("rg", ["--files", "-g", "*.rs", "✏️s/🔌️plugins"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 }).trim().split("\n");
const changed = new Set<string>();
const movedSymbols = new Map<string, Set<string>>();
const expand = (text: string): string[] => {
  const brace = text.indexOf("{");
  if (brace < 0) return [text.trim()];
  const prefix = text.slice(0, brace), middle = text.slice(brace + 1, text.lastIndexOf("}"));
  let depth = 0, start = 0; const parts: string[] = [];
  for (let i = 0; i <= middle.length; i++) {
    if (middle[i] === "{") depth++;
    if (middle[i] === "}") depth--;
    if (i === middle.length || middle[i] === "," && depth === 0) { const part = middle.slice(start, i).trim(); if (part) parts.push(...expand(prefix + part)); start = i + 1; }
  }
  return parts;
};
const imports = (source: string): string[] => [...source.matchAll(/^use\s+([^;]+);/gmu)].flatMap(match => expand(match[1]!));
const binding = (path: string): string => path.includes(" as ") ? path.split(" as ").at(-1)! : path.split("::").at(-1)!;
const write = (file: string, source: string): void => { mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, source); changed.add(file); };
for (const file of files.filter(path => path.endsWith("/🚪️io/📝️text/🔺️diff/🦀️.rs"))) {
  const source = readFileSync(file, "utf8");
  if (!/impl MutationDiff|apply_to_artifact/u.test(source)) continue;
  const target = file.replace("/🚪️io/📝️text/", "/🧬️schema/");
  const schema = readFileSync(target, "utf8");
  const tree = parser.parse(source)!;
  if (tree.rootNode.hasError()) throw Error(`Rust syntax invalid before extraction: ${file}`);
  const moved: string[] = [], retained: string[] = [], tests: { path: string; text: string }[] = [];
  const symbols = new Set<string>();
  let prefix = "";
  for (const node of tree.rootNode.namedChildren) {
    if (node.type === "line_comment") { if (node.text.startsWith("///")) prefix += node.text + "\n"; continue; }
    if (node.type === "attribute_item") { prefix += node.text + "\n"; continue; }
    const chunk = prefix + node.text; prefix = "";
    if (node.type === "use_declaration") { if (node.text.startsWith("pub use diff_codec")) retained.push(chunk); continue; }
    const name = node.childForFieldName("name")?.text ?? "";
    const isCodec = node.type === "const_item" && name.startsWith("COMPONENT_GRAMMAR") || node.type === "type_item" && /(?:Text|Binary)$/u.test(name) || node.type === "mod_item" && (name === "diff_codec" || name.includes("grammar")) || node.type === "function_item" && /^(?:encode|decode|parse|print)_/u.test(name) || node.type === "impl_item" && /(?:DiffText|DiffBinary|ArtifactDsl|ArtifactPack)/u.test(node.childForFieldName("trait")?.text ?? "");
    if (isCodec) { retained.push(chunk); continue; }
    if (node.type === "mod_item" && /#\[cfg\(test\)\]/u.test(chunk)) {
      const path = chunk.match(/#\[path = "([^"]+)"\]/u)?.[1];
      if (!path) throw Error(`Inline test requires manual handling: ${file}`);
      tests.push({path, text: chunk}); moved.push(chunk); continue;
    }
    moved.push(chunk);
    if (name && /^(?:function_item|struct_item|enum_item|trait_item|type_item)$/u.test(node.type)) symbols.add(name);
  }
  const presentImports = imports(schema), presentNames = new Set(presentImports.map(binding));
  const presentDeclarations = new Set([...schema.matchAll(/(?:pub(?:\(crate\))?\s+)?(?:struct|enum|type|trait|fn)\s+(\w+)/gu)].map(match => match[1]!));
  const addedImports: string[] = [];
  for (const path of imports(source)) {
    const name = binding(path);
    if (/^(?:crate::schema::diff|crate::standards::v1::subsets::any::schema::diff|crate::diff::schema)::/u.test(path)) {
      if (path.includes(" as ")) addedImports.push(`use self::${path.split("::").at(-1)};`);
      continue;
    }
    if (presentImports.includes(path) || name !== "*" && (presentNames.has(name) || presentDeclarations.has(name))) continue;
    addedImports.push(`use ${path};`); presentImports.push(path); presentNames.add(name);
  }
  const io = "//! 📝️ Physical text diff representation.\n\n" + retained.join("\n\n") + "\n";
  const semantic = schema.trimEnd() + "\n\n" + addedImports.join("\n") + "\n\n" + moved.join("\n\n") + "\n";
  for (const [path, body] of [[file, io], [target, semantic]]) {
    const parsed = parser.parse(body)!;
    if (parsed.rootNode.hasError()) throw Error(`Rust syntax invalid after extraction: ${path}`);
    parsed.delete();
  }
  for (const test of tests) {
    const oldRoot = dirname(join(dirname(file), test.path)), newRoot = dirname(join(dirname(target), test.path));
    const testFiles = files.filter(path => path.startsWith(oldRoot + "/"));
    for (const oldFile of testFiles) {
      const newFile = join(newRoot, relative(oldRoot, oldFile));
      if (existsSync(newFile)) throw Error(`Test target already exists: ${newFile}`);
      let body = readFileSync(oldFile, "utf8");
      body = body.replace(/(?:include_str!|include_bytes!|#\[path\s*=)\s*\(?"([^"]+)"/gu, (match, literal) => {
        if (!literal.startsWith(".")) return match;
        const resolved = resolve(dirname(oldFile), literal), rewritten = relative(dirname(newFile), resolved);
        return match.replace(literal, rewritten.startsWith(".") ? rewritten : "./" + rewritten);
      });
      write(newFile, body); unlinkSync(oldFile); changed.add(oldFile);
    }
  }
  write(file, io); write(target, semantic);
  movedSymbols.set(file, symbols);
  tree.delete();
}
for (const file of files.filter(path => path.endsWith(".rs") && existsSync(path))) {
  let source = readFileSync(file, "utf8"), next = source;
  const pluginRoot = file.split("/🗿️artifacts/")[0]!;
  const matching = [...movedSymbols].filter(([path]) => path.startsWith(pluginRoot + "/"));
  if (!matching.length) continue;
  const symbols = new Set(matching.flatMap(([, names]) => [...names]));
  next = next.replace(/(?:crate::)?(?:standards::v1::subsets::any::)?io::text::diff::(\w+)/gu, (path, symbol) => symbols.has(symbol) ? path.replace("io::text::diff::", "schema::diff::") : path);
  next = next.replace(/use ((?:crate::)?(?:standards::v1::subsets::any::)?io::text::diff)::\*;/gu, (path) => {
    if (file.includes("/🔺️diff/🧪️tests/") && !file.includes("grammar")) return path.replace("io::text::diff", "schema::diff") + "\nuse protocol::MutationDiff;\nuse crate::standards::v1::subsets::any::schema::*;\nuse crate::*;";
    return path;
  });
  next = next.replace(/use ((?:crate::)?(?:standards::v1::subsets::any::)?io::text::diff)::\{([^}]+)\};/gu, (whole, prefix, list) => {
    const moved = list.split(",").map((name:string) => name.trim()).filter((name:string) => symbols.has(name.split(" as ")[0]!));
    if (!moved.length) return whole;
    const remaining = list.split(",").map((name:string) => name.trim()).filter((name:string) => !symbols.has(name.split(" as ")[0]!));
    return `use ${prefix.replace("io::text::diff", "schema::diff")}::{${moved.join(", ")}};` + (remaining.length ? `\nuse ${prefix}::{${remaining.join(", ")}};` : "");
  });
  if (next !== source) write(file, next);
}
parser.delete();
const report = ["# Semantic Diff Ownership Extraction", "", `Extracted ${movedSymbols.size} mounted text diff owners into their canonical schema siblings. Rust Tree-sitter parses every changed owner before and after extraction. Semantic tests moved with their implementations; physical grammar tests remain in IO. Existing semantic-only Mutation trait has no codec supertraits.`, "", "## Changed Files", "", ...[...changed].sort().map(path => `- ${path}`), ""].join("\n");
writeFileSync(join(import.meta.dir, "semantic-diff-extraction.md"), report);
console.log(`[DEBUG] semantic-diff-extraction owners=${movedSymbols.size} files=${changed.size} syntax=tree-sitter`);
