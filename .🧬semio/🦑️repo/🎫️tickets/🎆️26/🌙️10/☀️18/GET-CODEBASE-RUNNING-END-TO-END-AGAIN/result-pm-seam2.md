# result-pm-seam2 (executor pm-seam, wave 1021a, round 2)

READY2: round 2 is staged on top of round 1 in `C/pm-seam-tools/staged2/` (122 anchored edits in 40 files, 16 whole-file rewrites of round-1 files and schema bases, 7 new files; 144 operations). Mirror-verified, NOT applied. Apply only after round 1 (and, as before, chain17 green, round 5, GO). `C` = this ticket, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `K` = `L/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers`. Nothing outside `C/` was written; overlays in `/private/tmp/pm-seam/` are deleted.

## 1. Design change that made the deferred items possible
The 100 `new Function` extraction tests run storage functions in isolation and bind only the schema and some ports, so module-level seam constants cannot be used inside those functions. Rewriting the tests to proper imports would need dependency injection into hundreds of functions. Instead round 2 moves every manager-specific value into **data that those functions already read**:
- The install and generation schemas are now **neutral bases** (`$manager` expressions and `{{token}}` placeholders). `K/🧩️materialize` fills them from the descriptor of the selected manager (`javascriptInstallSchemaV1`, `javascriptGenerationSchemaV1` in `D/💾️storage/🧱️schemas`). The materialized install schema carries `$defs.ManagerProtocol` (lock file, scope configuration, generation directory, strict profile prefix, cache variable, lazily the runtime identity) and, non-enumerable, the live manager (`.manager`: render, configuration names, projection, document reader). Every extracted function reaches the manager through the `installSchema` binding it already has.
- Proof that nothing changed for bun: `materialize(base, bun)` equals the pre-seam `install.json` and `generation` schema (modulo the renamed runtime field and the new `ManagerProtocol` def) by canonical JSON, the recipe keeps its hashed key order, and a twin materializer in the Python oracle produces the same digest for all six managers.

