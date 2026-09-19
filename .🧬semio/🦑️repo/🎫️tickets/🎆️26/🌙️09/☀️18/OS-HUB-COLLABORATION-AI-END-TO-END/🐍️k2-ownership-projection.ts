/** 🔎️ Replays `📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts`'s third test for EVERY owner
 * (the real gate short-circuits on the first rejected `expect` inside `Promise.all`) and prints, per
 * owner, which of `root` / `test.root` / `test.name` / `projectionSha256` disagrees, plus the portable
 * projection of the first hash mismatch so the machine-dependent field is visible. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { loadConfigFromFile } from "vite";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const fixturePath = resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { owners: { configurationRoot: string; ownerPath: string; expectedName: string | null; projectionSha256: string }[] };

function portable(value: unknown): unknown {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}

function projection(config: Record<string, any>, expectedRoot: string): unknown {
  return {
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
}

const write = process.argv.includes("--write");
let rootBad = 0;
let nameBad = 0;
let hashBad = 0;
let machineBad = 0;
let shown = 0;
for (const owner of fixture.owners) {
  const ownerPath = resolve(repoRoot, owner.ownerPath);
  const expectedRoot = resolve(repoRoot, owner.configurationRoot);
  const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, ownerPath, expectedRoot, "silent", undefined, "native");
  if (!loaded) {
    console.log(`UNLOADABLE ${owner.ownerPath}`);
    continue;
  }
  const config = loaded.config as Record<string, any>;
  if (config.root !== expectedRoot || config.test?.root !== expectedRoot) {
    rootBad += 1;
    console.log(`ROOT ${owner.ownerPath}\n  expected ${expectedRoot}\n  config   ${config.root}\n  test     ${config.test?.root}`);
  }
  if ((config.test?.name ?? null) !== owner.expectedName) {
    nameBad += 1;
    console.log(`NAME ${owner.ownerPath}\n  expected ${owner.expectedName}\n  actual   ${config.test?.name ?? null}`);
  }
  const shape = projection(config, owner.configurationRoot);
  const serialized = JSON.stringify(portable(shape));
  if (serialized.includes(repoRoot) || /"\/(Users|home|private|tmp|var)\//u.test(serialized)) {
    machineBad += 1;
    console.log(`MACHINE ${owner.ownerPath}\n  ${serialized}`);
  }
  const actual = createHash("sha256").update(serialized).digest("hex");
  if (actual !== owner.projectionSha256) {
    hashBad += 1;
    console.log(`HASH ${owner.ownerPath}\n  fixture ${owner.projectionSha256}\n  actual  ${actual}`);
    if (shown < 3) {
      shown += 1;
      console.log(`  projection ${JSON.stringify(portable(shape))}`);
    }
    owner.projectionSha256 = actual;
  }
}
if (write && hashBad > 0) writeFileSyncSafe();
function writeFileSyncSafe(): void {
  const { writeFileSync } = require("node:fs");
  writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
  console.log(`WROTE ${hashBad} row(s)`);
}
console.log(`owners=${fixture.owners.length} rootBad=${rootBad} nameBad=${nameBad} hashBad=${hashBad} machineBad=${machineBad}`);
