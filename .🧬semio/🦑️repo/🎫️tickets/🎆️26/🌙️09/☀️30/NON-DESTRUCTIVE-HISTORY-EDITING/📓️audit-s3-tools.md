# 📓️ Audit S3-TOOLS — session-3 tool conversions (read-only, 2026-10-02 ~20:00)

Reviewer: S3-AUDIT-TOOLS (Sonnet, read-only). Method: read each WP's "Session 3" section + design §5/§12/§13/§17/§19/§20 + plan W3-T brief, then the cited code on disk. **Nothing was compiled or run** (tree red from the peer DSL extraction); every verdict is static reading. Counts below are from `git ls-files`+grep over `✏️s`, `🌎️hub`, `🧰️framework` at audit time.

Aliases: `FW`=`🧰️framework/🔨️modules`; `OSM`=`🧰️framework/🛍️products/💻️os/🔨️modules`; `PL`=`✏️s/🔌️plugins`; `SUB`=`🏅️standards/🔖️1/🪆️subsets/✳️any`; `TM`=`FW/🛠️tool-machine`; `PLG`=`OSM/🔌️plugin/🦀️.rs`; `DAG`=`OSM/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`; `WGPU`=`OSM/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`; `FLOW`=`PL/🌊️flow/🗿️artifacts/🌊️flow/SUB`; `G3`=`PL/🌀️procedural/🗿️artifacts/🧊️generation3d/SUB`.

## Verdicts

| WP | Verdict | Why (one line) |
|---|---|---|
| S3-FLOWCAD (flow/cad/§12/node-graph rows) | **accept with changes** | contract + one decoder + intent leaves are sound; dag journal can silently drop moves, wgpu row writer is a second, source-scan-tested encoder (F1–F6) |
| S3-CONTROLS (config press, energy leaves, scrub glue) | **accept with changes** | glue is correct on disk; 130 energy fixture files are `{}` placeholders, forms frozen law unresolved with `[DEBUG]` (C1–C4) |
| S3-SPATIAL (shooting, fem, lowpoly, World3dHost) | **accept with changes** | wgpu has no live gumball consumer (N9), gen3d has no corpus law, fem clocks are twin copies (S1–S4) |
| S3-STROKES (wfc, raster fill-region, remodel §15, process3d) | **accept with changes** | §15 import re-decodes 15 frames per tick, no gesture replay laws, wgpu has no bucket (K1–K5) |
| S3-TEXT (N7/N8, trinity, stdio text) | **accept with changes** | N7/N8 designs are right; trinity lost add-node and keeps whole-graph inverses (T1–T4) |
| S3-PROCEDURAL (change-widget-input, intent kinds, gen3d live) | **accept with changes** | §19.4 decision not implemented; leaf accepts phantom channels (P1–P5) |
| S3-LAYOUT (wgpu path paint, chips) | **accept** | shared corpus + independent numpy/Three.js oracles; only documented approximations (L1–L2) |
| S3-DRAW / NOTE | **accept** | hand labels gone, chips + references declared, laws written; only run-evidence owed (D1) |
| S3-GRAPHS (shared decoder, wires drag, guests) | **accept with changes** | decoder + guests converge; 12 near-identical drag-emit wrappers, no gesture replay laws (G1–G4) |
| S3-PUZZLE (3d/5d, every-example, N3) | **accept with changes** | O(document) publication red, 5d/2d law runs never executed, per-subset law copies (Z1–Z4) |

No WP is rejected. Greps confirm: `coalesce_key|Emit::amend|amend_config|AmendLast` = 0 hits in `✏️s|🌎️hub|🧰️framework`; only genuine replace intents keep `SetSnapshot` in flow (`FLOW/✏️editor/🦀️.rs:2959`, `setActiveExample`) and gen3d config (`set-active-example`, `import-document`).

## X — cross-cutting (route to coordinator)

