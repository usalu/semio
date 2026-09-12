# Wave B44 — what one mutation costs on a large puzzle-3d document

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B44, 2026-09-12 (report file dated 2026-09-13 per the
coordinator's naming). Written incrementally while the wave ran. Predecessors:
`📓️2026-09-10-wave-P5-lanes-render.md` (lane split), `📓️2026-09-10-wave-R-intake-delta-cost.md`
(intake pricing), `📓️2026-09-12-wave-B22-brush-mesh-upload.md`,
`📓️2026-09-12-wave-B24-command-ingress-round-trips.md`,
`📓️2026-09-12-wave-B32-world-lane-after-completion.md`,
`📓️2026-09-12-wave-B34-interaction-scope.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with peers (B42 on the fill
  cancel path, B45 bisecting the dock/pane reds, a Codex peer inside
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`). No `git commit`/`stash`/`checkout` was run;
  the ticket is NOT closed; nothing under `🗑️generated` was deleted.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- React serve already running on `127.0.0.1:6013` (`bun nx run workspace:dev -- 3d fixture concrete`,
  pid 30469, host TS live off the repo). The probe slot (`pgrep -f 'bun .*browser-probe'`) was empty
  before every browser run.
- Machine load average 26–28 throughout, so every absolute timing below is a debug/loaded number. The
  wave's claims are all **ratios measured on the same machine minutes apart**, never absolutes.
- `RUST_MIN_STACK=134217728` on every cargo invocation. No `CARGO_TARGET_DIR`/`RUSTC_WRAPPER` was set.

## 1 The instrument

Three probes, all in this ticket folder.

- `🔍️b44-mutation-latency.ts` — boots :6013 fresh, switches the example to Nakagin, proves a world
  selection, dispatches ONE mutation and timestamps every console line, then prints a phase table and
  a tap census. Output under `🗑️generated/b44-<stamp>-<label>.md` / `.ndjson`.
- `🔍️b44-switch-inspect.ts` — the scratch probe that settled what the example picker IS.
- `✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs` — the native attribution harness (`b44.native`) plus
  this wave's three laws.

Three temporary host taps carried the browser attribution and were **removed before the wave
finished** (`b44.intake` and `b44.project` in `🔌️PluginRuntime/🟦️.tsx`, `b44.world` in
`🌐️World3dHost/🟦️.tsx`).

### 1.0 One finding about the harness first

The battery's example picker is a `BUTTON[data-slot=select-trigger]` (`#playground.navbar.fixture`),
and a hit-tested Playwright click on it **times out** while the element is neither disabled nor
covered (`elementFromPoint` returns the trigger itself; `aria-expanded` stays `"false"`). Two full
probe runs read "the Nakagin switch never lands" because of exactly that. With `force: true` the
listbox opens and the switch lands in **2.8 s** (dispatch → `b44.world bytes=54254 count=180`). Wave
B45, which is bisecting the dock/pane reds, owns this class of obstruction — recorded here because any
probe clicking the navbar needs `force: true` until it is fixed.

## 2 Attribution

### 2.1 Browser, one mutation on the 180-object document (`b44-…-b2.md`)

