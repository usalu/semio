# T2 — hub + repo-product TypeScript: scoped programs, typecheck targets, debt reduction

Slice T2 (Opus). Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`.
Scope: TypeScript under `🌎️hub/**` and `🧰️framework/🛍️products/🦑️repo/**` (incl. the repo MCP client glue).
Inputs: `📓️audit-typescript-debt.md` (clusters 2, 3, 6), `📓️t1-codegen-corruption.md`, `📓️t4-typescript-os-renderer.md`.
All captures in `🗑️generated/t2-*.txt`.

**Final state (T2c, 2026-09-19 11:00–): both programs are at ZERO owned diagnostics.**
Repo product **507 → 0**, hub **99 → 0** (`t2c-repo-final.txt`, `t2c-hub-final.txt`). The 37 diagnostics
still in the repo-product program and the 94 in the hub program are all foreign — `✏️s/🔌️plugins` (T4b)
and `🧰️framework/🔨️modules` + `💻️os` (T3b/T4b). §§15-21 are the T2c run; §§1-14 are the earlier passes
and their "did not reach zero" statements describe those passes, not the final state.

## 1. Inherited state

`git status --short` on the slice paths at start showed a dead predecessor's uncommitted work, all kept:

| file | what the predecessor had done |
|---|---|
| `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` | moved the ambient `.d.ts` from `include` into `files` |
| `🌎️hub/📦️packages/🟦️typescript/{📋️project.json,📜️script.ts}` | a `lint` target running `tsc -p tsconfig.json` |
| `🧰️framework/…/📚️library/📦️packages/🟦️typescript/tsconfig.json` | `target` ES2022→ES2023, ambient `.d.ts` into `files` |
| `🧰️framework/…/📚️library/🏃️process/🌿️environment/🟦️.d.ts` | +216 lines of curated Bun ambient declarations |
| `.vscode/🧩️launch.seed.jsonc` | rows `📦️check🗄️os-hub🟦️types`, `📦️check🦑️repo📚️library🟦️types` |

No `🗑️generated/t2-*` captures survived and no report existed. No file in either slice path had been
modified in the preceding 90 minutes, so there was no live-peer collision to work around.

**Kept the hand-written Bun ambient file rather than switching to `@types/bun`** (which T1 installed).
Its own docstring proposes its own deletion, but it is deliberately narrowed to the surface this repo
uses — e.g. `Bun.spawn`'s `stdout`/`stderr` are declared non-nullable because every call site passes
`"pipe"`. Swapping in `@types/bun` would re-open ~120 call sites for no type-safety gain and would
conflict with the same `declare namespace Bun`. Extended it instead (§4, F7).

### A naming bug fixed on the way

In this repo `lint` means **eslint / dependency-cruiser** and `typecheck` means **tsc** — see
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/…/📜️script.ts`, which has both (`LintScript` → eslint,
`TypecheckScript` → `tsc --noEmit`). The library's long-standing `lint` target and the predecessor's new
hub `lint` target both ran `tsc`. Both are renamed to `typecheck`, which is also the verb this slice was
asked to deliver. Root `workspace:lint` `dependsOn` `lint` of every project and nx silently skips
projects without that target, so nothing breaks — and `workspace:lint` is now purely the
dependency-cruiser gate it claims to be.

## 2. Deliverables

| thing | where |
|---|---|
| hub scoped tsconfig | `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` (kept, retargeted) |
| hub `typecheck` nx target | `os-hub-ts:typecheck` → `🌎️hub/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` |
| **repo product scoped tsconfig (new)** | `🧰️framework/🛍️products/🦑️repo/tsconfig.json` |
| repo `typecheck` nx target | `@semio-tech/repo-lib:typecheck` → `…/📚️library/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` + `package.json` script |
| launch rows | `.vscode/🧩️launch.seed.jsonc` → regenerated `.vscode/launch.json` (10 rows, §5) |

Both verbs were **run** (`🗑️generated/t2-target-{hub,repo}-typecheck.txt`): each compiles its program and
prints diagnostics with paths relative to the package. They are not yet green, so both exit non-zero.

### Why a product-scoped tsconfig, not the library-scoped one

The pre-existing `📚️library/📦️packages/🟦️typescript/tsconfig.json` covered only `📚️library/**`. The slice
owns all of `🦑️repo/**` — the repo MCP client glue (`💻️client/🔌️mcp/**`, `🔌️mcp/🧪️tests/**`), the
`🖥️server/🎛️coordinator` Next.js app, `💻️client/🧩️vscode`, `⌨️cli`, `🔗️graphql`, `🧪️test`. So a
**product-level** `🦑️repo/tsconfig.json` was added, mirroring T4's `💻️os/tsconfig.json`, and the
`typecheck` target points at it. Widening the scope from `📚️library` to the whole product raised the
owned count from 308 to 507 — 199 previously **unmeasured** errors, not new ones.

### Two programs, because the coordinator is a Next.js app

`🖥️server/🎛️coordinator/📦️packages/🟦️typescript/next-env.d.ts` pulls in Next's global augmentation of
`NodeJS.ProcessEnv`, which makes `NODE_ENV` **required**. Compiled in one program with the repository
tooling that alone produced **120 diagnostic lines** — every `env: { … }` literal passed to `spawnSync`
was rejected (`t2-repo-before.txt` had 0 `ProcessEnv` lines under the library-only scope and 120 under
the naive product scope). The Next package is therefore excluded from the product program and keeps its
own `tsconfig.json` next to its `next.config.ts`; `TypecheckScript` runs **both** programs. The
coordinator's own non-Next sources (`🎛️coordinator/🎫️ticket`, `📡️event`, …) stay in the product program,
and the product tsconfig carries their `@/lib/*` aliases.

## 3. Before / after counts

Command: `bun tsc --noEmit -p <tsconfig> --pretty false`. "Owned" excludes files the import graph drags
in from T3/T4 areas.

### Repo product — `🧰️framework/🛍️products/🦑️repo/tsconfig.json`

| capture | in program | **owned by T2** | what changed |
|---|---:|---:|---|
| `t2-lib-before.txt` | 445 | 308 | old library-only scope, for reference |
| `t2-repo-before.txt` | 651 | **507** | new product scope — the baseline |
| `t2-repo-r1.txt` | 645 | 500 | analyzer-fixture exclude (F4) |
| `t2-repo-r2.txt` | 595 | 451 | `allowJs`, missing deps installed (F1, F2) |
| `t2-repo-r3.txt` | 567 | 423 | typed law-source fixtures (F3) |
| `t2-repo-r5.txt` | 427 | 311 | Next.js program split (§2) + coordinator repairs (F6) |
| `t2-repo-r9.txt` | 350 | 263 | `🔬️workspace-contract` (F5) |
| `t2-repo-final.txt` | 292 | **232** | `⚡️cache-contracts` (F8) |

Non-T2 residue in the final capture: 22 `✏️s` (T4), 20 `💻️os` (T4), 18 `🧰️framework/🔨️modules` (T3).
Coordinator program (`t2-coordinator-final.txt`): 49 total, 31 T2 — 29 of which are the *same*
`📚️library` files already counted above, plus 1 in `🎛️coordinator/🎫️ticket/🟦️.ts`.

### Hub — `🌎️hub/📦️packages/🟦️typescript/tsconfig.json`

| capture | in program | **owned by T2** |
|---|---:|---:|
| `t2-hub-before.txt` | 373 | **99** |
| `t2-hub-r3.txt` | 240 | 84 (Ajv fixture typing, F9) |
| `t2-hub-final.txt` | 228 | **72** (WebCrypto/relay byte typing, F10) |

Hub-owned split at baseline: 94 in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (the cargo + browser-e2e
harness, 16 641 lines), 5 in `🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts`. The other 274 in the
program are T4 os (84), T2 repo-library (32), T4 `✏️s` (23), T3 (17) — reached through imports.
**T1's "hub 800" is not comparable**: it predates T1's own `bun install` settling and this slice's
dependency repairs.

## 4. Fix families

### F1 — missing dev dependencies (−36)

`TS7016`/`TS2307` for `picomatch` (5), `micromatch` (1), `markdown-it` (3) and **`graphql`** (5 plus a
21-error cascade in `🔗️graphql/🧪️tests/📜️sdl-dump/🟦️.ts`). `graphql` was **not installed at all**, yet
five `🔗️graphql` suites import it as the third-party oracle AGENTS.md requires. The other three were
present transitively but undeclared and untyped. Root fix:
`bun add -d graphql picomatch micromatch markdown-it @types/picomatch @types/micromatch @types/markdown-it`
(`t2-bun-add.txt`, 24 packages, lockfile updated).

### F2 — the `.mjs` nx-plugin modules had no types (−~25)

`🦑️repo/🔨️modules/{📚️library,🧪️test}/…/🟨️.mjs` are plain-JS nx plugins (nx loads them directly, so they
cannot become `.ts`). 25 TypeScript call sites imported them and got `TS7016` → `any`. Root fix:
`allowJs: true` in the product tsconfig, so tsc **infers** the real exported types from the JS. No
hand-written declaration files, no `any`; `checkJs` stays off because the JS is not a TypeScript source.
This is strictly stronger typing and it surfaced genuinely new call-site errors that `any` had hidden
(`TS7053` 2 → 18, `TS2345` 89 → 118).

### F3 — untyped JSON fixtures in the `🧱️root-*-source` contract tests (−35)

`const fixture = JSON.parse(readFileSync(…))` types a whole fixture as `any`, cascading into
`Set<unknown>`, `TS7006` implicit-any callback params and `TS2345 unknown → string`. Root fix: a local
`interface …Fixture` per test file mirroring that test's own `🧬️schema/**/🔣️.json`, and
`const fixture: …Fixture = JSON.parse(…)`; the now-redundant inline `(owner: { path: string }) =>`
annotations were removed. Files: `📚️library/🧪️tests/🧱️root-{clean-scaffold, surface-abstraction-law,
artifact-schema-law, inference-law, schema-field}-source/🟦️.ts`. Also in that family:

- `ts.parseJsonText(…).parseDiagnostics` — a real runtime member the public `JsonSourceFile` type omits;
  typed with the intersection this repo already uses in ~10 other files
  (`as ts.JsonSourceFile & { readonly parseDiagnostics: readonly ts.Diagnostic[] }`).
- `new Map<string, string[]>([...owners].map((owner) => [owner, []]))` inferred an array, not a tuple →
  annotated the callback return `[string, string[]]`.

### F4 — deliberately malformed analyzer fixtures excluded (−6)

`🔨️modules/💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/**` holds three `.tsx` samples that are *input data*
for the region analyzer, intentionally re-declaring the same top-level `const`s across sibling
directories (`TS2451` ×6). They are not program source; the product tsconfig excludes that one fixture
tree, exactly as T4's os tsconfig excludes `🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/**`. **This is the
only `.ts`/`.tsx` exclusion this slice added**, and it is called out rather than buried.

### F5 — `🔬️workspace-contract/🟦️.ts`: 85 → 4

The repo's largest contract suite (7 700 lines). Root fixes, in order of yield:

| file:line | defect | fix |
|---|---|---|
| `:188` etc. (12) | `structuredClone(loadTaxonomy())` then mutating a deeply-`readonly` `Taxonomy` | `ProbeTaxonomy = Omit<Taxonomy, 4 registries> & { … mutable … }` + one documented `probeClone` cast; a structured deep copy really is a fresh graph |
| `:134-190` (14) | the fixed-parent vector is `JSON.parse` → `any`, so `parents` built a `{pathPattern: unknown}` record | `interface FixedParentCases` + `Readonly<Record<string, FixedDirectoryContract>>` + `common … as const` |
| `:46,49,123-125` (7) | `@iarna/toml` types every document `AnyJson` | one `readToml<T>()` boundary helper + `CargoWorkspaceManifest` / `CargoMemberManifest` |
| `:1266-1318` (12) | fixture crates `{ shape, ownerRel, pluginId }` miss `PolicyCrateRef`'s `dir`/`libRelPath`/`role` | added the three fields (`policyWindowCompletenessBreaches` reads only `shape`/`pluginId`/`ownerRel`, so behaviour-neutral) |
| `:543-619` (6) | `semanticOwnedInputFileSnapshot(…).bytes` is `Uint8Array`, but `.toString("utf8")` is a `Buffer` method | `Buffer.from(bytes)` at each boundary; `captured` retyped `Map<string, Uint8Array>` |
| `:787` | `import { cleanIsWindowsIllegalName } from "…/📜️script.ts"` — **the symbol does not exist there** | it moved to `📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts` (the very extraction the `root-clean-scaffold-source` contract describes); import repointed |
| `:2186` | `findRepoRoot()` called with no argument | `findRepoRoot(start?: string)` in `📚️library/🏃️process/🧭️routing/🟦️.ts:72` — the body already did `start?.trim() ? start : getWorkspaceRoot()`, so the signature was simply wrong |
| `:3970` | `buildSemanticCensus(…).problems.filter(p => p.kind === …)` — `SemanticProblem` has **`code`**, not `kind`, so this filter matched nothing at runtime | `p.code` |

### F6 — the gutted coordinator scripts (T1 hand-off, in my path)

- `🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts` was left by commit `bb961413d4` with an
  empty `BuildScript` body, no `DevScript`/`StartScript`, no imports and no `router`, while its last line
  still called `runBundleScriptMain(router, …)`. Reconstructed from the pre-corruption version
  (`bb961413d4~1`) **retargeted to Next.js**, which is what the surviving docstrings and
  `📋️project.json` (`dev`/`build`/`start`/`test`) describe: `next build|dev|start` run inside
  `PACKAGE_DIR` via `runBunx`, `TestScript` untouched.
- `🖥️server/🎛️coordinator/📦️packages/🐹️go/📜️script.ts:5` imported the library through **seven** `../`
  when the package sits eight levels deep — the same off-by-one family the hub audit found. Fixed;
  `bun build --target=bun` now resolves both scripts (`t2-build-coordinator{,-go}.txt`, exit 0).
- `…/🟦️typescript/tsconfig.json` sets `incremental: true` with no `tsBuildInfoFile`, so running its
  program dropped a `tsconfig.tsbuildinfo` into the tree; pointed at `node_modules/.cache/` and the
  stray file removed.

### F7 — the curated Bun ambient declarations (`🌿️environment/🟦️.d.ts`)

`BunSpawnOptions.stdin` was `BunStdioMode` only, so `Bun.spawnSync(["git","check-ignore","--stdin"],
{ stdin: Buffer.from(…) })` failed. Bun writes an in-memory body and closes the pipe, so the declaration
now reads `BunStdioMode | ArrayBufferView | ArrayBuffer | Blob`.

### F8 — `⚡️cache-contracts/🟦️.ts`: 31 → 14, by typing one helper

`📚️library/⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🟦️.ts:7` declared
`testNativeInventory(workspace: string, inventory: (root: string) => any): Promise<any>`. Because the
result is `any`, `contracts.find((project) => …)` gives its callback parameter no contextual type at all
— 25 × `TS7006` in the caller. The real type already exists: `inventory()` returns `CacheInventory`.
One signature change (`(root: string) => CacheInventory` / `Promise<CacheInventory>`) removed 17 of the
31, and three `(project: any)` annotations inside the helper went with it. The rest were
`.find()`-possibly-undefined (`!`, the file's own idiom) and four real `(path: string)` annotations.

### F9 — Ajv narrows the validated value to `unknown` (hub, −15)

`new Ajv().compile(schema)` returns `ValidateFunction<unknown>`, which is a **type guard**, so
`if (!validate(fixture))` narrowed `fixture` from `any` to `unknown` — 18 × `TS18046` downstream in
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`. Ajv's `compile` is generic: added
`type BrowserActorChildWorkerFixture` and `type BrowserActorGisDescribeFixture` (mirroring their
`🧬️schema/🔣️.json`) and used `compile<T>(schema)` at the three sites (`:5457`, `:5874`, `:6024`).

### F10 — `Uint8Array<ArrayBufferLike>` is not `BufferSource` (hub, −12)

`crypto.subtle.digest("SHA-256", bytes)` and `new Response(bytes)` require an `ArrayBuffer`-backed view;
`Uint8Array`/`Buffer` default to `ArrayBufferLike`, which admits `SharedArrayBuffer`. Root fixes:
`readLocalRelayResponse` now declares `Promise<Uint8Array<ArrayBuffer>>` (it already built
`new Uint8Array(retained)`), and a single documented `webSha256Hex(bytes)` helper replaced **14**
copies of `Buffer.from(await crypto.subtle.digest("SHA-256", x)).toString("hex")`.

**No `any` was introduced, no `@ts-ignore`/`@ts-expect-error`, no `skipLibCheck` change, and no source
file was excluded to hide an error** (F4 excludes fixture *data*, and is declared).

## 5. Tests run

All captures in `🗑️generated/`. Everything below was executed; nothing is inferred.

### The five `🧱️root-*-source` contract suites — now fully green

Before my launch rows: `6/7`, `8/9`, `10/11`, `7/8`, `3/4` — each failing **only** on
`registers one Bun Nx and seed-derived launch route`, which asserts the target's row exists in
`.vscode/🧩️launch.seed.jsonc` **and** `.vscode/launch.json`. `git show HEAD` proves those rows never
existed (0 `repo-lib:` rows in the committed seed), so the failures were pre-existing, not mine. Each
fixture declares its exact `launchName`/`launchCommand`, so I added the rows and regenerated:

```
root-clean-scaffold-source            exit=0   7 pass  0 fail
root-surface-abstraction-law-source   exit=0   9 pass  0 fail
root-artifact-schema-law-source       exit=0  11 pass  0 fail
root-inference-law-source             exit=0   8 pass  0 fail
root-schema-field-source              exit=0   4 pass  0 fail
```

### Three more ownership gates, same missing-row defect

`repo-source-ownership` 5/7 → **6/7**, `vitest-configuration-ownership` 4/7 → **5/7**,
`tool-configuration-ownership` 6/9 → **7/9**. Each gained its `registers … launch` case from the rows
added for `🧹clean🦑️repo🧪️source-ownership`, `🧹clean🧩️taxonomy🎚️vitest-configuration-ownership`,
`🧹clean🧩️taxonomy🎚️tool-configuration-ownership`. Ten seed rows total; `.vscode/launch.json` regenerated
through `🔌️plugin/📇️registry/📜️script.ts generate` (60 plugin crates, 65 playgrounds, 54 framework
packages) — never hand-edited.

The remaining failures in those three are **unrelated pre-existing defects** outside this slice:
`members-of-modules/🕸️dependencies` taxonomy ancestry naming; `♻️mit-bestand/🧺️demonstrator` importing a
`🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts` that does not exist; `🧰️framework/🧪️tests/🎚️config/🟦️.ts`
effective-root drift; a Playwright "7 tests" count that is now 12.

### `🔬️workspace-contract` — targeted runs over what I edited

`bun test ./…/🔬️workspace-contract/🟦️.ts -t "<names>"`:

- `t2-test-wc-1.txt`: **3 pass** — `preserves the authored extension directory…` (F5 TOML typing),
  `admits only declared parents and one exact leaf…` (F5 `probeClone`/vector typing),
  `rejects Windows-illegal components…` (F5 moved import). 1 fail:
  `clearDiscoveryCache forces a fresh walk` timed out after 5 000 ms (a full-repo walk on a machine
  running the whole fleet) — a file I did not touch.
- `t2-test-wc-2.txt` (`-t "window|mode|packaging"`): 9 pass, 4 fail. The packaging one fails at
  `:4018` — `problems.every(p => p.message.includes(p.path))` — which is the line **before** my single
  `kind`→`code` edit at `:4019`; the other three (artifact-example-model-catalog-projection ×2,
  mode-160000 ancestor) are in tests I never touched.

### Script resolution

`bun build --target=bun` on the two reconstructed coordinator scripts and on the repo-library router:
**exit 0** for all three (`t2-build-{coordinator,coordinator-go,repolib}.txt`). The hub rust router does
not resolve under `bun build` — by design: `:13520-13521` `await import("/controller.js")` and
`"/plugin-runtime.js"` inside a Playwright `page.evaluate` callback are **browser** URLs served by the
dev server, not bundler specifiers (see §6).

### Not run

`⚡️cache-contracts` needs a live Nx project graph (`readCachedProjectGraph`) plus cargo, and the
`🔬️workspace-contract` full file is a multi-thousand-expect cargo-driving suite; under the running fleet
neither is a proportionate gate for type-level edits. **My `⚡️cache-contracts` changes are verified by
typecheck only.** The hub's own `os-hub-ts:test` suite is gated behind `HUB_E2E=1` and builds the real
`os-hub` cargo binary; not run.

## 6. Honest gaps — the 232 + 72 still open

### Repo product — 232, a flat long tail

Top files: `📚️library/🧹️normalization/🟦️.ts` 27, `📚️library/🧪️tests/🔏️path-emoji-statutes` 19,
`⚡️caching/🔒️leases/🧪️tests/🔒️resource-leases` 11, `⚡️caching/🧪️tests/🧊️live-activation` 10,
`🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery` 9, `🧪️tests/❄️frozen-markdown-coordinates` 6,
`💻️client/🧩️vscode/🟦️.ts` 6 — then ~120 files with 1–4 each. Codes: TS2345 50, TS2339 43, TS7006 22,
TS2322 19, TS18046 15, TS7053 12, TS2554 12, TS2769 10.

Two named sub-problems worth a successor's attention:

- **`🧹️normalization/🟦️.ts` (27) is genuine schema drift, not noise.** `TaxonomyRemovalAuthority` is a
  discriminated union whose `caseId` / `generatorContractId` members several writers assume
  unconditionally (`:7650`, `:9210`, `:10965`); `SemanticDescendantNode` vs
  `SemanticDescendantKindNode` at `:5687`; `"file" | "symlink"` compared against `"directory"` at
  `:6916`/`:6929` — that last pair is a **comparison that can never be true at runtime**.
- **`🔬️workspace-contract:3021`** builds a fake taxonomy with a `.schema` member because
  `generatorInputPaths` takes normalization's module-private `LoadedTaxonomy`, while `loadTaxonomy()`
  returns discovery's `Taxonomy` (no `.schema`). Fixing it means exporting `LoadedTaxonomy` + a loader
  from `🧹️normalization`, which is a real API decision, not a type annotation.

### Hub — 72, effectively one file

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` 67: TS18048 "possibly undefined" 23, TS2769 10, TS2339 10,
  TS2345 8, TS18046 6, TS2367 3. The three `TS2367` (`:1713`, `:2219`, `:2228` — *"types `0` and `1`
  have no overlap"*) are **latent logic bugs**, not typing noise, and deserve a reader.
  `:13520-13521` `import("/controller.js")` / `"/plugin-runtime.js"` are browser-served modules inside a
  Playwright `page.evaluate`; the honest fix is an ambient `declare module "/plugin-runtime.js"` for the
  served surface, which I did not write.
- **`🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts` 5 — blocked on T3, deliberately left.** The file
  passes `OwnedBuildPlugin` (the owned Vite abstraction in
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`) straight into vite's
  `defineConfig`. `OwnedBuildConfig` is not structurally compatible with vite's `UserConfig`: chasing it
  gave `assetsInclude: readonly string[]` vs `string | RegExp | (string|RegExp)[]`, then
  `plugins: readonly (OwnedBuildPlugin | …)[]` vs `PluginOption[]` (which admits `undefined`/`false`),
  i.e. re-deriving vite's type inside the owned abstraction — exactly what AGENTS.md's
  "no external types in exported API" rule forbids. **I tried the `assetsInclude` widening, confirmed it
  only moves the error one level down, and reverted it** — that file is T3's and is untouched in the
  final tree. The right fix is for the hub config to be built through `defineOwnedBuildConfig`
  (`🛠️build-tooling/🟦️.ts:112`) instead of vite's `defineConfig`, which needs `react()`/`tailwindcss()`
  to be admitted by the owned plugin type. **Hand-off to T3.**

### Cross-slice blockers on a green target

Neither target can go green on T2's work alone: the repo program still compiles 22 `✏️s` + 20 `💻️os`
(T4) and 18 `🧰️framework/🔨️modules` (T3) files through the import graph, and the hub program compiles
84 `💻️os` + 23 `✏️s` + 17 framework.

### Other honest notes

- `runBunx` exits the process on a non-zero status, so when the product program fails the **coordinator
  program is not reached** in the same `typecheck` run. Fail-fast, but it means a red product hides the
  coordinator's 49.
- `bun nx run @semio-tech/repo-lib:typecheck` / `os-hub-ts:typecheck` were **not** exercised through the
  nx daemon (T1 recorded `NX The daemon timed out while processing REQUEST_PROJECT_GRAPH` under fleet
  load). Both were run through their `📜️script.ts` verbs, which is what the targets invoke.
- I did **not** remove the `[DEBUG]` `console.log`/`console.info` lines that AGENTS.md forbids and that
  sit in `🔬️workspace-contract/🟦️.ts:606,609` and `⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🟦️.ts:30`
  — out of this slice's remit, but they are real and someone should sweep them.
- The three `⌨️cli/🧫️fixtures/🔎️analyzer-paths` `.tsx` files remain unchecked by any program (F4).

## 7. Files changed

Configuration and wiring:
```
🧰️framework/🛍️products/🦑️repo/tsconfig.json                                   NEW — product-scoped program
🧰️framework/…/📚️library/📦️packages/🟦️typescript/📜️script.ts                   TypecheckScript (two programs)
🧰️framework/…/📚️library/📦️packages/🟦️typescript/📋️project.json                lint → typecheck
🧰️framework/…/📚️library/📦️packages/🟦️typescript/package.json                  lint → typecheck script
🧰️framework/…/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json             tsBuildInfoFile
🌎️hub/📦️packages/🟦️typescript/📜️script.ts                                     LintScript → TypecheckScript
🌎️hub/📦️packages/🟦️typescript/📋️project.json                                  lint → typecheck
.vscode/🧩️launch.seed.jsonc                                                   2 renamed + 8 new rows
.vscode/launch.json                                                           regenerated (registry script)
package.json, bun.lock                                                        7 dev dependencies
```

Source:
```
🌎️hub/📦️packages/🦀️rust/📜️script.ts                                           F9, F10 (+220/−…)
🧰️framework/…/📚️library/🏃️process/🧭️routing/🟦️.ts                             findRepoRoot(start?)
🧰️framework/…/📚️library/🏃️process/🌿️environment/🟦️.d.ts                        BunSpawnOptions.stdin
🧰️framework/…/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts                     F5
🧰️framework/…/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts             F3
🧰️framework/…/📚️library/🧪️tests/🧱️root-surface-abstraction-law-source/🟦️.ts    F3
🧰️framework/…/📚️library/🧪️tests/🧱️root-artifact-schema-law-source/🟦️.ts        F3
🧰️framework/…/📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts              F3
🧰️framework/…/📚️library/🧪️tests/🧱️root-schema-field-source/🟦️.ts               F3
🧰️framework/…/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts              F8
🧰️framework/…/📚️library/⚡️caching/📇️inventory/🧪️tests/🕸️coverage/🟦️.ts         F8 (the root signature)
🧰️framework/…/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts       F6 reconstruction
🧰️framework/…/🖥️server/🎛️coordinator/📦️packages/🐹️go/📜️script.ts               F6 import depth
```

Reverted on purpose: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` (T3's; §6).

Captures (`🗑️generated/`): `t2-{hub-before,lib-before,repo-before,repo-probe1,repo-r1…r10,repo-final,
hub-r1…r4,hub-final,coordinator-r1,coordinator-final}.txt`, `t2-bun-add.txt`,
`t2-registry-generate{,2,3}.txt`, `t2-target-{hub,repo}-typecheck.txt`,
`t2-test-{root-*,repo-source-ownership,vitest-configuration-ownership,tool-configuration-ownership}.txt`,
`t2-test-wc-{1,2,fixedparent}.txt`, `t2-build-{coordinator,coordinator-go,repolib,hub-rust}.txt`.

---

# T2b — continuation: drive both targets to zero

Slice T2b (Opus), same ticket. Captures in `🗑️generated/t2b-*.txt`.

## 8. T2b baseline (re-measured 2026-09-19)

| program | total in program | **owned** |
|---|---:|---:|
| repo product `🧰️framework/🛍️products/🦑️repo/tsconfig.json` (`t2b-repo-base.txt`) | 292 | **232** |
| hub `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` (`t2b-hub-base.txt`) | 228 | **72** |

Both full programs run in **10–18 s** under the running fleet, so T2b used the full program as the loop
and did not need T4's scoped-tsconfig trick.

## 9. Hub — 72 → **0** owned diagnostics

| capture | in program | **owned by T2** | what changed |
|---|---:|---:|---|
| `t2b-hub-base.txt` | 228 | **72** | baseline at hand-over |
| `t2b-hub-r1.txt` | 204 | 64 | F11 (`🌐️vite` owned config), F12 (oracle counters) |
| `t2b-hub-r2.txt` | 144 | 11 | F13–F19 |
| `t2b-hub-r3.txt` | 136 | 3 | F20–F22 |
| `t2b-hub-r4.txt` | 133 | **0** | F23 |

The 133 still in the program are entirely T3/T4 files reached through the import graph
(`💻️os` 84, `✏️s` 23, `🧰️framework/🔨️modules` 17, `🦑️repo` 9 — the last being `🧹️normalization`, closed in §10).

### F11 — the flagged `OwnedBuildConfig` vs vite `UserConfig` decision (−5)

`🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts` imported vite's `defineConfig`, `@vitejs/plugin-react`
and `@tailwindcss/vite` directly and handed owned `OwnedBuildPlugin` values to vite's types. T2 tried
widening `OwnedBuildConfig` towards `UserConfig` and reverted it, correctly: AGENTS.md forbids an owned
type from re-deriving an outside one.

**The adapter boundary already exists** — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`
is the single file in the repo that imports vite/vitest/react/tailwind and converts them to the owned
contract (`uiReactBuildPlugin()`, `uiTailwindBuildPlugins()`, `defineOwnedBuildConfig()`), and
`🎨️styling/🏗️builder/🌐️vite/🟦️.ts:1611` is the reference consumer. The hub config was simply not going
through it. Fix: the hub config now imports **only** those three owned helpers, and the file contains no
external import at all. Zero widening of the owned type; the conversion stays in the one adapter.

Verified by the contract that owns this file: `🎚️tool-configuration-ownership`'s
`loads all five Vite owners with the installed native loader and preserves effective roots` loads
`hub-admin-vite` through vite's own `loadConfigFromFile` and asserts its effective root (§11).

### F12 — the three `TS2367` "logic bugs" are a narrowing artifact, not runtime bugs (−3)

`:1720`, `:2245`, `:2254` — *"types `0` and `1` have no overlap"*. The hand-over classified these as
latent logic bugs. **They are not.** `upstreamState.effects` is declared `number` (`:1613`) and is
advanced **only inside the `Bun.serve` fetch handler** (`:1692`). TypeScript's control-flow analysis does
not invalidate a property narrowing across calls, so the preceding assertion `… || upstreamState.effects
!== 0` freezes the field at the literal `0` for every later read in the same block; the *next* literal
comparison then has no overlap. Reproduced in isolation — `🐍️t2b-narrowing-probe.ts`, line 14 errors and
line 15 (the same read through a function) does not.

Root fix: one `observedCount(counter: number): number` reader (`:1615`), applied to **all 36** counter
comparisons in the two relay oracles rather than only the three TypeScript happened to flag — the three
were incidental, and a future edit would have re-frozen a different one. Runtime behaviour is unchanged.

### F13 — a real logic bug: the Ready coordinate that the schema does not have (−2)

`:773-775` read `status.ready.documentId`. `status` is produced by `parseSpaceArtifactCreationStatusJsonV1`
and `🧰️framework/…/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts:82` accepts a Ready payload
**only** when its keys are exactly `artifactId, kindId, artifactSchema, parentDialect` — `documentId` can
never be present. So `!/^artifact-[0-9a-f]{32}$/u.test(status.ready.documentId)` tested the string
`"undefined"` and the guard threw unconditionally, and the returned
`{ …fixture, documentId: status.ready.documentId }` carried `undefined` onward. Fixed to the schema:
`status.ready.artifactId` at both sites. Covered by a new test (§11).

### F14 — `lstatSync`'s last overload (−6)

`new Map<string, ReturnType<typeof lstatSync>>()` resolves `ReturnType` against the **last** overload,
which is the `throwIfNoEntry: false` one returning `Stats | undefined`. Typed `Map<string, Stats>` with
`Stats` imported from `node:fs`; the call sites never pass `throwIfNoEntry`.

### F15 — `assert(artifactRoot?.includes(…))` narrows nothing (−4)

Four sites asserted an optional-chain *expression*, which cannot narrow `artifactRoot`, so every
following `mkdirSync`/`mkdtempSync(join(artifactRoot, …))` was `string | undefined`. Rewritten as
`assert(artifactRoot !== undefined && artifactRoot.includes("🗑️generated"))` — the assertion now says
what it meant.

### F16 — `Uint8Array<ArrayBufferLike>` is not `BodyInit` (−3)

The F10 family again, on the request side: `readLocalRelayBody` builds `new Uint8Array(retained)` but
declared `Uint8Array`, and `checkpointPublicationProcessFixture` builds `Buffer.from(readFileSync(…))`
but declared `Buffer`. Both retyped `…<ArrayBuffer>`, which is exactly what they construct.

### F17 — the dev-server handles (−4)

`let viteServer: { close(): Promise<void> } | undefined` was cast to `{ listen(): Promise<void> }`
(TS2352 — the two shapes do not overlap) and `let vite: { …; listen(): Promise<void>; … }` could not
accept a `ViteDevServer` whose `listen()` resolves to the server. Both owned shapes now declare
`listen(): Promise<unknown>` and the cast is gone.

### F18 — mutable accumulators typed `readonly` (−2)

`receipts` (`runExactCargoLaws` returns `readonly ExactCargoLawReceipt[]`) and `groups`
(`Parameters<typeof runExactCargoLaws>[0]["groups"]`) are both appended to. `receipts` now copies the
result; `groups` is declared `ExactCargoLawGroup[]` (the exported element type) instead of indexing into
the callee's readonly parameter.

### F19 — three shapes that lost a field (−7)

- the browser worker probe state `{ messages: [], errors: [], started: false }` inferred `never[]` and no
  `worker`, so all four writes into it failed — annotated with the shape the probe actually builds;
- `executionTargetLeaseInstall`'s inline parameter type is now the named
  `ExecutionTargetLeaseInstallInput`, and the hostile corpus's `mutated` copy carries it, so the declared
  `missing?: string` branch type-checks;
- `BrowserActorGisDescribeFixture.descriptor` gained `manifest` and `executionProtocol`, and the two
  `structuredClone` working copies are typed `BrowserActorGisDescriptorDraft` (the mutable form) because
  the hostile cases write one field each.

### F20 — the two untyped oracle packages (−4)

`lodash-es/{findIndex,every,sum}.js` and `@webassemblyjs/leb128/lib/leb.js` are third-party oracles with
no reachable types. The repo already owns declarations for the second
(`💻️os/📦️packages/🦀️rust/📐️ambient.d.ts`); the hub program simply did not include it. Added
`🌎️hub/📦️packages/🦀️rust/📐️ambient.d.ts` (NEW — the three `lodash-es` members actually called) and
listed both in the hub `tsconfig.json` `files`. Declared surface only: no package is re-typed wholesale.

### F21 — `Ajv.compile` narrowed a `document.indexed` body to `unknown` (−4)

F9's family, in `proveDocumentIndexFixture`: `validateIndex(body)` inside a `&&` chain narrowed `body`
from `any` to `unknown` for every later read on the same line. Typed `compile<DocumentIndexedEventBodyV1>`,
with the type written from `schema://os.directory/DirectoryEventBody`'s `document.indexed` branch
(`scope`, `descriptorDigestV1`, `entry.name`, `entry.dialect.{artifactKind,standard,subset}`).

### F22 — the dev-server-served browser modules (−2)

`await import("/controller.js")` / `"/plugin-runtime.js"` inside `page.evaluate`. TypeScript classifies a
**rooted** specifier as relative, so no ambient `declare module` can ever match it — the honest fix the
hand-over proposed is not expressible that way. Instead the two surfaces are declared as global
interfaces in the new hub ambient file and applied at the import:
`const served = <T,>(url: string): Promise<T> => import(url) as Promise<T>;`. The call sites are now
typed (`openDirectoryHomeOwnerV1`, `applyDirectoryEventPageBootstrapV1`, `closeDirectoryHomeOwnerV1`,
`setPluginRuntimeActor`, `loadPluginModule`) instead of `any`.

### F23 — patching `postMessage` cannot go through `.call` (−3)

The child-worker containment oracle replaces `Worker.prototype.postMessage` and
`MessagePort.prototype.postMessage`. Both are **overloaded** (`(message, transfer: Transferable[])` and
`(message, options?: StructuredSerializeOptions)`), and `.call` resolves against the *last* overload
only, so forwarding a transfer list was unrepresentable. The replacements now take
`transfer?: Transferable[] | StructuredSerializeOptions` and forward through
`Reflect.apply(native, this, [message, transfer])` — the arguments are passed on untouched, which is what
the oracle needs.

## 10. Repo product — 232 → **156** owned diagnostics

| capture | in program | **owned by T2** | what changed |
|---|---:|---:|---|
| `t2b-repo-base.txt` | 292 | **232** | baseline at hand-over |
| `t2b-repo-r1.txt` | 249 | 211 | F24 (`🧹️normalization` literal + union drift) |
| `t2b-repo-r2.txt` | — | 202 | F24 rest — **`🧹️normalization/🟦️.ts` 27 → 0** |
| `t2b-repo-r3.txt` | — | 194 | F25 (the gutted gate law) |
| `t2b-repo-r5.txt` | — | 176 | F26 (Bun subprocess ambient) |
| `t2b-repo-r7.txt` | — | 165 | F27 (nx-plugin return type) |
| `t2b-repo-final.txt` | 193 | **156** | F28 (contract-union narrowing) |

Non-T2 residue in the final capture: 22 `✏️s`, 15 `🧰️framework/🔨️modules` + `💻️os`.

### F24 — `🧹️normalization/🟦️.ts`: 27 → 0, and what the "union drift" really was (−27)

The hand-over flagged this file as *"real union drift and two comparisons that can never be true"*. Both
were real; here is what each turned out to be.

**The two never-true comparisons are dead branches, and a guard above them proves it.**
`planSymlinkTargetEdits` (`:6912`) already refuses a directory target outright —
`violation("symlink-target-directory-authority-unresolved", …); continue;` — so the later
`targetEntry.nodeKind === "directory" ? { state: "directory" } : …` (`:6916`) and
`windowsLinkType: (targetEntry?.nodeKind === "directory" ? "dir" : "file")` (`:6929`) are unreachable.
`:10934` confirms it from the other side: the verifier throws when `edit.windowsLinkType !== "file"`.
Both branches deleted; `windowsLinkType` is now `"file" as const`. Behaviour identical.

**The union drift is `TaxonomyRemovalAuthority`, and it came from literal narrowing.** Each member pins
literals (`packageId: "wgpu-renderer"`, `status: "closed"`, `contentState: "zero-byte"`,
`disposition: "remove"`, `contractId: "ticket-important-markdown-v1"`). `parseRemovalAuthority` validated
them with `row.packageId !== "wgpu-renderer" || …` compound guards, but `planRecord` returns a
`JsonRecord`, so an equality guard against a `JsonValue` narrows only to `string` — never to the literal.
Three `return result` statements therefore did not satisfy their own union. Root fix: one
`requiredLiteral(value, name, allowed)` helper next to `requiredString`, applied at the six literal
fields (and at the regeneration preview's `nodeKind`, and `packageSourceDispositions.validator`), with
the now-redundant halves of the compound guards removed. Same validation, same error surface, and the
parser's declared type is finally what it checks.

Also in this file:

| site | defect | fix |
|---|---|---|
| `:6917`, `:10935` | `resolveFileKind(path, taxonomy, [], [])` passes an **array** where `parentKindId: string \| undefined` is declared | `undefined`. Behaviour-neutral in fact: the only use is `spec.parentDirectoryKindId !== parentKindId`, and a declared parent kind matches neither `[]` nor `undefined` — but the call was simply wrong |
| `:7650`, `:9210`, `:10965` | `authority.generatorContractId` / `.caseId` read inside a **nested callback**, where the enclosing `kind === …` narrowing is gone | the `retired` filter became a type guard (the idiom this file already uses at `:7663`); the two sentinel sites bind `const authority = entry.authority` before the `.find` |
| `:7256`, `:7289` | `ReferenceInventoryContext` requires `sourceAdmission`, which `repositoryReferenceCandidatePaths` never reads | the parameter is now `Pick<…, "ticketDir" \| "transactionRoots" \| "exactEvidencePaths">` — the three fields the walk actually uses |
| `:1017` | `scopeRow` was `JsonRecord \| ParsedScope`, so five `scopeRow.<field>` reads failed | the named-set branch is projected back to a `JsonRecord`, keeping the existing re-parse path byte-for-byte |
| `:5694` | `matches(node: SemanticDescendantNode)` but its only caller is `contract.requiredNodes: readonly SemanticDescendantKindNode[]`, and its body needs `kindId` | narrowed the parameter to `SemanticDescendantKindNode` |
| `:4011` | `["{", ":"].includes(next)` with `next: string \| undefined` | `next === "{" \|\| next === ":"` |
| `:8446` | `!Number.isSafeInteger(value.revision) \|\| (value.revision as number) < 0` — validated but cast | `typeof value.revision !== "number" \|\| …`; the cast and the downstream `TS2322` are gone |
| `:6205`, `:6710`, `:7431`, `:6932` | `boolean \| undefined` into `boolean`; `row.bytes` unknown behind `Number.isSafeInteger`; a widened `disposition`; a `readonly` result into a mutable field | `Boolean(…)`, a `typeof` guard (which also removed an `as number`), `as const`, one spread |

### F25 — a verification law that was gutted, and is fatal today (−9)

`🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts` reads
`INTERACTIVITY_ALL_APP_REQUIRED_GATES[0].name`, and TypeScript reported
*"Tuple type `readonly []` of length `0` has no element at index `0`"*. It is right:
`📜️script.ts:9003` holds `const INTERACTIVITY_ALL_APP_REQUIRED_GATES = [] as const;`.

`git log -S` names the commit: **`6f33e313da` (2026-09-15)** deleted the six `⚖️gate…` rows from the
constant, the enforcement loop inside `interactivityAllAppLaunchesFromSource`, **and** the matching rows
from `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (0 occurrences of any of the six names in
either file today). The law has no members, no enforcement and no registrations.

The consequence is not cosmetic: **`bun ./📜️script.ts verify interactivity apps` cannot pass.** Run
directly (`🐍️t2b-allapp-selftest.ts`, capture `t2b-allapp-selftest.txt`) the self-tests threw
`[verify interactivity apps] missing gate self-test was falsely accepted` at line 34, because with zero
declared gates the "remove the first gate" fixture produces no failure to match.

Fix applied — and deliberately **not** a restoration: the six rows and the enforcement loop are a
launch-row/verification-gate decision that belongs to the live Z1 and V1 slices, and re-adding the
constant without re-adding the rows would fail the law instead of passing it. So the constant keeps its
single source of truth with a declared element type and a docstring naming the commit that emptied it,
and the self-test states the contract that is actually in force: with a declared gate it still proves the
missing-gate failure; with none it proves a gateless launch file is accepted. After the change the
self-tests **run and return 25** instead of throwing (same capture).

**Hand-off to V1/Z1:** decide whether the six gates come back. If they do, the restoration is
`6f33e313da`'s three deletions in reverse (constant rows, the `for (const gate of …)` loop, the six
`⚖️gate…` launch + seed rows), and the self-test's `firstGate` branch already covers it.

### F26 — the curated Bun ambient declared the wrong subprocess shape (−12)

`🏃️process/🌿️environment/🟦️.d.ts` declared `BunSubprocess.stdin` as `WritableStream<Uint8Array>`, but
`Bun.spawn({ stdin: "pipe" })` hands back an incremental **sink**: 12 `.stdin.write`, 7 `.stdin.end` and
5 `.stdin.flush` call sites in the repo, none of which a `WritableStream` has. `kill` was declared
`kill(code?: number)` while call sites pass `"SIGKILL"`, and `stdout`/`stderr` were plain
`ReadableStream`s although every drain loop is a `for await`. Declared `BunFileSink` and
`BunReadableStream` (which extends `ReadableStream<Uint8Array>` with `[Symbol.asyncIterator]`) and
widened `kill(signal?: number | NodeJS.Signals)`. One ambient file, −12 across the program.

### F27 — the nx plugin's generated targets had no declared shape (−11)

`⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts` indexes
`cacheInternals.playgroundPreparationTargets(…)` by `prepare-<variant>-wgpu-<profile>` and friends.
Under F2's `allowJs`, tsc infers that function's return from the JS and finds exactly one statically
known key, so every dynamic index was `TS7053`. Root fix in the plugin itself
(`📚️library/🟨️.mjs:975`): a JSDoc `@returns
{Record<string, { cache: boolean, continuous?: boolean, dependsOn: string[], outputs: string[], inputs?: unknown[], options: { command: string, forwardAllArgs?: boolean } }>}`
— the generator now declares the contract its consumers read, instead of consumers re-declaring it.

### F28 — contract unions read as if they were one member (−9 so far)

`🧫️cases.json`-backed suites and the taxonomy contract registries: `🔒️resource-leases` gained
`ResourceLeaseFixture`/`ResourceLeaseParticipant`/`ResourceLeaseChild` written from its own
`🧬️schema/🔣️.json`, and `🔏️path-emoji-statutes` now proves the union member it is about to read
(`contractKind === "exact-owner-path-catalog"`, and an `exactDescendantContract()` helper for
`SemanticExactDescendantContract`) instead of indexing the union.

## 11. Tests run (T2b)

Everything below was executed; captures in `🗑️generated/t2b-*`.

### The five `🧱️root-*-source` contract suites — **39/39**, still green after every edit

```
root-clean-scaffold-source            exit=0   7 pass  0 fail
root-surface-abstraction-law-source   exit=0   9 pass  0 fail
root-artifact-schema-law-source       exit=0  11 pass  0 fail
root-inference-law-source             exit=0   8 pass  0 fail
root-schema-field-source              exit=0   4 pass  0 fail
```

Run twice — once after the `🧹️normalization` work and again after the root `📜️script.ts` and Bun-ambient
changes. Two of them need `SEMIO_TEST_ARTIFACT_DIR` (they `mkdtemp` inside it and assert it is truthy);
run without it they fail on their own precondition, which is why they are invoked with
`SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/t2b-artifacts`.

### `verify interactivity apps` self-tests — fatal → **25**

`🐍️t2b-allapp-selftest.ts` → `t2b-allapp-selftest.txt`: `exit=1` with
`[verify interactivity apps] missing gate self-test was falsely accepted` before F25, `exit=0` and
`all-app self-tests = 25` after.

### The hub admin Vite owner — loaded through vite's own native loader

`🎚️tool-configuration-ownership`'s `loads all five Vite owners …` case is the contract that owns this
file, but it **cannot reach** the hub owner: it iterates the five owners in fixture order and dies on
`demonstrator-vite` with `Cannot find module '../../../🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts'` — the
same pre-existing demonstrator defect §5 already recorded (suite result today: 7 pass, 2 fail, both
pre-existing). So F11 is verified directly, with the same call the suite makes
(`🐍️t2b-hub-vite-load.ts` → `t2b-hub-vite-load.txt`, exit 0):

```
hub-admin-vite: root=🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript base=/admin/ plugins=10
hub-admin-vite plugins: semio-emoji-index-html, semio-favicon-serve, semio-favicon-build,
  static-deploy-markers, vite:react-babel, vite:react-refresh, vite:react-virtual-preamble,
  @tailwindcss/vite:scan, @tailwindcss/vite:generate:serve, @tailwindcss/vite:generate:build
```

The effective root and base are unchanged and all ten plugins — including the React and Tailwind ones
now obtained through the owned adapter — materialize exactly as before.

### `🔬️workspace-contract` — targeted runs over what T2b edited

- `-t "symlink"` (`t2b-test-wc-symlink.txt`): **11 pass, 1 fail**. The failure is
  `an exact symlink leaf is inventoried without following target content`, an `ENOENT` while the test
  writes its own `🧪️target/📝️.md` fixture — inventory setup, not planning, and nothing T2b touched.
- `-t "authority"` (`t2b-test-wc-authority.txt`): **22 pass, 20 fail**. Every failure is a missing input
  in the working tree or an environment precondition, none of them type-level: 8 ×
  `Missing or duplicate CAD scenario source identity …`, 2 × `ENOENT` on a `🖍️draw` / `📐️cad` file that
  does not exist, `seed file .vscode/🧩️launch.seed.jsonc is missing the devLaunchers marker`, 4 ×
  missing `⚖️gate`/Draw launch rows (`expected length 1, received 0` — the same registration debt as
  F25), `SEMIO_TEST_ARTIFACT_DIR must remain inside the active ticket generated directory` (that suite
  pins a *different* ticket's folder), and `null is not an object` matching `policyStructuralSource` in
  the root `📜️script.ts`, a function that no longer exists there.
  The CAD/Draw trees are being edited by the live B3d slice right now.
  **This file was not run in full** (341 tests, cargo-driving) under the running fleet.

### Not run

`os-hub-ts:test` is a single `🤝️integration` case gated behind `HUB_E2E=1` that builds the real `os-hub`
cargo binary; this slice is TypeScript-only on a machine saturated by a cargo fleet, so it was not run —
same disposition as §5. No cargo was started by T2b.

## 12. The dependency question — `graphql` + the three `@types`

`bun ./📜️script.ts verify dependencies literal-external` (`t2b-deps-base.txt`) is red repo-wide and has
been for a long time: `target=0, current=236, oracle-conflicts=15, toolchain-owner-conflicts=2`. That is
the whole repository's third-party surface (rust 85, js 117, python 34), not this slice's seven rows.

For the seven rows T2 installed, `verify dependencies list js --raw` (`t2b-deps-list-js.txt`) is
unambiguous — all seven are **dev/test-only and not production-reachable**:

| row | kinds | `productionReachable` | consumers |
|---|---|---|---|
| `graphql ^17.0.2` | `repository-tooling` | **false** | only `🔗️graphql/🧪️tests/{📜️sdl-dump,🔀️variable-coercion,🚫️syntax-errors,📃️document-parsing,▶️query-execution}` — the third-party oracle AGENTS.md requires |
| `picomatch ^4.0.7` | `repository-tooling` | **false** | 6 `🧪️tests` files |
| `micromatch ^4.0.8` | `repository-tooling` | **false** | `🏠️workspace/🧪️tests/🃏️glob-matching` |
| `markdown-it ^15.0.2` | `repository-tooling` | **false** | 4 `🧪️tests` + one probe script |
| `@types/{picomatch,micromatch,markdown-it}` | `repository-tooling` | **false** | type-only |

All seven are in the root `package.json` **`devDependencies`** and every importer is a `🧪️tests` suite or
a `🔬️probes` script — no production module imports any of them.

The gate reports **nothing specific to them**: no `oracle-conflict`, no `toolchain-owner-conflict`, and
`verify dependencies parity js` lists **zero** undeclared imports under `🌎️hub/**` or
`🧰️framework/🛍️products/🦑️repo/**` and zero for these four packages (its 1332 undeclared imports are all
in other products). They appear only in the `unownedRows` bucket — the root manifest has no root-scope
source evidence for them — alongside 19 pre-existing root rows (`@types/node`, `eslint`, `playwright`,
`ajv-formats`, `esbuild`, `react-dom`, …); that bucket is opt-in (`--no-unowned-rows`) and is not part of
the default gate.

**One real defect found, deliberately not changed:** the root declares `markdown-it ^15.0.2` while its
two owning packages declare `14.3.0` exactly (`@semio-tech/repo-lib`, `@semio-tech/print`), so `bun.lock`
carries **two** major versions, and `@types/markdown-it@14.2.0` types the v14 API. The consumers all sit
inside the two v14 packages, so nothing is mistyped today, and `lock-mismatches=0`. Aligning the root row
to `14.3.0` requires a `bun install` that rewrites `bun.lock` — on a machine where a dozen peers are
mid-edit that is a worse trade than the drift, so it is **reported, not applied**. One-line fix for a
calm window: root `package.json` `"markdown-it": "14.3.0"` + `bun install`.

## 13. Honest gaps after T2b

- **Hub: 0 owned.** `os-hub-ts:typecheck` still exits non-zero because the hub program compiles 106
  diagnostics from `💻️os`, `✏️s` and `🧰️framework/🔨️modules` through the import graph. Nothing left in
  `🌎️hub/**`.
- **Repo product: 156 owned**, down from 232. Top remaining: `🔏️path-emoji-statutes` 10,
  `❄️frozen-markdown-coordinates` 6, `💻️client/🧩️vscode` 6, `🧪️test/🧪️tests/🧪️test-platform` 5, then
  ~100 files with 1–4. Codes: TS2339 28, TS2345 24, TS18046 14, TS2554 12, TS7006 11, TS2322 11,
  TS2769 10, TS7031 9. The shape of the remainder is the F3/F28 family (untyped `JSON.parse` fixtures and
  contract unions read as one member) plus `🎛️coordinator`/`⌨️cli` script drift; no further *logic* bugs
  were found in what was read.
- **The three hub `TS2367`s were not logic bugs** (F12) — the hand-over's classification is corrected,
  with a standalone reproduction rather than an assertion.
- **The regression gate for both real bugs is the typecheck target itself.** F13 (`status.ready.documentId`)
  and the `resolveFileKind(…, [], …)` argument are both re-caught the moment they are reintroduced, by
  `os-hub-ts:typecheck` and `@semio-tech/repo-lib:typecheck` respectively — which is exactly why they
  survived until now: nothing ran those programs. F25 has a runtime gate instead (`verify interactivity
  apps`), which is why its fix is proven by execution and not by types.
- `🔬️workspace-contract` was not run in full, and `os-hub-ts:test` was not run at all (both cargo-bound).
- No `any` blanket, no `@ts-ignore`/`@ts-expect-error`, no `skipLibCheck` change and no source exclusion
  was added by T2b.

## 14. Files changed (T2b)

```
🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts                              F11 — owned build config only
🌎️hub/📦️packages/🦀️rust/📜️script.ts                                          F12–F13, F14–F23
🌎️hub/📦️packages/🦀️rust/📐️ambient.d.ts                                       NEW — lodash-es + served browser surfaces
🌎️hub/📦️packages/🟦️typescript/tsconfig.json                                   files: + two ambient declarations
📜️script.ts                                                                   F25 — gate law type + docstring, launch type export
🧰️framework/…/📚️library/🧹️normalization/🟦️.ts                                F24 — 27 → 0
🧰️framework/…/📚️library/🏃️process/🌿️environment/🟦️.d.ts                       F26 — BunFileSink, BunReadableStream, kill
🧰️framework/…/📚️library/🟨️.mjs                                                F27 — playgroundPreparationTargets @returns
🧰️framework/…/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts             F27 consumer
🧰️framework/…/📚️library/⚡️caching/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts    F28
🧰️framework/…/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts                   F28
🧰️framework/…/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts  F25 consumer
```

Ticket scripts: `🐍️t2b-narrowing-probe.ts` (F12 reproduction), `🐍️t2b-hub-vite-load.ts` (F11
verification), `🐍️t2b-allapp-selftest.ts` (F25 verification).
Captures: `🗑️generated/t2b-{repo-base,repo-r1…r8,repo-final,hub-base,hub-r1…r4,hub-final}.txt`,
`t2b-deps-{base,list-js,parity,parity.json}.txt`, `t2b-test-root-*.txt`, `t2b-test-wc-{symlink,authority}.txt`,
`t2b-test-tool-config-ownership.txt`, `t2b-hub-vite-load.txt`, `t2b-allapp-selftest.txt`.

---

# T2c — continuation: repo product 156 → 0, framework-program T2 residue, markdown-it alignment

Slice T2c (Opus), same ticket. Captures in `🗑️generated/t2c-*.txt`.

## 15. T2c baseline (2026-09-19)

| program | total | **owned by T2** |
|---|---:|---:|
| repo product `🧰️framework/🛍️products/🦑️repo/tsconfig.json` (`t2c-repo-base.txt`) | 193 | **156** |

Foreign residue at baseline: `✏️s` 22 (T4b), `🧰️framework/🔨️modules` + `💻️os` 15 (T3b/T4b).
Owned codes: TS2339 28, TS2345 24, TS18046 14, TS2554 12, TS7006 11, TS2322 11, TS2769 10,
TS7031 9, TS2305 4, TS2349 4, then a tail.

Loop: the whole repo-product program runs in **10.7 s** under the fleet, so T2c used the full
program (`bun tsc --noEmit -p … --pretty false` + `🐍️t2c-count.py`) rather than `🐍️t3b-scope.py`'s
per-file config — the per-file trick is only worth it for the ~32 s framework program.

## 16. Repo product — 156 → 30 so far

| capture | in program | **owned by T2** | what changed |
|---|---:|---:|---|
| `t2c-repo-base.txt` | 193 | **156** | baseline at hand-over |
| `t2c-repo-r1.txt` | 156 | 119 | F29 (Ajv `AnySchema`), F30 (TypeScript compiler API), F31 (signature drift), F32 (`Uint8Array.toString`) |
| `t2c-repo-r2.txt` | 133 | 96 | F33 (`process.env` spreads), F34 (Ajv type-guard narrowing, round 3) |
| `t2c-repo-r3.txt` | 121 | 84 | F35 (readable streams are async iterable), F36 (`Bun.Transpiler` target) |
| `t2c-repo-r4.txt` | 116 | 79 | F37 (the VS Code extension's corrupted codegen + lifecycle boxes) |
| `t2c-repo-r5.txt` | 113 | 76 | F38 (probe taxonomy), F39 (fixture array typo) |
| `t2c-repo-r6.txt` | 93 | 56 | F40 (contract parameters narrowed to what they read), F41 (a fixture class that does not exist) |
| `t2c-repo-r7.txt` | 84 | 45 | F42 (the coordinator's second off-by-one import), F43 (JSON-Schema document shape) |
| `t2c-repo-r8.txt` | 80 | 41 | F44 (the nx plugins' `createNodesV2` result shape) |
| `t2c-repo-r9.txt` | 70 | **30** | F45 (`Bun.JSONC.parse` consumers), F46 (remaining implicit-any parameters) |

| `t2c-repo-r10.txt` | 59 | 22 | F47 (`🔍️discovery` narrowing gaps), F48 (a validating boundary that could not be called wrongly) |
| `t2c-repo-r11.txt` | 54 | 17 | F49 (`bun:test`'s `mock.module`), F50 (a dead oracle clause) |
| `t2c-repo-r12.txt` | 50 | 13 | F51 (a self-test registry completed instead of the reader narrowed) |
| `t2c-repo-r13.txt` | 46 | 9 | F52 (deep-mutable probe projection), F53 (rollup output narrowing) |
| `t2c-repo-r15.txt` | 39 | 2 | F54 (the `LoadedTaxonomy` API decision) |
| `t2c-repo-r18.txt` | **37** | **0** | F55 (two competing `markdown-it` `Token` types), F56 (a stale fixture annotation) |

The 37 that remain are **all foreign**: `✏️s/🔌️plugins` 22 (T4b) and `🧰️framework/🔨️modules` + `💻️os` 15
(T3b/T4b). `🐍️t2c-count.py` splits owned from foreign on every capture.

The hub program was re-measured on resume (`t2c-hub-r1.txt`): one **new** owned diagnostic had appeared
since T2b closed it at zero — a peer made `LocalHubRun.publicSessionIssuance` required and
`🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts:231`'s probe run did not declare it. Added
`publicSessionIssuance: false` (the pipe-only development default the field documents);
`t2c-hub-r2.txt` is **0 hub-owned** again (94 foreign, all `🧰️framework` + `✏️s`).

## 17. T2c fix families

**F29 — Ajv schemas are `AnySchema`, not `object`.** `new Ajv().compile(x)`/`validate(schema, x)` take
`AnySchema`; a `JSON.parse(...) as Record<string, unknown>` does not satisfy it. Fixed by declaring what
each caller actually reads instead of widening: the five adapter suites
(`📄️ticket-document-codec`, `📄️goal-document-codec`, `🔣️json-encoding-conformance`,
`📝️todo-markdown-roundtrip`, `🪪️contributor-identity-parse`) now parse their schema as
`{ readonly $schema?: string; readonly $defs: Readonly<Record<string, unknown>> }` — the two members
they hand to `ajv.compile`.

**F30 — the TypeScript compiler API surface.** `ts.getModifiers` needs a `HasModifiers`
(`📦️package-boundary-classification:461` now guards with `ts.canHaveModifiers`); a `CallExpression`'s
callee is a `LeftHandSideExpression`, so `:357` reads the identifier the line above already narrowed
(`callee.text`) instead of re-reaching through `expression.expression`.

**F31 — signature drift.** `dependencyClassifyOracleEntry` lost its registry parameter; the two
`🔬️dependency-truth` call sites still passed `{}` in third position. Dropped.

**F32 — `Uint8Array.toString(encoding)` is not a thing.** Nine `ctx.fixtureBytes(uri).toString("utf8")`
sites across the five `🔗️graphql` suites are now `Buffer.from(ctx.fixtureBytes(uri)).toString("utf8")`.
`AdapterContext.fixtureBytes` returns `Uint8Array`, whose `toString` takes no argument — the old spelling
silently produced `"1,2,3,…"` under any runtime that is not Node's Buffer subclass.

**F33 — spreading `process.env` into a literal drops its index signature.** `{ ...process.env, X: y }`
infers `{ X: string }`, so every later `delete env.NO_COLOR` / `env[key]` failed. Four call sites
(`🧊️wasm-outputs` ×2, `🔒️trunk-lockfile`, `🖥️services`) now annotate `NodeJS.ProcessEnv`.

**F34 — Ajv validators narrow `any` to `unknown` (round 3).** `!ajv.compile(schema)(value)` is a type
predicate, so the rest of the `||` chain sees `unknown`. Fixed by naming the payload:
`ReviewedFixtureAuthority` (both `readme-current-source-*` suites, now the type of the `const` itself so
the whole module sees it), `GlobFixture` (`🃏️glob`), `TransactionProcessObservation`
(`⚙️transaction-process-ownership`, mirroring its own `🧬️schema/🔣️.json`).

**F35 — every readable stream this runtime hands out is async iterable.** The DOM lib does not say so,
so `for await (const chunk of response.body)` and `of child.stdout` both failed. The curated ambient now
declares `interface ReadableStream<R> { [Symbol.asyncIterator](): AsyncIterableIterator<R> }` once, which
is true of Bun and of Node ≥18; `BunReadableStream` stays as the byte-typed child-pipe alias. Verified it
introduces no declaration conflict in the framework program (`grep ReadableStream` on a full run: empty).

**F36 — `Bun.Transpiler` accepts a `target`.** Added to the ambient constructor options.

**F37 — the VS Code extension could not load.** `💻️client/🧩️vscode/🟦️.ts:866-884` held a corrupted
`🧬️CodegenGql` block (T1's codemod family): overload signatures with spliced string literals, one
swallowed by a docstring, all typed against a `documents` map that **does not exist anywhere in the
repo** — so `graphql()`, called at module top level by fifteen `⌛️Queries` constants, threw
`ReferenceError: documents is not defined` on activation. Replaced the whole block with the honest
declaration (`graphql(source: string): string` returning its own source, which is what the repo CLI's
`graphql --query` is given). Separately, four `ephemeralBox<T>(key, undefined)` globals lied about their
type; they are now `ephemeralBox<T | undefined>` with two lifecycle accessors (`repoDiagnostics()`,
`kitDiagnostics()`) that fail loudly if read before `activate`, instead of four `!`.

**F38 — `structuredClone`-then-mutate probes.** `🏺️historical-package-owner-identity` gained the
`Mutable`/`ProbeTaxonomy`/`probeClone` trio `🔬️workspace-contract` already uses, opening exactly
`generatorContracts` and `frozenCoordinateEvidenceContracts`.

**F39 — a missing `[]`.** `🔬️marker-only-folders:11` declared `trees: readonly Readonly<{…}>` for what is
an array; three separate diagnostics (TS1354/TS2537/TS2488) were that one typo.

**F40 — contract parameters narrowed to what they read.** `matchesTarget`/`matchesFixture`/`matchesRow`
and `ratchetDependencies` each read a handful of fields; their tests build minimal rows. Declaring
`Pick<…>` (plus `Partial<Pick<…>>` on `matchesRow`, whose selectors are individually optional) makes the
real signature honest rather than forcing the tests to fabricate whole manifests. The one place where the
reverse was right — `validateCaseContract`, whose call graph consults most of the registry — the
**test** was completed instead (F51).

**F41 — a fixture class that does not exist.** `🧱️command-composition-source` selected on
`"canonical"`; `FIXTURE_CLASSES` is `real-world | handcrafted | third-party-generated`. The test now uses
`handcrafted` in all four places, so it exercises a value the domain can actually produce.

**F42 — the coordinator's second off-by-one import.** T2 fixed the Go router's depth; the Rust sibling
`🎛️coordinator/📦️packages/🦀️rust/📜️script.ts:3` still had seven `../` for an eight-deep package, which
made `BundleScript` an error type and `this.repoRoot` vanish in all three script classes. Also
`⌨️cli/📦️packages/🦀️rust/📜️script.ts` called `runCmd` without importing it — a `ReferenceError` on
`semio mcp …`.

**F43 — JSON-Schema document shape** — see F29. The coordinator's `🔬️schema` suite reads `$schema` off a
parsed document whose annotation omitted it.

**F44 — the nx plugins' `createNodesV2` result.** `export default { createNodesV2: [glob, fn] }` in a
`.mjs` infers `(string | fn)[]`, so `createNodesV2[1](…)` is "not callable" and its result is `unknown`
at four suites. Both plugins (`🧪️test/🟨️.mjs`, `📚️library/🟨️.mjs`) now carry `GeneratedProject` /
`GeneratedNodes` JSDoc typedefs and a `@type` tuple annotation.

**F45 — `Bun.JSONC.parse` returns `unknown`** (correctly). `🚀️runtime-bootstrap` now parses the
devcontainer once into a declared `DevcontainerConfig` instead of re-parsing it four times;
`🔒️trunk-lockfile` names the launch-configuration shape it filters.

**F46 — implicit-any parameters** from `any`-typed fixtures: `paths` is `string[]` in the three
`🧱️*-command-source` suites (which unblocked their `.map((diagnostic) => …)`), plus
`🖥️services`'s `row`/`stream` and `🌐️service-readiness`'s `url`.

**F47 — `🔍️discovery` narrowing gaps.** `implementationLeafPolicy.roles.includes(fileKind.role)` compared
a scoped-kind role union against a narrower policy union (`(… as readonly string[]).includes(…)`, the
file's own idiom); three `EcmaRouteExpression` member/property reads follow the discriminated-union
convention the surrounding code already uses.

**F48 — a validating boundary that could not be called wrongly.**
`registryCompilerInputDependencies(source, path, role)` declared `role: RegistryCompilerInputRole` and
then threw for anything else — so its runtime guard was unreachable by type and
`🌐️registry-import-language`'s negative case could not be written. The parameter is now `string` with an
`isRegistryCompilerInputRole` predicate, which makes the guard live and the roles still closed.

**F49 — `bun:test`'s `mock.module`.** Declared in the curated ambient as a namespace member of `mock`,
plus `kind` on Bun's `onResolve` plugin args (`🪢️cargo-provider-binding`, `🌐️registry-import-language`).

**F50 — a dead oracle clause.** `🧪️test/🟦️.ts:7303` filtered on
`requirement.oracle === undefined || oracle.id === requirement.oracle`. `OracleRequirement` has no
`oracle` member and `🧬️schema/🔣️.json`'s `OracleRequirement` is `additionalProperties: false` with
`capability`/`qualifyingKind`/`distinctEngineFamilies` — so the clause was always `true`. Removed;
behaviour is bit-identical and the filter now says what it does.

**F51 — a self-test registry completed.** `🧪️test-platform:443` built a six-field registry for
`validateCaseContract`, whose call graph (`profileTable`, `mutationCoverageBreaches`,
`caseAboveSubsetBreaches`) consults most of an `OracleRegistry`. The literal now declares every
collection (empty where the case does not use it) and is annotated with the real type — the file's three
other `as unknown as OracleRegistry` casts are pre-existing and left alone.

**F52 — a deep-mutable probe projection.** `💠️inventory-artifact-shards` cast a published shard set to a
hand-written `{ manifest: Record<string, any>; … }` that no longer overlapped it. Replaced with
`DeepMutable<TaxonomyInventoryArtifactShards>`, which also let the cast back to `built` disappear.

**F53 — rollup output narrowing.** `vite`'s `build()` returns `RollupOutput | RollupOutput[] |
RollupWatcher`; `🎚️tool-configuration-ownership` now narrows with `"output" in result` and picks CSS
assets with a `flatMap` instead of a `filter` that could not narrow.

**F54 — the `LoadedTaxonomy` API decision** (flagged as open by T2 §6). `generatorInputPaths` took the
walk's private `LoadedTaxonomy`, so `🔬️workspace-contract:3035` hand-built one by spreading discovery's
unrelated `Taxonomy` — which has no `discoverySchema`/`pathMatcher`, i.e. the test was passing `undefined`
for both of the values the function reads. Fixed on the production side: `🧹️normalization` now exports
`GeneratorInputTaxonomy` (`discoverySchema | pathMatcher | exclusions`) and `loadNormalizationTaxonomy`,
and `isExcluded`/`generatorNodeRecord`/`compilerInputRecords` take `Pick<LoadedTaxonomy, "exclusions">`.
The test asks for the real thing. **This is a runtime behaviour change** — see §18.

**F55 — two competing `markdown-it` `Token` types.** markdown-it 15.0.2 ships its own typings
(`dist/markdown-it.d.cts`) and a stale `@types/markdown-it` is also installed;
`🥒️gherkin-description-inline-code` annotated against the latter, whose `Token.attrs` is
`[string, string][]` where the real one is `TokenAttribute[]`. The annotation now uses
`import("markdown-it").Token`. **The duplicate `@types/markdown-it` dependency is a real defect left
open** — removing it is a lockfile change and belongs to whoever owns the dependency gate.

**F56 — a stale fixture annotation.** `🔬️workspace-contract:7754` typed the schema-rust-entries cases
with `exports: string[]`; both the fixture and `🧬️schema/🧬️schema-rust-entries/🔣️.json` declare an object
map of `{ file, facet }`, which is also what `SchemaCatalogScope.exports` is.

## 18. Deletion and behaviour changes T2c made

**One file deleted.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🟦️typescript/📜️script.ts`
imported four symbols — `buildRepoCliBin`, `resolveRepoImplementation`, `REPO_GO_CLI_DIR`,
`REPO_RUST_CLI_CRATE` — that **exist nowhere in the repository** (`grep -rn` over `*.ts`: zero hits
outside that file). Its directory held no `package.json` and no `📋️project.json` while its `🐹️go` and
`🦀️rust` siblings have both; the root `package.json` `workspaces` does not list it; no fixture,
manifestless-source closure or contract references the path. It was an unregistered, unbuildable
duplicate of the live `💻️client/⌨️cli/📦️packages/🟦️typescript` package. Four TS2305 diagnostics.

Behaviour changes worth naming, all verified in §19:
1. **F54** — `🔬️workspace-contract`'s compiler-manifest test now passes a real loaded taxonomy where it
   previously passed an object missing both fields the function reads.
2. **F37** — the VS Code extension module no longer throws on load.
3. **F42** — `semio mcp …` and the coordinator's Rust router now resolve their imports.
4. **F31/F50/F41/F51/F56** — argument, clause, literal and annotation corrections inside test suites.

## 19. Tests run (T2c)

Everything below was executed; captures in `🗑️generated/t2c-tests-{1,2,3,4}.txt`,
`t2c-probe.txt`, `t2c-vscode-load.txt`. `bun test` needs a `./`-prefixed path or it treats the emoji
path as a name filter and silently matches nothing (`t2c-tests-1.txt`'s first run).

### The two riskiest behaviour changes — green

| test | result |
|---|---|
| `🔬️workspace-contract` — `discovers only exact byte-matched compiler manifest inputs` (**F54**) | **1 pass** |
| `🔬️workspace-contract` — `cross-checks the Rust export registry dump against the catalog` (**F56**) | **1 pass** |

### Full suites

| suite | result | are the failures T2c's? |
|---|---|---|
| `💠️inventory-artifact-shards` | **8 pass, 0 fail** | — |
| `🔬️marker-only-folders` | **3 pass, 0 fail** | — |
| `repo-source-ownership` (route) | 6 pass, 1 fail | no — byte-identical failure name to T2's own `t2-test-repo-source-ownership.txt` baseline, so the deleted orphan router changed nothing |
| `manifestless-source-closure` (route) | 8 pass, 1 fail | no — `registers the closure gate in both editor launch authorities`, the launch-row family T2 §5 documented; the closure itself did **not** complain about the deletion |
| `path-emoji-statutes` (route) | 30 pass, 7 fail | no — failures sit at lines 86/122/147/302/393/573/754; T2c's edits are at 18-27, 415-420 and 662, and the two tests containing them (`mutation catalogs declare one canonical implementation and fixture bundle pair`, `taxonomy accepts single keycaps…`) are not in the failure list |
| `cargo-transaction-command-source` (route) | 8 pass, 2 fail | no — T2c's only edit here is the type annotation `paths: string[]`, which emits nothing |
| `root-artifact-dependency-source` (route) | 2 pass, 3 fail | no — the three fail at :46 (Ajv rejects a fixture whose `wgpuOutputRoots[0].producer` the schema forbids), :100 (a `preview-generated` subprocess) and :128 (launch rows). T2c's edit is in `exportedNames`, used only at :63 inside a **passing** test |
| `🧱️command-composition-source` | 9 pass, 1 fail | no — `registers the ordinary Bun Nx launch route…`, the launch-row family; the test T2c edited (`keeps selector behavior implementation-neutral`, **F40/F41**) passes |
| `❄️frozen-markdown-coordinates` | 34 pass, 2 fail | no — the registration test aborts at `:92`, where the live taxonomy now carries **441** frozen-markdown contracts against the fixture's ten; T2c's `coordinates ?? []` is at `:118`, never reached |
| `🏺️historical-package-owner-identity` | 25 pass, 1 fail | no — `:179` receives 110 live `taxonomy/kind-only-basename` findings for `🦀️component.rs` leaves under `✏️s/🔌️plugins/🖍️draw`, an in-flight peer rename; **F38** is a type-level projection that emits nothing |
| `🧪️test-platform` | 90 pass, 20 fail (462 s) | not established suite-wide — no pre-change baseline was taken (each run is ~8 min of a loaded fleet). The **four tests T2c touched were re-run in isolation: 4 pass, 0 fail** (`t2c-tests-4.txt`). The 20 failures are whole categories that read the live registry and repository (`oracle purity`, `discovery and contract`, `cross-language oracle hosts`, `recorded production debt`, …), none of them T2c's three ratchet cases or the `differential-without-evidence` case |

### Entry points whose own suites are gated behind a live Nx graph or cargo

`🐍️t2c-probe.ts` calls them directly (`t2c-probe.txt`, **exit 0**):

```
PASS 🃏️glob verifyFixtureGlobOracle                      (F34)
PASS 🚀️runtime-bootstrap testContainerRuntimeBootstrap   (F45)
PASS 🧩️host-build testExtensionHostBuild                 (the vite `configFile: false as const`)
```

### Script resolution and the extension's module load

`bun build --target=bun`, **exit 0** for all three routers T2c repaired or touched:
`💻️client/🧩️vscode/📦️packages/🟦️typescript`, `⌨️cli/📦️packages/🦀️rust`,
`🖥️server/🎛️coordinator/📦️packages/🦀️rust` (**F42**).

**F37 proved at runtime.** `🐍️t2c-vscode-load.ts` registers a `Bun.plugin` virtual `vscode` module
generated from the 37 `vscode.*` members the extension reaches for (`🐍️t2c-vscode-host.mjs`) and imports
the extension:

```
exports=35 queryDocuments=11 graphql=function
  AnalyzeDocument: string chars=492
  BundlesDocument: string chars=165
  ContributorsDocument: string chars=990
```

The eleven `⌛️Queries` constants now evaluate. Against the pre-fix source the same load throws
`ReferenceError: documents is not defined` at the first of them.

### Not run

- `⚡️cache-contracts` as a whole (needs a live Nx project graph plus cargo) — its one member T2c edited,
  `🚀️runtime-bootstrap`, was executed directly instead.
- `🔒️trunk-lockfile`, `🖥️services`, `🧊️wasm-outputs`, `🌐️service-readiness`, `🧬️generator-ownership`,
  `🦀️inputs`, `⚡️exhaustive-cache-inputs`, `📐️test-layout`, `🧲️rust-physical-reference-context`,
  `🪢️cargo-provider-binding`, `🪶️artifact-empty-facet-*`, `🔄️transaction-v2`,
  `⚙️transaction-process-ownership`, `🔗️graphql`'s five suites, `🪪️field-parity`,
  `📦️package-boundary-classification`, the `readme-current-source-*` pair, `🧬️mutation-fixtures`,
  `🎚️tool-configuration-ownership`. Each needed cargo, trunk, a live Nx graph or a full-repo walk under a
  loaded fleet. **Their edits are verified by typecheck only**; all but three are pure annotations that
  emit nothing. The three with runtime content are `🥒️gherkin-description-inline-code` (run: 6 pass /
  2 fail, both pre-existing — a missing `✏️s/🔌️plugins/🗄️stdio/…/mutate-svg-1-1-basic/🥒️.feature` and a
  launch-route assertion still spelling the pre-migration `🧪️tests/🟦️gherkin-description-inline-code.ts`),
  `🧬️mutation-fixtures` (`if (source === undefined) continue`, over keys `delete` already removed) and
  `❄️frozen-markdown-coordinates` (run above).
- The hub's own `os-hub-ts:test` — gated behind `HUB_E2E=1` and a real `os-hub` cargo binary.

## 20. Honest gaps after T2c

1. **`@types/markdown-it` is a duplicate of markdown-it 15's own typings** and they disagree
   (`Token.attrs`). F55 works around it at the one call site. Removing the dependency is a lockfile
   change and belongs with the dependency gate, not here.
2. **`🧪️test-platform` is 20/110 red** and T2c did not establish whether that predates this slice. The
   failing categories all read live repository state, and the four tests T2c touched pass in isolation.
   It deserves its own slice.
3. **Launch-row assertions** keep failing in `manifestless-source-closure`,
   `cargo-transaction-command-source`, `root-artifact-dependency-source` and
   `🧱️command-composition-source` — the same family T2 §5 fixed for five other suites by adding seed rows.
   T2c did not extend `.vscode/🧩️launch.seed.jsonc` further: the slice's mandate was the type debt, and
   each row needs its fixture's exact `launchName`/`launchCommand`.
4. **Live repository drift** surfaced by two suites and left for their owners: the frozen-markdown
   evidence registry has 441 contracts against a ten-contract fixture, and `✏️s/🔌️plugins/🖍️draw` has 110
   `🦀️component.rs` leaves the taxonomy wants spelled `🦀️.rs`.
5. `🔬️workspace-contract:3038`'s sandbox probe now loads the repository's real taxonomy
   (`loadNormalizationTaxonomy({ repoRoot })`) rather than the sandbox's; that is what the old code
   intended and what makes the assertion meaningful, but it does mean the test reads outside its sandbox.

## 21. Files changed (T2c)

147 paths under `🧰️framework/🛍️products/🦑️repo/**` and `🌎️hub/**`. The ones that are not a
single-line annotation:

| file | change |
|---|---|
| `…/⌨️cli/📦️packages/🟦️typescript/📜️script.ts` | **deleted** (§18) |
| `…/⌨️cli/📦️packages/🦀️rust/📜️script.ts` | `runCmd` added to the import list |
| `…/🖥️server/🎛️coordinator/📦️packages/🦀️rust/📜️script.ts` | import depth 7 → 8 |
| `…/🖥️server/📚️library/🗄️persistence/🟦️.ts` | `insertTicketFiles(… files: readonly string[])` |
| `…/💻️client/🧩️vscode/🟦️.ts` | corrupted `🧬️CodegenGql` block replaced; four `ephemeralBox` globals + two lifecycle accessors |
| `…/💻️client/🧩️vscode/🏗️builder/🟦️.ts` | `configFile: false as const` |
| `…/📚️library/🏃️process/🌿️environment/🟦️.d.ts` | `ReadableStream[Symbol.asyncIterator]`, `Transpiler.target`, `onResolve` `kind`, `mock.module` |
| `…/📚️library/🔍️discovery/🟦️.ts` | `isRegistryCompilerInputRole` boundary; four narrowing gaps |
| `…/📚️library/🧹️normalization/🟦️.ts` | `GeneratorInputTaxonomy` + `loadNormalizationTaxonomy` exported; `isExcluded`/`generatorNodeRecord`/`compilerInputRecords` narrowed |
| `…/📚️library/🟨️.mjs`, `…/🧪️test/🟨️.mjs` | `GeneratedProject`/`GeneratedNodes` typedefs + `createNodesV2` tuple |
| `…/🧪️test/🟦️.ts` | dead oracle clause removed; `ratchetDependencies` registry narrowed |
| `…/🧪️test/🔍️discovery/🎛️selection/🟦️.ts` | three matcher parameters narrowed to what they read |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` | `publicSessionIssuance: false` on the probe run |

Ticket-folder scripts added: `🐍️t2c-count.py` (owned/foreign split), `🐍️t2c-ts-api.py` (the F30 codemod),
`🐍️t2c-probe.ts`, `🐍️t2c-vscode-load.ts` + `🐍️t2c-vscode-host.mjs`.
