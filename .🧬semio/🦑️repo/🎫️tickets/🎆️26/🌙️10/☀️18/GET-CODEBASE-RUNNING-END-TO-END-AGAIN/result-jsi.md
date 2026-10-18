# Result jsi (build inference module `🕸️graph/📥️inference/🟨️.mjs`, census 6 P12)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo, no build. `bun repo --help` booted (exit 0, stage `complete`) before, after the split and after the final edits. `bun repo show projects` was not run: `node_modules` is still a dangling symlink (dependencies not installed).

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `M` = `L/🏗️build/🕸️graph/📥️inference`, `C` = this ticket.

## 0. Verdict

DONE. The 2,120-line monolith is now a 151-line root (`M/🟨️.mjs`, same path, so all 64 importers stay unchanged) plus a 92-line revision-graph kernel and 17 components of 24 to 287 lines each (all well under 600). Reload no longer mixes stale and current code, and no analysis id is registered twice. Differential battery against the original: 622 checks, 0 differences.

## 1. Design (the report's proposal, realised)

The report's blockers were (1) children keep a cached instance after a root reload, (2) analysis ids derive from the module URL and a child with a constant URL re-registers the same id with a new `analyze` closure ("Ambiguous source analysis revision registration"), (3) eleven `let` bindings written by `libraryBootstrap` and read everywhere, (4) `import.meta` identity sites, (5) the `libraryRevision` hash of `🧪️test/🟨️.mjs`, (6) 61 importers.

- **One revisioned module graph.** `M/🗂️components/🔣️.json` is the schema-first roster (`M/🗂️components/🧬️schema/🔣️.json`): ordered components, each with a portable relative path and the earlier components it needs (acyclic by construction).
- **Components never import each other.** Each component exports `component` (its roster name) and `wire(registry)`; the registry holds only the components it declares as needs plus the shared `services`, `revisionIdentity` and `register`. Cross-component names are bound as module-level `let`s in `wire`, so every function body stays as it was.
- **Same revision token for every child.** `M/🪢️revision-graph/🟨️.mjs` (kernel, pure JS, runs in Bun and Node, has a `.d.mts`) imports every component as `./<dir>/🟨️.mjs?revision=<sha256>` with a relative string specifier. I measured that this gives a fresh instance per token in both runtimes and keeps the query in `import.meta.url`, whereas `import(fileURL.href + "?x")` collapses to the cached module in Bun and needs the old `require.cache` eviction. `importRevision` (eviction) stays only for the helper modules `libraryBootstrap` loads, unchanged.
- **Scope per revision.** `createRevisionLoader` caches one wired graph per revision (adopted for the first one), drops a failed load, wires a module instance at most once, and lets a graph that failed to wire completely be wired again. A revisited revision reuses its components and registrations.
- **Registration through the root's registry.** Components do not register at import time. `register` (root) is idempotent per (id, latest revision), so the taxonomy projection (id = root URL, revision = function text) is shared by graphs whose projection text did not change, and a graph with a changed text replaces it. All other analysis ids hash (root URL, graph revision) as before, taken from `revisionIdentity` instead of `import.meta.url`.
- **State.** The bootstrap-assigned bindings live in one exported record `loaded` (`loaded.runtimeComponentClosure(...)` etc.); `libraryBootstrap` is created in `bootstrap.wire`, so it still exists synchronously when the root has evaluated. `BUN_LOCK_CACHE` is read through `bunLockCacheHash()`.
- **Revision covers the whole graph.** `implementationRevision()` hashes root, kernel, roster and every component in roster order (`services.sourceFiles`), then the same authority list as before.
- **Reload.** `invokeCurrentImplementationV1` is verbatim except the reload branch: `services.loadRevision(revision)` loads the new graph and `module.invokeCurrentImplementation(kind, args)` replaces the four `module.*` dispatches (they were exactly that call). The `cacheInternals` identity guard is kept.
- **Root.** Static literal imports of every component and the roster JSON (`with {type:"json"}`), so command-closure analysis still sees the whole module and `require()`/sync consumers get `cacheInternals` immediately (no top-level await). Public API unchanged plus two new exports below. A root edit and a roster change take effect when the module is loaded again (documented in the root); every other edit reloads in process.

New root exports (declared in `🟨️.d.mts`, listed in the time-travel export-roster corpus): `currentImplementationRevisionV1()` and `loadCurrentImplementationV1()`. `🧪️test/🟨️.mjs` used to `import(<root>?revision=<hash of the root bytes>)` and hash only the root; with a split root that would hash too little and, in Node, share stale children. It now asks the module for its current implementation and revision instead.

## 2. Files

