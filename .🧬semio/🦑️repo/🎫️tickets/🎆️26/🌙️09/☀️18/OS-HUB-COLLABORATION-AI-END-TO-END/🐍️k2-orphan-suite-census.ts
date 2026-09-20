/** 🕳️ Census of test suites no runner reaches.
 *
 * Candidates: every `*.test.*`, every `🧪️tests/**` `🟦️.ts(x)` and every file carrying an
 * `import.meta.vitest` block anywhere in the repo.
 * Reachability, in the two ways this repo runs tests:
 *  1. vitest — each owner in `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` is
 *     imported directly (`defineConfig` is identity, so the module's default export *is* the resolved
 *     user config) and its `include`/`includeSource` globs are resolved against that owner's
 *     `test.root` and matched **absolute** — exactly what vitest collects. Matching them relative to
 *     the root discards every pattern that climbs above it (`../../📥️cold-pair/🟦️.ts`,
 *     `../../../../🧱️elements/…/🧩️component/🟦️.tsx`), which is most of the `🖱️ui` and `🎭️actor`
 *     owners, and reports their suites as orphans. Loading through vite's `loadConfigFromFile` instead
 *     costs ≈20 s per owner (esbuild bundle per config) and does not change any glob.
 *  2. `bun test` — a candidate is reached when its repo-relative path appears literally in any
 *     `📜️script.ts`, `📋️project.json` or fixture `🔣️.json` (the `runTestBudgeted(…)` / registration
 *     `source` idiom), or when a `🧪️tests/<name>` directory it lives in is named there.
 *  3. case discovery — `🧪️test`'s `discoverTestCases()` generates one cacheable Nx project per
 *     `**\/🧪️tests/<case>/` directory holding a feature file, and runs each adapter beside it through
 *     its own native host. That is a whole runner the vitest globs and the `bun test` registrations know
 *     nothing about; without it the entire `📓️print` visualization gallery (105 suites) reads as orphaned.
 *  4. transitively — a `🧪️tests/<name>/🟦️.ts` module exporting `registerTestsN` is never collected by
 *     a glob; it is `await import(…)`ed from the `import.meta.vitest` block of the module under test.
 *     Reachability therefore closes over relative-import edges from every file reached by (1) or (2).
 *     Without this closure the census reports every registered suite as an orphan (448 vs 89).
 *
 * `--json` appends the machine-readable census. */
import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve, isAbsolute, dirname } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");

