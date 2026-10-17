// 🎚️ bd9: recomputes every vitest owner's behavioral projection hash + emptiness exactly like the ownership gate; prints a JSON table.
// usage: bun vitest-owners.ts [--write]   (--write re-pins projectionSha256 for owners whose patterns all select files)
import {createHash} from "node:crypto";
import {readFileSync, readdirSync, writeFileSync} from "node:fs";
import {isAbsolute, join, relative, resolve} from "node:path";
import {pathToFileURL} from "node:url";

const repoRoot = "/Users/ueli/Documents/semio";
const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const write = process.argv.includes("--write");
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
const UNWALKED = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated", "🤖️jco", ".git", ".🧬semio", "⚡️cache", "temp"]);
const files: string[] = [];
const stack = [repoRoot];
while (stack.length) {
  const d = stack.pop()!;
  let entries; try { entries = readdirSync(d, {withFileTypes: true}); } catch { continue; }
  for (const e of entries) { if (e.isSymbolicLink()) continue; const p = join(d, e.name); if (e.isDirectory()) { if (!UNWALKED.has(e.name)) stack.push(p); } else files.push(p); }
}
const rows: any[] = [];
for (const owner of fixture.owners) {
  const ownerPath = resolve(repoRoot, owner.ownerPath);
  const expectedRoot = resolve(repoRoot, owner.configurationRoot);
  let row: any = {ownerPath: owner.ownerPath, pinned: owner.projectionSha256};
  try {
    const mod = (await import(pathToFileURL(ownerPath).href)) as {default?: unknown};
    const exported = mod.default;
    const config = (typeof exported === "function" ? await (exported as any)({command: "serve", mode: "test", isSsrBuild: false, isPreview: false}) : exported) as Record<string, any>;
    const hash = createHash("sha256").update(canonical(projection(config, owner.configurationRoot))).digest("hex");
    const patterns: string[] = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])];
    const empty = patterns.filter((pattern) => { const g = new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(expectedRoot, pattern).replaceAll("\\", "/")); return !files.some((f) => g.match(f)); });
    Object.assign(row, {hash, same: hash === owner.projectionSha256, declared: patterns.length, empty, rootOk: config.root === expectedRoot && config.test?.root === expectedRoot, nameOk: (config.test?.name ?? null) === owner.expectedName});
  } catch (e) { row.error = String((e as Error).message).slice(0, 160); }
  rows.push(row);
}
console.log(JSON.stringify(rows.map((r) => ({p: r.ownerPath, same: r.same, declared: r.declared, empty: r.empty?.length, rootOk: r.rootOk, nameOk: r.nameOk, error: r.error})), null, 0).replaceAll("},", "},\n"));
if (write) {
  let changed = 0;
  for (const owner of fixture.owners) {
    const r = rows.find((x) => x.ownerPath === owner.ownerPath);
    if (r?.hash && !r.same && r.empty.length === 0 && r.declared > 0 && r.rootOk && r.nameOk) { owner.projectionSha256 = r.hash; changed++; }
  }
  writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
  console.log("repinned", changed);
}
