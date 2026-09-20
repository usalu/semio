# Terra Publication and Settings Audit

Read-only source audit of the Sol publication-refusal packet against the current WGPU shell and the in-progress generic disclosure path. No build, native test, or browser run was performed.

## Findings

### P1 — A live refused window head-of-line blocks unrelated successful publications

`complete_window_topology_refresh` requeues a live, document-missing publication at the front of `window_topology_publications` and immediately breaks ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6732)). A later publication whose document rendered successfully is never inspected during that refresh, so its accepted transfer command remains in the bounded journal. The next forced full refresh first retires all pre-existing window documents ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6843)), repeatedly invalidating that later window’s lease while the first window consumes its retry budget.

The cancelled-window branch does drain: a removed window discards its owned action and continues to the next publication ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6727)). The blocking condition is specifically a still-live refused window.

**Negative vector.** Publish `main-2` and `main-3` through the actual Display drag producer. Script `main-2` to refuse once and let `main-3` render. After the first full refresh, `main-3` must dispatch its one `shell.windowSplit` without waiting for `main-2`’s retry or terminal retirement; subsequent retries for `main-2` must not duplicate or revoke `main-3`’s command. The new fixture only scripts `main-2` ([WGPU shell-input tests](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:168)), so it cannot expose this ordering failure.

**Repair direction.** Process every publication that has a current document in a refresh, retaining only unresolved entries for retry. If retry entries cease to be FIFO, bind final journal removal to a publication token rather than a naked `pop_front`.

### P1 — Journal admission can fail after the dock mutation has committed

`finish_dock_drag` checks only publication-entry capacity before `dock.apply_drop` ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12526)), mutates and registers the topology publication ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12529)), then attempts bounded action publication ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12561)). `queue_window_topology_action` records `Refused` but returns the admission error ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6661)). Thus an item- or byte-credit failure can report an unsuccessful drag after its dock topology is already committed, with neither rollback nor a producer-level terminal outcome asserted.

**Negative vector.** Fill `window_topology_actions` to its item or byte credit limit while leaving publication-entry capacity. Drive a real captured Display handle through pointer release. Assert that the rejected release leaves the dock unchanged, or that the API instead returns a documented durable refusal while its committed topology remains observable. The present credit test calls `admit_window_topology_publication` and `queue_window_topology_action` directly ([WGPU shell-input tests](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:339)); it does not exercise `finish_dock_drag` and cannot prove the producer boundary is atomic.

**Repair direction.** Reserve and construct the required action before applying the drop, or make the post-mutation refusal an explicit, durable result with a matching topology fault contract. Exercise it through the ProgramBridge/Display fixture rather than a queue helper.

### P2 — The disclosure test does not cover a second toggle while layout is in flight

The generic retained-section regression test clicks open and closed consecutively, then starts layout work ([retained-section-collapse tests](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧪️tests/📂️retained-section-collapse/🦀️.rs:97)). It therefore never places a layout job or session between the two dirty transitions. The production path now relies on root dirty state and a queued-layout epoch after `toggle_disclosure` ([tree](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs:444)) instead of the removed router layout-change field; stale queued entries are recreated by `next_layout` ([engine](../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1323)). That lifecycle has no regression coverage.

**Negative vector.** Open a generic `Section`, advance the layout engine until its new generation has checked out a job/session, close the same header before that work completes, then drain layout and frame work. Assert final closed reachability, no child hit registration, and no stranded queued/dirty layout state. Repeat with close→open during the same in-flight window.

**Repair direction.** Extend the retained generic test with scheduler steps between toggles; direct `UiTree` state assertions alone cannot validate the root-dirty replacement.

### P1 — The Settings footer-close overlap lacks an actual hit-routing regression gate

The only Settings panel-bound test calculates a nested tab-row rectangle and compares its lower edge with a formula for the footer root toggle ([settings-general-layout tests](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs:65)). It does not render the Shell, collect actual `HitKind::Toggle` and `HitKind::PanelTab` records, or press the footer root control. Panel rows deliberately omit the root ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22876)), while the root is painted in the footer with a separate toggle hit ([Shell WGPU](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23753)). The reported Browser11 close failure consequently has no native guard.

**Negative vector.** Render BottomRight Settings with `framework.settings.general` active. Obtain the actual footer `framework.settings` toggle rectangle and the panel `framework.settings.general` tab rectangle from the rendered hit records; assert they do not intersect. Press and release the footer root center and assert the anchor closes, no panel tab is selected, and no dock-tab drag starts. Repeat while a fresh document layout is pending.

**Repair direction.** Make the panel/tab placement derive from the rendered footer-root exclusion rectangle, and cover the real hit ordering in the Shell fixture. This turns the Browser11 observation into a stable native regression.
