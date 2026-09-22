# TS2 — os product TypeScript 0 + vitest green

Slice TS2, session 8 (2026-09-22, started 17:21 CEST). Inputs: `📓️worker-preamble.md`,
`📓️ts1-os-typescript-zero-and-offline-resilience.md` (§1.1 the four remaining TS errors, §3 the six
vitest projects, §3.1 the `.ralph-tui` coordinator item), `📓️status.md` tail (Session 8).
Foreground only, no sub-agents.

Machine at start: load 14.02 / 22.45 / 41.75, **96 GiB free** on `/System/Volumes/Data`,
wasm mutex held by `tc3e` since 16:56.

## 0. Baseline (measured)

`cd 🧰️framework/🛍️products/💻️os && bun x tsc -p tsconfig.json --noEmit`
→ `🗑️generated/ts2-typecheck-000-baseline.txt`, **4 `error TS…` diagnostics, exit 1** — byte-identical
to TS1 §1.1's four (puzzle `BoardSession`, editor `EditorSession`, the vite builder config, the
cold-pair double).

## 1. TypeScript **4 → 0** ✅

| capture | count |
|---|---|
| `ts2-typecheck-000-baseline.txt` | **4** |
| after §1.2 (vite builder config) | **3** |
| after §1.3 (cold-pair double) | **2** |
| after §1.1 (the two `pkg/` rebuilds) | **0**, exit 0 (`ts2-typecheck-final.txt`, 20:53 CEST) |

### 1.1 The two stale `wasm-pack` `pkg/` artifacts — **REBUILT**

Both are gitignored `wasm-pack` outputs whose `.d.ts` predated the Rust's own
`#[wasm_bindgen(js_name = pointerCancelScreen)]`, so the shell's pointer-cancel path called a method
the loaded module did not have — **a live defect, not only a type one**.

The real build verb is the same in both crates (`📋️project.json` target `wasm` →
`bun ./📜️script.ts wasm` → `runWasmPackWebBuild({ shipProfile: "wasm-release" })`). Both ran as ONE
ordered fleet-mutex hold (`📜️mutex-ordered.sh 20260922171000 ts2`, wrapper `📜️ts2-wasm-rebuild.sh`,
`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `CARGO_INCREMENTAL=0`), 4th in the queue behind s11/tc3e/nb1:

| crate | was | now | build |
|---|---|---|---|
| `semio-s-plugin-puzzle` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`) | `pkg/` **2026-09-17 03:45**, `semio_puzzle.d.ts` 24 077 B | **2026-09-22 17:54**, 27 094 B, `semio_puzzle_bg.wasm` 70 988 507 B | rc 0 |
| `semio-framework-editor` (`🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust`) | `pkg/` **2026-09-13 04:50**, `framework_editor.d.ts` 15 024 B | **2026-09-22 17:55**, 15 146 B, `framework_editor_bg.wasm` 27 127 578 B (25.87 MiB), 34.2 s | rc 0 |

`grep -c pointerCancelScreen` on the two `.d.ts`: **2 and 2** (was 0 and 0) — the declaration and its
use in the session interface. Capture `🗑️generated/ts2-wasm-rebuild.txt`.

### 1.2 The vite builder config — **LANDED, 1 → 0 diagnostics in that file**

The dev server's own configuration (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`)
sat on **two** boundaries at once: it imported vite's `defineConfig`, vite's `react()` and
`@tailwindcss/vite` directly, while every other plugin in its `plugins` array comes from the repo's own
`OwnedBuildPlugin` factories (`🖱️ui/🎨️styling/🏗️builder/🌐️vite`, `🧑‍💻dev/🔌️vite-plugins`). TS1 measured
that annotating the return `Promise<UserConfig>` turns the one diagnostic into **16**; TS2 reproduced
that exactly (19 project-wide, 16 in the file) before choosing a side.

**It is now wholly on the owned boundary** — `🖱️ui/🎯️targets/⚛️react/🛠️build-tooling`, whose docstring
already states its purpose ("without exporting Vitest or Vite types"). No `any`, no `@ts-ignore`, no
cast in the config itself; the vite import is gone:

- `defineConfig(async ({command}) => …)` → `defineOwnedBuildConfigFactory(async ({command}): Promise<OwnedBuildConfig> => …)`;
- `react()` → `uiReactBuildPlugin()`, `tailwindcss()` → `uiTailwindBuildPlugins()` (the boundary's own
  documented adapters, which already existed and were simply not used here).