Components (lines; all under `M/`, each `<dir>/🟨️.mjs`, docstring emoji = folder emoji, names resolve in the live taxonomy via `c6fin2-tools/tax.ts`, RGI, max path 183 bytes):

| dir | lines | content |
| --- | ---: | --- |
| `📍️coordinates` | 81 | names, paths, POLICY, ownership data, physical source reads, `workspaceDataDirectory` |
| `🔁️bootstrap` | 94 | `importRevision`, `loaded`, `libraryBootstrap` (hand written) |
| `🧬️source-analyses` | 179 | taxonomy projection, Rust reference scanner, `prepareSourceAnalysesV1` |
| `🧶️bun-lock` | 105 | lock graph |
| `🎯️target-policy` | 69 | execution contracts, target policy, `targetWithDefaults` |
| `🦀️native-manifests` | 257 | TOML, Cargo/Go manifests, dependency roots, mounted Rust source, native preparation |
| `🧭️script-inputs` | 173 | command script import closure |
| `🏭️generator-outputs` | 89 | generator output coupling |
| `🧩️component-targets` | 129 | cargo, component, release, print, root-command targets |
| `🎮️playground-targets` | 287 | playground catalog, session, preparation, site targets |
| `🧱️project-assembly` | 218 | `projectWithDefaults`, `projectInputs`, native watch plan |
| `🔎️owner-discovery` | 209 | owner and manifest discovery, admission facts |
| `📦️project-nodes` | 160 | `emojiProjectJsonNodes` |
| `🕸️dependency-edges` | 219 | import edges, metadata, dependencies |
| `♻️revision` | 56 | `implementationRevision`, dependency resolution authority |
| `🧰️internals` | 24 | `cacheInternals` (built in `wire`) |
| `🚦️invocation` | 50 | `invokeCurrentImplementation(V1)` and the reload branch |

Also new: `M/🪢️revision-graph/🟨️.{mjs,d.mts}`, `M/🗂️components/{🔣️.json,🧬️schema/🔣️.json}`, `M/🧪️tests/🟦️.ts` + `🔣️.json` + `🧬️schema/🔣️.json` (reload test), `M/🧪️tests/📖️source/🟦️.ts` (reader helper). Changed: `M/🟨️.mjs`, `M/🟨️.d.mts`. Tooling and the pre-split input are in `C/jsi-tools/` (`split.cjs`, `spec.cjs`, `manual/`, `assemble.sh`, `install.sh`, `diff.mjs`, `lossless.cjs`, `check-scope.cjs`, `buntest*.sh`, `orig/inference-orig.mjs`); the scratch `node_modules` is `/Users/ueli/.cache/jsi/nm`.

## 3. Language-agnostic reload test

`M/🧪️tests/🟦️.ts` (registered in `L/🏗️build/🧪️tests/🟦️.ts`, which `bun repo run` executes through the build contract):

