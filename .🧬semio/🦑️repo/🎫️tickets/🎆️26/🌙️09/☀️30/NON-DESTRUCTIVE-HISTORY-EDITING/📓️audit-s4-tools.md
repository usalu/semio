# S4-AUDIT-TOOLS: read-only audit of the session-4 plugin-side changes (2026-10-04, 12:2x-12:4x)

Auditor: S4-AUDIT-TOOLS (Sonnet, read-only; no cargo/bun/nx run; nothing edited except this report). Evidence is the tree on disk plus the
`## Session 4` report sections. Almost all session-4 plugin work is "source-complete, compile/test OWED" (rules 41-44, cargo freeze), so every
verdict below is a static-read verdict; where a claim was only a report line it is marked (report). Untracked/staged churn is still moving
(mathematical was being rewritten at 12:27-12:31: 60 files). Aliases: `PL` = `✏️s/🔌️plugins`; `SUB` = `🏅️standards/🔖️1/🪆️subsets/✳️any`;
`PLG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`; `TM` = `🧰️framework/🔨️modules/🛠️tool-machine`;
`GRAPH` = `PL/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations`. Core-side findings are in
`📓️audit-s4-core.md` (F1-F19) and are not repeated.

## 0. Verdicts

| # | Area | Verdict | One line |
|---|---|---|---|
| 1a | §20.15 wires, dag, imperative/procedure, sequence | accept-with-changes | Parent vocabularies are uninhabited, board/content edits are child-lane graph/flow leaves, reload law present; missing child-history law (wires), a panicking reader (sequence), dead diff surface, no reload-after-child-edit law (F6, F7, F13, F22). |
| 1b | §20.15 flow | accept-with-changes | Wave is source-complete and coherent (10 leaves, codecs, `flow_content_edit` deleted) but compose-on-read still rides `FlowWorkingScene`/`local_owner` and genesis reads it (F4); compile unverified. |
| 1c | §20.15 playbook | accept-with-changes | 8 parent leaves gone, `change-title` only, reload law present; block edits are ONE absolute `blocksJson` string set per press = the §19.3 masking anti-pattern (F2). |
| 1d | §20.15 din18599 | accept | Model (a) as approved: parent-owned `climate`, derived `climateTable` child, reload law + content-id fixture; whole-record `update-*` leaves are inherited (F20). |
| 1e | §20.15 mathematical, jack/rewriting, cad | reject (not on disk / in flight) | Parent leaves still read working scenes: 16 + 8 + 6 + 8 leaves (F1). |
| 2 | Shared graph vocabulary (drag-nodes, node/edge property trio, resize/rename, `at`) | accept-with-changes | Schemas, keyed removes (F15 fixed) and exact inverses hold; `delete-node` footprint under-declared (F3), annotation gaps (F18), trinity still duplicates 6 leaves (in F1). |
| 3 | Tool machines (one transaction per gesture, shared runners) | accept-with-changes | `drive_gesture` + node-drag helpers reused by fem/lowpoly/flow/dag/wires/sequence; three plugin-local copies remain (F5), no corpus for `drive_gesture` (F8). |
| 4 | Fault notices, `app.command.tool-mismatch` | accept-with-changes | draw/note/fem/puzzle/dag tables are en+de and complete; 21 raw `*-tool-mismatch` codes remain (F9), load refusals are `plugin.internal` (F16). |
| 5a | stdio D3 (exact multi-part inverse > 1 MiB) | accept | Planner, `continued` splice, 1 MiB +/- 1 and 16 MiB laws, fast-json-patch/json_patch replay of every part (report 17/0, TS 43/0). |
| 5b | stdio D4 (oracle + feature row per patch leaf) | accept-with-changes | 47 of 55 patch leaves have a row; 8 have none (F10). |
| 5c | png dead-file cleanup | accept | 197 files deleted (unstaged), aggregate = 5 leaves, no code reference to the 12 dead leaves; only generator doc comments are stale (F19). |
| 6 | LOAD W-b, composed export carrier, `pk:` run-edge carrier | accept-with-changes | Child heads, MCP deps, guest key acceptance, carrier laws exist (OWED); carrier encode is unstepped (F11), two carriers disagree (F12), MCP child read is unconditional and unpinned (F17). |
| 7 | AGENTS.md compliance | accept-with-changes | No inline comments or compat shims in the new graph/tool-machine code; docstring-emoji reuse, `[DEBUG]` left in puzzle tests, legacy-wire decode laws (F14, F15, F21). |