That exposed **four real gaps in the owned contract** — each an honest expansion to the shape the
build tool actually hands the hook, not a weakening (the residual errors named them one at a time):

| owned declaration | was | is | why |
|---|---|---|---|
| `OwnedBuildServer.watcher` | absent | `{ emit(event: string, ...args: readonly unknown[]): boolean }` | `semioSourceFreshnessVitePlugins` drives `server.watcher.emit` — the server really has it |
| `OwnedBuildServer.httpServer` | absent | `{ once(event: string, listener: () => void): void } \| null` | the rendezvous and backbone SSE plugins dispose on `close`; the real server hands `null` in middleware mode |
| `OwnedBuildServer.config` | `{ root, cacheDir }` | `+ base: string`, `+ server: { hmr?: unknown }` | the host-HTML plugin reads `server.config.base` and `server.config.server.hmr` |
| `OwnedBuildEnvironment` | `{ command, mode }` | `+ isSsrBuild?`, `+ isPreview?` | the distribution build passes all four |
| `OwnedBuildConfig.worker` | `Record<string, unknown>` | `{ format?: "es" \| "iife"; plugins?: () => readonly (OwnedBuildPlugin \| readonly OwnedBuildPlugin[])[]; [key: string]: unknown }` | the distribution build calls `config.worker.plugins()` |

One product bug fell out of it: `semioAgentBridgeRendezvousVitePlugin`
(`🧑‍💻dev/🔌️vite-plugins/🟦️.ts:1205`) declared `httpServer?: { once… }` **without `| null`**, while the
build server passes `null` whenever it runs in middleware mode — the plugin would then never register
its `close` disposer. Declared `| null` now, matching its sibling at line 711 which already had it.

The one place the two descriptions genuinely meet is
`🧑‍💻dev/🚚️distribution/📋️plan/🟦️.ts`, which `await import("vite")`s and hands the production config to
vite's own `build()`. That crossing is now a single named, documented function `buildToolConfig()`
with the reason written out — the same pattern the owned boundary already uses in
`uiTailwindBuildPlugins()`.

### 1.3 The cold-pair double — **LANDED as a real double, 363/363 still green**

`🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`'s `installVerifiedDocumentBackbonePair` set
`state.verifiedColdPair = { assertCurrent() {}, drop() {} }` — an inert stand-in for
`VerifiedColdDocumentPair`, a nominal class with a private mint token (`🏪️store/👷️worker/🟦️.ts:894`).
Six laws installed it, and its no-op `assertCurrent()` made `documentBackboneAdmissionReady`
(`:3431`) unconditionally true, so none of those six ever exercised the retention contract they
depend on.

**It now mints the real thing.** The worker already exports `VerifiedColdDocumentPair`,
`verifiedColdDocumentPairMintToken`, `DocumentExecutionTargetLease` and
`documentExecutionTargetLeaseMintToken` into the test module (`:6511`), and the same file already
mints a real pair in the gis-inference suite — the offline-resilience helper simply had not. It now
builds, from the state's own hub binding, a real `DocumentExecutionTargetLease`
(`closed-browser-actor`, checkpoint digests matching the bootstrap, scope = the document's own
space/id/schema, `hubOrigin` = the binding's base url) and a real `WireArtifactBootstrap`, assigns
`state.executionTargetLease`, and constructs the pair through the mint token. No `as`, no interface
extraction, the mint invariant untouched.

Minting the real owner turned two of the six laws red, and **both reds were the laws lying about
production order**, not the product:

| law | what the real owner refused | re-expressed as | contract line |
|---|---|---|---|
| `keeps two concurrent document grant actors isolated …` | installed the pair **before** `flushSocketGrantTurns()`, i.e. before a socket existed; the owner captures `state.socket` and `assertCurrent()` refuses a changed one | installs after both sockets open, which is when a cold transfer really happens | `🏪️store/👷️worker/🟦️.ts:971` `this.state.socket !== this.socket` |
| `a mutation made while offline is queued, then flushed once the hub reconnects` | installed at `head_edit_id: ""`, then fed a `Welcome` whose `server_frontier` is `head_edit_id: "e0"`; the owner is bound to the baseline it was transferred at | the helper takes the baseline, and the law mints at `"e0"` — the frontier its own `Welcome` confirms | `🏪️store/👷️worker/🟦️.ts:978` `!equalFrontiers(this.state.frontier, …)` |

Nothing was weakened: both laws still assert exactly what they asserted before, and they now run
against an owner that can refuse them.

`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript` → `bun ./📜️script.ts test quick`:
**6 files, 363 / 363 passed, exit 0** (`🗑️generated/ts2-vitest-framework-os-002.txt`, 17:39:22, 9.97 s).
The intermediate run with the real owner and the two stale laws was **361 passed / 2 failed**
(`ts2-vitest-framework-os-001.txt`) — proof that the double is no longer inert.

## 2. `.ralph-tui` — taxonomy no longer requires a third-party tool's directory — **LANDED**

### 2.1 What actually threw

`loadTaxonomy()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1396`)
runs `validateGeneratorContractsAgainstWorkspace`, whose line **6245** is

