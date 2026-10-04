import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
import Parser from "web-tree-sitter";
import ts from "typescript";
import { applyPatch } from "diff";
import TOML from "@iarna/toml";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), output = resolve(ticket, "🗑️generated/graph-os-record-cut"), epoch = process.argv[2] ?? "8";
const observation = process.argv[3] ?? epoch;
const hash = (body: string | null) => body === null ? null : createHash("sha256").update(body).digest("hex");
const directory = resolve(output, `publication-21-production-${epoch}`), ready = JSON.parse(readFileSync(resolve(directory, "ready.json"), "utf8")), journalSource = readFileSync(resolve(directory, "publication.json"), "utf8"), journal = JSON.parse(journalSource);
assert.equal(journal.rows.length, 4516);
assert.deepEqual(journal.rows.map(row => row.path), ready.rows.map(row => row.path));
const frames = journal.rows.map(row => {
  const source = readFileSync(row.frame.path, "utf8"), frame = JSON.parse(source);
  const checks = { frameHash: hash(source) === row.frame.sha256, beforeHash: hash(frame.before) === frame.beforeHash, authoredHash: hash(frame.authored) === frame.authoredHash, inverse: frame.inverse === frame.before, forward: applyPatch(frame.before ?? "", frame.forward) === (frame.authored ?? ""), reverse: applyPatch(frame.authored ?? "", frame.inverseEdit) === (frame.before ?? ""), immediate: row.immediateExact && row.publishedHash === row.observedHash, identity: row.path === frame.path };
  assert.ok(Object.values(checks).every(Boolean), row.path);
  return { journal: row, frame, checks };
});
const supplementalPath = process.argv[4];
let supplemental: any = null;
if (supplementalPath) {
 const body = readFileSync(supplementalPath,"utf8"); supplemental = JSON.parse(body); assert.equal(supplemental.rows.length,7);
 for(const row of supplemental.rows) { const source=readFileSync(row.frame.path,"utf8"),frame=JSON.parse(source); const checks={frameHash:hash(source)===row.frame.sha256,beforeHash:hash(frame.before)===frame.beforeHash,authoredHash:hash(frame.authored)===frame.authoredHash,inverse:frame.inverse===frame.before,forward:applyPatch(frame.before,frame.forward)===frame.authored,reverse:applyPatch(frame.authored,frame.inverseEdit)===frame.before,immediate:row.immediateExact&&row.authoredHash===frame.authoredHash,identity:row.path===frame.path}; assert.ok(Object.values(checks).every(Boolean)); frames.push({journal:row,frame,checks}); }
}
const lastByPath = new Map(frames.map(row => [row.frame.path, row]));
const currentRows = [...lastByPath.values()].map(row => {
  const current = existsSync(resolve(root, row.frame.path)) ? readFileSync(resolve(root, row.frame.path), "utf8") : null;
  return { ...row, current, currentHash: hash(current), currentExact: current === row.frame.authored };
});
const retirements = currentRows.filter(row => row.frame.authored === null).map(row => ({ path: row.frame.path, currentAbsent: row.current === null, finalFrameBeforeHash: row.frame.beforeHash }));
assert.ok(retirements.every(row => row.currentAbsent), "Exact owned retirement path recreated");
writeFileSync(resolve(output, `independent-complete-publication-current-frames-${observation}.json`), JSON.stringify({ at: new Date().toISOString(), journalSource, rows: frames.length, uniquePaths: currentRows.length, frames, currentRows, sourceWrites: 0, nativeExecuted: 0 }, null, 2));

const cargoChecks = currentRows.filter(row => row.frame.path.endsWith("Cargo.toml") && row.current !== null).map(row => {
 const bun = Bun.TOML.parse(row.current!), independent = TOML.parse(row.current!);
 const normalize = (value: any): any => Array.isArray(value) ? value.map(normalize) : value && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map(key => [key, normalize(value[key])])) : value;
 assert.deepEqual(normalize(bun), normalize(independent), row.frame.path);
 const providers: any[] = [];
 const inspect = (table: any, role: string) => { for (const [key,value] of Object.entries(table ?? {}) as any) if (value && typeof value === "object" && value.path) { const manifest = resolve(root, dirname(row.frame.path), value.path, "Cargo.toml"); assert.ok(existsSync(manifest), manifest); const body = readFileSync(manifest,"utf8"), parsed: any = TOML.parse(body); assert.equal(parsed.package.name, value.package ?? key, manifest); providers.push({ key, role, manifest, body, sha256: hash(body), identity: parsed.package.name }); } };
 inspect(bun.dependencies,"normal"); inspect(bun["dev-dependencies"],"dev"); inspect(bun["build-dependencies"],"build"); for(const [target, tables] of Object.entries(bun.target ?? {}) as any) { inspect(tables.dependencies,`target:${target}:normal`); inspect(tables["dev-dependencies"],`target:${target}:dev`); inspect(tables["build-dependencies"],`target:${target}:build`); }
 return { path: row.frame.path, sha256: row.currentHash, completeParserAgreement: true, authoredExact: row.currentExact, providers };
});
writeFileSync(resolve(output, `independent-complete-current-cargo-${observation}.json`),JSON.stringify({ at:new Date().toISOString(), cargoChecks, sourceWrites:0, nativeExecuted:0 },null,2));