- `🔣️.json` (closed by `🧬️schema/🔣️.json`) describes an abstract three-component graph (values, needs), eight steps (symbolic revision, value edits, concurrent loads, refused wiring) and twelve roster admission cases, with the observable result per step: rendered value, wiring order, evaluation counts, graph identity.
- The test replays the corpus in a Bun and in a Node child process through the kernel; both must equal the corpus and each other (Node's native ESM loader is the independent oracle). Roster admission is checked by the kernel and by Ajv against the roster schema.
- The actual graph is audited with the TypeScript syntax tree: each component names itself, imports no sibling, and its `wire` reads exactly the registry entries its roster `needs` declare; the root statically imports exactly the rostered components; the real root loads.
- Mutation check: loading components without the revision token fails the Bun/Node test (4 of 5 pass, the replay fails); restored afterwards.

Result: 5/5 pass.

## 4. How I verified (exact commands and outcomes)

1. Original vs split, 622 checks over the whole `cacheInternals` roster (61 members, typeof and behaviour on fixtures: target policy matrix, lock graph, Rust module scanning, Cargo dependency closures, script closures, project inputs, targets, error messages): `NODE_PATH=/Users/ueli/.cache/jsi/nm/node_modules bun C/jsi-tools/diff.mjs C/jsi-tools/orig/inference-orig.abs.mjs M/🟨️.mjs` gives `checks=622 diffs=0 errored-both=12` (the 12 are intended refusals). The original is replayed from a rewritten copy that points at the real module location.
2. Losslessness oracle (`lossless.cjs`): every top-level declaration of the monolith appears verbatim in a component modulo the declared rewrites: 134 identical; 12 changed only by the declared rewrites (`observedFs` destructuring per component, two depth-adjusted path constants, `register`/`revisionIdentity`, `services.sourceFiles`, `services.loadRevision`, `bunLockCacheHash()`); the rest moved into `wire`, the root or `bootstrap` by design.
3. Scope oracle (`check-scope.cjs`, TypeScript semantic diagnostics): no undefined name; only diagnostics the monolith already had.
4. Real-graph reload with the installed tree copied next to itself (Bun): edit a component, reload shows the edit (`EDITED-1`), repeat uses the cached graph, new edit (`EDITED-2`), revert and revisit return the earlier graph without "Ambiguous" error, concurrent calls share one load, the dependency-resolution digest changes on a leaf edit and returns to the original on revert. The copy was removed.
5. Tests: 61 test files that read, import or stub the module were run one file per process with a script-process envelope and the scratch `node_modules`, before (HEAD exported to a scratch tree) and after. Tests failing after but not before: 8 (artifact-source, three graph tests, two workspace-negation-closure tests, test-layout, build/execution). For all of them I put the original monolith back into the real tree and re-ran: the same tests fail identically, so they are environment failures (no installed Nx, no ticket env, graph writer custody, strict Ajv), not caused by the split. The original was restored byte-identical (`cmp`). The scratch tree and all outputs are removed, and four disk images the graph tests left mounted were detached.
6. Newly green as a side effect: `🎛️phase` ("selected producer fixture passes its genuine phase into actual inference admission": it now supplies the three `requireCurrent…` ports), `native-dependency-closures` (supplies `memoizedObservedComputationV1`), plus the reader tests below.

## 5. Text readers and consumers re-pointed

Readers use the helper `M/🧪️tests/📖️source/🟦️.ts` (`readInferenceSourceV1(name)`, `readInferenceGraphSourceV1()`) and strip `export` where they evaluate function text:

- `🧰️framework/🔨️modules/⏪️time-travel/🧪️testing/📥️values/🧪️tests/🔣️.json` (export roster gains the two new exports)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` (playground-targets, whole graph)
- `🧰️framework/🛍️products/📓️print/📦️artifacts/🧪️tests/🟦️.ts` (component-targets)
- under `L/`: `⚡️caching/🧪️tests/🧊️preparation/🧪️tests` (graph source, `loaded` ports), `⚡️caching/🕸️graph/✅️verification/🧪️tests`, `⚡️caching/🕸️graph/🔏️seal/🎯️selection/🧪️tests/🎛️phase`, `⚡️caching/💾️storage/…/🏪️store/🧪️tests/📥️required`, `⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body` (`loaded` ports), `⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures`, `🏗️build/📥️inputs/📋️data/🧪️tests` (4 tests: revision services, bootstrap string), `🏗️build/📥️inputs/🧪️tests`, `🏗️build/👁️observation/🧪️tests/📥️reuse`, `🏗️build/🧪️tests/🎟️phase/🏭️producer`, `🏗️build/🧪️tests/🕸️graph`
- `L/🧪️test/🟨️.mjs` (uses `loadCurrentImplementationV1` and `currentImplementationRevisionV1`) with its stubs in `L/🧪️test/🧪️tests/🚷️discovery-boundaries` and `L/🧪️test/🕸️dependencies/🧪️tests/🦀️inputs`
- `L/⚡️caching/🧪️tests/🔁️graph-revision/🟦️.ts` and its fixture: the "implementation" stage now edits `DEFAULT_EXECUTOR` in the `📍️coordinates` component (a child edit) and the expected executors are the current `repo:script` / `repo:noop` (the file held the stale `nx:run-commands` / `nx:noop`, which the monolith never contained at HEAD).
- Import-only consumers (61 importers, the four `✏️s/🧑‍💻dev/🧩️catalog` tests, the build-owner callers) are unchanged.

## 6. Open risks

- **Not run here:** `bun repo show projects` (no `node_modules`), the `graph-revision` test (`bun L/⚡️caching/📜️script.ts test graph-revision` needs the tool-runner and lease infrastructure), and the real root under plain Node (the repo's TypeScript graph does not load there without flags; the kernel is verified under Node). The updated graph-revision stage is therefore unverified; its Bun/Node comparison remains the coordinator's to run once dependencies are installed.
- Root, kernel, roster edits take effect on the next module load (not in process); documented in the root. Everything else reloads in process.
- A graph that failed to wire is not cached and can be retried; a graph whose asynchronous bootstrap rejected stays cached (its rejection is the answer for that revision), as the previous per-module state did.
- Three pre-existing `[DEBUG]` trace lines (selected-owner stage, settlement round, TOML parse refusal) moved verbatim with their functions; `newcheck.py` flags them. Not removed (behaviour identical).
- Test readers that select variable declarations from the concatenated graph source must skip the injected `let a, b;` bindings (one test, `native-dependency-closures`, now filters on initializers).
- Concurrent edits: the original was byte-identical (sha256 `e1b2fd55…`) until installed; `install.sh` refuses when it changed.
