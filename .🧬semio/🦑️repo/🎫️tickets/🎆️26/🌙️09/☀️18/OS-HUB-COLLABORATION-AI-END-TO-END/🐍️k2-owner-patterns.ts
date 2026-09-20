/** 🔭️ Prints every vitest owner's resolved `test.root` and its `include`/`includeSource` patterns,
 * plus how many files on disk each pattern actually matches.
 *
 * The owner module is imported in a child `bun` process (`defineConfig` is identity, so the default
 * export *is* the resolved user config) with a deadline, so an owner whose module-level code walks the
 * whole repo cannot hang the survey. */
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
  process.stdout.write(JSON.stringify({ root, include: config?.test?.include ?? [], includeSource: config?.test?.includeSource ?? [] }));
  process.exit(0);
}

const DEADLINE_MS = 120_000;
const LANES = 4;
const SKIP = new Set(["node_modules", "🗑️generated", "dist", "target", "🤖️generated", ".git", ".🧬semio", "⚡️cache", "storybook-static", "temp", "🤖️jco"]);
const libraryRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");

const files: string[] = [];
const stack = [repoRoot];
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
    } else files.push(relative(repoRoot, path).replaceAll("\\", "/"));
  }
}
process.stderr.write(`files=${files.length}\n`);

const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as { owners: { ownerPath: string }[] };

async function resolveOwner(ownerPath: string): Promise<{ root: string; include: string[]; includeSource: string[] } | null> {
  const child = Bun.spawn(["bun", import.meta.path, "--owner", ownerPath], { stdout: "pipe", stderr: "pipe", cwd: repoRoot });
  const deadline = setTimeout(() => child.kill(9), DEADLINE_MS);
  const text = await new Response(child.stdout).text();
  await child.exited;
  clearTimeout(deadline);
  if (child.exitCode !== 0) return null;
  try {
    return JSON.parse(text) as { root: string; include: string[]; includeSource: string[] };
  } catch {
    return null;
  }
}

for (let lane = 0; lane < fixture.owners.length; lane += LANES) {
  const wave = fixture.owners.slice(lane, lane + LANES);
  const loaded = await Promise.all(wave.map(async (owner) => [owner.ownerPath, await resolveOwner(resolve(repoRoot, owner.ownerPath))] as const));
  for (const [ownerPath, config] of loaded) {
    if (config === null) {
      console.log(`UNRESOLVED ${ownerPath}`);
      continue;
    }
    const absolute = files.map((path) => resolve(repoRoot, path).replaceAll("\\", "/"));
    const rows = [...config.include.map((pattern) => ["include", pattern] as const), ...config.includeSource.map((pattern) => ["includeSource", pattern] as const)];
    const counts = rows.map(([key, pattern]) => {
      const glob = new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(config.root, pattern).replaceAll("\\", "/"));
      return `${key}:${pattern}=${absolute.filter((path) => glob.match(path)).length}`;
    });
    console.log(`OWNER ${ownerPath} root=${relative(repoRoot, config.root) || "."} patterns=${rows.length} ${counts.join(" ")}`);
  }
  process.stderr.write(`· ${lane + wave.length}/${fixture.owners.length}\n`);
}