**X1 critical (gate) — almost no session-3 Rust has run.** Per the reports' own tails, WRITTEN BUT UNVERIFIED: flow/cad/plugin glue/os-infinite/surface (FLOWCAD S3.6 owed 1–6), energy/forms/gis/playbook (CONTROLS S3.5), fem/lowpoly/shooting (SPATIAL S3.4), raster/pixels/remodel/wfc/process3d (STROKES S3.7), writer/rewriting/stdio/wgpu (TEXT S3.4), gen2d/gen3d/DEV (PROCEDURAL S3.7), layout/wgpu canvas2d (LAYOUT S3.9), draw/note (DRAW S3.8), dag/wires/math/seq/space (GRAPHS S3.2), puzzle 2d/5d (PUZZLE S3.2). Only TS/Python/lint/tool-machine/ui-crate suites are green. Fix: after TREE GREEN run the owed lists in order, one gated cargo at a time; nothing in this audit is "accepted" until its owed run is green. Owner: coordinator + each WP.

**X2 major — one-liner copied ~20×; the node-drag emit wrapper copied 12×.** `HybridLogicalTimestamp { actor: 0, physical_ms: default_now_ms().unwrap_or(0), logical: 0 }` is hand-written in wfc 2d/3d/bitmap, flow, cad, gen2d/3d, dag, math, sequence ×2, trinity, wires, process3d, shooting, lowpoly, layout, puzzle 2d/3d, note, raster, draw, space, fem 2d/3d, `🔌️plugin/⏯️tool-run/🦀️.rs:1947`, `OSM/🔌️plugin/🛠️tool-machine/🦀️.rs:195,197,562`. `node_drag_commit(format!("{APP}#{verb}"), ActorId(seed), gesture, leaves, clock)` + the `seed.is_empty() → plain / else commit_transaction` branch is repeated in wfc 2d/3d (`🛠️tools/✋️drag/🦀️.rs:44`), flow (`✋️drag/🦀️.rs:44`), gen2d (`✏️node-graph-edit:110`), gen3d (`:83`), dag (`:127`), sequence (`🕸️node-graph:88`, `✏️editor/🦀️.rs:1964`), math (`:126`), trinity (`:130`), wires (`✏️editor/🦀️.rs:290`), space (`:84`). AGENTS: repeated code must be close / shared. Fix: `TM` gains `authoring_clock()` and `node_drag_emit(app_id, verb, seed, gesture, leaves) -> NodeDragEmit<M>` (commit/plain/none) used by all 12; delete the copies. Owner: S3-GRAPHS (with FLOWCAD/STROKES/TEXT/PROCEDURAL adopting).

**X3 minor — `[TRACE]` unconditional logs in gesture hot paths.** `dag_debug_log(&format!("[TRACE] …"))` ×12 in `DAG` — `:4903` logs every moved node of every drag tick (one console line per node per pointer move), `:5534`/`:5706` every slider tick, `:5638` port press, `:6699` per draw — allocate and `console.log`/`eprintln!` unconditionally; `engine_canvas_debug_log("[TRACE] wgpu node-graph screen gesture …")` at `WGPU:4108-4125` prints the edit list per gesture. Not `[DEBUG]`-prefixed, so the cleanup grep cannot find them. Fix: delete or gate behind a compile-time feature; convention `[DEBUG] `. Owner: S3-FLOWCAD (dag/wgpu node-graph regions).

**X4 minor — core `[DEBUG]` probes still in production store** (S3-W1G bisect): `OSM/🏪️store/🦀️.rs:17920,18015,18708,18913`. Out of this scope but must not survive closure. Owner: S3-W1G.

**X5 minor — design.md numbering:** the second `4.` under §20 ("§19.2 applies to every inserted operator") is §19.4 (status.md calls it that); fix the heading so adopters cite one number. Owner: coordinator.

## S3-FLOWCAD

F1 **major — dag journal silently drops edits past 64 rows; callers ignore the answer.** `DAG:2108` `DAG_GRAPH_EDIT_CAPACITY = 64`; `journal_moves_since` (`DAG:4996`) is `drags.into_iter().all(journal_drag)` and returns `false` when full; callers discard it (`FW/🗺️surface/🕸️node-graph/🦀️.rs:620,674`, `OSM/🌊️flow/🖥️host/🦀️.rs:1830`); `push_graph_edit` (`DAG:4963`) drops silently too. One `move` row per DISTINCT offset (align/grid-snap give one per node) so align of >64 nodes moves all nodes on the host and tells the guest about 64: host and document diverge, next resync jumps. Also inconsistent with `NODE_GRAPH_EDIT_MAX_ROWS = 256` (`TM:902`). Fix: collapse to ONE `move` row per release when offsets are equal, else raise the capacity to the shared 256 and make overflow a hard refusal (abort the gesture: `pointer_cancel_screen` semantics, zero trace) instead of a partial journal; add a law with 65 distinct offsets. Owner: S3-FLOWCAD.

