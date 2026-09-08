/**
 * ✅️ Compiles every `🗄️stdio` mutation-module aggregate together with the leaf payload schemas it
 * `$ref`s, then replays each leaf's committed `🧪️tests/…/🦠️mutation/🔣️.json` wire object through both
 * the aggregate and the leaf's own schema.
 *
 * ajv is the third-party oracle here, not a runtime dependency of the repository: it is only ever
 * asked whether the handcrafted draft-07 documents accept the handcrafted fixtures.
 *
 * Run: bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/wp4-stdio-validate.mjs
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import Ajv7 from "ajv";

const REPO = new URL("../../../../../../../", import.meta.url).pathname;
const ARTIFACTS = join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
const MUTATIONS = "🧬️mutations";
const SCHEMA = "🧬️schema";

/** 🗂️ Every directory under `root`, depth-first. */
function* directories(root) {
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const path = join(root, entry.name);
    yield path;
    yield* directories(path);
  }
}

const read = (path) => JSON.parse(readFileSync(path, "utf8"));

/** 🔗️ Collects every `$ref` string in a document. */
function references(node, out = []) {
  if (Array.isArray(node)) for (const item of node) references(item, out);
  else if (node !== null && typeof node === "object") {
    if (typeof node.$ref === "string") out.push(node.$ref);
    for (const value of Object.values(node)) references(value, out);
  }
  return out;
}

/** 🧭️ How one aggregate branch carries its leaf payload, read off the branch itself. */
function branchShape(branch) {
  if (Array.isArray(branch.allOf)) return { kind: "internal", pick: (value) => value };
  const properties = branch.properties ?? {};
  const keys = Object.keys(properties);
  const content = keys.find((key) => typeof properties[key]?.$ref === "string");
  const tag = keys.find((key) => typeof properties[key]?.const === "string");
  if (content === undefined) return null;
  if (tag === undefined) return { kind: "external", pick: (value) => value?.[content] };
  return { kind: "adjacent", pick: (value) => value?.[content] };
}

const aggregates = [];
for (const directory of directories(ARTIFACTS)) {
  if (basename(directory) !== MUTATIONS) continue;
  if (basename(dirname(directory)) !== SCHEMA) continue;
  const path = join(directory, "🔣️.json");
  try {
    statSync(path);
  } catch {
    continue;
  }
  aggregates.push({ directory, path, document: read(path) });
}

let compiled = 0;
let leavesChecked = 0;
let fixturesChecked = 0;
const failures = [];