```
if (output.inclusion === "tracked" && !existsSync(join(root, output.path)) && … ) problems.push(`generatorContracts[…] tracked output […] is missing.`)
```

The taxonomy declared a generator contract `setup-wizard-config` (`ownership: "external"`,
`ownerPath: null`, `target: null`) whose **seven** `outputRoots` were the seven `.ralph-tui/**` files,
each `inclusion: "tracked"`. The user deleted `.ralph-tui` (staged-deleted, 7 files) together with
`.kiro`, `.windsurf`, `.copilot` and `.factory` — every third-party AI tool's state directory. So
`loadTaxonomy()` threw seven "tracked output … is missing" problems for every caller. Restoring it
with git is forbidden (the deletion was deliberate), so the contract was the defect.

### 2.2 The root fix

**The taxonomy no longer declares a third-party tool's own state directory as a repository output at
all.** Measured first: `.ralph-tui` was the ONLY foreign tool directory with a generator contract —
`.kiro`, `.codex`, `.copilot`, `.cursor`, `.factory`, `.windsurf`, `.claude`, `.cargo`, `.config`,
`.github`, `.devcontainer`, `.agents` all carry only *path-identity* `fixedDirectoryContracts`
(authority "Kiro", "Codex", …), which never require existence. `.ralph-tui` was the anomaly.

