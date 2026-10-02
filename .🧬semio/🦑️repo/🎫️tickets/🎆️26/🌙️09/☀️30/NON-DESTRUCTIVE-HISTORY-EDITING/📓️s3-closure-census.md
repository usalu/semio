# 📓️ S3 Closure Census — what is left before the CLOSURE deletions

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. READ-ONLY census by S3-CLOSURE-CENSUS (Sonnet), 2026-10-02 11:00–11:40.
Method: `/usr/bin/grep -rnI` over `✏️s`, `🧰️framework`, `🌎️hub` (excl. `node_modules`, `target`, `dist`, `🗑️generated`, `.git`) for
`Emit::amend|amend(|amend_config|AmendLast|coalesce_key|coalesceKey|UtilityPreviewContract|transformBegin|transformEnd|paintStrokeBegin|paintStrokeEnd`;
second pass for tool-machine users (`ToolMachineRunner|ToolYield|ScrubMachine|TypingMachine|NodeDragRecord|node_drag_commit|statechart!`),
whole-snapshot leaves (`SetSnapshot|snapshot_edit_*|ReplaceSnapshot`), footprint declarations
(`for_one_invertible_item|for_one_item|ArtifactStoreOneItemFootprint`), pointer/drag handlers. Source-level only: nothing compiled or run.
Peers (20 S3 agents, load 130) edit concurrently: line numbers are as of 11:22 and drift; anchor on symbols.
Aliases: `PL`=`✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>` (the `🗿️artifacts/` segment is dropped below), `S1`=`🏅️standards/🔖️1/🪆️subsets/✳️any`,
`S1w`=`…/🌐️any`, `ED`=`✏️editor`, `OS`=`🧰️framework/🛍️products/💻️os/🔨️modules`, `FW`=`🧰️framework/🔨️modules`, `HUB`=`🌎️hub/🧩️compositions`.

## 0. Verdict

1. **`Emit::amend(` has ZERO production callers.** The 6 session-2 sites (remodel 3, dag 2, hub space 1) are converted (remodel `Emit::stream_transaction`/`commit_transaction`,
   dag/space `node_drag_commit` + `Emit::commit_transaction`). Remaining `Emit::amend(` text: the definition (`OS/🔌️plugin/🦀️.rs:12869`), 1 runtime test
   (`OS/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:882`), the tool-run policy gate + its planted-violation test.
2. **`AmendLast` has zero plugin-tree callers** (3 comments). Live callers are all in the framework: DOCUMENT lane (`artifact_lane_commands`, deletable now) and two CONFIG lanes
   (doc-config `config:{key}`, window-config `window:{id}:{key}`) that serve the 13 legitimate view/config coalesce sites (§2). `AmendLast` cannot be deleted without a decision on the config lane (D1).
3. **Artifact-lane static `coalesce_key: Some(...)`: 0.** All `Some(` sites are view/config/transient lanes (13) — see §2 V. flow's 4 keys and puzzle 3d `set-active-example` are gone.
4. **Brackets: 0 in source.** `transformBegin/End`, `paintStrokeBegin/End` survive only in 7 stale generated descriptors (3 `OS/🧑‍💻dev/🔌️plugin-modules/*/🔣️.json`, 4 `HUB/*/🔣️.json`; GATES regenerates) and 2 deliberate negative-case artifacts (kept).
5. **`UtilityPreviewContract`: 3 doc lines** (anchor `OS/🔌️plugin/🦀️.rs:13435` + 2 refs from `amend`/`commit` docs) — a rename, not a behaviour change.
6. **Vendored statechart copies: 0 in source** (draw `fsm` crate gone; only gitignored `dist/` leftovers). Plugin-local `machine::statechart!`: 15, all folded by the `🛠️tool-machine` runner (= already tool machines).
7. **One hand-rolled gesture left, UNOWNED:** `💡️reasoning/🔌️wires` node drag (window-transient drag state + absolute `move_node(id,x,y)` on release, no `ToolTransaction`).
8. **Whole-snapshot absolute leaves remain** (not covered by the closure API deletions, but break "every history mutation is editable"): flow content `SetSnapshot` (10 call sites, S3-FLOWCAD),
   stdio `snapshot_edit_set_snapshot` per host event (77 call sites; 4 S3-TEXT, 73 UNOWNED), norm `ReplaceSnapshot` ×15 standards, md/html net-diff fallbacks.
9. **Fold footprint: 80 hand-written declaration sites, 0 derived.** 2 suspect `work_items: 1` on artifact lanes (sequence editor, sourcing curation); shooting/remodel/trinity-rewriting/norm/architect declare nothing (framework flat default).
10. **44+1 hand-rolled `protocol::Edit {… coalesce_key: None …}` literals** across plugins (one per plugin store-preparation region) — the field cannot be deleted mechanically without a framework `next_edit` helper (§4 step 3).

## (a) Per-plugin table — gesture paths converted / remaining

Gesture path = one host-driven continuous input family (pointer drag/stroke/gumball, node-drag record, scrub control family, typing run, streamed ingest).
`SC`=`machine::statechart!` tool machine folded by `ToolMachineRunner`; `ND`=`node_drag_commit` (framework one-shot node-drag machine); `SR`=routed by the framework scrub runtime (`gesture`/`commit` args → `ToolMachineRuntime`, plugin only supplies the leaf); `TY`=`TextWindowKit`/typing runtime; `ST`=`stream_transaction`.
Res = residual closure hits: `E`=Edit literal `coalesce_key: None`, `V`=view/config coalesce site, `T`=test asserting `coalesce_key`, `D`=doc/comment, `G`=stale descriptor, `W`=whole-snapshot leaf.
Foot = footprint sites as `flat2` (`for_one_invertible_item`) / `n` (`for_one_item(n,…)`) / `lit` (struct literal; window-transient `work_items:1` included).