const historical = [1,2,3,4,6,8,11].map(prior => {
 const path=resolve(output,`publication-21-production-${prior}/publication.json.jsonl`), body=readFileSync(path,"utf8"), records=body.trim().split("\n").map(line=>JSON.parse(line));
 const observations=records.map(row=> { const source=readFileSync(row.frame.path,"utf8"), frame=JSON.parse(source); assert.equal(hash(source),row.frame.sha256); assert.equal(hash(frame.before),frame.beforeHash); assert.equal(hash(frame.authored),frame.authoredHash); assert.equal(frame.inverse,frame.before); assert.equal(applyPatch(frame.before??"",frame.forward),frame.authored??""); assert.equal(applyPatch(frame.authored??"",frame.inverseEdit),frame.before??""); assert.ok(row.immediateExact && row.publishedHash===row.observedHash); return {row,frame}; });
 return {prior,path,sha256:hash(body),rows:records.length,uniquePaths:new Set(records.map(row=>row.path)).size,writes:records.filter(row=>row.changed).length,observations};
});
writeFileSync(resolve(output,`independent-all-partial-journal-history-${observation}.json`),JSON.stringify({at:new Date().toISOString(),historical,sourceWrites:0,nativeExecuted:0},null,2));

const helperPath = resolve(ticket, "graph-os-record-cut-proposal/📜️script.ts"), helper = readFileSync(helperPath, "utf8"), prefix = helper.slice(helper.indexOf("function flattenRustUse("), helper.indexOf("const walk ="));
const transpiled = ts.transpileModule(prefix + "\nreturn {flattenRustUse,rustLexicalAliases,resolveRustAlias,canonicalTokenPathEdits,lexicalScopeCache};", { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
const lexical = new Function("assert", transpiled)(assert);
const census = JSON.parse(readFileSync(resolve(output, "parsed-owned-caller-census-4.json"), "utf8")), raw = JSON.parse(readFileSync(resolve(output, "full-caller-manifest-census-2.json"), "utf8")), observations = new Map(census.observations.map(row => [row.path, row])), manifests = new Map(raw.manifests.map(row => [row.path, row]));
const names = new Set(Object.values(census.taxonomy).flat() as string[]), oldRoots = new Set(["semio_framework_os_kernel", "semio_framework_os_dsl_derive", "dsl", "os_dsl", "dsl_core", "dsl_derive"]), os = "🧰️framework/🛍️products/💻️os";
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
const roots = new Map(), residuals = [], qualified = [], tombstones = [], grammars = [];
const retired = (parts: string[], ownOS: boolean) => {
  let offset = parts[0] === "crate" && parts[1] === "os_dsl" ? 2 : oldRoots.has(parts[0]) ? 1 : parts[0] === "crate" && ownOS ? 1 : -1;
  if (offset < 0) return false;
  if (parts[offset] === "os_dsl") offset++;
  if (parts[offset] === "schema") return !(parts[0] === "crate" && parts[1] === "schema");
  if (parts[offset] === "notation") return ["EdgeLabel", "EdgeLink", "EdgeNode", "EdgeValue", "print_edge", "parse_edge_text"].includes(parts[offset + 1]);
  return names.has(parts[offset]);
};
for (const row of currentRows.filter(row => row.frame.path.endsWith(".rs"))) {
  const path = row.frame.path, source = row.current;
  if (source === null) { tombstones.push({ path, authoredNull: row.frame.authored === null }); continue; }
  const observed: any = observations.get(path), info: any = manifests.get(observed?.manifest);
  if (info && !roots.has(observed.manifest)) {
    const entry = info.library && existsSync(resolve(root, info.library)) ? readFileSync(resolve(root, info.library), "utf8") : "";
    const tree = parser.parse(entry)!; lexical.lexicalScopeCache.clear();
    const aliases = new Map(lexical.rustLexicalAliases(tree.rootNode));
    if (info.package) aliases.set("__package_identity__", [info.package.replaceAll("-", "_")]);
    for (const [name, binding] of aliases as Map<string, string[]>) if (binding?.[0] === "self" && info.package) aliases.set(name, [info.package.replaceAll("-", "_"), ...binding.slice(1)]);
    roots.set(observed.manifest, aliases); tree.delete();
  }
  const defaults = roots.get(observed?.manifest) ?? new Map(), tree = parser.parse(source)!;
  lexical.lexicalScopeCache.clear();
  const classify = (parts: string[], node: any, kind: string) => {
    let resolved = lexical.resolveRustAlias(parts, node, defaults);
    if (resolved[0] === "crate" && resolved[1] !== "os_dsl" && Array.isArray(defaults.get(resolved[1]))) resolved = [...defaults.get(resolved[1]), ...resolved.slice(2)];
    if (retired(resolved, path.startsWith(os + "/"))) residuals.push({ path, start: node.startIndex, reference: parts.join("::"), resolved, kind });
    else if (oldRoots.has(parts[0]) || parts.slice(0, 2).join("::") === "crate::os_dsl") qualified.push({ path, start: node.startIndex, reference: parts.join("::"), resolved, kind });
  };
  const visit = (node: any) => {
    if (["line_comment", "block_comment", "string_literal", "raw_string_literal", "char_literal"].includes(node.type)) return;
    if (node.type === "use_declaration") {
      const leaves = lexical.flattenRustUse(node.childForFieldName("argument"));
      for (const leaf of leaves) classify(leaf.path, node, "import");
      if (path === os + "/🔨️modules/🗣️dsl/🦀️.rs" && node.namedChildren.some(child => child.type === "visibility_modifier")) for (const leaf of leaves) if (["semio_framework_dsl_record", "semio_framework_dsl_record_derive", "semio_framework_value", "semio_framework_diagnostic"].includes(leaf.path[0])) residuals.push({ path, start: node.startIndex, reference: leaf.path.join("::"), kind: "retired-public-generic-os-facade" });
      return;
    }
    if (path === os + "/🔨️modules/🗣️dsl/🦀️.rs" && ["trait_item", "struct_item", "enum_item", "mod_item"].includes(node.type) && ["DslField", "DslVariants", "schema", "native_encoding", "variants_text"].includes(node.childForFieldName("name")?.text)) residuals.push({ path, start: node.startIndex, reference: node.childForFieldName("name")!.text, kind: "retired-generic-os-definition-or-mount" });
    if (["scoped_identifier", "scoped_type_identifier"].includes(node.type)) { classify(node.text.replace(/^::/, "").split("::"), node, "ordinary-path"); return; }
    for (const child of node.namedChildren) visit(child);
  };
  visit(tree.rootNode);
  lexical.canonicalTokenPathEdits(tree.rootNode, (parts, node) => { classify(parts, node, "macro-token-path"); return null; });
  const expression = /\b(?:dsl|os_dsl|dsl_core|dsl_derive|semio_framework_os_kernel|semio_framework_os_dsl_derive)::[A-Za-z_][A-Za-z_0-9]*/g;
  for (const match of source.matchAll(expression)) {
    const node = tree.rootNode.descendantForIndex(match.index!, match.index! + match[0].length);
    let ancestor = node, excluded = false, bound = false;
    while (ancestor) { if (["line_comment", "block_comment", "string_literal", "raw_string_literal", "char_literal"].includes(ancestor.type)) excluded = true; if (ancestor.type === "attribute_item" && /\bbound\s*=/.test(ancestor.text)) bound = true; ancestor = ancestor.parent; }
    if (excluded && bound) classify(match[0].split("::"), node, "semantic-derive-bound");
    else if (excluded) qualified.push({ path, start: match.index, reference: match[0], kind: "comment-or-literal", nodeType: node.type });
  }
  if (tree.rootNode.hasError()) { const errors: any[] = []; const collect = (node: any) => { if (node.type === "ERROR" || node.isMissing()) errors.push({ type: node.type, start: node.startIndex, end: node.endIndex, text: node.text, isMissing: node.isMissing() }); for(const child of node.children) collect(child); }; collect(tree.rootNode); grammars.push({ path, rootHasError: true, errors, scope: "Current third-party grammar only; unrelated grammar errors are not compiler claims" }); }
  tree.delete();
}
parser.delete();
const unique = [...new Map(residuals.map(row => [JSON.stringify(row), row])).values()];
const result = { at: new Date().toISOString(), journal: { path: resolve(directory, "publication.json"), sha256: hash(journalSource), rows: 4516, uniquePaths: currentRows.length, exactFrames: frames.length, supplementalRows: supplemental?.rows.length ?? 0, changed: journal.rows.filter(row => row.changed).length }, retirements, currentAdvances: currentRows.filter(row => !row.currentExact).map(row => ({ path: row.frame.path, authoredHash: row.frame.authoredHash, currentHash: row.currentHash })), residuals: unique, qualified, tombstones, grammars, taxonomy: census.taxonomy, helper: { path: helperPath, sha256: hash(helper) }, sourceWrites: 0, nativeExecuted: 0, scope: "Complete durable publication and observation-time current executable namespaces, including lexical token trees/repetitions; last owned authored state folded by actual path, with complete per-frame history; foreign advancement does not require a forest freeze" };
writeFileSync(resolve(output, `independent-complete-publication-current-residual-${observation}.json`), JSON.stringify(result, null, 2));
console.log(JSON.stringify({ journalRows: 4516, supplementalRows: supplemental?.rows.length ?? 0, uniquePaths: currentRows.length, frameChecks: frames.length, currentAdvances: result.currentAdvances.length, residuals: unique.length, qualified: qualified.length, tombstones: tombstones.length, grammarFrames: grammars.length, sourceWrites: 0, nativeExecuted: 0 }));
