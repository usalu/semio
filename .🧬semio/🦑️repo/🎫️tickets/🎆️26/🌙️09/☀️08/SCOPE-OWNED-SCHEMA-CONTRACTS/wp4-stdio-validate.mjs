/**
 * ✅️ Compiles every `🗄️stdio` mutation-module aggregate against a single Ajv registry that holds every
 * mutation leaf payload schema and every mutation codec facet document keyed by its own `$id`, then
 * replays each leaf's committed `🧪️tests/…/🦠️mutation/🔣️.json` wire object through both the aggregate
 * and the leaf's own schema.
 *
 * Cross-partition row 79: aggregate `oneOf` branches name the leaf's absolute `$id`, resolved through
 * the catalog — never a filesystem-relative path — so the oracle must be a catalog too: one Ajv, every
 * document added under its declared `$id`, no re-keying and no disk resolution of `$ref`s.
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
const FACETS = ["📝️text", "💾️binary"];

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
const exists = (path) => {
  try {
    statSync(path);
    return true;
  } catch {
    return false;
  }
};

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

const failures = [];

const aggregates = [];
const leafSchemas = [];
const facetDocuments = [];
for (const directory of directories(ARTIFACTS)) {
  const path = join(directory, "🔣️.json");
  const name = basename(directory);
  const parent = basename(dirname(directory));
  if (name === MUTATIONS && parent === SCHEMA && exists(path)) aggregates.push({ directory, path, document: read(path) });
  else if (name === SCHEMA && directory.includes(`${MUTATIONS}/`) && exists(path)) leafSchemas.push(path);
  else if (FACETS.includes(name) && parent === MUTATIONS && basename(dirname(dirname(directory))) === SCHEMA && exists(path)) facetDocuments.push(path);
}

// 📇️ One registry, every document under its declared `$id` — the oracle's stand-in for the catalog.
const registry = new Ajv7({ strict: false, allErrors: true, validateFormats: false });
const leavesById = new Map();
let registered = 0;
for (const [path, kind] of [...leafSchemas.map((path) => [path, "leaf"]), ...facetDocuments.map((path) => [path, "facet"])]) {
  const document = read(path);
  if (typeof document.$id !== "string") {
    failures.push(`${relative(REPO, path)}: ${kind} declares no $id, so no aggregate can address it`);
    continue;
  }
  try {
    registry.addSchema(document);
    registered += 1;
  } catch (error) {
    failures.push(`${relative(REPO, path)}: not addable to the registry (${error.message})`);
    continue;
  }
  if (kind === "leaf") leavesById.set(document.$id, { target: path, leaf: document });
}

let compiled = 0;
let leavesChecked = 0;
let fixturesChecked = 0;

for (const aggregate of aggregates) {
  let validate;
  try {
    validate = registry.compile(aggregate.document);
    compiled += 1;
  } catch (error) {
    failures.push(`${relative(REPO, aggregate.path)}: does not compile (${error.message})`);
    continue;
  }

  const branches = Array.isArray(aggregate.document.oneOf) ? aggregate.document.oneOf : [];
  for (const [index, branch] of branches.entries()) {
    const shape = branchShape(branch);
    if (shape === null) {
      failures.push(`${relative(REPO, aggregate.path)}: oneOf[${index}] carries no leaf $ref`);
      continue;
    }
    const reference = references(branch)[0];
    const entry = leavesById.get(reference);
    if (entry === undefined) {
      failures.push(`${relative(REPO, aggregate.path)}: oneOf[${index}] $ref ${reference} names no registered leaf $id`);
      continue;
    }
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
      if (!validate(value)) failures.push(`${relative(REPO, fixture)}: rejected by ${relative(REPO, aggregate.path)} :: ${registry.errorsText(validate.errors).slice(0, 240)}`);
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

// 🎨️ Every codec facet document on its own: same dialect, same compilability bar.
let facetsCompiled = 0;
for (const path of facetDocuments) {
  const document = read(path);
  if (document.$schema !== "http://json-schema.org/draft-07/schema#") failures.push(`${relative(REPO, path)}: dialect is ${document.$schema}`);
  const ajv = new Ajv7({ strict: false, allErrors: true, validateFormats: false });
  try {
    ajv.compile(document);
    facetsCompiled += 1;
  } catch (error) {
    failures.push(`${relative(REPO, path)}: does not compile (${error.message})`);
  }
}

console.log(`aggregates=${aggregates.length} compiled=${compiled} leaf-branches=${leavesChecked} leaf-schemas=${leafSchemas.length} compiled-standalone=${standalone} facets=${facetDocuments.length} facets-compiled=${facetsCompiled} registered=${registered} fixtures=${fixturesChecked} failures=${failures.length}`);
const limit = process.argv.includes("--all") ? failures.length : 40;
for (const failure of failures.slice(0, limit)) console.log("  FAIL", failure);
if (failures.length > limit) console.log(`  … ${failures.length - limit} more`);
process.exit(failures.length === 0 ? 0 : 1);
