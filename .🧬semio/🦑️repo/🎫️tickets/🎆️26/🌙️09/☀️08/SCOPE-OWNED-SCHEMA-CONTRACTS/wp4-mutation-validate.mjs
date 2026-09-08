/** 🧪️ WP4 mutation payload-schema validator.
 *
 * Walks every `🧬️mutations/` root outside `✏️s/🔌️plugins/🗄️stdio/` and `.🧬semio/`, and asserts:
 * (a) each leaf descriptor's `payloadSchema` resolves to an existing file,
 * (b) that file parses, declares draft-07, `$id` and `title`, and its `$id` is unique repo-wide,
 * (c) every aggregate `oneOf` branch names its leaf by **absolute `$id`** (cross-partition row 79) and
 *     that `$id` belongs to a document on disk,
 * (d) ajv (draft-07) compiles every aggregate with the whole transitive `$ref` closure registered in one
 *     instance keyed by `$id` — the resolution model row 79 mandates, not filesystem resolution,
 * (e) ajv compiles every leaf schema standalone, with the same closure registration.
 *
 * Run: `bun <ticket>/wp4-mutation-validate.mjs`
 */
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, dirname, resolve as resolvePath, relative } from "node:path";
import Ajv from "ajv";

const REPO = "/Users/ueli/Documents/semio";
const DRAFT7 = "http://json-schema.org/draft-07/schema#";
const SKIP = new Set([".git", "node_modules", "target", "dist", ".🧬semio"]);
const roots = [];

const walk = (dir) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (!entry.isDirectory() || SKIP.has(entry.name)) continue;
    const path = join(dir, entry.name);
    if (path.includes("/✏️s/🔌️plugins/🗄️stdio/")) continue;
    if (entry.name === "🧬️mutations") { roots.push(path); continue; }
    walk(path);
  }
};
walk(REPO);

const leavesOf = (root) => {
  const found = [];
  const NON_LEAF = new Set(["🔺️diff", "↩️inverse", "📝️text", "💾️binary", "🦠️mutation", "🧩️plan", "🧬️schema", "🧬️wire", "🛜️wire", "🧪️tests", "🧪️fixtures", "🧪️descriptor", "🧫️fixtures", "🧫️fixture"]);
  const descend = (dir) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (!entry.isDirectory() || NON_LEAF.has(entry.name)) continue;
      const path = join(dir, entry.name);
      if (existsSync(join(path, "🔣️.json"))) {
        let descriptor;
        try { descriptor = JSON.parse(readFileSync(join(path, "🔣️.json"), "utf8")); } catch { continue; }
        if (descriptor.payloadSchema && descriptor.semanticKind) { found.push({ path, descriptor }); continue; }
      }
      descend(path);
    }
  };
  descend(root);
  return found;
};

/** 🌐️ `$id` index over the whole tree: with row 79 every cross-document `$ref` is an `$id`, so the
 * resolver is a map lookup, never a path walk. The file path is kept only to report where a document
 * came from and to resolve the one legacy relative `$ref` that still exists (see §row-65 request). */
const byId = new Map();
const indexModules = (dir) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (SKIP.has(entry.name)) continue;
    const path = join(dir, entry.name);
    if (entry.isDirectory()) { indexModules(path); continue; }
    if (!entry.name.endsWith(".json")) continue;
    try {
      const doc = JSON.parse(readFileSync(path, "utf8"));
      if (doc && typeof doc === "object" && typeof doc.$id === "string" && !byId.has(doc.$id)) byId.set(doc.$id, { doc, path });
    } catch {}
  }
};
indexModules(join(REPO, "✏️s"));
indexModules(join(REPO, "🧰️framework"));

const problems = [];
const ids = new Map();
let leafCount = 0, aggregateCount = 0, compiled = 0, leafCompiled = 0, branchCount = 0;

/** 🧩️ One Ajv instance holding the transitive `$ref` closure of `entry`, every member keyed by `$id`.
 * A relative `$ref` (none should remain) is resolved against the referring FILE and registered under a
 * file key, so a leftover is reported by the compile rather than silently resolving to nothing. */
const compileClosure = (entryDoc, entryPath, label) => {
  const ajv = new Ajv({ allErrors: true, strict: false });
  const fileKey = (path) => `file:///${encodeURI(relative(REPO, path))}`;
  const pending = [];
  let ok = true;
  const normalize = (node, home) => {
    if (Array.isArray(node)) return node.map((item) => normalize(item, home));
    if (node === null || typeof node !== "object") return node;
    const out = {};
    for (const [name, value] of Object.entries(node)) {
      if (name === "$ref" && typeof value === "string" && !value.startsWith("#")) {
        const base = value.split("#")[0];
        const pointer = value.includes("#") ? value.slice(value.indexOf("#")) : "";
        if (base.startsWith("http")) {
          if (!byId.has(base)) { problems.push(`${label}: $ref ${base} names no document on disk`); ok = false; out.$ref = value; continue; }
          pending.push({ key: base, doc: byId.get(base).doc, path: byId.get(base).path });
          out.$ref = value;
          continue;
        }
        const target = resolvePath(home, decodeURI(base));
        if (!existsSync(target)) { problems.push(`${label}: relative $ref ${value} does not resolve`); ok = false; out.$ref = value; continue; }
        let doc;
        try { doc = JSON.parse(readFileSync(target, "utf8")); } catch (error) { problems.push(`${label}: cannot read ${relative(REPO, target)} (${error.message})`); ok = false; out.$ref = value; continue; }
        pending.push({ key: fileKey(target), doc, path: target });
        out.$ref = fileKey(target) + pointer;
        continue;
      }
      out[name] = normalize(value, home);
    }
    return out;
  };
  const root = normalize(entryDoc, dirname(entryPath));
  const done = new Set([entryDoc.$id].filter(Boolean));
  while (pending.length > 0) {
    const next = pending.shift();
    if (done.has(next.key)) continue;
    done.add(next.key);
    const member = { ...normalize(next.doc, dirname(next.path)), $schema: DRAFT7, $id: next.key };
    try { if (ajv.getSchema(next.key) === undefined) ajv.addSchema(member, next.key); }
    catch (error) { problems.push(`${label}: cannot register ${next.key} (${error.message})`); ok = false; }
  }
  if (!ok) return false;
  try { ajv.compile({ ...root, $schema: DRAFT7 }); return true; }
  catch (error) { problems.push(`${label}: ajv compile failed (${error.message})`); return false; }
};

