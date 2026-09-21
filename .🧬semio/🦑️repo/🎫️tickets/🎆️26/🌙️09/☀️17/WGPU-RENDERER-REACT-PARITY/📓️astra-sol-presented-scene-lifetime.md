# Presented Scene Lifetime

## Concrete defect

The presented-input implementation keeps two move-owned `UiTree`/`EventRouter` pairs per window. Candidate document reconciliation runs against `UiWindow.tree`; accepted input continues through `UiWindow.presented_tree` until the presenter acknowledges the exact candidate witness.

`Ui::step_document_reconcile` currently passes the public `window.scene_retirements` queue directly into `UiTree::step_document_reconcile_preserving`. A candidate replacement or removal therefore publishes `UiRetiredComponentScene` immediately. Interpreter takes that record at the end of the candidate render opportunity and calls `Scenes::retire_scene_identity`. The next scene close step marks the old host retiring before the candidate pixels are accepted. The old `presented_tree` can still draw and dispatch that exact host during this interval, but `scene_host_retiring(host_id)` now refuses its input and its local state begins closing.

This violates the same presentation rule already enforced for hit geometry and retained widget dispatch: candidate topology cannot retire the owner of still-presented pixels.

## Host identity defect

The two tree arenas are alternated by `acknowledge_presented_input`. Candidate baseline construction rebuilds a document into the inactive arena. Reconciliation currently derives `host_id` from `(window_generation, candidate arena NodeId, candidate-local component_generation)`.

Those fields are not a window-level component mount identity:

- an unchanged logical scene can receive a different host because it was rebuilt in the other arena;
- a removed and later re-added scene can reuse an arena address and component generation and alias an earlier host;
- the candidate's old tree is two presentations behind after a swap, so preserving only its local `component_generation` does not establish identity with the current presented tree.

`UiRetiredComponentScene` previously reconstructed the host downstream instead of carrying the exact host stored in the retired node. It now carries that actual `host_id` through reconcile, engine retirement, and Interpreter consumption.

## Bounded repair

Each `UiWindow` owns a checked, monotonic component-mount epoch. The runtime host string is allocated from the exact surface lifetime plus this epoch; it is independent of either arena address.

During one candidate Mount step, scene identity resolves in this order:

1. if the current presented document has the same document-node identity, explicit key, component kind, and wire surface ID, transfer its exact `host_id`;
2. otherwise, if the candidate-only old node is the same logical mount, preserve that host;
3. otherwise allocate one fresh window mount epoch or refuse with the existing component-generation fault boundary on exhaustion.

This is an exact runtime metadata transfer. It does not make wire `surfaceId` unique and does not copy scene state.

Candidate reconciliation separates retirements:

- a retired candidate-only host absent from `presented_tree` can enter the normal scene retirement queue immediately;
- a host still mounted in `presented_tree` enters a fixed deferred candidate queue and remains live;
- after reconciliation, one deferred entry per opportunity is compared with the final candidate tree: a host that survives is discarded, while a true removal is retained for post-acceptance retirement;
- candidate readiness and sealing wait for this resolution cursor;
- acknowledgement move-swaps the already-resolved post-acceptance queue into the public scene-retirement queue without an acknowledgement-time scan;
- discarding or superseding an unaccepted candidate cannot mark a presented host retiring.

The fixed logical retirement capacity remains 64. Public, candidate-deferred, and accepted-deferred queues have independent storage so move publication does not allocate, but every enqueue checks their combined length against the one unchanged cap. Resolving one candidate retirement pops before it can append to the accepted queue, so that transfer cannot increase the logical total. Backpressure pauses the reconcile cursor; no retirement is dropped and no bound is raised. If a newer document supersedes an unaccepted candidate, accepted-deferred entries move back to candidate resolution so a reintroduced host cancels its stale retirement.

## Presented capture transfer

`UiWindow::step_presented_interaction_rebase` currently returns `false` whenever `presented_router.capture()` is populated. `seal_presented_input_candidate` independently refuses a candidate while that capture exists. A held Slider, Scrollbar, or Ring can therefore stop all new pixel presentation until pointer release. This is visible starvation rather than a safe presentation fence.

Capture is part of the presented interaction revision and must transfer through an exact logical-node mapping. Candidate rebase maps the captured presented node through protocol `UiNodeId`, explicit key, widget kind, and component mount identity. It stages the candidate router with the mapped capture while the presented router remains the sole dispatcher until pixel acknowledgement. A missing or changed target refuses that candidate revision; it does not clear or redirect the presented capture. After acknowledgement, the moved candidate router continues the same pointer owner so move/release terminates normally on the newly presented geometry.

