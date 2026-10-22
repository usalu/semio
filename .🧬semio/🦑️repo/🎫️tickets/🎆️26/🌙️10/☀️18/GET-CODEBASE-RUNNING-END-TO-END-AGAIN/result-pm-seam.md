# result-pm-seam (executor pm-seam, wave 1021a)

READY: staged under `C/pm-seam-tools/` (22 new library files + 26 anchored edits in 10 files), mirror-verified on top of the round-5 closure batch, NOT applied. Nothing outside `C/` was written; overlays lived in `/private/tmp/pm-seam/` and are deleted. Apply only after chain17 green, round 5 done, and the coordinator's GO.

## 1. What the seam is
One neutral package-manager interface for every ecosystem, schema first. A manager is a descriptor (JSON, read by any language) plus a lock reader; every behavior of the core is driven by the descriptor.

| piece | file (under `…/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/`) |
|---|---|
| contract | `🧬️schema/🔣️.json` (Descriptor, Request, Invocation, LockGraph, Receipt, protocol limits) |
| selection | `🔣️policy.json` (ecosystem to manager id) plus neutral env override `SEMIO_PACKAGE_MANAGER_<ECOSYSTEM>` |
| neutral core | `🟦️.ts`: registry, `render(lock\|sync\|verify)`, backend choice (refuses aliasing backends), isolated environment, runtime identity, `generationKeyV1`, `bootstrapKeyV1`, `generationLocationV1`, sealed receipts, `expectedProvidersV1`/`walkProvidersV1` (provider walk, progress + cancellation), `ensureGenerationV1` (receipt gate, sync, walk, seal) |
| formats | `📖️formats/🟦️.ts`: JSONC and TOML-subset readers (no external library at runtime) |
| adapters (descriptor + lock reader each) | `🍞️bun` `📦️npm` (javascript), `🦀️cargo` (rust), `🐹️go`, `🐍️uv` (python), `🔷️dotnet` |
| composition root | `📚️catalog/🟦️.ts`: the only module that names adapters |

An adapter declares: executable and version probe, manifests, lock file names and formats, configuration file names per role, isolated environment, the install recipe (backends per platform such as clonefile/copyfile, `clone`/`copy` for uv, lifecycle, linker), the operation argument templates, the installed layout (containers, entry template, name transform, peer-suffix rule), exports-resolution profiles, the ecosystem's generation directory and its graph component. Neutral core, formats and schema name no manager, lock, config file or host runtime (law test).

Swap: `SEMIO_PACKAGE_MANAGER_JAVASCRIPT=npm` changes lock name (`package-lock.json`), command (`npm ci …`), layout and key, nothing in the flow (test "swapping the javascript manager"). Generations and receipts are neutral: location `<ecosystem directory>/<key20>` (`j`, `r`, `g`, `p`, `n`), one sealed `dependency-generation-receipt` envelope, the same key shape for every manager (the manager contributes recipe and runtime identity). Managers of one ecosystem must share toolchain and generation directory (registry refuses otherwise).

## 2. Contract tests (all 80 pass in the mirror)
- Language-agnostic vectors `🧪️tests/🔣️.json` (93 KB), minted only by the independent Python oracle `🐍️oracle.py` (stdlib, own code: regex/line readers, hashlib); the TypeScript suite only compares. Per manager: descriptor digest, render of every operation on 4 platforms (+ missing-request refusal), backend table, environment, runtime identity (embedded and probed), 5 generation keys, bootstrap key, lock fixtures to graph, provider walk (match and one missing), receipt seal. The suite fails if a registered manager has no vectors (no hard-coded list).
- Third-party agreement: `ajv` and the repo validator on descriptors/graphs/receipts; `smol-toml` equals the TOML reader on Cargo/uv fixtures and real repository locks; `jsonc-parser` equals the JSONC reader; real repository bun/Cargo/uv locks parse and package counts are bounded by the third-party parse; every rendered flag is checked against the real tool's own `--help` (bun, npm, cargo, go, uv, dotnet; npm `--cache` via `npm config ls -l`).
- Real tool output: `gen-fixtures.sh` generated real locks offline with bun 1.3.14, npm 11.16.0, cargo 1.99.0-nightly, uv 0.11.15, go, dotnet 10.0.300 (two-member local projects), embedded as `real-members` fixtures; plus real repository excerpts (tools bun.lock, lowpoly Cargo.lock, styling uv.lock, a ticket go.sum, the dotnet empty lock). Composed rows (npm registry row, go escape, nuget frameworks) are labelled and carry test hashes.
- Laws: ratchet over production code (below), neutrality, adapter import discipline, bun descriptor equals the live install recipe, **bun generation key, bootstrap key and sync arguments equal the pre-seam live formulas byte for byte** (so the swap itself causes no cold re-install; the round-5 cold sync is still coming for its own reasons).
- Negative controls: a wrong flag in the bun descriptor fails 4 tests (digest, render, oracle, real `--help`); a `bun.lock` literal added to `🕸️graph/🟦️.ts` fails the ratchet.