| Plugin / artifact | Owner | Machines on disk | Conv | Rem | Res | Foot (flat2/n/lit) |
|---|---|---|---|---|---|---|
| puzzle 2d | S3-W2D | SC select-tool (`utilities/🖱️select:235`) | 1 | 0 | E2 T2 | 0/0/3 (+n via `work_items` var) |
| puzzle 3d | S3-PUZZLE | SC transform (`utilities/🔄️transform:301`) | 1 | 0 | E2 T5 | 1/3/2 |
| puzzle 5d | S3-PUZZLE | SC transform (`utilities/🔄️transform:251`) | 1 | 0 | E2 T1 + retire `ED:3082` | 1/5/2 |
| block 2d/3d/5d | UNOWNED | none (one-click config) | 0 | 0 | E3 | 2/0/3 |
| draw | S3-DRAW | SC canvas tool (`canvas-pointer-down:600`) | 1 | 0 | E1 T2 V1 (viewer camera) | 0/1/1; 10 `Emit::commit(…, "label")` one-shots |
| note | S3-DRAW | SC ink tool (`ink-apply-events:240`) | 1 | 0 | retained accumulator coalesce ×5 (`🧵️retained:173,179,180,215,358`) | 0/0/1 |
| shooting | S3-SPATIAL | SC gumball (`🧭️gumball:86`) | 1 | 0 | V2 (`amend_config` camera + draft-label keystrokes) | **no declaration** |
| fem 2d | S3-SPATIAL | SC gumball (`🕹️interaction/🧭️gumball:259`) | 1 | 0 | E1 T3 V2 (playback) | 1/1/0 |
| fem 3d | S3-SPATIAL | SC gumball (`…:227`) | 1 | 0 | E1 T3 V3 (playback) | 1/1/1 |
| lowpoly | S3-SPATIAL | SC `LowpolyTool` gumball + paint (`🖌️session:570`) | 2 | 0 | E1 D1 G | 2/0/0 |
| layout | S3-LAYOUT | SC transform (`🔄️transform:197`) | 1 | 0 | T2 | 0/1/1 (only plugin whose `n` comes from an inverse-length fn: `layout_mutation_inverse_rows`) |
| cad | S3-FLOWCAD | SC transform (`🧭️transform:132`) + engagement commit stamped with `TransactionRef` | 2 | 0 | E2 T6 | 4/0/1 |
| flow | S3-FLOWCAD | ND drag + move-media-node + SR widgets (`patch-flow-widgets`) | 3 | 0 | E2 guard `ED:1519` T1 **W10** | 0/1/2 |
| procedural gen2d | S3-PROCEDURAL | ND + move-media-node + SR widgets | 3 | 0 | E2 | local helper `generation2d_one_item_footprint:847` |
| procedural gen3d | S3-PROCEDURAL | SC transforms (`🧭️transforms:176`) + ND + SR | 3 | 0 | E1 | local helper `generation3d_one_item_footprint:445` |
| writer | S3-TEXT | TY (TypingFold/TextSplice) | 1 | 0 | E1 T1 W2 (`setSnapshot`,`setSnapshotJson` commands) | 1/0/1 |
| vcs | S3-TEXT | TY | 1 | 0 | E1 | 1/0/0 |
| trinity jack | S3-TEXT | TY | 1 | 0 | – | 0/0/2 (windows) |
| trinity rewriting | S3-TEXT | ND via host-snapshot diff (`HOST_SNAPSHOT_GESTURE`) | 1 | 0 | T1 | **no declaration** |
| stdio md, html, binary, deflate (txt) | S3-TEXT (txt UNOWNED) | TY via `TextWindowKit` | 5 | 0 | **W6** (4 `snapshot_edit_set_snapshot` + fallbacks md `:236`, html `:231`) | – |
| stdio all other families (73 sites) | UNOWNED | none | – | – | **W73** | semio/zip n=1,1; docx flat |
| raster | S3-STROKES | SC paint-stroke (`🖌️paint-stroke:122`) | 1 | 0 | E2 T1 | 0/0/2 (flat `work_items:2` literals) |
| wfc bitmap | S3-STROKES | SC brush (`🖌️brush:157`) | 1 | 0 | E1 | 0/1/1 (`bitmap_inverse_rows`) |
| wfc 2d / 3d | S3-STROKES | ND `drag-slots` (`🛠️tools/✋️drag`) | 2 | 0 | T2 | – |
| wfc grid2d/3d | S3-STROKES | none (viewer pointers are `HostOnly`) | 0 | 0 | – | 0/0/1 |
| process3d | S3-STROKES | SC world (`🌍️world:54`) | 1 | 0 | E2 T2 **V2** (cursor `⏱️cursor:32`, engagement `🎛️engagement:30`) W1 (`SetDocument`) | 1/0/2 |
| remodel | S3-STROKES | ST (`import-video-frame-payload:235`) + commit | 1 | 0 | – | **no declaration** |
| dag | S3-GRAPHS | ND (`✏️node-graph-edit`) | 1 | 0 | E1 | 0/0/1 |
| sequence | S3-GRAPHS | ND (child leaves, never whole snapshot) | 1 | 0 | E1 | 0/0/2 (**`work_items:1` on artifact lane `ED:1241` suspect**) |
| mathematical equation | S3-GRAPHS | ND | 1 | 0 | E1 | 1/0/0 |
| space (PL core/home + HUB space) | S3-GRAPHS | ND (`HUB/🪐️space/…/✏️node-graph-edit`) | 1 | 0 | E3 | 1/0/2 |
| energy | S3-CONTROLS | SR | scrub | 0 | E1 T1 V3 (viewer camera `👁️viewer:133`, editor camera `ED:1275`) | 0/0/1 |
| forms | S3-CONTROLS | SR + typing | scrub | 0 | E1 **B1** (try-value transient emit with key, `set-try-value:320,470`) | 1/0/1 |
| gis terrain / map | S3-CONTROLS | SR | scrub | 0 | E2 T1 V1 (viewer camera `👁️viewer:103`) | 1/0/1 |
| norm (15 standards) | S3-CONTROLS | SR | scrub | 0 | W (`ReplaceSnapshot` ×15 `set-snapshot` + `set-active-example`; one-shot intents) | – |
| playbook | S3-CONTROLS | SR | scrub | 0 | E1 | 0/0/1 |
| animate presentation | UNOWNED | SR (`patch-tile-crops`); pointers `HostOnly` | scrub | 0 | E1 | 0/0/1 (`lit3`) |
| **reasoning wires** | **UNOWNED** | **none: hand-rolled drag** | 0 | **1** | **B1** (`ED:267-423`, `708-720`; commit absolute `move_node`) | 0/0/1 (window) |
| architect | UNOWNED | none (`nodeGraphEdit` = connect/delete only, `move` ignored) | – | – | – | **no declaration** |
| sourcing curation | UNOWNED | none | – | – | E2 D1 | 0/0/4 (**`work_items:1` at `ED:726` suspect**) |
| demonstrator playground | UNOWNED | TY | 1 | 0 | E1 G | 0/0/1 |
| imperative | UNOWNED | TY (`TextWindowKit`) | 1 | 0 | – | – |
| **Totals** | | 15 SC + 10 ND + 6 scrub families + TY (writer, vcs, jack, md, html, binary, deflate, txt, imperative, demonstrator) + 1 ST | **49** | **1** | | |

