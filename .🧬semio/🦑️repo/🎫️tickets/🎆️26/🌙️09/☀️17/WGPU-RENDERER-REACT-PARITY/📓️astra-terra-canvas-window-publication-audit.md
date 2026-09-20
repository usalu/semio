# Canvas and Window Publication Audit

**Scope.** Read-only audit of the current WGPU Shell, Scenes, Interpreter, ProgramBridge, shared fixtures, Native18 log, and the later Native19 status supplied by root. No build, browser, Cargo, source, or test command was run for this audit.

## Verdict

The ordinary transfer path has the right ordering: dock mutation admits one bounded topology note, owes a Full refresh, the settle lane refreshes before draining deferred guest work, and the succeeding frame dispatches the note. The current test-only ProgramBridge render fixture proves that successful path through `render_with_document`; its earlier manual paint/queue acknowledgement is now supplemental rather than the sole evidence.

One production correctness gap remains: **a window-body render refusal is reported as a surface fault but still releases the topology journal.** The queue may therefore emit `shell.windowMove` for a new dock instance that has no retained `window_ui` document. This is exactly the failure mode the prior empty-body probe made consequential. Native19's successful runtime journey does not cover this refusal branch.

Native19's current receipt is **1,172/1,173**, with the earlier physical-hit and duplicate-Canvas-release red checks passing. The remaining failure is the Display test's World3d retirement assertion (drop before terminal empty at `World2133`). Do not use it as a green host-lifetime receipt until the post-Native19 Chrome cleanup reruns it.

## Actual Production Paths

### New window and publication

1. The retained Display transfer gesture applies the dock drop, calls `owe_window_topology_refresh`, and queues one bounded `noteShellCommand(shell.windowMove)`.
2. `settle_pump_step_inner` gives `window_topology_refresh_owed` priority, so `refresh_ui(Full)` runs before `drain_deferred_actions`.
3. `refresh_ui` computes the live dock roster, retires absent retained documents, then calls `ProgramBridgeEntry::render_with_document` for each wanted body and inserts a successful lease in `window_ui`.
4. The next settle step drains the deferred journal to the guest. A guest refusal is then consumed once; the fixture's action counter verifies that normal success/refusal boundary.

The `cfg(test)` fixture is a useful Shell/ProgramBridge seam: `render_with_document` reaches the fixture before either JS or Wasm backend, so it proves the host scheduling and document ownership route without a browser. It is not a real guest exchange oracle. The test fixture constructs a lease directly, while the production Wasm path sends `SurfaceVisible` and retained advance events to a `KernelClient`.

The direct `openDisplayWindow → open_display_window` path mutates the same dock and owes the same Full refresh, but does **not** enqueue `shell.windowMove`. That is coherent only if the journal is deliberately an audit of transfer gestures, not of every topology mutation. Its current generic “topology journal” naming and comments imply the latter. Make this contract explicit and test either:
- every accepted new-window mutation journals once after publication, or
- only transfer gestures journal, while direct Display opens are intentionally journal-free.

### Refusal defect — P1

`refresh_ui` removes a prior document before rendering a wanted window. When `render_with_document` returns `Err`, it records a surface fault and continues; the outer refresh returns `Ok(())`. Its final unconditional `complete_window_topology_refresh()` clears the owed flag and transfers all queued notes into `deferred_actions`.

The priority settle branch retries only when `refresh_ui` itself returns `Err`. It therefore cannot distinguish this per-body rejection from a complete first-body publication.

**Bounded implementation packet:**

- Attach the required newly created window identities to each admitted topology entry, or retain a per-refresh required-ID set with the owed queue.
- Let refresh return a publication outcome that identifies every successfully inserted required `window_ui` lease.
- Release only entries whose required IDs have successfully published in the current live roster. On a body fault, keep the entry and the topology debt; re-owe the appropriate refresh.
- Bound permanent guest refusal with an explicit terminal outcome/fault for that entry. It must either remain retriable with a stated ceiling or become a visible one-time topology-publication refusal; it must not silently dispatch a success-shaped note.
- Change `queue_window_topology_action` from debug-only `()` admission/refusal to a caller-observable outcome. Today a queue-credit refusal leaves the dock mutation live but produces no journal, which is zero-or-one behavior rather than an explicit exactly-once/refusal contract.

