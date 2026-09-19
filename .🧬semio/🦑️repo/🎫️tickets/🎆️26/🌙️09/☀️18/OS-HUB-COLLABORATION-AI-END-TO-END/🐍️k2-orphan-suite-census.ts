/** 🕳️ Census of test suites no runner reaches.
 *
 * Candidates: every `*.test.*`, every `🧪️tests/**` `🟦️.ts(x)` and every file carrying an
 * `import.meta.vitest` block under `🧰️framework` and `✏️s`.
 * Reachability, in the two ways this repo runs tests:
 *  1. vitest — each owner in `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` is loaded
 *     through installed Vite and its `include`/`includeSource` globs are matched against the candidate,
 *     relative to that owner's `test.root` (this is exactly what vitest collects).
 *  2. `bun test` — a candidate is reached when its repo-relative path appears literally in any
 *     `📜️script.ts`, `📋️project.json` or fixture `🔣️.json` (the `runTestBudgeted(…)` / registration
 *     `source` idiom), or when a `🧪️tests/<name>` directory it lives in is named there.
 *
 * `--json` writes the machine-readable census next to this script's output. */
import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { loadConfigFromFile } from "vite";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const SKIP = new Set(["node_modules", "🗑️generated", "dist", "target", "🤖️generated", ".git"]);

function walk(root: string, visit: (path: string) => void): void {
  const stack = [root];
  while (stack.length > 0) {
    const directory = stack.pop()!;
    let entries;
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      continue;
    }
    for (const entry of entries) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) {
        if (!SKIP.has(entry.name)) stack.push(path);
      } else visit(path);
    }
  }
}

const productRoots = ["🧰️framework", "✏️s"].map((path) => resolve(repoRoot, path));

type Candidate = { path: string; kind: "named" | "semantic" | "in-source"; inSource: boolean };
const candidates = new Map<string, Candidate>();
const runnerSources: string[] = [];
runnerSources.push(readFileSync(resolve(repoRoot, "📜️script.ts"), "utf8"));

for (const root of productRoots) {
  walk(root, (path) => {
    const relativePath = relative(repoRoot, path).replaceAll("\\", "/");
    if (relativePath.endsWith("/📜️script.ts") || relativePath.endsWith("/📋️project.json") || relativePath.endsWith("/🔣️.json") || /\/🏃️execution\/🟦️\.ts$/u.test(relativePath)) {
      try {
        runnerSources.push(readFileSync(path, "utf8"));
      } catch {
        /* unreadable source is not a runner */
      }
    }
    if (!/\.(ts|tsx|mts|cts)$/u.test(relativePath)) return;
    const named = /(^|\/)[^/]*\.test\.[^/]+$/u.test(relativePath);
    const semantic = /\/🧪️tests\/.*\/🟦️\.tsx?$/u.test(relativePath);
    let inSource = false;
    try {
      inSource = readFileSync(path, "utf8").includes("import.meta.vitest");
    } catch {
      /* unreadable candidate stays a candidate */
    }
    if (!named && !semantic && !inSource) return;
    if (relativePath.endsWith("/🧪️tests/🎚️config/🟦️.ts")) return;
    candidates.set(relativePath, { path: relativePath, kind: named ? "named" : semantic ? "semantic" : "in-source", inSource });
  });
}

const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as { owners: { configurationRoot: string; ownerPath: string }[] };
const reachedByVitest = new Map<string, string>();
const unloadable: string[] = [];

for (const owner of fixture.owners) {
  const ownerPath = resolve(repoRoot, owner.ownerPath);
  const configurationRoot = resolve(repoRoot, owner.configurationRoot);
  let loaded;
  try {
    loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, ownerPath, configurationRoot, "silent", undefined, "native");
  } catch {
    loaded = null;
  }
  if (!loaded) {
    unloadable.push(owner.ownerPath);
    continue;
  }
  const config = loaded.config as Record<string, any>;
  const root = typeof config.test?.root === "string" ? config.test.root : configurationRoot;
  const patterns = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])] as string[];
  const globs = patterns.map((pattern) => new Bun.Glob(pattern));
  for (const candidate of candidates.keys()) {
    if (reachedByVitest.has(candidate)) continue;
    const local = relative(root, resolve(repoRoot, candidate)).replaceAll("\\", "/");
    if (local.startsWith("..")) continue;
    if (globs.some((glob) => glob.match(local))) reachedByVitest.set(candidate, owner.ownerPath);
  }
}

const runnerText = runnerSources.join("\n");
function reachedByBunTest(path: string): boolean {
  if (runnerText.includes(path)) return true;
  const suite = path.match(/\/🧪️tests\/([^/]+)\//u)?.[1];
  return suite !== undefined && runnerText.includes(`🧪️tests/${suite}/`);
}

const orphans: Candidate[] = [];
for (const candidate of candidates.values()) {
  if (reachedByVitest.has(candidate.path)) continue;
  if (reachedByBunTest(candidate.path)) continue;
  orphans.push(candidate);
}
orphans.sort((left, right) => left.path.localeCompare(right.path));

for (const orphan of orphans) console.log(`ORPHAN [${orphan.kind}${orphan.inSource ? "+in-source" : ""}] ${orphan.path}`);
if (unloadable.length > 0) console.log(`UNLOADABLE OWNERS: ${unloadable.join(", ")}`);
console.log(`candidates=${candidates.size} vitest=${reachedByVitest.size} orphans=${orphans.length}`);
if (process.argv.includes("--json")) console.log(JSON.stringify(orphans, null, 2));
