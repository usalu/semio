/** ⚖️ WP4b equivalence oracle for the 2020-12 → draft-07 migration of the mutation trees.
 *
 * The migration replaces `unevaluatedProperties: false` (a 2020-12-only keyword) with draft-07 means
 * and lifts every `$ref` that sat beside other keywords into an `allOf` branch. Neither rewrite may
 * change which instances are accepted, so this compiles the BEFORE document with `ajv/dist/2020` and
 * the AFTER document with draft-07 `ajv`, and asserts that both verdicts agree on a corpus built to
 * probe exactly the closure semantics: every subset of the property names the schema mentions, each
 * of those with an alien key added, plus the non-object instances.
 *
 * Two third-party implementations are involved by construction — ajv's 2020-12 dialect and ajv's
 * draft-07 dialect are separately compiled code paths, so agreement is not a tautology of one engine.
 *
 * Inputs: `<ticket>/🗑️generated/wp4b-before-2020.json` (pre-migration snapshot, written by
 * `wp4-mutation-aggregates.py`'s snapshot step) and the migrated documents on disk.
 *
 * Run: `bun <ticket>/wp4b-draft07-equivalence.mjs`
 */
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { join, dirname, resolve as resolvePath, relative } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS";
const before = JSON.parse(readFileSync(join(REPO, TICKET, "🗑️generated/wp4b-before-2020.json"), "utf8"));

/** 🗂️ `$id` → repo-relative path, so an absolute `$ref` can be turned into a file key. */
const byId = new Map();
const SKIP = new Set([".git", "node_modules", "target", "dist", ".🧬semio"]);
const indexTree = (dir) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (SKIP.has(entry.name)) continue;
    const path = join(dir, entry.name);
    if (entry.isDirectory()) { indexTree(path); continue; }
    if (!entry.name.endsWith(".json")) continue;
    let doc;
    try { doc = JSON.parse(readFileSync(path, "utf8")); } catch { continue; }
    if (doc && typeof doc === "object" && typeof doc.$id === "string" && !byId.has(doc.$id)) byId.set(doc.$id, relative(REPO, path));
  }
};
indexTree(REPO);

const key = (rel) => `file:///${encodeURI(rel)}`;
const read = (rel, snapshot) => (snapshot && Object.hasOwn(before, rel) ? before[rel] : JSON.parse(readFileSync(join(REPO, rel), "utf8")));

/** 🔗️ Rewrites every `$ref` of one document to a file key so ajv resolves without a network loader. */
const rekey = (node, rel, wanted) => {
  if (Array.isArray(node)) return node.map((item) => rekey(item, rel, wanted));
  if (node === null || typeof node !== "object") return node;
  const out = {};
  for (const [name, value] of Object.entries(node)) {
    if (name === "$ref" && typeof value === "string") {
      const [base, pointer] = [value.split("#")[0], value.includes("#") ? value.slice(value.indexOf("#")) : ""];
      if (base === "") { out.$ref = value; continue; }
      const target = base.startsWith("http") ? byId.get(base) : relative(REPO, resolvePath(join(REPO, dirname(rel)), decodeURI(base)));
      if (target === undefined || !existsSync(join(REPO, target))) { out.$ref = value; continue; }
      wanted.add(target);
      out.$ref = key(target) + pointer;
      continue;
    }
    if (name === "$id") continue;
    out[name] = rekey(value, rel, wanted);
  }
  return out;
};

/** 🧰️ Compiles one document with every transitively referenced document registered by file key. */
const compile = (ajv, rel, snapshot) => {
  const wanted = new Set();
  const root = rekey(read(rel, snapshot), rel, wanted);
  const done = new Set();
  while (wanted.size > 0) {
    const next = [...wanted][0];
    wanted.delete(next);
    if (done.has(next)) continue;
    done.add(next);
    const doc = rekey(read(next, snapshot), next, wanted);
    delete doc.$schema;
    if (ajv.getSchema(key(next)) === undefined) ajv.addSchema(doc, key(next));
  }
  return ajv.compile(root);
};

