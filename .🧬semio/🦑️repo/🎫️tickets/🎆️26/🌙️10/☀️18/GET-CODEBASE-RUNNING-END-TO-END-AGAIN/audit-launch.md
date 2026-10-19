# Audit Launch: launch.json and launch seed after today's edits

Read-only audit, 2026-10-18. No launch file, project file or source was modified. Snapshot: `.vscode/launch.json` mtime 19:08:53, seed mtime 15:53:09, checker run about 19:13. Both files were still being edited by peers during the audit, so counts from earlier runs (for example 13808 configurations, before the 19:08 edit) differ slightly from the numbers below.

Verdict: (1) PASS, (2) FAIL, (3) FAIL, (4) PARTIAL.

## Method and scripts (all in this ticket folder)

- `audit-launch-check.ts`: copy and extension of `osdev-targets.ts`. Adds package.json scripts as targets, `run-many` and `exec --projects=` parsing, `${input:...}` expansion over pickString options, ticket-scoped project detection, and a Cargo `run` inference for binary crates. The Cargo `run` inference wrongly marks `@semio-tech/repo-cli-rs:run` as resolved; those 8 routes are corrected by hand below. Raw outputs were generated under `🗑️generated/audit-launch/` and removed after this report.
- `audit-launch-parse.ts`, `audit-launch-shape.ts`, `audit-launch-inputs*.ts`: JSONC parse, shape, inputs.
- `audit-launch-canon.ts`, `audit-launch-diff.ts`, `audit-launch-gen.ts`, `audit-launch-cats.ts`: seed versus launch comparison (canonical JSON with sorted keys).
- `audit-launch-extra.ts`, `audit-launch-dup.ts`: duplicate manifest names, compounds, duplicate config names.
- `audit-launch-new.ts`, `audit-launch-coverage.ts`: untracked manifests and launch-route coverage.
- `audit-launch-targets.ts`: declared targets of selected project.json files.
- `audit-launch-nx-probe.sh`: `nx show project` probe. It failed with `Original bounded Nx namespace required`, which is the repo's own guard. The guard was not circumvented, so nx-inferred targets are not verified.

## (1) Parse

- `.vscode/launch.json`: parses as JSONC. 13814 configurations, 4 compounds, 126 inputs. Keys: version, configurations, compounds, inputs.
- `.vscode/🧩️launch.seed.jsonc`: parses as JSONC. 10349 object configurations, 89 placeholder strings `"@generated:<variant>:<react|wgpu>"`, 21 compounds, 69 inputs, plus `devLaunchers` (47 keys) and `projectLaunchers` (7 keys).
- Duplicate config names: launch 3 names. `⚖️cargo-registry-ready-native prototype` and `... release` are identical pairs. `🧱️artifact-io-⚖️installed-service🧰️os🟦️` is a real collision: two different commands (`@semio-tech/framework-os:installed-service-source` and `@semio-tech/gis-gismap-js:inference-service-receiving-check`) with the same group `4_gate` and the same order `411.2631`. Seed: the same collision, 1 name.

## (2) Route resolution (`bun repo run|run-many|exec` and `nx run|exec`)

Totals, launch.json: 14452 routes (run 11446, run-many 181 plus 1 glob, exec 2824).

| Status | launch.json | seed |
|---|---|---|
| Resolved (target declared, or project with no target needed) | 12096 | 7504 |
| Ticket-scoped project (`exec --projects=<ticket project>`, project exists in a ticket folder, 95 names, target not checked) | 1837 | 1722 |
| Project not defined in repo or any ticket folder | 462 | 453 |
| Target not declared | 49 | 39 |
| Dynamic or glob (promptString or glob, not expandable) | 8 | 8 |

Corrected for `repo-cli-rs:run`: the first checker version counted it as resolved through a Cargo `run` inference I added. The original checker and the declared `project.json` do not provide `run`, so it is counted as target-missing (4 routes in launch, 4 in seed).