| hop | measured |
| --- | --- |
| `Delete` keypress → `command ingress lane {actionId:"deleteSelection"}` | **3 ms** |
| `command ingress lane` → `command ingress settled` (the GUEST's own command turn) | **6 209 ms** |
| each queued `interactionSelect` (a canvas pick, touches no object) | **6 200–25 400 ms** |
| each queued `setCamera` (touches no object at all) | **2 300–4 800 ms** |
| host UI intake for the whole 4-surface publication (`b44.intake`, all lanes) | **96 ms** (largest surface), 30 ms warm |
| host projection of the viewport body (`b44.project`, 27 nodes / 2 708 steps) | **1 ms**, ×3 per refresh |
| `World3dHost` parse+apply of the 55 KiB instance lane (`b44.world`) | **0 ms** |
| example switch (dispatch → 180 instances rendered) | **2 760 ms** |

Two things fall out of that table and they decide the wave.

1. **The host is not the cost.** Intake, projection and the React commit together are under 150 ms per
   publication at Nakagin size. W-P5's 15–35 s repaint is gone (W-P4/W-R/B22/B24 closed it); the world
   lane becomes instances in **2.8 s** from the switch.
2. **The cost is per-COMMAND, in the guest, and it is paid by verbs that touch no object at all.** A
   `setCamera` on the 180-object document costs 2.3–4.8 s. That is not a payload cost — the camera
   lane is ~120 bytes.

And the 30 s budget is burned by QUEUEING: the commands dispatched at +66 s in run `b2` were still
settling at +105 s. Five queued hovers/picks/cameras at 2–7 s each is 30 s before the mutation is even
reached. That is exactly the checkpoint battery's shape — `gumball-scene-delta`,
`relocate-pose-delta`, `duplicate-selection`, `delete-selection`, `catalogue-add-object-kind` and
`volume-brush-add-target-volume` all timing out at `waitedMs≈30300` after the fill, while the same
verbs land in 3–20 s on a 1–3 object document.

### 2.2 Native, the per-stage split (`b44.native`, debug build, same machine)

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib b44_measures -- --test-threads=1 --nocapture`

| stage | 1 object | 180 objects | ratio | after this wave (180 objects) |
| --- | --- | --- | --- | --- |
| typed snapshot → fixture decode | 58 µs | 225 µs | 3.9× | 193–207 µs |
| `world_instances_geometry_json` (whole set) | 89 µs / 270 B | 2 449 µs / 55 154 B | 27× | 2 918–3 055 µs (cold path only) |
| **`fixture_geometry_fingerprint`** | 839 µs | **20 716 µs** | 24.7× | **641–652 µs (32× faster)** |
| `world_meshes_json` | 34 µs | 113 µs | 3.3× | 91–96 µs |
| `world_vortices_json` | 2 µs | 12 µs | — | 9 µs |
| viewport render, cold | 6 320 µs | 48 329 µs | 7.6× | 32 651–36 693 µs |
| **viewport render, geometry cache HIT** | 3 569 µs | **38 201 µs** | 10.7× | **22 846–23 487 µs (1.65× faster)** |
| `setCamera` dispatch+settle | 26 900 µs | 37 653 µs | 1.4× | 18 943–30 466 µs |
| `interactionSelect` dispatch+settle | 1 677 µs | 33 734 µs | **20×** | 32 385–51 513 µs (untouched) |
| `deleteSelection` dispatch+settle | 21 611 µs | 68 799 µs | 3.2× | 60 972–62 448 µs |

**`fixture_geometry_fingerprint` was 54 % of a cache-HIT viewport render on Nakagin** — a
`format!` of four `to_json_string` calls (~60 KiB materialized) hashed byte by byte, as the change key
for a cache that was about to hit. It has exactly two production callers and BOTH are on the render
path (`Puzzle3dPlayApp::geometry_jsons` and `Puzzle3dDocumentTreeKey::of`), so a refresh of two world
panes plus the outliner paid it three times — ≈ 62 ms per refresh, hashing JSON nobody read.

### 2.3 The turn count, and why it is the residual rather than the fix

A turn is one host↔guest round trip. `Puzzle3dSettled::turns` (added this wave) counts them.

| document | turns for one `deleteSelection`, at the host's own 256-item grant |
| --- | --- |
| 1 object | 104 / 113 / 120 / 153 / 215 / 235 |
| 180 objects | 188 / 208 / 217 / 269 / 284 / 362 |

So ~100–250 round trips per mutation on a ONE-object document, and the growth with `n` is real but
swamped by a document-independent term. At the browser's observed round-trip cost that fixed term
alone is seconds — it is why `setCamera` costs 2.3 s on a document it does not read.

The named mechanism: the typed-operation publication pages its emit at **one item per turn** —
`store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }`
in `publish_mounted_typed_operation_unit`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), plus the retirement state machine behind
it. Raising that grant is a framework-wide behavioural change for every artifact app and every law
that reads intermediate publication states; it is NOT something this wave could land and verify, and a
Codex peer is live in that same file. **It is the wave's named residual (§6), with the measurement
that makes it actionable: the emit itself is O(changed) (§4, law 3), so the turn count is the paging
constant, not the delta.**

## 3 What was NOT the dominant cost (measured, not argued)

| candidate from the brief | verdict | evidence |
| --- | --- | --- |
| all 350 instances re-serialised per mutation | real, but 2.4 ms of a 6 200 ms command | §2.2 row 2; fixed anyway (§4) |
| the world lane payload re-sent per mutation | real, but ≈ 96 ms of host intake | `b44.intake` at Nakagin size, §2.1 |
| how many command/patch pages the lane takes | 4 packed leaves, unchanged since W-P5 | W-P5 §2 |
| `registerBrushMesh` re-announce per brush object | **not per object** — 24 commands for the 14-mesh Nakagin catalog, ~300 ms each, and they settle | `b44-…-b1.md` tape, seq 15–22 |
| host intake first publication vs re-publish delta | re-publish is 30 ms warm vs 96 ms cold | `b44.intake` warm/cold pairs |
| React commit of the instance lane | 0 ms | `b44.world` |
| precompute invalidation of the whole spatial index | **never runs for these verbs** — `puzzle3d_action_uses_precompute` is false for `setCamera`/`interactionSelect`/`deleteSelection`, so `prologue.sync` taps are 0 in every browser run | tap census `prologue: 0` |
| **`fixture_geometry_fingerprint` hashing every object's JSON** | **THE dominant per-render term** | §2.2 |
| **the per-command turn count** | **THE dominant per-command term** | §2.3 |

## 4 What this wave changed

### 4.1 Guest — per-object instance residency (`🧊️main/🦀️.rs`, `✏️editor/🦀️.rs`)

`Puzzle3dInstanceResidency` is now the ONE owner of the published instance text. `refresh(fixture)`
walks the objects, computes a per-object `instance_record_fingerprint` (a handful of `Hash::hash`
calls over exactly the fields `instance_record_json` reads — no JSON, floats by `to_bits`), reuses the
retained record string for every object whose key did not move, re-serialises only the ones that did,
reassembles by concatenation, and reports the exact changed/removed id sets. It is carried across
dispatches and worker hops by the existing session slot (`Puzzle3dSessionState::instances`), and it
answers `bytes()` for the process-wide session census.

`Puzzle3dPlayApp::geometry_jsons` now returns `(instances, meshes, delta)`: instances off the
residency, meshes still keyed on the whole-fixture fingerprint (which is what actually changes when
the catalogs or the mesh set do), and the delta the residency just produced. A no-op refresh keeps the
LAST delta standing, because two window instances of one document render off one session and the
second render must be able to hand its own consumer the same `base`/`revision` pair.

### 4.2 Guest — the fingerprint is structural, not textual

`fixture_geometry_fingerprint` and `world_fit_revision` no longer materialise JSON. They fold
per-element structural hashes over objects (vortices included), references, target volumes and the two
opaque catalog members, through a new `hash_dsl_value` / `hash_axes` / `hash_optional_dsl_value`
trio. Same O(n) key over the same four members, none of the allocation: **20 716 µs → 650 µs.**

### 4.3 A nineteenth world-3d scene lane: `instancesDelta`

`instances_delta_json` (`instancesDeltaJson`, `framework.scene.world3d.instancesDelta`, optional)
carries `{base, revision, count, changed:[record], removed:[id]}`. Added on all four pinned sides so
neither can drift: the Rust `World3dScene` + `World3dSceneLane` tables and its pack wire
(`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`), the language-neutral declaration
(`🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`), and the TypeScript mirror
(`🔨️modules/🔺️mesh/🟦️.ts`).

**`instancesJson` stays AUTHORITATIVE on every publication, deliberately.** A pose-only lane would be
O(changed) BYTES but would leave every consumer that does not merge deltas — the wgpu world target
builds `World3dScene` directly — rendering a stale pose. The delta rides alongside it, so ignoring the
lane is always correct and the byte size of the authoritative lane stays O(n). That is a priced
decision, not an oversight: §2.1 measures the cost of keeping it at ≈ 96 ms of host intake, against a
cross-target staleness hazard.

### 4.4 Host — in-place instance application by id (`🌐️World3dHost/🟦️.tsx`)

`advanceWorldInstanceResidency(previous, instancesJson, instancesDeltaJson)` advances one consumer's
retained set. When the delta's `base` equals the retained revision, it substitutes the changed records
BY ID and every untouched record keeps its object identity — which is what lets the downstream
instanced-mesh memos see only the instances that moved. In-place application is restricted to pure
UPDATES of already-retained ids: `instancesJson` is index-addressed (a world pick resolves a hit by
array position, which is why a hidden instance stays in the array at zero scale), so an addition, a
removal, a `count` disagreement, a stale `base`, a malformed delta or no delta at all falls back to
the full parse.

### 4.5 The delta is suppressed when it buys nothing — and its text is assembled once

Two costs this wave introduced and then caught with the tree's own laws, recorded because the numbers
matter more than the fix:

1. `delta_json` originally re-parsed every changed record through `serde_json::from_str` to nest it in
   a `json!` object — on EVERY call, and `geometry_jsons` calls it on every render whether or not the
   residency republished. On a cold Nakagin publication that is 180 parses per render, more than the
   whole-set re-serialization this wave removed. The delta text is now assembled ONCE per
   republication, by concatenating the record strings the residency already holds.
2. A COLD publication published a 180-record delta beside the 55 154-byte full set. A consumer with
   nothing retained must read the full set anyway, so those bytes are pure wire cost and one more lane
   carrier to page. `delta_is_worth_publishing` now suppresses the lane entirely unless the residency
   is past its first revision AND the delta names fewer than half the set — so a cold publication, an
   example switch and an import carry no delta lane at all, and only an incremental edit does.

### 4.6 What was tried and REVERTED: a per-refresh projection memo

`ownedUiRefreshResponse` was memoized per `(surface, view.revision)` on the hypothesis that the three
identical `puzzle.3d.play.viewport` projections per refresh were one surface walked three times. The
after-run proved otherwise: they are the THREE distinct window surfaces (`1:puzzle3d-main`,
`1:puzzle3d-main-top`, `1:puzzle3d-main-perspective`), each legitimately projecting the same body key.
The memo removed no measured work, so it was reverted rather than left as unjustified churn in a file
a peer is live in. Recorded so the next wave does not re-derive the same hypothesis.

## 5 Laws

### 5.1 Guest — the delta is O(changed)

`✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs`:
**`a_pose_edit_reserializes_and_names_exactly_the_objects_that_moved`** — on the 180-object Nakagin
fixture: a cold refresh serialises every record exactly once; an unchanged fixture republishes nothing
and re-serialises nothing; moving ONE object re-serialises exactly 1 record, names exactly that id,
removes nothing, and the delta's own byte size is more than 8× under the full set. Measured:

```
[DEBUG] b44.delta moved=0a23d9c7-b75b-4166-8730-351367df9f8a deltaBytes=368 fullBytes=55154 rebuilt=1
```

**`the_incremental_residency_assembles_byte_identically_to_the_whole_set_encode`** — the differential
law that makes the per-object path safe: for both authored examples, at a cold refresh, after a
pose+hidden edit, after a removal and after an addition, the residency's assembled text is
byte-identical to `world_instances_geometry_json`'s one-shot encode.

### 5.2 Guest — the emit is O(changed), so the one-item-per-turn publication is too

**`a_mutation_emits_o_changed_operations_whatever_the_document_size`** — deleting the FIRST of 180
objects emits ONE operation (an index-addressed list diff would rewrite the whole tail — 180 round
trips at the publication's one-item grant); moving one object emits one; moving two emits two.

```
[DEBUG] b44.emit objects=180 deleteOperations=1 removed=01890804-66f2-4544-98f0-b6f0c0615492
[DEBUG] b44.emit objects=180 moveOperations=1
```

### 5.3 Host — applying a delta updates only those instances

`🧑‍🎨engine/🧪️tests/🚚️world3d-instance-delta/🟦️.ts` (11 laws), fixture-driven from
`🌐️World3dHost/🧫️fixtures/🚚️instance-delta.json` — the same neutral declaration the Rust producer is
pinned against. Cases: a cold consumer reads the full set; one moved instance is substituted by id
while `a` and `c` keep their object identity; an empty delta at the matching base keeps the whole
retained set; and a stale `base`, a removal, an addition, a `count` disagreement, a missing delta and a
malformed delta each fall back to the authoritative full set with NO record reused. Registered in
`⚛️react/📦️packages/🟦️typescript/vitest.config.ts`'s `engineTestSuites` — a co-located suite in no
include list is a gate that reads green while measuring nothing (wave B38).

### 5.4 Native — the turn measurement

**`b44_measures_the_turns_one_mutation_settles_in`** (measurement, not a gate — §2.3 explains why the
gate is the emit size) and `b44_measures_what_one_command_costs_per_document_size` (the §2.2 table).
`Puzzle3dSettled::turns` plus `settle_with_items` / `dispatch_reporting_with_items` were added to the
harness so a latency law can count round trips at the host's OWN 256-item grant rather than at the
one-item paging every other law reads.

## 6 Before / after in the browser (wasm #57)

The guest halves ride **wasm #57**: `@semio-tech/puzzle-plugin:component-dev` (4 m 44 s, `wasm-dev`
profile, `CARGO_PROFILE_WASM_DEV_DEBUG=false`) then `:materialize-dev`, landing
`…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/semio_s_plugin_puzzle_component.core.wasm`
at 15:01 (#56 was 12:20). The host halves are live TS off the repo and need no rebuild.

Same probe, same serve, same machine, 25 minutes apart — the one large-document publication that can
be driven end to end without a selection:

| measurement | #56 (before) | #57 (after) | |
| --- | --- | --- | --- |
| Nakagin `setActiveExample` guest command turn (`command ingress lane` → `performInvocation settled`) | **1 235 ms** | **808 ms** | 1.53× |
| picker click → 180 instances in `data-instances-json` | **2 760 ms** | **1 689 ms** | 1.63× |
| host UI intake, whole 13-surface publication | 282 ms | 282 ms | — |

### 6.1 The battery subset, and why it could not measure a large-document mutation

`bun 🔍️browser-probe.ts --only=fill-tab,fill-wait-ready,fill-apply-max,brush-stroke,gumball-drag,relocate,selection-keybindings,example-switch`
→ `🗑️generated/probe-2026-09-12T13-08-38.md` and a second run, both **FAULTS=0, hard=0, guest-death=0**:

```
verdict gumball-handle-enter PASS      verdict duplicate-selection PASS
verdict gumball-scene-delta PASS       verdict duplicate-reselects-clone PASS
verdict brush-preview-place PASS       verdict focus-selection PASS
verdict relocate-arm PASS              verdict delete-selection PASS
verdict relocate-pose-delta FAIL beforeLen=282 afterLen=282 instances=1 waitedMs=30443
verdict relocate-no-hard-fault PASS    verdict example-switch-instances PASS
verdict guest-alive-mutate PASS        verdict guest-alive-replace PASS
```

`gumball-scene-delta`, `duplicate-selection`, `focus-selection` and `delete-selection` — four of the
six verbs that burned their 30 s budget in the checkpoint battery — are **green**. But the honest
caveat is large and it is the wave's main measurement failure:

- The probe's group order is `read → mutate → replace`, and `example-switch` lives in `replace`, so
  every mutate verb ran on the **1-object** Concrete Forest document. The fill steps did not grow it
  either (`instances count=1 ids=["seed-left-001"]` throughout), so this run does NOT reproduce the
  169-object condition the checkpoint battery failed under. **Those four greens are not evidence of a
  fix.** Reproducing it needs either the full battery (whose fill does apply) or a probe that can order
  the switch before the mutations.
- `relocate-pose-delta` fails at **one** object (`instances=1`, `waitedMs=30443`) exactly as it failed
  at 169. It is therefore SIZE-INDEPENDENT and not a latency defect at all — a separate defect in the
  relocate gesture, and this wave's clearest statement about it.

### 6.2 No world selection can be established on the 180-object document

Six canvas picks at six pane fractions (0.50/0.50, 0.50/0.62, 0.45/0.55, 0.55/0.45, 0.50/0.72,
0.42/0.48), each waited 6–16 s: `data-selection-json` stayed empty on both panes, every time, across
three runs. The outliner is no second route — after opening `framework.panel.artifact` the tree
publishes **zero** entity rows on that document (`outliner entityRows=0`). The SAME pick at the SAME
fraction selects `seed-left-001` immediately on the 1-object Concrete Forest document
(`keybindings precondition attempt=0 via=pane-fraction ids=["seed-left-001","seed-left-001"] waitedMs=1`).

So on Nakagin there is nothing to mutate, which is why this wave could not put a `translateSelection`
or `deleteSelection` on a 180-object document at all and had to attribute through the example switch
plus the native harness instead. **This is a blocking defect for the whole large-document mutation
lane and it is upstream of latency**; it is handed over, not fixed here.

## 7 Verification

Every run foreground, tails quoted.

| command | result |
| --- | --- |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | **0 errors** |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests` | **0 errors** |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | **0 errors** |
| `RUST_MIN_STACK=134217728 cargo test … --lib mutation_latency -- --test-threads=1 --nocapture` | **5 passed / 0 failed** |
| `… --lib residency` | **2 passed / 0 failed** |
| `… --lib delta` | **4 passed / 0 failed** |
| `… --lib geometry` | **49 passed / 0 failed** |
| `… --lib example_switch` | **7 passed / 0 failed** |
| `… --lib nakagin` | **25 passed / 1 failed** — the one failure is a peer's (§7.1) |
| `cargo test -p semio-framework-ui-scene --lib -- --test-threads=1` | **117 passed / 0 failed** |
| `bun ./📜️script.ts test long` (renderer-react, whole suite) | **31 files, 1 020 passed / 0 failed**, and on a later re-run **30 passed / 1 failed** on a peer's mid-relocation import (§7.1) |
| `bun ./📜️script.ts test long --testNamePattern='world-3d instance delta'` | **1 file, 11 passed** |
| `bun ./📜️script.ts test long --testNamePattern='scene lane\|surface-scene\|world-3d\|Nakagin'` | **3 files, 31 passed** |
| `bun x tsc --noEmit -p .` (renderer-react) | **865 → 875 `error TS`** as peers moved the baseline, **zero** in any file this wave touched |
| `bun nx run @semio-tech/puzzle-plugin:component-dev` then `:materialize-dev` | wasm #57 materialized |