for (const root of roots.sort()) {
  const rel = relative(REPO, root);
  const leaves = leavesOf(root);
  for (const { path, descriptor } of leaves) {
    leafCount += 1;
    const schemaPath = join(path, descriptor.payloadSchema);
    if (descriptor.payloadSchema !== "🧬️schema/🔣️.json") problems.push(`${relative(REPO, path)}: payloadSchema is ${descriptor.payloadSchema}, not the taxonomy default`);
    if (!existsSync(schemaPath)) { problems.push(`${relative(REPO, path)}: payloadSchema target missing`); continue; }
    let doc;
    try { doc = JSON.parse(readFileSync(schemaPath, "utf8")); } catch (error) { problems.push(`${relative(REPO, schemaPath)}: unparseable (${error.message})`); continue; }
    if (doc.$schema !== DRAFT7) problems.push(`${relative(REPO, schemaPath)}: dialect ${doc.$schema}`);
    if (!doc.$id) problems.push(`${relative(REPO, schemaPath)}: missing $id`);
    if (!doc.title) problems.push(`${relative(REPO, schemaPath)}: missing title`);
    if (doc.title && descriptor.aggregateVariant && doc.title !== descriptor.aggregateVariant) problems.push(`${relative(REPO, schemaPath)}: title ${doc.title} != aggregateVariant ${descriptor.aggregateVariant}`);
    if (doc.$id) {
      if (ids.has(doc.$id)) problems.push(`${relative(REPO, schemaPath)}: duplicate $id with ${ids.get(doc.$id)}`);
      else ids.set(doc.$id, relative(REPO, schemaPath));
    }
    if (compileClosure(doc, schemaPath, `${relative(REPO, schemaPath)} [standalone]`)) leafCompiled += 1;
  }

  const aggregatePath = join(root, "🔣️.json");
  if (!existsSync(aggregatePath)) continue;
  aggregateCount += 1;
  let aggregate;
  try { aggregate = JSON.parse(readFileSync(aggregatePath, "utf8")); } catch (error) { problems.push(`${rel}/🔣️.json: unparseable (${error.message})`); continue; }
  if (aggregate.$schema !== DRAFT7) problems.push(`${rel}/🔣️.json: dialect ${aggregate.$schema}`);
  // 🌿️Two union shapes are legal. A plugin aggregate's branch is a bare `$ref` to the leaf (G-B).
  // A framework/os conformance aggregate's branch is a discriminated `allOf` of the leaf payload and a
  // `const` operation tag; the leaf reference is still exactly one member, so both are checked the same
  // way: every branch must name at least one leaf, by `$id`.
  const branches = aggregate.oneOf ?? (aggregate.allOf || aggregate.properties ? [aggregate] : []);
  const refsOf = (branch) => {
    const found = [];
    const dig = (node) => {
      if (Array.isArray(node)) { for (const item of node) dig(item); return; }
      if (node === null || typeof node !== "object") return;
      for (const [name, value] of Object.entries(node)) {
        if (name === "$ref" && typeof value === "string" && !value.startsWith("#")) found.push(value);
        else dig(value);
      }
    };
    dig(branch);
    return found;
  };
  for (const branch of branches) {
    const refs = refsOf(branch);
    if (refs.length === 0) { problems.push(`${rel}/🔣️.json: union branch names no leaf schema`); continue; }
    for (const ref of refs) {
      branchCount += 1;
      if (!ref.startsWith("http")) { problems.push(`${rel}/🔣️.json: branch $ref ${ref} is a path, not the target $id (row 79)`); continue; }
      if (!byId.has(ref.split("#")[0])) problems.push(`${rel}/🔣️.json: branch $ref ${ref} names no document on disk`);
    }
  }
  if (compileClosure(aggregate, aggregatePath, `${rel}/🔣️.json`)) compiled += 1;
}

const plugins = problems.filter((problem) => problem.startsWith("✏️s/"));
const deferred = problems.filter((problem) => !problem.startsWith("✏️s/"));
console.log(`roots=${roots.length} leaves=${leafCount} aggregates=${aggregateCount} branches=${branchCount} ajvCompiled=${compiled} leavesCompiledStandalone=${leafCompiled}`);
console.log(`WP4 partition (✏️s/🔌️plugins, non-stdio) problems=${plugins.length}`);
for (const problem of plugins.slice(0, 60)) console.log(`  ${problem}`);
console.log(`framework/os mutation trees problems=${deferred.length}`);
for (const problem of deferred.slice(0, 60)) console.log(`  ${problem}`);
process.exit(problems.length === 0 ? 0 : 1);
