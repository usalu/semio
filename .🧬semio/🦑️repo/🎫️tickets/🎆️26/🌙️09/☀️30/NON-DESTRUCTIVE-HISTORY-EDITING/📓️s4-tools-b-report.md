# 📓️ S4-TOOLS-B Report — controls (forms, gis, energy, playbook, press glue) + procedural (generation 2d/3d)

Successor of S3-CONTROLS (`📓️w3-t2-controls-report.md`) and S3-PROCEDURAL (`📓️w3-t2-procedural-report.md`).
Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`; fleet rules 1–38; design §13.1, §17.1, §19, §20.1, §20.9–20.11, §20.13, §20.15.

## Session 4 — 2026-10-04

### S4.1 Repair-first (rule 34) — 02:15

- Inherited reports read: controls last section S3.12 (playbook design, 11:50 10-03), procedural last sections S3.7/S3.8.
- Half-finished edit found: the S3-CONTROLS predecessor STARTED the playbook §20.15 conversion between 12:01 and 12:07 on 10-03
  (cut at 12:07): artifact root `📖️playbook/🦀️.rs` rewritten (ContentBridge chain order, ComposeOnRead `playbook_composed_spec`,
  ChildLane leaf builders), demo example + flow asset, snapshot schema family (sqlite, TS, JSON/proto/GraphQL, text grammar, diff
  text). The editor, commands, the 8 parent leaves, viewer, io and tests were NOT converted yet → the crate is mid-conversion.
- No other owned tree carries a half edit (procedural S3.9/S3.10 and controls S3.10/S3.11 completed before the cut; files touched
  10-04 00:32 are the Codex peer's value/DSL sweep).

### S4.2 Playbook §20.15 completion — in progress (11:55)

Session clock note: this agent was suspended between ~02:25 and ~11:50; coordinator rules 39–43 read (checks only, gate v3, ABI freeze).

Done in source (compile pending, crate is mid-conversion = INFRA's wasip2 census "playbook 55"):
- `PlaybookMutation` = `ChangeTitle` only (`KINDS = ["change-title"]`, re-exports, `seed_playbook_scene_json` deleted); schema facets
  rewritten schema-first: mutations `🔣️.json` (oneOf change-title), `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`, binary `📡️.protocol.semio`
  (`change-title tag=0`, greenfield renumber; leaf `🔣️.json` `binaryTag: 0`), text grammar `.semio`/`.g4`/`.ebnf`.
- DELETED (rule 32; repo-wide grep: only generated `📚️library/🔣️schema-catalog.json` (16 rows → central `schema generate`) and the
  describe-generated `🌎️hub/🧩️compositions/📖️playbook/🔣️.json` still name them): leaf dirs `🧬️schema/🧬️mutations/{➕add-step,
  ➖remove-step,↔️move-step,🧱add-block,🗑️remove-block,🔀move-block,🔄replace-block,🩹update-step}` (6 files each) and their fixtures
  `🧫️fixtures/🧬️mutations/<same>` (5 files each).
- Next: editor (child-lane commands through `playbook_*_leaves` + `playbook_flow_emit`, publication lane `Child`, reducer reads
  `context.children`), viewer/windows over `playbook_composed_spec`, inference over the parent, tests + `composed_reload_law!`,
  change-title fixtures without `document`, oracle catalogs, mutate-playbook-1 case.

### S4.3 Playbook §20.15 — source complete (12:30)

- Root `📖️playbook/🦀️.rs`: ComposeOnRead (`playbook_flow_content`, `playbook_composed_spec`, genesis catalogue,
  `playbook_child_restore_projection`, `playbook_fault_notices` — 7 three-segment codes `playbook.{flow.unavailable,flow.dialect,
  flow.content,flow.projection,step.missing,step.duplicate,block.missing}` en/de) and ChildLane builders
  (`playbook_{add,remove,move}_step_leaves`, `playbook_{add,remove,move}_block_leaves`, `playbook_edit_blocks_leaves`,
  `playbook_blocks_leaf`, `playbook_chain_leaves`, `playbook_flow_emit`); 8 leaf module mounts removed.
- Editor: the six structural verbs publish on `ArtifactToolPublicationLane::Child` (one `flow` child edit each, through
  `playbook_child_leaves_emit`), the retained reducer composes `ArtifactView::with_children(.., context.children)
  .bound_to_operation(..)`, ids minted from the admission's authoring seed (`playbook_minted_id`, `store::content_id`), topology /
  render / `import_media` compose on read (uncomposed child = named fault), `child_restore_projection` + `fault_notices` declared,
  `app.command.unsupported` / `app.command.tool-mismatch` framework codes replace the anonymous faults (S4-GATES routing).
- Viewer: composed steps tree, `child_restore_projection` + `genesis_child_pack` declared. Windows take `&PlaybookSpec`.
- Inference: pure over the parent (`reads: ["flow"]`), the topology of no steps until AGNOSTIC W-b hands child packs.
- Tests rewritten: root (child projection, lossless bridge, chain-order law, undecodable-blocks fault, **child-leaf vectors law**
  folding + undoing every vector), editor (`live_spec` over the child store, `composed_reload_law!("playbook", …)`, topology over
  composed children + uncomposed refusal, undo/redo over the child lane, two-instance convergence on the child, import emit shape,
  example verb host-only + genesis, notice table), commands, windows, viewer, inference, snapshot codecs, mutation leaf laws,
  mutate-playbook-1 case (Rust adapter + Python reference + feature → change-title only).
- Language-agnostic vectors `🗿️artifacts/📖️playbook/🧫️fixtures/🧫️child-leaves/🔣️.json` (9 cases) written by the independent Python
  implementation `T/🧪️s4-tools-b-playbook-child-leaves.py` (`--check` exit 0).
- Fixtures re-sealed: change-title before/after/diff without `document` (child id `playbook-flow`); oracle catalog kinds/vectors/
  manifest → change-title; DELETED `🗿️artifacts/📖️playbook/🧫️fixtures/👑️playbook-scene-owner-law.json` (scene-owner law gone).
- `cargo check -p semio-s-artifact-playbook-playbook --lib` (12:16–12:22): **5 errors, all peer value/store API fallout in my files**
  (`protocol::{ValueError,ToValue,FromValue,DslValue}` no longer re-exported; one-item `close_step` now `Result<_, ValueError>`) —
  fixed in source; 0 errors from the conversion itself. Re-check OWED (rule 44 CARGO FREEZE from 12:2x).
- Open (playbook): mounted `chapters:in` import needs a reserved import job (`build_reserved_tool_job`, sequence's
  `SequenceImportJob` pattern) — the synchronous `import_media` emits correctly but a registered app routes imports through the
  reserved job (`interactive-job.missing-reserved-builder`); pre-existing gap, not introduced here.

### S4.4 Source-only wave under rule 44 (12:40–13:30)

- **Framework fault codes (S4-GATES routing)**: anonymous / plugin-local `tool-mismatch` faults → `app.command.tool-mismatch` in
  gismap editor + viewer, gisterrain editor, energy editor + viewer, forms editor, gen2d editor, gen3d editor + viewer, playbook
  editor; playbook unknown action → `app.command.unsupported`.
- **gen2d/gen3d `⏯️tool-run`** (4 anonymous faults): `generation{2d,3d}.preview.session-closing` + en/de notice
  (`preview_eval_fault_notices`). Malformed two-segment `generation{2d,3d}.child-projection` → `generation{2d,3d}.child.projection`;
  new root regions `🔖️FaultNotices` (`generation3d_document_fault_notices`: child.projection, io.export, io.import-accept,
  widget.add; `generation2d_document_fault_notices`: child.projection). Editors declare gumball + preview + document notices
  (gen3d) / preview + document (gen2d); both viewers and the playbook viewer declare theirs.
- **close_step ABI (S4-INFRA routing)**: `T/🧪️s4-infra-close-step-abi.py --apply` on gismap editor, gismap `💾️binary`, gisterrain
  editor, energy editor (4 migrated); playbook editor done by hand (S4.3).
- **Energy short kinds (coordinator decision, design §14)**: input script `T/🧪️s4-tools-b-energy-short-kinds.py` (dry run default,
  `--write`, idempotent: 2nd run 0 changes) renamed 16 dirs and rewrote 88 files (+2 S3-CONTROLS generator inputs). Scheme
  `change-<record>-<field>`: `change-ground-{building,shallow,deep}` (entity `ground`), `change-run-{start-month,start-day,end-month,
  end-day,year}` (entity `run`); `change-site-*` unchanged. All 13 new leaves now have dir == semanticKind, max path 236 B ≤ 240.
  Rule replica: identity/owner mismatches 153 → 147 (146 pre-existing + `📝️text` codec dir without descriptor).
  Coordinator action: central `schema generate` must catalogue `s.energy.model.mutation.change-{ground-building,ground-shallow,
  ground-deep,run-start-month,run-start-day,run-end-month,run-end-day,run-year}` (replacing the session-3 list's ground/run names).
- **REPO-PATH-BUDGET hand-off (out of scope, separate task)** — the 146 pre-existing energy dir≠kind leaves:
  `↗️change-air-loop-supply`→`change-air-loop-supply-node`, `↘️change-air-loop-return`→`change-air-loop-return-node`, `⌛️change-equipment-gain`→`change-equipment-gain-schedule`, `⏬️change-battery-max`→`change-battery-max-discharge`, `⏱️change-lighting-gain`→`change-lighting-gain-schedule`, `⏳️change-infiltration`→`change-infiltration-schedule`, `☀️change-material-solar`→`change-material-solar-absorptance`, `☄️add-electrical-load-center`→`add-electrical-load-center-pv`, `☔️change-humidistat-humidifying-setpoint`→`change-humidistat-humidifying-setpoint-schedule`, `☕️change-plant-loop-supply`→`change-plant-loop-supply-temperature`, `♌️change-pv-system-inverter`→`change-pv-system-inverter-efficiency`, `♨️change-material-specific`→`change-material-specific-heat`, `⚛️change-pv-system-dc`→`change-pv-system-dc-capacity`, `⚡️change-equipment-gain-watts`→`change-equipment-gain-watts-per-area`, `⛓️change-outdoor-air-system`→`change-outdoor-air-system-air-loop`, `⛱️change-shading-surface`→`change-shading-surface-transmittance-schedule`, `⛵️change-solar-thermal-system`→`change-solar-thermal-system-azimuth`, `⛽️change-ideal-loads-system`→`change-ideal-loads-system-max-heating-capacity`, `✂️disconnect-referenced`→`disconnect-referenced-model`, `⬇️change-fenestration-sill`→`change-fenestration-sill-height`, `🅰️change-infiltration`→`change-infiltration-constant-term-coefficient`, `🅱️change-infiltration-temperature-term`→`change-infiltration-temperature-term-coefficient`, `🆎️change-infiltration`→`change-infiltration-velocity-term-coefficient`, `🆑️change-infiltration-velocity-squared-term`→`change-infiltration-velocity-squared-term-coefficient`, `🌄️create-solar-thermal`→`create-solar-thermal-system`, `🌆️delete-solar-thermal`→`delete-solar-thermal-system`, `🌐️change-fenestration-u`→`change-fenestration-u-value`, `🌘️remove-electrical-load`→`remove-electrical-load-center-pv`, `🌞️change-people-gain`→`change-people-gain-sensible-fraction`, `🌟️change-lighting-gain`→`change-lighting-gain-radiant-fraction`, `🌠️change-equipment-gain`→`change-equipment-gain-radiant-fraction`, `🌥️change-sizing-object`→`change-sizing-object-design-day-type`, `🌧️change-humidistat`→`change-humidistat-humidifying-throttle-range`, `🌪️create-mechanical`→`create-mechanical-ventilation`, `🌫️change-infiltration-flow`→`change-infiltration-flow-per-exterior-area`, `🌲️create-outdoor-air`→`create-outdoor-air-system`, `🍃️change-surface-wind`→`change-surface-wind-exposed`, `🍥️change-air-loop-design`→`change-air-loop-design-supply-air-flow`, `🍧️change-zone-equipment`→`change-zone-equipment-cooling-capacity`, `🍵️change-shw-system-heater`→`change-shw-system-heater-capacity`, `🎄️change-annual-schedule`→`change-annual-schedule-holiday-daily-schedule`, `🎈️change-mechanical-ventilation-fan-delta`→`change-mechanical-ventilation-fan-delta-pressure`, `🎉️add-annual-schedule`→`add-annual-schedule-holiday`, `🎊️remove-annual-schedule`→`remove-annual-schedule-holiday`, `🎌️change-annual-schedule`→`change-annual-schedule-default-daily-schedule`, `🎐️change-lighting-gain`→`change-lighting-gain-return-air-fraction`, `🎖️change-pv-system-module`→`change-pv-system-module-efficiency`, `🎗️change-zone-equipment`→`change-zone-equipment-priority`, `🎚️change-thermostat-heating`→`change-thermostat-heating-throttle-range`, `🎛️change-thermostat-cooling`→`change-thermostat-cooling-throttle-range`, `🎞️delete-time-series`→`delete-time-series-schedule`, `🎣️change-fault-target`→`change-fault-target-equipment`, `🎩️change-fenestration`→`change-fenestration-overhang-offset`, `🎼️change-setpoint-manager`→`change-setpoint-manager-schedule`, `🏃️change-people-gain`→`change-people-gain-activity-schedule`, `🏅️change-solar-thermal`→`change-solar-thermal-system-efficiency`, `🏋️change-refrigeration`→`change-refrigeration-system-design-load`, `🏝️change-humidistat`→`change-humidistat-dehumidifying-setpoint-schedule`, `🏢️change-ideal-loads-system`→`change-ideal-loads-system-zone`, `🏦️create-electrical-load`→`create-electrical-load-center`, `🏭️change-infiltration-stack`→`change-infiltration-stack-height`, `🐋️change-fenestration-fin`→`change-fenestration-fin-offset`, `🐧️change-thermostat-cooling`→`change-thermostat-cooling-setpoint-schedule`, `🐬️change-fenestration-fin`→`change-fenestration-fin-depth`, `👁️change-material-visible`→`change-material-visible-absorptance`, `👥️change-people-gain-people`→`change-people-gain-people-per-area`, `💠️change-mechanical-ventilation-fan-total`→`change-mechanical-ventilation-fan-total-efficiency`, `💦️change-equipment-gain`→`change-equipment-gain-latent-fraction`, `💧️change-people-gain-latent`→`change-people-gain-latent-fraction`, `💰️change-outdoor-air-system`→`change-outdoor-air-system-economizer-enabled`, `📆️change-mechanical`→`change-mechanical-ventilation-schedule`, `📐️change-zone-floor-area`→`change-zone-floor-area-participation`, `📗️insert-annual-schedule`→`insert-annual-schedule-rule`, `📙️remove-annual-schedule`→`remove-annual-schedule-rule`, `📡️change-people-gain-radiant`→`change-people-gain-radiant-fraction`, `🔀️reorder-construction`→`reorder-construction-layers`, `🔃️replace-setpoint-manager`→`replace-setpoint-manager-kind`, `🔄️change-infiltration-design`→`change-infiltration-design-flow-ach`, `🔅️change-lighting-gain`→`change-lighting-gain-visible-fraction`, `🔆️change-material-thermal`→`change-material-thermal-absorptance`, `🔋️add-electrical-load-center`→`add-electrical-load-center-battery`, `🔌️change-lighting-gain-watts`→`change-lighting-gain-watts-per-area`, `🔓️remove-thermal-enclosure`→`remove-thermal-enclosure-zone`, `🔥️change-material`→`change-material-conductivity`, `🔳️change-ideal-loads-system-outdoor-air-per`→`change-ideal-loads-system-outdoor-air-per-area`, `🔴️change-ideal-loads-system`→`change-ideal-loads-system-max-heating-supply-air-temp`, `🔵️change-ideal-loads-system`→`change-ideal-loads-system-min-cooling-supply-air-temp`, `🔶️replace-fenestration`→`replace-fenestration-vertices`, `🔷️change-glazing-material`→`change-glazing-material-thickness`, `🔻️delete-electrical-load`→`delete-electrical-load-center`, `🔼️change-solar-thermal`→`change-solar-thermal-system-tilt`, `🕑️change-refrigeration`→`change-refrigeration-system-defrost-schedule`, `🕒️change-water-system`→`change-water-system-schedule`, `🕓️change-fault-start`→`change-fault-start-schedule`, `🕔️replace-daily-schedule`→`replace-daily-schedule-hourly-values`, `🕕️change-daily-schedule`→`change-daily-schedule-interpolation`, `🕘️replace-time-series`→`replace-time-series-schedule-values`, `🕙️change-time-series`→`change-time-series-schedule-timestep`, `🕝️change-constant-schedule`→`change-constant-schedule-value`, `🕟️change-daily-schedule`→`change-daily-schedule-limits`, `🕳️change-infiltration`→`change-infiltration-effective-leakage-area`, `🕶️change-daylight-zone`→`change-daylight-zone-glare-limit`, `🖊️rename-electrical-load`→`rename-electrical-load-center`, `🖼️change-fenestration-frame`→`change-fenestration-frame-conductance`, `🗂️reorder-annual-schedule`→`reorder-annual-schedule-rules`, `🗄️change-refrigeration`→`change-refrigeration-system-case-count`, `🗺️replace-shading-surface`→`replace-shading-surface-vertices`, `🚤️change-plant-loop-design`→`change-plant-loop-design-flow`, `🚧️change-surface-boundary`→`change-surface-boundary-condition`, `🚫️delete-mechanical`→`delete-mechanical-ventilation`, `🚰️change-infiltration`→`change-infiltration-discharge-coefficient`, `🚾️change-water-system-peak`→`change-water-system-peak-flow`, `🚿️change-mechanical`→`change-mechanical-ventilation-design-flow`, `🛏️create-room-air-model`→`create-room-air-model-assignment`, `🛢️change-shw-system-storage`→`change-shw-system-storage-volume`, `🟠️change-glazing-material`→`change-glazing-material-conductivity`, `🟡️change-glazing-material`→`change-glazing-material-solar-transmittance`, `🟣️change-gas-material`→`change-gas-material-thickness`, `🟧️change-ideal-loads-system`→`change-ideal-loads-system-max-cooling-capacity`, `🟩️change-solar-thermal`→`change-solar-thermal-system-collector-area`, `🥃️change-daylight-zone`→`change-daylight-zone-window-transmittance`, `🥉️change-battery-round-trip`→`change-battery-round-trip-efficiency`, `🥵️change-thermostat-heating`→`change-thermostat-heating-setpoint-schedule`, `🥽️change-glazing-material`→`change-glazing-material-visible-transmittance`, `🦋️change-outdoor-air-system`→`change-outdoor-air-system-min-oa-flow`, `🧃️change-solar-thermal`→`change-solar-thermal-system-storage-volume`, `🧇️change-zone-equipment`→`change-zone-equipment-heating-capacity`, `🧊️bind-fenestration-glazing`→`bind-fenestration-glazing-construction`, `🧍️change-ideal-loads-system-outdoor-air-per`→`change-ideal-loads-system-outdoor-air-per-person`, `🧢️change-fenestration`→`change-fenestration-overhang-depth`, `🧭️change-mechanical`→`change-mechanical-ventilation-zone`, `🧰️change-surface`→`change-surface-construction`, `🧲️change-fenestration`→`change-fenestration-surface`, `🧷️change-fenestration`→`change-fenestration-divider-conductance`, `🧺️delete-room-air-model`→`delete-room-air-model-assignment`, `🧻️change-humidistat`→`change-humidistat-dehumidifying-throttle-range`, `🧾️change-sizing-object`→`change-sizing-object-sizing-type`, `🩻️change-glazing-material`→`change-glazing-material-infrared-emissivity`, `🪔️change-daylight-zone`→`change-daylight-zone-illuminance-target`, `🪗️create-time-series`→`create-time-series-schedule`, `🪝️remove-electrical-load`→`remove-electrical-load-center-battery`, `🪣️change-water-system`→`change-water-system-fixture-count`, `🪹️remove-air-loop-terminal`→`remove-air-loop-terminal-zone`, `🫖️change-plant-loop-return`→`change-plant-loop-return-temperature`, `🫗️clear-fenestration-glazing`→`clear-fenestration-glazing-construction`, `🫠️delete-refrigeration`→`delete-refrigeration-system`

### S4.5 Runs allowed under rules 43/44, analyses, owed list (12:50)

Clock note: S4.3/S4.4 headings carry estimated times; the real clock at this section is 12:50.

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-playbook-playbook --lib` (12:16–12:22, before rule 44) | 5 errors, all peer value/store-API fallout in my files, fixed in source (S4.3); 0 from the conversion |
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | **34 pass / 0 fail** (25 092 expects) |
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🔋️energy` (after the short-kind rename) | **0 findings**, 600/600 payloads, 301/301 leaves witnessed, 138 negatives rejected |
| `… schema mutation-payloads --under ✏️s/🔌️plugins/📖️playbook` | **0 findings**, 5/5, 4/4 witnessed |
| `… schema mutation-inputs --under ✏️s/🔌️plugins/🔋️energy` | 16 findings = 13 `leafUncatalogued` (new names) + 3 deleted `update-*` still catalogued → central `schema generate` |
| `… schema mutation-inputs --under ✏️s/🔌️plugins/📖️playbook` | 8 findings = the 8 deleted parent leaves still catalogued → central `schema generate` |
| `… schema mutation-inputs --under ✏️s/🔌️plugins/🌀️procedural` | 1 finding = `change-widget-input` `leafUncatalogued` → central `schema generate` |
| `python3 T/🧪️s4-tools-b-playbook-child-leaves.py --check` | exit 0 (9 vectors) |
| `python3 T/🧪️s4-tools-b-energy-short-kinds.py` (second run) | 0 renames, 0 rewrites (idempotent) |

**P5 (live-only, React gumball re-anchor) — code read, live check still owed:** `SceneGumball`
(`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` ≈3178) ignores every `target` change while `draggingRef` is set, so a
streamed answer cannot re-anchor the widget mid-drag; on release it pins the pivot to the gesture's `after` pose. For whole gen3d
shapes the guest sends no `gumballTarget` (component mode sends `component_pivot`), so the host's target is the leftover overlay's
selected-instance `position` (`leftoverWorldGumballPoseV1`). Live risk to verify on a serve: once the overlay stops applying the
target is `undefined` and the gumball unmounts (`!target → null`), and after a release the next instance `position` re-anchors
the pivot. Needs a procedural activation + serve (coordinator).

**D11 CLOSURE-5 (ready, waiting for CARGO OPEN + rule 39):** `settle_press` (`OSM/🔌️plugin/🛠️tool-machine/🦀️.rs` ≈505) passes
`semio_framework_tool_machine::authoring_clock(0)` — every press opened in one millisecond mints the same `TransactionRef`.
Fix: `let clock = self.tool_machines.clock();` (the per-instance monotonic tick the typing ledger already uses; computed before
the `presses.send(..)` borrow) + the `clock()` docstring naming presses; law in `🔌️plugin/🧪️tests/🧪️scrub`: two presses released
at the same `now_ms` carry distinct refs. One-line shared-crate edit, applied together with its gated `cargo check -p
semio-framework-plugin --lib` once cargo opens.

**D12 two-phase release (design, not started):** `ScrubLedger` keeps the pre-release open state of a released press in a
`releasing` map until its publication settles; `settle(window, gesture)` forgets it, `reopen(window, gesture)` restores it (overlay
back, press open, the next release/abort decides). The plugin runtime already binds each admitted operation to its `ToolTag`
(`ToolMachineRuntime::bind`); the completion path calls `settle` on a landed edit and `reopen` on a downstream refusal instead
of surfacing a Fault with both lanes gone. Rust + TS twin + conformance cases + plugin glue law. Shared crates (tool-machine,
plugin) → only with cargo open.

**OWED (rule 43/44), exact commands** (gate v3 before each; `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-tools-b` for tests):
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-playbook-playbook -p semio-s-artifact-energy-model -p semio-s-artifact-forms-forms -p semio-s-artifact-gis-gismap -p semio-s-artifact-gis-gisterrain -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-procedural-generation3d --lib --message-format=short` (+ `--features <gen2d/gen3d>/component-app-assembly`), then the same with `--target wasm32-wasip2`, then `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-{playbook,energy,forms,gis,procedural} --target wasm32-wasip2 --lib` → "COMPOSITION GREEN <plugin>".
2. `cargo test -p semio-framework-tool-machine --lib`; `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- tool_machine:: tool_run_tests::`; `cargo test -p semio-framework-os-flow --lib`; `cargo test -p semio-framework-os-kernel-neural-engine --lib`.
3. Playbook: `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-playbook-playbook --lib` (child-leaf vectors law, composed_reload_law, history_edit_acceptance_law, editor/command/window/viewer/inference/codec tests).
4. Energy: `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test … -p semio-s-artifact-energy-model --lib -- writes_the_committed_vector_when_requested` then the lib (L4 law after S4-RUNTIME W2A-2).
5. Forms `-- field_transactions change_block_field block_field try_value`; gis gismap/gisterrain lib.
6. gen2d/gen3d `--lib --tests --keep-going` checks + `cargo test --lib` each + DEV `--test generation3d-app-laws` / `--test generation2d-example-export`; D24: run `debug_print_normalized_examples` / `debug_print_normalized_example` with `--nocapture`, `T/🧪️s3-procedural-normalize-examples.py` (dry run, then `--write`), delete both `[DEBUG]` printers, re-run the two example laws.
7. TS: React lanes / graph sliders vitest (S3-CONTROLS list) when the renderer suites are cleared.