F2 **major — "same rows on both hosts" is pinned only by source-text scans.** wgpu writes rows with its own bounded builder `write_graph_edit_action` (`WGPU:4127-4193`) while React/flow use `dag_graph_edit_rows_json` (`DAG:2088`); the only wgpu check is `…/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts:55-61` (`expect(wgpu).toContain('builder.string(Some("operation"), "connect")')`) — field names/values/order are never compared to the fixture. Docstring claims "byte for byte". Fix: Rust law in the wgpu crate that runs `write_graph_edit_action` over the fixture `journal.answers` and decodes the produced action through `node_graph_edit_rows` (and compares to `dag_graph_edit_rows_json`); or give the bounded builder a shared row-visitor so one function owns the shape. Owner: S3-FLOWCAD + S3-W2C.

F3 minor — **wgpu `Leave` = release.** `WGPU:4093,4098` map `DagPointerPhase::Up | Leave` to `pointer_up_screen` (commits and journals a drag) while the projection plan treats `Leave` as Idle (`DAG:3326`) and React's `onPointerLeave` only clears hover (`OSM/🔌️…/🕸️NodeGraph/🟦️.tsx:1186`); the cancel path exists (`node_graph_pointer_cancel_into`, `WGPU:3899`) but `Leave` never reaches it. Confirm pointer capture makes `Leave` unreachable mid-drag; otherwise route `Leave` during a gesture to cancel. Owner: S3-FLOWCAD / S3-W2C.

F4 minor — **closed-vocabulary hole in the flow guest.** `FlowNodeGraphEditOp::InsertPort.side: String` (`FLOW/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:25`) is a `DslEnum` payload decodable from DSL/binary without passing `node_graph_edit_rows`; `node_graph_edit_result` treats every non-`"input"` string as output (`:118-119`). Fix: carry `NodePortSide` in the op (derive its DSL form) and refuse other values at decode. Owner: S3-FLOWCAD.

F5 minor — **row label of a wire+drag release is the wire.** `flow_drag_tool_emit` (`FLOW/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs:44-47`) puts `prepared` (insert-edge…) before `drag-nodes`; only gen3d adopts `tool_intent_kinds` (`G3/✏️editor/🦀️.rs:1958`), so the §19.1 default (first op) labels the row "Connect". Fix: flow (and every guest that prepends support leaves: wfc, wires) declares `tool_intent_kinds("…#nodeGraphEdit") = ["drag-nodes"]`. Owner: S3-FLOWCAD.

F6 minor — **hand-written, raw-id label in §12 finalize.** `OSM/🔌️plugin/⏪️time-travel/🦀️.rs:2311` builds `LocalizedLabel::native("History edited in {store_id}", "Verlauf in {store_id} bearbeitet")` (raw `<slot>/<childId>`, two hard-coded locales). Also `:2272` `publish_time_travel_member(..).await?` (same file) runs AFTER the store finalized: a backbone error returns before `TimeTravelEvent::Finalized` is applied, leaving the session stuck although the member is finalized (short connection loss must not freeze, AGENTS). Fix: apply `Finalized` first, report the publish failure as a retryable notice; label from the framework glossary with the entity label. Owner: S3-W2A (§12 region owner S3-FLOWCAD).

F7 minor — `node_graph_delete_selection_spec` (`PLG:15215`) takes `is_de: bool` and hard-codes "Knoten/node" phrases (no-default-language rule), and an empty selection drops the item without a reason (`:15216`) instead of a disabled row with `reason`. Fix: use the locale glossary + `disabled_because`. Owner: S3-FLOWCAD / S3-W2A.