### Unresolved, non-ticket routes (launch.json, the list VS Code runs)

Project not defined anywhere (repo, package.json, Cargo, ticket folders):
- `ticket-interface-current-origin`: 433 routes (seed 424). No project.json defines it. Dead routes from an earlier ticket.
- `ticket-engine-layout-source-control` with no suffix and suffixes -2 to -18: 28 routes.
- `ticket-world-current-source-model-5`: 1 route.
- `${input:repoBuildTask}` (promptString, user-typed `Project:task`): 6 routes. Dynamic, not broken.
- `run-many --projects="@semio-tech/stdio-*-rs"` (glob, `:test`): 1 route. Parse-level artifact, glob not expanded, not verified.

Target not declared in the project's `project.json`, `package.json` scripts or Cargo-inferred set (nx plugin inference not verified, so these may still be valid):
- `@semio-tech/process-extension-{wood,metal,concrete,robotic}-rust:describe`: 3 routes each (12). Via `run-many -t describe`.
- `@semio-tech/sourcing-extension-{beams,slabs,windows}-rust:describe`: 2 routes each (6). Via `run-many -t describe`.
- `@semio-tech/repo-cli-rs`: `run` (4), `daemon` (2), `workflow`, `install`, `preferences` (1 each). The project declares `build`, `test`, `repo`, `mcp`, `build-cli`. Result note `result-osdev.md`: the dashboard moved to `@semio-tech/repo-dashboard-rs`, which has no `workflow`.
- `@semio-tech/repo-lib`: `test-process-tree-termination`, `test-draw-destination-observation`, `lint-framework-module-product-direction` (1 each). Declared names drifted (for example `lint-dependency-direction`).
- `@semio-tech/framework-os-mcp-rs`: `test-original-inference-source`, `test-original-inference-native` (1 each).
- `@semio-tech/framework-rs`: `test-artifact-kind` (declared: `test-artifact-kind-source`), `test-fixture-ownership` (declared: `test-fixture-ownership-source`), `test-snapshot-sqlite-io` (1 each).
- `@semio-tech/plugin-registry`: `test-launch-name-contract`, `test-launch-placement-contract`, `check-launch-seed`, `reconcile-launch-seed`, `test-launch-seed-reconcile` (1 each). The launch generator under `🚀️launch` was deleted (commit `2604f70cac1`, `result-demo.md`), so these gates have no target.
- `@semio-tech/framework-io-schema-rs:test-binding` (1).
- `@semio-tech/value-rs`: `test-native-decode-child-source`, `test-native-decode-child-native` (1 each).
- `@semio-tech/s-services-native:source-check` (1, via `${input:projectTarget.source-check}`; declares only `check`).
- `@semio-tech/framework-os-dev:dev` (1, via `${input:projectTarget.dev}`; `dev` not declared; the dev entry points were renamed to `dev-<variant>-...`).
- `@semio-tech/framework-renderer-wgpu`: `dev` (1, via `projectTarget.dev`), `serve` (1, via `projectTarget.serve`); neither declared.
- `@semio-tech/fixture-generator-stdio-tiff-6-0-document:{🏭️generator-byte-order, 🏭️generator-byte-order-manifests}` (1 each).

Key sets differ between the two files. Seed only: `@semio-tech/stdio-xlsx-rs:test-snapshot-sqlite-native`. Launch only (11 keys, all target-missing): `framework-os-dev:dev`, `framework-renderer-wgpu:dev`, `framework-renderer-wgpu:serve`, `s-services-native:source-check`, `plugin-registry:{check-launch-seed,reconcile-launch-seed,test-launch-seed-reconcile}`, `value-rs:{test-native-decode-child-source,test-native-decode-child-native}`, `fixture-generator-stdio-tiff-6-0-document:{generator-byte-order,generator-byte-order-manifests}`. Most launch-only ones come from `projectTarget.*` inputs that exist only in launch (see section 3).

