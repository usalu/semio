# 1028 Split Audit: New Crates Against House Layout

Status: 30 untracked (`??`) Cargo.toml crates checked. 12 fail workspace membership, 14 lack build-owners entries, 5 lack a source-boundary block, 2 fixture gaps are likely. Paths, lib roots and path dependencies are clean.

Read-only audit. Nothing edited except this report. No cargo, no builds, no git modifying commands.

## Checks run

| Check | Result |
|---|---|
| Taxonomy path ends in `📦️packages/🦀️rust/Cargo.toml`, lib root `🦀️.rs` exists in the node dir | OK for all 30 |
| `[lib] path` matches sibling convention | OK for all 30 (`../../🦀️.rs`; framework `plugin/declaration` and `job` siblings use the same) |
| Listed in exactly the workspace root declared by `package.workspace` | 12 fail (see table) |
| Path dependencies resolve to a dir with `Cargo.toml` and the expected package name | OK for all 30 |
| `workspace = true` dependencies declared in the owning root's `[workspace.dependencies]` | OK for all 30 |
| `📋️project.json` where siblings have one, and listed in `📋️build-owners.json` | OK for framework crates; 14 energy crates have no build-owners entry (siblings: 50 of 52 mutation crates have one, as Cargo.toml owners) |
| `[package.metadata.semio.rust-source-boundary]` where siblings have one | 5 missing (see table) |
| Sibling names in JSON fixtures | Gaps in `surface-fixture.json` and `surface-fixture-lock.json` for the two declaration crates; `feature-consumers.json` is a per-consumer dependency closure, so not counted as a gap |
| Path length (limit 256) | OK. Max repo-relative 165 chars; max absolute 207 UTF-16 units |

Workspace roots in use: repo `Cargo.toml` (framework and most crates), `🧰️framework/Cargo.toml` (members plus large `exclude`), `🌎️hub/Cargo.toml`, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml`, energy `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml`.

## Per-crate table

Paths are repo-relative. Prefix abbreviations: `E` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`, `M` = `E🧬️schema/🧬️mutations/`.

