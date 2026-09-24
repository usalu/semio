# WP-O2c Dev-Facing Launch Path (nx daemon + warm cache)

## Goal
Make `bun nx run @semio-tech/framework-os-dev:serve-s-react-dev` seconds on a warm daemon/unchanged tree. Fix non-deterministic `session-*` cache misses and slow project-graph construction.

## Baseline (O2b)
| Step | Wall | CPU (user+sys) | Notes |
|---|---|---|---|
| Direct ServeScript warm HTTP 200 | 3.98 s | n/a | no nx |
| nx project-graph prime (cold, iso) | 334.6 s | n/a | |
| First nx serve HTTP 200 | 241.2 s | n/a | 0/3 cache on session chain |
| Truly warm second serve | not measured | | this WP |

## Root causes found
1. **`repo:generator-inputs` was `cache: false`** (policy `uncached` + project.json). Every serve paid ~36–79 s wall / ~12 s CPU for catalog fingerprinting even on an unchanged tree → critical-path miss for the whole `generate`/`session-*` chain.
2. **Project graph bloat**: command script closures (~500 files) were **copied into every cacheable target's `inputs`**. Result: ~417k string inputs, **133 MB** `project-graph.json`, multi-second deserialize.
3. **`createDependencies` read every JS/TS file** (~7k) for import edges; entry-only narrowing was tried then **rejected** (see coordinator review).

## Fixes landed
### 1. Cacheable `generator-inputs` (sound inputs) — accepted
- `caching/policy.json`: removed from `uncached`; added to `cachedExact`.
- `caching/project.json`: `cache: true` + explicit catalog inputs.
- Updated nx-contract fixture, feature wording, receipt test, and cache-contracts guard assertion (`guard.cache === true`).

### 2. Intern command closures as `namedInputs` — accepted
- `library.mjs` `projectWithDefaults`: unique command input arrays stored once as `commandSourcesN` namedInputs; targets reference the name.
- Graph size **133 MB → 83 MB**; string inputs **417k → ~86k**.

### 3. Incremental full import scan (replaces rejected entry-only narrowing)
- **Edge-loss proof** (`generated/edge-diff.json`): `EDGE_LOSS_CONFIRMED` — full import edges 385 vs entry-only 435; **260 missing project→project edges** in entry-only (`onlyInFull` 381). Entry-only dropped relative imports from non-entry files (tests/fixtures/submodules).
- **Replacement**: keep full per-file static import scan; process only `CreateDependenciesContext.filesToProcess`; persist targets under `$NX_WORKSPACE_DATA_DIRECTORY/emoji-import-edges/<fileHash>.json`; cold pass uses bounded parallel `fs/promises` reads (concurrency 32).
- **Equality**: fixture `caching/fixtures/import-edges` (deep `a/deep/util.js` → `b`) + repo assertion `createDependencies` import edges == `collectImportEdges` on full fileMap; warm hash cache preserves edge set (`equalFullScan: true`, cold 4010 edges / 830 import edges). Wired into cache-contracts as `testImportEdgeEquality`.

## Measurements (isolated NX_SOCKET_DIR / NX_WORKSPACE_DATA_DIRECTORY / NX_CACHE_DIRECTORY under `generated/nx-iso*`)

### Warm serve (iso4, after incremental scan)
| Run | Cache | Dep task times | nx run duration | HTTP 200 wall |
|---|---|---|---|---|
| serve-a (fill cache) | **0/3** | gen-inputs 22.5s, … | 49.6 s | 133 s |
| serve-b (warm) | **3/3** | 19ms / … / 7ms | **1.6 s** | 70 s* |
| serve-c (warm) | **3/3** | 2ms / … / 5ms | **1.2 s** | 80 s* |

\*HTTP wall still elevated under fleet load (`[stale]` plugins). Nx dependency chain is milliseconds once cached.

### Project graph (iso4, incremental full scan)
| Mode | Wall | CPU user+sys | createNodes | createDependencies | Notes |
|---|---|---|---|---|---|
| Cold NX_DAEMON=false | **98.4 s** | **31.3 s** | 39.9 s | 28.8 s | first fill of edge-hash cache |
| Direct `createDependencies` cold/warm (equality probe) | 3.0–3.9 s / 0.24–0.48 s | — | — | — | hash cache hit path |
| Warm daemon show (unchanged) | **11.0 s** | 2.2 s | — | — | mostly deserialize / fleet |
| Warm daemon show (single-file edit) | **68.9 s** | 2.1 s | rebuild | incremental filesToProcess | low CPU; wall dominated by peer churn |

Cold CPU improved vs prior 42.5 s; wall varies with fleet. createDependencies alone with warm edge cache is sub-second.

## Verification
- `generated/edge-diff.json`: `EDGE_LOSS_CONFIRMED` (260 missing project edges under entry-only).
- `generated/equality-probe.json`: `equalFullScan: true`, `warmEqualsCold: true`.
- Import-edges contract: **PASS** (`cold=4010 import=830`) inside `repo:test` / dedicated runner.
- Policy/graph contracts: generator-inputs cached + `commandSources` interning on workspace/session from project-graph snapshot.
- `@semio-tech/framework-os-dev:test-quick`: **163 passed** | 28 skipped.
- Full `repo:test` later native/Trunk fixtures can contend on fleet `deps-cargo` locks; nested `--skip-nx-cache` still breaks unrelated `/0\/1 hit/` assert (pre-existing).

## Files changed
- `library/🟨️.mjs` — namedInput interning; incremental full import scan + hash edge cache
- `caching/🔣️policy.json` — generator-inputs cacheable
- `caching/📋️project.json` — cache + inputs
- `caching` tests/fixtures — generator-inputs cacheable; **new** `import-edges` fixture + equality test

## Captures
Under `.tmp-ticket/wp-o2c/generated/`: `edge-diff.json`, `equality-probe*`, `incr-*-time.txt`, `incr-serve-*`, `test-os-dev-quick2.txt`, `repo-test-cache2.txt` (import-edges PASS).