**Coordinator actions (cumulative):** central `schema generate` (energy: catalogue `change-site-{latitude,longitude,elevation,
time-zone,north-axis}`, `change-ground-{building,shallow,deep}`, `change-run-{start-month,start-day,end-month,end-day,year}`, drop
`update-site`, `update-ground-temperature`, `update-run-period`; playbook: drop the 8 parent leaves `add-step`, `remove-step`,
`move-step`, `add-block`, `remove-block`, `move-block`, `replace-block`, `update-step`; procedural: catalogue
`change-widget-input`); `describe` playbook (child-lane verbs, notices, no `document` slot), energy (renamed leaves, framework
tool-mismatch code), forms, gis, procedural (notices, named codes); activation + serve for the P5 live check; REPO-PATH-BUDGET task
for the 146 energy leaves (S4.4).

### S4.6 PARKED at 13:2x (coordinator usage limit) — audit `📓️audit-s4-tools.md` items, exact state

No file was mid-edit when parked: every edit listed in S4.3–S4.5 is complete; nothing of S4.6 is written yet (analysis only).

**Verification state per area (report hygiene, audit item 6):**

| Area | State |
|---|---|
| Playbook §20.15 | SOURCE-COMPLETE (S4.3); native lib check showed only 5 peer-API errors, fixed; re-check + wasip2 + lib tests OWED (rule 44) |
| Energy | 13 field leaves + §14 short kinds (S4.4) source-complete; payload gate 0 findings; input gate = 16 catalog rows (central `schema generate`); lib tests + fixture writer OWED; L4 law waits for S4-RUNTIME W2A-2 |
| Forms | `app.command.tool-mismatch` adopted (source); `field_transactions change_block_field block_field try_value` tests OWED; window transient → `transient_root!` is S4-RUNTIME's (§21.5) |
| GIS (gismap, gisterrain) | close_step ABI + framework tool-mismatch code (source); gismap/gen2d/gen3d store initializers were migrated by S4-STORE; checks/tests OWED |
| gen2d/gen3d | preview-eval + document notices/named codes (source); D24 normalization OWED (needs cargo for the `[DEBUG]` printers) |
| D11 | fix identified (S4.5), to be staged/landed with its gated plugin check at CARGO OPEN |
| D12 | design written (S4.5), not implemented |
| P5 | code-read verdict (S4.5); live check needs procedural activation + serve |
| Tool-machine TS | 34/0 (13:46 run, S4.5) |

