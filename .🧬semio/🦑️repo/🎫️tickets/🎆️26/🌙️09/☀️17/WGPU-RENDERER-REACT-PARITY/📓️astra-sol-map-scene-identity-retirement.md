# Tiled Map Retained Scene Identity Retirement

## Boundary

A Map gesture belongs to the exact retained scene tuple: window id, window generation, node id, node key, component kind, and surface id. An ordinary publication with the same tuple keeps the active gesture and persistent Map host. A key, kind, node, or surface replacement and a removed node retire the old gesture without publishing PointerUp or selection. The persistent MapSyncCache, camera, and tiles survive a same-surface Map successor.

The identity-aware observation point is `UiTree::step_document_reconcile_preserving`. Mount has the old retained node before replacement; Retire has the old node before removal. Document lease replacement is too early because the old retained tree still exists. Shell document retirement is also used for normal successor publication and cannot distinguish a surviving owner.

The UI seam must use a fixed-capacity, backpressured retirement ledger. Reconciliation must yield before mutating the tree if the ledger cannot accept the old identity. Interpreter must take a retirement while it owns UI state, release the `UI_ENGINE` borrow, then execute the Map callback. It must remain in reconciliation while any retirement is pending, so the successor hit registry cannot publish ahead of retirement. This ordering also avoids the nested UI lock deadlock observed in Renderer Native78.

## Neutral and React laws

The closed v1 fixture covers a same-identity refresh, keyed replacement, kind replacement, and removal. The neutral law validates the schema, computes retirement from the full scene tuple, and fixes persistent-host expectations.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-react:test-long' -- '../../../../🧪️tests/♻️tiled-map-gesture-lifecycle/🟦️.ts' --silent=false --reporter=verbose
```

Receipt: one file, 3/3 passed in 567 ms; Nx completed in 1.2 seconds.

The actual React component oracle renders `TiledMapHost`, begins a gesture through its real `MapRenderer`, then rerenders first with the same key and then with a changed key. The same key retains one active renderer. The changed key disposes the old renderer and creates a fresh inactive renderer.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run '@semio-tech/framework-renderer-react:test-long' -- '../../../../🧱️elements/🧭️TiledMapHost/🧪️tests/🧩️component/🟦️.ts' --silent=false --reporter=verbose
```

Receipt: one file, 23/23 passed in 4.95 seconds; Nx completed in 5.7 seconds. An initial run exposed and corrected the fixture's relative import path before this green receipt.

## Native fail-first registration

The native laws publish real `UiDocumentLease` values and drive actual `render_ui_document_step` reconciliation. They introduce no production symbol dependency:

```text
retained_map_same_identity_refresh_preserves_the_active_gesture
retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input
```

The first law expects the current same-identity behavior to remain green. The second should fail at runtime on current production because `SceneSurfaceState` is keyed only by surface id and has no retained identity retirement path. It separately asserts that replacement/removal publish no PointerUp or selection and that a successor can start a fresh gesture. Root owns the Cargo/Nextest receipt.

## Native receipts and implemented typed seam

Renderer Native82 observed the intended runtime failure after real document publication: `retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input` failed because the old keyed Map gesture remained active after generation 1 reconciled to generation 2. This was a behavioral RED rather than a missing-symbol compile failure.

Renderer Native84 then ran 1,281 tests: 1,275 passed, six unrelated Canvas/foreign-cancel/inventory tests failed, and zero were skipped. Both Map retained-lifecycle laws passed. The raw receipt was `🗑️generated/astra-runtime/native84/run.log` when inspected; generated output is subject to the ticket cleanup owner, so the census is recorded here.

The implemented Map pointer-down seam requires the exact `ScenePointerTarget`; there is no optional or bare-surface path. Scenes records an exact interaction owner on the drag and MapHost active interaction. A dequeued UI retirement invokes `retire_tiled_map_scene_identity` with that tuple and clears gesture-local drag, marquee, and hover state only when the stamp matches. It locally cancels the matching MapHost interaction without camera settlement, PointerUp, or selection and leaves MapSyncCache, camera, and tiles intact.

The identity-aware UI laws were also registered:

```text
same_key_scene_kind_replacement_emits_the_exact_old_identity_once
a_full_scene_retirement_ledger_yields_before_mutating_the_old_node
```

The first full UI run executed 655 tests: 652 passed and these two plus the earlier retirement fixture stopped at decode because the fixture used the invalid surface kind `world3d`. The canonical spelling is `world-3d`; the fixture was corrected without adding an alias. That run measured `UiSurfaceRegistry` at 161,944 bytes versus the prior 161,912-byte fixture, an exact 32-byte increase for the fixed retirement ledger. A later pointer-capture layout change still requires a fresh exact inventory receipt, so 161,944 is the retirement-only measurement rather than a final current-tree claim.

Native84 measured `EngineSurfaceRegistry` at 75,816 bytes versus 75,720 bytes. The 96-byte increase is the exact retained Map host identity stamp and the inventory fixture now reflects it.

## Ownership boundary

Map pointer down must receive the required exact `ScenePointerTarget`; there is no optional or bare-surface compatibility path. Scenes records an exact interaction owner on the drag and MapHost active interaction. A dequeued UI retirement invokes `retire_tiled_map_scene_identity` with that same tuple and clears gesture-local drag, marquee, and hover state only if the stamp matches. It locally cancels the matching MapHost interaction without camera settlement, PointerUp, or selection and leaves MapSyncCache, camera, and tiles intact.

The renderer call site is root-owned. Its required signature is `tiled_map_pointer_down_into(owner: &ScenePointerTarget, controller_id, bounds, ...)`. Renderer passes its already-validated captured target. Interpreter converts the neutral UI retirement row to the same target type only after dropping its UI borrow. Pointer ownership and hit-registry integration remain root-owned.

Window closure and persistent host token retirement are separate from this bounded replacement/removal packet and are not claimed by these receipts.