| # | Crate | Path (repo-relative) | Missing items |
|---|---|---|---|
| 1 | semio-s-artifact-energy-model-snapshot-sqlite-admission | `E🚪️io/🪶️sqlite/📸️snapshot/📏️cells/📦️packages/🦀️rust` | build-owners entry (sibling energy snapshot crate `E🧬️schema/📸️snapshot` has one); no same-parent sibling to compare |
| 2 | semio-s-artifact-energy-model-mut-window-layer-optics-visible | `M🌈️window-layer-optics/🔭️visible/📦️packages/🦀️rust` | not in energy workspace members; build-owners entry |
| 3 | semio-s-artifact-energy-model-mut-air-loop-nodes | `M🌬️air-loop/🔀️nodes/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 4 | semio-s-artifact-energy-model-mut-plant-loop-design | `M🏭️plant-loop/🧭️design/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 5 | semio-s-artifact-energy-model-mut-people-gain-fractions | `M👥️people-gain/🍰️fractions/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 6 | semio-s-artifact-energy-model-mut-lighting-gain-emission | `M💡️lighting-gain/🔆️emission/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 7 | semio-s-artifact-energy-model-mut-ideal-loads-limits | `M💯️ideal-loads/🚧️limits/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 8 | semio-s-artifact-energy-model-mut-year-schedule-holidays | `M📆️year-schedule/🏖️holidays/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 9 | semio-s-artifact-energy-model-mut-site-ground | `M📍️site/🌋️ground/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 10 | semio-s-artifact-energy-model-mut-model-simulation | `M🔋️model/🧮️simulation/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 11 | semio-s-artifact-energy-model-mut-surface-properties | `M🧱️surface/🛠️properties/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 12 | semio-s-artifact-energy-model-mut-fenestration-geometry | `M🪟️fenestration/📐️geometry/📦️packages/🦀️rust` | not in energy members; build-owners entry |
| 13 | semio-s-artifact-energy-model-native-mutations-lifecycle | `M🧳️native-lifecycle/📦️packages/🦀️rust` | build-owners entry; `[package] description` missing (all siblings have one) |
| 14 | semio-s-artifact-energy-model-native-mutations-singletons | `M🧳️native-singletons/📦️packages/🦀️rust` | build-owners entry; `[package] description` missing (all siblings have one) |
| 15 | semio-s-artifact-stdio-ifc-catalog | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧩️composition/📦️packages/🦀️rust` | none blocking. Member of ifc root, build-owners entry present. Only sibling is `test-oracle`; its `[lints]` and `[package]` keys differ, not a layout violation |
| 16 | semio-hub-stdio-catalog-projection | `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/🏗️projection/📦️packages/🦀️rust` | none. Hub member, build-owners entry, sibling `catalog-definition` also has no project.json |
| 17 | semio-hub-stdio-catalog-definition | `🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️definition/📦️packages/🦀️rust` | none (same as 16) |
| 18 | semio-framework-os-store-engine-checkpoint-graph | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🌳️checkpoint-graph/📦️packages/🦀️rust` | none. Repo-root member, project.json plus build-owners entry, in `crate-boundaries` fixture |
| 19 | semio-framework-os-store-engine-branch | `…/⚙️engine/🌿️branch/📦️packages/🦀️rust` | none (same as 18) |
| 20 | semio-framework-os-store-engine-command | `…/🕰️history/🎮️command/📦️packages/🦀️rust` | none. In `crate-boundaries` fixture |
| 21 | semio-framework-os-store-engine-replay | `…/🕰️history/🏁️replay/📦️packages/🦀️rust` | none. In `crate-boundaries` fixture |
| 22 | semio-framework-plugin-tool-run-operations | `…/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/📦️packages/🦀️rust` | none. Repo-root member, project.json plus build-owners entry |
| 23 | semio-framework-plugin-job-receiving | `…/🔌️plugin/👷️job/🛬️receiving/📦️packages/🦀️rust` | Not in repo-root `members`. It sits in the framework root's `exclude` list only; 129 of 133 siblings are in repo-root members. Missing `[package.metadata.semio.rust-source-boundary]` (123 of 133 siblings have it) |
| 24 | semio-framework-plugin-decl-media | `…/🔌️plugin/📜️declaration/🎞️media/📦️packages/🦀️rust` | Missing `[package.metadata.semio.rust-source-boundary]` (4 of 4 siblings have it). Not in `surface-fixture.json` / `surface-fixture-lock.json`, which list the decl-views and decl-app peers |
| 25 | semio-framework-plugin-decl-staged-form | `…/📜️declaration/👁️view/staged-form/📦️packages/🦀️rust` | Missing `[package.metadata.semio.rust-source-boundary]` (4 of 4 siblings have it). Not in the surface fixtures (same as 24) |
| 26 | semio-framework-plugin-decl-tool-contract | `…/📜️declaration/👁️view/🛬️lifted/👷️job/📦️packages/🦀️rust` | none. Repo-root member, project.json plus build-owners entry. No same-parent siblings |
| 27 | semio-framework-plugin-instance-initialization | `…/🔌️plugin/🧵️instance/🌱️initialization/📦️packages/🦀️rust` | Missing `[package.metadata.semio.rust-source-boundary]` (10 of 11 siblings have it). Fixture: `feature-consumers.json` lists the instance-stores and instance-children peers (per-consumer closure, verify only) |
| 28 | semio-framework-plugin-instance-construction | `…/🔌️plugin/🧵️instance/🧱️construction/📦️packages/🦀️rust` | Missing `[package.metadata.semio.rust-source-boundary]` (10 of 11 siblings have it). Fixture note as for 27 |
| 29 | semio-framework-plugin-window-config-mutation | `…/🔌️plugin/🪟️window/🎚️config/📬️mutation/📦️packages/🦀️rust` | none. Repo-root member, project.json plus build-owners entry |
| 30 | semio-framework-plugin-window-config-registry | `…/🔌️plugin/🪟️window/🎚️config/🗂️registry/📦️packages/🦀️rust` | none. Repo-root member, project.json plus build-owners entry |

Abbreviation `…` in rows 18 to 30 stands for `🧰️framework/🛍️products/💻️os/🔨️modules/`.

## Findings in priority order

1. Membership (12 crates: rows 2 to 12 and 23). Energy crates are listed in `[workspace.dependencies]` of the energy root but not in its `members`. Cargo treats a package under a workspace root that is not a member as an error ("believes it's in a workspace when it's not"). Expect `cargo metadata` to fail for these until they are added to the owning `members` array. Cargo was not run (brief forbids builds); this is a static inference, verify with the owner. Row 23 needs an entry in the repo-root `members` next to its siblings. Its framework `exclude` entry already matches the siblings' pattern (130 of 133), so leave it.
2. Build owners (14 crates: rows 1 to 14). Energy crates have no `📋️build-owners.json` entry, while 50 of 52 siblings do with a `Cargo.toml` manifest.
3. Source boundary blocks (5 crates: rows 23, 24, 25, 27, 28). Add the block the same way the siblings do. Do not raise limits. Sibling values apply.
4. Fixtures (2 likely gaps: rows 24, 25). `surface-fixture.json` and `surface-fixture-lock.json` list the decl peers. Owner to confirm whether declaration crates belong there.
5. Minor. Rows 13 and 14 lack `description` in `[package]`, which every sibling has.

## Notes

- Path length: no path exceeds 256. The longest is about 207 UTF-16 units absolute (row 13).
- Path dependencies: all resolve, including dev-dependency paths such as `semio-framework-trace` from row 13.
- No `project.json` gaps for the framework crates: every framework crate that has a sibling project.json has one.

## Method and evidence

- Crate list: `git ls-files --others --exclude-standard -- '*Cargo.toml'` (30 entries), saved at `/private/tmp/claude-501/-Users-ueli-Documents-semio/f9862ee2-2e3c-43fc-9b16-d905a75045d7/scratchpad/new-crates.txt`.
- Analysis script: `/private/tmp/claude-501/-Users-ueli-Documents-semio/f9862ee2-2e3c-43fc-9b16-d905a75045d7/scratchpad/splitaudit/audit2.py`. Output: `.../scratchpad/splitaudit/out2.txt`, per-crate JSON `.../scratchpad/splitaudit/report2.json`, fixture hits `.../scratchpad/splitaudit/fixture-hits.json`.
- The first script run had a parser bug (missed table-form `[workspace.dependencies.<name>]`), which produced false dependency-resolution errors. The fixed run is the one reported above.
- Files written: this report only.