## 2. Deferred items, one by one
| item | now |
|---|---|
| exports-profile family | profile names, conditions and the strict-first prefix come from the descriptor (`resolution.profiles/conditions/strictPrefix`), read through `ManagerProtocol` |
| `process.versions.bun` receipt runtime tier | `ManagerProtocol.const.runtime` (host version key or the manager's own probe, memoized); generation key and runtime tier use it; key byte-equal to the pre-seam formula (law) |
| 36 `"j"` store-directory literals (+ `j/` root) | `ManagerProtocol.const.generationDirectory`; the incremental glue imports the derived export |
| `install.json` | neutral base, 21 token sites; roles, lock/config names, patterns, refusal text, recipe, profiles all from the descriptor |
| generation `runtime.bun` | `runtime.manager` (core type, compare, capture, schema, tests); producer inputs, store role files, env prefixes and tier text from `descriptor.producerInputs` |
| lock projection (`Bun.JSONC` plus the bun workspace-path rewrite inside the driver) | adapter capability `projectLock`/`parseLockDocument` (bun full; npm implemented but not run through a real install); the driver calls the manager. The bun projection equals the pre-seam arrow in both directions (law) |
| staged installer configuration text | `descriptor.stagedConfiguration` |
| `⚡️caching/🔣️policy.json` and 9 more repository data files | declared in `K/🔣️owned.json` (taxonomy, caching policy and schema, retention schema, parity rules, hermetic table, scenarios, assets projection, bazel schema, TypeScript project inputs). A law proves each still names the selected manager's files, so flipping the manager fails visibly until they are edited |
| graph members, graph inference components, inputs, selection seal, dependency truth/inventory, metrics path rules, pinned tooling recipe (7 files), publication layout, help text | derive from the selected manager through the catalog (TypeScript) or the CommonJS accessor `K/🟨️.cjs` (plain `.mjs`); the inference implementation revision now covers the descriptors (`sourceFilesV1`) |
| bun-specific modules (`bun-lock` graph component, `🗂️workspaces/🟦️bun`, cargo preparation) | declared adapter-owned in the descriptors (`ownedModules`) and exempt from the ratchet by law |
| PM literals in production | **0 outside adapters**: 117 literals in 37 files on the round-1 stack, now 25 in 12 files, all inside adapter-owned modules (4) or the 10 declared data files (21). The ratchet pattern is the package manager vocabulary (`bun.lock`, `bun.lockb`, `bunfig`, `--frozen-lockfile`, `--lockfile-only`, `BUN_INSTALL`, role names `bun-cache|…|bun-import`); Bun runtime APIs (`Bun.JSONC`, `process.versions.bun` outside the driver, `BUN_OPTIONS`) stay with runtime-seam |

## 3. Live glue for the other ecosystems
`prepareDependencies` (native dependency sync) now builds the python, rust-lock and go commands from the managers (`render(...)`); the cargo preparation pair runs exactly the commands the cargo manager renders (law, the module is adapter-owned); `tools/🟦️.ts` installs the pinned tooling recipe through the new `install-pinned` operation. New renderer features: optional argument groups, `each` lists, scopes, per-operation values, `provision`, environment overrides.
Coordination with langs-conc (answered by aad162251aeef58f8): environments and pins stay theirs. The python branch is an **alternative pair of edits**: against the live text it renders `uv sync --locked --all-packages --all-groups`; against the text their READY3 leaves it renders `uv python install <series>` and `uv sync --locked --project <root> --group …` with their `plan.variables` as environment, no `--cache-dir`, no `--managed-python`. `apply.py` takes whichever variant matches and refuses otherwise; `goEnvironment` and the dotnet pin check are untouched (dotnet stays a pin check). Both variants are proven (anchors; the langs-conc variant also syntax-checked on their overlay).
Not done: a `repo dependencies … --ecosystem` flag (the parser and the pinned help snapshots are schema-closed; the native `deps-*` targets are the entry for now) and production ports for `ensureGenerationV1` for non-JS ecosystems (proven with fake ports; their environments live in langs-conc's plan).

## 4. Verification
- Mirror = live copy + lock-sync closure + round 1 (+ round 2). `apply.py --round 2` applies the staged set to a fresh round-1 stack (144 operations), is idempotent (0 pending) and **undo restores all 56 files byte for byte** (snapshot-based; round 2 has overlapping replacements, so anchored reversal is ambiguous). On the live tree it refuses until round 1 is in.
- Seam suite: 88 pass (adds: materialization equality with the pre-seam schemas, no leftover tokens for any manager, bun projection equality, optional/each/scope/values/provision rendering, CommonJS accessor equals registry, owned modules and data laws, zero ratchet, cargo commands, native glue). Independent Python oracle re-derives every vector (render, keys, materialization digests, lock readers).
- `tsc` over 50 production files: identical diagnostics to the round-1 stack (16, all pre-existing).
- Regression: 44 existing suites (storage, reading, generation, incremental, concurrent, bootstrap, graph observation, inference, tools/sdk, retention, truth, selection) identical pass/fail counts on the round-1 stack and with round 2. Seven existing tests pinned storage source text or read the raw schema and were updated to the new text or to module imports. Many suites fail at load in any mirror (they need the 4.8 GB `💻️os` tree as real directories for the authority-ancestry check), so the inference component tests and several tools tests have no evidence beyond `node --check` and the accessor law.
- Paths: longest repository path 182 units, no Windows overflow (`winpath`).

## 5. Apply (after round 1)
```
cd C/pm-seam-tools
python3 apply.py check --round 2          # rc 1 pending, rc 2 refused (round 1 missing or drift)
python3 apply.py apply --round 2          # dry run
python3 apply.py apply --round 2 --write  # refuses while dependencies sync|lock|ensure or show projects runs
sh rebuild2.sh                            # rebuilds the round-1 stack in a scratch tree, applies round 2 on a copy, proves it
python3 apply.py undo --round 2 --write
```
Cache effect: the generation recipe, install key and bootstrap key are unchanged for bun (law); the generation receipt body changes (`runtime.manager`), so every generation receipt is re-sealed once.

## 6. Open items
1. Extraction tests are kept (not rewritten to imports); they now read manager values through data. A DI redesign of the storage functions is the real fix if they must go.
2. npm through the full JavaScript driver is unproven: the staged configuration text, the isolated-layout shape (`ProducerConfiguration` def) and the projection are bun-shaped; only the neutral pieces and the descriptors exist for it.
3. Declared data files (10) are guarded by a law, not derived.
4. `dependencies ensure|lock|sync --ecosystem`, production ports for the neutral ensure flow, and a real-install proof for the non-bun adapters.
5. The `verify taxonomy` and `verify layering` gates were not run (no `repo` process during the freeze); new folders: `🧩️materialize`, `🧱️schemas`, and the adapter extension declarations.

## Files
`C/pm-seam-tools/`: `plan2.py` and `plan2_{schemas,storage,types,tests,native,literals}.py` (the anchored edits), `staged2/plan.json` and `staged2/new/w/*` (rewrites, new files, snapshots), `mkbase.py` and `pm_materialize.py` (bases and twin materializer), `mkdescriptors.py`, `mkschema.py`, `mkratchet.py`, `apply.py` (`--round`, alternatives, rewrites, snapshot undo), `rebuild2.sh`, `regress-new.sh`, `regress-extra.sh`, `regress-both.sh`, `suites-extra.txt`, `one.sh`, `tsc2.sh`.