if (process.argv[2] === "--owner") {
  const ownerPath = process.argv[3]!;
  const module = (await import(pathToFileURL(ownerPath).href)) as { default?: unknown };
  const exported = module.default;
  const config = (typeof exported === "function" ? await (exported as (env: unknown) => unknown)({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported) as Record<string, any>;
  const declaredRoot = config?.test?.root ?? config?.root;
  const root = typeof declaredRoot === "string" ? (isAbsolute(declaredRoot) ? declaredRoot : resolve(dirname(ownerPath), declaredRoot)) : dirname(ownerPath);
  process.stdout.write(JSON.stringify({ root, patterns: [...(config?.test?.include ?? []), ...(config?.test?.includeSource ?? [])] }));
  process.exit(0);
}

const OWNER_LOAD_DEADLINE_MS = 240_000;
const OWNER_LOAD_LANES = 4;
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const SKIP = new Set(["node_modules", "🗑️generated", "dist", "target", "🤖️generated", ".git", ".🧬semio", "⚡️cache", "storybook-static", "temp", "🤖️jco"]);

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

type Candidate = { path: string; kind: "named" | "semantic" | "in-source"; inSource: boolean; imports: string[] };
const candidates = new Map<string, Candidate>();
const RELATIVE_SPECIFIER = /(?:from\s*|import\s*\(\s*)["'](\.\.?\/[^"']+)["']/gu;
const runnerSources: string[] = [];
const discoveredCaseDirectories = new Set<string>();

walk(repoRoot, (path) => {
  const relativePath = relative(repoRoot, path).replaceAll("\\", "/");
  if (relativePath.endsWith(".feature") && /\/🧪️tests\/[^/]+\/[^/]+$/u.test(relativePath)) discoveredCaseDirectories.add(dirname(relativePath));
  const registrationSource = relativePath.endsWith("📜️script.ts") || relativePath.endsWith("📋️project.json") || relativePath.endsWith("🔣️.json") || relativePath.endsWith("package.json");
  let text = "";
  if (registrationSource || relativePath.endsWith("🟦️.ts") || relativePath.endsWith("🟦️.tsx")) {
    try {
      text = readFileSync(path, "utf8");
    } catch {
      /* unreadable file is neither runner nor candidate */
    }
  }
  if (registrationSource || text.includes("extends BundleScript")) runnerSources.push(text);
  if (!/\.(ts|tsx|mts|cts)$/u.test(relativePath)) return;
  const named = /(^|\/)[^/]*\.test\.[^/]+$/u.test(relativePath);
  const semantic = /\/🧪️tests\/.*\/🟦️\.tsx?$/u.test(relativePath);
  if (text === "") {
    try {
      text = readFileSync(path, "utf8");
    } catch {
      /* unreadable candidate stays a candidate */
    }
  }
  const inSource = text.includes("import.meta.vitest");
  if (!named && !semantic && !inSource) return;
  if (relativePath.endsWith("/🧪️tests/🎚️config/🟦️.ts")) return;
  const imports = [...text.matchAll(RELATIVE_SPECIFIER)].map((match) => relative(repoRoot, resolve(dirname(path), match[1]!)).replaceAll("\\", "/"));
  candidates.set(relativePath, { path: relativePath, kind: named ? "named" : semantic ? "semantic" : "in-source", inSource, imports });
});

const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as { owners: { configurationRoot: string; ownerPath: string }[] };
const reachedByVitest = new Map<string, string>();
const unloadable: string[] = [];
const emptyOwners: string[] = [];

async function resolveOwner(ownerPath: string): Promise<{ root: string; patterns: string[] } | null> {
  const child = Bun.spawn(["bun", import.meta.path, "--owner", ownerPath], { stdout: "pipe", stderr: "pipe", cwd: repoRoot });
  const deadline = setTimeout(() => child.kill(9), OWNER_LOAD_DEADLINE_MS);
  const text = await new Response(child.stdout).text();
  await child.exited;
  clearTimeout(deadline);
  if (child.exitCode !== 0) return null;
  try {
    return JSON.parse(text) as { root: string; patterns: string[] };
  } catch {
    return null;
  }
}

const resolved = new Map<string, { root: string; patterns: string[] } | null>();
for (let lane = 0; lane < fixture.owners.length; lane += OWNER_LOAD_LANES) {
  const wave = fixture.owners.slice(lane, lane + OWNER_LOAD_LANES);
  const loaded = await Promise.all(wave.map(async (owner) => [owner.ownerPath, await resolveOwner(resolve(repoRoot, owner.ownerPath))] as const));
  for (const [ownerPath, config] of loaded) {
    resolved.set(ownerPath, config);
    process.stderr.write(`· ${ownerPath} ${config === null ? "UNRESOLVED" : `patterns=${config.patterns.length}`}\n`);
  }
}

for (const owner of fixture.owners) {
  const config = resolved.get(owner.ownerPath) ?? null;
  if (config === null) {
    unloadable.push(owner.ownerPath);
    continue;
  }
  if (config.patterns.length === 0) emptyOwners.push(owner.ownerPath);
  const globs = config.patterns.map((pattern) => new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(config.root, pattern).replaceAll("\\", "/")));
  for (const candidate of candidates.keys()) {
    const absolute = resolve(repoRoot, candidate).replaceAll("\\", "/");
    if (!globs.some((glob) => glob.match(absolute))) continue;
    if (!reachedByVitest.has(candidate)) reachedByVitest.set(candidate, owner.ownerPath);
  }
}

const runnerText = runnerSources.join("\n");
function reachedByBunTest(path: string): boolean {
  if (runnerText.includes(path)) return true;
  const suite = path.match(/\/🧪️tests\/([^/]+)\//u)?.[1];
  return suite !== undefined && runnerText.includes(`🧪️tests/${suite}/`);
}

function reachedByCaseDiscovery(path: string): boolean {
  return discoveredCaseDirectories.has(dirname(path));
}

const reached = new Set<string>();
const frontier: string[] = [];
for (const candidate of candidates.keys()) {
  if (reachedByVitest.has(candidate) || reachedByBunTest(candidate) || reachedByCaseDiscovery(candidate)) {
    reached.add(candidate);
    frontier.push(candidate);
  }
}
const directlyReached = reached.size;
while (frontier.length > 0) {
  const current = candidates.get(frontier.pop()!);
  if (current === undefined) continue;
  for (const imported of current.imports) {
    if (!candidates.has(imported) || reached.has(imported)) continue;
    reached.add(imported);
    frontier.push(imported);
  }
}

const orphans: Candidate[] = [];
for (const candidate of candidates.values()) if (!reached.has(candidate.path)) orphans.push(candidate);
orphans.sort((left, right) => left.path.localeCompare(right.path));

for (const orphan of orphans) console.log(`ORPHAN [${orphan.kind}${orphan.inSource ? "+in-source" : ""}] ${orphan.path}`);
if (unloadable.length > 0) console.log(`UNLOADABLE OWNERS (${unloadable.length}):\n  ${unloadable.join("\n  ")}`);
if (emptyOwners.length > 0) console.log(`ZERO-PATTERN OWNERS (${emptyOwners.length}):\n  ${emptyOwners.join("\n  ")}`);
console.log(`candidates=${candidates.size} vitest=${reachedByVitest.size} direct=${directlyReached} transitive=${reached.size - directlyReached} orphans=${orphans.length} unloadable=${unloadable.length}`);
if (process.argv.includes("--json")) console.log(JSON.stringify(orphans, null, 2));