```
 Test Files  31 passed (31)
      Tests  1020 passed (1020)
   Duration  12.89s
```

```
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 736 filtered out; finished in 1.38s
```

```
test result: ok. 117 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

```
[DEBUG] b44.delta moved=0a23d9c7-b75b-4166-8730-351367df9f8a deltaBytes=368 fullBytes=55154 rebuilt=1
[DEBUG] b44.emit objects=180 deleteOperations=1 removed=01890804-66f2-4544-98f0-b6f0c0615492
[DEBUG] b44.emit objects=180 moveOperations=1
```

### 7.1 The failures that are NOT this wave's, with their attribution

A whole-suite `cargo test -p semio-s-artifact-puzzle-3d --lib -- --test-threads=1` reported twelve-odd
failures. Each was run in isolation and attributed:

| failure | attribution |
| --- | --- |
| `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` — worst turn 4.6 ms against a 2 ms budget, reproducible | **B42.** It measures `Puzzle3dWindowCommandWork`'s precompute turns, and `✏️editor/⏳️precompute/🦀️.rs` carries 62 uncommitted insertions whose own comment says "ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42" (`reap_fill_envelopes_for_test`, `fill_envelope_terminal_intent_is_pending`). This wave never edits that file, and `geometry_jsons`/the fingerprint are render-path only — not on this law's path. |
| `every_context_menu_row_dispatches_a_declared_action` — `left: 5 right: 3` Zoom rows | a peer adding context-menu rows; the checkpoint battery's own `context-menu-object-vocabulary FAIL missing=["hide-show","lock-unlock"]` is the work in flight |
| `two_instances_converge_disjoint_object_edits_via_backbone` | `module.vcs` fault: "remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized" — a VCS/backbone peer |
| `a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh` — `a camera move republished ["framework.scene.world3d.interaction"]` | the INTERACTION lane, whose `fillBuild`/`meshResidency` members this wave never touches; it PASSES in the `nakagin` family run, so it is order/global-state dependent on the live fill work |
| `settings_panel_*`, `catalogue`, `world_pick_null_clears…`, `selected_object_inspector…`, `gumball_active_only_for_transform_utilities…` | pass in isolation / in family runs; cascade of the above under `--test-threads=1` in one process |
| `cargo test -p semio-framework-ui-scene` (parallel) — `draw_permit_is_reserved_before_publication_and_orphan_close_is_one_page` | pre-existing parallel-order flakiness (inventoried in `📓️2026-09-10-order-dependent-tests-audit.md`); the whole suite is green at `--test-threads=1` |
| renderer-react `🧩️package-integration`, and briefly nine more suites | peer relocations mid-flight: `🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` → `🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` (resolved itself within minutes, 30/31 green after), and `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script` still missing |
| `semio-framework-plugin` TEST cfg `E0405: cannot find trait SpaceMember / MemberFactory`, and `semio-framework-schema` `couldn't read ⚛️component/🤖️generated.rs` | both transient mid-edit states in shared files; `cargo check` on each crate is clean and the same test binaries compiled minutes before and after |

