# 🧪 Rust test map for generation3d (2026-11-25)

Read-only audit. No git-modifying command, no build, no browser. Only this report was written. Scratch parsing output lives in `/private/tmp/claude-501/` (outside the repo).

## Scope and method

- Roots: `✏️s/🔌️plugins/🌀️procedural` (generation3d, generation2d, foundation), `✏️s/🔌️plugins/🌊️flow` (hub artifact, extensions brep/math), `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow`, `🧰️framework/🔨️modules/🧵️job`.
- Manifests: 237 `Cargo.toml` in scope (13 are workspace-only). Test targets are read from explicit `[[test]]` blocks. Test counts are `#[test]` / `#[tokio::test]` regex counts in the target file, so harness files that pull tests in through `#[path]` modules show 0 (their real counts come from the notes).
- Run results come from the 2026-11-2x notes in this folder (56 files, the `📓️*2026-11-2*.md` set).

## Caveats

1. The requested folder `🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END` is not a normal ticket. It holds 1325 entries: about 675 `📓️` notes plus a large mirror of repo-root scripts. It has no `🎫️ticket.json` or `📌️important`. Its notes are used as-is. The older ticket `🎆️26/🌙️08/☀️29/PROCEDURAL-3D-END-TO-END` has `📓️status.md` and `🔍️research`.
2. "Changed since 2026-11-23" means mtime after 2026-11-23 00:00 in the four roots: 287 files (208 `.rs`). 272 of them are attributed to a crate by package directory (nearest `📦️packages/🦀️rust` parent). 15 are unattributed (mostly `Cargo.lock`, `🦀️.rs` harnesses and `🧪️` helper files). Attribution is heuristic. "No run recorded" means no note line that names the package together with a pass or test-result phrase.
3. The lane audit (`📓️lane-state-audit-2026-11-23.md`) and the 11-24 run-5 addendum (`📓️procedural-and-retained-tests-2026-11-23.md`) are the newest authority for generation3d. Later source edits (Drop-guard rewrite across 137 files, 11-24) were never re-run.

## Crate map (ranked by value for proving generation3d)

Shared command template (note `--parallelism=1`, not `-j`):

`repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path <MANIFEST> [--features testkit] [-p <PKG>] [--lib | --test <TARGET>] --no-fail-fast`