## Required laws

Native UI/renderer laws must cover:

- presented scene A remains live and dispatchable while a candidate removes it;
- aborting that candidate preserves A and publishes no retirement;
- accepting that candidate publishes A's exact retirement only after acknowledgement;
- an unchanged scene across A→A retains one exact host across alternating tree arenas;
- A→B→B gives B one stable host, distinct from A;
- remove→re-add allocates a fresh host and cannot receive A's late gesture, focus, clipboard completion, or scene-state retirement;
- a candidate-only mount removed before presentation retires without affecting the presented host;
- deferred retirement capacity and per-opportunity progress remain bounded.
- a held Slider/Scrollbar/Ring can accept and present a candidate revision without dropping capture;
- move and release after acknowledgement reach the mapped captured node once;
- changed or removed capture targets keep old pixels/router authoritative until the original gesture terminates.

Three native UI laws are now registered:

- `candidate_scene_removal_retires_only_after_presentation_acknowledgement`;
- `component_scene_host_survives_alternating_arenas_and_readd_gets_a_fresh_mount`;
- `held_pointer_capture_transfers_to_the_accepted_candidate_and_releases_once`.

Renderer Native107 supplied the concrete pre-repair two-arena Canvas remount/deadline failure. Renderer Native108 ran 38 selected renderer laws: 30 passed and 8 failed. The three focus/clipboard laws, corrected presented keyboard law, and valid sibling Canvas camera laws passed. The remaining three Canvas camera failures established downstream scene-lifetime work outside the UI identity owner.

UI7 ran 664 UI engine laws: 658 passed and 6 failed. Three new laws passed: removal retirement stayed deferred through acknowledgement, held capture transferred and terminated once, and a superseded removal rechecked rather than publishing a stale retirement. The alternating-arena remove/re-add law established the intended RED: the re-added scene incorrectly reused `scene.1.1`. The repair now distinguishes an active unaccepted candidate from an accepted inactive scratch arena, rebuilds the scratch tree through the credited baseline after every acknowledgement, and admits old-owner retirement before allocating a fresh mount epoch. UI8 is the first post-repair gate; no GREEN UI receipt is claimed yet.

The Native107 keyboard failure was a fixture mismatch: its test constructor used `commit: "enter"` with a Commit binding, while the contract maps Commit to `commit: "blur"` and treats Enter as that mode's explicit terminal. Debug showed both keyboard events reached the presented node. The corrected typed fixture passed in Native108.

The renderer's separate scene-pointer owner table still needs the same exact pre-acknowledgement node mapping. `Ui::candidate_is_sealed_for` and `Ui::candidate_scene_node_for_presented_node` now expose a witness-gated, scene-only map from presented arena node through its Node-owned protocol `UiNodeId` to the candidate tree's binary lookup. EventRouter, tooltip, and composite-owner reverse lookup use the same O(1) Node metadata instead of scanning document bindings. Root owns the bounded update of at most 16 renderer scene-pointer owners after the matching acknowledgement.

## UI8 and UI9 receipts

UI8 ran 664 tests: 662 passed and 2 failed in 4.672 seconds. All six UI7 failures were resolved. The exact fixed-slot census measured capacity 64, element size 164128 bytes, and owner size 520 bytes; the authored budget now records those values without changing capacity. The remaining capture-law failure was a fixture precondition: a one-node baseline reconstructed the receiver at `NodeId { index: 0, generation: 0 }` in both arenas.

UI9 ran 664 tests: 663 passed and 1 failed in 3.596 seconds. The updated census passed. A fully reconstructed A → A,B → A,C,B helper still canonicalized B at `NodeId { index: 2, generation: 0 }` in both arenas, so the raw-address inequality remained invalid. The fixture now creates an actual divergent retained history without mutating private arena state: it accepts A, reconciles an unaccepted A,C candidate, supersedes and accepts A,B, then stages A,C,B. B retains its protocol ID, key, widget kind, and presented capture; the removed C changes the accepted arena's allocation history. UI10 has not run.

## Canvas camera retirement audit

The three Native108 Canvas failures share one downstream witness. `renderer_canvas_retirement_probe` checks out the old `SceneCameraDispatchCursor` before it paints and acknowledges replacement/removal. `paint_component_pointer_documents` drives the real retained document consumer through paint, seals and acknowledges its input candidate, and continues opportunities while a published scene retirement drains. A checked-out deadline publishes only if `SceneCameraDeadline::owns_surface` still finds its exact old host in `SCENE_STATE`.

