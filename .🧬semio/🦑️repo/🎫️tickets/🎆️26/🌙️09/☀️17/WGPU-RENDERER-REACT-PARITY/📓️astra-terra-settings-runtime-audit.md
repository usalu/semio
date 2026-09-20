# Settings Runtime Audit — WGPU and React

## Scope and evidence

This is a read-only source and runtime-evidence audit. No source was edited and no build or test command was run for this audit.

The runtime checkpoint supplied for this audit establishes all of the following:

- React `Settings → General` exposes the Appearance control.
- WGPU has no visible state change after a 15-second operation and 15-second settle, although its generation rises from 17 to 34.
- The worker console, excluding the duplicate page stream, records the actual down and up at `x=1402.379, y=798.4` as `PanelTab framework.settings.general` at 32620 and 32653.
- The WGPU settings leaf shown after that input is blank and its manual canvas reports the red `puzzle3d.panel.settings…` validation.

Thus this is neither an absent-hit-handler inference nor a stale-coordinate explanation. The input really reached the General tab target. The source trace below explains why that valid target selects the app Settings leaf instead.

Relevant sources:

- [WGPU Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs)
- [React Chrome panels](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx)
- [React Shell host](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx)
- [Theme and accessibility WGPU laws](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs)
- [UI action value contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs)

## Confirmed selection defect

The WGPU pointer route is present and reaches selection:

1. `handle_shell_hit` recognizes `PanelTab framework.settings.general` through `panel_tab_anchor`.
2. It calls `select_panel_tab`.
3. The shell-owned-leaf branch of `select_panel_tab_step` calls `toggle_anchor_tab(anchor, tab_id)`.
4. `toggle_anchor_tab` calls `ShellDock::reconcile_path(anchor, &[tab_id.to_string()])`.

`reconcile_path` requires a root-to-leaf path. It begins at the selected anchor's root tab list and only descends after the current path component has matched. A singleton framework leaf such as:

```text
[framework.settings.general]
```

cannot match the bottom-right root, which is:

```text
[framework.settings]
```

When no component matches, `reconcile_path` returns `default_path(anchor)`. The default bottom-right path is the framework-settings branch plus its first child.

`ShellState::default_dock` constructs that branch by placing registered app Settings leaves before framework leaves:

```text
[framework.settings, puzzle3d.panel.settings, framework.settings.general, …]
```

For the checkpoint's Puzzle3D app, the resulting fallback is therefore:

```text
[framework.settings, puzzle3d.panel.settings]
```

This matches the observed red Puzzle3D Settings validation exactly.

The rest of the rendering path faithfully renders this incorrect selection:

- `ShellAnchorState::active_tab` takes `path.last()`.
- `refresh_ui` creates shell-owned settings documents, including General and the Appearance select.
- `render_panel_step` looks up the document for that active leaf and paints it.

So the General document producer exists. The selected path reaches the app leaf before document lookup, which is why WGPU paints the app Settings outcome rather than General.

React has the required semantic behavior: its Shell host resolves a panel tab with `findPanelTabPath` and applies the full nested path. React's General tree contains the Appearance select. WGPU's documentation likewise describes `ShellDock::locate` as the twin of React path lookup, but the programmatic WGPU selection route does not use it.

### Bounded implementation

In `toggle_anchor_tab`, resolve the requested id from the current live dock with `ShellDock::locate(tab_id)`, verify its anchor, and reconcile/apply that full root-to-leaf path. For General, the value supplied to path reconciliation must be:

```text
[framework.settings, framework.settings.general]
```

Keep the current close behavior for an already-active tab. A small `resolved_tab_path(anchor, tab_id)` helper is appropriate if it centralizes the anchor validation and avoids future singleton-leaf calls. The fix belongs in the selection route; changing pointer geometry, delaying input, special-casing Puzzle3D, or patching document construction would only hide the cause.

## Accessibility law 24

The native renderer11 report records:

```text
shell::theme_editor_and_accessibility_tests::accessibility_activation_updates_settings_switch_and_active_tab_projection
expected true, got false
```

Its accessibility activation route calls `handle_accessibility_event`, which validates the shell target and delegates to the same `handle_shell_hit` route. The General half of this law therefore has the same direct cause: after accessibility activation, `chrome_accessibility_selected` checks whether the selected id is in the stored anchor path, but the fallback path contains `puzzle3d.panel.settings`, not `framework.settings.general`.

