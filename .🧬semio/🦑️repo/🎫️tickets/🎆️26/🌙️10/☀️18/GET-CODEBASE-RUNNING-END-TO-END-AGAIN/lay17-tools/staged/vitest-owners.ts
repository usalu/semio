/**
 * 🎚️ Updates the library's Vitest-configuration ownership fixture and gate after lay17 moved plugin renderer-contract tests into the composition suite
 * (removes the three retired owners, adds the composition owner, recomputes `projectionSha256` of every owner whose include list changed, fixes the owner count).
 *
 *   bun vitest-owners.ts            dry run (prints the plan)
 *   bun vitest-owners.ts --write    writes the fixture and the gate (library files: only after the HARD FREEZE / pause)
 *   SEMIO_ROOT=<dir> ...            run against a scratch mirror
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = process.env.SEMIO_ROOT ? resolve(process.env.SEMIO_ROOT) : resolve(import.meta.dir, "../../../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const fixturePath = resolve(repoRoot, library, "🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const gatePath = resolve(repoRoot, library, "🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const retired = new Set([
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️renderer-contract/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🧪️tests/🎚️renderer-contract/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/📺️renderer/🟦️.ts",
]);
const added = { configurationRoot: "✏️s/🧪️tests/🏘️plugin-composition", previousPath: "✏️s/🧪️tests/🏘️plugin-composition/vitest.config.ts", ownerPath: "✏️s/🧪️tests/🏘️plugin-composition/🎚️renderer-contract/🟦️.ts", expectedName: "plugin-composition-renderer-contract", projectionSha256: "" };
const recompute = new Set([
  "✏️s/🔌️plugins/🏗️fem/🧪️tests/🎚️config/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts",
  added.ownerPath,
]);

const portable = (value: unknown): unknown => {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${value.slice(repoRoot.length + 1)}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([l], [r]) => l.localeCompare(r)).map(([key, child]) => [key, portable(child)]));
  return value;
};
const projection = (config: Record<string, any>, root: string): Record<string, unknown> => ({
  aliases: config.resolve?.alias ?? null, cacheDir: config.cacheDir ?? null, coverageInclude: config.test?.coverage?.include ?? null, root, exclude: config.test?.exclude ?? null,
  include: config.test?.include ?? null, includeSource: config.test?.includeSource ?? null, name: config.test?.name ?? null, passWithNoTests: config.test?.passWithNoTests ?? null,
  projects: config.test?.projects ?? null, setupFiles: config.test?.setupFiles ?? null,
});
const hash = (config: Record<string, any>, root: string): string => createHash("sha256").update(JSON.stringify(portable(projection(config, root)))).digest("hex");
const policy = fixture.gatePolicy;
async function load(ownerPath: string, cwd: string): Promise<Record<string, any>> {
  process.env.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd, toolPath: resolve(repoRoot, policy.toolPath), runtime: policy.runtime, coverageRuntime: policy.coverageRuntime, cacheRoot: resolve(repoRoot, policy.cacheRoot), coverageDirectory: resolve(repoRoot, policy.coverageDirectory), budgetMs: policy.budgetMs });
  const module = (await import(pathToFileURL(resolve(repoRoot, ownerPath)).href)) as { default?: unknown };
  const exported = module.default;
  return (typeof exported === "function" ? await (exported as (e: unknown) => unknown)({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported) as Record<string, any>;
}

const owners = fixture.owners.filter((o: any) => !retired.has(o.ownerPath));
if (!owners.some((o: any) => o.ownerPath === added.ownerPath)) owners.push(added);
for (const o of owners) {
  if (!recompute.has(o.ownerPath)) continue;
  const root = resolve(repoRoot, o.configurationRoot);
  const config = await load(o.ownerPath, root);
  const sha = hash(config, o.configurationRoot);
  console.log(`${o.ownerPath.slice(-60)}: ${o.projectionSha256 === sha ? "unchanged" : `${o.projectionSha256.slice(0, 12) || "new"} -> ${sha.slice(0, 12)}`}`);
  o.projectionSha256 = sha;
}
fixture.owners = owners;
const gate = readFileSync(gatePath, "utf8");
const count = String(owners.length);
const nextGate = gate.replace(/toHaveLength\(55\)/u, `toHaveLength(${count})`).replace(/\)\.size\)\.toBe\(55\)/u, `).size).toBe(${count})`);
console.log(`owners: ${fixture.owners.length}; gate count ${nextGate === gate ? "unchanged (already updated?)" : "updated to " + count}`);
if (process.argv.includes("--write")) {
  writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
  writeFileSync(gatePath, nextGate);
  console.log("written");
} else console.log("dry run: nothing written");
