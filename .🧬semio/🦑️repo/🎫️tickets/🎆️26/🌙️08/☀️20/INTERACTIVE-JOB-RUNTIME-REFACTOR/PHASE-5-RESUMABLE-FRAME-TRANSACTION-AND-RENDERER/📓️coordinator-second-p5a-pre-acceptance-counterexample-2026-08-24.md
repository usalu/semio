# Coordinator Second P5a Pre-acceptance Counterexample

Date: 2026-08-24  
Verdict: **RED — the callee-graph remediation is still not source-acceptable.**

## Counterexample 1: Whole-subtree Calls Remain One Grant

`ShellState::render_chrome_step` advances only one named phase, but a phase is not a bounded semantic
unit. Its `MainWindow`, `LeftPanel`, `RightPanel`, `Navbar`, `TutorialBar`, `Footer`, `Overlay`, tree
drag, tutorial gesture, and error phases each call the corresponding complete renderer once. In
particular, `render_main_window`, `render_left_panel`, and `render_right_panel` retain their existing
whole-subtree traversal/allocation call graphs. Adjacent helpers such as `left_tabs` and `right_tabs`
still clone/collect complete dynamic panel-tab collections and the panel-tab artifact search remains
recursive.

The P5a contract requires at most one presentation node, retained child opportunity, scalar, owner,
or page per worker grant. Splitting one complete chrome build into ten complete subtree calls does not
satisfy that gate, and a deep/wide main window or panel can still cross the 8 ms ceiling within one
`FrameTransaction::step`.

## Counterexample 2: Whole Atlas Backing Is Allocated Before Page Work

`AppRuntime::frame_before_input_step` creates `icon_pixels` with
`Vec::with_capacity(self.icons.pixels.len())`, and `frame_after_input_step` does the same for the full
glyph atlas. The following opportunities copy only 16 KiB slices, but the initial opportunity already
reserves the complete dynamic backing without a fallible P5a byte/page admission authority. On
close, repeated `truncate` calls reduce logical length only; the entire capacity remains allocated and
is finally freed when the `Vec` owner drops. This is post-allocation accounting and bulk backing
retirement, not fixed-page ownership.

## Counterexample 3: Map Cleanup Clones Uncredited Keys

Shell `FrameSetup` repeatedly uses `keys().next().cloned()` before removing tooltip, element, and
widget-map entries. Although only one entry is selected per call, an unbounded dynamic key backing is
cloned before any exact byte admission. The old entry and cloned key may then both be destroyed in
the same opportunity.

## Verifier Gap

The 33-mutation verifier rejects restoring the original wrapper calls and two named chrome-child
calls in one outer phase. It does not recursively inspect or mutate the named child bodies, prove a
node-level retained cursor through main/panel/widget rendering, reject whole-capacity atlas
preallocation, or prove fixed-page backing release. Baseline success therefore remains a false
positive.

## Required Remediation

- Replace the remaining whole subtree render calls with retained node/widget/tree cursors. One grant
  must advance one admitted node/scalar/owner/page or one independently bounded child opportunity.
- Replace atlas `Vec` candidates with actually fixed admitted pages; reserve page/item/byte/process
  credit before allocation/transfer and release one page backing per close grant.
- Remove key cloning from cleanup. Retain exact admitted key owners or use fixed generation-tagged
  slots with identity-preserving refusal.
- Extend hostile mutations into the live bodies of every mounted child and atlas/page authority.

No Cargo, Nx, Wasm, browser, or timing command was run while overlapping Rust source packets were
active.