The repaired UI lifetime changes the required facts without a renderer camera special case: removal/replacement publishes the old host retirement only after acknowledgement, and remove/re-add allocates a fresh host. The checked-out old deadline must therefore fail `owns_surface`; a replacement's separately scheduled deadline remains in the global deadline table and should be the sole later action. Native108 predates the final accepted-baseline/re-add repair. The next renderer run must verify all three laws before they are called green.

## Presented editor address RED laws

Three native laws now use the same accepted A / unaccepted A,C / accepted A,B / staged A,C,B history through actual document publish, reconcile, presentation seal, and acknowledgement:

- `presented_editor_text_focus_rebases_only_after_same_host_pixels_are_accepted`;
- `presented_editor_ink_focus_rebases_only_after_same_host_pixels_are_accepted`;
- `presented_editor_ink_clipboard_owners_rebase_only_after_same_host_pixels_are_accepted`.

They assert that candidate construction cannot mutate the presented address and that acknowledgement rebases the same mounted host to the candidate `NodeId`. The clipboard law covers the focused Ink address, an active fixed-slot stream, and a generation-stamped pending native clipboard owner. Production rebase remains held pending a valid native RED. The intended seam is bounded prepare/commit: prepare queries the witness-qualified UI map for the few mounted globals and at most 32+32 fixed clipboard slots without mutation; commit after successful acknowledgement updates entries only when their exact `(window, generation, old node, host)` still matches. A nonparticipating window produces no prepared entry and remains untouched.

## UI10 receipt and component presentation witness

UI10 ran the complete native UI suite: 664 passed, zero failed, and zero skipped. Test execution took 4.249 seconds and Nx completed in 30.3 seconds. The accepted A / unaccepted A,C / accepted A,B / candidate A,C,B construction now proves a strict raw arena-node change while preserving the same protocol node, component kind, key, host, window lifetime, and capture. This closes the UI-side capture-transfer and fixed-capacity laws; it does not establish renderer camera or editor-address rebasing.

The three Native108 Canvas failures expose a separate liveness problem. A checked-out camera deadline currently asks only whether `SCENE_STATE` still names the same component host. After one accepted frame removes several components, UI publication makes every old component non-presented at once, while the bounded Scenes retirement lane can admit only one retirement record per opportunity. The second old host can therefore remain in `SCENE_STATE` long enough to publish a camera action for pixels that no longer exist. Draining one retirement immediately after acknowledgement cannot establish the required atomic presentation boundary for multiple removals.

Resource liveness needs a first-party component presentation witness. The witness is the exact window ID and generation, protocol `UiNodeId`, stable component `host_id`, explicit key, component kind, public wire `surfaceId`, and component generation. UI checks it against the accepted `presented_tree` using the protocol binding's bounded binary lookup. Arena `NodeId` is intentionally absent from this resource-liveness decision: an unchanged component may move between the two arenas at acknowledgement. Pointer ownership remains stricter and continues to validate and rebase the exact arena node because pointer routing targets the current UI event record.

Two renderer laws are required before the camera predicate changes:

- accepting one candidate that removes two Canvas components must invalidate both checked-out camera deadlines immediately, even while the sole scene-retirement handoff contains only the first owner;
- inserting a sibling so an unchanged Canvas moves to another arena `NodeId` must preserve that Canvas camera deadline through acknowledgement and publish exactly one action for its accepted host.

The schema change must make protocol identity required. `UiRetiredComponentScene` carries the retired record's protocol `UiNodeId`, and `ScenePointerTarget` carries the same ID from its mounted UI record. No wire-surface fallback or optional identity is valid. `same_component_host` includes the protocol node; pointer liveness additionally keeps the arena-node equality already enforced by the renderer capture table.

## Remaining owners at the presentation boundary

A read-only owner audit after Native113 distinguishes owners that already survive a same-host arena rebase from owners that still use the wrong identity:

