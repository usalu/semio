# WGPU vs React parity re-audit: 20 contract components (live path)

Read-only re-audit of the current tree. No source edited, no cargo or repo build run. Only this report was written.

Root `W` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/` (live UI wgpu target). Paths are relative to `W` unless absolute. Status: FIXED, PARTIAL, OPEN, or unverified (not checked or not verifiable by grep/read).

Path drift vs the prior audits: the immediate widget root `🪀️widgets/🦀️.rs` (WidgetNode/ControlNode) no longer exists. Only `🪀️widgets/🧰️kit/🦀️.rs` (text/context helpers) and the select/slider/stepper element kits under `🔘️controls/` remain, reached through `🖌️paint/🦀️.rs:31`. Element-level `🧱️elements/*/🎯️targets/🧊️wgpu` kits for Button, Input, IconSelector, Ring and Tree are gone. Gaps that lived only in those kits are therefore N/A. Test-only `🖌️paint/🔬️reference/` is not shipped.

## Basic widgets audit (prior): gap status

| Component | Prior gap | Status | Evidence (current) |
|---|---|---|---|
| Button | Label hidden in `labels=icons` | FIXED | `🖌️paint/🎨️…/🎛️control-nodes/🦀️.rs:20,25-28` (`icon_only`) |
| Button | Label colour does not follow hover | OPEN (low) | `control-nodes/🦀️.rs:42` still `theme.text` |
| Button | Busy/in-flight disabled + aria-busy | OPEN (med) | no `busy` in paint or events; a11y `busy=false` only (`♿️accessibility/🦀️.rs:127`) |
| Button | Tooltip in icons mode | unverified | not traced |
| Button | Tone on button | unverified | tree tone is applied (`🌲️retained-tree-node-painting/🦀️.rs:198`), button tone not traced |
| Input | Multi-line longText | FIXED | `🖌️paint/✍️retained-edit-text-steps/🦀️.rs:81,137`; `📌️mounted_layout/🧱️job-state-and-admission/🦀️.rs:254` (`TEXTAREA_MIN_HEIGHT`, `🧮️layout/🦀️.rs:537`) |
| Input | IME preedit | FIXED | `✍️retained-edit-text-steps/🦀️.rs:21-31`; `control-nodes/🦀️.rs:71,75` |
| Input | Selection highlight | FIXED | `✍️retained-edit-text-steps/🦀️.rs:33`; span step in `control-nodes` phase 2 (`:~100-115`) |
| Input | Number refusal message not painted | FIXED | `control-nodes/🦀️.rs:68,130`; `✍️…/🦀️.rs:70-71`; height `📌️…/job-state…/🦀️.rs:255` |
| Input | Field-level invalid border | OPEN (med) | `control-nodes/🦀️.rs:76,84`: `Invalid` only when `refused`; container error not read |
| Input | Date/file/colour native pickers | OPEN (med) | no picker located in paint or host |
| Input | Copy/cut | FIXED | `⚡️events/✍️drag-scroll-and-edit-api/🦀️.rs:584,591` (`ClipboardCopy`/`ClipboardCut`) |
| Select | Segmented appearance painted as dropdown | FIXED | `control-nodes/🦀️.rs:161-162,310-312`; `⚡️events/🧮️control-commit-rules/🦀️.rs:286-287`; a11y `♿️accessibility/🦀️.rs:180-184` |
| Select | Hard-coded "Select…" placeholder | FIXED | `control-nodes/🦀️.rs:192` (`ui_common_label(…labels.select)`) |
| Toggle | (no gap) | FIXED/matches | `control-nodes/🦀️.rs:414,431`; `♿️accessibility/🦀️.rs:339` |
| KeyValueList | Fixed row pitch, no wrapping | OPEN (low) | `📝️text-and-structure-nodes/🦀️.rs:104` `control_height` row pitch |
| Slider | No hover, focus or drag visuals | FIXED | `🎚️slider-stepper-and-progress-nodes/🦀️.rs:8-30` (`hovered`, `dragging`, `focus_visible`); `📶️…/🦀️.rs:238` |
| Slider | Kit readout ignores unit/precision | OPEN (low, kit only) | `🧱️elements/🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs:118` (`format_ui_number`); whether this kit is still called is unverified |
| NumberStepper | "Mixed" hard-coded | FIXED | `🎚️…/🦀️.rs:135` (`labels.mixed_values`, fallback at `:100`) |
| Ring | t=0 at 3 o'clock, drag off by 90° | FIXED | paint `🧮️layout/🦀️.rs:561` (`TAU·t − π/2`); drag `:578-581` (`atan2 + π/2`) |
| Ring | Orb size and track colour | FIXED | `📶️…/🦀️.rs:293` (track `text_muted` @0.3), `:301` (radius 5/7) |
| Ring | Focusable with no focus paint | FIXED | `⚡️events/🧭️event-model-and-focus/🦀️.rs:313` excludes Ring from focus |
| Ring | No keyboard operation, role slider | OPEN (shared, product) | no key handler for Ring; React is also an unfocusable SVG (`⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:253` only a press guard) |
| IconSelect | Text value, no icon picker | FIXED | `control-nodes/🦀️.rs:452-490`; `🖌️paint/🔄️interactive-state-sync/🦀️.rs:335,453-494` (cells); editor via synthesized Input (`⚡️events/🧮️control-commit-rules/🦀️.rs:58`) |
| IconSelect | Disabled click-through | unverified | hit registration not traced |
| Progress | Indeterminate pulse | OPEN (low, deliberate) | `📶️…/🦀️.rs:63-66` static centred band, documented as intentional |
| Image | Non-`data:` src never loaded | OPEN (high, by design) | `🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs:10-16` (`data:` only; fetch left to host) |
| Image | Paged-load progress not drawn | FIXED | `🖌️paint/🎨️…/🖼️media-and-slot-nodes/🦀️.rs:33` (`source_page` step) |
| Image | Fallback shows node id | FIXED | `media-and-slot-nodes/🦀️.rs:51` uses `alt` |

## Structure and overlays audit (prior): gap status

| Area | Prior gap | Status | Evidence (current) |
|---|---|---|---|
| Tree | No keyboard model | FIXED | `⚡️events/🌳️tree-roving-and-keys/🦀️.rs:1-60` (ordered rows, WAI-ARIA keys) |
| Tree | Roving tabindex | FIXED | `⚡️events/🧭️event-model-and-focus/🦀️.rs:332-340` (one tab stop per tree) |
| Tree | Section fold chevron | FIXED | `🖌️paint/🌲️retained-tree-node-painting/🦀️.rs:6,104` |
| Tree | Section label uppercase/semibold | unverified | label step not re-read |
| Tree | Indent guides and elbows | FIXED | `🌲️…/🦀️.rs:99` (guide), `:331-349` (elbows and stems) |
| Tree | Modifier selection, double-click | FIXED | `⚡️events/🌳️tree-roving-and-keys/🦀️.rs:104,116` |
| Tree | Reorder drop preview, palette drag, section reorder | OPEN (high) | no drop-preview or drag-reorder code located in `🌲️…/🦀️.rs` or `⚡️events/🫳️drag-drop/` |
| Tree | Detail band and inline toolbar paint | OPEN (med) | only control width reserved (`🌲️…/🦀️.rs:228,258`); no detail paint located |
| Tree | Empty-state slot | OPEN (low) | not located |
| Tree | RTL mirroring (React forces LTR) | OPEN (decision) | `🌲️…/🦀️.rs:13-23` mirrors; React `RUI/🧱️elements/🌳️Tree/🟦️.tsx:521` forces LTR |
| Table | Fixed column widths | FIXED | `📊️table-model/🦀️.rs:64-68`; `🧮️layout/🦀️.rs:390-396` |
| Table | Keyboard (arrows, Page, Home/End, Enter/Space) | FIXED | `⌨️widget-keyboard-and-commit/🦀️.rs:50-65` (`table_key`) |
| Table | Row double-click | FIXED | `⚡️events/🌳️tree-roving-and-keys/🦀️.rs:116` |
| Table | Empty label "No rows" literal | FIXED on UI path | no literal found; empty label in `📊️table-model` (module doc). OS surface table is out of scope |
| Table | aria-sort | PARTIAL | `📊️table-model/🦀️.rs:171` defines `table_aria_sort`, zero callers; no header projection in `♿️accessibility/🦀️.rs` |
| Table | Row context-menu trigger button | OPEN (med) | no kebab or trigger paint located |
| Table | Drag activation delay/tolerance | OPEN (med) | no delay logic located in `⚡️events/🫳️drag-drop/🦀️.rs` |
| Table | Drop-target highlight | unverified | not located |
| Extension | Arm for `Component::Extension` | PARTIAL | `media-and-slot-nodes/🦀️.rs:99-120` paints React's "Extension unavailable: <ext>" text; no dynamic host bridged |
| Overlays | Opaque surface and hardcoded scrim | FIXED | `🖌️paint/🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs:22-23` (`veil_glass`), `:27-28` (`glass`) |
| Overlays | Shadow invisible | FIXED by design | `:26` comment: React `--shadow-lg` is `0 0 transparent` |
| Overlays | Square corners | OPEN (low) | `:28` glass radius passed as `0.0`; React rounding for Select unverified |
| Overlays | Focus restore on close | FIXED | `⚡️events/🧲️overlays-drag-and-commands/🦀️.rs:38-39`; `⏱️tooltip-clock-and-overlay-api/🦀️.rs:191,210,269-285` |
| Overlays | Modal background inert | PARTIAL | pointer hits scoped to modal (`⚡️events/🏗️router-construction-and-intents/🦀️.rs:359`); sibling `aria-hidden`/`inert` not located |
| Overlays | Tooltip reveal on keyboard focus | OPEN (med) | only hover reveal (`⏱️tooltip-clock-and-overlay-api/🦀️.rs:176`) |
| Overlays | Dialog open/close animation, X control | unverified | not located |
| Scroll | Scrollbar thumb painting | FIXED | `⚙️engine/🪟️surfaces/📍️window-and-tooltips/🦀️.rs:358-371`; `🖌️…/🖼️…/🦀️.rs:391` |
| Window layout | Contract `WindowLayoutNode` unused | unverified | consumers now include `🧬️contract/…/🪟️window-layout-ops/🦀️.rs` (contract side); live dock consumer not re-traced |
| Dock splitter | Coarse 28 px hit | unverified | OS dock is out of the UI-target scope; not located by grep |

## New gaps found

1. **aria-sort is modelled but never projected.** `table_aria_sort` has no caller (`📊️table-model/🦀️.rs:171`). Medium, accessibility.
2. **Ring and shared keyboard gap restated.** Ring is now non-focusable on both sides (`event-model-and-focus:313`), which matches React. The contract still reports role `slider` (`♿️accessibility/🦀️.rs:~313`). Product decision needed.
3. **Button tooltip and pressed-state parity** remain unverified. These are not gaps until traced.

## Unverified (not checked or not verifiable here)

- Button tooltip in icons mode, button tone, button pressed visuals, disabled dim strength (0.35 in `🖌️…/📶️…/🦀️.rs:24`, approximate).
- Section label typography, Field band sizes, Separator, Container section fold.
- IconSelect disabled click-through.
- Overlay corner rounding versus React `rounded-sm` on Select.
- Window header controls and Dialog close control.
- Coarse splitter hit in the OS dock.
- Table header typography and avatars.