F8 minor — schema typo `TM/🧬️schema/🔣️node-graph-edit-rows/🔣️.json:5` "bounded to256"; the fixture is sha256-pinned in `TM/🧪️tests/🧪️node-graph-row-ownership/🟦️.ts` (a seal, not a law) — fine, but re-seal with the typo fix. Owner: S3-GRAPHS.

Passed: one decoder (`TM:900-1006`, 3 laws + Ajv strict), all consumers read `TM/🧫️fixtures`, `setHostSnapshot`/`deleteSelection` gone (0 refs), flow content edits land as id-keyed leaves (`flow_content_leaves`), drag = one `ToolTransaction` of relative `drag-nodes` with zero-offset = zero trace (`✋️drag/🦀️.rs:44-61`), dag cancel clears the journal (`DAG:5746-5755`), §12 member finalize now publishes every authored transition (`⏪️time-travel/🦀️.rs:667-668,2270-2273`). CAD: nothing new beyond S2 + `coalesce_key` removal (0 hits).

## S3-CONTROLS

C1 **major — the new energy leaves have no evidence.** All 130 files under `PL/🔋️energy/🗿️artifacts/🔋️model/SUB/🧫️fixtures/🧬️mutations/*change-{site,ground-temperature,run-period}*` are 2-byte `{}` placeholders (e.g. `🌍️change-site-latitude/✅️sets/🦠️mutation/🔣️.json`); the "26 vectors", the Python second implementation and the feature rows therefore compare nothing, and CONTROLS never reported `schema mutation-payloads --under energy`. Fix: run the writer (`SEMIO_ENERGY_WRITE_FIXTURES=1 …writes_the_committed_vector_when_requested`), then payloads lint must be 0; add a gate that refuses `{}` witness bodies. Owner: S3-CONTROLS.

C2 **major — frozen-press law unresolved, `[DEBUG]` left.** `PL/📋️forms/…/✏️editor/🧪️tests/🧪️field-transactions/🦀️.rs:121,123,127,130,132` (`a_history_edit_freezes_an_open_press_with_zero_trace`) — the late release after a freeze "did not retire within 30 seconds". The ledger logic is right on paper (`TM:728-815`: `ScrubLedger::abort_all` closes the press, `send` answers `Idle` for a closed gesture), so either the verb path still waits on a retained operation (`settle_tool_operation` `None if emit.transaction.is_none() => command_logged`) or the law's pump is wrong. This is the zero-trace abort path for the frozen reason; do not close with it red. Fix: root-cause with S3-W2A, remove the eprintlns. Owner: S3-CONTROLS + S3-W2A.

C3 minor — `settle_press_config` Abort arm answers `true` (`OSM/🔌️plugin/🛠️tool-machine/🦀️.rs:543`); unreachable today (`admit_tool_dispatch` settles aborts before the verb, `OSM/🔌️plugin/🛠️tool-machine/🦀️.rs:393-410`) but a future caller would publish an abort's config lanes. Fix: Abort arm clears both lanes and returns `false`. Owner: S3-CONTROLS.

C4 minor — `emit.description = None` ×2 in the glue (`OSM/🔌️plugin/🛠️tool-machine/🦀️.rs:572,588`) and `description` still destructured in `dispatch_emit_inner` (`PLG:27997`): §20.6 deletion of `Emit.description` not done (S3-CLOSURE). Owner: S3-CLOSURE.

Passed: config-lane press design (`hold_config/release_config/abort_config`, `config_closed`, overlays at 4 render seams, retire through stores; abort/freeze/retire = zero trace) matches §20.1; ring/colour/refused-draft wgpu press laws ran green (14/14, ui crate 768/768); energy leaves schema-first with hard bounds + en/de `x-semio-ui` (`🌍️change-site-latitude/🧬️schema/🔣️.json`), `RunPeriod::is_interval` → `target-mismatch`.

## S3-SPATIAL