| Owner | Current identity | Required treatment |
| --- | --- | --- |
| renderer scene-pointer capture, maximum 16 | full `ScenePointerTarget`, including arena node | Keep exact pointer liveness. The existing pre-ACK mapping rebases the arena node only after accepted pixels. |
| `SceneSurfaceState.mount_owner` and `canvas_camera_owner` | full target, compared by `same_component_host` | Resource identity. Add required protocol node to the host comparison and consult accepted UI presentation for autonomous camera output. |
| `EngineCanvas.map_interaction_owner` | full target, retired with exact `PartialEq` | Incorrect across accepted arena swaps. Retirement must compare the component witness, while live pointer routing remains arena-exact. |
| Canvas pointer gesture and catalogue hover | window generation plus stable host, without arena node | They already survive same-host swaps. Component removal can remain observable until bounded scene retirement reaches the host, so a later cancellation can still emit an old document action. A future removal fence needs the same accepted-component witness if these terminal paths remain externally callable. |
| row/list transfer session | pointer, source window generation, stable host | Same-host safe. Its terminal is reached only through the rebased live pointer route or explicit close/cancel; no arena-node change is required. |
| raster producer and external engine close | stable host and retained engine token | Resource owner. They close by host after scene retirement; they do not need an arena node, but their work may continue until the bounded retirement record is admitted. |
| queued `SceneInteractionIntent` and nested Ink job | tree revision, window generation, arena node, host, painted rect | Still stale across acknowledgement. `process_scene_interaction` rejects the old tree revision and node even when the component host survives. Candidate acceptance can therefore drop an already-admitted presented pointer event or retire a partially stepped Ink job. This needs either a bounded same-host intent prepare/commit rebase with accepted geometry, or an explicit presentation barrier while the exact intent is checked out. It must not be folded into camera liveness. |

The queued-intent issue is a concrete acceptance blocker, not evidence that the presenter should wait for every scene resource. It requires an explicit semantic law before choosing between a bounded intent barrier and address rebasing. No law or production change was added in this packet because Native115 is intentionally limited to the editor repair and two camera laws.

Further inspection narrows that recommendation: `InkInteractionJob` snapshots the old scene document, selection, utility, camera, and drag when the intent first steps. A same-host candidate may legitimately contain newer authored document content. Rebasing that partially stepped job to the candidate node and rectangle would combine old semantic data with new pixels. React completes one DOM handler synchronously against the displayed props before a later render commits. The equivalent native boundary is therefore a pre-presentation barrier for admitted scene intents from the participating presented revision, with bounded scheduler progress until each such intent either publishes or closes. It is not valid to rewrite a started job onto the new scene merely because `host_id` matches. Any future law must include a same-host authored document change, not only an arena-node change.

## Queued Ink presentation laws

The neutral `scene-pointer-owner` fixture and strict schema now include the actual `[2,4] -> [2,3,4]` sibling sequence, unchanged `text -> text` authored content, and changed `text -> pan` authored content. Two Interpreter laws publish, reconcile, seal, and acknowledge the first document, enqueue a real pointer event, and step it only far enough to install an `InkInteractionJob`. They then build and seal the same-host sibling candidate while that job still owns the presented document snapshot.

Both laws require acknowledgement to refuse the candidate while the job is live. The unchanged row then requires the old event and the successor event to each publish one `inkApplyEvents` action. The changed row requires the admitted old text event to publish one action before acceptance, while the successor pan event publishes zero. This distinguishes a bounded presentation barrier from a node rebase that would mix an old document/camera/utility snapshot with new authored pixels. The exact native filter is `test(presented_ink_intent_)`; production remains held until an actual native RED receipt.

The required production seam is an exact presented-revision intent barrier in `acknowledge_presented_input`. A queued or checked-out intent admitted by a window that participates in the sealed witness keeps that witness pending until the FIFO owner publishes or closes the intent. The existing one-unit `drive_scene_interaction_step` remains the only progress owner. Hidden windows and windows absent from the candidate do not block the witness. A partial Ink job is never rebound: its document, selection, utility, camera, drag, rect, node, and tree revision are one old-pixel transaction. Any input arriving while those pixels remain presented joins the same barrier. The implementation must expose a bounded per-window/revision live-intent witness, rather than accepting new pixels and trying to reconstruct part of an already-started job against successor state.

The pre-RED production draft is `🩹️astra-sol-ink-presentation-barrier.patch`. It scans only the fixed 256-slot queue and its one retiring owner, and blocks only an intent whose window is sealed for the exact witness and whose tree revision plus surface generation still equal the presented revision. `git apply --check` succeeded against the pre-RED source, and the draft remained ticket input until the native laws produced behavioral RED.

Native125 produced the required behavioral RED: both `presented_ink_intent_` laws reached their acknowledgement assertions and accepted the candidate while the old Ink transaction was live (lines 1169 and 1208). The selected ownership census ran 64 tests: 59 passed and five failed in 2.289 seconds; the other failures are outside this packet. Receipt: `🗑️generated/astra-runtime/renderer-native125-retained-owner-red/run.log`.

