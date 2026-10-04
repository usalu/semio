# Repo Infrastructure Conventions for `🧰️framework/🛍️products/<emoji>quiz`, `<emoji>teaching/…`, and the quiz website

Scope: exact, verbatim-sourced conventions an implementer needs to add (a) a new framework product, (b) a
new top-level domain tree (sibling of `👴️leutwiler`, `♻️mit-bestand`, `🏢️semio-tech`, `🌎️hub`), and (c) a
website, without guessing. All paths are relative to `C:\git\semio`.

**Caveat found while exploring (report honestly, not fixed — read-only explorer):** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts` currently has an
**unresolved git merge conflict** (`<<<<<<< Updated upstream` at line 13; `git status` shows `UU` for this
file — a concurrent agent's work). Because `📜️script.ts` eagerly imports the whole library module graph,
`bun ./📜️script.ts <anything>` currently fails to even parse (`error: Unexpected <<`). This is unrelated to
this ticket; per AGENTS.md ("You SHOULD NOT stop because of detecting others are working on the same
area… ignore unrelated recent changes"), I did not touch it, but the quiz/teaching implementer will hit the
same failure until someone resolves it. `bun ./📜️script.ts verify taxonomy report --scope 👴️leutwiler` was
the read-only check attempted; its full output (the parse error) is at
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/🗑️generated/verify-taxonomy-leutwiler.log`.

---

## 1. Root `📜️script.ts` (1,879,809 bytes / 26,047 lines) — structure and dispatch

Header (`📜️script.ts:48-50`):

```ts
/**
 * 🧭️ Monorepo command router: `bun ./📜️script.ts <verb> [segments…]` (e.g. `📜️script.ts dev`, `📜️script.ts dev mcp`).
 */
```

It is one flat file: hundreds of `Script` subclasses grouped in `//#region 🔖️XyzScript` … `//#endregion` blocks,
ending in a single dispatch table (`📜️script.ts:15732-15772`):

```ts
//#region 🔖️Dispatch
const router = new ScriptRouter(WORKSPACE_ROOT, WORKSPACE_ROOT)
  .register("os", OsScript)
  .register("semio", SemioScript)
  .register("examples", ExamplesScript)
  .register("setup", SetupScript)
  .register("start", StartScript)
  .register("dev", DevScript)
  .register("generate", GenerateScript)
  .register("scale-fixture", /* … inline class … */)
  .register("new", CleanMechanismNewScript)
  .register("schema", SchemaScript)
  .register("lint", LintScript)
  .register("verify", VerifyScript)
  .register("format", FormatScript)
  .register("test", TestScript)
  .register("bench", BenchScript)
  .register("stdio", StdioScript)
  .register("build", BuildScript)
  .register("cpp", CppScript)
  .register("publish", PublishScript)
  .register("clean", CleanScript)
  .register("micro-commit", MicroCommitScript)
  .register("commit", CommitScript);
//#endregion 🔖️Dispatch
```

The `Script`/`ScriptRouter` base classes live in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts` (verbatim, 82 lines total):

```ts
/** 🧭️Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script {
  protected readonly root: string;
  protected readonly repoRoot: string;
  constructor(root: string, repoRoot: string) { this.root = root; this.repoRoot = repoRoot; }
  abstract run(segments: string[]): void | Promise<void>;
}

/** 📦️Bundle-scoped command with `root` at the package directory. */
export abstract class BundleScript extends Script {
  constructor(bundleRoot: string, repoRoot?: string) { super(bundleRoot, repoRoot ?? findRepoRoot(bundleRoot)); }
}
export type ScriptCommand = new (root: string, repoRoot: string) => Script;

/** 🧭️Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter {
  private readonly commands = new Map<string, ScriptCommand>();
  readonly bundleRoot: string;
  readonly repoRoot: string;
  constructor(bundleRoot: string, repoRoot: string = findRepoRoot(bundleRoot)) { this.bundleRoot = bundleRoot; this.repoRoot = repoRoot; }
  register(name: string, Command: ScriptCommand): this { this.commands.set(name, Command); return this; }
  usage(): string { /* … */ }
  hasCommands(): boolean { return this.commands.size > 0; }
  async run(segments: string[]): Promise<void> {
    const name = segments[0];
    if (!name) { console.error(`usage: ${this.usage()}`); process.exit(1); }
    const Command = this.commands.get(name);
    if (!Command) { console.error(`unknown command ${JSON.stringify(name)}`); console.error(`usage: ${this.usage()}`); process.exit(1); }
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)));
  }
}

/** 📁️Walks parents until the monorepo root (`nx.json` + workspace `package.json`); starts from the Nx workspace root when no directory is given. */
export function findRepoRoot(start?: string): string { /* walks up looking for nx.json + package.json */ }
```

A per-directory `📜️script.ts` **extends** this by constructing its own `ScriptRouter`, registering a handful
of `BundleScript` subclasses, and calling `runBundleScriptMain(router, import.meta.url, …)` (also from the
shared library `🟦️.ts`) at the bottom. It NEVER duplicates dispatch logic — `AGENTS.md`'s "extend the script
functionality from `./📜️script.ts`" is satisfied by importing `Script`, `BundleScript`, `ScriptRouter`,
`runBundleScriptMain` (and whatever helpers it needs) from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`, which is the one giant
shared-helper barrel file the root `📜️script.ts` itself imports from (see its own import block,
`📜️script.ts:51-125`: `buildBudgetMs`, `discoverPackages`, `resolveTestLevel`, `runCmd`, `runVitest`,
`runCargo`, `runCmdStatus`, `spawnDaemon`, `TEST_LEVELS`, `layeringBreaches`, `loadTaxonomy`, etc.).