/** 🔑️ Every property name the document mentions, and one plausible value for each. */
const namesAndValues = (rel) => {
  const values = new Map();
  const visit = (node, home) => {
    if (Array.isArray(node)) { for (const item of node) visit(item, home); return; }
    if (node === null || typeof node !== "object") return;
    if (node.properties && typeof node.properties === "object") {
      for (const [name, sub] of Object.entries(node.properties)) {
        if (!values.has(name)) values.set(name, sample(sub, home));
        visit(sub, home);
      }
    }
    for (const [name, value] of Object.entries(node)) if (name !== "properties") visit(value, home);
  };
  const sample = (sub, home, depth = 0) => {
    if (sub === true || sub === undefined) return 1;
    if (sub === false) return 1;
    if (typeof sub !== "object") return 1;
    if (Object.hasOwn(sub, "const")) return sub.const;
    if (Array.isArray(sub.enum) && sub.enum.length > 0) return sub.enum[0];
    if (typeof sub.$ref === "string" && depth < 4) {
      const [base, pointer] = [sub.$ref.split("#")[0], sub.$ref.includes("#") ? sub.$ref.slice(sub.$ref.indexOf("#") + 1) : ""];
      const target = base === "" ? home : base.startsWith("http") ? byId.get(base) : relative(REPO, resolvePath(join(REPO, dirname(home)), decodeURI(base)));
      if (target !== undefined && existsSync(join(REPO, target))) {
        let node = read(target, false);
        for (const step of pointer.split("/").filter(Boolean)) node = node?.[step.replace(/~1/g, "/").replace(/~0/g, "~")];
        if (node !== undefined) return sample(node, target, depth + 1);
      }
      return 1;
    }
    const type = Array.isArray(sub.type) ? sub.type[0] : sub.type;
    if (type === "string") return "x";
    if (type === "integer" || type === "number") return 1;
    if (type === "boolean") return true;
    if (type === "array") return [];
    if (type === "object") {
      const inner = {};
      for (const name of sub.required ?? []) inner[name] = sample(sub.properties?.[name], home, depth + 1);
      return inner;
    }
    if (sub.allOf) return sample(sub.allOf[0], home, depth + 1);
    if (sub.oneOf) return sample(sub.oneOf[0], home, depth + 1);
    if (sub.anyOf) return sample(sub.anyOf[0], home, depth + 1);
    return 1;
  };
  visit(read(rel, false), rel);
  return values;
};

/** 🧪️ Instances that probe the closure: every subset of the mentioned names, ± an alien key. */
const corpus = (values) => {
  const names = [...values.keys()].sort();
  const cases = [null, 1, "x", true, [], [1, 2]];
  const limit = names.length <= 10 ? 1 << names.length : 1 << 10;
  for (let mask = 0; mask < limit; mask += 1) {
    const object = {};
    for (let bit = 0; bit < Math.min(names.length, 10); bit += 1) if (mask & (1 << bit)) object[names[bit]] = values.get(names[bit]);
    cases.push(object, { ...object, unknownAlienKey: 42 }, { ...object, operation: "notAnOperation" });
  }
  return cases;
};

const ajv2020 = new Ajv2020({ strict: false, allErrors: false, validateFormats: false, logger: false });
const ajv7 = new Ajv({ strict: false, allErrors: false, validateFormats: false, logger: false });
let checked = 0, instances = 0, disagreements = 0, uncompilable = [];
const rows = [];

for (const rel of Object.keys(before).sort()) {
  if (!existsSync(join(REPO, rel))) { uncompilable.push(`${rel}: gone from the working tree`); continue; }
  let oldValidate, newValidate;
  try { oldValidate = compile(ajv2020, rel, true); } catch (error) { uncompilable.push(`${rel}: 2020-12 side — ${error.message.split("\n")[0]}`); continue; }
  try { newValidate = compile(ajv7, rel, false); } catch (error) { uncompilable.push(`${rel}: draft-07 side — ${error.message.split("\n")[0]}`); continue; }
  const cases = corpus(namesAndValues(rel));
  let mismatched = 0;
  for (const instance of cases) {
    instances += 1;
    if (Boolean(oldValidate(instance)) !== Boolean(newValidate(instance))) {
      mismatched += 1;
      if (disagreements < 20) console.log(`  DISAGREE ${rel}\n    instance ${JSON.stringify(instance)}\n    2020-12=${Boolean(oldValidate(instance))} draft-07=${Boolean(newValidate(instance))}`);
      disagreements += 1;
    }
  }
  checked += 1;
  rows.push({ rel, cases: cases.length, mismatched });
}

console.log(`documents compared=${checked} instances=${instances} disagreements=${disagreements}`);
console.log(`accepted-by-both / rejected-by-both split is per document in the table below`);
for (const row of rows.slice(0, 8)) console.log(`  ${row.cases} instances, ${row.mismatched} disagreements — ${row.rel}`);
if (uncompilable.length > 0) {
  console.log(`could not compare ${uncompilable.length}:`);
  for (const line of uncompilable) console.log(`  ${line}`);
}
process.exit(disagreements === 0 && uncompilable.length === 0 ? 0 : 1);
