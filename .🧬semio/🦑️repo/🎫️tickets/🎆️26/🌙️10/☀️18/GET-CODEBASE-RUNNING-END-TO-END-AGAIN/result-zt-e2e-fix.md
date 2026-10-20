# result-zt-e2e-fix: zero-touch CLI and tools-setup closures, declared E2E readiness budgets

`C` = this ticket folder, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `T0` = `L/⚡️caching/🚀️bootstrap/🛠️tools`, tools = `C/zt-e2e-fix-tools/`. Stage only: nothing written to the repo outside `C/` (no git writes, no sync, no cargo, no activation). All verification ran on throw-away copies under `/private/tmp` (short path: the product limits absolute paths to 256 code points), without `node_modules`, never symlinks; removed afterwards (the first overlay run mounted three native-cache images inside the overlay; they were force-detached and deleted, none remain).

## 1. Bare imports in the CLI closure

Findings (scanner `C/scan-bare-imports.ts` is regex based and over-reports type-only imports; I added `zt-e2e-fix-tools/closure-bare.ts`, which transpiles TypeScript first so type-only and unused imports are erased, like the bundler, and splits static/dynamic edges):

| Closure | Files | Bare edges now |
|---|---|---|
| `bun repo` (`L/🏗️build/📜️script.ts`, all dynamic edges) | 669 | 0. The vit4 defect `@semio-tech/ui/axes/plan` is already fixed by a peer (`🚀️bootstrap/🎚️ui-axes/🟦️.ts` imports relative since 11:55; the zero-touch roster has a row `bootstrap-source-ui-axes`). `bun repo --help` and `bun repo run workspace:deps-browsers` get past module resolution on a no-`node_modules` copy. |
| root `📜️script.ts`, static edges only | 17 | 0 |
| root `📜️script.ts`, all dynamic edges | 1191 | 201 edges in 273 files if fully relativized (557 replacements) plus third-party packages that cannot be relativized (`typescript`, `ajv`, `zod`, `three`, `immer`, `fast-glob`, `graphql`, ...). All of them sit behind lazily loaded commands (`verify`, `dev`, `generate`, `schema`, `cmake`, `examples`, laws, normalization, discovery). |
| `tools setup nx` source captures (new, from the coordinator's ruling) | runtime 252, compiler 600, cargo compiler 294 | 9 / 12 / 7 bare edges, 23 distinct (file, specifier) pairs in 11 files |

The root-script result is by design, not fixable by relative imports: `bun build --target=bun 📜️script.ts` cannot succeed without `node_modules` because lazily reached commands need the TypeScript compiler API and other installed packages. The existing zero-touch roster models this correctly (`root-router` is mode `static`). The contract that must hold is `bun repo` + `tools setup` + dependency sync, and that now holds (below). Decision for the coordinator: do not relativize the root script's lazy commands.

### The `tools setup nx` failure and its fix
bzl-xplat's finding is real and reproduced: on an unpatched no-`node_modules` copy, `captureNxAuthorizerSourcesV1` (the step `tools setup nx` runs, `T0/🟦️.ts:154,156`; `Bun.resolve` at `T0/🔏️authorizers/🟦️.ts:46`) fails for all three entries with `Cannot find module '@semio-tech/framework-filesystem/observation/journal'` / `'@semio-tech/process/artifacts/native-build/storage/platform/contract'`. The closure files the capture walks import workspace packages by name. Rewritten to relative paths through each package's own `exports` map (planner `zt-e2e-fix-tools/rel-plan.ts`, reads the live tree, writes only edits):

- 28 replacements in 12 files, all value imports reached by the three captured entries (nx authorizer runtime, nx authorizer compiler, cargo compiler entry): the daemon startup service, journal rpc/worker/index, `authorizers/📦️runtime`, the cargo `inputs` and `snapshot`, the authorizer and cargo `script-commands` tests (they are in the compile closure), `💾️development`, the compiler-ingress `reading`, and the `📦️packages/🟦️typescript/💾️storage` barrel (it re-exported `@semio-tech/nx-bootstrap-tools/*`).
- Package-boundary decision: the self-references `@semio-tech/nx-bootstrap-tools/...` become relative paths into `T0` (same package family); the cross-package ones (process, filesystem, schema validator) point at the owning module's published export target, not at internals, so the target files are exactly what the bare specifier resolved to. No code moved.
- Frozen follow-up (applied only with `--include-frozen`): drop `"@semio-tech/framework-filesystem": "workspace:*"` from `T0/package.json` `dependencies` (the dependency only existed for that bare import and broke the closed pinned-recipe schema, bzl-xplat finding 1; the `devDependencies` row stays) and the matching line of the root `bun.lock`. After applying run `bun repo dependencies lock` once.
- Roster (the existing language-agnostic law `L/🏗️build/📥️source/🛗️zero-touch`): three new closures `nx-authorizer-runtime`, `nx-authorizer-compiler`, `cargo-compiler` (mode full) so the law fails if a bare specifier returns.

### Verification (overlay copies without `node_modules`; plan applied with `apply.py`)
- Capture, the actual failing step: `bun zt-e2e-fix-tools/capture-authorizers.ts <root>`: unpatched copy: 3 of 3 `ok:false` (messages above); patched copy: runtime ok 256 files, compiler ok 625 files, cargo ok 303 files, 0 resolutions each.
- Zero-touch law function `checkZeroTouchClosuresV1`: patched copy with the new roster rows: 17 closures, 0 findings; unpatched tree with the same roster rows: `workspace-specifier` findings for the cargo compiler (and the others), proving the rows catch the defect.
- `bun build --target=bun` with no `node_modules`: `🏗️build/📜️script.ts` 669 modules, nx authorizer runtime 254, compiler 601, cargo compiler 299, all 0 errors. `closure-bare.ts` on the five entries (+ `T0/🟦️.ts`): 0 bare edges each.
- `bun <build script> repo --help`: ok; `repo tools setup --help --lang=de`: ok (German text).
- `tsc --noEmit` (strict) over the changed session module in the overlay: 0 errors.
- `apply.py --dry-run` against the live tree: 20 edits + 2 new files (+2 frozen), no drift.
- Not run: `bun test` of the zero-touch, authorizer or service-readiness suites in the real tree (stage-only), an actual `bun repo tools setup nx` (it mutates caches and mounts images; it has no `--dry-run`; the capture above is the failing step run read-only), `dependencies lock`.

## 2. E2E readiness: declared, schema-first, per product

Located exactly: play `🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts:25` and demonstrator `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts:16` called `waitForServiceReady(root, session, signal)` with the silent 60 000 ms default of `L/⚡️caching/🌐️vite/🧾️session/🟦️.ts:99`. No other caller used the default (the two library laws pass explicit values).

Design (schema-first, domain-neutral framework + product data):
- Schema `ReadinessBudget` / `BootRange` in `L/⚡️caching/🌐️vite/🧾️session/🧬️schema/🔣️.json` (the module that owns the wait): `readyTimeoutMs` 60 000..3 600 000, `progressIntervalMs` 1 000..60 000, `evidence {kind: estimate|measurement, coldBootMs, warmBootMs, marginFactor >= 2, source}`.
- Reader `parseServiceReadinessBudgetV1` in the same module enforces what a JSON schema cannot: boot ranges ordered, warm maximum not above cold maximum, and `readyTimeoutMs >= coldBootMs.maximum * marginFactor`, so a timeout cannot be declared without the evidence that carries it.
- `waitForServiceReady(root, session, signal, timeoutMs, wait?)`: the 60 s default is gone (`timeoutMs` is required and must be a positive integer, so a caller cannot fall back silently); the optional `wait = {intervalMs, report}` reports `{elapsedMs, timeoutMs, phase: awaiting-ready-record | verifying-listener}` at once and then on the declared cadence; the timeout error names the budget (`... within 600000 ms`). Cancellation through the existing `AbortSignal` is unchanged.
- Progress line renderer with EN and DE text and language choice like the CLI help (`SEMIO_LANGUAGE`, then `LC_ALL`/`LC_MESSAGES`/`LANG`, then the first catalogue language): `serviceReadyProgressLine`, `serviceReadyProgressLanguage`. Re-exported from the `@semio-tech/repo-lib/dev` barrel (`🎮️playground/🟦️.ts`).
- Product data (new): `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧪️e2e/🔣️.json` and `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️e2e/🔣️.json`, next to the existing per-product E2E owner and session-root constants. Values from the audit (`audit-e2e-readiness-1020.md` section 4 and R1): serve boot cold 1-3 min, warm 30-60 s for both products; margin 3 on the 180 s cold maximum gives 540 s, rounded up to the audit's 600 s floor: `readyTimeoutMs` 600 000, `progressIntervalMs` 15 000, `kind: "estimate"`. Honest limit: these are the audit's estimates, not measurements; replace `kind` with `measurement` and the ranges when a real cold run exists (the law keeps the timeout above margin x maximum).
- Both `TestScript`s read their budget, pass it to the wait, and print the localized progress line every 15 s.
- Law (language-agnostic, extends `L/⚡️caching/🧪️tests/🌐️service-readiness/🟦️.ts` and `🧫️cases.json`): both product files validate against the schema with ajv (third-party cross-check) and our parser and satisfy the margin and a 600 000 ms floor; 9 invalid rows (the 3 that a schema cannot express are `schema: true` rows that only our parser rejects); progress cadence (>= 3 events in 450 ms at 100 ms, first at ~0, spacing >= interval, phase and timeout fields), the `0` timeout refusal, and 2 EN+DE line rows and 5 language-resolution rows.
- Verification: `zt-e2e-fix-tools/verify-readiness.ts <root>` replays exactly those assertions without the vite part: patched overlay `{"ok":true,"events":4,"products":2,"invalidRows":9}`; the live (unpatched) tree fails it (`fixture.readiness` undefined), so it discriminates. Demonstrator e2e script bundles with the budget JSON import; the play e2e bundle check hit only my overlay's missing `🧪️tests` directory, not the change. Not run: the full service-readiness law with vite, the Playwright runs.
- Not done: the launcher `run-e2e.sh` smoke serve has its own `E2E_SERVE_TIMEOUT` (default 1800 s) and stays; the course gate does not use this wait.

## 3. Apply (after "PAUSE LIFTED")
```
cd /Users/ueli/Documents/semio
python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/zt-e2e-fix-tools/apply.py --dry-run          # content hashes per target, refuses on drift
python3 .../zt-e2e-fix-tools/apply.py                                  # 20 edits + 2 new files; refuses while a sync or show projects runs
python3 .../zt-e2e-fix-tools/apply.py --include-frozen                 # also T0/package.json + bun.lock (then: bun repo dependencies lock)
bun .../zt-e2e-fix-tools/verify-readiness.ts "$PWD" && bun .../zt-e2e-fix-tools/capture-authorizers.ts "$PWD"
bun test ./<L>/🏗️build/📥️source/🛗️zero-touch/🧪️tests/🟦️.ts           # one explicit file at a time
```
`plan.json` (hash-checked, idempotent, `--root DIR` for an overlay) is regenerated by `rel-plan.ts` + `build-plan.py` (`e2e-edits.py` holds the authored edits); if a peer touches a target the apply refuses and the plan is rebuilt in a minute.

Files the plan edits (22): the 12 import files above, `🧾️session/🟦️.ts`, `🧾️session/🧬️schema/🔣️.json`, `🧾️session/🧫️cases.json`, `⚡️caching/🧪️tests/🌐️service-readiness/🟦️.ts`, `📦️packages/🟦️typescript/🎮️playground/🟦️.ts`, the two product `🧪️e2e/🟦️.ts`, the zero-touch roster `🗂️closures/🔣️.json`; frozen: `T0/package.json`, `bun.lock`. New: the two product `🧩️runtime/🧪️e2e/🔣️.json`.

## 4. Open items
1. Coordinator decision confirmed by this data: root `📜️script.ts` lazy commands stay install-dependent (201 bare edges plus unrelativizable third-party packages).
2. Overlap with bzl-xplat: its staged `nxExternalPinsV1` (drops `workspace:*` from the installer recipe) becomes a no-op for `framework-filesystem` once the frozen `package.json` edit is applied; keep it for any future workspace dependency of the tools package. Its second finding (`Cannot find module '@semio-tech/nx-bootstrap-tools/use'`) is fixed here by the relative imports; its "order `deps-build-tools` after the JS install" alternative is no longer needed.
3. The authorizer capture's `resolutions` feature (workspace specifier index) now has no input in the real toolkit (0 resolutions); the permanent fixtures still exercise it. Leave it, or retire it with the schema once the roster is trusted.
4. Cross-package relative imports from `📚️library` into `🔨️modules/...` follow the existing precedent (`🏗️build/📥️source/🔏️closure` already does this). If a layering law later objects, move the shared piece (journal, leases, schema validator) under the tools package instead.
5. Timeout values are estimates until a cold serve is measured; the field `evidence.kind` is the marker.