## 3. Live wiring (anchored edits, groups for `apply.py --group`)
| group | files | effect |
|---|---|---|
| seam | 21 new files | the library above |
| ratchet | 1 new file | allowlist: package-manager literals per production file |
| bootstrap | `🏗️build/🚀️bootstrap/🟦️.ts` | lock name, key, locked-version lookup (graph instead of a bun-format regex) and generation directory come from the selected manager |
| graph | `🏗️build/🔣️policy.json`, `🕸️graph/🟦️.ts`, observation test | `bun.lock` leaves the policy basenames; `graphMembers.lockEcosystems:["javascript"]` pulls the selected manager's lock names (only that ecosystem: Cargo.lock/uv.lock/go.sum stay out of the graph as before) |
| storage | JS dependency storage, incremental glue, 3 tests | lock name (17 sites), config names (scope/home/config), producer arguments (rendered by the manager), producer environment, install-key runtime identity, JSONC reader, generation directory for the incremental glue; source-pin tests updated to the new text; Node-bundle test prelude imports the catalog |
| register | `⚡️caching/📋️project.json`, script-commands test | target `test-package-manager-seam` and suite row `package-manager-seam` |

Anchors are proven against the live tree and live + round-5 closure group (counts identical in both; `build-plan.py` refuses otherwise). Re-applying is idempotent (0 pending), `undo` restores the baseline storage byte for byte, longest repository path 163 units, no Windows overflow (`winpath`).

Measured literals (ratchet pattern `bun.lock|bunfig|--frozen-lockfile|--lockfile-only|process.versions.bun|Bun.JSONC`, production files of the library): live 137 in 50 files, after the set 103 in 48 files; bootstrap 0, JS dependency storage 1.

## 4. Verification evidence (mirror = live copy of library, process, filesystem, schema, compiler + lock-sync `closure` group + this set)
- `tsc` over new and touched files: no diagnostic in them. Three errors exist in the live `💾️storage/🟦️.ts` at unrelated lines (`JavascriptDependencyStoreSelectionV1` kind `scope` vs `scopes`); identical in the baseline mirror, not mine.
- New suite: 80 pass, 0 fail (nice 10, one process).
- Regression: 24 existing suites (storage, reading, diagnostics, fence, paths, providers, generation x3, incremental, concurrent, initial preparation, dependency-bootstrap (3 pass), graph observation, build-surfaces, script-commands, workspaces-bun, caching dependency suites, closure, parsers, phase) run in a baseline mirror and in the mirror with the set: rows identical except one timing flake (`concurrent pointer switch … torn in-place write`, fails or passes in either mirror, passes alone). Many suites already fail in both mirrors (for example reading 36, storage 19, generation 2, observation 1): mirror limits and peer churn, unchanged by this set. A first comparison also showed a deterministic difference in `paths` that came from a peer editing `🏃️process/…/📈️progress` between two mirror copies, not from this set (proved by copying every changed file into the older mirror).
- Cost: with the set the bootstrap witness parses the 671 KB root lock once per inspect, 13.3 ms vs 0.7 ms for the old regex (memoized). Not measured end to end (needs the lease and an idle machine).

