# Accepted-Frame Chrome Accessibility and Localization Audit

Read-only source audit on 2026-09-26. It inspected the current React and WGPU sources plus prior ticket reports. It made no production edits, did not run a build or runtime probe, and does not claim that a repair has passed.

## Confirmed cause of the observed labels

The browser mirror is a projection consumer. It copies `node.label` into `aria-label` in `🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`; it must not translate control identifiers. The producer is therefore the only valid repair point.

`ShellState::acknowledge_presented_input` in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` promotes the accepted hit list, then derives and publishes `chrome_accessibility_nodes(input.hits())`. This is the correct accepted-frame boundary: the accessibility dump and hit registry refer to the same presented frame.

`chrome_accessibility_nodes` asks `CHROME_CONTROL_NAMES` for each hit name and otherwise calls `humanize_control_id`. The fallback only splits a dot-separated final segment and camel case; it neither resolves an instance title nor applies a locale. Consequently a raw canvas hit `puzzle3d-main-top` becomes `Puzzle3d-main-top`.

Three current hit producers bypass the existing name collector:

| Painted control | Current producer | Visible-label authority | Current AX consequence |
|---|---|---|---|
| Live window body | `ShellState::render_main_window_step` registers the `ScrollRegion` with raw `window_id` | `ShellState::dock_chrome_maps()` resolves an instance title from `default_layout` or `layout_override`, then the localized window-kind label | `puzzle3d-main-top` and `puzzle3d-main-perspective` are announced as raw identifiers although their painted tabs say `Top` and `Perspective`. |
| Dock tab, focus/unfocus, close, and drag | `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` registers each hit directly through `DockRenderContext.input` | `dock_tab_label` has the exact instance title. The React counterpart defines localized action labels and a target-substituted drag instruction. | All dock hits fall through to English identifier humanization. |
| Window pane chips | `paint_window_pane_chips_step` paints a localized `ChromeGroupItem` with `register_hit = false`, then appends a raw `HitTarget` to `pane_overlay_hits` | The local `label = shell_chrome_string(chip.label_key(), is_de)` is already the visible string. | `Window Options`, fold/unfold and projection/action/search/utility controls lose their localized label at the final manual hit registration. |

The generic retained chrome painter does call `note_chrome_control_name` before registering its hit. Its nearby comment that it covers every chrome chip is therefore inaccurate: it does not cover the three manual paths above.

## Reference labels and existing Rust authority

React's mode dock at `🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` paints a tab title as the tab's accessible label. Its focus action uses `ui.window.focus` unless maximized, then uses `ui.window.unfocus`; close uses `ui.window.close`. Its `DragHandle` calls `useLabel("ui.tree.drag.sortTarget", { target: tab.title })`.

The current Rust localization authority is `shell_chrome_string(key, is_de)` in the WGPU Shell. It already has the required window action vocabulary:

| Semantic label | React English / German | Current Shell key and English / German |
|---|---|---|
| Focus | `Focus` / `Fokussieren` | `common.focus`: `Focus` / `Fokussieren` |
| Unfocus | `Unfocus` / `Fokus aufheben` | `common.unfocus`: `Unfocus` / `Fokus aufheben` |
| Close | `Close` / `Schließen` | `common.close`: `Close` / `Schliessen` |
| Drag a title | `Click and hold left click to drag {{target}}` / `Linksklick gedrückt halten, um {{target}} zu ziehen` | no current Shell key |

`common.close` is an independently source-confirmed German text mismatch: the existing WGPU table spells `Schliessen`, while React spells `Schließen`. Correct the existing Shell entry rather than create a second action-label dictionary. Add the drag target templates to that same Shell authority and interpolate the already resolved instance title; this is one localization source on the WGPU target, not a browser-side translation.

`dock_tab_actions(show_maximize, maximized)` keeps the control-id suffix `focus` while choosing a maximize or minimize icon. The name resolver must therefore receive `maximized` and announce `Unfocus` when it is true, even though the raw id ends in `.focus`.

## Repair boundary

The minimal safe repair is a frame-staged semantic-name sink that lives with the WGPU chrome input build and is promoted with that frame's hits. Producers write the exact string used to paint each manual control; `chrome_accessibility_nodes` consumes the staged lookup before `humanize_control_id`.

Required invariants:

1. A name written during a discarded, failed, or superseded build must never appear in the accepted accessibility dump.
2. A name must be keyed by the exact current `HitTarget.control_id`; it must not parse puzzle ids or infer a title after acceptance.
3. Window-body and dock-tab selection names come from the resolved instance title, preserving authored `Top` and `Perspective` without creating a puzzle-specific mapping.
4. Dock action names come from the same locale and state used by the painter. The drag action receives the resolved title as the template argument.
5. Pane chips pass the `label` already calculated for painting to their final manual hit registration.
6. `humanize_control_id` remains an emergency fallback for controls with no user-visible semantic label; it is not a localization implementation.

The Dock producer is the long-term owner of dock hit semantics. The current parent integration slice may thread a sink through `DockRenderContext`, but no separate Dock/Tree implementation is assigned by this audit because that area is already owned by Sols. The browser mirror, React catalog, ARIA event validation, and accepted-frame rejection rules remain unchanged.

## Required regression laws

Use one real accepted-frame fixture, not a direct map/unit-only assertion. Start a German `puzzle3d` session with layout instances `puzzle3d-main-top` (`Top`) and `puzzle3d-main-perspective` (`Perspective`), render, seal, and acknowledge the input candidate. Inspect the published chrome projection.

It must assert all of the following:

- the raw window body ids project labels `Top` and `Perspective`;
- the dock tab selection target has its exact visible title;
- focus is `Fokussieren` when unmaximized and `Fokus aufheben` when maximized;
- close is `Schließen`;
- a drag hit for `Top` is `Linksklick gedrückt halten, um Top zu ziehen`;
- each painted pane chip has the exact German text already resolved for it, including `Fensteroptionen` where that chip is present;
- none of those nodes has a raw identifier or an English fallback label; and
- a rejected candidate does not replace the previous accepted hit/name pair.

Pair it with an English case for the drag template and the normal Focus/Close strings. The browser-level companion should use the production accessibility mirror only to assert the already-published strings become `aria-label`; it should not reproduce the label resolver in TypeScript.

## Source-confirmed residual parity inventory

The following are still worth separate work after this AX slice. They are source findings, not runtime completion claims.

| Priority | Gap | Current evidence | Bounded next slice |
|---|---|---|---|
| P0 accessibility | Accepted WGPU chrome drops producer-owned labels and therefore leaks raw ids and English fallbacks in German. | The three direct hit paths above bypass `note_chrome_control_name`; accepted projection uses the fallback. | The staged sink and accepted-frame laws described above. |
| P1 locale/time | Event feed timestamps are materially different across time zones and hour cycles. | React `📡️EventFeedHost/🟦️.tsx` uses `Date(...).toLocaleTimeString`; WGPU `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` uses `event_feed_time_of_day_utc`, a modulo-day `HH:MM:SS` formatter. | Define a small owned host time-formatting port with supplied locale and zone; establish React-oracle fixtures for winter/summer in UTC, Europe/Berlin, and America/New_York before changing Scenes. |
| P1 evidence | Full runtime parity cannot be inferred from old image and probe reports. | This audit found many formerly open reports implemented in current source, but did not launch paired current React/WGPU servers or inspect a current accepted dump. | After merges settle, run a single controlled paired acceptance journey with captured locale, zone, viewport, route, app seed, and accepted chrome dump. Treat any remaining visual claim as unconfirmed until that run. |

Current source invalidates several reports that must not be used to create duplicate work: Window Measures now projects a compact retained `Tree`, selects checkbox appearance, sets `TopEnd` flow, and uses accepted intrinsic height; the earlier browser-mirror described-input and paragraph defects are repaired; chrome checked/selected projection is now derived from live state; Display leaves, merge policy, theme import/export, tool measures, conflicts, named-layout reads, history check-in, and Worker host-platform resolution all have current implementations. Select and Tree/Dock behavior remain outside this audit's implementation boundary.