S1 **major — wgpu has no live gumball consumer; hosts differ.** `OSM/♾️infinite/🌍️world/🦀️.rs` has no `gumballLiveDispatch` (grep: only `WorldGumballGesture` commit path); React streams stream/commit/abort via `worldGumballStep` (`World3dHost/🟦️.tsx:2786`). With gen3d now setting `gumballLiveDispatch: true` for shapes (S3-PROCEDURAL S3.3), wgpu ignores the flag (SPATIAL S3.4 says so) and keeps the local rigid preview: downstream operators do not follow the drag on wgpu (not exercised live). SPATIAL S3.4 only "designed" N9. Fix: implement (publish `phase:"stream"` per update_step, `commit` tail, skip local preview while live, `captureLost` abort on Close) against the SAME fixture. Owner: S3-SPATIAL + S3-W2C.

S2 **major — shared corpus has one guest consumer.** `🛠️gumball-live-protocol.json` is read by React (engine-contract) and fem 3d (`PL/🏗️fem/…/🧊️3d/…/🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs:133-139`) only; generation3d (the stated live consumer) and wgpu have none. Fix: gen3d law decoding every dispatch through `command_from_action` (swap `ids` for one movable target), wgpu host law driving `update_step`. Owner: S3-PROCEDURAL / S3-SPATIAL.

S3 minor — fem results playback clock: `…/◻️2d/…/📊️results/🫧️transient/🦀️.rs` and the `🧊️3d` twin are 170 lines each, differing in 8 lines after `Fem2d↔Fem3d` renaming (same for `⏱️set-playback-clock/🦀️.rs`, 4 lines). Fix: one `FemResultsAnimation`/clock in the shared fem engine crate, both editors register it. Owner: S3-SPATIAL. Same pattern: `FemGumballTransient` vs lowpoly paint drive (S2.6).

S4 minor — gen3d component-gumball refusals and `Err(String)` are English-only (`G3/✏️editor/🎮️commands/🧭️transforms/🦀️.rs:451,452,456,458,461`); localize (AGENTS: en+de, no default). Owner: S3-PROCEDURAL.

Passed: fem/lowpoly/shooting have no amend/coalesce (0 hits); `SetCamera` one config edit per settled gesture; N12 colour = sRGB `[f32;3]` with schema bounds; `selection_reference_id` hook with mounted law; playback press = one config edit, cancel = none, never a history row (law written for both fem crates).

## S3-STROKES

K1 **major — §15 import re-decodes up to 15 stored frames on every tick.** `PL/📸️remodel/🗿️artifacts/📸️remodeling/SUB/✏️editor/🎮️commands/📼️import-video-frame-payload/🦀️.rs:87-107,213` (`rebuild_video_import_scratch` runs a `BoundedStillDecoder` per recent frame, then `local_sharpness_score`) — 200 frames ⇒ ~3000 decodes, each inside a dispatch. The window transient created for this very import (`RemodelingImport`, `✏️editor/🫧️transient/🦀️.rs`) can hold the ≤15-score rolling window. Fix: add `rolling_scores: Vec<f32>` to `RemodelingImport`, delete the rebuild. Owner: S3-STROKES.

K2 minor — tick `0` starts a fresh `RemodelingImport::default()` even when `window.import` is `Some` (`:201-203`), leaving the earlier stream's open transaction un-aborted. Fix: tick 0 over a live import aborts it first (`Emit::abort_transaction`) or is refused. Owner: S3-STROKES.

K3 minor — dead scaffolding left: seven empty `//#region`/`//#endregion` pairs (`:111-131`); window-less refusal is a bare code string `"remodeling-import-window-required"` (not localized/typed); ~220 lines of per-plugin window-transient boilerplate (`✏️editor/🫧️transient/🦀️.rs`: mutation, JSON, pack, retire) that flow, fem, remodel, cad, lowpoly each re-write — a derive/macro in the framework would remove it. Owner: S3-STROKES / S3-W2A.

K4 **major — wgpu has no bucket/wand; no gesture replay laws for fill-region/§15.** Report: wgpu raster surface (`FW/🗺️surface/🎨️paint`) has brush/eraser only; React `Paint2dHost` dispatches `fillRegion`. Raster/remodel carry only the generic `history_edit_acceptance_law!` (`PL/🖨️raster/…/🧪️tests/🔬️unit/🦀️.rs:44`, `PL/📸️remodel/…:45`) — one representative leaf, not the gesture leaf (brief item 5). `🪣️fill-region` has no fixture quintet yet (`unwitnessed`, emission "pending compile"). Fix: wgpu bucket on the guest verb; a time-travel law editing `seed`/`tolerance`/`color` of a committed `fill-region` against a fresh fold; emit the 8 quintets. Owner: S3-STROKES + S3-W2C.

