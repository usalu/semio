# Checkpoint 16 Dedicated Physical Docking

The existing eight-case schema-first adapter now runs against the sealed WGPU16/React9 browser artifacts. Each case owns a fresh browser context. Root added durable failure receipts, screenshots, console/error capture, WebGPU Chromium flags and ticket-local output selection before running it.

## Harness corrections established from real runs

- Initial WGPU invocation read the beacon before boot; its eight immediate failures are harness failures. The adapter now waits for the armed beacon and stable mounted dock/camera geometry before physically dismissing the tour, then requires the tour controls to retire.
- Drag promotion removes the source tab and changes the target stack bounds. A saved pre-drag tab rectangle is stale. The adapter now re-resolves the same target window after promotion and moves to its current drop zone before release.
- React Display branch rows are selectable; their gutter disclosure button controls expansion. The adapter now clicks that actual button, rather than treating a selected row as expanded.
- WGPU `chrome.surfaces` lists visible bodies and omits inactive tabs. It cannot be the identity authority for all dock tabs. The adapter now matches published tab hits against the retained `dumpStructure().windowIds` registry; this preserves inactive identities without inventing a window or ignoring the real tab roster.
- React's orthographic template encodes cardinal `view: plan` (existing engine-contract law agrees), which yields the required top camera. The producer assertion now uses that canonical spelling and still requires the resulting camera direction and up vector.

## Executed evidence

Run C: React passes all four splits, merge, reorder and Escape cancellation. Its remaining template failure is the corrected `plan` spelling assertion. WGPU passes all four splits and Escape cancellation. Its merge/reorder failures occurred in the inactive-tab identity adapter and are rerunning after the correction. WGPU template expansion currently exposes no published physical disclosure; this is under diagnosis and is not accepted as equivalent.

Run D completes: React template configuration passes with actual orthographic direction `[0,0,1]` and up `[0,1,0]`; WGPU merge and reorder pass after the complete tab identity correction. WGPU template configuration still fails because the published Parallel branch has no physical disclosure hit. Combined C+D establishes all8React cases and7WGPU cases; it does not establish8/8paired parity. The WGPU failed-state snapshot is saved alongside its screenshot. Evidence lives in `🗑️generated/astra-runtime/checkpoint-16-dock-{c,d}` and renderer-specific logs.

The WGPU failure snapshot publishes a section at852.8and three child rows at876.8/900.8/924.8, while the immediate failure screenshot still shows a compact closed-looking panel near the footer. This may include a presentation lag because failure capture follows first child-hit publication; it is not alone a settled paint mismatch. The separate settled full16Drivers receipt does prove that an expanded Up-flow section header hit overlaps its Save button. Sol is tracing both retained Tree paths and the next test must preserve exact target geometry.

These runs do not contain the pending General/driver publication or text-owner repairs. Artifact activation remains owned by root and is deferred until physical probes finish.
