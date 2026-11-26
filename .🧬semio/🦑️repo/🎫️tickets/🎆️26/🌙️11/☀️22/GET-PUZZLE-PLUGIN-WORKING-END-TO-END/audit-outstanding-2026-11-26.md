# Puzzle Plugin Outstanding Audit, 2026-11-26

Read-only audit. Nothing was built, run or edited; the only file written is this report. Rust counts are static attribute counts over `.rs` files (not executed). Owner attribution uses the nearest folder that contains a `📦️packages` directory, because the crates mount sources with `#[path]` and their `🦀️.rs` lib files hold no tests. Newest reports: `current-verification.md` and `law-review.md` (05:23 and 05:28, 26.11.), `guest-component-verification.md` (05:28), `ui-*.md` (26.11. 05:08 back to 25.11. 20:27), `p1-3d.md` (25.11. 16:53), `p2-2d.md` (25.11. 17:27), `p3-5d.md` (25.11. 17:09). Other `T/*.md` files (`p4-wasm.md`, `p5-live.md`, `p6-interaction.md`, `lane1-ts-and-laws.md`, `disk-2026-11-25.md`, `paged-utf8-projection.md`, coordination and audit notes) were not in the requested set and were not read.

Status key: OPEN = no later report records a pass. CLAIMED = a newer report states a pass; not re-verified here. STALE = older report superseded by a newer one.

## 1. Registered puzzle projects

Source: `project.json` / `📋️project.json` files under `✏️s/🔌️plugins/🧩️puzzle` and `🌎️hub/🧩️compositions/🧩️puzzle`. All nine are registered; `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🦀️rust` does not exist.

| # | Project name | Project directory | Targets |
|---|---|---|---|
| 1 | `@semio-tech/puzzle-js` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript` | `test`, `publication-authority-audit` |
| 2 | `@semio-tech/puzzle-2d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust` | `build`, `check`, `graph-generate`, `graph-wire-check`, `preview-generated`, `test`, `test-presence-source`, `test-presence-native`, `test-snapshot-sqlite`, `test-snapshot-sqlite-source`, `test-snapshot-sqlite-native`, `verify-snapshot-sqlite-source`, `test-browser-time-travel` |
| 3 | `@semio-tech/puzzle-3d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust` | `build`, `check`, `graph-generate`, `graph-wire-check`, `preview-generated`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-source`, `test-snapshot-sqlite-native`, `verify-snapshot-sqlite-source`, `verify-import-acceptance`, `verify-original-work-construction`, `verify-original-fill-fixture`, `verify-original-fill-fixture-native`, `verify-runtime-lifecycle`, `test-browser-lifecycle`, `test-browser-wgpu-clipboard` |
| 4 | `@semio-tech/puzzle-3d-rs-compute-ownership` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏛️ownership/🧮️compute` | `test` |
| 5 | `@semio-tech/puzzle-5d-rs` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust` | `build`, `check`, `graph-generate`, `graph-wire-check`, `preview-generated`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-source`, `test-snapshot-sqlite-native`, `verify-snapshot-sqlite-source`, `verify-import-acceptance`, `verify-edit-acceptance`, `verify-catalog-fields`, `verify-inline-capacity`, `verify-optional-fields`, `verify-original-work-construction` |
| 6 | `@semio-tech/puzzle-5d-react` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript` | `test`, `verify-lifecycle` |
| 7 | `@semio-tech/s-puzzle-composition-tests` | `✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript` | `verify-puzzle-fill-policy-self-tests`, `verify-browser-contribution` |
| 8 | `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust` | `wasm`, `test`, `test-guest-io`, `fixtures-lint`, `canonical-architecture` |
| 9 | `@semio-tech/hub-puzzle-retained-test` | `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained` | `test-native`, `test-ownership` |

Note: `puzzle-3d-rs` and `puzzle-5d-rs` run `verify-*` targets through `repo ./📜️script.ts verify ...`; `puzzle-3d-rs` also has two browser `verify time-travel` targets (`--renderer react` and `--renderer wgpu`).

## 2. Rust test targets and `#[test]` counts (2D, 3D, 5D workspaces)

