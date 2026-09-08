/** 🧪️ WP4 mutation payload-schema validator.
 *
 * Walks every `🧬️mutations/` root outside `✏️s/🔌️plugins/🗄️stdio/` and `.🧬semio/`, and asserts:
 * (a) each leaf descriptor's `payloadSchema` resolves to an existing file,
 * (b) that file parses, declares draft-07, `$id` and `title`,
 * (c) every aggregate `oneOf` `$ref` resolves to a real leaf schema file,
 * (d) ajv (draft-07) compiles every aggregate together with the leaf schemas it references.
 *
 * Run: `bun <ticket>/wp4-mutation-validate.mjs`
 */
import { readdirSync, readFileSync, statSync, existsSync } from "node:fs";
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

/** 🌐️ Cross-scope `$id` index: every module-level `🧬️schema/🔣️.json` a leaf may `$ref` by URL. */
const external = new Map();
const indexModules = (dir) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (!entry.isDirectory() || SKIP.has(entry.name)) continue;
    const path = join(dir, entry.name);
    if (entry.name === "🧬️schema" && existsSync(join(path, "🔣️.json"))) {
      try {
        const doc = JSON.parse(readFileSync(join(path, "🔣️.json"), "utf8"));
        if (doc.$id) external.set(doc.$id, doc);
      } catch {}
    }
    indexModules(path);
  }
};
indexModules(join(REPO, "✏️s"));

const problems = [];
const ids = new Map();
let leafCount = 0, aggregateCount = 0, compiled = 0;

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
  }

  const aggregatePath = join(root, "🔣️.json");
  if (!existsSync(aggregatePath)) continue;
  aggregateCount += 1;
  let aggregate;
  try { aggregate = JSON.parse(readFileSync(aggregatePath, "utf8")); } catch (error) { problems.push(`${rel}/🔣️.json: unparseable (${error.message})`); continue; }
  const branches = aggregate.oneOf ?? [];
  const targets = [];
  for (const branch of branches) {
    if (!branch.$ref) { problems.push(`${rel}/🔣️.json: oneOf branch without $ref`); continue; }
    if (branch.$ref.startsWith("#") || branch.$ref.startsWith("http")) continue;
    const target = resolvePath(root, decodeURI(branch.$ref));
    if (!existsSync(target)) { problems.push(`${rel}/🔣️.json: $ref ${branch.$ref} does not resolve`); continue; }
    targets.push({ ref: branch.$ref, target });
  }
  if (!aggregate.$schema || aggregate.$schema !== DRAFT7) problems.push(`${rel}/🔣️.json: dialect ${aggregate.$schema}`);

  const ajv = new Ajv({ allErrors: true, strict: false });
  let ok = true;
  const keyed = { ...aggregate, $id: undefined, oneOf: [] };
  delete keyed.$id;
  const collectExternal = (node, into) => {
    if (Array.isArray(node)) { for (const item of node) collectExternal(item, into); return; }
    if (node && typeof node === "object") {
      for (const [key, value] of Object.entries(node)) {
        if (key === "$ref" && typeof value === "string" && value.startsWith("http")) into.add(value.split("#")[0]);
        else collectExternal(value, into);
      }
    }
  };
  const wanted = new Set();
  targets.forEach(({ target }) => collectExternal(JSON.parse(readFileSync(target, "utf8")), wanted));
  for (const id of wanted) {
    if (!external.has(id)) { problems.push(`${rel}/🔣️.json: cross-scope $ref ${id} has no module owner`); ok = false; continue; }
    try { ajv.addSchema({ ...external.get(id), $schema: DRAFT7 }, id); } catch (error) { problems.push(`${rel}/🔣️.json: cannot register external ${id} (${error.message})`); ok = false; }
  }
  targets.forEach(({ target }, index) => {
    const key = `urn:wp4:leaf:${index}`;
    const leaf = JSON.parse(readFileSync(target, "utf8"));
    delete leaf.$id;
    try { ajv.addSchema(leaf, key); keyed.oneOf.push({ $ref: key }); } catch (error) { problems.push(`${rel}/🔣️.json: cannot register ${relative(REPO, target)} (${error.message})`); ok = false; }
  });
  if (!ok) continue;
  try { ajv.compile(keyed); compiled += 1; } catch (error) { problems.push(`${rel}/🔣️.json: ajv compile failed (${error.message})`); }
}

const plugins = problems.filter((problem) => problem.startsWith("✏️s/"));
const deferred = problems.filter((problem) => !problem.startsWith("✏️s/"));
console.log(`roots=${roots.length} leaves=${leafCount} aggregates=${aggregateCount} ajvCompiled=${compiled}`);
console.log(`WP4 partition (✏️s/🔌️plugins, non-stdio) problems=${plugins.length}`);
for (const problem of plugins.slice(0, 60)) console.log(`  ${problem}`);
console.log(`framework/os mutation trees (deferred, see 📓️wp4-mutations.md §5) problems=${deferred.length}`);
for (const problem of deferred.slice(0, 8)) console.log(`  ${problem}`);
process.exit(plugins.length === 0 ? 0 : 1);