K5 minor — raster §17.2 "preview from window transient while streaming" not done (host-local preview, one `paintStroke` at release in both hosts); hosts never dispatch `importAbort` on a cancelled pick (routed to W2B/W2C). Track, not blocking.

Passed: `fill-region` schema is complete (bounds, `reference`/`segmented`/`slider`/`color` widgets, en+de, `x-semio-invariant ordered-selection-runs`; codes `mutation.apply.*` are in the vocabulary), one Rust flood engine + TS twin + independent Python BFS corpus; wfc rows only through the shared decoder with the fixture law; G7 labels 30 → 0; process3d cursor = one plain config write per seek; §15 faults (stale tick after abort, retired window) fixed with laws written.

## S3-TEXT

T1 **major — trinity lost a gesture.** Adding a node on the working graph has no path any more (TEXT S3.4.2: the shared rows carry no add-node; `addWidget` is flow's). That is a feature regression dressed as a product decision. Fix: a guest verb `addWorkingNode {x,y,…}` (math's `addNode` precedent) with a relative `add-working-node` leaf, or record the removal in the design. Owner: S3-TEXT + coordinator.

T2 minor — `connect-working-ports` / `disconnect-working-edges` inverse is ONE `edit-before-fixture` = the whole base working graph JSON (`PL/🔱️trinity/🗿️artifacts/♻️rewriting/SUB/🧬️schema/🧬️mutations/🔌️connect-working/↩️inverse/🦀️.rs:7-13`): a one-wire edit stores O(graph) bytes in every edit (`.ops`, wire). Prefer the exact relative inverse (`disconnect-working-edges {targets}` for a connect, `connect-working-ports` for a cut). Also `source`/`target`/`kind` have no `maxLength` (`🧬️schema/🔣️.json`). Owner: S3-TEXT.

T3 minor — N7 fix (`PLG:27997-28180`, `dispatch_emit_inner`: row logged before `revalidate_interaction_state_after_document_change`) covers the plain and config-only branches; the composed-child group branch returns at `PLG:28106-28108` (`dispatch_emit_group`, then `apply_interaction_writes`) and the retained-finish call at `PLG:31368` — neither is shown to log before revalidation — flow drags (child group) on an app with a `Topology` domain may still double-file. Add a law on the group branch. Owner: S3-TEXT / S3-W2A.

T4 minor — `snapshot_edit_net` publishes `net(snapshot,&next)` as a state-diff (`PL/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:1324`): fine for text, but the four named replace-intent fallbacks (md/html other-schema, binary/deflate) must have a law that they alone emit `set-snapshot`; reports list them, no law is cited. Owner: S3-TEXT / S3-STDIO.

Passed: N8 (shared `hostSignals` corpus, TS + Rust ui-scene + wgpu engine law, Worker `host-page-hidden` message, transport law) is the right shape and TS-verified (71 vitest); writer/vcs/jack/md/html/txt/binary/deflate payload+input lints 0; the `"Edit document details"` literal is gone (0 hits).

## S3-PROCEDURAL

P1 **major — §19.4 not implemented.** `G3/✏️editor/🎮️commands/🧭️transforms/🦀️.rs:478-479` (`ensure_component_node`) still sets `mode`/`selection`/`pivot` inside the spliced operator's params; the decision (status 19:35, mis-numbered under §20) requires default params + one appended `change-widget-input` per user-set channel so every channel is individually history-editable. Open item 2 in the report. Fix: splice with defaults, append the three input leaves in the same transaction (reuse check reads from the leaves' effect, not params). Owner: S3-PROCEDURAL.