Counts are census-of-source gesture families, not verified at runtime. Plugin-local phase parsers (`Some("commit") =>` re-implemented 7×: puzzle 2d select, layout transform, fem2d transient (fem3d reuses), lowpoly session, wfc bitmap brush, gen3d transforms, note ink) are duplicated scaffolding around the runner, candidates for one framework `ToolPhase` (not a closure blocker).
CAD engagement interpreter (`⚙️engine/🕹️interaction`, JSON-defined interaction machines, `apply_event_generic`) is a domain interpreter, not a scrub/drag statechart; it commits stamped with `TransactionRef`.

## (b) Full hit list grouped by owner

Legend: **B** behavioural blocker (must change before CLOSURE) · **V** legitimate view/config coalesce (stays until D1) · **M** mechanical (field/literal/test/doc, dies with the API) · **F** hand-written footprint · **W** whole-snapshot absolute leaf · **G** stale generated descriptor.

### S3-PUZZLE (puzzle 3d/5d)
- B none. V none.
- M E-literal `coalesce_key: None`: `puzzle/🖐️5d/S1/ED/🦀️.rs:8549,8768`; `puzzle/🧊️3d/S1/ED/🦀️.rs:6688,6872`. Retire path: `puzzle/🖐️5d/S1/ED/🦀️.rs:3082` (`puzzle5d_retire_optional_string_step(&mut owner.coalesce_key…)`). Tests: `🖐️5d/…/🔬️puzzle5d-retained-retirement-laws:245`; `🧊️3d/…/🔬️example-switch:68,178`, `🔬️unit:416,1957,1963` (bracket-ABSENCE law: keep); `puzzle/🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts:76,80`.
- F (14; art 12): 5d `ED:8571-8576` (5 n-rows via `targets.len()` / `PUZZLE5D_REMOVAL_INVERSE_ROWS`, 1 flat2), `ED:8706` lit2; window `🪟️window:285`; 3d `ED:6786` lit2, `ED:6889-6892` (3 n-rows via `PUZZLE3D_SELECTION_INVERSE_ROWS`/`REMOVAL`/`targets.len()`, 1 flat2); window `🪟️window:222`.
- G: `OS/🧑‍💻dev/🔌️plugin-modules/🧩️puzzle/🔣️.json:15243,15288`; `HUB/🧩️puzzle/🔣️.json:16408,16454` (GATES regenerates).

### S3-W2D (puzzle 2d)
- B/V none. M E-literal `puzzle/◻️2d/S1/ED/🦀️.rs:2450,2627`; tests `🧪️select-tool-transactions:458,534`.
- F (3): `ED:2548` lit2, `ED:2651` (`work_items` var), window `🪟️window:300`.

### S3-DRAW (draw + note)
- B none. V `draw/🖍️drawing/S1/👁️viewer/🦀️.rs:86` (`drawing.viewer.camera:<window>` window-config lane).
- M E-literal `draw/…/S1/ED/🦀️.rs:1347`; tests `🕹️nudge-selection/🧪️tests:75`, `🧪️tests/🔬️unit:1093`. Note retained accumulator: `note/🗒️note/S1/ED/🧵️retained/🦀️.rs:173,179,180,215,358` (Emit merge of `coalesce_key`; delete with the field).
- F: draw `ED:1372` n-rows (`drawing_inverse_rows`), window `🖼️canvas/🫧️transient:116`; note `🪟️window:186` (window).
- Side (not closure): 10 hand-labelled `Emit::commit(…, "label")` in draw (`edit-selection:122,127`, `edit-path:35`, `drop-layer-kind:22`, `patch-layers:32`, `delete-selection:58`, `edit-fill:20`, `duplicate-layer:20`, `move-layer:50`, `add-layer:61`), 2 in norm app-surface (`:1583,1591`), 1 lowpoly (`🗑️object:46`) — conflicts with §16.2 generic labels.

### S3-SPATIAL (shooting + fem + lowpoly)
- B shooting `camera-draft-label` keystrokes folded into the doc-config ledger by `Emit::amend_config` (`shooting/🎥️shooting/S1/ED/🎮️commands/🎥️camera/🦀️.rs:93`) — per-keystroke typed draft belongs in Draft/window transient, not a config amend. Shooting declares no footprint.
- V shooting `…/🎥️camera/🦀️.rs:112` (`SetCamera` viewport tick, doc-config lane `config_mutations`); fem 3d `S1w/ED/🎮️commands/⏯️set-result-animation/🦀️.rs:22,178`, `⏱️result-animation-tick:16,55,64`; fem 2d `⏯️set-result-animation:21,148`, `⏱️result-animation-tick:10,37` (window-config lane `window_config_mutations`, `PLAYBACK_COALESCE_KEY`).
- M E-literal `fem/🧊️3d/S1w/ED/🦀️.rs:443`, `fem/◻️2d/S1w/ED/🦀️.rs:430`, `lowpoly/S1/ED/🦀️.rs:1291`; tests `fem/🧊️3d/…/🧭️gumball/🧪️tests:36`, `⏱️result-animation-tick/🧪️tests:163`, `fem/◻️2d/…/🧭️gumball:36`, `…tick:104`; doc `lowpoly/S1/ED/🎚️config/🦀️.rs:158`.
- F (7 art): fem 2d `ED:362` n-rows, `:363` flat2; fem 3d `ED:416` flat2, `ED:476` n-rows (`node_ids.len()+solid_ids.len()`); lowpoly `ED:1215`,`:1253` flat2 (`move-selection`/`rotate-selection`/`scale-selection` have N-row inverses → verify, hazard class); fem 3d window `📊️results/🫧️transient:133`.
- G: `OS/🧑‍💻dev/🔌️plugin-modules/💠️lowpoly/🔣️.json:2139,2227,2941,3211,4891,5605,5875`; `HUB/💠️lowpoly/🔣️.json:1882,1938,2047,3512,5092`; `HUB/🏗️fem/🔣️.json:12468,12514`.