## 5. Apply after GO (order: chain17 green, round 5 batch applied, then this)
```
cd C/pm-seam-tools
python3 apply.py check            # rc 1 = pending, rc 2 = drift/refused (nothing written)
python3 apply.py apply            # dry run
python3 apply.py apply --write    # refuses while dependencies sync|lock|ensure or show projects runs
sh test.sh                        # builds the mirror itself if absent; or live: bun repo run <caching project>:test-package-manager-seam
python3 apply.py undo --write     # reverse while the files still hold our result
```
If round 5 changes its plan, `python3 build-plan.py` re-proves every anchor against live and live + lock-sync and refuses on drift; `sh rebuild.sh` rebuilds the mirror and re-cuts the ratchet. Ratchet must be re-cut (`python3 mkratchet.py ROOT --write`, then `build-plan.py`) after any peer adds or removes such a literal. Staged files live under `staged/new/w/` (aliases of paths Windows cannot create; maintained by the shared `winpath` tool).

## 6. What is NOT done (honest residuals)
1. The ~100 existing `new Function` extraction tests run storage functions in isolation with hand-picked bindings, so any extracted function that mentions a module-level seam constant fails (found and fixed three: exports selector, borrowed-provider normalization, standalone producer arguments). The set therefore wires only module-level and glue code. Still bun-literal: the exports-profile family (`"bun-import"`, `startsWith("bun-")`), `javascriptDependencySourceOwnerRuntimeV1` (`process.versions.bun` in the receipt runtime tier), the 36 `"j"` store-directory literals in storage (equal to the descriptor by law, derived constant used by bootstrap and incremental glue), and the whole `📥️install.json` schema (6 literals; the descriptor recipe equals its recipe const by law). Clean fix: move those functions together with their tests into `🍞️bun/` and let storage call the manager.
2. Generation receipt schema and code still carry `runtime.bun` (`🧾️generation` core, storage, schema, tests); renaming changes every sealed receipt and fixture hash.
3. `⚡️caching/🔣️policy.json` `manifestBasenames` (`bun.lock`) and `toolchains.javascript` (`bunfig.toml`, `bun --version`) feed admission, the cache script and the input-boundary suite; the graph inference component `bun-lock` and the parity/hermetic rules still name the file (the descriptor declares `graphComponent`).
4. Cargo, Go, uv and dotnet adapters render, key, parse and walk (vectors, real locks, real `--help`), but no live glue executes them yet; `ensureGenerationV1` is proven with fake ports only. No `repo dependencies --manager` verb (catalogue/help snapshots are pinned).
5. The Rust/Go/Python lock readers are the TypeScript ones only; the Python oracle is a second implementation of the vectors, not a runtime port.
6. Bun runtime APIs (`Bun.Glob`, `Bun.spawn`, `Bun.TOML`, `Bun.semver`) are class (b) and untouched.
7. The taxonomy gate (`verify taxonomy`) was not run (no `repo` process during the freeze); folder names `🔌️managers/🍞️bun|📦️npm|🦀️cargo|🐹️go|🐍️uv|🔷️dotnet/📚️catalog/📖️formats/⚖️ratchet/🧫️fixtures` reuse emoji already used for the same concepts elsewhere. Run it right after apply.

## Files
`C/pm-seam-tools/`: `apply.py`, `build-plan.py`, `overlay.py`, `paths.py`, `rebuild.sh`, `check.sh`, `test.sh`, `regress.sh`, `regress-all.sh`, `gen-fixtures.sh`, `mkdescriptors.py`, `mkschema.py`, `mkfixtures.py`, `mkratchet.py`, `mint.ts`, `real-locks.json`, `staged/plan.json`, `staged/new/w/*` (22 aliased new files + `map.json`). Generated outputs and both mirrors were deleted.

## Round 6 re-stage by integrate2 (script.ts split)
pm-seam-2: the edit of the removed closure test (🚀️bootstrap/🧪️tests/🎯️closure) is dropped (it only swapped the install-schema import for the facade). round6-tools/staged/make-pm-seam.py.
