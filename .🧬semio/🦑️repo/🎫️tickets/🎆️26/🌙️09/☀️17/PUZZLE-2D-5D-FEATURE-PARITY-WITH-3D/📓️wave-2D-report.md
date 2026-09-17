# 📓️ Wave 2D — puzzle ◻️2d clipboard, `createEdge`/`deleteEdge`, proximity auto-connect

Slice 2D of the 2026-09-17 parity fleet. Scope: OWED 4 + OWED 7 of `📓️E2-2d-gap-vs-3d.md`, verb-diff rows
`createAttraction` / `worldRelocate` / `setProximityRadius`, and `📓️E1` rows 21/30/40-41/61-63 + §H2.

Paths below are relative to the repo root. `EDITOR2` =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

---

## 1. What landed

### 1.1 Clipboard (`E1` §H2 / rows 61-63; `E2` §21)

| piece | file:line |
|---|---|
| `PUZZLE2D_CLIPBOARD_SCHEMA = "puzzle.2d.clipboard.v1"`, `PUZZLE2D_PASTE_OFFSET = 24.0` | `EDITOR2/🦀️.rs:1317` (region `📋️Clipboard`) |
| `puzzle2d_selected_node_ids` (handle-granularity selection resolves to its parent node, the 2d twin of `puzzle3d_selected_objects_from`) | `EDITOR2/🦀️.rs` §`📋️Clipboard` |
| `puzzle2d_selection_is_locked` | same region |
| `puzzle2d_copy_fragment_from` — nodes **with their handles** + every edge whose BOTH endpoints were copied | same region |
| `puzzle2d_cut_operations_from` — copy-then-delete as ONE mutation list via `puzzle2d_document_delta_operations` | same region |
| `puzzle2d_paste_operations_on` — fresh node/handle/edge ids, edges rewired onto the clones, default offset, returns the pasted node ids for reselection | same region |
| `Puzzle2dClipboardJob` (`ArtifactReservedJob`, outside the command enum — ported from `Puzzle3dClipboardJob` EDITOR3 `🦀️.rs:7646-7753`) | `EDITOR2/🦀️.rs:4562`, `impl InteractiveJob` `:4614`, `impl ArtifactReservedJob` `:4655` |
| `clipboard_media_type` / `copy_fragment` / `cut_operations` / `paste_operations` hooks | `EDITOR2/🦀️.rs:5018…` |
| `build_reserved_tool_job` routes `copy`/`cut`/`paste` before `import-media` | `EDITOR2/🦀️.rs:5045` |
| stale comment *"puzzle2d owns no clipboard fragment vocabulary of its own"* **deleted** (was EDITOR2 `🦀️.rs:4007`) | verified absent (`grep -c` = 0) |

Behaviour, all as asked: `copy` emits only `Effect::ClipboardWrite`; `cut` emits the ClipboardWrite **plus one**
artifact edit **plus** a subtractive interaction write that clears the selection; `paste` emits one artifact edit
**plus** an interaction write that re-selects the clones; a **locked** node refuses the cut with exactly one
`Effect::Notify` and no edit.