That law currently has two similarly shaped assertions: one for the framework-settings switch and one for General selection. The branch-id switch should use a valid root path, while the General leaf uses the invalid singleton path. The terse failure output does not identify which assertion emitted it. Add assertion-specific diagnostic text, but treat the General assertion as a confirmed consequence of this defect rather than treating the entire law as proof that both paths share a cause.

## First-publication panel-rectangle transient

The source independently explains the first panel rectangle moving from approximately `y=51.2` to `y=787.2`:

- `anchor_content_height` reads retained content height for the active leaf.
- Before the document has an accepted retained layout, it is absent.
- `anchor_panel_rect` then falls back to the full anchor band.
- Once retained content height exists, that height is clamped to the band and the bottom anchor is placed with the resulting slack, moving it down toward the bottom.

This is a source-defined pre-layout-to-measured-layout transition. It warrants a layout-stability law, but it does not explain this activation outcome: the exact supplied pointer coordinates were recorded against General at input time. The hit record rules out a stale target for this event.

## Independent theme producer failure 27

Renderer11 failure 27 reports that the all-open theme leaf fails at row 528 when publishing `setThemeAppearancePaint`:

```text
measure action 'setThemeAppearancePaint' args exceed the retained contract:
data did not match any variant of untagged enum UiValue
```

The source producer emits a valid four-scalar action map:

```json
{"appearance": "...", "group": "...", "paint": "...", "channel": "hex"}
```

The UI contract explicitly permits four-entry argument maps. The action therefore is not untagged or semantically malformed.

`UiValue` uses a bounded global page/slot arena. Dropped values enqueue their slots for return; they become available again only when the retirement page is closed. The all-open theme test repeatedly builds temporary `panel_ui_records` across scroll rows without driving that retirement path. Near row 528 it exhausts the arena, and Serde reports the admission failure as the untagged-enum mismatch.

This is separate from the General selection defect. It proves a test lifecycle defect and exposes a possible production pressure path: `refresh_ui` constructs records for shell-owned leaves before unchanged-publish suppression can avoid ingress. The evidence does not establish that this exhaustion caused the supplied General click failure, whose wrong selected leaf is already fully explained by path fallback.

### Bounded follow-up

First make the theme law retire each temporary projected document/record set and drain the UI-value retirement page before the next viewport, then assert that arena headroom returns to its baseline. Use the real lease/retirement route rather than enlarging the arena, dropping action arguments, or accepting the Serde error.

If a production stress law still shows accumulation, avoid rebuilding unchanged shell-owned documents before action-value allocation. Reuse a source-semantic cached document or publish only active/dirty leaves, with a revision key that can be evaluated without constructing fresh `UiValue` arguments.

## Required laws after the selection fix

1. **Pointer nested-leaf law.** Construct a bottom-right framework-settings branch with an app Settings leaf before General. Activate `framework.settings.general` through the actual pointer route. Require a visible anchor, exact path `[framework.settings, framework.settings.general]`, General as the active tab, and a General document containing `framework.settings.appearance`. Forbid `puzzle3d.panel.settings` as the active leaf.
2. **Accessibility nested-leaf law.** Activate the same General chrome node through `handle_accessibility_event`. Require `aria-selected=true`, the exact full path, and the General panel projection. Give the switch and leaf assertions distinct failure labels.
3. **Layout-independent activation law.** Run the pointer and accessibility laws both before retained measurement and after the panel has a retained content height. Selection must be identical in both states. This tests against a coordinate-dependent regression without misclassifying the documented geometry transition as the current fault.
4. **Differential React law.** Use the same dock fixture in React and WGPU. Both must select General and expose Appearance. The React full-path resolver is the behavioral reference, not a shared implementation.
5. **Theme retirement law.** Sweep the all-open theme rows through actual record publication and retirement. Require every action map to remain representable, no arena-headroom exhaustion, and restored headroom after retirement.

## Acceptance evidence

After the path fix, rerun the checkpoint with the worker-only console retained. The actual General down/up must still hit `PanelTab framework.settings.general`; structure and accessibility must identify General as active; the canvas must contain the General Appearance control; and no Puzzle3D Settings validation may appear for that activation. The rectangle transition may still occur during initial publication, but it must not alter the selected leaf in either pointer or accessibility activation.

The canonical checkpoint-8 artifact remains an observation of the prior runtime. This audit recommends a source-stable fix and laws for a subsequent validation run; it does not rewrite that artifact.