## 1. Findings

Severity: critical = wrong result / goal clause unmet today; major = design contract or goal clause not met, or hazard on the critical path;
minor = hardening, consistency, convention.

### Critical

**F1. critical (known, in flight). Four composed plugins still fold parent-lane leaves over an ephemeral working scene (38 leaves).**
Evidence: mathematical has the parent-owned `graph`/`geometry` snapshot fields (model a, `EquationSnapshot`) but the leaf set is unchanged: under `PL/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/`
the dirs `📐️geometry/🧬️schema/🧬️mutations/{🎯️move-point,➕️insert-point,➖️remove-point,🔄️replace}` and `🕸️graph/🧬️schema/🧬️mutations/{🔗️connect-nodes,❌️delete-node,…}` still exist, there is
no `move-points`/`set-point-positions` leaf (grep: 0), no `composed_reload_law!` (grep: 0), and `✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:511-521` still calls the deleted
`equation_scene_owner`; the oracle rationale `📐️equation/🏅️standards/🔖️1/🪆️subsets/➗️equation/🔮️oracles/🔣️.json:39` still describes the `local_owner` carrier. jack: 9 parent leaf dirs
(`PL/🔱️trinity/🗿️artifacts/🔌️jack/SUB/🧬️schema/🧬️mutations/{create-node,create-edge,delete-node,delete-edge,rename-node,move-node,change-data-property,remove-data-property,set-query}`)
and 53 `jack_working_scene`/`JackWorkingScene`/`jack_content_for_handle` references; rewriting: six working leaves (`drag-/patch-/delete-/connect-/disconnect-/add-working`) read the child
(`parentLeafReadsChild` 6 NEW per S4-TEXT); cad: `PL/📐️cad/🗿️artifacts/📐️cad/🦀️.rs:171` `child.local_owner::<CadWorkingScene>().or_else(…bundled…)`, no reload law. A decoded, reloaded
or remote parent folds these leaves on an EMPTY scene, so replay (and Report-mode replay of a superseded op) is wrong today for these four plugins. Trinity also still duplicates 6 graph leaves
(create/delete node+edge, rename, move) beside `s.stdio.semio@v1/graph`, which wires and dag reuse (item 2).
Fix: finish math (relative `move-points` + absolute `set-point-positions` inverse, delete `move-point`, stale test/oracle text), convert jack/rewriting onto graph leaves after the peer is quiet, cad after flow compiles; each with `composed_reload_law!`
and `composed_child_history_law!`. Owner: S4-WIRES-MATH (math), S4-TEXT + trinity peer (jack/rewriting), S4-FLOWCAD (cad).

### Major

**F2. major. Playbook block edits are one absolute whole-list string per press (design §19.3 / §17.1 anti-pattern).**
`PL/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:244-246` (`playbook_blocks_leaf` = flow `set-node-param {id, key: "blocksJson", value: String}`), `:274-312`
(`playbook_edit_blocks_leaves`, `…add/remove/move_block_leaves`; a cross-step move yields two whole-list sets). Editing an earlier add/field step in history is masked by every later whole-list set
(the later op replays its absolute list and re-introduces or drops the edited block), and the time-travel editor gets one JSON text field because the flow `set-node-param` schema has no `x-semio-ui`
(properties `id,key,value` are bare strings). The S3.12 design chose this ("absolute per step"); it contradicts §19.3 (static records get field-granular leaves) and §17.1 (forms `change-block-field`).
Fix: field/intent leaves on the flow child (`insert-block {step, at, block}`, `remove-block {step, block}`, `move-block`, `change-block-field {step, block, field, value}`) with typed `x-semio-ui`, or a typed
`blocks` child; keep `blocksJson` out of the leaf vocabulary. Owner: S4-TOOLS-B.