P2 **major — `change-widget-input` accepts channels that do not exist.** `G3/🧬️schema/🧬️mutations/🎛️change-widget-input/🦀️.rs:140-165`: for a `Neuron` with no stored param `channel`, `current` is `None` and the leaf merges an arbitrary string into `params` (`generation3d_with_params`, `🧬️mutations/🦀️.rs:348`); `NoSuchInput` is only reachable for non-neurons. The schema types `channel` as free `text`, so a history edit of the channel field creates a phantom param; P8 validates against the declared port, the replayed leaf does not. Fix: validate `channel` against the operator's declared inputs (flow neuron kind info) in `landing`, refuse `target-mismatch`; offer the declared names as the option list. Owner: S3-PROCEDURAL.

P3 minor — no hard bounds on `number`/`point`/`vector` values (`schema oneOf[0].value`: only `type: number`) although §19.2 says "hard bounds per variant"; `admissible()` is `is_finite` only (`🎛️change-widget-input/🦀️.rs:40`, payload `:132`). Take bounds from the operator param descriptor (clamped warning per vocabulary) or document why none apply. The text label embeds the whole text value (`:80`, up to 1 MiB per history row): truncate. Owner: S3-PROCEDURAL.

P4 minor — P9 mesh edits are one-shot plain edits without `TransactionRef` (report open 4): row reads `create-widget … (+N)` not the intent; give them a tool id + `tool_intent_kinds`. Gesture-first-grab label law exists for transforms only. Owner: S3-PROCEDURAL.

P5 minor — live-only check open: React must not re-anchor the gumball on the moving answer for shapes (no `gumballTarget`); needs the S2 corpus law for gen3d (see S2).

Passed: `tool_intent_kinds` adopter + law `every_gumball_tool_declares_the_leaf_it_yields_as_its_intent`; leaf is schema-first with discriminated root union, en/de per variant, TS twin + semantic-wire corpus (58 checks) + payload lint 55/55; outcome codes in vocabulary; preview overlay law `an_open_gesture_previews_exactly_what_its_release_commits`.

## S3-LAYOUT

L1 minor — wgpu path painting keeps declared approximations vs React (`render_canvas_scene_node` doc: group opacity per piece, rotated images/text axis-aligned, ≥1 px strokes, raster quads in a later pass). Acceptable; pin each in the corpus as `approximate: true` cases so the parity gap is a tested statement, not prose. `gl-matrix` is imported by React suites without a declared devDependency (dependency-truth, S3-GATES). `verify layout-frame-selection` still has no `📋️project.json` target and the two Python corpus scripts no launch rows (coordinator list) — a law with no target is not a law. Owner: S3-LAYOUT / coordinator.

L2 minor — `🟦️GumballOverlay.tsx` home blocked by taxonomy (`🧭️gumball` only a command member); leave tracked under TAX.

Passed: shared schema + 16-case/68-probe corpus authored by an independent numpy implementation, Rust twin and React both checked (React vs Three.js `SVGLoader` third-party), negative control (4 flipped probes ⇒ 4 failures), standalone proof harness 6/6, 0 `//` comments inside definitions in `📐️Canvas2dHost/🎨️paint/🦀️.rs`; frame chips read kind+content+page in every locale with a law.

## S3-DRAW / NOTE

D1 minor — all Rust of this WP is unverified (X1); `{:?}`-of-`Option` labels fixed with the law `document_setting_labels_read_the_value_not_a_debug_option`; the five leaf oracle tests moved to aggregate open-pattern cases (clean). Two pre-existing draw TS `sharp` reds and four older leaf `🔬️unit` taxonomy findings are not this WP. No finding beyond running the owed suites.

Passed: 10 hand-labelled `Emit::commit` removed (0 left in draw/note), `coalesce_key`/`amend` 0, references declare `strokes/stroke` and `blocks/block` domains, chip laws written.

## S3-GRAPHS

G1 **major — no gesture-specific replay law for the drag leaves.** wires `move-nodes`, trinity `drag-working-nodes`, sequence/math/space drag leaves only have the generic `history_edit_acceptance_law!` (`PL/💡️reasoning/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:67`, sequence `:79`, math `:30`, trinity `:19`); the brief requires editing the leaf's offset through time travel and comparing the head to a fresh fold (flow has it: `editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows`). Fix: one parametrised law in `TM`/plugin test-support fed by each guest's drag record. Owner: S3-GRAPHS.