## 8 Residual / handed over

1. **The per-command turn count is the ceiling and it is unfixed.** ~100–250 host↔guest round trips
   per mutation on a ONE-object document (§2.3), which at the browser's round-trip cost is the whole
   2–7 s. The named mechanism is the one-item publication grant in
   `publish_mounted_typed_operation_unit` plus the retirement ladder. Raising it changes behaviour for
   every artifact app and every law that reads intermediate publication states — a framework wave, in a
   file a Codex peer is currently live in. This wave's contribution to it is the measurement plus the
   law that the emit is already O(changed), so the count is the paging constant, not the delta.
2. **No world selection is possible on the 180-object document** (§6.2) — blocking for the whole
   large-document mutation lane, upstream of latency.
3. **`relocate-pose-delta` fails at one object** (§6.1) — size-independent, a relocate-gesture defect.
4. **The navbar example picker needs `force: true`** (§1.0) — B45's obstruction family.
5. **`interactionSelect` is still 20× at 180 objects** natively (1.7 ms → 33.7 ms, §2.2) and this wave
   did not touch it. It is the next per-command O(n) term after the fingerprint.
6. **The authoritative `instancesJson` lane stays O(n) bytes** by the §4.3 decision. Making it
   O(changed) means every world consumer — the wgpu target included — must merge deltas; the delta lane
   and its host applier now exist, so that is a contained follow-up rather than a design question.

