/** 🧾️ Recomputes the `projectionSha256` of vitest-configuration-ownership owners with the gate's own projection logic: `bun ownership-hash.mjs <phase> [--write]`.
 * Phase `before` loads the pre-edit text from the snapshot through a temporary sibling copy (deleted at once); phase `after` loads the real file. `--write` stores the `after` hash into the fixture for owners whose `before` hash equalled the fixture. */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, existsSync, rmSync, mkdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = "/Users/ueli/Documents/semio";
const F = process.env.F;
const G = process.env.G;
const [phase, flag] = process.argv.slice(2);
const fixturePath = `${repoRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`;
const fixtureText = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(fixtureText);
const snapshotIndex = JSON.parse(readFileSync(`${G}/ass-snap/index.json`, "utf8"));

function portable(value) {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}
const canonical = (value) => JSON.stringify(portable(value));
const projection = (config, expectedRoot) => ({
  aliases: config.resolve?.alias ?? null,
  cacheDir: config.cacheDir ?? null,
  coverageInclude: config.test?.coverage?.include ?? null,
  root: expectedRoot,
  exclude: config.test?.exclude ?? null,
  include: config.test?.include ?? null,
  includeSource: config.test?.includeSource ?? null,
  name: config.test?.name ?? null,
  passWithNoTests: config.test?.passWithNoTests ?? null,
  projects: config.test?.projects ?? null,
  setupFiles: config.test?.setupFiles ?? null,
});
const hashOf = (config, expectedRoot) => createHash("sha256").update(canonical(projection(config, expectedRoot))).digest("hex");

async function load(ownerPath, cwd, text) {
  process.env.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd, toolPath: `${repoRoot}/node_modules/vitest/vitest.mjs`, runtime: "/Users/ueli/.bun/bin/bun", coverageRuntime: "/Users/ueli/.bun/bin/bun", cacheRoot: `${G}/ass-vitest-cache`, coverageDirectory: `${G}/ass-vitest-cov`, budgetMs: 1500000 });
  process.env.SEMIO_TEST_LEVEL = "long";
  let path = resolve(repoRoot, ownerPath);
  let temporary;
  if (text !== undefined) {
    temporary = join(dirname(path), "🟦️.ass-before.ts");
    writeFileSync(temporary, text);
    path = temporary;
  }
  try {
    const mod = await import(pathToFileURL(path).href + (temporary ? `?t=${Date.now()}` : ""));
    const exported = mod.default;
    return typeof exported === "function" ? await exported({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported;
  } finally {
    if (temporary) rmSync(temporary, { force: true });
  }
}

const rows = [];
let updated = fixtureText;
for (const owner of fixture.owners) {
  const touched = owner.ownerPath in snapshotIndex;
  const cwd = resolve(repoRoot, owner.configurationRoot);
  let hash, error;
  try {
    const text = phase === "before" && touched ? readFileSync(`${G}/ass-snap/${snapshotIndex[owner.ownerPath]}`, "utf8") : undefined;
    const config = await load(owner.ownerPath, cwd, text);
    hash = hashOf(config, owner.configurationRoot);
  } catch (e) {
    error = String(e).slice(0, 140);
  }
  rows.push({ owner: owner.ownerPath, touched, fixtureMatches: hash === owner.projectionSha256, hash, error });
  if (flag === "--write" && phase === "after" && touched && hash && hash !== owner.projectionSha256) {
    const before = JSON.parse(readFileSync(`${G}/ass-ownership-before.json`, "utf8")).find((r) => r.owner === owner.ownerPath);
    if (before?.fixtureMatches) {
      if (updated.split(owner.projectionSha256).length !== 2) throw new Error("fixture hash not unique: " + owner.ownerPath);
      updated = updated.replace(owner.projectionSha256, hash);
      rows[rows.length - 1].written = true;
    }
  }
}
writeFileSync(`${G}/ass-ownership-${phase}.json`, JSON.stringify(rows, null, 1));
if (flag === "--write" && updated !== fixtureText) {
  if (readFileSync(fixturePath, "utf8") !== fixtureText) throw new Error("fixture changed during run");
  writeFileSync(fixturePath, updated);
}
const touchedRows = rows.filter((r) => r.touched);
console.log(`phase=${phase} owners=${rows.length} touched=${touchedRows.length} fixtureMatchesAmongTouched=${touchedRows.filter((r) => r.fixtureMatches).length} errors=${rows.filter((r) => r.error).length} written=${rows.filter((r) => r.written).length}`);
for (const r of rows) console.log(r.touched ? "T" : " ", r.fixtureMatches ? "match   " : r.error ? "ERROR   " : "mismatch", r.owner.slice(-70), r.error ?? "");