### Three real per-directory `📜️script.ts` examples

**Simplest — a leaf bundle with no test-level ladder** (`👴️leutwiler/💤️realparts-of-powers-z-n/📜️script.ts`, all 27 lines):

```ts
#!/usr/bin/env bun
/** 💤️ `@leutwiler/realparts-of-powers-z-n` task router: `bun ./📜️script.ts <deps|build|test> [lean|📯️notes|🏆️proof]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { build, prepareDependencies, verifyAxioms } from "./🟦️.ts";

class DepsScript extends BundleScript {
  async run(): Promise<void> { await prepareDependencies(this.root, this.repoRoot); }
}
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> { await build(this.root, this.repoRoot, segments[0]); }
}
class TestScript extends BundleScript {
  async run(): Promise<void> { await verifyAxioms(this.root, this.repoRoot); }
}

const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
```

Note the actual logic (`prepareDependencies`, `build`, `verifyAxioms`) is **not** in `📜️script.ts` — it's
imported from the sibling generically-named `🟦️.ts` (the package's one TypeScript implementation file).
`📜️script.ts` is purely the CLI adapter, exactly matching `AGENTS.md`: "You MUST NOT create any other script
files other than `📜️script.ts`" — everything else lives in taxonomy-named source files that `📜️script.ts`
imports.

**A package with a standard test-level ladder** (`🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/📜️script.ts`, all 15 lines):

```ts
#!/usr/bin/env bun
/** 🎤️ `@semio-tech/presentation` router: `bun ./📜️script.ts test [fundamental|quick|long|exhaustive]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
```

`resolveTestLevel` implements the shared `quick|long|exhaustive` (`TEST_LEVELS`) ladder used everywhere.

**A large server/hub router** (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts`, 587 lines) — same shape, many more
registered subcommands (`test`, `two-client-e2e`, `document-growth-e2e`, `backend`, `backup-restore-drill`,
`shutdown-drill`, `residency-watch`, `boot-watch`, `hub-freshness`, `agent-ceiling-check`,
`docker-image-build`, `docker-image-check`, `typecheck`). Same import of `BundleScript`/`ScriptRouter` from
the shared library, same `router = new ScriptRouter(import.meta.dir).register(...)…` ending, same
`await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" })` tail. Its acceptance-check
rows are also the canonical example of the `LocalizedText` i18n convention (see §7).

### Shared helpers available from the library barrel (`📦️packages/🟦️typescript/🟦️.ts`)

Non-exhaustive, taken from the root `📜️script.ts` import list (`📜️script.ts:51-125`): `Script`,
`BundleScript`, `ScriptRouter`, `runBundleScriptMain`, `runWorkspaceScriptMain`, `resolveTestLevel`,
`TEST_LEVELS`, `runVitest`, `runCargo`, `runCmd`, `runCmdStatus`, `runProbe`, `spawnDaemon`,
`runTestBudgeted`, `buildBudgetMs`, `daemonBudgetOpts`, `orchestratorBudgetOpts`, `discoverPackages`,
`discoverOwners`, `discoverPackageProblems`, `loadTaxonomy`, `layeringBreaches`/`layeringCounts`/
`layeringReferences`/`writeLayeringBaseline`, `goCoverageArgs`/`goLevelTestArgs`/`goProfileToLcov`,
`mergeLcov`/`parseLcov`/`renderLcov`/`summarizeCoverage`/`enforceCoverageThreshold`, `getWorkspaceRoot`,
`getRepoMetaDir`, `getSemioRoot`, `installMicroCommitGitHooks`, `runCommit`/`runMicroCommit`,
`TechnologyLinter`, `tryRun`.

---

## 2. `📋️project.json` conventions

`project.json` files contain ONLY nx target definitions whose `command` is always
`bun ./📜️script.ts <verb> [subverb…]` (never inline logic). Three verbatim examples:

**Per-package, leaf bundle** (`👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json`):

```json
{
  "name": "@leutwiler/realparts-of-powers-z-n",
  "$schema": "../../node_modules/nx/schemas/project-schema.json",
  "namedInputs": { "default": ["{projectRoot}/**/*"] },
  "targets": {
    "deps": { "executor": "nx:run-commands", "cache": false, "options": { "cwd": "👴️leutwiler/💤️realparts-of-powers-z-n", "command": "bun ./📜️script.ts deps" } },
    "build-lean": {
      "executor": "nx:run-commands", "cache": true, "dependsOn": ["deps"],
      "inputs": ["{projectRoot}/📜️script.ts", "{projectRoot}/🟦️.ts", "{projectRoot}/🧘️lean/**/*"],
      "outputs": [],
      "options": { "cwd": "👴️leutwiler/💤️realparts-of-powers-z-n", "command": "bun ./📜️script.ts build lean" }
    },
    "build": { "executor": "nx:noop", "dependsOn": ["build-lean", "build-notes", "build-proof"] },
    "test": {
      "executor": "nx:run-commands", "cache": true, "dependsOn": ["build-lean"],
      "inputs": ["{projectRoot}/📜️script.ts", "{projectRoot}/🟦️.ts", "{projectRoot}/🧘️lean/**/*"],
      "outputs": [],
      "options": { "cwd": "👴️leutwiler/💤️realparts-of-powers-z-n", "command": "bun ./📜️script.ts test" }
    }
  }
}
```

**Per-package, framework/hub** (`🌎️hub/📦️packages/🟦️typescript/📋️project.json`, targets abbreviated):

```json
{
  "name": "os-hub-ts",
  "$schema": "../../../node_modules/nx/schemas/project-schema.json",
  "namedInputs": {
    "default": ["{projectRoot}/**/*", "{workspaceRoot}/🌎️hub/📦️packages/🦀️rust/**/*.rs", "{workspaceRoot}/🌎️hub/🧪️tests/🎚️config/🟦️.ts"]
  },
  "targets": {
    "typecheck": { "executor": "nx:run-commands", "cache": true, "options": { "cwd": "🌎️hub/📦️packages/🟦️typescript", "command": "bun ./📜️script.ts typecheck", "forwardAllArgs": true }, "outputs": [] },
    "test": { "executor": "nx:run-commands", "cache": true, "options": { "cwd": "🌎️hub/📦️packages/🟦️typescript", "command": "bun ./📜️script.ts test", "forwardAllArgs": true }, "outputs": [] },
    "test-quick": { "... command": "bun ./📜️script.ts test quick" },
    "backend-up": { "cache": false, "options": { "command": "bun ./📜️script.ts backend up" } }
  }
}
```

**Root domain-level** (`📋️project.json`, project name `"workspace"`, 71,329 bytes / 2,465 lines — this is the
"orchestrator" that `package.json`'s `bun nx run workspace:<target>` scripts call into):

```json
{
  "name": "workspace",
  "$schema": "node_modules/nx/schemas/project-schema.json",
  "projectType": "application",
  "sourceRoot": "",
  "metadata": { "semio": { "taxonomy": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" } },
  "namedInputs": { "default": ["{projectRoot}/**/*", "{workspaceRoot}/🧪️tests/🎚️config/🟦️.ts", "{workspaceRoot}/🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts"] },
  "targets": { "...": "hundreds of nx:run-commands targets, each 'bun ./📜️script.ts verify|schema|lint|test|build ...'" }
}
```

**Discovery:** `nx.json`'s custom plugin (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`, 1,489
lines) is registered with `include: ["**/📋️project.json", "**/Cargo.toml", "bun.lock", "**/*.patch"]`
(`nx.json:50-53`) — **every** `📋️project.json` anywhere in the tree is auto-discovered as an nx project; no
separate registration file. `.nxignore` (7 lines) excludes `**/pkg/**` (wasm-pack output),
`/.🧬semio/🦑️repo/🎫️tickets/**`, `/.🧬semio/🦑️repo/⚡️cache/**`, `/.🧬semio/🗺️map/**`, `**/.lake/**`.

**Naming:** `name` is usually `@semio-tech/<kebab-slug>` for framework/plugin packages, but is NOT universal —
a top-level non-`semio-tech` domain gets its own scope (`@leutwiler/realparts-of-powers-z-n`), and some hub
packages are unscoped bare names (`os-hub-ts`, `os-hub`, `os-hub-admin`). **For a new `👴️leutwiler`-sibling
domain (e.g. `🎓️teaching`), follow the `@leutwiler/…` precedent: `@teaching/<slug>` (or similar) rather than
`@semio-tech/…`.** For the `🧰️framework/🛍️products/<emoji>quiz` product, follow the framework-product
precedent and use `@semio-tech/quiz…` package names (matching `@semio-tech/presentation`,
`@semio-tech/print`, etc. seen throughout `package.json`'s `scripts`).

**`tags`** appear only occasionally (mostly on Rust packages, e.g.
`🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust/📋️project.json:7` → `"tags": ["scope:framework"]`); most
TypeScript packages (leutwiler, hub, presentation) carry none. **`implicitDependencies`** appears rarely,
e.g. `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📋️project.json:110-112`:

```json
"implicitDependencies": ["@semio-tech/framework-schema"]
```

Neither is required scaffolding; add them only if genuinely needed (schema-generation ordering, scope
tagging for nx affected-graph queries).

---

## 3. Root `package.json`, per-package `package.json`, `tsconfig.json`, `bunfig.toml`

**Root `package.json` (328 lines) `workspaces` is an EXPLICIT array of paths, not a glob** (`package.json:6-134`).
New TypeScript packages under a new product or domain **must be added by hand** to this array, e.g.:

```json
"workspaces": [
  "...",
  "👴️leutwiler/💤️realparts-of-powers-z-n" /* NOT present as an entry — leutwiler is a bun/nx script.ts+🟦️.ts leaf with no package.json, so it never joined workspaces */,
  "🌎️hub/📦️packages/🟦️typescript",
  "🏢️semio-tech/🎡️play",
  "🧰️framework/📦️packages/🟦️typescript",
  "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript",
  "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript",
  "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript",
  "..."
]
```

(`👴️leutwiler/💤️realparts-of-powers-z-n` has no `package.json` at all — it is a `bun`+`nx` leaf with
`📋️project.json` + `📜️script.ts` + `🟦️.ts` only, so it is not a bun workspace member; its dependency needs
are Lean/`lake`, handled entirely inside `deps`. A `🧰️framework/🛍️products/<emoji>quiz/📦️packages/🟦️typescript`
package, by contrast, needs a `package.json` and an entry appended to this `workspaces` array to be linked.)

`engines`: `{ "bun": ">=1.2.0", "node": "24.15.0" }`; `packageManager: "bun@1.3.14"`. Root `scripts` all
delegate to nx, e.g. `"test": "bun nx run workspace:test"`, `"lint": "bun nx run workspace:lint"`,
`"schema:check": "bun nx run workspace:schema-check"`, and per-bundle convenience aliases exist too, e.g.:

```json
"build:leutwiler:realparts-of-powers-z-n": "bun nx run @leutwiler/realparts-of-powers-z-n:build",
"test:leutwiler:realparts-of-powers-z-n": "bun nx run @leutwiler/realparts-of-powers-z-n:test",
"dev:print": "bun nx run @semio-tech/print:watch",
"dev:semio-tech:play": "bun nx run @semio-tech/semio-tech-play:dev",
```

A new quiz product / teaching domain should add analogous `dev:…`/`build:…`/`test:…` root scripts (matching
the existing naming: `<verb>:<domain>[:<subdomain>]`).

**`nx.json`** (62 lines): `cli.packageManager: "bun"`, `defaultBase: "⛳wip"`, `cacheDirectory:
".🧬semio/🦑️repo/⚡️cache/nx"`, `maxCacheSize: "200GB"`, `analytics: false`. Plugins:
`@nx/js` (package.json/source/lockfile analysis all disabled), `@nxlv/python`, and the two repo-owned
plugins (`🟨️.mjs` under `📚️library` for `📋️project.json`/`Cargo.toml`/`bun.lock`/`*.patch`, and one under
`🧪️test`).

**No `tsconfig.json` path aliases** — `compilerOptions` has no `"paths"` map; every source file imports its
neighbors with plain relative paths (confirmed by every `📜️script.ts` shown above using `"../../…/🟦️.ts"`
chains). `moduleResolution: "bundler"`, `strict: true`, `allowJs: false`, `jsx: "react-jsx"`,
`noEmit: true`. `exclude` already ignores `**/🗑️generated/**`, `**/🤖️generated/**`, `.🧬semio/🦑️repo/🎫️tickets/**`,
`**/target/**`/`**/🎯️target/**` (Rust/Cargo build output), etc. — nothing to add for a new product unless it
introduces a new generated-output directory name.

**`bunfig.toml`** (3 lines, entire file): `[install]\nlinker = "hoisted"` — needed for Electron/Playwright
native deps; nothing product-specific to configure here.

**Per-package `package.json`** — none of the three script.ts examples above show one being hand-inspected in
this pass beyond the workspaces list; new quiz packages should mirror the neighboring
`🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/package.json` shape (name
`@semio-tech/presentation`, no build-time external runtime deps per AGENTS.md's "MUST NOT create runtime
dependencies on external libraries").

---

## 4. `.vscode/launch.json` / `.vscode/🧩️launch.seed.jsonc`

Both files are **hand-maintained, committed text**, not auto-generated by any `📜️script.ts generate`
subcommand (`GenerateScript` only has `taxonomy`, `plugin-glue`, `scale-fixture` — no `launch` verb).
`.vscode/launch.json` is 17,636 lines / **1,250** `"name"` entries; the seed `.vscode/🧩️launch.seed.jsonc` is
4,939 lines / **366** entries — the seed is a hand-authored SUBSET (special-purpose rows: the required
gates, hub/backend/mit-bestand/semio-tech-play dev servers, print deps, etc.), not a template that gets
expanded into the full file by tooling.

`📜️script.ts` DOES read both files as data for a coverage check (`📜️script.ts:9265-9266`):

```ts
const INTERACTIVITY_ALL_APP_LAUNCH_FILE = ".vscode/launch.json";
const INTERACTIVITY_ALL_APP_LAUNCH_SEED_FILE = ".vscode/🧩️launch.seed.jsonc";
```

and cross-validates that every discovered plugin `app`/`action` from the plugin manifests has a matching
launch row (`interactivityAllAppLaunchCoverageFailures`, `📜️script.ts:9581-9591`), plus that six required
gate rows exist verbatim in `.vscode/launch.json`'s `4_gate` group (`📜️script.ts:9268-9289`):

```ts
/** ⚖️ Launch rows every `.vscode/launch.json` must register exactly once, in the `4_gate` group.
 *
 * AGENTS.md: "All devs are using `launch.json` and never use the cli" — a verification verb that no
 * launch row runs is a verb no dev can run. ... */
const INTERACTIVITY_ALL_APP_REQUIRED_GATES: readonly { readonly name: string; readonly command: string }[] = [
  { name: "⚖️gate⚡️interactivity", command: "bun nx run workspace:verify -- interactivity" },
  { name: "⚖️gate⚡️interactivity🎯️tool-jobs", command: "bun nx run workspace:verify -- interactivity tool-jobs" },
  { name: "⚖️gate⚡️interactivity🧭️apps", command: "bun nx run workspace:verify -- interactivity apps" },
  { name: "⚖️gate⚡️interactivity🧭️apps🎛️actions", command: "bun nx run workspace:verify -- interactivity apps --actions" },
  { name: "⚖️gate📦️dependencies", command: "bun nx run workspace:verify -- dependencies" },
  { name: "⚖️gate📦️dependencies0️⃣", command: "bun nx run workspace:verify -- dependencies literal-external" },
  { name: "⚖️gate🪆️composed-child-refs", command: "bun nx run workspace:verify -- composed-child-refs" },
];
```

Verified by `bun nx run workspace:verify -- interactivity apps` (i.e. `bun ./📜️script.ts verify interactivity apps`
under the hood — see §5).

### Grouping / ordering / naming

`presentation.group` value counts across `launch.json`: `4_gate` (667), `3_dev` (410), `4_build` (162),
`2_mouse` (4), `0_dev` (4), `1_keyboard` (2) — i.e. almost everything is `3_dev` (interactive dev servers /
one-off scripts), `4_gate` (verification/policy gates), or `4_build` (release builds). `order` is a float
that clusters related rows (e.g. `386.4`, `386.5`, `386.6` for three `🖍️draw` test variants; `387`, `387.002`,
`387.003`, `387.004` for `os-hub` dev variants; `213`, `213.1`, `213.2`, `213.3` for
`mit-bestand-demonstrator`/`semio-tech-play`) — pick the next free `.N` decimal under the nearest sibling
domain's integer order, or append a new integer bucket at the end.

Name scheme is `<verb-emoji><verb><domain-emoji><domain>[<subdomain-emoji><subdomain>]`, concatenated with NO
spaces, e.g. `🧪️test🖍️draw🟦️`, `🛠️dev♻️mit-bestand🧺️demonstrator`, `🛠️dev🗄️os-hub🐘️postgres`,
`⚖️gate⚡️interactivity`. Verbatim `3_dev` example (`.vscode/launch.json` around mit-bestand-demonstrator/semio-tech-play):

```json
{
  "name": "🛠️dev♻️mit-bestand🧺️demonstrator",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run @semio-tech/mit-bestand-demonstrator:dev",
  "cwd": "${workspaceFolder}",
  "env": { "MIT_BESTAND_DEMONSTRATOR_PORT": "6029", "SEMIO_RENDERER": "react" },
  "presentation": { "group": "3_dev", "order": 213 },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6029)",
    "uriFormat": "%s"
  }
},
{
  "name": "🛠️dev🏢️semio-tech🎡️play",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run @semio-tech/semio-tech-play:dev",
  "cwd": "${workspaceFolder}",
  "env": { "SEMIO_TECH_PLAY_PORT": "6033", "SEMIO_RENDERER": "react" },
  "presentation": { "group": "3_dev", "order": 213.3 },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6033)",
    "uriFormat": "%s"
  }
}
```

Hub `3_dev` example (`.vscode/launch.json` around line 3591-3600):

```json
{
  "name": "🛠️dev🗄️os-hub",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run os-hub:dev",
  "cwd": "${workspaceFolder}",
  "env": { "OS_HUB_PORT": "8787", "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/" },
  "presentation": { "group": "3_dev", "order": 387 },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
    "uriFormat": "%s/admin"
  }
}
```

Gate example (`.vscode/launch.json:6297-6306`):

```json
{
  "name": "⚖️gate⚡️interactivity",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run workspace:verify -- interactivity",
  "cwd": "${workspaceFolder}",
  "presentation": { "group": "4_gate", "order": 411.43 }
}
```

**GOTCHA — do not copy the leutwiler precedent here:** `👴️leutwiler` was registered in taxonomy (§5) but has
**zero** entries in either `.vscode/launch.json` or `.vscode/🧩️launch.seed.jsonc` (`grep -n
"leutwiler|realparts-of-powers"` on both files returns nothing). This is a real gap against AGENTS.md's "All
devs are using `launch.json` and never use the cli… register all executable commands there" — the
quiz/teaching implementer MUST add `🛠️dev`/`🧪️test`/`🔨️build` rows for every new product/domain command
directly to `.vscode/launch.json` (following the naming/group/order scheme above), not skip it the way
leutwiler's ticket did.

**To register a new entry correctly:** add a JSON object to `.vscode/launch.json`'s `configurations` array
(and to the seed file too if it is a "special" row like a dev server/gate rather than a per-app row the
discovery mechanism already covers), using `type: "node-terminal"`, `request: "launch"`, `command: "bun nx
run <project>:<target> [-- extra args]"`, `cwd: "${workspaceFolder}"`, and a `presentation.group`/`order`
consistent with the table above. Then run `bun nx run workspace:verify -- interactivity apps` to confirm
coverage.

---

## 5. Taxonomy validation

### Files

| File | Size | Role |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | 29,434 lines | SSOT taxonomy vocabulary — file kinds, semantic-directory kinds, member kinds, contracts, area/layer registries. |
| `🔒️dependencies.json` (repo root) | 4,195 lines / 177,952 bytes | **Generated** truth-of-external-dependencies (schemaVersion 2, `generatedAt`/`commit` stamped) — do not hand-edit; AGENTS.md forbids new runtime external deps anyway. |
| `🧅️layering.json` (repo root) | 50 lines | Shrink-only "layering ratchet": per-file allowance of references into an implementation area; regenerate deliberately with `bun ./📜️script.ts verify layering write-baseline`, never to silence a failure. |
| `🚚️migration.json` (repo root) | 13 lines | Shrink-only ratchet counting `unmanagedTests` (test files outside the canonical owner-root test tree) per top-level `areas` key. |
| `📋️project.json` (repo root) | 71,329 bytes / 2,465 lines | The `"workspace"` nx project — hundreds of `verify`/`schema`/`lint`/`test`/`build` targets, all `bun ./📜️script.ts …`. |
| `🧰️framework/🛍️products/🔣️.json` | 1,390 bytes | An `x-semio` **collection manifest** listing the 5 framework products (`💻️os`, `📓️print`, `🎤️presentation`, `🦑️repo`, `🖥️server`) each with `directory`, `id` (e.g. `"framework.product.print"`), `kind: "product"`, `responsibility`. **A new `<emoji>quiz` product MUST add a sixth member here.** |

Verbatim `🧰️framework/🛍️products/🔣️.json` (entire file, 33 lines):

```json
{
  "x-semio": {
    "kind": "collection",
    "members": [
      { "directory": "💻️os", "id": "framework.product.os", "kind": "product", "responsibility": "Hosts the composable operating-system shell, application runtime, and product-scoped capabilities." },
      { "directory": "📓️print", "id": "framework.product.print", "kind": "product", "responsibility": "Builds and verifies canonical print templates, fonts, and generated LaTeX assets." },
      { "directory": "🎤️presentation", "id": "framework.product.presentation", "kind": "product", "responsibility": "Provides the render-independent declarative presentation model and its React + reveal.js renderer for slide decks." },
      { "directory": "🦑️repo", "id": "framework.product.repo", "kind": "product", "responsibility": "Provides repository discovery, validation, orchestration, and developer tooling." },
      { "directory": "🖥️server", "id": "framework.product.server", "kind": "product", "responsibility": "Hosts the authoritative application server: CQRS command/query buses, the actor turn protocol, replication gateway, policy engine, and the server-instance framework instances compose." }
    ]
  }
}
```

### Emoji rules (from `🔣️taxonomy.json`'s `variationSelectorPolicy` / `unicodeNormalization` / `pathEmojiPolicy` / `collisionPolicy`)

```json
"variationSelectorPolicy": { "selector": "️", "requiredAfterEmoji": true, "comparison": "ignore-selector" },
"unicodeNormalization": { "form": "NFC", "caseFold": "lower", "locale": "und" },
"pathEmojiPolicy": {
  "inventory": "git-visible",
  "identity": "single-emoji-grapheme",
  "siblingNamespace": "files-and-directories",
  "genericEmojiIdentities": ["📁️", "📂️", "📄️"],
  "reservedSubtreeDirectoryNames": [".git", ".🧬semio", ".semio", ".vscode", ".github", ".cursor", ".claude", ".agents", ".devcontainer", ".kiro", ".codex", ".windsurf", ".copilot", ".factory", ".ralph-tui", ".cargo", ".config", ".storybook"]
},
"collisionPolicy": { "comparisons": ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"], "maxPathBytes": 240, "rejectWindowsReservedNames": true, "rejectTrailingDotsAndSpaces": true }
```

So: **every** identifying emoji MUST be followed by U+FE0F (variation selector 16) — this is why every emoji
in this codebase (and in this report) is written `👴️`, `🧰️`, `🟦️`, not the bare codepoint; a file and a
directory at the same level share one namespace, so a new `<emoji>quiz` directory's emoji must be unique
among ALL siblings (files and folders) at that level, not just other directories; README.md / README (no
emoji) are reserved exceptions (`fixedFilenameContracts.reserved-readme` /
`reserved-readme-markdown`, `.../🔣️taxonomy.json`).

### How a new top-level domain area registers (verbatim precedent: `👴️leutwiler`)

`🔣️taxonomy.json`'s top-level key for "what emoji+slug names a directory of kind X" is
`semanticDirectoryKinds` (NOT `kinds` — the JSON has no literal `"kinds"` root key despite ticket scripts'
variable names). Its "what children a kind of directory owns" key is `semanticDirectoryMemberKinds`. The
leutwiler ticket's `register_taxonomy.py` did four things, all still `insert_after_block`/`insert_map_line`
text-surgery on the same JSON file (verbatim, from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️21/LEUTWILER-REAL-PARTS-OF-POWERS-LEAN-PROOF/register_taxonomy.py`):

```python
def kind(emoji, slug):
    return {"emoji": emoji + VS, "slugPattern": "^" + slug + "$", "allowEmojiOnly": False}

def members(owner, names):
    return {"ownerKindIds": [owner], "memberNames": names, "source": "registry"}

assert AREA not in text  # idempotency guard — refuses to double-register
text = insert_after_block(text, "fileKinds", "toml", {
    "lean-source": {"emoji": "🧘" + VS, "extensionChains": [".lean"], "role": "source"},
})
text = insert_after_block(text, "fileKindResolutionRules", "physical-toml-toml", {
    "physical-lean-source-lean": {"extensionChain": ".lean", "fileKindId": "lean-source", "priority": 0},
})
text = insert_after_block(text, "semanticDirectoryKinds", "semio-tech", {
    "leutwiler": kind("👴", "leutwiler"),
    "leutwiler-realparts-of-powers-z-n": kind("💤", "realparts-of-powers-z-n"),
    "leutwiler-notes": kind("📯", "notes"),
    "leutwiler-proof": kind("🏆", "proof"),
    "leutwiler-lean": kind("🧘", "lean"),
})
text = insert_after_block(text, "semanticDirectoryMemberKinds", "members-of-semio-tech", {
    "members-of-leutwiler": members("leutwiler", [PROOF]),
    "members-of-leutwiler-realparts-of-powers-z-n": members("leutwiler-realparts-of-powers-z-n", [NOTES, PAPER, LEAN]),
})
text = insert_after_block(text, "fixedFilenameContracts", "cargo-manifest", {
    "lake-manifest": contract(LEAN_ROOT + "/lakefile.toml", "Lake package configuration discovery", "lake build"),
    # … lake-lock, lean-toolchain, lean-module-source — special build-tool files needing fixed-filename contracts
})
text = insert_map_line(text, "areas", "🏢" + VS + "semio-tech", AREA, "clean")
# then a hand-rolled insert into "areaLayers" right after the "semio-tech": "implementation" line, adding:
#   "👴️leutwiler": "implementation"
```

Resulting taxonomy state (verified directly in the live file):

```json
// semanticDirectoryKinds
"leutwiler": { "emoji": "👴️", "slugPattern": "^leutwiler$", "allowEmojiOnly": false },
"leutwiler-realparts-of-powers-z-n": { "emoji": "💤️", "slugPattern": "^realparts-of-powers-z-n$", "allowEmojiOnly": false },
// semanticDirectoryMemberKinds
"members-of-leutwiler": { "ownerKindIds": ["leutwiler"], "memberNames": ["💤️realparts-of-powers-z-n"], "source": "registry" },
// areas (must be "clean" — the default enforcement state; "exempt" only for ticket-log trees)
"👴️leutwiler": "clean",
// areaLayers ("framework" vs "implementation" — controls the layering ratchet in 🧅️layering.json)
"👴️leutwiler": "implementation"
```

**For `🎓️teaching` (a new top-level domain, sibling of `👴️leutwiler`/`♻️mit-bestand`/`🏢️semio-tech`/`🌎️hub`):**
add a `semanticDirectoryKinds` entry (emoji + `^teaching$` slug pattern), a
`semanticDirectoryMemberKinds["members-of-teaching"]` entry listing its direct children (e.g.
`🖥️proctor`, `🏛️architecture`), recurse the same pattern for each nested level (`🏛️architecture` →
`🔥️energy` → `{⚛️physics,🔥️heating,❄️cooling}` → `❓️quiz`, choosing one distinct emoji per new kind and
registering `semanticDirectoryKinds`/`semanticDirectoryMemberKinds` pairs all the way down), then add
`"🎓️teaching": "clean"` to `areas` and `"🎓️teaching": "implementation"` to `areaLayers`. **For
`🧰️framework/🛍️products/<emoji>quiz`**, this is simpler: it nests inside the ALREADY-registered `products`
kind (`semanticDirectoryKinds.products` → `slugPattern: "^products$"`) as a normal framework product, so it
just needs a `semanticDirectoryMemberKinds["members-of-products"]`-style addition (find and extend the
existing products membership list the same way) plus the `🧰️framework/🛍️products/🔣️.json` collection-manifest
member (§ table above) — it does NOT need its own `areas`/`areaLayers` entry, since `🧰️framework` as a whole
is already `"clean"`/`"framework"`.

Reserved ecosystem/lang vocabulary if the quiz product needs non-JS code:
`langs: ["🦀️rust","🟦️typescript","🟨️javascript","🐹️go","🐍️python","🔷️dotnet"]` — same six as everywhere
else; no new ecosystem should be needed.

### Validation command

`bun ./📜️script.ts verify taxonomy <report|enforce> [--scope <path>]` (`📜️script.ts:8406-8451`, dispatched
from `VerifyScript.run` at `📜️script.ts:7250-7253`). There is also `bun ./📜️script.ts verify taxonomy
implementation <report|enforce>` for the separate implementation-filesystem-vs-taxonomy census
(`📜️script.ts:8407-8429`), and package.json aliases `verify:taxonomy:implementation:report` /
`verify:taxonomy:implementation:enforce`. I attempted:

```
bun ./📜️script.ts verify taxonomy report --scope 👴️leutwiler
```

**Result: it currently cannot run at all** — see the caveat at the top of this report (unrelated `UU`
merge-conflict file breaks Bun's module parse for every `📜️script.ts` invocation right now). Full output
saved at `🗑️generated/verify-taxonomy-leutwiler.log` in this ticket.

---

## 6. Docstring convention and file/dir-naming emoji table

**Docstrings** are one-line (or hanging) `/** <emoji> <sentence>. */` comments immediately above the
construct, ALWAYS starting with an emoji (AGENTS.md: "You MUST start all docstrings with a unique and
fitting emoji"), e.g.:

```ts
/** 🧭️Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script { … }

/** 🌍 A user-visible string in both supported document languages; there is no default language. */
export type LocalizedText = { readonly en: string; readonly de: string };

/** 🐳️ `docker-image-build [--tag <tag>] [--jobs <n>] [--log <path>]` — a cold `docker build` of … */
class DockerImageBuildScript extends BundleScript { … }
```

Longer module-level docstrings additionally use `@see` doc-links to related files (AGENTS.md: "add links to
related resources in native docstring format"), e.g. (`🧰️framework/🛍️products/📓️print/🧬️schema/🟦️.ts:1-9`):

```ts
/** 📊️ Typed twin of `🧬️schema/🔣️.json`: the print visualization catalogue contract.
 * ...
 * @see 🖼️assets/🔣️viz-catalog.json — the catalogue this contract describes
 * @see 🖼️assets/📊️viz-taxonomy.md — the handcrafted taxonomy the catalogue covers
 */
```

**File/dir emoji table** (from `🔣️taxonomy.json`'s `fileKinds` / `semanticDirectoryKinds`, plus what's
visible on disk):

| Emoji+name | Kind id | Meaning |
|---|---|---|
| `🟦️.ts` / `🟦️.tsx` / `.mts`/`.cts`/`.d.ts` | `typescript-source` | The (generically-named) TypeScript implementation file of a package/module — semantics live in the FOLDER path, not the filename. |
| `🦀️.rs` | `rust-source` | Rust implementation file. |
| `🥒️.feature` | `gherkin-feature` | Gherkin/BDD feature spec (`role: "test"`). |
| `📝️.md` | `markdown` | Documentation (`role: "documentation"`). |
| `🔣️.json` (e.g. `🔣️.json`, `🔣️taxonomy.json`) | `json` | Schema/contract JSON (`role: "schema"`) — also the collection-manifest convention (`x-semio` `kind: "collection"`). |
| `⚙️.toml` | `toml` | Configuration (`role: "configuration"`; e.g. `Cargo.toml`, `lakefile.toml`). |
| `📜️script.ts` | fixed filename contract | The ONE permanent per-directory CLI script (AGENTS.md). |
| `📋️project.json` | fixed filename contract | The ONE nx project file per directory. |
| `🧫️fixtures` / `🧫️examples` (dir) | `fixtures` (`slugPattern: ^(fixtures\|examples)$`) | Test input fixtures. |
| `🧪️tests` / `🧪️oracle` (dir) | `tests` (`slugPattern: ^(tests\|oracle)$`) | Test suite root. |
| `🔮️oracles` (dir) | `oracles` | Third-party/independent oracle implementations tests compare against (AGENTS.md: "create the same output of a test with at least one third-party library"). |
| `🧬️schema` / `mutations`/`contract`/`wire` (dir) | `schema` (`slugPattern: ^(schema\|mutations\|contract\|wire)$`) | Schema-first contract definitions. |
| `📚️examples` (dir) | `examples` | Worked/example usage. |
| `📦️packages` (dir) | `packages` | Per-ecosystem package root (`🟦️typescript`, `🦀️rust`, `🔷️dotnet`, …). |
| `🔨️modules` (dir) | `modules` | Shared sub-module tree. |
| `🛍️products` (dir) | `products` | Framework product tree (`💻️os`, `📓️print`, `🎤️presentation`, `🦑️repo`, `🖥️server`, and where a new `<emoji>quiz` goes). |
| `🎯️targets` (dir) | — | Per-render-target subpackage (e.g. `⚛️react`, `🧊️wgpu`). |
| `🗑️generated` (dir) | rootDataDirNames-adjacent | Ticket-scoped throwaway output (mandated location per AGENTS.md ticket rules). |
| `🤖️generated` (dir) | `rootDataDirNames` | Machine-generated source (not hand-edited). |

`README.md`/`README` are the sole EXEMPT basenames (no emoji, `fixedFilenameContracts.reserved-readme*`).

---

## 7. i18n convention (en/de, no default language)

No single shared "LocalizedText" module is imported repo-wide — per AGENTS.md's "if code is repeated, it
MUST be close to each other", the SAME two-line type is **redeclared locally in each schema module that
needs it**. Canonical shape, found identically in multiple places:

```ts
// 🧰️framework/🛍️products/📓️print/🧬️schema/🟦️.ts:13
export type LocalizedText = { readonly en: string; readonly de: string };

// 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts:33
export type LocalizedText = Readonly<{ en: string; de: string }>;

// 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:154
export interface LocalizedNoticeTextV1 { readonly en: string; readonly de: string }
```

Usage example (hub acceptance-check summaries, `🌎️hub/📦️packages/🟦️typescript/📜️script.ts:234-236`):

```ts
summary: {
  en: `${passed}/${rounds.length} backup/restore rounds pass${first?.error ? `; ${first.error.slice(0, 200)}` : ""}`,
  de: `${passed}/${rounds.length} Runden Sicherung/Wiederherstellung bestanden${first?.error ? `; ${first.error.slice(0, 200)}` : ""}`,
},
```

For **UI-facing app/action labels** (framework `💻️os` plugin registry), the convention is richer — a
`native`/`reuse` terminology axis crossed with locale, generated schema-first from `🎚️axes/🔣️.json` into
(`🧰️framework/🔨️modules/🖱️ui/🎚️axes/📽️projection/🟦️.ts:66-81`):

```ts
export const SHELL_LOCALES = [...] as const;      // e.g. ["en","de"]
export type ShellLocale = (typeof SHELL_LOCALES)[number];
export const SHELL_TERMINOLOGIES = [...] as const; // e.g. ["native","reuse"]
export type ShellTerminology = (typeof SHELL_TERMINOLOGIES)[number];
/** 🗺️ Full locale×terminology matrix for a manifest label, mirroring Rust `LocalizedLabel`. */
export type LocalizedLabel = Readonly<Record<ShellTerminology, Readonly<Record<ShellLocale, string>>>>;
```

and a policy check (`interactivityAllAppLocalizedLabelExact`, `📜️script.ts:9310-9317`) enforces that every
app/action label has BOTH `en` and `de` non-empty under each terminology key and carries **no `default`
field** — i.e. the taxonomy layer actively rejects a "default language" (matching AGENTS.md: "no default
language").

**Recommendation for quiz/teaching:** for plain data/report strings (quiz question text, feedback messages,
proctor status text), declare a local `type LocalizedText = { readonly en: string; readonly de: string }`
next to its schema, exactly like `print`'s `🧬️schema/🟦️.ts`. For UI app/window/action labels registered in a
plugin manifest, reuse the existing `ShellLocale`/`LocalizedLabel` axis machinery instead of inventing a new
one.