The reviewed barrier is now applied in the Interpreter. It performs no allocation and no rebase: one bounded scan recognizes only queued/retiring transactions from an exact participating presented revision, and acknowledgement retries after the existing scene-interaction lane reaches a terminal state. `rustfmt --emit stdout` parses the current source. Root owns the native GREEN execution.

## Presenter progress integration

Review of the real presenter found that an acknowledgement-only barrier was incomplete. `AppPresentPhase::Acknowledge` previously consumed the prepared packet witness before asking the Shell to promote input. A legitimate `false` therefore faulted after GPU submit, and `AppPresenter::has_pending_presentation` simultaneously prevented the next frame transaction from reaching the only scene-intent drain.

The integrated seam now preflights and advances one bounded intent unit in `AppPresentPhase::Render` before `begin_prepared_present`. A fixed three-state result reports stale, pending, or ready without rebasing the Ink job. The OS host defers runtime apply ingress only through Render, CloseGpu, and Acknowledge, which prevents a new old-pixel intent from entering between the last preflight and the atomic input/packet acknowledgement. Final acknowledgement promotes the exact input witness before consuming the packet witness, and preserves both witnesses on Pending.

The neutral fixture adds current-participant, hidden-window, and stale-revision scope rows. Native laws prove that hidden-window and stale-revision owners do not veto an unrelated exact candidate, that the presenter-facing seam drains the FIFO without acknowledging early, and that the real presenter orders preflight before GPU submit while holding runtime ingress. Each Ink test closes every mounted UI window through `request_ui_document_close` and the bounded close ladder. The widened exact filter remains `test(presented_ink_intent_)`. All six touched Rust files parse through `rustfmt --emit stdout`; native execution is root-owned.

The neutral JSON and draft-2020 schema pass strict Ajv validation through scoped `bun nx exec`; receipt: `🗑️generated/astra-runtime/ink-presenter-barrier/neutral-schema.log`.

A scoped browser-worker rerun after the neutral fixture edit reached 153 tests: 152 passed and one unrelated concurrent source-policy assertion failed because the root worker source now contains `next_deadline`. The ten other files, including the strict fixture/schema reader, passed. Receipt: `🗑️generated/astra-runtime/scene-intent-presentation-neutral/browser.log`. This is fixture validation, not the required native behavioral receipt.

`EngineCanvas::retire_map_interaction_owner` is simpler: its stored owner is component-local resource state, yet it uses full `ScenePointerTarget` equality. After a same-host accepted arena swap, later component retirement arrives with the current node and cannot clear the old stored owner. The correct comparison is the component witness (`document UiNodeId`, host, window lifetime, key, kind, wire surface, component generation). This does not change live Map pointer capture, which still resolves through the exact rebased arena node.

### Prepared Map RED law

`retained_map_same_host_sibling_rebase_retires_engine_interaction_owner` will reuse the proven neutral sibling sequence `[A] → [A,B] → [A,C,B]`. It paints the actual paged retained documents, starts a Map gesture on B while B has its first arena node, accepts the sibling insertion, and proves B kept its exact host while its arena node changed. It then calls the production EngineCanvas retirement seam with B's current accepted owner. The required result is one matching retirement and no remaining Map interaction owner. Current exact `PartialEq` compares the stale arena node and returns false, which is the intended behavioral RED. The test does not clear state manually and does not weaken pointer capture.

### Intent barrier schema and React oracle

The strict neutral draft is `🧪️presented-scene-intent-barrier-draft.json`, with its JSON Schema beside it. It fixes the existing bounds at 256 intent slots and one intent unit per opportunity. Its required case holds one same-host Ink candidate whose authored document changes from A to B while an A pointerdown is pending. The candidate cannot seal until that intent publishes or closes; the intent publishes against A; B becomes presented only afterward.

The mounted React oracle should render an Ink-like host with document A and a synchronous `pointerdown` handler that records the document captured by that render, then requests document B from the parent during the same event. The assertions are ordered: the handler record is A, the DOM still exposes A inside the handler, and the committed DOM exposes B only after dispatch returns. This is the independent browser fact the native barrier mirrors. An async React handler would not be a valid oracle because the real React host handlers do not await inside pointer dispatch.


## Native117 receipt and protocol component witness

