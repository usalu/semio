// 🎚️ bd10: rebuilds the vitest ownership fixture: drops removed owners, registers the call-site configs, pins every projection under the gate policy.
// usage: bun vitest-fixture.ts [--write]
import {createHash} from "node:crypto";
import {readFileSync, readdirSync, writeFileSync} from "node:fs";
import {isAbsolute, join, relative, resolve} from "node:path";
import {pathToFileURL} from "node:url";

const repoRoot = "/Users/ueli/Documents/semio";
const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const write = process.argv.includes("--write");
const REMOVED = new Set(["✏️s/🔌️plugins/🌊️flow/🧪️tests/🎚️config/🟦️.ts"]);
const ADDED = [
  "🌎️hub/🧪️testing/🎚️config/🟦️.ts",
  "🌎️hub/🧩️compositions/🗄️stdio/📇️publication/🧪️tests/🎚️config/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️renderer-contract/🟦️.ts",
  "✏️s/🔌️plugins/📏️layout/🧪️tests/🎚️renderer-contract/🟦️.ts",
  "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/📺️renderer/🟦️.ts",
  "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🎚️config/🧵️gis-inference-bridge/🟦️.ts",
  "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/❓️quiz/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/🐾️pets/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts",
  "✏️s/🔌️plugins/🏗️fem/🧪️tests/🎚️config/🟦️.ts",
];
fixture.gatePolicy = {cacheRoot: ".🧬semio/🦑️repo/⚡️cache/vitest", coverageDirectory: ".🧬semio/🦑️repo/⚡️cache/vitest-coverage", toolPath: "node_modules/vitest/vitest.mjs", runtime: "bun", coverageRuntime: "bun", budgetMs: 30000};
fixture.selector.ownerPattern = "(?:^|/)🎚️[^/]+(?:/[^/]+)*/🟦️\\.ts$";
fixture.owners = fixture.owners.filter((owner: any) => !REMOVED.has(owner.ownerPath));
const known = new Set(fixture.owners.map((o: any) => o.ownerPath));
const portable = (value: unknown): unknown => {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return {flags: value.flags, regex: value.source};
  if (typeof value === "function") return {function: (value as Function).name || "anonymous"};
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([l], [r]) => l.localeCompare(r)).map(([key, child]) => [key, portable(child)]));
  return value;
};
const canonical = (value: unknown) => JSON.stringify(portable(value));
const projection = (config: Record<string, any>, root: string) => ({aliases: config.resolve?.alias ?? null, cacheDir: config.cacheDir ?? null, coverageInclude: config.test?.coverage?.include ?? null, root, exclude: config.test?.exclude ?? null, include: config.test?.include ?? null, includeSource: config.test?.includeSource ?? null, name: config.test?.name ?? null, passWithNoTests: config.test?.passWithNoTests ?? null, projects: config.test?.projects ?? null, setupFiles: config.test?.setupFiles ?? null});
const policyFor = (cwd: string) => JSON.stringify({version: 1, cwd, toolPath: resolve(repoRoot, fixture.gatePolicy.toolPath), runtime: "bun", coverageRuntime: "bun", cacheRoot: resolve(repoRoot, fixture.gatePolicy.cacheRoot), coverageDirectory: resolve(repoRoot, fixture.gatePolicy.coverageDirectory), budgetMs: fixture.gatePolicy.budgetMs});
const UNWALKED = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated", "🤖️jco", ".git", ".🧬semio", "⚡️cache", "temp"]);
const files: string[] = [];
const stack = [repoRoot];
while (stack.length) { const d = stack.pop()!; let es; try { es = readdirSync(d, {withFileTypes: true}); } catch { continue; } for (const e of es) { if (e.isSymbolicLink()) continue; const p = join(d, e.name); if (e.isDirectory()) { if (!UNWALKED.has(e.name)) stack.push(p); } else files.push(p); } }
const load = async (ownerPath: string, cwd: string) => {
  process.env.SEMIO_VITEST_POLICY = policyFor(cwd);
  try {
    const mod = (await import(pathToFileURL(resolve(repoRoot, ownerPath)).href)) as {default?: unknown};
    const exported = mod.default;
    return (typeof exported === "function" ? await (exported as any)({command: "serve", mode: "test", isSsrBuild: false, isPreview: false}) : exported) as Record<string, any>;
  } finally { delete process.env.SEMIO_VITEST_POLICY; }
};
for (const ownerPath of ADDED) {
  if (known.has(ownerPath)) continue;
  const config = await load(ownerPath, repoRoot);
  const root = relative(repoRoot, config.test?.root ?? config.root) || ".";
  fixture.owners.push({configurationRoot: root, previousPath: `${root}/vitest.config.ts`, ownerPath, expectedName: config.test?.name ?? null, projectionSha256: ""});
}
const report: string[] = [];
for (const owner of fixture.owners) {
  const expectedRoot = resolve(repoRoot, owner.configurationRoot);
  try {
    const config = await load(owner.ownerPath, expectedRoot);
    const hash = createHash("sha256").update(canonical(projection(config, owner.configurationRoot))).digest("hex");
    const patterns: string[] = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])];
    const empty = patterns.filter((pattern) => { const g = new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(expectedRoot, pattern).replaceAll("\\", "/")); return !files.some((f) => g.match(f)); });
    const rootOk = config.root === expectedRoot && config.test?.root === expectedRoot;
    const nameOk = (config.test?.name ?? null) === owner.expectedName;
    const absolute = /"(?:\/|[A-Za-z]:[\\/])[^"]*"/u.exec(canonical(projection(config, owner.configurationRoot)))?.[0] ?? null;
    const ok = !empty.length && rootOk && nameOk && !absolute && (patterns.length > 0 || owner.configurationRoot === ".");
    if (ok) owner.projectionSha256 = hash;
    else report.push(`PROBLEM ${owner.ownerPath}: empty=${JSON.stringify(empty)} rootOk=${rootOk} nameOk=${nameOk} absolute=${absolute} declared=${patterns.length}`);
    if (ok && hash !== owner.projectionSha256) report.push(`repinned ${owner.ownerPath}`);
  } catch (e) { report.push(`ERROR ${owner.ownerPath}: ${String((e as Error).message).slice(0, 200)}`); }
}
console.log(report.join("\n"));
console.log("owners", fixture.owners.length);
if (write) writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