Workspace roots: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/Cargo.toml`. Counts are per whole workspace tree (every `.rs` file under the root), so they cover every member package.

| Workspace | Members | Files with tests | `#[test]` | `#[async_test]` (incl. `#[semio_framework_async_macros::async_test]`) |
|---|---:|---:|---:|---:|
| 2D | 32 | 214 | 1,221 | 63 |
| 3D | 36 | 147 | 833 | 298 |
| 5D | 30 | 131 | 764 | 99 |
| **Total** | **98** | **492** | **2,818** | **460** |

Declared `[[test]]` targets (explicit Cargo test targets; all other test code is in lib or `mod` mounts):

| Workspace | Package | `[[test]]` target path (relative to package) |
|---|---|---|
| 2D | `semio-s-artifact-puzzle-2d-mutations` | `../../../../../🧬️mutations/🧪️tests/📝️typed-json/🦀️.rs` |
| 2D | `semio-s-artifact-puzzle-2d-text-inverse` | `../../../../../../🚪️io/📤️export/📦️owned/🧪️tests/🦀️.rs` |
| 3D | `semio-s-artifact-puzzle-3d-editor-session` | 2 targets (`…/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs`, `…/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`) |
| 3D | `semio-s-artifact-puzzle-3d-editor` | 8 targets (`action-producer`, `task-resume`, `unit`, `defects`, `manifest`, `panels`, `budgets`, `gestures`) |
| 3D | `semio-s-artifact-puzzle-3d-mutations` | 7 targets (`object`, `pose`, `vortex`, `volume`, `reference`, `catalog`, `selection`) |
| 3D | `semio-s-artifact-puzzle-3d` | 1 target (`🧪️tests/🗃️snapshot-sqlite/🦀️.rs`) |
| 3D | `semio-s-artifact-puzzle-3d-host` | 1 target (`🧪️tests/🦀️.rs`) |
| 5D | none | 0 |

Per-owner breakdown (owner = folder with a `📦️packages` child that contains the test file; sorted by `#[test]` count):


**◻️2d: owners=26 #[test]=1221 async=63**

| Owner folder (relative to dimension root) | `#[test]` | `#[async_test]` | Files |
|---|---:|---:|---:|
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 650 | 3 | 104 |
| `🏅️standards/🔖️1` | 168 | 8 | 39 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection` | 118 | 0 | 19 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine` | 95 | 0 | 9 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host` | 36 | 0 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit` | 23 | 48 | 8 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles` | 23 | 0 | 4 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🔒change-node-locked` | 21 | 0 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/🆔change-manifest-id` | 16 | 0 | 2 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog/📚replace-kind-catalogs/🧪️tests/📇️installs` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️edge/🧮replace-edge-geometry/🧪️tests/📍️repositions-edge-1` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️derives` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window` | 7 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences` | 7 | 0 | 3 |
| `🧪️tests/🔬️unit` | 5 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime` | 5 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🧊replace-node-geometry/🧪️tests/🚫️rejects` | 5 | 0 | 1 |
| `🤖️generated/🏢️nakagin` | 4 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare` | 3 | 1 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose/📍move-node/🎮️prepare` | 3 | 1 | 3 |
| `🤖️generated/◻️puzzle2d-default` | 3 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone` | 2 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🧪️tests` | 1 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse` | 1 | 1 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot` | 1 | 0 | 1 |
| `.` | 0 | 1 | 1 |

**🧊️3d: owners=31 #[test]=833 async=298**

| Owner folder (relative to dimension root) | `#[test]` | `#[async_test]` | Files |
|---|---:|---:|---:|
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection` | 150 | 0 | 23 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor` | 98 | 277 | 37 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️object` | 72 | 0 | 9 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀️vortex` | 64 | 0 | 8 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️reference` | 64 | 0 | 8 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️volume` | 64 | 0 | 8 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️catalog` | 40 | 0 | 5 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry` | 38 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill` | 33 | 0 | 4 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush` | 32 | 0 | 2 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io` | 28 | 2 | 7 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels` | 27 | 0 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute` | 24 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️pose` | 24 | 0 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit` | 10 | 0 | 4 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 10 | 12 | 3 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences` | 7 | 5 | 3 |
| `🧪️tests/🔬️design-parity-schema` | 5 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session` | 5 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology` | 5 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer` | 5 | 0 | 2 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema` | 4 | 0 | 2 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples` | 4 | 0 | 2 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot` | 3 | 0 | 1 |
| `🤖️generated/🧊️puzzle3d-default` | 3 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config` | 2 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action` | 2 | 0 | 1 |
| `🧪️tests/🔬️manifest-catalog` | 1 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧪️every-example` | 1 | 0 | 1 |
| `🔨️modules/🏠️host/🧰️owned` | 0 | 2 | 2 |

**🖐️5d: owners=15 #[test]=764 async=99**

| Owner folder (relative to dimension root) | `#[test]` | `#[async_test]` | Files |
|---|---:|---:|---:|
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 505 | 1 | 74 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor` | 129 | 85 | 26 |
| `🏅️standards/🔖️1` | 63 | 3 | 16 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema` | 11 | 0 | 4 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🧪️tests/📍️moves-part-a` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/🌱️appends-part-c` | 8 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds` | 8 | 0 | 1 |
| `🧪️tests/🔬️unit` | 7 | 0 | 1 |
| `🔨️modules/♻️rewriting` | 6 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit` | 6 | 10 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🧪️tests/🚫️rejects` | 5 | 0 | 1 |
| `🤖️generated/🖐️puzzle5d-default` | 3 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️host-admission` | 2 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records` | 2 | 0 | 1 |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog` | 1 | 0 | 1 |

