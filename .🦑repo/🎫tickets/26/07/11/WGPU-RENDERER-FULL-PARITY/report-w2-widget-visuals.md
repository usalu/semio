# w2-widget-visuals — final report

Scope: `ui/wgpu/rs/lib.rs`'s `paint` region only. Did not touch `scene_slots`/`engine` signatures,
`flex`/`reconcile` geometry, `framework/renderer/react/index.tsx`, or
`framework/renderer/wgpu/rs/lib.rs`. Re-grepped before every edit; the file changed under this agent
multiple times mid-task from concurrent sessions (most visibly the SceneHost workstream adding a
`has_scene_host: bool` param to `paint_node`/`paint_stack`/`paint_tree`) — incorporated those as
given, never reverted.

**1. Caret + selection-highlight for a focused `Input`.** `tree::EditState` existed and was fully
wired by `events`, but `paint_input` never read it — not even to show the live buffer. Fixed in
`paint_input` (`ui/wgpu/rs/lib.rs:12897`): new `edit: Option<&EditState>` param (from
`node.state.edit.as_ref()` at the `paint_node` call site; `None` from `paint_control`'s inline-only
site). While focused with `edit.is_some()`: live `EditState::text` (with IME `composition` spliced at
the caret) displays instead of stale `node.value`; a translucent `theme.accent.with_alpha(0.3)` rect
renders a real `anchor != caret` selection; a static 1px `theme.accent` line renders the caret. Added
a small local `edit_selection_bounds` helper rather than reaching across the `paint`/`events` module
boundary for `events::selection_bounds` (private, one-liner).

**2. Select popup — already closed, verified only.** Confirmed `WidgetState.open`/
`toggle_select_popup`/`paint_select`'s `open`/`retained` params are already fully wired from an
earlier W2 pass. No changes needed; did not redo this work. Tree row real layout and Stack
`activate`/`selected`/`drop_action` from that same pass also re-verified still correct.

**3. Systematic 19-kind state pass.** Traced the actual declarative contract
(`framework/core/js/index.ts`'s hand-written flat-field `UiNode` types, which is what
`renderUiControl`/`interpretUiNode` actually read) before changing anything. `presence_overlay`
already uniformly handles disabled/loading/waiting/selected/introducing for all 19 kinds —
untouched, correct. Found most interactive controls (`Input`/`Select`/`Toggle`/`Slider`/
`NumberStepper`/`IconSelect`/`KeyValue`) carry **no** disabled/loading/waiting flags in the
declarative schema at all, so deliberately did *not* invent per-widget disabled dimming beyond what
the prior session already did for `Button` (which does carry `disabled`). The one clear,
evidence-based gap: **focus rings** — only `paint_input` (and `widgets::render_input`) had any
`NodeFlags::FOCUSED` handling; `formControlFocusBorderClass` (`focus-visible:border-accent`) in React
applies to `Button`/`Select`/`Toggle`/`NumberStepper`/(transitively) `IconSelect` too. Ported the
same `border_emphasized`-on-focus convention to `paint_button`, `paint_select`, `paint_toggle`,
`paint_number_stepper` (gained a `flags` param it lacked — both call sites updated),
`paint_icon_select`. Also gave `NumberStepper` a hover tint on its shared minus/plus background
(React's `hover:bg-muted`), approximated at the whole-control level since there's no per-segment
`NodeId`. Deliberately left `Slider`/`Ring` focus (no border primitive to swap) and `IconSelect`'s
real multi-part React UI (mode-select/editor/file-picker — a feature gap, not a state-matching one)
alone.

**4. Stale doc comments fixed.** The `paint` module header's "still default/empty until M5"/
"closed-rest-state only" language (both false now) rewritten to reflect that `events` is fully landed
and both Select's popup and Input's edit buffer paint live state; `paint_section`'s doc comment's
stale comparison to "`paint_select`'s closed-only popup" corrected (Section's own underlying gap —
no `WidgetState`-backed collapse persistence — is still real and left as-is).

**Tests:** extended `paint::tests` with a new `🔖W2WidgetVisuals` subregion, 10 new tests covering
unfocused-input-no-caret, focused-collapsed-caret, focused-selection-highlight, live-buffer-over-
stale-value, and focus-border-swap for Button/Select/Toggle/NumberStepper/IconSelect plus
NumberStepper hover tint.

**Verification:** `cargo check -p ui_wgpu --features engine` clean. `cargo test -p ui_wgpu --features
engine`: **205 passed, 0 failed** (up from 195 at task start; a previously-known golden-JSON failure
is gone too, fixed by someone else concurrently). `cargo clippy -p ui_wgpu --features engine --lib`:
clean, 16 pre-existing warnings, none from this pass's lines. `cargo test -p
semio-framework-renderer-wgpu --lib`: **could not verify** — blocked by a pre-existing, unrelated,
actively-changing compile failure entirely inside `kernel_3d_brepkit` (`Curve2`/`Curve3`/
`surface::Surface` missing `Serialize`/`Deserialize`, 12 `E0277` errors), confirmed via `git log`/
`git status` to be the concurrent `NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT` ticket's in-progress
work (new commits and untracked files appearing mid-task). Retried once after a wait; unchanged. Not
chased further per this ticket's own shared-file-blocker carve-out.

**Files touched:** `ui/wgpu/rs/lib.rs` (`paint` module only — the functions/comments listed above).
