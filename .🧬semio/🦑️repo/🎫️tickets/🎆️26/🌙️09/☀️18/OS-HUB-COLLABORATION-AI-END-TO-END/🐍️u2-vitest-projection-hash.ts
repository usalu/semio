/** 🔁️ Recomputes the `projectionSha256` of every owner in
 * `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`, exactly as
 * `📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts`'s `projectionHash` does, and reports which
 * rows drifted. Pass `--write` to update the fixture. */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { loadConfigFromFile } from "vite";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { owners: { configurationRoot: string; ownerPath: string; projectionSha256: string }[] };

function portable(value: unknown): unknown {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}

function projectionHash(config: Record<string, any>, expectedRoot: string): string {
  const projection = {
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
  };
  return createHash("sha256").update(JSON.stringify(portable(projection))).digest("hex");
}

const write = process.argv.includes("--write");
let drifted = 0;
for (const owner of fixture.owners) {
  const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, resolve(repoRoot, owner.ownerPath), resolve(repoRoot, owner.configurationRoot), "silent", undefined, "native");
  if (!loaded) {
    console.log(`UNLOADABLE ${owner.ownerPath}`);
    continue;
  }
  const actual = projectionHash(loaded.config as Record<string, any>, resolve(repoRoot, owner.configurationRoot));
  if (actual !== owner.projectionSha256) {
    drifted += 1;
    console.log(`DRIFT ${owner.ownerPath}\n  fixture ${owner.projectionSha256}\n  actual  ${actual}`);
    owner.projectionSha256 = actual;
  }
}
if (write && drifted > 0) {
  writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
  console.log(`WROTE ${drifted} row(s)`);
}
console.log(`drifted=${drifted} owners=${fixture.owners.length}`);
