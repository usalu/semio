import { createHash } from "node:crypto";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { loadCatalogTaxonomy, semanticDirectoryKindId } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

type Owner = Readonly<{ configurationRoot: string; previousPath: string; ownerPath: string; expectedName: string | null; projectionSha256: string }>;
type Fixture = Readonly<{
  schemaVersion: 1;
  owners: readonly Owner[];
  selector: Readonly<{ libraryDeclaration: string; relativeOwnerSuffix: string; editorSetting: string; editorPattern: string }>;
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; kind: string }>[];
  registration: Readonly<{ name: string; command: string; target: string }>;
}>;

const libraryRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const repoRoot = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🎚️vitest-configuration-ownership/🔣️.json"), "utf8"));

function portable(value: unknown): unknown {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}

function canonical(value: unknown): string {
  return JSON.stringify(portable(value));
}

function behavioralProjection(config: Record<string, any>, expectedRoot: string): Record<string, unknown> {
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

/** 🌍️ A quoted value that opens with a POSIX root or a Windows drive: the only way an environment-derived
 * path can survive `portable()` and pin `projectionSha256` to the machine that wrote the fixture. */
const ABSOLUTE_PATH_IN_PROJECTION = /"(?:\/|[A-Za-z]:[\\/])[^"]*"/u;

function projectionHash(config: Record<string, any>, expectedRoot: string): string {
  return createHash("sha256").update(canonical(behavioralProjection(config, expectedRoot))).digest("hex");
}

/** 🚫️ Directory names that hold neither a `runVitest` call site nor a collectable test file: installed
 * dependencies, generated and build output, VCS and repo-tool state. `.🧬semio` carries the shared cargo
 * build dir and every ticket folder, and `🗑️generated` the staged plugin bundles, so walking either
 * costs more than the rest of the repository together. */
const UNWALKED_DIRECTORIES: ReadonlySet<string> = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated", "🤖️jco", ".git", ".🧬semio", "⚡️cache", "temp"]);

let repositoryTreeCache: string[] | undefined;

/** 🗂️ Every file in the repository, absolute. Walked once — three separate walks of this tree were the
 * largest single cost in this gate, and each one is the same traversal. */
function repositoryTree(): string[] {
  if (repositoryTreeCache !== undefined) return repositoryTreeCache;
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
      if (entry.isSymbolicLink()) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        if (!UNWALKED_DIRECTORIES.has(entry.name)) stack.push(path);
      } else files.push(path);
    }
  }
  repositoryTreeCache = files;
  return files;
}

const PRODUCT_TREE_ROOTS = ["♻️mit-bestand", "✏️s", "🌎️hub", "🧰️framework"].map((path) => `${resolve(repoRoot, path)}/`);

function walkProductFiles(visit: (path: string) => void): void {
  for (const path of repositoryTree()) if (PRODUCT_TREE_ROOTS.some((root) => path.startsWith(root))) visit(path);
}

/** 📥️ The user configuration an owner module declares, resolved the way Vitest resolves it.
 *
 * 🩸️ Loaded by plain `import()`, not `vite.loadConfigFromFile`. Both yield the same object — Vite's
 * loader esbuild-bundles the file and evaluates it, and `defineConfig` is the identity function, so the
 * module's default export *is* the user config — but the bundle step costs seconds per owner and,
 * across 46 owners under fleet load, pushed this gate past the 120 s `runTestBudgeted` ceiling while
 * proving nothing the import does not. An owner exporting a config *function* is called with the same
 * `test`-mode environment Vitest passes. */