| # | Package | Manifest (relative to repo root) | Test targets | Features | Last recorded result (note) |
|---|---|---|---|---|---|
| 1 | `semio-s-artifact-procedural-generation3d` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` | pane-smoke, action-producer (3 expected), editor-routes, mutation-laws, host-laws, io-round-trip | none | RED. Run 5 (`📓️procedural-and-retained-tests-2026-11-23.md`, `proc-run20-all.log`): action-producer SIGABRT, editor-routes 6 ok/2 failed, host-laws 3/7, io-round-trip 55/7, mutation-laws 36/7, pane-smoke SIGABRT. Action-producer 3/3 was green in run 5 (`proc-run18`) before the Drop-guard rewrite. |
| 2 | `semio-s-artifact-procedural-3d-preview-eval` | `…/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/📦️packages/🦀️rust/Cargo.toml` | preview-channel-geometry (11 tests) | none | GREEN. Unit 20 and preview_channel_geometry 11 passed (`📓️procedural-and-retained-tests-2026-11-23.md`, Run 2, `proc-run3-a.log`). Also 20/20 in `📓️rust-law-verification-2026-11-22.md`. Changed 11-24 (5 files), not re-run. |
| 3 | `semio-s-artifact-procedural-3d-io-text` | `…/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml` | lib/tests via package | none | GREEN. 15 passed (Run 2, `proc-run3-a.log`). |
| 4 | `semio-s-artifact-procedural-3d-retained-commands` | `…/✏️editor/🧵️retained-commands/📦️packages/🦀️rust/Cargo.toml` | `--lib` (13 laws) | none | GREEN for lib. 13 passed, 0 failed (Run 2, `retained-commands-test-4h.log`). Earlier runs SIGABRT / E0432. 13 changed files since 11-23, not re-run. |
| 5 | `semio-s-plugin-flow-brep-tests` | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | 13 explicit targets, about 130 `#[test]` (original-brep-queries 31, solids 27, faces 25, booleans 8, mesh-workbench 18, sections 9, exchange 6, close 3, others) | none | NO RECORD. Brep extension build failed 11-24 (`📓️prototype-boot-2026-11-24.md`, E0277 `dyn ErasedSnapshotRetirement` not `Sync`). Brep is required by 9 examples (`📓️flow-extension-boot-needs-2026-11-23.md`). |
| 6 | `semio-s-artifact-flow-flow` (flow hub artifact) | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | set-contributions-registry (1), tree-projection-arena (1) | none | NO RECORD. 3 changed files since 11-23 (artifact `🪆️subsets` tree). Companion `…/✏️editor/🧪️tests/🔬️unit` (`semio-s-artifact-flow-flow-unit-tests`, testkit) is wired only as dev-dependency (`📓️flow-test-crate-leak-2026-11-24.md`). |
| 7 | `semio-os-flow` (framework flow hub) | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | flow_mesh_pack_wire (7), flow_retirement_law (2) | none | NO RECORD. Sub-crate `semio-os-flow-host` (13 changed files since 11-23) also has no recorded run. |
| 8 | `semio-framework-job-context` (plus `semio-framework-job`) | `🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml`; parent `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml` | context: 13 targets (≈41 tests); job: 23 targets (≈52 tests) | `testkit` (required by about 9 context targets and 20 job targets) | GREEN at last run. Lane 4 "all targets green" (`📓️handoff-2026-11-23-takeover.md`, `📓️job-context-completion-2026-11-22.md`, 75 passed whole-crate). 32 context files and 18 job files changed after 11-23, so they are not verified. |

Lower-ranked, for completeness:

