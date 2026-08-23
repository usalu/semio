# Coordinator Independent P5b Live Reactor Reconcile Second Reaudit

Date: 2026-08-23  
Verdict: **REJECT — the four originally reported paths are materially improved, but the replacement still performs unbounded semantic census in one grant, does not maintain exact persistent owner credits, and can deadlock instance close under terminal saturation.**

## Scope

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`
- mounted reactor wiring and the permanent interactivity verifier named by the implementation report

No Cargo, Nx, Wasm, browser, or runtime gate was run while overlapping Rust source packets remain active.

## Confirmed Remediation

- Mounted render reserves a fixed generation slot before transferring the rendered tree.
- Public unadmitted saturation returns the exact tree; mounted retry preserves the original generation.
- Ready, deferred, unadmitted, rejected, surface, and terminal classes are now represented in close and terminal-emptiness checks.
- Mounted terminal saturation retains the job locally instead of deliberately handing it to the unmounted global registry.
- Dynamic component/accessibility/binding/menu/value ownership is now considered before `build_record`, `record.clone`, and `diff_record` mutate candidate state.

These are valid improvements. They are not sufficient for source acceptance.

## Blocking Findings

### 1. Semantic preflight is itself an unbounded recursive worker step

`SurfaceReconcileCursor::step` calls `tree_node_semantic_usage(&node)` as one `TraversePresentation` opportunity (`reconcile.rs:521-592`). That helper walks every select item, key/value entry, binding, row action, data attribute, and recursively every `UiValue::List`/`UiValue::Map` descendant (`reconcile.rs:265-458`) before the cursor can yield. A hostile node with a very large vector/map or deep nested `UiValue` therefore performs arbitrary traversal and recursion in one worker grant. The cap is checked only after that complete scan.

This repairs clone-before-admission but not the governing bounded-opportunity rule. It also leaves a recursive stack path in preflight.

Required repair: retain an explicit fixed-capacity semantic-census cursor and advance one scalar/container/entry per nonzero fuel/deadline grant. Reject as soon as a page/item/byte/depth bound is exceeded. Add low-nonzero-fuel, near-deadline, very-wide, and deeply nested fixtures plus mutations that restore the whole-node helper.

### 2. The claimed exact item/byte authority excludes large live allocations and is released while owners remain

`SurfaceReconcileCursor::new_with_limits` eagerly allocates several `max_nodes` vectors/maps/sets and an `ops` vector with `max_items` capacity (`reconcile.rs:500-515`). `SurfaceReconcileRetained` also allocates a `max_nodes` retirement forest (`reconcile.rs:922-955`). None of those backing capacities enters `SurfaceReconcileUsage`, which starts at zero bytes, even though `Vec<UiPatchOp>::with_capacity(32_769)` alone can materially exceed the advertised byte authority depending on enum size. BTreeMap internal allocation is estimated from `len`, not exact allocation ownership.

More importantly, `SurfaceReconcileJob::take_ready` releases the operation credit before returning the candidate reconciler (`reconcile.rs:1036-1047`). The tracker then retains that reconciler and all of its dynamic records with no persistent credit. A subsequent job owns the old reconciler, source tree, candidate, and patch under only the new operation credit, while the semantic multiplier accounts only the new node's copies. The unadmitted mounted tree can likewise exceed 2 MiB while retained in a fixed slot before its progressive census rejects it.

The fixed slot is therefore not an exact item/byte ownership reservation and the 8 MiB aggregate does not bound live retained reconcile memory.

Required repair: include exact container/backing ownership in admission, keep persistent credits attached to retained reconciler/unadmitted/ready owners until their incremental retirement, and transfer rather than release credit at successful handoff. Oversized producer output must remain under an explicit admitted owner or be produced paged; a slot alone is not byte admission. Add a live aggregate-cap +1 fixture that measures all retained classes after successful publication and during the next generation.

### 3. Instance close can livelock when terminal capacity is full

`PatchTracker::close_step` computes one free terminal target. If a rendered unadmitted owner exists and no terminal slot is free, it returns immediately (`patches/component.rs:369-379`). The rejected-owner branch also returns even when no target exists (`381-386`), and the surface branch returns on the same condition (`388-401`). Those classes are checked before the branch that advances an already-retained matching terminal (`403-408`).

Consequently, a closing instance with a full terminal array plus any matching unadmitted/rejected/surface owner repeatedly returns without advancing the terminal that would free capacity. The all-class fixture does not saturate the terminal array, and the saturation fixture frees a slot externally before exercising rejection; neither discriminates this close-order deadlock.

Required repair: when no target is free, advance one matching close-marked terminal first, then retry owner conversion on a later grant. Add terminal-capacity + matching-unadmitted, +rejected, and +surface close fixtures that reach terminal-empty without external checkout.

### 4. Generation exhaustion reuses `u64::MAX`

`begin`, `retain_unadmitted`, `reserve_mounted`, and `mark_rejected` use `checked_add(1).unwrap_or(u64::MAX)` (`patches/component.rs:154-162, 208-230, 314-329`). Once exhausted, every later owner receives the same generation. That defeats the packet's ABA and original-generation guarantees.

Required repair: fail closed and return the exact owner/reservation on exhaustion. Add a near-maximum generation fixture and verifier mutation.

### 5. Public Drop handback is lossy at registry saturation

`handback_surface_reconcile` inserts only when one of the fixed global terminal slots is empty; otherwise the passed `SurfaceReconcileRetained` falls out of scope and recursively drops (`reconcile.rs:895-902`). Public `SurfaceReconcileJob`, rejected, and terminal Drop paths all rely on this helper. The mounted route avoids this registry during local saturation, but the public lifecycle contract still silently loses the cap +1 owner.

Required repair: make terminal handback capacity part of admission or otherwise provide a lossless owner-preserving saturation protocol. Add public ordinary-drop cap +1 and generation-reuse fixtures.

## Reaudit Gate

P5b remains source-rejected until all five findings have discriminating fixtures and permanent verifier mutations, scoped formatting/verifier/diff gates pass, and a fresh independent source audit accepts the repaired ownership and liveness model. The distinct `plugin_render` producer refactor and serialized executable timing matrix remain separate open gates after source acceptance.