**Audit items — next steps (none started):**
1. **F2 (major) playbook blocks.** Analysis: a child-lane leaf must belong to the child's own vocabulary (the member store decodes
   ops with the child's mutation type), so playbook-typed block leaves cannot ride the stdio `flow` child without adding playbook
   (or BPMN-like "form field") vocabulary to `stdio-semio` (shared crate, frozen, domain layering). Options weighed:
   (a) intent leaves in stdio flow — typed UI + same-snapshot validation, but stdio→playbook coupling and a frozen shared crate;
   (b) a typed blocks child — **recommended: compose the existing `s.forms.form` artifact (forms plugin, owned by this WP) as the
   playbook's content child instead of `flow`**: it already models steps + typed blocks with the full typed vocabulary
   (`create-step`, `create-block`, `delete-block`, `move-block-to-step`, `reorder-step`, `rename-step`, `change-block-field` with
   §17.1 typed value union + `x-semio-ui` en/de, fixtures, inverses, TS twins), so masking, typed controls and target-missing
   validation all hold and `blocksJson` disappears. Cost: a playbook `space_members!` roster (`Form("s.forms.forms", …)` beside
   the stdio flow variant) replacing `SemioMembers` in `PlaybookApplication`, the playbook hub composition, demo/genesis assets
   re-authored as a forms document, ChildLane builders → forms leaves, laws/vectors re-pointed. Note: forms' own snapshot composes
   `structure`/`results` stdio children — check that a nested composition is admitted before committing to (b);
   (c) blocks as flow nodes with one param per field — no shared edits, fixes masking, but `set-node-param.value` stays an untyped
   string (fails the typed-`x-semio-ui` requirement). Decision pending coordinator confirmation of (b) vs (a).
