# 🔎 S Loading Audit for the FEM Plugin

Read-only audit in `C:\semio`. No source files, catalogs, pins or tickets were changed. Only this report was written. `$T` was not set in the shell, so the report sits in the ticket folder `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\`.

## 1. Does the FEM plugin load in the s instance?

Static evidence says the plugin is registered and the channel versions agree. No runtime load was observed in this audit.

- Plugin registered in the s plugin catalog: yes.
  - `C:\semio\✏️s\🧑‍💻dev\🧩️catalog\🤖️generated\🔌️plugins.json` lines 305-329: `pluginId "fem"`, `packageId "semio:fem"`, `cratePath 🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust`, `wasmOut semio_hub_fem_component.wasm`, `executionMode "isolated"`, activation events for `s.fem.fem2d`, `s.fem.fem3d`, `computation.fem2d`, `computation.fem3d`, `2d.fem`, `3d.fem`.
  - `C:\semio\✏️s\🧑‍💻dev\🧩️catalog\🤖️generated\🦀️catalog.json` lines 187-201 (plugin entry, same data) and lines 1242-1254 (artifact/app entries): `fem2d` maps to `s.fem.fem2d@1/*#editor` with alias "fem 2d"; `fem3d` maps to `s.fem.fem3d@1/*#editor` with alias "fem 3d".
- Diagnostics: `C:\semio\✏️s\🧑‍💻dev\🧩️catalog\🤖️generated\🩺️diagnostics.json` is `[]`. No stale-channel diagnostic for fem. `🧰️framework.json` is also `[]`.
- Channel versions agree:
  - Pin: `C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🧑‍💻dev\🔖️channel-version\📌️pin\🔣️.json` has `"channelVersion": 25` (mtime 2026-11-05).
  - Committed FEM descriptor: `C:\semio\🌎️hub\🧩️compositions\🏗️fem\🧩️component\🔣️.json` line 18657 has `"appChannelVersion": 25` (mtime 2026-11-21 04:06).
  - Match: 25 equals 25.
- Staleness risk: the s catalog files are dated 2026-11-19 01:08. The FEM descriptor was rewritten 2026-11-21 04:06, after the catalog was generated. The catalog therefore predates the current descriptor. The version numbers match, but content drift was not diffed.
- Runtime evidence in the ticket logs, `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\🗑️generated\`:
  - `ts-femjs-test.log` and `ts-fem2d-check.log` show `FAIL` in `@semio-tech/fem-js` mutation-grammar tests for 2d and 3d.
  - The failure is `ENOENT` on a fixture file under `✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\🏅️standards\...\🦠️mutation\🔣️.json`. The fixture was missing when the log was written, 2026-11-22 00:57. It may have been added since.

## 2. Regeneration and launch targets

Regeneration targets found:

- `C:\semio\✏️s\🧑‍💻dev\📋️project.json` (project `@semio-tech/s-dev-composition`, root `✏️s/🧑‍💻dev`):
  - `generate`: `dependsOn` `generate-stdio-format-claims` and `@semio-tech/framework-os-dev:channel-version-descriptors-check`. Command `repo ./🧩️catalog/📣️publication/📜️script.ts generate`. Output is `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`.
  - `catalog-check`: command `repo ./📜️script.ts catalog-check`, cwd `✏️s/🧑‍💻dev`, cache true.
  - Also present: `check-generated`, `preview-generated`, `verify-catalog`, `test-catalog-publication`, `test-catalog-exports`, `generate-stdio-format-claims`, `check-stdio-format-claims`, `source-check`, `typecheck`, `composition-check`.
- `describe fem` (`workspace:components -- describe fem`): not found. Searches of `🧰️framework`, `✏️s` and `🌎️hub` `project.json` files for `workspace:components` returned nothing. The whole repository was not searched because the recursive grep timed out.
- Descriptor regeneration is therefore not wired as a repo target in the places searched. The ticket's `coordination-plan.md` also lists it as queued work, not an existing target.

Dev launch targets for fem 2d and fem 3d:

- None found in any `📋️project.json` under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev` (no file there mentions fem).
- The dev contribution is `C:\semio\✏️s\🧑‍💻dev\🎭️variants\🏗️fem\🧬️schema\🔣️.json` (273 bytes), referenced by the playground catalog `🎠️playgrounds.json` as `fem2d`.
- The ticket `26/11/20/GET-DEV-FEM-2D-WORKING-END-TO-END` (codex) is said to drive the FEM 2D React dev launch. It was not read.
- No fem 3d launch target was found. The coordination plan lists it as queued.

## 3. Every project.json under the FEM roots (excluding 🏅️standards)

| Path (under `C:\semio`) | Project name | Targets |
|---|---|---|
| `✏️s\🔌️plugins\🏗️fem\⚙️engine\📍node\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/fem-node-rs` | `test-source`, `test-native` |
| `✏️s\🔌️plugins\🏗️fem\⚙️engine\🧱️material\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/fem-material-rs` | `test-native`, `test-source` |
| `✏️s\🔌️plugins\🏗️fem\📦️packages\🟦️typescript\📋️project.json` | `@semio-tech/fem-js` | `verify-combination-stress`, `test` |
| `✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\📦️packages\🟦️typescript\📋️project.json` | `@semio-tech/fem-2d` | `build`, `check`, `test` |
| `✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/fem-2d-rs` | `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-material-source`, `test-material-native`, `test-fixed-dofs-source`, `test-fixed-dofs-native`, `verify-fem2d-numerical-native`, `verify-fem2d-mounted-native`, `verify-fem2d-invariants`, `verify-fem2d-invariants-source`, `verify-fem2d-presence-native` |
| `✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d\📦️packages\🟦️typescript\📋️project.json` | `@semio-tech/fem-3d` | `build`, `check`, `test` |
| `✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/fem-3d-rs` | `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-material-source`, `test-material-native`, `verify-fem3d-results-source`, `verify-fem3d-results-native`, `verify-fem3d-transform-intent`, `verify-fem3d-transform-intent-source`, `verify-fem3d-numerical-child-native`, `verify-fem3d-numerical-child-source` |
| `✏️s\🔨️modules\🏗️fem\⚙️engine\🕸️mesh\📦️packages\🟦️typescript\📋️project.json` | `@semio-tech/s-fem-mesh` | `test-surface-contract`, `test-neutral-surface` |
| `✏️s\🔨️modules\🏗️fem\⚙️engine\🕸️mesh\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/s-fem-mesh-rs` | `check`, `test`, `test-surface` |
| `✏️s\🧑‍💻dev\🏗️fem\📦️packages\🟦️typescript\📋️project.json` | `@semio-tech/s-fem-composition-tests` | 15 `verify-*` targets (owners, window-config contracts, live visual publication, numerical microcursor), including `-native` variants |
| `🌎️hub\🧩️compositions\🏗️fem\📦️packages\🦀️rust\📋️project.json` | `@semio-tech/fem-plugin` | `test` |

Notes:

- The `verify-*` target list for the dev package was cut at 15 by the extraction.
- The table omits targets whose names were not matched by the extraction. Targets written inline (the node project) were parsed by hand.

## 4. Shared work fleet status

- Latest round: `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️14\WORKERS-READY-WITH-LOW-RESOURCES\ROUND.md`, round 31.
  - Workers must stop running setup themselves. A `ready` child on each machine runs the stages native, repo-mcp, dependencies, tools, workspace.
  - Workers only observe and write `round-31.md`.
- Worker states in `...\WORKERS-READY-WITH-LOW-RESOURCES\workers\<origin>\state.json`:
  - Ready, native_rc 0: g1-m01 (2026-11-18), g1-m02 (2026-11-18), g1-m08 (ready via the auto runner, 2026-11-18), g1-m10 (2026-11-18).
  - g1-m03 is marked ready but has `native_rc` 130 and a try 5 record, which does not fit the ready rule in PROTOCOL.md.
  - Not ready: g1-m04 (round 30, `native_rc` 75, 2026-11-15), g1-m05 (`tools` stalled 30 tries, 2026-11-18), g1-m06 (round 18, 2026-11-16), g1-m07 (round 30, `native_rc` 7, 2026-11-15).
  - Every state file is older than the 2026-11-22 ticket. None is current.
- Latest tickets mentioning fleet: `☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\🗺️coordination-plan.md` (the audit row is this report), `☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\🎫️ticket.json` (fall back to native tools if the fleet fails), `☀️21\PROVISION-MAC-WORKER-G1-M13\verification.md`, `☀️20\PRESERVE-WORK-WHILE-RECONCILING-WORKER-GIT-STATE`.

Summary, five lines:

1. Builds are not dispatched to workers. What the fleet shares is the native setup, the dependency store (restored from a machine of the same kind), and the group server's project-graph reads.
2. Workers run setup through their own `ready` child, and sessions must not run `native-bootstrap.sh` or `repo tools setup all` at the same time.
3. Offloading is therefore not a supported path for `repo run` or `repo cargo` today; the agent-rules fallback to native `cargo` and `bun` applies.
4. Several machines are stuck (g1-m05 store contention, g1-m04 and g1-m07 setup failures), and none of the state files is newer than 2026-11-18.
5. The coordination plan states the fleet is the default and native tools the fallback. This audit did not test a fleet build.

## Open items and limits

- Runtime load of the FEM plugin in the s instance was not tested. A test would need a launch of `s.fem.fem2d` in the editor.
- Catalog content drift against the 2026-11-21 descriptor was not diffed.
- `describe fem` was not found, but the whole repository was not searched.
- No dev launch target for fem 2d or fem 3d was found in `project.json` files. The codex ticket `26/11/20/GET-DEV-FEM-2D-WORKING-END-TO-END` may hold the launch command and was not read.
