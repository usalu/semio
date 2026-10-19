# Result seed: one source of truth for the editor launch catalog

Executor `seed`, 2026-10-18, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` edit, nothing under the frozen storage / revision paths. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket folder.

## Design

`.vscode/🧩️launch.seed.jsonc` is the only authored source. `.vscode/launch.json` is its deterministic expansion (two-space JSON, final newline) and is never edited.

* Seed keys: `version`, `configurations` (literal routes plus placeholders), `compounds`, `inputs`, `devLaunchers`, `crateRoutes.subjects`.
* Placeholders: `@generated:<variant>:react|wgpu` (one dev launcher), `@generated:<variant>:users` (per-user launchers, variant `s`), `@generated:crate-routes` (the routes of all crate subjects).
* Dev launchers: command, port, `SEMIO_*` / `S_OS_PORT` env and `serverReadyAction` come from the playground registry (the `[[package.metadata.semio.playground]]` rows of the tracked `Cargo.toml` files, read with `git ls-files` and `Bun.TOML`); the seed only holds name prefix, order, extra env (`devLaunchers`). The build environment (`BUN_RUNTIME_TRANSPILER_CACHE_PATH`, `NODE_DISABLE_COMPILE_CACHE`) is taken from the build-environment schema, never spelled in the module.
* Crate routes: 796 subjects (`name`, optional `project`, `shape` lib / tests / package, optional `targets`, optional `arguments`) replace 4,772 literal entries; the protocol of the loc generator (`🚀️launch-routes`) is unchanged except for the new shape `package` and the optional `targets` / `arguments`. Orders continue above every other route of the group (computed, deterministic).
* The generators now write their data source: `registerCrateLaunchRoutes` (called by `new crates`) appends subjects to the seed and rewrites the catalog; the dashboard `launch generate|check <ticket>` projects the ticket commands into the seed and then expands (it wrote `launch.json` directly before).
* Commands: `bun ./📜️script.ts generate launch` writes the catalog, `… generate launch --check` fails on any problem or drift. Registered as `workspace:generate-launch` / `workspace:generate-launch-check` and `@semio-tech/repo-lib:test-launch-catalog`, with launch rows `📦️generate-launch`, `⚖️generate-launch-check`, `⚖️test-launch-catalog📚️library🟦️` (plus the missing rows `📦️generate-config`, `⚖️generate-config-check`).

## Fold (how the two files became one)

`launch.json` was the superset (13,816 entries vs 10,439) and the portable one (seed had 84 absolute `/Users/ueli` entries), so it is the base; seed-only content was folded in. Tool: `C/seed-tools/fold.ts` (+ `graph.ts`, `apply.ts`), re-readable and re-runnable; the apply step refuses when either file changed since the fold (hash check).

| item | result |
| --- | --- |
| seed after | 7,993 configurations: 7,679 literal + 314 placeholders (156 react, 156 wgpu, 1 users, 1 crate anchor); 4 compounds; 128 inputs; 156 `devLaunchers`; 796 crate subjects. 5.7 MB (was 7.2 MB) |
| launch after | 12,767 configurations, 4 compounds, 128 inputs. 8.3 MB (was 9.3 MB) |
| generated entries | every dev launcher of the old launch.json (the 314 placeholders) regenerates to the identical entry; the 4,772 crate routes regenerate identical modulo `order` (0 left literal); the only other entries that differ are the repaired ones below |
| new variant | `bim` had no dev launchers; `🛠️dev🏬️bim🏢️model⚛️react|🧊️wgpu🌐️wasm` are now generated (ports 6302/6402 from its manifest) |
| seed-only entries | 117 (the 115 absent names plus 2 misplaced ones): 34 were renames of entries already in launch, 68 live ones folded back in next to their predecessor, 15 dead ones dropped |
| misplaced content | the seed `compounds` array held 19 configuration objects (a botched insert), `inputs` 4, and compound 1 two inline configurations; the live ones were folded into `configurations`, the inline members removed from the compound (VS Code takes names only) |
| duplicate names (3) | `⚖️cargo-registry-ready-native prototype|release`: identical copies removed. `🧱️artifact-io-⚖️installed-service🧰️os🟦️`: two different commands in the same group and order; the second is now `🧱️artifact-io-⚖️inference-service-receiving-check🌍️gis🗺️gismap🟦️` with order +0.0001 |
| absolute paths | 0 (seed had 84) |
| inputs | `projectTarget.dev|serve|source-check` lost the options whose project has no such target; `bimSqliteOracleFile|Word` kept |

### Repaired (successor target evident)

* 9 dashboard rows `repo-cli-rs:{run,daemon,install,preferences,test}` -> `repo-dashboard-rs:<same>` (that project declares them); 2 repo-cli rows `repo-cli-rs:run` -> `repo-cli-rs:repo` (declared; `run` is not); `workflow` has no successor and is dropped.
* 4 catalog rows `run-many -t describe` of the process / sourcing extensions -> `-t package` (the extension router is `<test|package>`; `describe` is gone).
* 1 `repo-lib:lint-framework-module-product-direction` -> `lint-dependency-direction`.

### Dropped (1,122 of launch.json, 15 seed-only; full list with reasons in `C/🗑️generated/seed-run/fold-log.json`, old files in `old-launch.json` / `old-seed.jsonc`)

* 462 name a project that exists nowhere (433 `ticket-interface-current-origin`, 28 `ticket-engine-layout-source-control*`, 1 `ticket-world-current-source-model`).
* 500 have a `cwd` inside a swept `🗑️generated` folder (`fixed-slot-fixture-owners/…`), 142 run a script of the ticket `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O` that no longer exists.
* 17 name a target the project does not declare and has no successor: `plugin-registry:{check-launch-seed,reconcile-launch-seed,test-launch-seed-reconcile,test-launch-name-contract,test-launch-placement-contract}` (the generator is replaced by the commands above), `repo-lib:{test-process-tree-termination,test-draw-destination-observation}`, `framework-os-mcp-rs:test-original-inference-{source,native}`, `framework-rs:{test-artifact-kind,test-fixture-ownership,test-snapshot-sqlite-io}` (the `-source` rows exist), `framework-io-schema-rs:test-binding`, `value-rs:test-native-decode-child-{source,native}`, and the two `fixture-generator-stdio-tiff-6-0-document` byte-order rows.
* The picker rows `🛠️dev📋️`, `🛠️serve📋️`, `⚖️source-check📋️` stay; only their stale project options were removed from the `projectTarget.*` inputs (`C/seed-tools/restore-family-rows.ts` put them back after a first pass dropped them with the stale option).
* The 7-key `projectLaunchers` policy is dropped (the `devLaunchers` table grew from 47 to 156 rows); it has no consumer since the old expander was deleted; the 45 family rows and `projectTarget.*` inputs it once produced are now literal.

## Verification

| command | result |
| --- | --- |
| `bun test ./L/🧪️tests/🚀️launch-catalog/🟦️.ts` | 30 pass / 0 fail (dashboard launch test beside it: 5 pass): fixture expansion (Ajv, a lodash-template second implementation, `jsonc-parser`, `@iarna/toml`, `fast-json-patch` mutations for 16 broken-catalog cases), temp-checkout write / check / register / no-write-on-error, and the shipped files (`launch.json` equals the expansion of the seed; seed admitted by the schema; registry equals an independent TOML parse; unique names; compounds resolve) |
| `bun test ./L/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts` | 7 pass (updated: subject `targets` / `arguments`, shape `package`); aggregator `…/📦️crate-tree/🟦️.ts` 126 pass |
| `bun test ./…/🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts` | 45 pass (it asserted literal route text in the seed; now asserts the subject in the seed and the text in `launch.json`) |
| 33 test files that read either file, before vs after (`C/seed-tools/run-tests.ts`) | identical pass / fail names in 31; the other two changed for unrelated peer reasons (native-build recipe test needs `SEMIO_TEST_ARTIFACT_DIR`; os crate-boundaries improved 110/6 -> 115/1) |
| dashboard `🧪️tests/🚀️launch` | 5 pass before and after |
| `tsc` (repo options) over the module, both tests, `generate`, dashboard script | 0 errors (a seeded error in my test was caught first, so the check does type) |
| route resolution, `C/seed-tools/verify-routes.ts` (extended `osdev-targets.ts` model: project.json, package.json, Cargo, inferred playground / session / root targets, ticket projects, `run`, `run-many`, `exec`, `nx run`, input expansion) | launch: 12,057 resolved, 1,337 ticket-scoped (53 ticket projects that exist), 0 project / target missing; 8 by construction: `${input:repoBuildTask}` (6), `${input:playComponentPackage}`, glob `@semio-tech/stdio-*-rs`. Before: 462 project-missing + 49 target-missing |
| `bun C/osdev-targets.ts launch` | only ticket-scoped projects (outside its model) and `${input` remain |
| `checkLaunchCatalog` / `writeLaunchCatalog` | `[]`, `changed:false`; about 2.5 s (git ls-files + 156 variants) |

## Files

Production: `.vscode/🧩️launch.seed.jsonc` (rewritten), `.vscode/launch.json` (regenerated); new `L/🏗️authoring/🚀️launch-catalog/{🟦️.ts,🧬️schema/🔣️.json}`; changed `L/🏗️authoring/📦️crate-tree/🚀️launch-routes/{🟦️.ts,🧬️schema/🔣️.json}` (no longer names a catalog; `$id`s are URLs), `L/🏗️authoring/🎮️command/🟦️.ts` (import), `L/🎛️commands/🏭️generate/🟦️.ts` (`generate launch`), `L/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}` (`test launch-catalog`), root `📋️project.json` (`generate-launch`, `generate-launch-check`), `🎛️dashboard/📦️packages/🦀️rust/📜️script.ts` (projects into the seed).
Tests: `L/🧪️tests/🚀️launch-catalog/🟦️.ts`, `L/🧪️tests/🧫️fixtures/🚀️launch-catalog/🔣️.json`, `L/🧪️tests/📦️crate-tree/🚀️launch-routes/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts`.
Ticket inputs (kept): `C/seed-tools/{fold,graph,apply,verify-routes,run-tests,make-fixture,kinds,reg,projnames,an*}.ts`, `tsconfig.json`. Tool output (delete at close-out): `C/🗑️generated/seed-run/*`.

## Open

* Everyone who edits `.vscode/launch.json` directly (scripts, peers) must edit the seed instead; a direct edit is reported by `generate launch --check` and lost by the next `generate launch`. Please tell the fleet.
* Root `📜️script.ts` does not load at the moment (`Cannot find module …🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts`, a peer's move), so `bun repo run workspace:generate-launch` was not run end to end; the same code paths run through the module functions and the tests. `package.json` scripts for the new targets are not added (freeze).
* The registry read is tracked-files only (`git ls-files`): a variant in a Cargo manifest that git does not know yet appears after the next auto-commit. The untracked scan costs 12-28 s.
* Ticket-scoped rows (1,337) are only checked for their project, not their script content; nx-inferred targets beyond the osdev model are not verified (the real graph cannot load).
* Not restored: automatic rows for every declared project target (the deleted `projectLaunchers` generator); 2,326 Cargo packages still have no route, which AGENTS.md "register all executable commands" would ask for. Needs an owner decision.