### S3-FLOWCAD (flow + cad)
- **B flow whole-scene `SetSnapshot` on the composed `content` child, 10 call sites of 4 publication functions** (`flow/🌊️flow/S1/ED/🦀️.rs`): `flow_scene_publication:2883-2888` (builds `SemioFlowMutation::SetSnapshot`), `flow_content_edit:2892` (callers `:1106`, `:1167` retained route), `host_scene_edit:2905` (callers `🎮️commands/✂️disconnect:18`, `➖️remove-widget:22`, `🗺️reorganize:22`, `🔗️connect-media-ports:26`, `🗑️delete-selection:32`), direct `🎨️set-active-example:30`, `🏷️rename-flow-widget:78`, and `✏️node-graph-edit/🦀️.rs:205` (setHostSnapshot/connect/disconnect/deleteSelection folded through the host engine, then `SetSnapshot`). These are one-shot intents, not gestures, but the leaf is non-parametric (`insert_node`/`insert_edge`/`delete_*` leaves already exist in the stdio semio flow vocabulary).
- M E-literal `flow/S1/ED/🦀️.rs:387`, `flow/…/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs:370`, `cad/S1/ED/🦀️.rs:1582,1793`; guard `flow/S1/ED/🦀️.rs:1519` (`emit.coalesce_key.is_some()` in the child-group output contract); tests `flow/…/🛠️tools/✋️drag/🧪️tests:47`, `cad/…/🛠️tools/🧭️transform/🧪️tests:52`, `cad/S1/ED/🧪️tests/🔬️unit:1523,1540,2124,2138`.
- F (8; art 6): flow `ED:440` (`work_items` var), `📬️preparation:35` n-rows (`inverse_rows`); cad `ED:1540,1546,1750,1758` flat2 (transform leaves with N targets → hazard class); windows `flow 🌊️main/🫧️transient:129`, `cad 🫧️transient:144`.

### S3-LAYOUT
- B/V/M-E none. M tests `layout/…/🔄️transform/🧪️tests:53`, `🧭️gumball/🧪️tests:29` (`assert_eq!(emit.coalesce_key, None)` family). F `ED:1089` n-rows from `layout_mutation_inverse_rows`, window `🫧️transient:127`. Reference implementation for the derived helper (it already derives the row count from the leaf).

### S3-CONTROLS (scrub / gis / energy / forms / norm / playbook)
- **B forms** `set-try-value`: window-transient emission carrying `coalesce_key: "formsTry:<window>:<generation>"` (`forms/📋️forms/S1/ED/🎮️commands/🎯️set-try-value/🦀️.rs:320-321,470`). Blocks narrowing `Emit.coalesce_key` to config lanes; make the preview a plain transient write.
- V gis `gis/🗺️gismap/S1/👁️viewer/🦀️.rs:103`; energy viewer `energy/🔋️model/S1/👁️viewer/🦀️.rs:133`, editor `…/ED/🦀️.rs:1275` (all window-config lane).
- M E-literal `gis/🗺️gismap/S1/ED/🦀️.rs:593`, `gis/🏔️gisterrain/S1/ED/🦀️.rs:278`, `forms/S1/ED/🦀️.rs:768`, `playbook/S1/ED/🦀️.rs:332`, `energy/S1/ED/🦀️.rs:1926`; tests `gis/…/👁️viewer/🧪️tests:60`, `energy/…/👁️viewer/🧪️tests:79`.
- F (6 art): gis `🏔️gisterrain/ED:417` lit2, `🗺️gismap/ED:618` flat2; forms `ED:784` flat2; playbook `ED:349` lit2; energy `ED:1938` lit2; forms window `▶️try/🫧️transient:363`.
- W norm: 15 standards `set-snapshot` (`"setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot`, `ED:44`) + `🎨️set-active-example` via the same leaf; macro `norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:1741,1792`. One-shot, outside closure.

### S3-TEXT (writer / vcs / trinity / stdio text)
- **B/W stdio text per-event whole-snapshot leaf (6):** `snapshot_edit_set_snapshot(event, …)` at `stdio/🗿️artifacts/📝️md/…/ED:468`, `🌐️html/…/ED:508`, `💾️binary/…/ED:374`, `🗜️deflate/…/ED:375`; typing net-diff fallback to `SetSnapshot` at `📝️md/…/ED:236`, `🌐️html/…/ED:231`. Contract: `stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:1302` (`snapshot_edit_set_snapshot`; the compact `snapshot_edit_patch:1311` exists with 0 callers). `🔤️txt` is TY-converted (no SetSnapshot hit); `📰️xml:263,270,540,550`, `📊️csv:319,339,352,373`, `📑️tsv:325,338,614` are UNOWNED text-adjacent.
- W writer `setSnapshot`/`setSnapshotJson` commands (`writer/✒️writer/S1/ED/🦀️.rs:213,215`).
- M E-literal `writer/✒️writer/S1/ED/🦀️.rs:963`, `vcs/🌿️vcs/S1/ED/🦀️.rs:668`; tests `writer/…/🚪️io/…/💾️binary/🧪️tests/🔬️unit:118`, `trinity/♻️rewriting/S1/ED/🧪️tests/🔬️unit:490`.
- F: writer `ED:883` flat2, vcs `ED:685` flat2; windows writer `📢️publication:32`, jack `📊️results/🫧️transient:169`, `📝️editor/🫧️transient:92`. Trinity rewriting declares none.