## 3. Outstanding, pending, queued, red or failing items

Deduplicated across the requested reports. Source abbreviations (all in the ticket folder): `CV` = current-verification.md, `LR` = law-review.md, `GCV` = guest-component-verification.md, `UI3L` = ui-3d-latency.md, `UIB` = ui-browser.md, `UIH` = ui-history.md, `UIL` = ui-lookup.md, `UIR` = ui-replay.md, `UIW` = ui-window.md, `UIP` = ui-painted.md, `P1` = p1-3d.md, `P2` = p2-2d.md, `P3` = p3-5d.md. Line numbers refer to those files.

### 3.1 2D

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| 2D-1 | Mounted 100-placement fill: last run reached 51, then the release grant ran out after 1,050.32 s. Grouped-receiver and short-UTF-8 header fixes await a linked rerun. | OPEN | CV 7, 134, 142, 148 |
| 2D-2 | Mounted 8-placement fill passed once (one undo entry). Its interactive trace threshold is open: 8 KiB page gave worst step 2.225 ms against the 2 ms limit; 4 KiB run measured 2.196 ms. | OPEN | CV 7, 134 |
| 2D-3 | Native preparation: 54 of 55 pass after the reversed delete-inverse fix; the rerun of the full 55-case selection is pending. | OPEN | LR 168, CV 7 |
| 2D-4 | Native history/edit suite: 35 of 44 (CV 66), then 38 of the 45-case run passed with failures in remote transport, history preview, mounted fill and a 2.188 ms cold step. No later full green recorded. | OPEN | CV 66, 122 |
| 2D-5 | Long mounted replay (900 operations) and second selected preview stalled (12 of 915; 539 of 915 after 100,000 turns). Turn budget raised to 225 per operation; mounted behaviour explicitly unverified. | OPEN | UIR 21, 23, 35 |
| 2D-6 | Remote author stalled in Finalizing; mounted native history command queued on the shared Cargo lock. | OPEN | UIR 21, 23, 33, 53 |
| 2D-7 | Mounted two-peer presence, idempotence and convergence: UIR says queued; CV 7 (newer) says all three mounted two-peer tests passed. | CLAIMED (CV 7) | UIR 47, 49, 53; CV 7 |
| 2D-8 | History/edit integration rerun, full history replay, released operations and live editors still listed as pending. | OPEN | CV 28, 30, 54 |
| 2D-9 | Mounted fill integration for the six focused native fill tests: pending. | OPEN | CV 34 |
| 2D-10 | Frozen history/edit suite fails 22 of 44; seed-board refusal `field owner has no retained source projection` (5 tests) not root-caused. Framework projection and editor fixture repair tracked separately. | OPEN | UIL 17; P2 31, 33 |
| 2D-11 | Native lookup regression: narrow `current-native-tests.sh` run timed out twice on the shared artifact lock. Full SQLite oracle file: 61 pass, 3 fail (delete cascade, delete candidate, child preparation). | OPEN | UIL 9, 11 |
| 2D-12 | Declared-JSON importer: native regression pending. Fallback run passed (3 tests). | OPEN (native) | CV 18; UIL 15 |
| 2D-13 | Preparation owned cursors: delete inverse and candidates were over the inline 4,096-byte bound in earlier layouts; latest 54/55 leaves the delete-inverse ordering case. | OPEN | LR 72-86, 152-168 |
| 2D-14 | Board-host 36 passed and engine 27 passed (P2). Not green or not rerun: snapshot-sqlite bun test, board-normal-host, puzzle root workspace, tambour-single-storey, tambour-end-storeys, interaction, v1-editor-tests (E0433 fix pending rerun in summary4). | OPEN | P2 8, 10, 15, 22, 24 |
| 2D-15 | Retirement-authority refusal fix (window command and snapshot export worker frame size): 2D history/edit integration rerun queued. | OPEN | CV 28 |
| 2D-16 | Shared 2D build E0599 `OwnedCorePollLimits::from_scalars`: reported during concurrent edits; method present in current sources; targeted reruns queued. | STALE (resolved in source) | LR 22; UIL 11 |