Renderer Native117 ran 52 selected laws: 46 passed and 6 failed in 1.322 seconds; Nx completed in 3 minutes 15 seconds. The receipt is `🗑️generated/astra-runtime/renderer-native117-decoder-ready-red/run.log`. All three `presented_editor_` laws passed, establishing the bounded post-acknowledgement Text focus, Ink focus, and Ink clipboard address rebase. The new same-host sibling Canvas camera law passed. The multiple accepted removals law failed exactly because a checked-out camera for a component absent from the accepted presented tree remained live while the one-unit scene retirement queue drained. The three older camera replacement/remount laws remain red.

The production component witness now carries required protocol `UiNodeId` through `UiRetiredComponentScene`, every `ScenePointerTarget`, retained paint slots, and exact retirement conversion. Resource identity compares protocol node, stable host, window lifetime, component generation, key, kind, and wire surface while excluding arena `NodeId`. UI exposes one bundled borrowed `UiComponentSceneWitness` check against its accepted presented tree via the bounded document-node binary lookup. Camera deadlines require that accepted-presented match in addition to retained scene-state ownership. Pointer liveness remains stricter: it still validates the exact arena node and now also validates that node's protocol identity.

The Map fail-first law `retained_map_same_host_sibling_rebase_retires_engine_interaction_owner` is registered against actual retained documents `[2] → [2,4] → [2,3,4]`. It starts B's real Map gesture before the sibling acknowledgement, proves B has one component witness across different arena nodes, retires through the accepted owner, and then requires the old EngineCanvas owner to have been consumed. Production Map comparison remains unchanged pending the native RED.

## Presenter progress dependency audit

The final Render-to-Acknowledge ingress hold does not suspend anything the exact Ink barrier needs to finish:

- `InkInteractionJob::new`, `step`, `scan_one`, `publish`, and `close_step` use only the job's admitted document pages, local scene state, the current retained scene record, and the bounded input action authority. They perform no runtime apply, host I/O, clipboard, image decode, or callback wait.
- Once an Ink action has published, `SceneInteractionIntent.retiring` remains in the ordinary fixed FIFO. The same `drive_scene_interaction_step` called by presenter preflight reaches `process_scene_interaction`, and every `InkInteractionJob::close_step` removes at least one owned item or completes. It cannot wait for runtime ingress.
- Native and browser Ink clipboard streams, pending native clipboard callbacks, and UI image callbacks are separate address owners. They are prepared/rebased at acknowledgement, but no scene intent waits for them before it can publish or close.
- An earlier intent from another window can delay the exact blocking intent, but cannot park it. Presenter Pending schedules another `RESOURCE_READY` opportunity, the fixed FIFO preserves order, and every non-Ink scene branch is synchronous. An Ink head has the same bounded local progress described above. The fixed queue bound remains 256.
- Canvas terminal cancellation can precede the FIFO step, but its authority is limited to 16 gesture slots plus one catalogue hover and each terminal step consumes one cancellation. It also has no external-ingress dependency.

`SceneIntentQueue::retiring` is a separate whole-queue close scratch slot populated only by `close_scene_interaction_step`. Repository-wide Rust call-site inspection finds no production caller; the live window-close ladder uses `close_window_step` and keeps matching intents in the ordinary slots that presenter progress can drain. It is therefore not a concrete Render-ingress dependency. If a production whole-queue close owner is introduced later, it must gain its own presenter progress seam before it may participate in `blocks_presented_candidate`.

No additional regression was added from this audit. The existing `presented_ink_intent_barrier_has_an_independent_bounded_progress_owner` exercises the only live dependency by completing the exact FIFO owner without runtime apply ingress, while the presenter source law proves Pending is rescheduled and ingress stays held through acknowledgement. Adding a fabricated clipboard/image or whole-queue-close dependency would assert behavior that the production Ink transaction does not have.

The final browser-worker census after the Ink fixture/schema and presenter-integration source settled is green: 11 files and 154/154 tests passed; Vitest took 10.06 seconds and Nx took 19.0 seconds with cache disabled. Receipt: `🗑️generated/astra-runtime/browser-worker-final-ink-presenter/run.log`. No generated-source mismatch appeared, so the conditional generation checks were not rerun.

Renderer Native126 executed the final integrated ownership selection: all six `presented_ink_intent_` laws passed. The overall selection ran 68 tests, with 67 passing and one separate Map test failing only its final bounded-window cleanup assertion. Test execution took 2.409 seconds and Nx took 4 minutes 59 seconds. Receipt: `🗑️generated/astra-runtime/renderer-native126-integrated-owner-green/run.log`.