### S3-STROKES (raster / wfc / process3d / remodel)
- B none. **V process3d ×2** doc-config lane: `process/🧊️process3d/S1/ED/🎮️commands/⏱️cursor/🦀️.rs:13,32` (`Emit::amend_config(…, PROCESS3D_CURSOR_COALESCE_KEY)`), `🎛️engagement/🦀️.rs:6,30` (`replay` closure sets the same key).
- M E-literal `raster/S1/ED/🦀️.rs:660,828`, `wfc/🖼️bitmap/S1/ED/🦀️.rs:712`, `process3d/S1/ED/🦀️.rs:814,1149`; tests `wfc/◻️2d/…/🧪️tests/🔬️unit:236`, `wfc/🧊️3d/…:195`, `process3d/…/⏱️cursor/🧪️tests:36`, `…/🌍️world/🧪️tests:60`, `raster/…/🖌️paint-stroke/🧪️tests:111`.
- F (8): raster `ED:582,765` lit2; wfc bitmap `ED:536` n-rows; process3d `ED:753` flat2, `ED:1107` (var), `ED:1162` lit2; windows wfc grid2d `🪟️window:242`, wfc bitmap `🖼️input/🫧️transient:145`. Remodel declares none.
- W process3d `set_snapshot::SetDocument` (`process3d/S1/ED/🦀️.rs:210,1642`) — intent, outside closure.

### S3-PROCEDURAL
- B/V none. M E-literal `generation2d/S1/ED/🦀️.rs:828,1109`, `generation3d/S1/ED/🦀️.rs:1525`. F art via local helpers `generation2d_one_item_footprint:847`, `generation3d_one_item_footprint:445`; windows `generation3d/…/👁️preview/🫧️transient:120`, `…/👁️viewer/…:38`.

### S3-GRAPHS (dag / sequence / mathematical / space)
- B none. M E-literal `dag/S1/ED/🦀️.rs:481`, `sequence/S1/ED/🦀️.rs:1363`, `mathematical/➗️equation/S1/ED/🦀️.rs:919`, `PL/🪐️space/🫀️core/🦀️.rs:627`, `PL/🪐️space/🏠️home/S1/ED/🎚️config/🦀️.rs:299`, `HUB/🪐️space/⚙️engine/🪐️space/🦀️.rs:626`; doc `HUB/🪐️space/…/🎚️config/🦀️.rs:148`.
- F (7): dag `ED:441` lit2; **sequence `ED:1241` `work_items:1` on the artifact lane (suspect under-declaration)**; sequence window `📜️script/🫧️transient:104`; mathematical `ED:942` flat2; space core `🫀️core:643` flat2, home `🎚️config:341` lit3, `HUB/🪐️space/…/🦀️.rs:639` lit2.
- Reasoning wires drag (UNOWNED below) should be folded here by §13.3 (guests implement `move-nodes{ids,dx,dy}` and reuse the node-drag machine).

### UNOWNED
- **B reasoning/wires hand-rolled drag**: `reasoning/🔌️wires/S1/ED/🦀️.rs:267-284` (`drag_mutation`, `SetDrag{start,last,zoom}` window transient), `:330-410` (`CanvasPointerDown/Move/Up` work steps), commit `:395-403` (`Emit { artifact_mutations: [move_node(id, x, y)] }` ABSOLUTE position, plain edit, no `ToolTransaction`), preview `:708-720`; window footprint `🕸️canvas/🫧️transient:104`.
- W stdio non-text 73 `snapshot_edit_set_snapshot` sites (semio 19, pdf 10, step 7, ifc 5, pptx 3, docx 3, xlsx 3, svg 3, dwg 2, xml 2, gif 2, zip 2, bmp, ply, gltf, obj, dxf, stl, avi, tsv, bcf, mp3, epw, las), + 89 `fn snapshot_edit_mutations` impls. Queued WP S3-STDIO is not in the 20-agent fleet.
- M E-literal (10): `animate/🎬️presentation/S1/ED/🦀️.rs:477`, `demonstrator/🎪️playground/S1/ED/🦀️.rs:177`, `block/🧊️3d/S1/ED/🦀️.rs:441`, `block/🖐️5d/S1/ED/🦀️.rs:317`, `block/◻️2d/S1/ED/🦀️.rs:345`, `sourcing/🗂️curation/S1/ED/🦀️.rs:566,772`, `stdio/🧿️semio/ED/📬️preparation/🦀️.rs:192`, `stdio/📜️docx/…/📬️preparation/🦀️.rs:339`, `stdio/🎒️zip/…/📬️preparation/🦀️.rs:196`; doc `sourcing/🗂️curation/S1/ED/🎚️config/🦀️.rs:92`.
- F (15 incl. 5 window): animate `ED:516` lit3, demonstrator `ED:189` lit2, block 3d `ED:457,583` lit2, 3d window `🌐️world/🫧️transient:108`, 5d `ED:243` flat2, 2d `ED:272` flat2, sourcing `ED:496` (var), `579,785` lit2, **`726` `work_items:1` suspect**, stdio zip `📬️preparation:43` n, docx `:210` flat2, semio `:57` n.
- G: `OS/🧑‍💻dev/🔌️plugin-modules/🎪️demonstrator/🔣️.json:33228,35104`; `HUB/🎪️demonstrator/🔣️.json:25911,25957`.

### Framework-owned hits (RUNTIME / W1-G / CLOSURE) — see (c)

## (c) Framework-level deletions possible once owners finish

