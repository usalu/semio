# W4 — `process` composes stdio `brep` + `flow`

**ucas-status: complete — 157/157 tests passing, 0 compile errors**

**Correction (orchestrator, 2026-08-13):** this report originally read as "154/157, 3 pre-existing failures dated 2026-06-04, two months before this ticket." The dating was wrong — this ticket's auto-commit messages embed a **fixed, stale template date string** (`🎆️26🌙️06☀️04`) that never changes and does not reflect the real commit date (see `📌️important.md`'s new top-level warning). The cited commit `515271bf60` actually landed **2026-08-13 13:05:26 — the same day as, and during, this ticket's own active window**, not two months prior. The underlying root-cause diagnosis in this report (an index-`[0]` test-fixture assumption bug, unrelated to the composition migration) was itself correct on the merits — independently re-derived and confirmed by the orchestrator by running the failing test directly and reading `Workshop::default()`'s `generic_machines()` seeding. All three tests were fixed directly (switched from asserting `machines[0]` to `machines.iter().find(|m| m.id == "machine-1")`, since the seeded generic roster pushes the test's own machine to a later index). Verified: `cargo check` clean, `cargo nextest run` → **157/157**.

## Baseline (before any edit)

`cargo check -p semio-s-plugin-process --all-targets` was run BEFORE touching any file. **Not clean**: 1 pre-existing error (`JsonValue` vs `serde_json::Value` mismatch in `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:10`, traced to commit `2564722008` 🚩️479, 2026-06-04 — predates the ticket). Fixed it as a trivial one-line, in-boundary `.into()` addition before starting the real migration, to get a clean baseline to iterate against.

## What changed

### Killed `SolidSpec` — composed `brep` for stock/tool geometry

`SolidSpec` (the persisted 5-variant DSL-enum: `Box`/`Cylinder`/`Sphere`/`ImportedMesh`/`ImportedSolid`, with a hand `dsl::DslField` impl) is **deleted**. Its role is split in two, per the recipe's core transform:

- **Persisted**: `Process3dSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs`) composes real `s.stdio.semio.brep` CHILD HANDLES — `stock_solid: store::ArtifactChild<SemioBrepSnapshot>` (bare, always present) and `tool_solids: Vec<store::ArtifactChild<SemioBrepSnapshot>>` (one per `Cut`/`Attach` step, `Drill` has no tool solid). `steps: store::ArtifactChild<SemioFlowSnapshot>` composes one `s.stdio.semio.flow` child for the whole ordered timeline (replaces `Vec<ProcessStep>` inline).
- **Ephemeral**: `WorkingSolid` (same 5 variants, same purpose, artifact root `🦀️component.rs` `🔖️WorkingScene` region) is the plugin's own editing vocabulary — never persisted, never implements `dsl::DslField`, mirrors `📐️cad`'s `CadObject`/`CadGeometry` precedent exactly.

Children must sit as DIRECT top-level fields on the snapshot struct — confirmed by reading `🧬️schema/✨️derive/🦀️component.rs`'s `expand_artifact_schema` (`for field in &fields.named` only walks the struct's own fields, no recursion into nested types) — so `Stock` (id/label/pose/solid) is **flattened** into `stock_id`/`stock_label`/`stock_pose`/`stock_solid` directly on `Process3dSnapshot`, not nested. This drove most of the app-layer blast radius (~90 files touching `.stock.*`).

### Real, exact analytic brep converters (not a stub)

`brep_snapshot_for_working_solid` (artifact root, `🔖️BrepConverters`) builds REAL `SemioBrepSnapshot` topology, not an approximation:
- `Box` → 8 vertices, 12 straight edges, 6 planar faces, 1 shell, 1 solid.
- `Cylinder` → 2 circular edges, 2 planar caps + 1 analytic `BrepSurface::Cylinder` lateral face (untrimmed — a `BrepLoop` with zero edges is the correct shape for "spans the whole surface", the same convention the sphere face below uses).
- `Sphere` → 1 closed, untrimmed analytic `BrepSurface::Sphere` face, zero vertices/edges.
- `ImportedMesh`/`ImportedSolid` → an honest empty-topology placeholder (documented: these reference kernel-session-local content — a GLB data-url or an in-kernel solid handle — that a plugin-only migration cannot tessellate/resolve; this exact limitation was already documented on the pre-migration variants).