async function ownerConfiguration(ownerPath: string): Promise<Record<string, any>> {
  const module = (await import(pathToFileURL(ownerPath).href)) as { default?: unknown };
  const exported = module.default;
  const resolved = typeof exported === "function" ? await (exported as (environment: unknown) => unknown)({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported;
  return resolved as Record<string, any>;
}

/** 🕳️ Owner patterns that select nothing on disk — the suite an owner believes it runs and does not.
 *
 * 🩸️ A `📜️script.ts test` target whose config collects nothing is the repo's most expensive silent
 * failure: `@semio-tech/framework` shipped a config whose `includeSource` reached zero files, and
 * `…/📅️33.projektetage` selected `"🧪️vitest.config.ts"`, a path that has never existed. Neither is
 * visible from a green `test` target, because a pattern that matches nothing is not an error to
 * vitest — it simply narrows the run. Patterns resolve against the owner's own `test.root`, and many
 * legitimately climb above it (`../../📥️cold-pair/🟦️.ts`), so they are matched absolute. */
function patternsSelectingNothing(config: Record<string, any>, ownerRoot: string): string[] {
  const patterns: string[] = [...(config.test?.include ?? []), ...(config.test?.includeSource ?? [])];
  const files = repositoryTree();
  return patterns.filter((pattern) => {
    const glob = new Bun.Glob(isAbsolute(pattern) ? pattern : resolve(ownerRoot, pattern).replaceAll("\\", "/"));
    return !files.some((file) => glob.match(file));
  });
}

/** 🧾️ Absolute file patterns one `📋️project.json` declares as cache inputs, from every `namedInputs`
 * entry and every target's own `inputs`.
 *
 * 🩸️ Read as JSON and expanded, not searched as text. A project may declare an owner literally
 * (`{workspaceRoot}/…/🧪️tests/🎚️config/🟦️.ts`) or by glob (`{workspaceRoot}/…/🧪️tests/**\/*`), and both
 * make Nx re-run the target when the config changes — a substring search sees only the first and
 * reported `@semio-tech/framework-replication` as unregistered the moment its explicit list became an
 * equivalent glob. Negations and non-path tokens (`default`, `^default`, `{ externalDependencies }`)
 * carry no file and are dropped. */
function nxInputPatterns(projectPath: string): string[] {
  let project: Record<string, any>;
  try {
    project = JSON.parse(readFileSync(projectPath, "utf8"));
  } catch {
    return [];
  }
  const projectRoot = dirname(projectPath);
  const declared: unknown[] = [...Object.values(project.namedInputs ?? {}).flatMap((entry) => (Array.isArray(entry) ? entry : [])), ...Object.values(project.targets ?? {}).flatMap((target: any) => (Array.isArray(target?.inputs) ? target.inputs : []))];
  return declared
    .filter((entry): entry is string => typeof entry === "string" && entry.includes("/") && !entry.startsWith("!"))
    .map((entry) => (entry.startsWith("{workspaceRoot}/") ? join(repoRoot, entry.slice("{workspaceRoot}/".length)) : entry.startsWith("{projectRoot}/") ? join(projectRoot, entry.slice("{projectRoot}/".length)) : join(projectRoot, entry)).replaceAll("\\", "/"));
}

/** 🧭️ The `this.root` a `BundleScript` in `path` runs with. A router lives in a `📜️script.ts`, but its
 * `*Script` classes are routinely extracted into sibling semantic owners (`…/🏃️execution/🟦️.ts`), which
 * have no `📜️script.ts` above them — the bundle that imports them is the `📦️packages/🟦️typescript` of the
 * nearest semantic ancestor. */
function routerRoot(path: string): string {
  let candidate = dirname(path);
  while (candidate !== repoRoot) {
    if (existsSync(join(candidate, "📜️script.ts"))) return candidate;
    const bundle = join(candidate, "📦️packages", "🟦️typescript");
    if (existsSync(join(bundle, "📜️script.ts"))) return bundle;
    candidate = dirname(candidate);
  }
  return repoRoot;
}


for (const owner of fixture.owners) {
  if (!process.argv.slice(2).some((needle) => owner.ownerPath.includes(needle))) continue;
  const expectedRoot = resolve(repoRoot, owner.configurationRoot);
  const config = await ownerConfiguration(resolve(repoRoot, owner.ownerPath));
  console.log(owner.ownerPath, owner.projectionSha256, projectionHash(config, owner.configurationRoot));
}