| API | Definition | Remaining callers / consumers (non-test unless noted) | Deletable when |
|---|---|---|---|
| `Emit::amend(` | `OS/🔌️plugin/🦀️.rs:12869` (+ docs `:12866-12868`) | **0 production**. Test `OS/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:882` (`TestCommand::AmendLabel`); gate `📜️script.ts:9654,9721,9900-9931` (`INTERACTIVITY_TOOL_RUN_TRIGGER`, `Emit::amend` predicate) and its planted-violation test `FW/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts:25,92,120,124-126` | **now** (rewrite the gate to a repo-wide ban first) |
| `ArtifactCommand::AmendLast` / `AmendLastInLane` — DOCUMENT lane | plugin `artifact_lane_commands` `OS/🔌️plugin/🦀️.rs:24143-24155`; store enum `OS/🏪️store/🦀️.rs:3003,3038`, arms `3137,3166`, exec `20043-20044`, `amend_command:20751` | **0 callers** (all document keys gone) | **now** |
| `AmendLast` — CONFIG lanes | doc-config `OS/🔌️plugin/🦀️.rs:28115-28116` (`config:{key}`); window-config `OS/🔌️plugin/🪟️window/🎚️config/🦀️.rs:447` (`window:{id}:{key}`) | 13 view sites: `amend_config` ×3 (shooting `🎥️camera:93,112`, process3d `⏱️cursor:32`) + `Emit{coalesce_key}` literals ×10 (process3d engagement `:30`; fem 3d ×3 / fem 2d ×2 playback; gis viewer, energy viewer + editor, draw viewer; forms try-value is a transient lane, B) | after D1 |
| `AmendLast` codec | `OS/🏪️store/🦀️.rs:14166-14299` (`CommandHeaderLine::Amend`/`AmendInLane` print+parse), `14531-14704` (binary tags 0b01 presence), `20108-20788` | store/spr tests: `🏪️store/🧪️tests/🔬️unit/🦀️.rs` 18 `AmendLast` lines (6364-7557), `🧪️tests/🧪️tool-transaction` 2, `OS/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs:1845-1847` | with the variant (re-seal fixtures; greenfield, no compat) |
| `Emit.coalesce_key` field + ctors | `OS/🔌️plugin/🦀️.rs:12451,12519,12870,12927`; `TransactionProposalDraft.coalesce_key` `:15311,16896,28082,31700`; reset `OS/🔌️plugin/🛠️tool-machine/🦀️.rs:395,412`; retire `OS/🔌️plugin/🦀️.rs:20871`; shape guard `tool_transaction_shape_fault(…, coalesced)` `:24126-24130`; retained route `:31451-31783` (`set_coalesce_key`, `window_config_store.begin(…, key)`) | plugins: note accumulator ×5, flow guard `:1519`, puzzle5d retire `:3082`, forms B, 13 V sites | after D1 + owners' B/M |
| `Edit.coalesce_key` wire/store field | `FW/📡️replication/🎮️mutation/🦀️.rs:1667,1697-1755` (ToValue/FromValue); store schema spec id 7 `OS/🏪️store/🦀️.rs:7221`, retire `808-811`, hydration `12326,13002,13295-13439`, canonical digest `14974` + `🧵️canonical-edit/🦀️.rs:102,148`, publication `16528-16607,16749-16797`, `batch_amend_target` key branch `18912,19033,19053,19230` (the transaction branch STAYS), `GroupMeta.coalesce_key` `25464-25473`, `🧾️document/📜️history/💧️hydration:357`, `🎚️config/📥️retained:273`, `🧬️snapshot-clone:213`, `🔄️sync:1043`, `🧩️composition/🚪️open:352` | spr history `OS/📡️spr/📜️history/🦀️.rs:117,390-442,490,833-861,920-960` (`HistoryEdit`, presence bit 1<<2), spr cli `OS/📡️spr/⌨️cli/🦀️.rs:199,368,457`, spr channel `AppFrame::TransactionProposal` `OS/📡️spr/🧵️channel/🦀️.rs:2873,3901-3907,4057` + TS twin `OS/🟦️.ts:2705,3414,3584-3586` + generated `OS/📺️renderer/…/🎞️frame-worker/🤖️generated/🟨️.js:26116-26118` + plugin `:42657`; manifest `FW/🛂️manifest/🦀️.rs:4392` (`TutorialDocumentEventKind::Edit`) + generated TS `FW/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:1170` + `FW/🧬️schema/📽️projection/🦀️.rs:1527`; **44+1 plugin `protocol::Edit` literals** (`coalesce_key: None`) | after D1 (the key must not be persisted; see step 6). Fixtures to re-seal: `OS/🏪️store/🧫️fixtures/📤️outbound-announcement/🔣️.json` (19), `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` (7), `🧬️schema/📤️outbound-announcement` (2), `OS/🔌️plugin/🧵️retained-command/🔄️full-operation/🧫️fixtures` (7) + `🧬️schema` (2), tests: store 36 + 3 + 2, spr 7+2+2+2+2+1+1+1, replication causal 3, manifest `🔬️app-label:1055,1059,1214,1223`, plugin `🔬️app-typed-command-full-operation` 9, `🧬️mutation-fixtures-transaction` 2, `🧪️time-travel` 1 |
| `UtilityPreviewContract` | anchor `OS/🔌️plugin/🦀️.rs:13435` (+ refs `:12868,12875`) | none (3 doc lines) | **now** (rename to `ToolContract`; the contract text is still valid) |
| Bracket rule | deleted: `FW/🛂️manifest/🦀️.rs:2167` ("no bracket verb exists to classify") | descriptors only: `OS/🧑‍💻dev/🔌️plugin-modules/{🎪️demonstrator,🧩️puzzle,💠️lowpoly}/🔣️.json` (10 lines), `HUB/{🎪️demonstrator,🧩️puzzle,🏗️fem,💠️lowpoly}/🔣️.json` (11 lines); KEEP the negative cases `FW/🛂️manifest/🧫️fixtures/🖐️gumball-verb-audience.json:2,8,9` and `puzzle 3d 🧪️tests/🔬️unit:1957,1963` | GATES regeneration |
| Fold-footprint hand declarations | `ArtifactStoreOneItemFootprint::{for_one_invertible_item, for_one_item}` `OS/🏪️store/🦀️.rs:15886-15905`; framework flat defaults `OS/🔌️plugin/🧵️retained-command/🦀️.rs:154-156`, `OS/🔌️plugin/🦀️.rs:16871`; enforcement `fold_batch_item` `OS/🏪️store/🦀️.rs:19184` | 80 plugin sites (§5) | derived helper lands (step 4) |