This is a Shell/ProgramBridge packet. It should not alter Canvas event routing or the World3d retirement mechanics.

## Canvas Authority and Retirement

The Canvas implementation now has a coherent ownership key:

- `CanvasGesture` holds `window_id`, `surface_id`, and `document_generation`.
- Button and move paths use `canvas_gesture_matches`; a different generation on the same window/surface calls `cancel_stale_canvas_authority` before accepting the incoming event.
- `close_dock_window` marks both pointer and catalogue authority for cancellation. `Interpreter::drive_scene_interaction_step` checks live UI generation first and gives terminal emission priority over later scene intents.
- A terminal action clears authority only inside `publish_with`. A reservation failure therefore preserves the cancellation request for retry instead of losing the terminal. Once published, a duplicate release finds no matching authority and is inert.

This satisfies the requested generation-owned release rule: an old generation is cancelled; an unmatched release on the replacement generation does not end a gesture it does not own. The completed Native19 duplicate-release check is supporting evidence. React's raw Canvas host can still forward an unpaired pointer-up for legacy host behavior, so this new cross-renderer ownership rule needs a shared oracle rather than a text comparison alone.

Window closure removes the dock instance first, requests Canvas cancellation, then the refresh/retirement lane releases retained documents. World3d active state is removed during live-surface synchronization and moved to bounded retirement. The remaining Native19 World3d terminal-drain failure means this full close-to-empty chain is not yet accepted.

## Fixture and Test Gaps

The shared `Canvas2dHost/input-contract` fixture covers normal down/cancel/catalogue/drop/double-click wires. It does not name a document generation transition, close/reopen lifecycle, action-credit refusal, or exact terminal ordering against a successor document. The neutral `window-lifecycle-template-drag` fixture checks MIME, layout insertion, React refresh partitioning, and retirement-token intent, but it has no producer outcome sequence and cannot prove WGPU acknowledgement behavior.

The current WGPU test has three strengths:

- it reaches the real Display transfer handle and capture path;
- it proves the new ProgramBridge fixture creates a retained body before the admitted journal is dispatched;
- it proves one guest action refusal is consumed once after a successful body.

It still needs an independent negative producer law. The manual `publish_surface_records` plus direct `complete_window_topology_refresh` setup must not be used to prove it.

Add one schema-first fixture extension shared by TypeScript and Rust:

```json
{
  "publicationOutcomes": [
    { "opened": "main-2", "render": "refuse", "journal": "held", "surfaceFault": true },
    { "opened": "main-2", "render": "document", "journal": "dispatch-on-next-settle" }
  ],
  "canvasAuthority": {
    "generationOne": "down → replacement → cancelled canvasPointerUp once",
    "generationTwo": "unmatched release → no action",
    "close": "close → terminal before successor input",
    "creditRefusal": "terminal remains owed until one publish"
  }
}
```

The Rust law should inject a render refusal through the existing `cfg(test)` ProgramBridge seam, assert no deferred journal and no `window_ui` lease, then recover with one document and assert exactly one dispatch. A React-mounted oracle should exercise the corresponding close/replacement gesture and decode the same fixture action sequence. Keep the World3d retirement assertion as a separate final check after terminal drain; it currently remains red.

## Handoff

1. **Sol Shell/ProgramBridge:** make topology publication acknowledgement conditional on the actual initial body lease, expose journal admission/refusal, and add the refusal/recovery fixture law.
2. **Chrome lifecycle owner:** rerun the Native19 World3d retirement case after its test-only cleanup and repair only the real owner leak if it persists.
3. **Canvas owner:** add the generation replacement, close, and credit-refusal vectors to the shared fixture and both renderer oracles. The production Canvas routing itself should remain unchanged unless a vector disproves the current authority behavior.
