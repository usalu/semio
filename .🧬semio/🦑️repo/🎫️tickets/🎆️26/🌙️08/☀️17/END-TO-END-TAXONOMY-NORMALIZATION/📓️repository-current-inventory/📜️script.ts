import { createHash, webcrypto } from "node:crypto";
import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import stringify from "fast-json-stable-stringify";

const ticketRoot = resolve(import.meta.dir, "..");
const repoRoot = resolve(ticketRoot, "../../../../../../..");
const output = "📊️repository-current-inventory-02";
const aggregatePath = "🧾️repository-current-inventory-02-aggregates/🔣️.json";
const byteCompare = (a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b));
const sha = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");

/** 🛡️ Reads only exact ticket-owned nodes after checking every ancestor without following links. */
function checked(path: string, kind: "file" | "directory"): string {
  const absolute = resolve(ticketRoot, path);
  const local = relative(ticketRoot, absolute).replaceAll("\\", "/");
  if (!local || local.startsWith("../") || local !== path) throw new Error("Invalid retained inventory path: " + path);
  const parts = relative(repoRoot, absolute).replaceAll("\\", "/").split("/");
  let cursor = repoRoot;
  for (let index = 0; index < parts.length; index++) {
    cursor = join(cursor, parts[index]!);
    const state = lstatSync(cursor), leaf = index === parts.length - 1;
    if (state.isSymbolicLink() || (leaf && kind === "file" ? !state.isFile() : !state.isDirectory())) throw new Error("Unexpected retained inventory node: " + cursor);
  }
  return absolute;
}

/** 🔬️ Streams independent canonical serialization without allocating a second whole-census string. */
function independentDigest(value: Record<string, unknown>): string {
  const digest = createHash("sha256");
  digest.update("{");
  const keys = Object.keys(value).sort();
  keys.forEach((key, index) => {
    if (index) digest.update(",");
    digest.update(JSON.stringify(key) + ":");
    const field = value[key];
    if (Array.isArray(field)) {
      digest.update("[");
      field.forEach((row, position) => { if (position) digest.update(","); digest.update(stringify(row)); });
      digest.update("]");
    } else digest.update(stringify(field));
  });
  digest.update("}");
  return digest.digest("hex");
}

if (process.argv[2] !== "verify") throw new Error("Usage: 📜️script.ts verify");
const started = performance.now();
const manifestContent = readFileSync(checked(output + "/📊️shards/🔣️.json", "file"), "utf8");
const manifest = JSON.parse(manifestContent);
const aggregateBytes = readFileSync(checked(aggregatePath, "file"));
const aggregate = JSON.parse(aggregateBytes.toString());
const available: string[] = [];
for (const name of readdirSync(checked(output + "/📊️shards", "directory"))) {
  if (name === "🔣️.json") continue;
  if (!/^🔖️[a-f0-9]{64}$/u.test(name)) throw new Error("Unexpected retained inventory shard member: " + name);
  const parent = output + "/📊️shards/" + name;
  for (const child of readdirSync(checked(parent, "directory"))) {
    checked(parent + "/" + child, "file");
    available.push("📊️shards/" + name + "/" + child);
  }
}
const shards: { descriptor: any; content: string }[] = [];
let payloadBytes = 0;
for (const descriptor of manifest.shards) {
  if (!/^📊️shards\/🔖️[a-f0-9]{64}\/🔣️\.json$/u.test(descriptor.path)) throw new Error("Invalid declared retained shard path");
  const content = readFileSync(checked(output + "/" + descriptor.path, "file"), "utf8");
  const independent = Buffer.from(await webcrypto.subtle.digest("SHA-256", Buffer.from(content))).toString("hex");
  if (independent !== descriptor.digest) throw new Error("Independent shard digest mismatch: " + descriptor.path);
  shards.push({ descriptor, content });
  payloadBytes += Buffer.byteLength(content);
  if (shards.length % 500 === 0) console.log("[DEBUG] retained inventory readback", shards.length, manifest.shards.length);
}
const { validateTaxonomyInventoryArtifactShards } = await import(join(repoRoot, "📜️script.ts"));
const result = validateTaxonomyInventoryArtifactShards({ manifest, manifestContent, shards }, available.sort(byteCompare));
const inventory = result.inventory as Record<string, any>;
if (independentDigest(inventory) !== manifest.inventoryCanonicalDigest) throw new Error("Independent reconstructed inventory digest mismatch");
if (inventory.inventoryDigest !== aggregate.inventoryDigest || inventory.sourceTreeDigest !== aggregate.sourceTreeDigest) throw new Error("Aggregate identity differs from publication");
const entries = inventory.entries as any[], violations = inventory.violations as any[];
const count = (predicate: (row: any) => boolean) => entries.filter(predicate).length;
const totals = {
  entries: entries.length,
  directories: count((row) => row.nodeKind === "directory"),
  files: count((row) => row.nodeKind === "file"),
  symlinks: count((row) => row.nodeKind === "symlink"),
  leaves: count((row) => row.nodeKind !== "directory"),
  changingLeaves: count((row) => row.nodeKind !== "directory" && row.sourcePath !== row.normalizedPath),
  changingDirectories: count((row) => row.nodeKind === "directory" && row.sourcePath !== row.normalizedPath),
  violatingEntries: count((row) => row.violations.length > 0),
  violatingLeaves: count((row) => row.nodeKind !== "directory" && row.violations.length > 0),
  errors: violations.filter((row) => row.severity === "error").length,
  warnings: violations.filter((row) => row.severity === "warning").length,
  violations: violations.length,
};
if (stringify(totals) !== stringify(aggregate.totals)) throw new Error("Independent aggregate totals mismatch: " + JSON.stringify(totals));
for (const row of aggregate.codes) {
  const matching = violations.filter((entry) => entry.code === row.code);
  if (matching.length !== row.count || matching.filter((entry) => entry.severity === "error").length !== row.errors || matching.filter((entry) => entry.severity === "warning").length !== row.warnings) throw new Error("Aggregate violation code mismatch: " + row.code);
}
if (new Set(violations.map((row) => row.code)).size !== aggregate.codes.length) throw new Error("Aggregate violation codes are incomplete");
if (entries.some((row) => row.sourcePath === "compose" || row.sourcePath.startsWith("compose/") || row.sourcePath === "temp/compose" || row.sourcePath.startsWith("temp/compose/"))) throw new Error("Opaque path admitted to retained inventory");
console.log(JSON.stringify({ event: "retained-inventory-readback-verified", milliseconds: performance.now() - started, manifestBytes: Buffer.byteLength(manifestContent), manifestSha256: sha(manifestContent), aggregateBytes: aggregateBytes.length, aggregateSha256: sha(aggregateBytes), shards: shards.length, payloadBytes, inventoryCanonicalDigest: manifest.inventoryCanonicalDigest, totals, violationCodes: aggregate.codes.length, owners: aggregate.owners.length, physicalSourcesRevisited: false }));