- `semio-s-artifact-procedural-3d-editor` (`…/✏️editor/📦️packages`, testkit): no explicit `[[test]]`; 11 changed files; no recorded pass.
- `semio-s-artifact-procedural-3d-mutations`, `…-io`, `…-viewer`, `…-host`, `…-envelope`: 0 targets or tests in the notes; not run. Mutation kinds have 6 `[[test]]` with 0 test functions.
- `semio-s-artifact-procedural-generation2d` (`…/🗿️artifacts/🌀️generation2d`), `semio-s-artifact-procedural-2d-*`: manifests exist, 0 recorded runs for most; `…/2d-preview-eval` changed 3 files without a run.
- Foundation `✏️s/🔌️plugins/🌀️procedural/🫀️foundation` (`🌊️flow-source`, `🧬️generation`, `🧵️retained-source`, `🖼️semantic-ui`): no crate with explicit tests; about 7 `#[test]` files are not wired to any `[[test]]` (for example `🌊️flow-source/🔢️digest/🧪.rs`, `🧬️generation/🧵️fold/🧪️tests.rs`). Not run.
- Flow extensions (brep component, math, list, draw, dictionary, text, bim, primitive, logic): component manifests exist and changed. `semio-s-plugin-flow-math` and `…-math-component` have no `[[test]]`. `🧮️math/🧪️tests/🔬️extension-guest-standalone/🦀️.rs` is not wired. Not run.
- Hub composition `🌎️hub/🧩️compositions/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (`semio-hub-flow`) is outside the four roots. It was not inventoried here.

## Changed since 2026-11-23 with no pass recorded in notes

Attributed crates with recent changes and no note line that pairs the package name with a pass result (count = changed files):

- `semio-os-flow-host` 13, `semio-os-flow-host-read` 12, `semio-s-artifact-flow-flow-editor-retained` 11, `semio-s-artifact-flow-flow-editor` 11, `semio-s-artifact-procedural-3d-editor` 11
- `semio-os-flow-host-source` 7, `semio-os-flow` 6, `semio-framework-job-worker` 6, `semio-framework-job-return-budget` 5, `semio-framework-job-receiving` 5, `semio-s-artifact-procedural-2d-editor` 5
- `semio-framework-job-payload` 4, `semio-framework-job-park` 4, `semio-s-plugin-flow-brep-component` 4, `semio-s-artifact-procedural-3d-viewer` 4, `semio-s-artifact-flow-flow-editor-publication` 4
- `semio-s-artifact-procedural-3d-mutations` 3, `semio-framework-job-testing-caller` 3 (only 2 mentions, no pass), `semio-framework-job-native-turn` 3, `semio-s-plugin-flow-brep-original-jobs` 3, `semio-s-artifact-flow-flow-viewer` 3, `semio-s-artifact-procedural-2d-viewer` 3, `semio-s-artifact-procedural-2d-preview-eval` 3, `semio-s-artifact-procedural-3d-editor-panels` 3, `semio-s-artifact-procedural-2d-mode-edit` 3
- Crates with 1–2 files each and no pass: about 60 more (all `semio-s-plugin-flow-*-component`, `semio-s-artifact-procedural-3d-*` mode and command crates, `semio-framework-job-{failure,return,outcome-borrow,work-grant,capture,registry}`, `semio-os-flow-wasm`, `semio-s-plugin-flow-math`, `semio-s-procedural-core-semantic-ui`).

Changed crates that do have a pass recorded, but only before the change:

- `semio-framework-job-context` (32 files, last green 11-23), `semio-framework-job` (18 files, `original-fixture-caller` 5/5 on 11-22)
- `semio-s-artifact-procedural-3d-retained-commands` (13 files, lib 13/13 on 11-23)
- `semio-s-artifact-procedural-3d-preview-eval` (5 files, 20/20 on 11-22 and 11-23)
- `semio-s-artifact-procedural-3d-io-text` (2 files, 15/15 on 11-23)
- `semio-s-artifact-procedural-generation3d` (6 files): RED at last run, despite a single keyword hit.

## Example fixtures (the eight in the wgpu matrix)

Source of truth: `📓️battery-run-plan-2026-11-23.md` line 74 and `🐍️wgpu-example-matrix-probe.mjs:55`. All eight live under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/<slug>/` and each has `🧪️tests/🧫️fixtures/🧩️example/🔣️.json`.

The law that asserts each example evaluates is the bun file `🧪️tests/🧩️example/🟦️.ts` (`describe(<slug>)`). Its `it` "recomputes the committed … from the dsl sliders" checks the evaluated geometry against the fixture. The Rust `🧪️tests/🧩️example/🦀️.rs` asserts `primary_asset_is_nonempty`.

| Example | Evaluation law (bun, `🧩️example/🟦️.ts`) |
|---|---|
| hexagonal-mushroom-column | regular-polygon prism volume from sliders |
| rectangle-extrude-volume | prism volume and extent from sliders |
| rectangle-wire-preview | wire perimeter and extent from sliders |
| box-shell-preview | hollow-box volume from sliders |
| box-fillet-preview | rounded-box volume from sliders (Rust also: `inference_determinism_law`, `inference_of_the_example_is_a_non_trivial_acyclic_topology`) |
| sphere-cut-with-torus | difference volume by own quadrature |
| sphere-box-fuse | union volume by own quadrature |
| face-sweep-extrude | prism volume and extent from sliders |

Not in the matrix: `🥽️mesh-workbench` (ninth directory). It has no `🧫️fixtures` and no volume law, only one bun test "wires creation, editing, analysis and geometry preview". No note records a run of these example laws by name.

## Recommended run order

See the final chat reply. The top 8 commands are the ones in the crate table, in the same order.

## Gaps to close

1. Brep component compile (E0277) blocks rows 5 and 6 and the brep examples. Fix before any end-to-end claim.
2. Re-run generation3d product (row 1) after the Drop-guard rewrite. Its 6 targets have never been green together.
3. Re-run job-context and job (row 8) after the 11-24/25 edits.
4. Wire the unwired `🧪️` files (foundation, math, job `🧪️tests/🔬️…` helpers) into `[[test]]` or drop them.