Keep, different concept (rename to keep the zero-hit grep absolute): `OS/🛎️services/🦀️.rs:2469-2648` (`ChannelPolicy::Coalesced` mailbox key), renderer `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1579-2765` (turn-queue latest-wins `coalesceKey`; no caller outside the file passes it → likely dead option), `🕸️NodeGraph/🟦️.tsx:2549-2562` (24 ms host payload coalescer), SQL `COALESCE(` in `🌎️hub/📇️directory/🪶️sqlite|🐘️postgres`, `🌎️hub/💡️inference/🪶️sqlite`.

## 5. Fold-footprint census (§17.7 input)

80 declaration sites, 0 derived (non-test source). Artifact lane 60: 20 `for_one_invertible_item` (flat 2 rows), 16 `for_one_item(n,…)` hand-counted (puzzle 8, fem 2, flow, layout, wfc bitmap, draw, stdio zip/semio), 16 struct-literal `work_items: 2`, 2 `work_items: 3` (animate `ED:516`, space home `🎚️config:341`), **2 `work_items: 1` (sequence `ED:1241`, sourcing `ED:726`; contradicts `ArtifactStoreOneItemFootprint` doc: a durable item stages forward + inverse rows)**, 4 computed (flow `ED:440`, process3d `ED:1107`, sourcing `:496`, puzzle 2d `ED:2651`) + gen2d/gen3d local helper fns. Window-transient lane 20 literals `work_items: 1`.
Plugins with NO declaration (run on framework flat default `OS/🔌️plugin/🧵️retained-command/🦀️.rs:154`, `OS/🔌️plugin/🦀️.rs:16871`): shooting, remodel, trinity rewriting, norm, architect, imperative.
Multi-row-inverse leaf families still declared flat (hazard, unverified at runtime): lowpoly relative transforms (`ED:1215,1253`), cad transform (`ED:1540-1758`), fem 2d `_ =>` default (`ED:363`), wfc 2d/3d `drag-slots`, procedural transforms, flow `drag-nodes`, shooting gumball (none).
Only seams that compute rows from the leaf: layout `layout_mutation_inverse_rows`, draw `drawing_inverse_rows`, wfc bitmap `bitmap_inverse_rows`, flow `inverse_rows`, fem/puzzle `targets.len()`-style hand counts. Store enforcement: `ArtifactStore::fold_batch_item` (`OS/🏪️store/🦀️.rs:19184`) compares `forwards.len()+inverse.len()` to the merged declaration; preflight traits `ArtifactStoreOneItemPreparationFactory::preflight` (`:16162`), `ArtifactEphemeralOneItemPreparationFactory::preflight` (`:4335`).

## Decisions the brief needs (coordinator)

- **D1 config-lane amend.** §17.5 keeps `amend_config` for pure view/config state, §17.7/AGENTS require deleting `AmendLast*`; the two live config callers use `AmendLast` + a persisted `Edit.coalesce_key`. Recommended: (A) one type-restricted `ConfigAmend` path (`ArtifactCommand::AmendConfigLast`, accepts only `ConfigMutation`/`WindowConfigMutation`), the key lives in store RAM (`amend_key`), never in `Edit`/digest/wire/`HistoryEdit`; config-lane edits never surface as history rows (law L4). (B) alternative: drop config amends entirely, make camera/playback/cursor window-transient replace writes (no ledger). Either way `Emit::amend` and the document lane go first.
- **D2** reasoning wires drag → S3-GRAPHS (reuse `node_drag_commit` + relative `move-nodes`), or CLOSURE does it.
- **D3** stdio non-text `snapshot_edit_set_snapshot` (73) → queued S3-STDIO; switching to the existing `snapshot_edit_patch` (0 users) or leaf-level mutations.
- **D4** `Emit::commit(mutations, "label")` hand labels (13 sites: draw 10, norm 2, lowpoly 1) vs §16.2 generic labels: fold into G7 gate, not CLOSURE.

## (d) CLOSURE WP brief — ready to paste

**WP S3-CLOSURE.** Owner files: `OS/🔌️plugin/🦀️.rs` (Emit, artifact_lane_commands, retained route, `TransactionProposalDraft`), `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`, `OS/🔌️plugin/🪟️window/🎚️config/🦀️.rs`, `OS/🔌️plugin/🧵️retained-command/🦀️.rs`, `OS/🏪️store/🦀️.rs` + `🧵️canonical-edit` + `🧾️document/📜️history` + `🎚️config/📥️retained` + `🧬️snapshot-clone` + `🔄️sync` + `🧩️composition/🚪️open` (coordinate with W1-G), `OS/📡️spr/📜️history`, `OS/📡️spr/🧵️channel`, `OS/📡️spr/⌨️cli`, `FW/📡️replication/🎮️mutation`, `FW/🛂️manifest/🦀️.rs` (+ generated TS), `FW/🧬️schema/📽️projection`, `OS/🟦️.ts` + frame-worker generated TS/JS, root `📜️script.ts` policy region, `FW/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy`. Plugin trees: mechanical edits only (E-literals, tests, comments).
Precondition (owners done, re-grep): S3-FLOWCAD flow `SetSnapshot` ×10 → leaves or accepted-as-intent (D); S3-TEXT stdio text ×6; S3-CONTROLS forms try-value B1; S3-SPATIAL shooting draft-label B1; wires drag (D2); D1 decided. Mechanical steps may start earlier (1-3).