`mod+c`/`mod+x`/`mod+v` need no app declaration — the framework injects the three `ActionKind::Clipboard`
actions with those keys and EN/DE labels (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1062-1064`), exactly the way
puzzle3d gets them. What 2d adds over 3d is the **pointer route**: context-menu rows `copy` / `cut` / `paste`
(and a bare `paste` row on the empty-board branch) in `puzzle2d_context_menu_items`.

### 1.2 `createEdge` / `deleteEdge` (verb-diff `createAttraction`; `E1` rows 40-41)

- `EDITOR2/🎮️commands/💞️create-edge/🦀️.rs` — `createEdge {source, target, edgeKind?}`. Ported from
  `Puzzle3dCreateAttractionWork` / `🎮️commands/💞️create-attraction`. Validates against the document's
  `meta.kindCompatibility`; refuses (one notice, zero edits) on: missing/equal operands, an unknown handle id,
  an **occupied** handle, an incompatible kind pair.
- `EDITOR2/🎮️commands/💔️delete-edge/🦀️.rs` — `deleteEdge {id}`.
- Shared gate helpers on the editor root: `puzzle2d_handle_kind` `:1288`-adjacent, `puzzle2d_kinds_compatible`
  `EDITOR2/🦀️.rs:1288`, `puzzle2d_occupied_handles` `:1306`, `new_edge_id` / `puzzle2d_push_edge` / `puzzle2d_push_node`.
  Empty `meta.kindCompatibility` is permissive (matches the board engine's own linking gate).
- Engagement grammar: `connect <handle> <handle>` (and a bare `connect` over a two-handle selection) in
  `EDITOR2/🎮️commands/📨️engagement-submit/🦀️.rs`. **The operands are taken from the ORIGINAL line**, not the
  lowercased one the verb match reads — handle ids are case-sensitive. Placeholder updated at
  `EDITOR2/🎭️modes/✏️edit/🦀️.rs:228`.
- Context menu: exactly two selected handles ⇒ a `Connect` row bound to `createEdge`, shown **disabled** when
  either end is occupied or the kinds are incompatible (so the menu states what the document allows).

### 1.3 Proximity auto-connect (the 2d `worldRelocate`; `E1` rows 21/30)

- `EDITOR2/🎮️commands/🔗️proximity-connect/🦀️.rs`
  - `puzzle2d_proximity_pairs(fixture, node_id, radius)` — for every unoccupied handle of the moved node, the
    **nearest** unoccupied compatible handle of another node inside `radius`. Skips hidden/locked nodes and
    hidden/locked handles; each handle is claimed at most once.
  - `puzzle2d_proximity_connect(fixture, node_ids, radius)` — splices one edge per pair; the stationary peer
    stays `source` (the 3d resolution-root convention).
  - `proximityConnect {nodeId|ids, radius?}` — the programmatic verb (5d's `🎮️commands/📡️proximity-connect` twin).
- Drop wiring: `nodeDragEnd` in `EDITOR2/🎮️commands/🎲️apply-board-events/🦀️.rs` (the moves land first, then the
  auto-connect, **inside the same `applyBoardEvents` edit**) and `translateSelection`
  (`EDITOR2/🎮️commands/🚀️translate-selection/🦀️.rs`).
- Geometry: `puzzle2d_handle_world_position` `EDITOR2/🦀️.rs:1263` reuses the engine's own
  `handle_position_on_circle` / `handle_position_on_rectangle`, so a proximity hit is exactly a visual touch.
  (The rim distance is the **node** radius; a handle's own `radius` is its glyph size — this was a real trap.)
- `setProximityRadius` — `EDITOR2/🎮️commands/📡️set-proximity-radius/🦀️.rs`, absolute `value`/`radius` or a
  `delta`, clamped to `[0, PUZZLE2D_PROXIMITY_RADIUS_MAX = 480]`.

**Honest, spatially-bounded extent** (3d's Nakagin fault was `objects × 66 + attractions`):
- the search rejects a whole node on its **centre** alone via `puzzle2d_node_reach` `EDITOR2/🦀️.rs:1251`
  (circle radius / rectangle half-diagonal), so only nodes whose circle meets the query ball are ever expanded;
- `PUZZLE2D_PROXIMITY_CONNECT_MAX = 8` per moved node (`EDITOR2/🦀️.rs:89`) and
  `PUZZLE2D_PROXIMITY_GESTURE_MAX = 64` per **gesture** (`:93`);
- `puzzle2d_board_events_extent` `:2550` prices `rows + 64` when the batch carries a `nodeDragEnd`;
  `puzzle2d_generic_extent` `:2823` prices `addressed + 64` for `translateSelection`/`proximityConnect`.
  A whole-Nakagin drag is therefore `1024 + 64`, well inside the shared 4096 `PUZZLE_COMMAND_WORK_ITEMS`.

### 1.4 Config / schema (owed to slice 2C, see §5)

`Puzzle2dWindowConfig.proximity_radius` (`EDITOR2/🪟️window/🦀️.rs:24`, default `:45`, `runtime()` `:470`,
`split()` `:503`), `Puzzle2dPlayRuntime.proximity_radius` and
`PUZZLE2D_DEFAULT_PROXIMITY_RADIUS = 12.0` (`EDITOR2/🎚️config/🦀️.rs`). All four schema twins updated by hand:
`EDITOR2/🪟️window/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` (`proximityRadius`, `minimum: 0`,
proto field 12).

---

## 2. Registries touched (every new verb, every registry)

Verbs: **`createEdge`, `deleteEdge`, `proximityConnect`, `setProximityRadius`** (plus the three reserved
clipboard routes `copy`/`cut`/`paste`, which are framework-declared and app-owned only at the route).

| registry | where |
|---|---|
| `puzzle2d_command_variants!` | `EDITOR2/🦀️.rs:1650-1653` |
| `PUZZLE2D_RETAINED_TOOL_IDS` | `EDITOR2/🦀️.rs:1956-1959` |
| `PUZZLE2D_GENERIC_TOOL_IDS` | `EDITOR2/🦀️.rs:2010-2013` |
| `PUBLICATION_CONTRACTS` | `EDITOR2/🦀️.rs:2142-2145` — `createEdge`/`deleteEdge`/`proximityConnect` = `Artifact`; `setProximityRadius` = `WindowConfig` |
| `puzzle2d_dispatch_emit` match | `EDITOR2/🦀️.rs:2711-2714` |
| `bounded_first_step_tool_proofs!` tools | `EDITOR2/🦀️.rs:4714-4717` |
| `build_tool_job` | covered by the `PUZZLE2D_GENERIC_TOOL_IDS` arm (`Puzzle2dWindowCommandWork`) — no new arm needed |
| `.action_with` + `.action_args` | `EDITOR2/🦀️.rs:5205-5208`, `:5270-5275` |
| `.action_interactive_job(…, Migrated)` | `EDITOR2/🦀️.rs:5303-5306` |
| `command_from_action` | generated by the variants macro — no edit needed |
| terminology EN+DE | `EDITOR2/🗣️terminology/🦀️.rs` — `connect`, `disconnect`, `connect_needs_two_handles`, `connect_unknown_handle`, `connect_handle_occupied`, `connect_kind_incompatible`, `proximity`, `proximity_radius`, `copy`, `cut`, `paste`, `cut_locked` |
| retained-jobs fixture | `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` — four ids appended to `toolIds` |
| publication-authority fixture | `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json` — `copy` → `host-only`; `cut`,`paste` → `artifact+interaction`; `createEdge`,`deleteEdge`,`proximityConnect` → `artifact`; `setProximityRadius` → `window-config` |
| publication audit script | `PLUGIN/📦️packages/🟦️typescript/📜️script.ts` — `reserved2d` now `{copy,cut,paste,import-media}`; the `Puzzle2dWindowConfig` Ajv case gained `proximityRadius` (and, to unblock the run, `gridVisible`/`transformMove`/`transformRotate`/`areaBrushWidth`/`areaBrushHeight` that siblings 2C/2E/2F added to the schema without updating this case) |

To supply `ctx.labels` to the refusal notices, `Puzzle2dActionCtx` gained one field
(`labels: &'static Puzzle2dLabels`, `EDITOR2/🦀️.rs` §`🔖️ActionContext`), set at the single construction site in
`puzzle2d_dispatch_emit`. **Sibling slices adding `🎮️commands/*` arms get localized prose for free.**

---

## 3. Plugin-root reserved-routes policy — extended from 5d-only to 2d

`🧪️tests/🔬️tool-job-puzzle-reserved-routes` covered only puzzle5d. Added:

- `toolJobPuzzle2dReservedRoutesExact(source, host)` in the repo-root `📜️script.ts` (next to the 5d one),
  wired at the `puzzle2d = policyReadFileSafe(…)` read, the `puzzle2dReservedExact` call and its
  `failures.push(…)` row, and exported.
  It asserts what a **3d-shaped** (one-step) clipboard producer owes — 5d's fixed-page ingress clauses do not
  apply: no app-owned reserved factory; all four reserved ids branch inside `impl ArtifactEditor for
  Puzzle2dPlayApp`; an app-owned fragment vocabulary (`puzzle.2d.clipboard.v1` + the three hooks + the three
  `puzzle2d_*_operations/fragment` producers); `import-media` keeps its own branch; and the step is
  cancellable, **completes before it prepares its retained output**, returns the exact `CommitCandidate` shape
  and closes to terminal-empty.
- `toolJobPuzzle2dReservedRoutesSelfTests(host)` in
  `PLUGIN/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts`, called from the existing 5d self-test so the count
  rolls up: **11 hostile mutations + 1 valid fixture**.

**Verdict (run):** live puzzle2d source ⇒ `true`; self-tests executed ⇒ **26** (14 puzzle5d + 12 puzzle2d), every
hostile mutation refused. Probe kept at `TICKET/🗑️generated/2D/reserved-probe.ts`.

---

## 4. Laws

| file | laws |
|---|---|
| `EDITOR2/🧪️tests/🔬️clipboard/🦀️.rs` (registered as `mod clipboard_tests` in `EDITOR2/🦀️.rs` §`🧪️UnitTests`) | `copy_then_paste_round_trips_the_concrete_forest_seed` (fresh node **and** handle ids, default offset, every handle carried); `copy_then_paste_restores_a_twelve_node_nakagin_subgraph` (12 connected nodes derived from the edge list at test time; asserts `+12` nodes **and** `+internal_edges`); `cut_removes_and_one_undo_restores_the_topology`; `cut_refuses_a_locked_node_with_a_notice`; `paste_refuses_a_foreign_media_type` |
| `EDITOR2/🎮️commands/💞️create-edge/🧪️tests/🔬️unit/🦀️.rs` | `create_edge_connects_two_compatible_open_handles` (+ one-undo); `create_edge_refuses_an_incompatible_handle_kind_pair`; `create_edge_refuses_an_occupied_handle`; `delete_edge_drops_the_named_edge`; `engagement_connect_line_creates_the_same_edge` |
| `EDITOR2/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs` | `proximity_connects_inside_the_radius_and_never_outside`; `proximity_respects_the_radius_gate_and_the_compatibility_table`; `proximity_skips_locked_hidden_and_occupied_handles`; `proximity_never_exceeds_the_gesture_budget`; `node_drop_auto_connects_as_one_history_edit` (+ one-undo) |

No new document-mutation *kind* was introduced (every edit rides the existing typed
`puzzle2d_document_delta_operations` vocabulary), so no new language-neutral mutation fixture vector or Python
second implementation is owed.

---

## 5. Commands run — verdicts

| command | verdict |
|---|---|
| `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle2dPlayApp` | ❌ **exit 1, and NOT on puzzle2d** — it dies in `validateWindowOwnershipSchemas`, which validates all three artifacts regardless of the owner filter: `Puzzle5dWorldWindowConfig neutral fixture failed Ajv validation: must have required property 'voxelDims'`. Owed to slice **5G** (they added `voxelDims` to the 5d window schema without updating that Ajv case). |
| 2d half of the same oracle, isolated (`TICKET/🗑️generated/2D/oracle-probe.ts`) | ✅ **all four cross-checks true**: manifest `action_interactive_job` pairs ↔ fixture routes; `PUZZLE2D_RETAINED_TOOL_IDS` ↔ fixture migrated set; `PUBLICATION_CONTRACTS` keys **and lanes** ↔ fixture; `bounded_first_step_tool_proofs!` tools ↔ fixture migrated set. Zero diffs either way. |
| reserved-routes policy probe (`TICKET/🗑️generated/2D/reserved-probe.ts`) | ✅ live 2d source `true`; 26 self-tests executed, all hostile mutations refused |
| `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | ✅ **rc=0, `Finished` in 10m 37s, 0 errors, 7 warnings** (none in this slice's files) — §6.1 |
| `CARGO_INCREMENTAL=0 cargo test … --lib -- clipboard_tests create_edge proximity delete_edge engagement_connect` | ⚠️ **11 passed / 10 failed** — all 10 on a crate-wide `interactive-job.catalog-authority` harness fault that also kills the untouched `set_fill_count` test — §6.2 |

---

## 6. Build verdict

### 6.1 Native check — ✅ GREEN

```
CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d \
  --features component-app-assembly --message-format=short
→ rc=0
→ "Finished `dev` profile [unoptimized] target(s) in 10m 37s"
→ 0 errors
→ warning: `semio-s-artifact-puzzle-2d` (lib) generated 7 warnings
```
Full log: `TICKET/🗑️generated/2D/check-native-short.txt`.

The 7 warnings are **not** in this slice's new files: 5 × `unnecessary qualification`
(2 in `EDITOR2/🦀️.rs:3739-3740` — these were *caused* by my `Point` import shadowing a peer's fully-qualified
path, so I un-qualified those two lines; 4 in `EDITOR2/🪟️window/🦀️.rs:255-275`, slice 2C/2E's) and
1 × `unused imports: Buildable and HasBase` in `EDITOR2/🎭️modes/✏️edit/🦀️.rs:12` (slice 2B/2E's).
Neither the clipboard region, the three new command files, nor the three new test files emitted a warning.

### 6.2 Filtered unit tests — ⚠️ 11 passed / 10 failed, **all 10 on a crate-wide harness fault that is not this slice's**

```
CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib \
  -- clipboard_tests create_edge proximity delete_edge engagement_connect
→ 11 passed; 10 failed; 853 filtered out
```
Log: `TICKET/🗑️generated/2D/test-2d-filtered.txt`.

**Passed — every pure-function law of this slice (5/5):**
`proximity_connects_inside_the_radius_and_never_outside`,
`proximity_respects_the_radius_gate_and_the_compatibility_table`,
`proximity_skips_locked_hidden_and_occupied_handles`,
`proximity_never_exceeds_the_gesture_budget`,
`paste_refuses_a_foreign_media_type`.
(The other 6 passes are the board engine's own pre-existing `linking`/`brush` proximity laws that the filter
also matched — note `engine::linking::tests::board_host_node_drag_proximity_connect_*` already covers the
*engine-side* drag snap; the work in §1.3 is the guest/document-lane twin that owns the compatibility gate and
the window's `proximityRadius`.)

**Failed — all 10 are the registry-backed app-level tests**, and every one dies in the shared test harness
before reaching a single assertion of mine:

```
🧰️framework/…/🔌️plugin/🦀️.rs:21876
tool proof catalog must exactly join migrated generated declarations to live concrete factories:
FaultCode("interactive-job.catalog-authority")
"tool factory proof rejected tool 'setActiveTool': … factory='BoundedFirstStepCommandJobFactory'
 registered_factory='BoundedFirstStepCommandJobFactory', generated_migrated=false, typed_join=false"
```

**Proof this is not mine:** I re-ran a pre-existing, completely unrelated app-level test that this slice never
touched — `commands::set_fill_count::tests::set_fill_count_publishes_the_config_count_and_never_the_document`
— and it fails with the **byte-identical** fault (`TICKET/🗑️generated/2D/test-2d-baseline-probe.txt`,
`0 passed; 1 failed`). Every registry-backed puzzle2d test in the crate is currently red on it.

The rejected tool is **`setActiveTool`**, a framework-injected host-configuration verb declared by
`Puzzle2dHostConfigurationProofs` (`EDITOR2/🦀️.rs`, the deliberately *generic* `bounded_first_step_tool_proofs!`
block with `tools: ["setActiveTool", "setActiveUtility"]` and no `factory_type`). It is in the live `migrated`
set but not in `generated_ids` (= `PUZZLE2D_RETAINED_TOOL_IDS`), so the catalog cannot form the typed join.
None of this slice's four verbs appear in the fault: `createEdge`, `deleteEdge`, `proximityConnect` and
`setProximityRadius` are present and correctly joined in **both** the `migrated` set and `generated_ids`.
This belongs to whichever slice last moved the tool/utility proof block (2B's Fill/tool work is the likely
owner) — **owed to integration**, per the 19:55 addendum I did not chase it.

---

## 7. NOT verified / owed to integration

- **wasm32-wasip2 check** of `semio-s-artifact-puzzle-2d` — not run. Nothing this slice added is
  `cfg(wasm32)`-gated (the clipboard job, the commands and the geometry helpers are all target-neutral; the one
  wasm-only file, `EDITOR2/🌉️wasm/🦀️.rs`, was not touched), so the risk is low, but it is **owed to integration**.
- **The 10 app-level laws of this slice have never actually executed their assertions** — the crate's shared
  test harness rejects `setActiveTool` before any of them runs (§6.2). Re-run them the moment that is fixed:
  `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- clipboard_tests create_edge proximity delete_edge engagement_connect`
  Until then the copy/cut/paste round trips, the `createEdge` refusals, the engagement `connect` line and the
  one-edit drop are **compiled and reviewed but not executed**. The five pure-function laws did run and pass.
- **Runtime/browser**: no battery, no probe, no serve. The drop auto-connect, the `Connect` context-menu row and
  the clipboard hotkeys have never been driven in a browser by this slice.
- The publication-authority audit cannot exit 0 until slice **5G** fixes the 5d Ajv case (§5).
- **Crate-wide blocker owed to integration:** `interactive-job.catalog-authority` rejects `setActiveTool`, which
  reds every registry-backed puzzle2d test (§6.2). Not this slice's — reproduced on an untouched test.

---

## 8. Hand-offs

- **→ 2C (panels/options/settings).** The window-config field and the verb are **landed and registered**: build
  the settings stepper against `Puzzle2dWindowConfig.proximity_radius` / runtime `proximity_radius`, dispatching
  **`setProximityRadius` with `{ "value": <f64> }`** (it also accepts `radius`, or a relative `delta`). Label
  fields already exist: `labels.proximity_radius` ("Snap Radius" / "Fangradius") and `labels.proximity`
  ("Proximity" / "Nähe"). Range: `0.0 ..= PUZZLE2D_PROXIMITY_RADIUS_MAX (480.0)`, default `12.0`
  (`PUZZLE2D_DEFAULT_PROXIMITY_RADIUS`); `0` disarms the feature entirely. The verb is already in
  `PUBLICATION_CONTRACTS` as `WindowConfig` and in the publication fixture — **do not re-add it**.
- **→ 2C / 2E / 2F.** I added your new `Puzzle2dWindowConfig` keys to the audit script's Ajv case
  (`gridVisible`, `transformMove`, `transformRotate`, `areaBrushWidth`, `areaBrushHeight`) so the run gets past
  2d. Keep that case in sync when you add more.
- **→ 5G.** `Puzzle5dWorldWindowConfig`'s Ajv case in `PLUGIN/📦️packages/🟦️typescript/📜️script.ts` needs
  `voxelDims`; until then `publication-authority-audit` exits 1 for **every** owner.
- **→ 2G (battery).** Exact verb ids to exercise: `createEdge {source,target}`, `deleteEdge {id}`,
  `proximityConnect {nodeId|ids, radius?}`, `setProximityRadius {value}`, and the framework-reserved
  `copy` / `cut` / `paste` (keys `mod+c` / `mod+x` / `mod+v`). Context-menu row ids: `connectHandles`, `copy`,
  `cut`, `paste`. Engagement line: `connect <handleId> <handleId>`. Drop lane: an `applyBoardEvents` batch
  carrying `{"name":"nodeDragEnd","payload":{"moves":[{"id":…,"x":…,"y":…}]}}` — a drop landing two compatible
  open handles within 12 board units must produce **one** new edge and **one** undo step.
- **→ integration.** `Puzzle2dActionCtx` gained a `labels` field; any sibling adding a `Puzzle2dActionCtx { … }`
  literal must set it (there is exactly one construction site today, in `puzzle2d_dispatch_emit`).