Dimension not stated in source, reported as 2D or 3D by context (Nakagin fill latency; CV 80 and 84 mention board-preview fixes, so likely 2D): cold latency run stopped after 10,191 previews, result pending (CV 80); slowest minimum 2,135 µs against a 2,000 µs ceiling in an older executable (CV 84).

### 3.2 3D

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| 3D-1 | Eight-turn publication law RED. Translate 14 publication turns, delete 17/16 (both Nakagin and Concrete Forest). | OPEN | UI3L 9; CV 8 |
| 3D-2 | Eight-turn ladder law RED. Measured: 59/62 (LR 5, 17); 59 translate and 61 delete (LR 17); 78 turns / 803 units with 59 ladder turns and 76 turns / 941 units delete (P1 50, 94); 55/57 (LR 124); publication 17 turns and retirement 6-7 grants at 64-item grant (CV 122, 132). | OPEN | LR 5, 17, 124; P1 50, 94; CV 122, 132 |
| 3D-3 | Structural fixes not implemented: about 10x cut in store staging (561 one-item staging units per translate), and pricing of the retirement of the typed operation per fixed frame (720 of 1,532 items are GrantedMounted retirement). Expected ladder 59 to about 49, still above 8. Proposal only, sits in framework store layer. | OPEN | P1 54, 86-88 |
| 3D-4 | Gesture law `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`: three runs exit 124 after waiting for the artifact lock (1,200 s, 2,400 s, 3,600 s). | OPEN | LR 16, 19 |
| 3D-5 | Mounted latency run: retirement still uses the generic cursor for unused segmented-output backing; mounted runtime run queued. | OPEN | UI3L 11, 15, 21 |
| 3D-6 | Two-instance Memory test failed with two objects where three expected (refreshed run). Current-implementation integration run and full dispatch path pending. | OPEN | CV 132, 136, 140 |
| 3D-7 | Mounted fill stalled at 262,144 host turns in presentation; fix applied, mounted 3D editor rerun pending. Source-owner retirement at turn 9,080: fix applied, scenario open. | OPEN | CV 96, 98, 100, 102 |
| 3D-8 | Mounted fill with stale provisional styling after finalize: driver acknowledgement fix in tree; no rerun recorded. | OPEN | CV 116 |
| 3D-9 | Hover-storm: mounted job retirement failed; native rebuilt test passed; original mounted test queued again. | OPEN (mounted) | CV 86, 94 |
| 3D-10 | 3D panels: full 92-case run 89 pass (CV 116); two remaining panel cases passed later (CV 132); CV 8 claims all 92 have passing evidence across runs. No single green run. | CLAIMED | CV 8, 116, 132 |
| 3D-11 | Editor-surface suite: 66-case run 63 pass, 2 fail, 1 running (CV 90); later reruns report 66 distinct passes (CV 96). | CLAIMED | CV 8, 90, 96 |
| 3D-12 | Agent lane: mounted regression queued; native clean suite of 8 commands passed. | OPEN (mounted) | CV 44, 72, 82, 96 |
| 3D-13 | Native Nakagin suggestion extent and 3D scene regressions passed; popup exact-window authority fault: focused scene test queued, later three world-scene regressions passed. | CLAIMED | CV 42, 52, 72 |
| 3D-14 | (dimension inferred from context) Nakagin 120-placement latency passed five cold replays under 2,000 µs in 226.51 s; the native suite with the last capture change is queued. | OPEN (suite) | CV 92 |
| 3D-15 | `p1-3d.md` is still marked IN WORK with no suite green; its last measurement is 16:52 (4 ms wall, identical 78/803 turns). | OPEN | P1 1, 3, 91-97 |
| 3D-16 | Settlement law (200 turns) and new multi-item Close receipt assertion passed (2.52 s). | CLAIMED | LR 123-124; CV 110 |