## 9 Files

Changed:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`
  — `Puzzle3dInstanceResidency` (+`refresh`/`assemble`/`assemble_delta`/`delta_json`/`changed_ids`/
  `removed_ids`/`rebuilt_records`/`revision`/`bytes`/`delta_is_worth_publishing`),
  `instance_record_json`, `instance_record_fingerprint`, `hash_dsl_value`, `hash_axes`,
  `hash_optional_dsl_value`, `hash_object`; `world_instances_geometry_json`,
  `fixture_geometry_fingerprint`, `world_fit_revision` and `render` rewritten.
- `✏️s/…/✳️any/✏️editor/🦀️.rs` — `Puzzle3dPlayApp::{instance_residency, mesh_cache}`,
  `geometry_jsons`, `Puzzle3dSessionState::{instances, meshes}` + its `bytes`,
  `puzzle3d_session_check_out`/`_in`, the `render_body` call site, the new test-module declaration.
- `✏️s/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `Puzzle3dSettled::turns`, `settle_with_items`,
  `SETTLE_HOST_TURN_ITEMS`, `settle_into_reporting_with_items`, `dispatch_reporting_with_items`, and
  the session-registry laws retargeted off the removed `geometry` field.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` — `World3dScene::instances_delta_json` plus the
  nineteen-entry `World3dSceneLane` tables, `base`, `ToValue`/`FromValue` and the pack wire.
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json` — the `instancesDelta` row.
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts` — `instancesDeltaJson` on the scene type and in `WORLD3D_SCENE_LANES`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` —
  `WorldInstanceDeltaV1`, `WorldInstanceResidencyV1`, `parseWorldInstanceDelta`,
  `advanceWorldInstanceResidency`, and the host's own instance memo.
- `🧰️framework/…/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` — registers the new suite.

Created:

- `✏️s/…/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs` — the attribution harness and three laws.
- `🧰️framework/…/🧑‍🎨engine/🧪️tests/🚚️world3d-instance-delta/🟦️.ts` — the 11 consumer laws.
- `🧰️framework/…/🧱️elements/🌐️World3dHost/🧫️fixtures/🚚️instance-delta.json` — their neutral declaration.
- `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️b44-mutation-latency.ts`, `🔍️b44-switch-inspect.ts`, this report.

Touched and restored to their pre-wave state:
`🧰️framework/…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` (the §4.6 memo and the two temporary taps).