**F3. major. `delete-node` footprint is under-declared above 1024 incident edges, and nothing refuses it.**
`GRAPH/🗑️delete-node/🧬️schema/🔣️.json:5` `x-semio-inverse-rows: {bounded: 1025}`; `…/🔺️diff/🦀️.rs` applies for any degree (it only counts `severed`), `…/↩️inverse/🦀️.rs:10-17` returns `1 + degree` rows.
`ArtifactStoreOneItemFootprint::for_leaf` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16411`) trusts the declaration (`inverse_rows()+1`), so a 2000-edge node is admitted as 1026 items and yields 2001 inverse rows: the
exact hazard class §20.5 exists to remove (a `perTarget` leaf gets the cap+1 law, a `bounded` one does not). Consumers: wires `delete-selection`, dag `delete-selection`.
Fix: diff refuses degree > 1024 with `mutation.too-large` (localized) or declare `perTarget`-style rows from the degree; add the cap+1 property test for every `bounded` leaf (also the 55 patch leaves at `bounded: 128`). Owner: S4-GRAPHS + S4-GATES.

**F4. major. Flow compose-on-read still depends on a materialized `FlowWorkingScene`; flow's default document is not self-describing (D24).**
`PL/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:173` (`flow_genesis_content_pack` reads `document.content.local_owner::<FlowWorkingScene>()`), `:198-203` (`flow_composed_snapshot` falls back to the owner when the child is not composed and always re-attaches one),
`SUB/✏️editor/🦀️.rs:2549,2567` (`flow_scene_publication` / `flow_scene_replacement` diff against the owner; `Fault::from("flow-edit-scene-owner-missing")`, an anonymous code). A flow parent decoded from pack/text without archive members has
no owner, so `genesis_child_pack` answers `None` and every child-lane verb refuses; §20.15 states a decoded parent needs no materialization step. D24 and the S3.6 owed runs are not recorded in `📓️w3-t-flow-cad-report.md` S4 (it ends at the S4.2 deletion
wave; "cargo check running" when cut), so the flow crate compiles only per report. Also flow has 0 `fault_notices()` for 32 named + 90 anonymous faults (F9).
Fix: genesis from a plugin catalogue keyed by stable child id (playbook precedent: `playbook-flow`/`playbook-demo-flow`), read the child through `ChildContentView` only, drop `FlowWorkingScene` as a cache vehicle, name the fault. Owner: S4-FLOWCAD.

**F5. major. Three plugin-local copies of the shared streamed-gesture runner remain (`GesturePhase`/`drive_gesture`).**
`PL/🖨️raster/🗿️artifacts/🖨️raster/SUB/✏️editor/🎮️commands/🖌️paint-stroke/🦀️.rs:120-145` `RasterStrokePhase` (byte-identical to `TM/🦀️.rs:461-480` `GesturePhase`, plus `as_str`) with its own open/resume/abort/moved-base flow
(`:339-400`, `RasterStrokeToolState`); `PL/🀄️wfc/🗿️artifacts/🖼️bitmap/SUB/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🪛️utilities/🖌️brush/🦀️.rs:37` `BitmapStrokePhase`;
`PL/🌀️procedural/🗿️artifacts/🧊️generation3d/SUB/✏️editor/🎮️commands/🧭️transforms/🦀️.rs:249-266` `GumballPhase` + `GumballGesture{runner,verb,ids,base_revision}`. S4-TOOLS-A extracted `drive_gesture` from fem + lowpoly only and listed
gen3d/CAD as follow-up. Design §5/§13 and the brief: no plugin-local duplicates; repeated code must be close.
Fix: implement `GestureTool` for the three tools (raster stroke, bitmap stroke, gen3d gumball), delete the local phase enums and drive bodies; the raster `Once`/`Commit` semantics map onto `GesturePhase::Once|Commit`. Owner: S4-STROKES (raster, bitmap), S4-TOOLS-B (gen3d).

**F6. major. Wires and playbook have no child-lane history law (`composed_child_history_law!`).**
`PL/💡️reasoning/🗿️artifacts/🔌️wires/SUB/✏️editor/🧪️tests/🔬️unit/🦀️.rs:67-68` and `PL/📖️playbook/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:48-49` call `history_edit_acceptance_law!` + `composed_reload_law!` only; dag (`…/🧪️tests/🔬️unit/🦀️.rs:25-26`),
flow, sequence and imperative also call `composed_child_history_law!`. Wires and playbook put every edit in a child store, so "a composed child's mutation is editable end to end" is unproven for exactly the two plugins whose edits are 100 % child-lane.
Fix: add the law with a seeding gesture (wires `addNode`, playbook `addStep`/`addBlock`). Owner: S4-WIRES-MATH (wires), S4-TOOLS-B (playbook).

**F7. major. A reloaded sequence panics in its `topology` inference and in `retire_cold`.**
`PL/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs:300` `sequence_working_scene(snapshot)` = `…for_handle(&snapshot.content).expect("sequence child scene must be materialized before use")`, called by
`SUB/🧬️schema/💡️inferences/🧭topology/🦀️.rs:33` (`compute_sequence_topology`, documented "never panics, never fails"), reached through `SequenceInference::infer` (`🧬️schema/💡️inferences/🦀️.rs:25-29`, `reads: ["content"]`). A decoded/reloaded/remote
parent carries no owner, so the inference traps the guest; `SUB/🧬️schema/📸️snapshot/🦀️.rs:94` `.expect("exact sequence scene owner")` is a second panic on the cold-retire path. The S4-LOAD `child:<slot>/<id>` dependencies exist but this inference ignores them.
Fix: infer from the dependency head pack (flow-subset snapshot) or answer the empty topology; no `expect` on materialization anywhere. Owner: S4-GRAPHS.

**F8. major. `drive_gesture` has no language-agnostic corpus or third-party oracle.**
`TM/🦀️.rs:509-540` is the single control flow every streamed tool now relies on (abort = zero trace, base moved, verb switch, one-shot interruption); its only coverage is 3 Rust unit laws (`TM/🧪️tests/🔬️unit/🦀️.rs:1219-1224…`). `TM/🧫️fixtures/` has `node-drag-law`, `scrub-law`,
`transaction-law`, `typing-law`, `node-graph-edit-rows` but no gesture-drive transition table; `TM/🟦️.ts` has no twin (`GesturePhase|drive_gesture` = 0 hits). AGENTS: one language-agnostic test per feature, validated against a third-party library.
Fix: fixture `🧫️gesture-drive-law/🔣️.json` (persisted gesture x phase x base revision -> committed/next, 20+ rows) read by Rust and an xstate/fast-check TS twin of the transition table. Owner: S4-TOOLS-A.

**F9. major. `app.command.tool-mismatch` adoption is partial: 21 raw per-plugin codes remain, all inside the `history-editing` gate scope (`tool` segment).**
Raw `Fault::from("<app>-command-tool-mismatch")`: writer `✒️writer/…/✏️editor/🦀️.rs:1280`; equation `…/✏️editor/🦀️.rs:1129`; wfc grid2d `:718` (+ viewer `👁️viewer/🦀️.rs:305`), bitmap `:856`, grid3d `:666` (`wfc.grid3d.retained.tool-mismatch`); vcs `:915`; animate `:994`; playground `:392`;
fem 2d `…/🌐️any/✏️editor/🦀️.rs:806`, fem 3d `:860`; process3d `:567`, `:1413`; lowpoly `:1852`; wires `…/SUB/✏️editor/🦀️.rs:624`; layout `:1073`, `:1344`; cad `:2093`; norm contract `📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:2245`; jack `:827`, `:860`
(`jack-retained-document-tool-mismatch-or-capacity`); playbook module `📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:984` (`playbook.module.procedural.tool-mismatch`, not a framework code). Converted (grep of `app.command.tool-mismatch`): note, shooting, wfc 2d/3d, gen2d/3d (+ gen3d viewer), flow, sequence, gis x3, forms, remodel, energy x2, procedure, playbook editor; draw has none left. (21 raw codes listed, plus the playbook module code.)
Also raster/wfc/remodel/process have 0 `fault_notices()` while emitting tool-flow codes (`raster-stroke-phase-invalid`, `raster-stroke-window-required`, `wfc-…`). Fix: replace each by `FaultCode::new("app.command.tool-mismatch")`; add `fault_notices()` (en+de) to raster/wfc/remodel/process/flow/sequence. Owner: each plugin owner (TOOLS-A: fem/lowpoly/layout, STROKES: wfc/process/raster, WIRES-MATH: wires/equation, FLOWCAD: cad, TEXT: jack/writer/vcs).

**F10. major. D4: 8 of 55 `patch-snapshot` leaves have no `patch-snapshot` feature row (hence no oracle arm to execute).**
Leaf set (55 `patch-snapshot/🧬️schema/🔣️.json`) minus feature files containing `patch-snapshot` (48): `📕️xlsx 🧱️base`, `📜️docx 🧱️base`, `📰️xml 🧱️base`, `🧿️semio ✉️base`, `🧿️semio 📽️presentation`, `🧿️semio 🔺️mesh`, `🧿️semio 🖊️drawing`,
`🧿️semio 🖼️image` (and `🧊️gltf ♾️any` has a row but no patch leaf: check naming). The 35 Rust-oracle subsets and 14 Python semio arms are real (own RFC 6901 twin `patched_snapshot`, report 16/0), but the `test parity exhaustive` runs that execute them are OWED (rule 43).
Fix: rows + arms for the 8 (svg base is reported pending too); run the exhaustive cases. Owner: S4-STDIO.

**F11. major. The whole-document media carrier encodes O(document) in one unstepped call, in the guest, as base64 text.**
`PLG/🦀️.rs:14767-14780` `whole_document_media`: `encode_document_archive_bytes(archive)` then `pack_value_to_base64` (+33 %, one contiguous String) with no progress/cancel; `:24319-24330` `composed_artifact_media` does the same. AGENTS: progress and cancellation for every expensive operation; §20.14 wall budget; the
guest allocator never returns memory (64 KiB contiguous-request ceiling note). The decode side is async but the encode is not stepped. Fix: step the archive encode under the reactor deadline (`yield_once` between members) or move the carrier behind the stepped media-export job, bound the size with a localized refusal. Owner: S4-LOAD.

**F22. major. No law saves a document that already has child-lane history before reloading it (§20.15's own law).**
`PLG/🧪️tests/🧪️history-edit-acceptance/🦀️.rs:786-855` (`assert_documents_reload_identically`, behind `composed_reload_law!`) reloads only the SHIPPED documents (initial + examples, loaded as text, unedited) and runs the history-edit search AFTER the reload;
`:950-1060` (`assert_child_history_edits_end_to_end`, behind `composed_child_history_law!`) edits a child mutation but never saves and reloads the edited document (no `acceptance_reloaded` call in that range, unlike the parent-lane scenarios at `:486,:530,:564`).
So "child-lane edit -> overwrite/alternative -> save -> fresh load folds identically" is unproven for every composed plugin, which is the failure mode §20.15 exists to prevent (member `.spr` with edits, stable parent coordinate, derived children).
Fix: after the seeded child edit and after each finalize, save, reload, and compare head, members and window renders (reuse `acceptance_view_difference`). Owner: S4-AGNOSTIC.

### Minor

**F12. minor. Two carriers for `artifact:out` disagree.** `produce_media("artifact:out")` answers the FULL recursive archive (`PLG/🦀️.rs:35880`, history included); `export_media("artifact:out")` of a composed document answers `composed_artifact_media` (`:24328` `parent_spr: Vec::new()`), which
`begin_document_archive_load` refuses (`:35371` "parent pack and SPR must both be present"), and `export_media` now overrides an app's own `artifact:out` export for every composed document (`:35931`). Fix: one carrier, or mark the export carrier serialize-only and keep it off the `Document` wire. Owner: S4-LOAD.

**F13. minor. Dead diff surface for six uninhabited parents.** flow `🧬️schema/🔺️diff` 19 files (json/proto/graphql/ts + `💾️binary`/`📝️text`), imperative 19, dag/sequence/wires 5 each; `FlowStringList` (`PL/🌊️flow/…/SUB/🧬️schema/🔺️diff/🦀️.rs:32`) has no user. A parent that can never diff needs no diff schema/codecs.
Also `PL/🎬️sequence/…/SUB/🔮️oracles/🔣️.json`: `csv-rfc4180-reader` (capability `sequence-1-mutate`) and decision `sequence-step-graph-mutation-semantics` still name the deleted parent leaves (`create-step`, `move-step`, `connect-steps`, …). Owner: S4-FLOWCAD / S4-GRAPHS.

**F14. minor. Docstring emoji reuse (design §21.2, per touched file).** Added docstring blocks sharing a first emoji: `TM/🦀️.rs` (⏱ x5 `:403,990,1140,1272,1394`, 🛠 x4 `:1,976,996,1015`), playbook root (6 emojis / 15 blocks, e.g. 🔗 `:61,88,224`, 🧱 `:77,243,274`),
wires root (9 / 27, e.g. 📭 `:55,60,65,70`), `PLG/⏯️tool-run/🦀️.rs` (10 / 50), stdio `🩹️patch/🦀️.rs` (14 / 34, e.g. 📍 `:75,249,993`). Flow root and flow editor are clean. Owner: S4-GATES + owners.

**F15. minor. Temporary `[DEBUG]` logs and provisional bounds left by S4-PUZZLE.** `PL/🧩️puzzle/🗿️artifacts/🧊️3d/SUB/✏️editor/⏳️precompute/{📐️geometry:749,🖌️brush:932,🪣️fill:418,1166}/🧪️tests/🔬️unit/🦀️.rs`; `stepWorkCeiling` is a provisional 100 000 (report S4.6). Other `[DEBUG]` hits
(flow brep tests 34, draw 87, procedural 33, remodel 4, raster 2, gismap 2, playground 4, store space-history 6, plugin 7) are test narration of earlier waves, not S4-new; they violate "temporary logs removed" the same way. Fix: pin the ceilings after one measured run, delete the prints.

**F16. minor. Load/media refusals are anonymous.** `PLG/🦀️.rs:35359-35390` `begin_document_archive_load` raises 7 refusals through `plugin_sdk_fault` = `plugin.internal` (`:662`), which no framework table labels (grep in `🎠️kernel/🦀️.rs` and both notice fixtures: 0); only `plugin.media.schema-mismatch` (`:14762`) is named + localized. The gate scope is a token heuristic (`FAULT_NOTICE_TOOL_FLOW_CODE`/`_PATH` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts:1446-1460`) and misses tool refusals whose
code and path lack the tokens, e.g. wires `wires-drag-transient-invalid`, `wires-drag-offset-non-finite`, flow `flow-retained-direct-route-mismatch`, sequence `sequence-node-graph-route`.
Fix: label the load refusals and widen the gate (any `Fault::from` in a file that publishes `ChildEmit`/`Emit::*drag*`). Owner: S4-LOAD / S4-GATES.