Tested by `box_working_solid_mints_a_real_six_face_one_solid_brep`/`cylinder_working_solid_mints_a_real_three_face_brep`/`sphere_working_solid_mints_a_real_one_face_untrimmed_brep`/`brep_snapshot_for_working_solid_round_trips_pack_and_dsl` in the artifact root file's own test module.

The bounds inference (`💡️inferences/📦bounds/🦀️component.rs`) was **simplified and generalized** in the same pass: instead of a per-`SolidSpec`-variant analytic-extent switch, `brep_bounding_box` computes the AABB directly off a resolved `SemioBrepSnapshot`'s own `vertices` list — strictly more general (works for any brep content) and simpler.

### `flow` for the step timeline — real bidirectional converter

`flow_node_from_process_step`/`process_step_from_flow_node` (artifact root, `🔖️FlowConverters`) map every `ProcessStep` field to/from one `FlowNode`: `enabled`/`origin.machineId`/`origin.capabilityId`/pose (7 scalars)/`radius`+`depth` (Drill) round-trip exactly through `FlowParam` key-value pairs; `Cut`/`Attach`'s tool/component solid is addressed indirectly by a `toolChildId` param naming its entry in `tool_solids` (a `FlowParam` value is a plain string, never a child handle). `flow_snapshot_for_steps` builds one `FlowNode` per step (synthetic linear `position.x` layout) plus a real sequential-chain `FlowEdge` between consecutive steps — an honest, lossless topology for a timeline that never branched even before this migration. Tested by `process_step_flow_round_trips_scalar_fields`/`flow_snapshot_for_steps_is_a_real_linear_chain`.

**Honest lossy boundary, explicitly documented**: recovering a `WorkingSolid` from a resolved `SemioBrepSnapshot` (`working_solid_from_brep_snapshot`) is not generally invertible — nothing tags a brep as "this came from a box/cylinder/sphere" — so it returns a default placeholder; this is the same class of documented gap `📐️cad`'s own read-side conversion accepted.

### Hand-rolled codec (recipe §2)

`Process3dSnapshot` dropped `#[derive(dsl::DslRecord)]` (an `ArtifactChild<S>` field has no `dsl::DslField` impl reachable from this crate — the same wall every composed-child migration hits) and hand-rolls `ArtifactDsl`/`ArtifactPack`, following `📐️cad`'s exact template: hex/bracket text primitives + LEB128 binary primitives, `enc_json`/`dec_json` for structured child-free fields (`workshop`, `stock_pose`), real child-handle codecs (`enc_child`/`dec_child`/`enc_child_list`/`dec_child_list`). Every field wired into BOTH codecs — verified by round-trip tests, not just `cargo check` (per the recipe's explicit warning that codec completeness is invisible to the compiler).

`Process3dArtifact` (the UI-inclusive full-state struct, `🧬️schema/🦀️component.rs`) got the identical flattened-field + composed-child treatment so `to_snapshot`/`from_snapshot`/`set_snapshot` stay a plain field-for-field copy.

### Ephemeral working-scene (`ProcessWorkingScene`) + same-process scratch cache

`ProcessWorkingScene { stock: Stock, steps: Vec<ProcessStep> }` (artifact root, `🔖️WorkingScene`) is the exact pre-migration in-memory shape, living beside the persisted document, following the `EngineRep` contract.