for (const aggregate of aggregates) {
  const ajv = new Ajv7({ strict: false, allErrors: true, validateFormats: false });
  // 🔗️ The on-disk `$ref` is a RELATIVE FILE PATH; resolving it is the filesystem's job, not a URI
  // resolver's (percent-encoding every emoji segment only invents a second, unrelated identity). Each
  // reference is read from disk and re-keyed onto a `urn:` the compiler can resolve unambiguously.
  const registered = new Map();
  const keyed = new Map();
  let broken = false;
  const rekey = (node) => {
    if (Array.isArray(node)) return node.map(rekey);
    if (node === null || typeof node !== "object") return node;
    const out = {};
    for (const [property, value] of Object.entries(node)) {
      if (property === "$ref" && typeof value === "string" && !value.startsWith("#")) {
        const target = join(aggregate.directory, value);
        if (!keyed.has(target)) {
          let leaf;
          try {
            leaf = read(target);
          } catch (error) {
            failures.push(`${relative(REPO, aggregate.path)}: $ref ${value} does not resolve (${error.message})`);
            broken = true;
            keyed.set(target, null);
            continue;
          }
          const key = `urn:semio-stdio-leaf:${keyed.size}`;
          const clone = { ...leaf };
          delete clone.$id;
          try {
            ajv.addSchema(clone, key);
          } catch (error) {
            failures.push(`${relative(REPO, target)}: not addable to ajv (${error.message})`);
            broken = true;
          }
          keyed.set(target, key);
          registered.set(key, { target, leaf });
        }
        const key = keyed.get(target);
        if (key !== null) out.$ref = key;
        continue;
      }
      out[property] = rekey(value);
    }
    return out;
  };
  const rekeyed = rekey(aggregate.document);
  if (broken) continue;
  delete rekeyed.$id;
  let validate;
  try {
    validate = ajv.compile(rekeyed);
    compiled += 1;
  } catch (error) {
    failures.push(`${relative(REPO, aggregate.path)}: does not compile (${error.message})`);
    continue;
  }

  const branches = Array.isArray(rekeyed.oneOf) ? rekeyed.oneOf : [];
  for (const [index, branch] of branches.entries()) {
    const shape = branchShape(branch);
    if (shape === null) {
      failures.push(`${relative(REPO, aggregate.path)}: oneOf[${index}] carries no leaf $ref`);
      continue;
    }
    const entry = registered.get(references(branch)[0]);
    if (entry === undefined) continue;
    leavesChecked += 1;
    const leafAjv = new Ajv7({ strict: false, allErrors: true, validateFormats: false });
    let leafValidate;
    try {
      leafValidate = leafAjv.compile(entry.leaf);
    } catch (error) {
      failures.push(`${relative(REPO, entry.target)}: does not compile standalone (${error.message})`);
      continue;
    }
    const tests = join(dirname(dirname(entry.target)), "🧪️tests");
    let cases = [];
    try {
      cases = readdirSync(tests, { withFileTypes: true }).filter((item) => item.isDirectory()).map((item) => join(tests, item.name, "🦠️mutation", "🔣️.json"));
    } catch {
      cases = [];
    }
    for (const fixture of cases) {
      let value;
      try {
        value = read(fixture);
      } catch {
        continue;
      }
      fixturesChecked += 1;
      if (!validate(value)) failures.push(`${relative(REPO, fixture)}: rejected by ${relative(REPO, aggregate.path)} :: ${ajv.errorsText(validate.errors).slice(0, 240)}`);
      const payload = shape.pick(value);
      if (payload === undefined) {
        failures.push(`${relative(REPO, fixture)}: carries no ${shape.kind} payload for ${relative(REPO, entry.target)}`);
        continue;
      }
      if (!leafValidate(payload)) failures.push(`${relative(REPO, fixture)}: payload rejected by ${relative(REPO, entry.target)} :: ${leafAjv.errorsText(leafValidate.errors).slice(0, 240)}`);
    }
  }
}

// 🧬️ Every leaf schema on its own: draft-07 dialect, resolvable, compilable.
const leafSchemas = [];
for (const directory of directories(ARTIFACTS)) {
  if (basename(directory) !== SCHEMA) continue;
  if (!directory.includes(`${MUTATIONS}/`)) continue;
  const path = join(directory, "🔣️.json");
  try {
    statSync(path);
  } catch {
    continue;
  }
  leafSchemas.push(path);
}
let standalone = 0;
for (const path of leafSchemas) {
  const document = read(path);
  if (document.$schema !== "http://json-schema.org/draft-07/schema#") failures.push(`${relative(REPO, path)}: dialect is ${document.$schema}`);
  const ajv = new Ajv7({ strict: false, allErrors: true, validateFormats: false });
  try {
    ajv.compile(document);
    standalone += 1;
  } catch (error) {
    failures.push(`${relative(REPO, path)}: does not compile (${error.message})`);
  }
}

console.log(`aggregates=${aggregates.length} compiled=${compiled} leaf-branches=${leavesChecked} leaf-schemas=${leafSchemas.length} compiled-standalone=${standalone} fixtures=${fixturesChecked} failures=${failures.length}`);
for (const failure of failures.slice(0, 80)) console.log("  FAIL", failure);
if (failures.length > 80) console.log(`  … ${failures.length - 80} more`);
process.exit(failures.length === 0 ? 0 : 1);