### 3.3 5D

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| 5D-1 | Single fresh all-cases 5D run (111 cases) still pending; the earlier runs were 110 of 111 (create-part option refusal, fixed later) and a full ordinary history corpus passed in 901.50 s. | OPEN | CV 9, 110, 118, 130; LR 150 |
| 5D-2 | Mounted pause/step scenario (`semio-plugin` native graph): outstanding. | OPEN | CV 38 |
| 5D-3 | Editor full rerun after the catalog-child archive export fix: pending. | OPEN | CV 40 |
| 5D-4 | Rewrite-port and result-page presentation fixes: verification queued. Testkit production roster fix: rerun not recorded. | OPEN | CV 36, 46 |
| 5D-5 | Mounted multi-window camera restoration test in progress (single-camera probe passed at 21,996 turns). | OPEN | CV 42 |
| 5D-6 | Canonical history executable: runtime timeouts, SIGTERM and lock waits; later corpus pass recorded. | CLAIMED | LR 36, 38, 108, 140, 142, 150 |
| 5D-7 | Native 5D package sweep (`p3-5d.md`): `v1-unit-tests` blocked at `semio-s-artifact-stdio-pdf-model` (SIGTERM and SIGKILL). Not run: host-admission-tests, 29 crates, snapshot-sqlite, five verifiers, both p6 tests, standards-tree owners, hub crate. | STALE (per-case claims in CV supersede) but the package list is still not run | P3 28, 32-38 |
| 5D-8 | Older 5D unit binary (32 MiB stack abort; 29 pass, 8 fail) cannot establish current behaviour. | STALE | CV 32 |
| 5D-9 | Window replay turn allowance and 5D window restore: passing binary. | CLAIMED | UIW 15 |

### 3.4 Guest (WASI component, guest I/O, descriptors)

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| G-1 | Fresh `wasm32-wasip2` component not finally linked. Twelfth attempt on the updated offline locked graph pending; fresh component, descriptor comparison and lifecycle proof pending. | OPEN | GCV 53; GCV 35, 37, 39, 43 |
| G-2 | Descriptor drift against committed metadata: three asset hashes, media type of three 5D examples, precision of 3D default layout widths, missing 5D `rewrite:in` port. Shipped verification requires equality; prototype permits drift. | OPEN | GCV 7 |
| G-3 | Production guest I/O (`@semio-tech/puzzle-plugin:test-guest-io`) blocked by `EPERM` on the repo graph Unix listener (refs 027db973, 2e71dfd8); model-owning guest I/O test blocked on graph descriptor before publication. | OPEN | GCV 9, 31, 41; CV 64, 76 |
| G-4 | No genuine build-model receipt: standalone Cargo Wasm cannot be adopted into the model store. | OPEN | GCV 21, 41 |
| G-5 | Frozen WASI actor trap: out-of-bounds memory access (stack 1,048,576 bytes starting shadow; 8 MiB patch is diagnostic only). Fresh unmodified candidate required to attest lifecycle. | OPEN | GCV 33, 45, 47 |
| G-6 | Lifecycle on frozen component: `createAlternative` returned receipt but no active alternative; 122 running replies then completion; guest VCS error `no checkpoint for alternative` on empty editor; `addNode` refused with `UnsupportedOwner`; 2D closure with unfinished calls. Checkpoint-bearing guest edit under verification. | OPEN | GCV 47, 49, 51; CV 164 |
| G-7 | Frozen WASI edit publication and source close under investigation; actor-input split unresolved import at tenth attempt. | OPEN | CV 10; GCV 53 |
| G-8 | Full prototype component and wasm-bindgen step: still require verification. Unknown-target history-schema locals now 16,254 (was 59,535). | OPEN (component) | CV 22; GCV 3 |
| G-9 | Model-owning guest I/O test and official guest test: stopped at source capture; repo graph listener denied. | OPEN | CV 64, 76 |
| G-10 | Guest transport: Bun fd3 and subprocess transport tests pass; registered Puzzle guest I/O advanced to `source.served`. | CLAIMED | GCV 25-29 |
| G-11 | Ninth invocation stopped on E0599 `RetirementQueue::admit_owned_slot_demand` (live method newer than compiled queue); tenth retry on the warm graph, JCO watcher following it; no receipt yet. | OPEN | GCV 35, 39 |

