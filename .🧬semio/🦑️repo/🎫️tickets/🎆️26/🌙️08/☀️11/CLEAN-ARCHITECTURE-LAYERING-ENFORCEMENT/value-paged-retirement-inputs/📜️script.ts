import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import Ajv from "ajv/dist/2020";

let root = import.meta.dir;
while (!existsSync(resolve(root, "Cargo.toml")) || !existsSync(resolve(root, "nx.json"))) root = dirname(root);
const ticket = dirname(import.meta.dir);
const output = resolve(ticket, "🗑️generated/value-paged-retirement");
const owner = "🧰️framework/🔨️modules/🌱️value";
const carrier = owner + "/🧬️retained-clone/📦️paged";
const list = owner + "/🧬️retained-clone/📋️paged-list";
const hash = (source: string) => createHash("sha256").update(source).digest("hex");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const splice = (source: string, from: string, to: string, count = 1) => {
  assert.equal(source.split(from).length - 1, count, "exact owned span count");
  return source.replaceAll(from, to);
};
const edit = (before: string, after: string) => {
  let start = 0, end = 0;
  while (start < before.length && start < after.length && before[start] === after[start]) start++;
  while (end < before.length - start && end < after.length - start && before[before.length - 1 - end] === after[after.length - 1 - end]) end++;
  return { start, remove: before.slice(start, before.length - end), insert: after.slice(start, after.length - end) };
};
const apply = (source: string, delta: ReturnType<typeof edit>) => {
  assert.equal(source.slice(delta.start, delta.start + delta.remove.length), delta.remove);
  return source.slice(0, delta.start) + delta.insert + source.slice(delta.start + delta.remove.length);
};
const frame = (path: string, before: string, authored = before) => {
  const forward = edit(before, authored), inverseEdit = edit(authored, before);
  assert.equal(apply(before, forward), authored);
  assert.equal(apply(authored, inverseEdit), before);
  return { path, before, authored, inverse: before, beforeHash: hash(before), authoredHash: hash(authored), forward, inverseEdit, changed: before !== authored };
};
mkdirSync(output, { recursive: true });
const version = process.argv[3] ?? "1";
if (process.argv[2] === "publish") {
  const readyPath = resolve(output, `source-ready-${version}.json`), readySource = readFileSync(readyPath, "utf8"), ready = JSON.parse(readySource);
  const predecessorPath = process.env.SEMIO_PACK_ERROR_TEST_PUBLICATION;
  assert.ok(predecessorPath, "Root test-only publication is required before this successor");
  const predecessorSource = readFileSync(predecessorPath, "utf8"), predecessor = JSON.parse(predecessorSource);
  assert.equal(predecessor.completed, true); assert.equal(predecessor.rows.length, 3);
  for (const row of predecessor.rows) assert.equal(read(row.path), row.after, row.path);
  assert.equal(ready.rows.length, 2);
  for (const row of [...ready.rows, ...ready.contexts]) {
    assert.equal(hash(row.before), row.beforeHash); assert.equal(hash(row.authored), row.authoredHash);
    assert.equal(row.inverse, row.before); assert.equal(apply(row.before, row.forward), row.authored);
    assert.equal(apply(row.authored, row.inverseEdit), row.before); assert.equal(read(row.path), row.before, row.path);
  }
  const destination = resolve(output, `publication-${version}.json`);
  assert.equal(existsSync(destination), false, "immutable publication destination");
  const journal: any = { schemaVersion: 1, ready: { path: readyPath, sha256: hash(readySource) }, predecessor: { path: predecessorPath, sha256: hash(predecessorSource) }, completed: false, sourceWrites: 0, nativeExecuted: false, rows: [] };
  writeFileSync(destination, JSON.stringify(journal, null, 2) + "\n");
  for (const row of ready.rows) {
    const freshBefore = read(row.path); assert.equal(freshBefore, row.before, row.path);
    writeFileSync(resolve(root, row.path), row.authored);
    const current = read(row.path); assert.equal(current, row.authored, row.path);
    journal.rows.push({ ...row, freshBefore, freshBeforeHash: hash(freshBefore), current, currentHash: hash(current), immediateExact: true });
    journal.sourceWrites++;
    writeFileSync(destination, JSON.stringify(journal, null, 2) + "\n");
  }
  journal.completed = true;
  writeFileSync(destination, JSON.stringify(journal, null, 2) + "\n");
  console.log(JSON.stringify({ destination, completed: true, sourceWrites: journal.sourceWrites, nativeExecuted: false }));
  process.exit(0);
}
if (process.argv[2] === "inspect") {
  const source = readFileSync(resolve(output, `source-ready-${version}.json`), "utf8"), receipt = JSON.parse(source);
  console.log(JSON.stringify({ path: resolve(output, `source-ready-${version}.json`), sha256: hash(source), pairs: receipt.compiledPairs.map((row: any) => ({ path: row.path, preRetained: row.preRetained, preCurrentExact: row.preCurrentExact, consumedChecksumProven: row.consumedChecksumProven, emittedAssociations: row.matches.length })) }));
  process.exit(0);
}
assert.equal(process.argv[2], "stage");
const destination = resolve(output, `source-ready-${version}.json`);
assert.equal(existsSync(destination), false, "immutable stage destination");
const carrierPath = carrier + "/🦀️.rs", listPath = list + "/🦀️.rs";
const carrierBefore = read(carrierPath), listBefore = read(listPath);
const carrierAuthored = splice(carrierBefore, "        if !self.close.is_empty() {\n            return self.close.step(maximum_items, maximum_bytes);\n        }", "        if !self.close.is_empty() {\n            let step = self.close.step(maximum_items, maximum_bytes)?;\n            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });\n        }", 3);
let listAuthored = splice(listBefore, "    allocation_bytes:usize,\n    allocation_remaining:usize,\n", "");
listAuthored = splice(listAuthored, "        if maximum_bytes==0 {return RetirementStep::BudgetExhausted;}\n        if self.allocation_bytes==0 {\n            let Ok(required)=self.owner.next_release_allocation_bytes() else {return RetirementStep::BudgetExhausted;};\n            self.allocation_bytes=required;self.allocation_remaining=required;\n        }\n        let charged=maximum_bytes.min(self.allocation_remaining);self.allocation_remaining-=charged;\n        if self.allocation_remaining==0 {\n            match self.owner.release_empty_page(self.allocation_bytes) {\n                Ok(progress) if progress.progressed=>{self.allocation_bytes=0;},\n                _=>return RetirementStep::BudgetExhausted,\n            }\n        }\n        RetirementStep::Bytes(charged)", "        let Ok(required) = self.owner.next_release_allocation_bytes() else { return RetirementStep::BudgetExhausted; };\n        if required > maximum_bytes {\n            return RetirementStep::BudgetExhausted;\n        }\n        match self.owner.release_empty_page(maximum_bytes) {\n            Ok(progress) if progress.progressed => RetirementStep::Bytes(progress.released_allocation_bytes),\n            _ => RetirementStep::BudgetExhausted,\n        }");
listAuthored = splice(listAuthored, "            Some(1)\n", "            self.owner.next_release_allocation_bytes().ok()\n");
listAuthored = splice(listAuthored, "released: false,allocation_bytes:0,allocation_remaining:0", "released: false");
const rows = [frame(carrierPath, carrierBefore, carrierAuthored), frame(listPath, listBefore, listAuthored)];
const paths = [carrier + "/🧪️tests/🔬️unit/🦀️.rs", carrier + "/🧫️fixtures/📦️owners/🔣️.json", carrier + "/🧫️fixtures/📦️owners/🧬️schema/🔣️.json", list + "/🧪️tests/🔬️unit/🦀️.rs", list + "/🧫️fixtures/📦️copy/🔣️.json", list + "/🧫️fixtures/📦️copy/🧬️schema/🔣️.json", owner + "/📋️list/🦀️.rs", owner + "/📦️paged/🦀️.rs", owner + "/🧬️retained-clone/🦀️.rs", owner + "/♻️retirement/🦀️.rs", owner + "/📦️packages/🦀️rust/Cargo.toml", owner + "/📦️packages/🦀️rust/🦀️.rs"];
const contexts = paths.map(path => frame(path, read(path)));
const schema = JSON.parse(read(paths[2])), fixture = JSON.parse(read(paths[1]));
const validate = new Ajv({ strict: true }).compile(schema);
assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
const { validateJsonSchemaSubset } = await import(resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
assert.deepEqual(validateJsonSchemaSubset(schema, fixture), []);
const hostile = [{ ...fixture, foreign: true }, { ...fixture, grant: { ...fixture.grant, maximumCapacityBytes: 8192 } }, { ...fixture, expected: { ...fixture.expected, terminalEmpty: false } }];
for (const row of hostile) {
  assert.equal(validate(row), false);
  assert.notEqual(validateJsonSchemaSubset(schema, row).length, 0);
}
const bytes = Array.from({ length: fixture.bytes.length }, (_, ordinal) => (ordinal * fixture.bytes.multiplier + fixture.bytes.increment) & 255);
const buffer = Buffer.alloc(fixture.bytes.length);
for (let ordinal = 0; ordinal < buffer.length; ordinal++) buffer.writeUInt8((ordinal * fixture.bytes.multiplier + fixture.bytes.increment) % 256, ordinal);
assert.deepEqual(Array.from(buffer), bytes);
const text = fixture.text.segment.repeat(fixture.text.repetitions);
assert.equal(new TextDecoder("utf-8", { fatal: true }).decode(Buffer.from(text, "utf8")), text);
const law = read(paths[0]);
assert.ok(law.includes("serde octet oracle") && law.includes("text serde oracle"));
assert.ok(law.includes("retirement.next_close_byte_demand() > grant.maximum_copy_bytes"));
const Parser = (await import("web-tree-sitter")).default;
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
const grammar = rows.map(row => {
  const observe = (source: string) => {
    const tree = parser.parse(source), errors: any[] = [];
    const visit = (node: any) => { if (node.type === "ERROR" || node.isMissing()) errors.push({ type: node.type, start: node.startPosition, end: node.endPosition }); for (const child of node.children) visit(child); };
    visit(tree.rootNode); tree.delete(); return errors;
  };
  const before = observe(row.before), authored = observe(row.authored);
  assert.deepEqual(before, []); assert.deepEqual(authored, []);
  return { path: row.path, before, authored, rustCompilerVerified: false };
});
parser.delete();
const nativePrefix = resolve(ticket, "🗑️generated/current-native-worker/record-cut-production-core-value-replay-1");
const nativeArtifacts = ["-before-1.json", "-value-terminal-1.json", "-value-compiler-post-1.json"].map(suffix => { const path = nativePrefix + suffix, source = readFileSync(path, "utf8"); return { path, sha256: hash(source) }; });
const nativeBefore = JSON.parse(readFileSync(nativeArtifacts[0].path, "utf8"));
const nativePost = JSON.parse(readFileSync(nativeArtifacts[2].path, "utf8"));
const prior = JSON.parse(readFileSync(nativeBefore.priorPath, "utf8"));
const base = JSON.parse(readFileSync(nativeBefore.priorBase, "utf8"));
const nativeSources = new Map([...base.rows, ...prior.sourceDeltas, ...nativeBefore.sourceDeltas, ...nativePost.immediate.sourceDeltas].map((row: any) => [resolve(root, row.path), row]));
const compiledInputs = nativePost.units.filter((unit: any) => unit.selected).flatMap((unit: any) => unit.inputs.map((input: any) => ({ ...input, emittedUnit: unit.path })));
const compiledPairs = [...rows, ...contexts].map(row => {
  const retained: any = nativeSources.get(resolve(root, row.path));
  const pre = retained?.source ?? null;
  const matches = compiledInputs.filter((input: any) => input.physical === resolve(root, row.path));
  return { path: row.path, pre, current: row.before, inverse: pre, preHash: pre === null ? null : hash(pre), currentHash: row.beforeHash, preRetained: pre !== null, matches, preCurrentExact: pre !== null && pre === row.before, forward: pre === null ? null : edit(pre, row.before), inverseEdit: pre === null ? null : edit(row.before, pre), provenance: pre === null ? "Missing retained pre-body; current observation is not reconstructed" : "Latest retained immutable physical pre-body; actual checksum association is stated separately", consumedChecksumProven: matches.length !== 0 && matches.every((input: any) => input.preMatch && input.currentMatch), attribution: "No source writer or semantic equivalence is inferred" };
});
const gaps = [...rows, ...contexts].filter(row => read(row.path) !== row.before).map(row => row.path);
assert.deepEqual(gaps, []);
const receipt = { schemaVersion: 1, observedAt: new Date().toISOString(), sourceWrites: 0, rows, contexts, gaps, grammar, nativeArtifacts, compiledPairs, nativeExecuted: false, mountReady: false, retainedAssertionsChanged: false, controlsChanged: false, originalRuntime: { selected: 136, run: 16, passed: 14, failed: 2, skipped: 0, notRun: 120, capacityLawExecuted: false }, proof: { firstPartySchema: true, ajv: true, hostileRefusals: hostile.length, independentNodeBytes: bytes.length, independentNodeUtf8Bytes: Buffer.byteLength(text), retainedSerdeNativeOracles: true, physicalAllocationBytesClaim: "Exact next_release_allocation_bytes provider result, not fixture capacityBytes", protocol: "Inner retirement Complete remains outer Pending until remaining owners and source binding close; physical allocation release requires the complete per-turn demand without fractional credit" } };
writeFileSync(destination, JSON.stringify(receipt, null, 2) + "\n");
console.log(JSON.stringify({ destination, rows: rows.length, contexts: contexts.length, sourceWrites: 0, gaps, grammar: grammar.map(row => ({ path: row.path, before: row.before.length, authored: row.authored.length })), nativeExecuted: false }));