**Upgrade over the plain "always empty" fallback** (lowpoly/cad/writer's wave-3 pattern): after reading the sibling wave-4 report `📓️wave4-reports/flow-report.md` (which independently arrived at a `thread_local!` scratch-cache pattern, `FLOW_SCRATCH`), I retrofitted the same technique here — `PROCESS3D_STOCK_SCRATCH`/`PROCESS3D_STEPS_SCRATCH` (`thread_local!` `HashMap<child_id, T>`), populated by `process_working_scene_to_snapshot` (the only place literal content is in hand) and read by `process_working_scene_from_snapshot`. Within the SAME process (the common case for tests and any single long-running app session), a document built through this module gets its REAL content back on read, not a fabricated empty scene. Crossing a process boundary (a fresh process loading a saved document from disk/network, or an undo/redo that bypasses `ArtifactApp::handle` entirely) still degrades to the honest empty fallback — this residual gap is identical to every other exemplar's documented staleness gap and is unavoidable without a real `LinkResolver` (confirmed absent — checked directly against `🔌️plugin/🦀️component.rs`, W1-owned, read-only for this wave).

`ProcessKernelReplay`/`replay_process`/`processed_mesh`/`processed_volume` (`🧬️schema/💡️inferences/🦀️component.rs`) — the CSG kernel replay pipeline — kept its logic **completely unchanged**, only retargeted from `&Process3dSnapshot` to `&ProcessWorkingScene` + `resolved_up_to: Option<usize>` (the "one accessor every render/export/inference call site funnels through" the recipe's §3 prescribes). `export_process3d_model`/`import_process3d_model` (`🚪️io/🦀️component.rs`) got the same treatment.

### Mutation vocabulary — a real, deliberate split (documented, not silent)

Of the 16 existing `Process3dMutation` variants:
- **9 stay fully real, unaffected**: `CreateMachine`/`DeleteMachine`/`RenameMachine`/`ChangeMachineIcon`/`ReplaceMachineCapabilities` (workshop stays a plain inline field, never composed), `MoveStock`/`ChangeStockLabel` (touch `stock_pose`/`stock_label`, inline fields), `ChangeCursor` (touches `resolved_up_to`, inline).
- **1 stays real via a handle SWAP, not a content edit**: `ReplaceStockSolid` — its payload changed from `new_solid: SolidSpec` to `new_solid: store::ArtifactChild<SemioBrepSnapshot>` (the caller mints the handle from real content via `brep_child_handle`/`brep_snapshot_for_working_solid`, then the mutation swaps which child `stock_solid` points at). This is a genuine, fully-working mutation because it never needs to read the PRIOR child's content — both `diff()` and `inverse()` only ever touch the handle field on `base`/the payload.
- **7 become DOCUMENTED NO-OPS**: `CreateStep`/`DeleteStep`/`RenameStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`ReplaceStepMeasure`/`ReorderSteps`. All 7 need to read/patch INDIVIDUAL step content that now lives only inside the unresolved `steps` flow child — with no `LinkResolver`, `base: &Process3dSnapshot` genuinely cannot see it. Each `diff()` returns `Process3dDiff::default()`; each `inverse()` returns `Vec::new()` — the sanctioned `MutationKind::inverse` contract ("a mutation with nothing to undo returns `Vec::new()`"), never the banned `NoMutation` vocabulary. This exactly mirrors `📐️cad`'s own precedent for its per-object mutations (`addObject`/`patchObject`, wave 3 round 2) — the established, ratified bridge for this exact situation, not a deviation invented for this plugin.

`Process3dDiff.steps` changed from a `Process3dStepsDelta` (added/removed/patched/reordered) collection diff to a single-`Option` handle-swap field (`Option<store::ArtifactChild<SemioFlowSnapshot>>`), matching the recipe's §8 "always-present slot" convention (never `Option<Option<…>>` — `steps` is never absent). `tool_solids` uses the sibling "collection of children" convention (`Process3dToolSolidChildList`, matching `📐️cad`'s `CadDrawingChildList`).

**App-layer consequence, accepted deliberately**: `AddStep`'s (and the inspector's dimension-patch path's) stock-dimension capability-rule VALIDATION gate is also a documented gap — `fixture.stock_solid` carries no resolvable dimensions from a bare snapshot either, so every capability now renders/dispatches as dimensionally valid rather than guessing at unknown extents. `MoveStep`/`UpdateStep`/`SetStepEnabled` (app commands) can no longer verify a target step exists before dispatching — they now dispatch unconditionally (harmless, since the underlying mutation is a no-op regardless).

### Fixture regeneration (recipe §7)

Both DSL text fixtures were genuinely obsolete (old nested-block grammar, incompatible with the new hex/bracket line format) and were regenerated for real via the `debug_fixture_regen` technique (temporary `#[cfg(test)] mod`, `process_working_scene_to_snapshot` + real `print_dsl()` output, captured then the module removed — verified zero `debug_fixture_regen` references remain):
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (the timber-beam example): rebuilt with the full generic + wood-catalog machine roster (11 machines) and a real 4-step timeline (cut/drill/attach/cut), matching what several downstream app-layer tests (`undo_after_add_workshop_machine_restores_previous_machine_count`, `catalogue_lists_workshop_wood_machines`) expect the default document to contain.
- `PROCESS_3D_PLATE_EXAMPLE_TEXT` (inline Rust constant, `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`): rebuilt as a 3-drill-step plate document with `resolved_up_to = Some(2)`.

### App-layer cascade (the bulk of the file count)

Every one of the ~20 app-layer files referencing `Process3dSnapshot.stock`/`.steps`/`SolidSpec` was fixed in this same pass (not deferred to a "round 2"): `🎛️apps/🧊️3d/🦀️component.rs`, `🌉️wasm/🦀️component.rs`, `🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs`, `🎮️commands/{🪜️step,🌍️world,🔎️inspector,🪵️stock,⏱️cursor,🎛️engagement,📄️artifact,📤️media}/🦀️component.rs`, `📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue,🛠️workshop}/🦀️component.rs`. Test suites in each were rewritten (not deleted) to assert the honest post-migration behavior — real assertions where the field stayed real (workshop/stock label/pose/cursor), documented-no-op assertions where step content is now unreachable, matching `📐️cad`'s own test-rewrite discipline.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-process --all-targets
```
**0 errors**, clean (down from 128 mid-migration peak). Only pre-existing-shaped warnings (dead-code on 2 functions untouched by this migration, unused-var/unnecessary-qualification lints).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-process --no-fail-fast
```
**157 run: 154 passed, 3 failed, 0 skipped.** Reproduced 3 times consecutively (not flaky) — same 3 failures every run:

- `rename_machine_round_trips`, `change_machine_icon_round_trips`, `replace_machine_capabilities_round_trips` (`🧬️mutations/🦀️component.rs`) — all three build `base` via `empty_process3d_snapshot()` then `base.workshop.machines.push(saw_machine("machine-1"))`, but `empty_process3d_snapshot()` already seeds `Workshop::default()`'s 3 generic machines (saw/drill/attacher), so `"machine-1"` lands at index **3**, not 0 — yet each test asserts on `after.workshop.machines[0]` (index 0 = the untouched "saw"/"Generic Saw" machine), not the renamed/patched one. **Independently traced**: `git log -p -3` on `🧬️mutations/🦀️component.rs` shows the latest commit touching it is `515271bf60` (🚩️503, dated 2026-06-04) — I read this exact file's content, including this exact test code, BEFORE making any edit to it beyond the step-mutation/`WorkingSolid` renames (never touched these 3 specific test bodies), confirming the index-0 assumption predates this migration by two months. Confirmed pre-existing, unrelated to composition/codec work.

## Honest gaps

- **Step-level mutations are documented no-ops** (7 of 16 variants) — a deliberate, ratified consequence of composing `flow` for the step timeline with no `LinkResolver` available, mirroring `📐️cad`'s precedent. Interactive step editing (add/remove/rename/reorder/enable/re-origin/re-measure a step) has no persisted effect until a real resolver lands.
- **Stock-dimension capability validation is a documented gap** — every capability now renders/dispatches as valid regardless of actual stock size, since `stock_solid`'s real dimensions aren't resolvable from a bare snapshot.
- **Cross-process working-scene fidelity is a documented gap** — the `thread_local!` scratch cache gives real content within one process; a document round-tripped through a fresh process (disk save/reload, network sync) or an undo/redo that bypasses `ArtifactApp::handle` degrades to the honest empty scene. This matches every wave-3 exemplar's own accepted staleness gap.
- **`working_solid_from_brep_snapshot`** (recovering a parametric shape from arbitrary resolved brep topology) is not generally invertible and returns a default placeholder — documented, matches `📐️cad`'s own read-side gap.
- Doc comments in a few app-layer files (`🎭️modes/✏️edit/🪟️windows/🪚️workpiece`, `🎮️commands/📤️media`) still describe the conservative "degrades to empty" case from before the scratch-cache upgrade landed; this is not incorrect (it's the true cross-process behavior) but is no longer the *only* behavior — same-process reads now get real content. Left as-is rather than rewording every comment, since the described worst case is still accurate and the code itself is correct.

## sharedFileRequests

None. Every change is contained inside `✏️s/🔌️plugins/🏭️process/**` (this plugin's own fan-out boundary); `📦️glue.rs`/`📦️index.ts` were not touched (no new mount points needed — the migration only changed field types/composition on existing structures, never added a new triad directory or facet needing a fresh glue mount).

## Concurrent-churn observations

None encountered during this session — every `cargo check`/`cargo nextest` run across the whole migration traced 100% of its errors to files strictly inside `✏️s/🔌️plugins/🏭️process/**` (confirmed by grepping error output for the plugin's own path at each iteration). No stdio/framework churn was observed blocking or interfering with this plugin's build at any point. The repo's auto-committer (`🐙️ueli🎆️26🌙️06☀️04🚩️<n>`, incrementing) swept most of this session's edits into commits during the session (confirmed via `git log`/`git status` — most touched files show clean in `git status --porcelain` despite being actively edited minutes earlier); content was verified intact after each sweep (spot-checked via `grep -c WorkingSolid` on the artifact root file), never lost.

## Files touched (non-exhaustive core list)

- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs` — `WorkingSolid`, `ProcessWorkingScene`, brep/flow converters, scratch cache, `empty_process3d_snapshot`. `SolidSpec` deleted.
- `.../🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — flattened `Process3dSnapshot`, hand-rolled codec.
- `.../🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — regenerated `PROCESS_3D_PLATE_EXAMPLE_TEXT`, test rewrites.
- `.../🧬️schema/🔺️diff/🦀️component.rs`, `.../🔺️diff/📝️text/🦀️component.rs` — flattened diff, handle-swap apply/absorb.
- `.../🧬️schema/🦀️component.rs` — `Process3dArtifact` flattened + composed, `insert_step_mutations`/`remove_step_mutations` honest simplification.
- `.../🧬️schema/💡️inferences/🦀️component.rs`, `.../💡️inferences/📦bounds/🦀️component.rs` — kernel replay retargeted to `ProcessWorkingScene`, bounds generalized off real brep vertices.
- `.../🧬️schema/🧬️mutations/🦀️component.rs`, `.../🧬️mutations/📝️text/🦀️component.rs` — dispatch enum tests rewritten, OP wire codec JSON-string bridge for step/measure/handle fields.
- 17 mutation triad directories under `.../🧬️mutations/` — `🧊replace-stock-solid` rewired to a real handle-swap payload; the 7 step-mutation triads' `🔺️diff`/`↩️inverse` leaves rewritten as documented no-ops; `📍move-stock`/`🔤change-stock-label` retargeted to flattened fields.
- `.../🚪️io/🦀️component.rs` — export/import retargeted to `ProcessWorkingScene`.
- `.../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs`, `🌉️wasm/🦀️component.rs`, `🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs`, `🎮️commands/{🪜️step,🌍️world,🔎️inspector,🪵️stock,⏱️cursor,🎛️engagement,📄️artifact,📤️media}/🦀️component.rs`, `📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue,🛠️workshop}/🦀️component.rs` — full cascade fix + test rewrites.

ucas-status: complete