Ordered steps (test-first, each lands compile-atomic):
1. **RED gate.** Rewrite `📜️script.ts:9721,9900-9931` + draw gate `:5825` into ONE repo-wide ban: no `Emit::amend`, `AmendLast`, `UtilityPreviewContract`, `transformBegin|transformEnd|paintStrokeBegin|paintStrokeEnd`, `Emit.coalesce_key` on any artifact lane, `Edit.coalesce_key`, `snapshot_edit_set_snapshot` (allowlist: the 3 negative fixtures/tests). Planted-violation tests like the existing policy test. Gate is red until step 8.
2. **Delete the document lane now:** `Emit::amend`, `artifact_lane_commands` `AmendLast` arm (`OS/🔌️plugin/🦀️.rs:24154-24155`), the `coalesced` parameter of `tool_transaction_shape_fault` (`:24126,28032,31708`), tool-machine resets (`🛠️tool-machine:395,412`), rename `🔖️UtilityPreviewContract` → `🔖️ToolContract` (3 lines). Update runtime test `…plugin-runtime-plugin-builder-contract:882` (drop `AmendLabel`).
3. **`next_edit` helper.** One `ArtifactStoreOneItemLiveAuthority::next_edit(prefix, forwards, inverse, description)` (`OS/🏪️store/🦀️.rs:15961`) replacing the 45 per-plugin `protocol::Edit {…}` literal sites (`*_next_edit` builders) (list in (b) M-E); this removes every plugin `coalesce_key: None`.
4. **Derived fold footprint (§17.7).** Schema-first: leaf schema declares `inverseRows` (`fixed:N | perTarget:<field> | bounded:<const>`), `🗣️dsl`/mutation derive emits `fn inverse_rows(&self) -> usize`; add `ArtifactStoreOneItemFootprint::for_leaf(&impl InverseRows, retained_bytes)` + derive default for both preflight factories (`:16162`, `:4335`) and the retained route default (`OS/🔌️plugin/🧵️retained-command/🦀️.rs:154`, `OS/🔌️plugin/🦀️.rs:16871`); replace the 80 declarations (list in (b) F), fix the 2 suspect `work_items:1` (sequence, sourcing) and the 6 no-declaration plugins; make hand-written declarations a gate failure. Law over every plugin: for each leaf fixture `declared.work_items >= forwards.len() + inverse(base).len()` (`fold_batch_item` never refuses); layout's `every_committed_inverse_fits_its_declared_fold_footprint` is the template.
5. **Config lane per D1** (recommended A): replace `AmendLast` config usage (`OS/🔌️plugin/🦀️.rs:28115-28116`, window-config `:447`) by the type-restricted `ConfigAmend`; key in store RAM; rename the 13 view sites (`Emit::amend_config` ×3, `Emit{coalesce_key}` ×10) to the new ctor; forms try-value becomes a plain transient write.
6. **Store/wire deletion (with W1-G):** delete `ArtifactCommand::AmendLast`/`AmendLastInLane` (`OS/🏪️store/🦀️.rs:3003,3038`), codec arms (`14166-14299,14531-14704`), `batch_amend_target` key branch (keep the `TransactionRef` branch), `set_coalesce_key`, `Edit.coalesce_key` (`FW/📡️replication/🎮️mutation/🦀️.rs:1667` + schema spec id 7 `:7221` + canonical digest `:14974`/`🧵️canonical-edit:102,148` + `HistoryEdit` + spr history presence bit + CLI `(amend)` print + `GroupMeta.coalesce_key`), `AppFrame::TransactionProposal.coalesce_key` + TS twin + generated frame-worker, manifest `TutorialDocumentEventKind::Edit.coalesce_key` + generated TS. Greenfield: no compat; re-seal the fixtures listed in (c).
7. **Plugin sweeps** (owner files from (b)): note retained accumulator (`🧵️retained:173-358`), flow guard (`ED:1519`), puzzle5d retire (`ED:3082`), 13 view-lane sites, plugin tests asserting `coalesce_key` (list (b) M-tests; replace by `assert!(emit.transaction.is_some())` where the law is "a committed gesture is one stamped edit").
8. **Descriptors/GATES:** regenerate demonstrator/puzzle/lowpoly plugin-modules + HUB compositions (`describe`) so no bracket verb is listed; run `verify layering`, `verify dependencies literal-external`, `schema mutation-inputs`/`mutation-payloads`, docstring + `[DEBUG]` sweeps.
9. **Close the gate:** step 1 gate green; add the cross-plugin law harness cases below.

Acceptance (all from repo root, `/usr/bin/grep -rnI`, excluding `node_modules|target|dist|🗑️generated`, over `✏️s 🧰️framework 🌎️hub`):
- `Emit::amend\b` → 0 (incl. tests and gate regexes, except the gate's own planted-violation strings kept in one allowlisted file).
- `AmendLast` → 0. `amend_config` → 0 (renamed ctor per D1) or exactly the D1 ctor.
- `coalesce_key|coalesceKey` → 0 in `OS/🏪️store`, `OS/🔌️plugin`, `OS/📡️spr`, `FW/📡️replication`, `FW/🛂️manifest`, `FW/🧬️schema`, `✏️s/🔌️plugins`, `🌎️hub/🧩️compositions`; after renaming the 3 unrelated concepts (services mailbox, renderer turn queue, NodeGraph payload coalescer) the repo-wide count is 0 outside SQL `COALESCE`.
- `UtilityPreviewContract` → 0. `transformBegin|transformEnd|paintStrokeBegin|paintStrokeEnd` → 0 except the 3 negative-case artifacts (`🖐️gumball-verb-audience.json`, puzzle 3d bracket-absence test).
- `for_one_invertible_item|for_one_item|ArtifactStoreOneItemFootprint \{` → 0 in plugin trees (only the derived helper); every plugin leaf fixture satisfies the footprint law.
- `snapshot_edit_set_snapshot` → 0 (or explicitly D3-allowlisted).
- Laws: **L1** `ArtifactCommand` has no amend variant on the document lane (type-level) + repo gate; **L2** every tool machine family: N ticks + commit = exactly 1 edit, abort = 0 edits (transaction-law fixture across the 15 SC + 10 ND + 6 SR + TY families, via the cross-plugin harness §16.3); **L3** fold-footprint law (above); **L4** config-lane edits never appear in `HistoryMutationEntry`/history rows; **L5** canonical edit digest + wire schema have no `coalesceKey`, fixtures re-sealed, Rust + TS twins equal; **L6** no plugin descriptor lists a bracket verb and no handler outside `🛠️tool-machine` commits an absolute final-state leaf for a pointer/drag sequence (gate: any `CanvasPointerUp`/`WorldPointerUp`/`GraphPointerUp` handler returning `Emit { artifact_mutations }` without `commit_transaction` fails).