- `🔣️taxonomy.json` — the whole `setup-wizard-config` block removed (37 lines, 28393–28429).
  `generatorContracts` **23 → 22**; the JSON reparses. The `ralph-*` `fixedFilenameContracts` /
  `fixedDirectoryContracts` are **kept**: they are path identity (they exempt those paths from the
  repo's emoji naming policy if the tool is ever run again), exactly like `.kiro`'s, and they impose
  no existence requirement.
- `🔍️discovery/🟦️.ts` — the Ralph-specific law is replaced by a **general** one, so the class of bug
  cannot recur for any foreign tool:
  - the seven-path `ralphTrackedPaths` array and the
    `generatorContracts.setup-wizard-config must externally own exactly the seven tracked Ralph files`
    check are gone;
  - `foreignMetadataRoots` is now derived from the taxonomy itself — every `fixedDirectoryContracts`
    entry whose `scope.kind === "repository-root"` and whose `pathPattern` is a root-level dot
    directory (13 of them today);
  - new law: **no generator contract may declare an output inside a foreign metadata directory** —
    `generatorContracts "…" declares output "…" inside the foreign metadata directory "…" owned by
    "…"; a foreign tool's own state is never a repository output.`
  - the `Ralph contract … must not use a recursive wildcard` law is generalised to all foreign
    metadata roots (measured: zero existing contracts violate it).

  Checked that this does not catch the one legitimate dot-path generator output,
  `plugin-registry → .vscode/launch.json`: `.vscode` has no root-level directory contract (its
  contract is `**/.vscode`), so it is not a foreign metadata root.

### 2.3 Proof

```
bun …/lt.ts  →  loadTaxonomy OK; generatorContracts 22
```

Three laws in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
were re-expressed against the new contract (none weakened — the Ralph-surface law became a law over
**every** foreign metadata root, and the "incomplete Ralph" negative case became "a generator contract
that declares an output inside a foreign metadata directory is rejected"):

| law | result |
|---|---|
| `closes generator ownership and keeps foreign metadata directories out of generator outputs` | **1 pass, 0 fail**, 789 expect() calls |
| `rejects missing, external, and non-canonical generator preview targets` (now sampling `external-cargo-locks`) | **1 pass, 0 fail** |
| `rejects unsettled, broad foreign, foreign-owning, and false root generation contracts` | **1 pass, 0 fail** |

`bun test … -t "generat"` over that file: **20 tests, 17 pass, 3 fail**. The three reds are one
unrelated pre-existing cause — the normalization sandbox fixture does not contain
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (added 2026-09-19 04:13),
so `registryCatalogInputPaths` throws `Registry catalog content input is missing or a symlink`.
That file exists on disk; the fixture is stale. It is not `.ralph-tui`, not TS2's, and outside the os
product's seven vitest projects — named here so it is not invisible.

## 3. The seven vitest projects

| project | verb | TS1 | TS2 | capture |
|---|---|---|---|---|
| `@semio-tech/framework-os` | `test quick` | 363/363 | **363 / 363, exit 0** | `ts2-vitest-framework-os-002.txt` |
| `@semio-tech/framework-renderer-react` | `SEMIO_TEST_LEVEL=long` vitest | 1949/1955 | **1955 / 1955, 98/98 files, exit 0** | `ts2-vitest-renderer-react-final.txt` |
| `@semio-tech/framework-renderer-wgpu` | `test long --no-fail-fast` | **0 ran** | **1371 run, 1369 pass, 2 fail** | `ts2-vitest-renderer-wgpu-long.txt` |
| `@semio-tech/framework-os-mcp` | `test quick` | 45/50, 8 red files | **50 / 50, 7/7 files, exit 0** | `ts2-vitest-os-mcp-002.txt` |
| `@semio-tech/framework-os-shell` | `test quick` | 7/7 | **7 / 7, 2/2 files, exit 0** | `ts2-vitest-os-shell.txt` |
| `@semio-tech/plugin-registry` | `test long` | 60/60 | **60 / 60, 8/8 files, exit 0** | `ts2-vitest-plugin-registry-003.txt` |
| `@semio-tech/framework-os-dev` | direct vitest, `SEMIO_TEST_LEVEL=long` | 68/88 in one file | **175 / 195, 3/5 files, 18 red** | `ts2-vitest-os-dev-002.txt` |

### 3.1 `framework-renderer-wgpu` — the verb runs again: **0 → 1371 tests**

TS1 measured **zero** laws: the whole `test` verb died in `loadTaxonomy()` (`runCargoTestBudgeted` →
`resolveCargoPackageName` → `getCargoWorkspaceIndex` → `loadTaxonomy`). With §2 landed it runs.

- `bun ./📜️script.ts test quick` → the run STARTS (`Starting 1371 tests across 1 binary`) and
  fail-fasts at test 39; with `--no-fail-fast` the **quick level's own 30 s budget kills it**
  (`[budget] … exceeded 30000ms — killed`) before the corpus finishes.
- `bun ./📜️script.ts test long --no-fail-fast` →
  **`Summary [33.289s] 1371 tests run: 1369 passed, 2 failed, 0 skipped`.**

The two reds are both owned elsewhere and named here rather than papered over:

1. `agent_bridge::tests::every_gateway_to_shell_fixture_round_trips_through_this_codec` —
   `AgentReply did not decode: UnknownTag(10)`. AC1 added `GatewayToShell::AgentReply` (tag 10), its
   TS twin and the shared fixtures, and `📓️status.md` records the **wgpu twin was handed to the WGPU
   parity ticket**. The law already demands all ten tags
   (`🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs:56`); the wgpu codec
   (`🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs:393-398`) still stops at tag 9.
2. `shell::display_conflicts_marketplace_tests::conflict_resolution_buttons_are_inline_controls_before_row_selection`
   — the conflicts tab now emits **3** `TreeItem` records where the fixture declares
   `expected.rowCount` **1** (`🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs:309`).
   A wgpu shell-chrome drift, same parity ticket.

### 3.2 `framework-os-mcp` — 45/50 and 8 red files → **50 / 50, 7 / 7 files**

TS1 named six causes. One had already been fixed by a peer by the time TS2 measured
(`serverInfo.name`: `🏛️legacy-conformance` ×2 and `🌅️modern-era` were green at 20:58, baseline
`ts2-vitest-os-mcp-000.txt` = **48 / 50, 6 red files**). The other five are root-fixed:

| TS1 cause | root fix |
|---|---|
| 1 `🎚️config/🟦️.ts` collected as a suite | the project's include glob is `🧪️tests/*/🟦️.ts` and its own config lives at `🧪️tests/🎚️config/` — the config is now excluded, beside the `🧪️resolvemcpbinarypath` exclusion that already existed |
| 2 `💬️agent-reply` + `🤖️live-agent-loop`: `paths[0] must be of type string, got undefined` | these are **live gates, not vitest suites** — shebanged `#!/usr/bin/env bun` scripts with their own `*-check` Nx targets and `.vscode/launch.json` rows in group `4_gate`, which execute at import time, need a running serve handed to them by environment, and end in `process.exit`. The glob swept them in. Excluded. |
| 3 `🤖️hub-agent-participant`: `1 red row(s): 0 a hub answers /readyz` | same class (no `describe`/`it`, ends in `process.exit(0)`, needs a hub on `:7631`). Excluded. |
| 4 `serverInfo.name` ×3 | already green when TS2 measured; not TS2's |
| 5 `tools/list is the full 26-tool gateway surface` | the census is **deliberately duplicated** in the law ("this suite is an INDEPENDENT observer … so it must not import the value it is checking"), and the product grew to 28: `inference_run` and AC1's `conversation_reply`. The independent list now carries both **in the Rust's own order** (`🌉️mcp/🦀️.rs:282` `GATEWAY_TOOL_NAMES: [&str; 28]`), the law's name says 28, and the two stale "26 tools" docstrings (`🦀️.rs:441`, the suite header) were corrected. |
| 6 `the hub binary registers [6] … not the four routes this client calls` | the hub grew a sixth gis-map route, `…/inference/gis-map/jobs/reconcile`. The oracle greps the hub binary's `.route(` registrations and demands an EXACT set, which is the right strength — so the new route is named in the census (with why the client does not call it) rather than the comparison being loosened, `report.routes` 5 → 6, and the stale "four routes" wording now prints the real count. |

Excluding a gate does not stop it gating: all three keep their own `*-check` targets and launch rows,
which is where AC1 ran `⚖️gate🌉️os-mcp💬️agent-reply` 9/9 against a live serve.

### 3.3 `plugin-registry` — a red that arrived DURING this slice, root-fixed

`plugin-registry`'s `test long` was 59/60 when TS2 first ran it at 21:03 (TS1 had measured 60/60):
`🧪️tests/🚀️launch/🟦️.ts › WASI codegen profile policy` found an **eleventh** `[profile.wasm-dev.package.*]`
override in `Cargo.toml` — `semio-s-plugin-norm = { strip = "symbols" }`, landed by the NB1 slice at
~17:5x to get `📕️norm`'s wasm-dev component under `FRESH_COMPONENT_MAX_BYTES`. The declaration was in
`Cargo.toml` and nowhere else, so two owned artifacts had not followed:

- `🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` — the policy census the law compares
  `Cargo.toml` against. The entry is declared (one key, compact form preserved).
- `🧑‍💻dev/🧬️schema/🔣️.json` `$defs.WasiProfilePolicyV1` — every override was **required** to carry
  `opt-level` and nothing else, so simply declaring the fixture entry failed the AJV gate next
  (`🚀️launch/🟦️.ts:95`). The shape is now honest and still CLOSED: `additionalProperties: false`,
  `minProperties: 1`, and `strip` admitted as Cargo's own closed enum
  (`none` / `debuginfo` / `symbols`). No override may be empty and no unknown key is allowed.

### 3.4 `framework-renderer-react` — 1949 → **1955 / 1955**

Three separate causes, all root-fixed:

- **The six `🧩️package-integration` reds** were the `.ralph-tui` taxonomy throw (§2). That file alone
  is now **23 / 23** (`ts2-vitest-renderer-react-pkgint.txt`). No change to the law.
- **`📨️browser-frame-transport` › `refuses an aborted stream continuation before mutating its returned
  response owner`** (not in TS1's count; deterministic, and present in `HEAD` unchanged). The product
  is right: `assertBrowserAssetResponseContinuation` calls `controller.signal.throwIfAborted()`, and
  `ownerCalls` proves it threw. The law asserted `error instanceof DOMException` — but this suite runs
  under an emulated DOM whose `globalThis.DOMException` is a **different realm** from the host
  runtime's `AbortController`, so `instanceof` is false for the very object the contract mints. The
  law now pins the **name** (`error.name !== "AbortError"`), which is what the web contract actually
  specifies. **46 / 46.**
- **`🔬️engine-contract` › `retires a mounted graph gesture without synthesizing an up or graph
  commit`** (also not in TS1's count, also unchanged in `HEAD`). The law's own fixture made its
  assertion unreachable: hover is published to an interaction DOMAIN, and
  `nodeGraphHoverActionArgs` (`🧱️elements/🕸️NodeGraph/🟦️.tsx:252`) returns `undefined` when the
  scene declares none — the fixture declared none, so `["interactionHover"]` could never be observed
  no matter what the host did. The scene now carries its `interactionDomain`; every other assertion in
  the law (one `pointerCancelScreen`, no `pointerUpScreen`, no graph commit, a following pointer-down
  still reaches the session) is untouched and now measures something. **669 / 669.**

### 3.5 `framework-os-dev` — 19 → **18 red**, every one named (the only project NOT green)

This is the project TS1 unblocked (`🧪️ticket-owned-browser-host-staging` had never loaded at all) and
whose reds it named without fixing. TS2 measured the whole project, not one file:
**5 files, 195 tests, 175 passed, 18 failed, 2 skipped** (`🧪️ticket-owned-browser-host-staging`
88 tests / 16 red / 2 skipped, `🧹️config` 50 tests / 3 red — one of those three TS2 fixed).

**Fixed by TS2 (2, both the same class as §3.3's NB1 drift):**

- `WASI codegen profile policy › keeps single-CGU mitigation WASI-only …` asserted
  `manifest.profile["wasm-dev"]` `toEqual({ inherits: "dev", "codegen-units": 1 })` — an equality that
  has been false since the FIRST `[profile.wasm-dev.package.*]` override landed (there are eleven).
  The law's subject is that the single-CGU mitigation is the PROFILE's and no package's, so it now
  destructures the overrides out, asserts the profile exactly as before, and asserts what "WASI-only"
  actually means: **no per-package override may re-state `codegen-units`**. The exact census of those
  overrides stays where it belongs, in `plugin-registry`'s own law (§3.3).
- One `bun:sqlite` bundler refusal: TS1 had named the specifier through `const BUN_SQLITE_MODULE =
  "bun:sqlite" as const` plus `@vite-ignore`, but a `const` holding a string literal is exactly what
  the analyser constant-folds back into a static specifier. It is now assembled at runtime
  (`["bun", "sqlite"].join(":")`) with the module's real type restored in a TYPE position the bundler
  never sees. That cleared the analysis-time failure; see below for what remains.

**The 18 that remain, by cause — none is TS2's and none is a TypeScript or taxonomy fault:**

| # | cause | laws |
|---|---|---|
| 4 | `TypeError: reactor.stageColdPairPage is not a function` — the generated jco bridge (`bridge.js:144`) calls a reactor export the fixture reactor does not have | `pluginComponentBridgeSource` ×2, `rewriteJcoComponentAssetUrls` ×2 |
| 4 | jco/worker dispatcher drift (`Invalid URL`, `The URL must be of scheme file`, a versioned-component helper the emitted source no longer contains) | `rewriteJcoComponentAssetUrls` ×2, `PluginComponentInstantiation`, `ticket-owned browser host staging › matches the neutral schema …` |
| 4 | deployed vendor transport — `ENOENT` on a staged vendor file, `missing production function contentTypeForStaticDirAsset` (the law transpiles that function out of `🖱️ui/🎨️styling/🟦️.ts` by AST lookup and it is no longer a `FunctionDeclaration` there), and `Registry import discovery requires Bun's compiler runtime` reached through `🔍️discovery`'s `registryStaticImports` | `deployed vendor transport` ×4 |
| 2 | `bun:sqlite` at RUNTIME, not analysis: the `long` level runs under **jsdom**, so Vite resolves this graph as a client one and refuses the Bun builtin when `backboneDbHandleFor` actually calls it. TS2 tried `test.server.deps.external: [/^bun:/]` — **measured, it changes nothing**, so the declaration was removed rather than left as decoration. The honest fix is either running these two laws in the `node` environment or a Vite environment-level external, and it belongs to whoever owns that level split. | `backboneDbHandleFor` ×2 |
| 2 | pre-existing config-graph budget drift, unchanged by TS2's vite rewrite: **58 modules vs a declared bound of 40**, and the Bun-bundler cross-check disagrees by **5** (39 vs 44). Both numbers are byte-identical to TS1's, measured before and after the owned-boundary conversion. | `vite config module graph` ×2 |
| 1 | `dev server transform freshness … atomic save` — a live-server timing law | ×1 |
| 1 | `catalog state transition probe` | ×1 |

Two `console.log("[DEBUG] …")` lines survive in these laws
(`🧹️config/🟦️.ts:125`, `🧪️ticket-owned-browser-host-staging/🟦️.ts:1455`) against the repo rule that
forbids them. They are not TS2's and were left alone.

## 4. Runtime proof on the TS2 `s` serve (6199)

Every TS2 change that can affect runtime was taken to a live shell. Serve started by TS2 alone, on
its own port, detached, `SEMIO_VITE_HMR=0`, pid recorded, killed afterwards:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
nohup env S_OS_PORT=6199 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev \
  > 🗑️generated/ts2-serve-s-6199-b.txt 2>&1 & disown      # pid in 🗑️generated/ts2-serve-pid.txt
```

1. **The rewritten dev config is the one vite loads, and it starts.** This is the load-bearing proof
   for §1.2: `🏗️builder/🌐️vite/🟦️.ts` IS the configuration, `🔌️vite-plugins/🟦️.ts` is a module vite
   imports while starting, and both were rewritten. `VITE v7.3.6 ready in 2472 ms`,
   `http://127.0.0.1:6199/` answers **200**. (The first serve, before the last two edits, answered the
   same at `ready in 2320…2721 ms`.)
2. **The shell boots and paints its whole chrome** on the fully-edited tree
   (`🗑️generated/ts2-boot-diagnose.json`): `document.title` = `semio · s · home`, body carries
   `Artifact / Chat / Fullscreen / Editor ⌘️⌥️E / Viewer ⌘️⌥️V / Studios / Create Space / Actions /
   Utilities / Display / Remote: detached / No one else is here / signed out / Sign in / Settings /
   Marketplace / History / Tasks / Command`, root `DIV#root` present.
3. **The two rebuilt wasm packages are LIVE on that server, with the missing method.** Curling the
   served modules through the dev server's own `/@fs` route:

   | served module | `grep -c pointerCancelScreen` |
   |---|---|
   | `…/✍️editor/📦️packages/🦀️rust/pkg/framework_editor.js` | **2** |
   | `…/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js` | **2** |

   Before the rebuild those files were from 2026-09-13 and 2026-09-17 and had **0**. This closes the
   live half of §1.1: the shell's pointer-cancel path now finds the method on the module the running
   server hands the browser.

The serve was killed by its recorded pid when this section was written.

## 5. Honest gaps

1. **`framework-os-dev` is the one project not green: 175 / 195, 18 red.** Every one is categorised
   with its cause in §3.5. None is a TypeScript or taxonomy fault, none was introduced by TS2, and
   the two TS2 could reach at their root are fixed. The largest block (8 laws) is jco-bridge /
   generated-worker drift owned by the plugin-runtime slices.
2. **`framework-renderer-wgpu` is 1369 / 1371, not green** (§3.1). The verb was resurrected from
   **zero**; the two survivors are (a) the `AgentReply` tag-10 wgpu twin that `📓️status.md` records as
   **handed to the WGPU parity ticket**, and (b) a wgpu shell-chrome row-count drift in the same
   ticket's area. TS2 did not take either: they are Rust in another ticket's declared scope.
3. **The wgpu `quick` level cannot finish its own corpus.** `test quick` fail-fasts, and with
   `--no-fail-fast` its 30 s budget kills the run (`[budget] … exceeded 30000ms — killed`) while the
   same corpus finishes in **33.3 s** at `long`. Either the budget or the level assignment is wrong;
   that is a decision for whoever owns the levels. Same class as TS1's §3.3 finding for
   `plugin-registry` and `framework-os-dev`.
4. **Two runtime changes are proven at runtime, one is not.** §4 proves the rewritten vite config
   boots a live server and the rebuilt wasm glue is served with `pointerCancelScreen`. What is NOT
   proven live is a pointer-cancel actually reaching the puzzle/editor session in a mounted board:
   that needs a plugin activation (`activate-s-react-dev`), and the serve's own freshness report
   names ten plugins as stale. The method's presence on the served module is the half TS2 could
   measure without taking the machine for a re-activation.
5. **Three laws were re-expressed, none weakened**, each citing the contract line it now measures:
   two in `🧪️space-artifact-creation-owner` (§1.3) and one in `🔬️engine-contract` (§3.4). A fourth,
   in `📨️browser-frame-transport`, had its assertion made realm-safe rather than changed in subject.
   Three census-style laws were updated to the product's real surface (§3.2 tools 26→28 and the
   sixth inference route, §3.3 the eleventh profile override, §3.5 the wasm-dev profile).
6. **Three repo-product laws outside the os product fail for one unrelated, pre-existing cause**
   (§2.3): the normalization sandbox fixture does not contain
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`. TS2 named it rather than
   touching it.
7. **A cut at ~18:00 (usage limit, resumed 20:51)** landed in the middle of this slice. Nothing was
   lost: the wasm rebuild had finished at 17:55 and both `pkg/` trees survived, and the serve on 6199
   was still alive and was verified as TS2's by its own pid file before being reused.

## 6. Files changed

**Product (runtime behaviour):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` — the dev server's own
  configuration moved wholly onto the owned build boundary (§1.2); vite's `defineConfig`, `react()`
  and `tailwindcss()` imports removed.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` (+ its `🟦️.js` CJS mirror) —
  `defineOwnedBuildConfigFactory` / `OwnedBuildConfigFactory`; `OwnedBuildServer` gained `watcher`,
  `httpServer` and the two `config` fields the real server hands its hooks; `OwnedBuildEnvironment`
  gained `isSsrBuild`/`isPreview`; `OwnedBuildConfig.worker` states its real shape.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` — the rendezvous plugin's
  `httpServer` accepts the `null` the build server really passes (§1.2); `bun:sqlite` assembled at
  runtime so the analyser cannot fold it back (§3.5).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📋️plan/🟦️.ts` — one named, documented
  `buildToolConfig()` crossing where the owned config is handed to vite's own `build()`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/**` and
  `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg/**` — rebuilt (gitignored artifacts, §1.1).

**Contracts / taxonomy:**
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `setup-wizard-config` removed
  (§2.2), `generatorContracts` 23 → 22.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` — the Ralph-specific laws
  replaced by the general foreign-metadata-directory law (§2.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🔣️.json` — a wasm-dev package override may
  carry `strip` as well as `opt-level`, still closed (§3.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json` —
  NB1's `semio-s-plugin-norm` override declared (§3.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` — a stale "26 tools" docstring (the array
  already declared 28).

**Laws / configs (none weakened, none deleted):**
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` — three laws
  re-expressed over every foreign metadata root (§2.3).
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` — the real cold-pair
  double and the two laws it exposed (§1.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts` — the config itself and the
  three live gates excluded from the vitest project (§3.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔄️end-to-end/🟦️.ts` — the independent tool
  census at 28 (§3.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference-bridge/🟦️.ts` +
  `🧪️tests/💡️inference-bridge/🟦️.ts` — the sixth gis-map route (§3.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`
  and `…/🧪️tests/🔬️engine-contract/🟦️.ts` (§3.4).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` —
  the single-CGU law (§3.5).

## 7. Captures and probes

`🗑️generated/`: `ts2-typecheck-000-baseline.txt` (4) · `ts2-typecheck-final.txt` (0, exit 0) ·
`ts2-wasm-rebuild.txt` + `ts2-wasm-rebuild-pid.txt` · `ts2-vitest-framework-os-00{1,2}.txt` ·
`ts2-vitest-renderer-wgpu.txt`, `…-nofailfast.txt`, `…-long.txt` ·
`ts2-vitest-renderer-react-pkgint.txt`, `ts2-vitest-renderer-react.txt`,
`ts2-vitest-renderer-react-final.txt`, `ts2-vitest-rr-frame-transport.txt`,
`ts2-vitest-rr-engine-contract.txt` · `ts2-vitest-os-mcp-00{0,1,2}.txt` ·
`ts2-vitest-os-shell.txt` · `ts2-vitest-plugin-registry.txt`, `…-002.txt`, `…-003.txt` ·
`ts2-vitest-os-dev-00{0,1,2}.txt` · `ts2-serve-s-6199.txt`, `ts2-serve-s-6199-b.txt`,
`ts2-serve-pid.txt` · `ts2-boot-diagnose.json`.
Scripts: `📜️ts2-wasm-rebuild.sh` (both wasm-pack builds as ONE mutex hold).
Probe reused: `🐍️ts1-boot-diagnose.mjs`.
