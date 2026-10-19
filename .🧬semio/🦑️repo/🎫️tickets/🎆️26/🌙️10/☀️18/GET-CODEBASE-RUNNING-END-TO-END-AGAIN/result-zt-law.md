# Result zt-law: zero-touch closure law

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `Z` = `L/🏗️build/📥️source/🛗️zero-touch`. No git-modifying command, no sub-agent, no cargo, `node_modules` untouched.

## 1. Which closures must be node_modules-free

Read from the build entry dispatch (`L/🏗️build/📜️script.ts`, `runOwnedBuildInLeasesV1`) and `📋️project.json` (`deps-*`, `setup*`). Nine entries, authored in `Z/🗂️closures/🔣️.json` (schema `Z/🗂️closures/🧬️schema/🔣️.json`):

| id | entry | mode | why |
| --- | --- | --- | --- |
| repo-cli | `L/🏗️build/📜️script.ts` | full | `bun repo`: help, preflight, tools setup, dependencies lock/sync |
| dependencies-js | `L/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts` | full | `deps-js-lock`, `deps-js-all`, `deps-javascript` |
| dependencies-native | `…/📦️dependencies/🏗️native/📜️script.ts` | full | `deps-cargo/go/python/dotnet/cpp/browsers/tools/trunk` |
| wasm-tools | `…/🛠️tools/🕸️wasm/📜️script.ts` | full | `deps-wasm-opt` |
| bazel-bootstrap | `…/🛠️tools/🏗️bazel/📜️script.ts` | full | `tools setup bazel` |
| build-bootstrap | `L/⚡️caching/🚀️bootstrap/📜️script.ts` | full | `ensureBuildEnvironment`, Nx adapter, `tools setup nx` (`nx --version`) |
| root-router | `📜️script.ts` | static | every root route starts here; lazy routes load later |
| root-setup | `L/🎛️commands/🔧️setup/🟦️.ts` | full | `bun ./📜️script.ts setup …` runs in a half-installed workspace |
| root-entrypoints | `L/🚨️policy/🧭️entrypoints/🟦️.ts` | full | every non-repo root route dispatches through it |

Root `repo` route = the repo-cli closure. The other lazy root routes (the ~113 files the first scan counted: verify lanes, test self-tests, dev, lint, …) run after install and are deliberately NOT in scope; they stay covered by the layering law.

Seams (the one place pre-install code meets code that needs installed dependencies): a closure may load a seam by dynamic import or require (the edge is not followed) but never by static import (`seam-static`). Two seams:
- `L/🏗️build/🕸️graph/📥️inference/🟨️.mjs`: the build graph inference reads sources with the TypeScript compiler and `@iarna/toml` (`require("typescript")` etc.); `graph/🟦️.ts` imports it lazily for run/run-many/affected/show.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🟦️.ts`: Nx graph consumer path (`runWithToolingStore` skips it for `nx --version`).

## 2. Permanent law

- Core `Z/🟦️.ts`: scans modules with the system compiler (`Bun.Transpiler.scan` + `scanImports`, type-only imports dropped, same non-ASCII specifier decode as `🔏️closure`), walks each closure, and reports per import: `workspace-specifier` (bare name of a workspace package from the root `workspaces` list; detail names the relative owner file through the package `exports`, conditions and one `*` wildcard), `external-package` (detail text identical to the capture refusal: `External implementation runtime dependency refused: <specifier>`), `seam-static`, `missing-entry` (closure entry or seam gone). The existing refusal in `🔏️closure` resolves on disk at capture time (needs node_modules); the new law is the static, install-free counterpart with the same message, so I did not duplicate the mechanism, only mirrored its text and the decode quirk.
- Law adapter `L/⚖️laws/🛗️zero-touch-closure/🟦️.ts` (`policyZeroTouchClosureBreaches`, BreachRecord rows).
- Wiring (same mechanism as `verify build-owners`): route `verify zero-touch` in `L/🎛️commands/✅️verify/🟦️.ts`, `runZeroTouch` in `…/🧾️taxonomy-lanes/🟦️.ts`, `runGate` step in `…/🏁️gate-lanes/🟦️.ts`, root target `verify-zero-touch` (`cache: true`, `outputs: []`) in `📋️project.json`.
- Schema + fixture: `L/🏗️build/🧬️schema/🛗️zero-touch-closure.json`, `Z/🧪️tests/🔣️.json` (13 closure cases incl. external, workspace names with condition/wildcard exports, type-only, static vs full, seam cut/static/stale, missing entry, cycles, query suffix, emoji paths, shebang; 4 scan rows), `Z/🧪️tests/🟦️.ts`.
- Third-party cross-checks in the test: Ajv validates fixture and real roster against their schemas; the TypeScript compiler AST (ts.createSourceFile) agrees with our scan on every file (~900) of the real closures; package export resolution is compared with a Node-algorithm oracle.
- README: one paragraph EN + DE in `🧰️framework/🛍️products/🦑️repo/README.md` next to the build owner registry law.

## 3. Tree made to pass

The first real run found only `root-setup` offenders (the repo-cli closure was already relative; its `typescript`/`@iarna/toml` uses are behind the two seams). Applied `C/relativize-bare-imports.ts --write` on the setup entry (8 specifiers, 0 unresolved):
- `L/🎛️commands/🔧️setup/🟦️.ts`, `L/🏃️process/🟦️.ts`, `L/🏃️process/🔍️probe/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`, `…/🌉️mcp/📡️stdio/🎫️session/🟦️.ts`, `…/🌉️mcp/📡️stdio/📋️protocol/🟦️.ts`
Then the relativized `ensureMcpBinary` import in `setup` pointed at the package barrel (`🌉️mcp/📦️packages/🟦️typescript/🟦️.ts`), which re-exports `🧬️schema-validation` and so pulls `ajv`; I re-pointed it to the owner `🌉️mcp/🟦️.ts` (no barrel). Re-read before every edit; a peer concurrently edits `🌉️mcp/🟦️.ts` (new `serveWarmMcpStdio`), untouched by me.

## 4. Verification (all run)
- `bun test ./Z/🧪️tests/🟦️.ts`: 22 pass, 0 fail (~10 s).
- `bun ./📜️script.ts verify zero-touch`: `passed.`
- `bun build --target=bun 📜️script.ts` and `bun build --target=bun L/🏗️build/📜️script.ts`: both succeed; `bun L/🏗️build/📜️script.ts repo --help` prints usage.
- Targeted `tsc --noEmit` over core, test and law adapter (scratch config): 0 errors.
- `verify docstrings emoji-unique --files-from <touched>`: no repeat in the new files (the two repeats in `taxonomy-lanes` are pre-existing at other lines).
- Taxonomy: all 5 new directories resolve (`C/c6db1-tools/tax.ts`).
- `bun repo --help` with node_modules hidden: not run (not allowed). Bare `verify` (runGate) not run in full (it launches cargo/nx steps).

## 5. Open items
- A layering sweep that re-introduces bare workspace imports in these closures now fails `verify zero-touch` with file and specifier.
- Whether `tools setup nx` truly never evaluates the Nx inputs seam is argued from `runWithToolingStore` (`graphConsumer` excludes `--version`), not run with node_modules absent.
- `cache: true` on the root target relies on the root project's default inputs (same as `verify-build-owners`).