G2 minor — X2 duplication (12 drag-emit wrappers); wires additionally keeps a bespoke `WiresWindowDragWork` with hand-rolled cursors (`W/✏️editor/🦀️.rs:~300-330`) next to the shared machine: acceptable because `canvasPointer*` verbs are guest-run, but the commit must go through the shared helper.

G3 minor — sequence parent preparation declares `work_items: 1` for every `SequenceMutation` while `DeleteStep` folds 2×steps+edges rows (GRAPHS S3.3): either delete the dead parent lane or declare via `x-semio-inverse-rows` (§20.5). Owner: S3-GRAPHS / S3-CLOSURE.

G4 minor — `NODE_GRAPH_EDIT_ROOT_FIELDS` leaves `gesture/commit/abort` opaque in the schema (`gesture: true`) while the Rust decoder ignores them; tighten the schema to the scrub types (`string`/`boolean`/enum) so Ajv and Rust agree on the whole root. Owner: S3-GRAPHS.

Passed: shared decoder + schema + fixture (8 accepted / 14 refused, closed root, ≤ 256 rows) with Rust (3 laws), TS (Ajv strict) and every guest reading the same fixture; `setHostSnapshot`/ambient `deleteSelection` deleted everywhere; mutate-dag-1 completed for three gesture leaves; wires `move-nodes` is one relative leaf with exact absolute inverse (`set-node-positions`, `unique-node-ids`), zero offset = zero trace, outcome codes in vocabulary.

## S3-PUZZLE

Z1 **major — O(document) publication regression open.** `one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` (puzzle 3d) red: 22 vs 1590 publication units per translate (concrete forest vs Nakagin). A gumball release is one transaction, so a 180-object document costs 70× more — AGENTS performance. Routed to S3-W1G (bisect with `[DEBUG]` probes, X4). Do not close with it red. Owner: S3-W1G + S3-PUZZLE.

Z2 **major — 5d and 2d law runs never executed.** Only 3d ran (899 pass / 9 red); 5d blocked by the peer registry split; `every_registered_example_builds_its_document` and the N3 chip/highlight laws for 5d are unverified, and the 3d suite still has an open red (`an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`) plus four wall-clock laws (8 ms/2 ms ceilings) that fail under load by construction. Fix: run 5d/2d; convert the wall-clock laws to work-unit budgets (deterministic). Owner: S3-PUZZLE.

Z3 minor — `every_registered_example_builds_its_document` is copied three times (`P2|P3|P5/🧪️tests/🧪️every-example/🦀️.rs`) and gated by `--features component-app-assembly` (silently skipped by a default `cargo test`); the DSL brace break hit 7 puzzle fixtures and 12 more plugin assets remain bare (N13). Fix: one generic law over every registered plugin's `examples()` (G12 harness) run with the feature in the nx target. Owner: S3-INFRA / S3-AGNOSTIC.

Z4 minor — puzzle gestures send one pose delta on release (no `gumballLiveDispatch`), so there is no stream phase and the abort path is trivially zero-trace; OK by design (protocol 1), but record it in the corpus as a `local one-shot` host case for puzzle so React/wgpu parity is checked there too. Owner: S3-SPATIAL.

Passed: relative `drag/rotate/scale-selection(3d)` leaves with exact absolute inverses, deterministic fastener/attraction ids (no `DefaultHasher`), `partial`/`target-missing`/`no-op`/`invariant` per vocabulary, bracket verbs deleted (negative law kept), payload/input lints 0 for 3d/5d, one `Puzzle3dSelectionMotion` shared by 5d.

## Fix order (coordinator)

1. X1 owed runs after TREE GREEN; then re-audit F1/C2/Z1 (data-loss / abort / perf).
2. Majors by owner: S3-FLOWCAD F1 F2; S3-CONTROLS C1 C2; S3-SPATIAL S1 S2; S3-STROKES K1 K4; S3-TEXT T1; S3-PROCEDURAL P1 P2; S3-GRAPHS G1 + X2; S3-PUZZLE Z1 Z2.
3. Minors batch into S3-GATES (X3, F8, L1, K3) and closure.
