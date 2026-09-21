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

`UiRetiredComponentScene` also reconstructs the host downstream instead of carrying the exact host stored in the retired node. Root owns the narrow metadata correction that adds the actual host to the retirement record and updates its initializers/consumers.

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

The fixed retirement capacity remains unchanged. Backpressure pauses the reconcile/resolution cursor; no retirement is dropped and no bound is raised.

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

No native test receipt exists yet. Native106 is compiling the pre-repair snapshot for the independent focus/clipboard retirement packet.