Ticket-scoped routes (1837 launch, 1722 seed): `ticket-fixed-slots` (1186), `ticket-fixture-verification` (208), `draw-ticket-verification` (95), `current-native-origin-N`, `ticket-native-*`, `ticket-ui-layout-source-N` and others. Their projects exist in ticket folders, but targets were not checked (`exec` routes name no target).

## (3) Generated entries versus the seed

Verdict: NOT consistent.

- There is no generator. The 89 `@generated:` placeholders have no expander; the expander was deleted (`result-demo.md`). `devLaunchers` (47) and `projectLaunchers` (7) are stale: only 1 launch entry starts with any `devLaunchers` namePrefix.
- Seed objects not canonically present in launch: 396. Breakdown (overlapping): 115 names absent from launch; 222 `presentation` differences (mainly `order` float changes, for example `900.0151` vs `900.01511`); 36 `env` differences; 30 `command` differences; 6 `cwd` differences.
- Launch entries not canonically present in seed: 3859. 281 share a name with a seed entry but differ; 3578 have names absent from the seed (for example `🧪️ rust-command-data-source-test prototype`).
- Absolute paths: seed 84 `/Users/ueli` paths, launch 0. `${workspaceFolder}`: seed 16687, launch 20797. The seed was not rewritten to match the launch rewrite that `result-demo.md` describes.
- Compounds: launch has 4, seed 21. 17 seed compounds are missing in launch. Launch compounds contain 2 inline objects instead of name references (VS Code expects names). Seed has 4 name references that do not resolve.
- Inputs: launch 126, seed 69. 63 `projectTarget.*` inputs exist only in launch. Seed-only: `bimSqliteOracleFile`, `bimSqliteOracleWord`.
- `SEMIO_BUILD_MODE: "ship"`: 0 in both. `workspace:dev -- `: 0 in both.

## (4) AGENTS.md: executable commands registered, names unique

New manifests today (untracked, `git ls-files --others`): 29 Cargo.toml (28 packages, 1 workspace-only) and 26 project.json. Result files (`result-uicarve.md`, `result-cg*.md`, `result-lex.md`, `result-hdr3.md`) name these as the new crates.
- 26 of 28 have 6 resolved routes each (build, check, test and companion variants), consistent with the `uicarve` convention (`result-uicarve.md`: 192 routes, 32 crates).
- No route, no project.json (Cargo package only), test-only: `semio-s-artifact-layout-layout-engine-export-unit-tests` (`result-lex.md`: "No launch.json route referenced the moved tests").
- No route, no project.json, companion-support: `semio-framework-quiz-tests-support`.
- Decision recorded, not applied: `result-cg2.md` item 6 and `result-cg3.md` open item say no launch routes were added for the 236 carved crates (precedent R3/R4). Repository-wide, 2326 of 3355 Cargo packages have no launch route (186 of them test-like). Most are libraries and may legitimately have none. The owner should decide whether the carved executables need routes; this conflicts with "register all executable commands" in AGENTS.md.
- Duplicate names: no duplicate project or package names across manifests (0). Duplicate launch config names: 3 (see section 1), including the order collision `411.2631` in group `4_gate`.
- Plugin-registry launch gates (5 routes) and repo-cli-rs routes (9) are stale; listed in section 2.

## Unresolved summary

- Dead project references: 462 launch routes (433 `ticket-interface-current-origin`, 28 `ticket-engine-layout-source-control-N`, 1 `ticket-world-current-source-model-5`), seed 453.
- Declared-target gaps: 49 launch routes across 34 project:target pairs; seed 39 routes.
- Missing launch routes for new crates: 2 (test-only or companion support, no project.json).
- Seed/launch drift: 396 seed entries differ or are absent, 3859 launch entries are not in the seed, 89 placeholders have no expander.
- Duplicate config names: launch 3 (one divergent collision); seed 1 (same).

Not verified: nx-inferred targets (nx guard), runtime behaviour of any route, and the glob `run-many` expansion.