### 3.5 Browser (Storybook, Playwright, Chromium)

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| B-1 | Chromium refuses to launch in the sandbox (SIGTRAP, `sandbox_host_linux.cc:41`). Browser screenshot and interaction assertions unverified. | OPEN | UIB 5, 7, 30-32; CV 24 |
| B-2 | Storybook dev server cannot bind loopback (`uv_interface_addresses` errno 1, EPERM listener). | OPEN | UIB 5; CV 24 |
| B-3 | Storybook repo run timed out after 300 s (owner target and repeated run); registered backend stopped on removed grant-growth API (service ref d014ef76). | OPEN | CV 24, 70, 152 |
| B-4 | Browser execution of 144 Playwright cases (13 suites) remains a separate check; direct catalog run passed 60 Vitest tests. | OPEN | CV 10, 154 |
| B-5 | Renderer test through repo route failed (graph daemon lock under `/run/user/1000` read-only); native alternative passed 8 tests. Repo route red. | OPEN (tool) | UIB 23, 24 |
| B-6 | Native TypeScript check (`p4-tsconfig.json`) timed out at 100 s with no diagnostics. | OPEN | UIB 26 |
| B-7 | Fresh browser and guest builds; staged browser WASM probe predates current changes. Live editor, GPU canvas, input transport, archive restart and publication receipt unverified. | OPEN | CV 10, 24; UIB 30-32 |
| B-8 | Puzzle Playwright counts disagree between sources (20 puzzle cases and 23 in ticket config, UIB 17-18; 144 cases in 13 suites, CV 154). | NOTE | UIB 17-18; CV 154 |

### 3.6 Framework and tooling (shared by all dimensions)

| ID | Item (latest evidence) | Status | Source |
|---|---|---|---|
| F-1 | Shared Cargo artifact lock: native runs exit 124 after 750 s, 1,200 s, 2,400 s, 3,600 s waits (2D, 3D, 5D). | OPEN | LR 18-19, 23, 32, 38; UIL 11; UIR 47, 53; CV 20 |
| F-2 | rustc killed (SIGKILL or SIGTERM) on large crates (`pdf-model`, `semio-ui-contract`, `semio-plugin-instance-jobs`, `semio-s-artifact-stdio-pdf-model`). | OPEN | P3 32-38; P2 13; LR 142; GCV 35 |
| F-3 | Launcher and sandbox: read-only cache, download and process descriptor errors; Unix listener EPERM for the graph service. | OPEN | CV 20, 24, 64, 76; GCV 31 |
| F-4 | `repo test-file` rejects Rust sources by design. | NOTE | LR 15; UIR 13 |
| F-5 | Shared-tree foreign edits broke compiles (E0433 `semio_plugin_store_lifecycle`, E0460 archive identity, E0599 `admit_owned_slot_demand`, E0599 `ready_for_owned`). Last recorded one retried in tenth invocation. | OPEN (latest) | GCV 17, 19, 37, 39; P2 24; P1 70 |
| F-6 | Measurement validity: P1 and P2 runs used masked debug hooks (`mask-dbg.sh`) and frozen 11:28 builder copies; evidence is for those states only. | NOTE | P1 15-17, 39-42; P2 13, 23 |
| F-7 | Process: `P1` line 10 records a `pkill -f` against the agent's own queued run, which the brief forbade (rule 3). | NOTE | P1 10 |

## 4. Superseded or resolved in newer reports (not outstanding)

| Item | Older source | Newer evidence |
|---|---|---|
| 3D `higher-ranked lifetime error` in `semio-s-artifact-stdio-semio-brep-snapshot` | P3 27 | P3 31 (compiles) |
| 5D history corpus `timeTravel.unchanged` refusal (create-part option) | LR 128-134; CV 110 | CV 118, 130; LR 150 |
| 2D retained-command and fill refusals (`retained command input requires its original app-owned retirement authority`) | UIH 3, 9 | UIH 19 (final fill-only run passed six native tests) |
| 2D `Window owner must declare its original disposer birth` (19 tests) | P2 29 | fix in tree; P2 31 cause 2 remains |
| Older WASI component `59,535` locals (over 50,000 limit) | GCV 3 | CV 22 (fresh build max 16,254) |
| 2D inference duplicate-member case | CV 26 | passed (20 tests) |