**F17. minor. MCP `infer_real` reads child heads unconditionally and from a different truth than the parent.** `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1691,1905-1913`: every inference with an `artifact_document` pays `ReadChildHeads` (guest `child_head_packs`, `PLG/🦀️.rs:35341`: whole head pack per member, sequential `await`s) whether or not the inference declares a child dependency,
and the children come from the live session guest while the parent is the caller-supplied document (no revision/child-id check ties them; content-addressed child ids in the parent handle may not equal the dependency keys). Fix: read only declared `dependsOn`/`reads` slots and compare child ids to the parent's coordinates. Owner: S4-LOAD.

**F18. minor. Graph leaf annotation and style gaps.** `GRAPH/➕add-node-property/🧬️schema/🔣️.json`: `node_id` and `index` carry no `x-semio-ui` (its edge twin annotates all three; no `role: target` ref, so no "use selection"); `📍move-node`, `🗑️delete-node`, `✂️delete-edge`, `🖍️change-node-label`, `🔧change-node-kind`, `🔌add-node-port`, `🔚remove-node-port`
leave `id`/`node_id` bare (inherited; only `drag-nodes.targets`, `rename-node`, `resize-node` and the trio carry the reference descriptor). Payload keys mix `node_id`/`new_id` (snake) and `sourcePort` (camel). `➕add-node-property/🔺️diff/🦀️.rs` has a `// 🚫️async` line comment instead of a docstring and an `.expect("checked above")` on a user-reachable path.
All graph diffs clone the whole node/edge list (`base.nodes.clone()`), O(graph) per leaf and per Report-mode replay step (§20.14, inherited). Owner: S4-GRAPHS.