2. **F6** `composed_child_history_law!` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` beside `composed_reload_law!`, seed `addStep` then
   `addBlock` (wait for F2's final verbs).
3. **F5** gen3d gumball onto the shared `GestureTool`/`drive_gesture` (`🛠️tool-machine`, S4-TOOLS-A): implement `GestureTool` for
   the gumball in `🧊️generation3d/…/✏️editor/🎮️commands/🧭️transforms/🦀️.rs` (≈249–266), delete `GumballPhase` + the local drive body;
   read S4-TOOLS-A's runner first; plugin-local only (no shared edit).
4. **F9** `🧩️extensions/🌀️procedural/🦀️.rs:984` `playbook.module.procedural.tool-mismatch` → `app.command.tool-mismatch`.
5. **F14** unique first docstring emojis in the playbook root `🦀️.rs` (🔗 at the chain helpers / ChildLane emit, 🧱 at
   `playbook_step_node` / `playbook_blocks_leaf` / `playbook_edit_blocks_leaves`).

Coordinator actions unchanged from S4.5.

### S4.7 Resume 19:53 — F2 admission check (design §21.7): NOT ADMITTED → F2 stopped; F6, F9, F14 done

**Verdict (read from code, no playbook-specific path written):** a forms child inside playbook is NOT admitted, because nested
composition with DERIVED grandchildren has no live runtime. The archive FORMAT and the closure VALIDATOR are recursive, but every
live maintenance path stops at the root:

| Piece | Where | Nested? |
|---|---|---|
| Archive member entries carry `owner {parent ref, slot, child_id}` | `📡️spr/🧵️channel/🦀️.rs` ≈147–171 (`OwnedDocumentMemberPackEntry`, `DocumentArchivePack`) | yes |
| Closure walk BFS over every member's own projection | `🏪️store/🧩️composition/🌳️closure/🦀️.rs` ≈196–245 (`walk`, `child_projection(Some(index))`) + `DocumentClosureSourceView` (`PLG` ≈10368–10412) | yes — a declared grandchild missing from the member registry is `Incomplete` |
| Boot genesis | `PLG` ≈25420 `seed_genesis_children` — `ChildRestoreProjection::from_snapshot(self.store.snapshot_ref())` + `A::genesis_child_pack` | **root only** |
| Live follow of re-minted derived children | `PLG` ≈25454 `follow_derivable_children` — root projection + `A::genesis_child_pack` | **root only** |
| Archive-load genesis (`Effect::LoadDocument` ships pack+spr only) | `PLG` ≈26240 `complete_document_archive_genesis` | **root only** |
| Live replacement genesis | `PLG` ≈26288 `complete_store_replacement_genesis` | **root only** |
| Opening a member | `PLG` ≈27276 `open_child` → `declared_child_reference` / `validate_parent_child_restore` against the ROOT parent; owner = root ref | **root only** |
| Member registry / content view / child emits | `ChildMemberRegistry`, `ChildContentView`, `ChildEmit { slot, child_id }` keyed `(slot, child_id)` | no owner path |

Consequence for playbook → forms: forms declares two DERIVED children (`structure` = content-addressed `forms-value-<hash(steps)>`,
`results`), and every forms definition leaf re-mints `structure` (`📋️forms/…/🔺️diff/🦀️.rs:174` `forms_diff_from_delta` →
`forms_children_from_steps`). Inside playbook nothing would ever mint those grandchildren (boot, LoadDocument, replacement), nothing
follows the re-mint after a child-lane forms edit, and the next save / `artifact:out` / folder persistence would fail closure
validation as `Incomplete`. A forms member also has no derivation authority: `MemberFactory` (`🏪️store/🦀️.rs` ≈25748) only
`create`s and `open`s; per-dialect `genesis_child_pack` lives on the composing `ArtifactApp`, never on a member.

**Generic framework gap (owner suggestion: S4-RUNTIME + S4-STORE; coordinator decides):**
1. A per-dialect derivation authority on the member roster (e.g. `MemberFactory::genesis_child_pack(member_snapshot, slot,
   child_id)` generated by `space_members!` from the member artifact's own app, or an app-roster lookup by dialect).
2. Recursion over member projections, with the member as `owner.parent`, in `seed_genesis_children`, `follow_derivable_children`,
   `complete_document_archive_genesis`, `complete_store_replacement_genesis` and `open_child`/`declared_child_reference`.
3. An owner PATH (not `(slot, child_id)`) as the key of `ChildMemberRegistry`, `ChildContentView` and the per-member lanes, so a
   grandchild is addressable without colliding with a root child of the same slot/id.
4. Child-lane edits of a member that re-mint its derived children: the member's follow pass must run after every child-lane
   publication (today only the root parent generation triggers `follow_derivable_children`).
5. Laws: a two-level composed fixture (root → member with a derived child) through boot, child-lane edit + re-mint, save → fresh
   load (`composed_reload_law!`), history edit of the member (`composed_child_history_law!`), time travel over the reloaded document.

Until that lands, F2 stays OPEN on the current flow-child model (whole-list `blocksJson` set per block edit). Interim option the
coordinator may prefer instead (no framework change): blocks as `flow` nodes with one param per field (masking fixed, untyped
editor controls) — not done, awaiting decision.

**Done this resume (source only, cargo OWED — kernel red, S4-PACKFIX):**
- **F6**: `composed_child_history_law!("playbook", PlaybookPlayApp, playbook_manifest_for_tests, [("addStep", "{}"),
  ("addBlock", r#"{"kind":"number"}"#)])` beside `composed_reload_law!` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- **F9**: `🧩️extensions/🌀️procedural/🦀️.rs:984` → `app.command.tool-mismatch` (repo grep: 0 remaining refs to the old code).
- **F14**: playbook root `🦀️.rs` docstring first emojis unique (10 lines re-emojied; per-file duplicate census 0).

### S4.8 F5 gen3d gumball on the shared runner — source complete; PARKED (20:2x)

Coordinator 20:0x: decision recorded as design §21.9 (recursive composition, S4-STORE + S4-RUNTIME implement the S4.7 list after
KERNEL GREEN); no interim flow-node option; F2 stays OPEN and lands on top of §21.9 (resume after the two-level laws are green).

**F5 (audit, gen3d) — `🧊️generation3d/…/✏️editor/🎮️commands/🧭️transforms/🦀️.rs`:**
- `GumballPhase` deleted; the shared `semio_framework_tool_machine::GesturePhase` is used (re-exported from `transform_commands`
  for the DEV crate, which has no tool-machine dependency).
- `Generation3dGumballTool: GestureTool` (`start`/`resume`/`verb`/`base_revision`/`abort`/`send`/`persist`) over the existing
  `gumball_tool` statechart; the window's open gesture is the PartialEq record `GumballGesture { states, verb, authoring_seed,
  base_revision (hex), transaction, entries, ids }` (resume rebuilds the stream context from the net leaf,
  `GumballRecord::of_leaf`); tick = `GumballTick { ids, record }`.
- `GumballGestures::dispatch` now drives ONE dispatch through `drive_gesture::<Generation3dGumballTool>` — the local
  open/abort/base-moved/capture-lost drive body is deleted. Kept app-side: the splice rows re-derived on the committed base at the
  release (prepended to the committed leaves, one `commit_transaction`), the interaction writes, the preview revision, and the
  selection a continuing gesture keeps (`GumballGesture::continues` reads the runner's continuation rule before the drive). A
  selection the base no longer splices drops the gesture and refuses with the named gumball code (as before).
- Editor glue (`✏️editor/🦀️.rs` ≈855): `GesturePhase::parse`; the anonymous `generation3d-gumball-phase-unknown` fault →
  framework `app.command.invalid-args`.
- Tests: the phase-parse law moved to the shared type (deleted here); new law `the_gumball_rides_the_shared_gesture_runner`
  (stream persists ONE net leaf + opening selection, resumed gesture keeps its selection, release commits the net motion under the
  first-tick ref, base move drops with zero trace, `continues` = runner rule). DEV law `↔️translate-selection` updated to
  `GesturePhase`. Docstring first emojis unique in the transforms file (7 re-emojied); the test file's pre-existing `⚖️ LAW`
  repetition (8) is left to S4-GATES' §21.2 policy.

**OWED (KERNEL GREEN / CARGO OPEN), exact commands (gate v3 first):**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-playbook-playbook -p semio-s-plugin-playbook-procedural --features semio-s-artifact-procedural-generation3d/component-app-assembly --lib --tests --message-format=short` (F5 + F6 + F9 + F14 + the S4.3 playbook conversion).
2. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-composition-laws --tests --message-format=short` (DEV `↔️translate-selection` test).
3. then the S4.5 list (native + wasip2 checks → COMPOSITION GREEN, lib tests incl. `child_history_edits_end_to_end`,
   `documents_reload_identically`, `the_gumball_rides_the_shared_gesture_runner`, DEV `--test generation3d-app-laws`).

**Parked state:** no file is mid-edit. F2 waits for §21.9; everything else of the S4 audit list for this WP is source-complete.

### S4.9 CARGO OPEN (plugin-local checks) 20:52 — check 1 peer-blocked; pack-error API migration of my trees

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-playbook-playbook -p semio-s-plugin-playbook-procedural --features semio-s-artifact-procedural-generation3d/component-app-assembly --lib --tests --keep-going` (20:55–20:58) | **exit 101, 74 errors, 0 in my crates** — all in 10 stdio crates (deflate, las, stl, ply, xml, json, dxf, obj, step, dwg) on the new `semio-framework-pack-error` API: E0308 `expected &PackError, found &PackRefusal`, E0599 `no variant Schema/Malformed/ValueRefusal on PackError`. My crates never compiled (every one of my 8 crates depends on ≥ 2 of those stdio crates per `cargo tree`). Reported to `main` (owner guess S4-STDIO / S4-PACKFIX) |

- **Pack-error migration of my own trees (source)**: input script `T/🧪️s4-tools-b-pack-refusal-api.py` (dry run default, `--write`,
  idempotent: second run 0 files) moved 32 files in forms, gis, energy, playbook, procedural from the removed
  `store::PackError::{Schema, ValueRefusal}` to `store::PackError::Refusal(store::PackRefusal::Malformed { kind: InvalidValue,
  what: "pack", offset: 0, detail })` / `store::PackError::from` (kernel re-exports `PackError, PackRefusal`, `From<ValueError>`).
- Checks 2 and 3 not started (same stdio closure). OWED as listed in S4.8, re-run on "stdio green".