**F19. minor. png: stale generator provenance.** `PL/🗄️stdio/🗿️artifacts/📷️png/SUB/🏭️generator/🔁️codec/🦀️.rs:261-385` still documents `ChangeHeaderMutation … ReplacePixelsMutation` (comments inside the `match`, and deleted-leaf names); the `*-applied` fixture dirs stay because `🔮️oracles/🔣️.json` and
`🧪️tests/🔀️mutate-png-1-2/🥒️.feature` still use them as codec vectors (not dead). `git ls-files -d` = 197 unstaged deletions: stage them with the rest.

**F20. minor (inherited). Whole-record leaves persist in din18599.** `PL/📕️norm/🗿️artifacts/⚡️din18599/SUB/🧬️schema/🧬️mutations/{update-lighting,update-cooling,update-ventilation,update-renewables,update-climate,replace-zones,replace-elements}`:
the §19.3 masking class (energy got field-granular leaves; norm did not). Not an S4 regression; record for the next wave.

**F21. minor. Compatibility/legacy decoding laws and process notes.** "legacy one-per-event" pointer wire laws pin optional-field decoding of the pre-batching `canvasPointerMove/Up` shape in wires (`🪟️windows/🕸️canvas/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:417-436`), draw (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1205-1224`), layout (`🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs:178-181`),
fem 2d (`:337`), vcs (`:535`): an AGENTS "no legacy support" breach (make `samples`/`cancelled` required, delete the laws). `drive_gesture` swallows `ToolRefusal` (`TM/🦀️.rs:511` `resume(..).ok()`, `:530` `start(..).ok()`, `:536` `Ok(_) | Err(_)`): a refused tick is a silent no-op, never a named fault.
Per-plugin node-drag wrappers repeat `Emit::node_drag_child(node_drag_emit(..), slot, child_id)` + `ui_scope: Full` in flow (`🛠️tools/✋️drag/🦀️.rs:36-45`), dag (`🎮️commands/✏️node-graph-edit/🦀️.rs:120`), wires (`✏️editor/🦀️.rs:315-321`), sequence x2 (`🎮️commands/🕸️node-graph/🦀️.rs:86`, `✏️editor/🦀️.rs:1517-1519`), equation (`:125`), rewriting (`:114`): one `Emit::child_node_drag(..)` helper. Hand edit of the gitignored generated
`🗣️writer-languages` codec by sweep script (S4-TEXT S4.4) is harmless (regenerated) but violates "never hand-edit generated files". Report hygiene: `📓️s4-tools-b-report.md` records only the playbook start (no D11/D12/D24/P5/energy/forms/gis record), `📓️s3-wires-report.md` and `📓️s3-math-report.md` S4 "Changes/Verification" are "(in progress)"; the §21.1 member-run laws are still absent (core F4).

## 2. What was verified and holds (no action)

- Wires: `WiresMutation`/`DagMutation`/`SequenceMutation` are uninhabited with clean `match *self {}` bodies; `WiresWorkingScene`, `local_owner`, `materialize_wires_content` have 0 references; add-node/add-relationship/delete-selection/drag publish graph leaves through `wires_child_emit`/`Emit::node_drag_child` (`SUB/✏️editor/🎮️commands/*`, `✏️editor/🦀️.rs:315-321`); Reorganize is a member run (`🛠️tools/🗂️reorganize/🦀️.rs:27-28`, dag `:52`).
- Dag editor verbs map to shared leaves (`patch-dag-nodes` -> `change-node-label` / `set-node-property` / `resize-node`, `rename-dag-node` -> `rename-node`, `delete-selection` -> `remove_nodes_leaves`): no dag-local equivalents. `composed_reload_law!` present in flow, sequence, wires, dag, imperative, playbook, din18599.
- Graph vocabulary: `remove-node-property`/`remove-edge-property` are keyed by `key` and invert to `add-*-property` at the BASE index (F15 of the core audit is fixed); `add-*`/`set-*` inverses mirror the diff conditions (no-op add never detaches a pre-existing entry); `drag-nodes` diff is base-relative with `mutation.partial`/`target-missing`/`no-op`, inverse = absolute `move-node` per node at the BASE position, `x-semio-inverse-rows: {perTarget: {targets: 1}}`; new leaves carry en+de labels (`add-node-property` partly, F18).
- Tool machines: no `Emit::amend`, `AmendLast`, `coalesce_key` or plugin-local scrub machine remains in any plugin (grep 0 each); fem 2d/3d and lowpoly implement `GestureTool` (`fem_gumball_drive`, `lowpoly_paint_drive`); `authoring_clock(0)` mints refs from the admission seed, not the clock.
- Fault notices: draw `drawing_fault_notices()` (9), note (5), fem (8), puzzle 2d/3d/5d (5/5/5), dag, playbook all en + de; framework `app.command.tool-mismatch` row (en/de) in `🎠️kernel/🦀️.rs:2182`; `plugin.media.schema-mismatch` row added with both schemas as parameters.
- stdio: `SNAPSHOT_PATCH_MAX_INVERSE_PARTS = 128`, 1 MiB parts (`🩹️patch/🦀️.rs:20-24`), `continued` defers the whole-snapshot check to the last part, TS twin + `chunkedInverses` fixture (8 cases, 80 parts) replayed with fast-json-patch (report 43/0), 16 MiB text/octet/row laws; refusal `snapshot-edit.inverse-limit` -> `target-mismatch` instead of a silent empty inverse.
- png: aggregate = `SetSnapshot, PatchSnapshot, ChangeGamma, PatchPixels, PaintNativeSamples`; `PngMutation::<other>` references: 0.
- No inline `//` comments added in the graph leaf files, `TM/🦀️.rs`, wires/dag/flow roots (indented `//` count 0; the 4 in playbook `:335-338` are pre-existing); no compat shim, deprecation or migration script found in the S4 plugin diffs; no script files outside `📜️script.ts` and the Python case implementations (`🐍️.py`) the multi-implementation rule prescribes.

## 3. Owner routing

S4-WIRES-MATH: F1 (math), F6 (wires), F9 (wires, equation), F13. S4-TEXT + trinity peer: F1 (jack/rewriting), F9 (jack, writer, vcs). S4-FLOWCAD: F1 (cad), F4, F9 (cad), F13. S4-TOOLS-B: F2, F5 (gen3d), F6 (playbook). S4-GRAPHS: F3, F7, F13, F18. S4-STROKES: F5 (raster, bitmap), F9 (wfc, process, raster).
S4-TOOLS-A: F8, F9 (fem, lowpoly, layout), F21. S4-STDIO: F10, F19. S4-LOAD: F11, F12, F16, F17. S4-GATES: F3 (cap+1 law for `bounded`), F14, F16. S4-AGNOSTIC: F22.
