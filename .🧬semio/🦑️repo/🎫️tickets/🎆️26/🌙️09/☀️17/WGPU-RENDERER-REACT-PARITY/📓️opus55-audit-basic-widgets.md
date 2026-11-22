# WGPU vs React parity audit: basic contract widgets

Read-only audit. No source file was edited and no git command was run.

## Scope, roots and method

Roots used in the tables below (all absolute):

- `RI` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/` (React interpreter)
- `RU` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/` (UI module: contract, declaration, elements, styling, retained tree)
- `EL` = `RU/🧱️elements/` (React design-system elements and their per-element wgpu targets)
- `WG` = `RU/🎯️targets/🧊️wgpu/` (wgpu engine)
- `CT` = `RU/🧬️contract/🧩️component/` (contract)

Paths are written relative to those roots, with the line numbers verified by reading the files.

**Two wgpu paths exist, and they diverge.** This matters for every fix:

1. The live retained path. `WG/🖌️paint/🎨️node-painting-with-driver/🦀️.rs:23-109` (`paint_node_step_with_driver`) dispatches each `UiNode` to a `paint_*_node_step` in `🎛️control-nodes`, `🎚️slider-stepper-and-progress-nodes`, `📝️text-and-structure-nodes`, `🖼️media-and-slot-nodes`. The `UiNode` is built by the reconcile layer `RU/🌳️retained/🌳️tree/🔀️reconcile/🎛️widget-nodes/🦀️.rs` and `…/🎞️media-transport-and-node-from-record/🦀️.rs`.
2. The immediate kit. `WG/🪀️widgets/🦀️.rs` (`WidgetNode`/`ControlNode`) calls `semio_ui_wgpu_controls` (`WG/🔘️controls/🦀️.rs`), which is the per-element files `EL/<Element>/🎯️targets/🧊️wgpu/🦀️.rs`. The retained painter's own doc says this kit "still exists" as a mechanically ported copy. Test-only `WG/🖌️paint/🔬️reference/🦀️.rs` is compiled only under `cfg(test/testkit)` (`WG/⚙️engine/🦀️.rs:23`), not in production.

Where the two paths disagree, both are listed. A fix must change the retained path, and the kit path if it still ships.

**Token parity.** Colours, control height, padding, gap and font sizes in wgpu are read from the same Rust token constants that generate the CSS (`RU/🎨️styling/🎭️theme/🧩️component/🦀️.rs:254-293` maps `chrome.*` and `typography::*`; the tokens are in `RU/🎨️styling/🔤️tokens/🦀️.rs:334-348`). The token values line up with the CSS (`--size-medium: 22.4px`, `--size-small: 16px` in `RU/🎨️styling/🎨️palette/🎨️.css:227-228`, and `TEXT_SM_PX`=12.8, `TEXT_XS_PX`=11.2). So colour and metric *sources* are shared. Per-class Tailwind mapping in React (for example `h-medium`, `px-double`, `gap-x-single`) was spot-checked, not mapped exhaustively. Pixel-level equality is therefore **unverified** except where stated.

Severity: **high** = a user-visible or semantic difference that changes what the user sees, what a value means, or what they can do; **med** = a visible state, i18n or interaction difference; **low** = cosmetic or approximate.

---

## 1. Container (`Component::Container`, roles plain/section/group/field/form/toolbar/sheet/cardStack/card)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Role dispatch | `RI/🎛️control-port/container.tsx:31-49` (section/group → `SharedBorderRow` with `h2` title; cardStack; card → `OverviewCard`; field → `Field`; row/plain → `div`) | `RU/🌳️retained/🌳️tree/🔀️reconcile/🎞️media-transport-and-node-from-record/🦀️.rs:146-160` (section→Section, group→Group, cardStack, card→OverviewCard, field→Field, plain/form/toolbar/sheet→Stack) | Structure matches. `Sheet` has no React rendering and is painted as a plain stack in both. Form/toolbar role exposure is contract-driven in both. | low |
| Section/Group collapsibility | Section and Group are both `SharedBorderRow` in `container.tsx:34` with no fold state. Any fold lives elsewhere (`RI/🧱️node-tree/🟦️.tsx`, not read) | `UiSectionNode`/`UiGroupNode` carry `default_open` (`…/🎞️media-transport…/🦀️.rs:150-152`) and the retained painter has section/group steps (`WG/🖌️paint/🎨️…/🦀️.rs:92`) | **unverified**: whether React Section/Group fold when `defaultOpen` is false. wgpu does fold (`🪀️widgets/🦀️.rs:287` comment: "Group folds through `collapsed_sections`"). | med (if React does not fold) |
| Field label/description/required/error | `RI/🎛️control-port/container.tsx:47` (`Field` with `label`, `description`, `required`, `error`) | `WG/🖌️paint/🎨️node-painting-with-driver/📝️text-and-structure-nodes/🦀️.rs:130-170` (label semibold, red `*` when required, description muted wrap, error in `theme.error`) | Content matches. React `Field` element internals were not read, so the exact band sizes are **unverified**. | low |
| Drop overlay | `ContainerProps.drop_overlay` (contract `CT/🎛️container-roles-and-row-actions/🦀️.rs`) | `WG/🖌️paint/🖼️…` not checked for drop overlay | **unverified** | low |

---

## 2. Text (`Component::Text`)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Size / weight | `RI/🔤️display-views/🟦️.tsx:38-47`: non-emphasised `text-sm`, emphasised `font-semibold` (no size, inherits) | `WG/🖌️paint/🎨️…/📝️text-and-structure-nodes/🦀️.rs:16-19`: non-emphasised `theme.font_size_body` (=TEXT_SM 12.8), emphasised `font_size_emphasized` (=TEXT_BASE 14.4), `TextWeight::of(emphasize)` | Non-emphasised matches the token. Emphasised size depends on the inherited body size in React, which was **unverified**. | low |
| Colour | `text-foreground` | `theme.text` | Token parity (same chrome token). | low |
| Wrap / clip | `<p>` wraps | `RetainedTextFlow::Wrap` (`…/📝️retained-text-steps/🦀️.rs:117`) | Matches. | low |
| Accessibility | `accessibilityAriaProps` + `describedBy` (`display-views:38-47`) | Contract role/label via `WG/♿️accessibility/🦀️.rs` | Shared contract. | low |

---

## 3. Button (`Component::Button`)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Label visibility | `EL/🔳️ButtonGroup/🟦️.tsx:110-121`: label is drawn inline only when `useControlInlineText` returns text. `EL/🏷️Label/🟦️.tsx:139-143`: that returns `undefined` when `driver.labels === "icons"`, so the label becomes tooltip/aria only | `WG/🖌️…/🎛️control-nodes/🦀️.rs:35` always draws the label; element `EL/🔘️Button/🎯️targets/🧊️wgpu/🦀️.rs:33` also always draws it. No check of the UI driver `labels` setting. | In `labels=icons` mode React hides the text and wgpu shows it. | med |
| Tooltip (title) | `EL/🏷️Label/🟦️.tsx:149-160` (`useControlTooltipText`: only when no inline text, honours `driver.tooltips`, appends hotkey) | Generic tooltip subsystem exists (`WG/⚡️events/⏱️tooltip-clock-and-overlay-api/🦀️.rs`), but no check that Button participates | **unverified** whether a Button in icons mode gets its tooltip. | med |
| Icon | `RI/🔤️display-views/🟦️.tsx:49-89` → `resolveControlIconNode(component.icon)` | `WG/🖌️…/🎛️control-nodes/🦀️.rs:25` paints `button.icon_id` at `ICON_TINY`. Element kit `EL/🔘️Button/…/🦀️.rs:23-26` falls back to the **label text** as the icon key when none is given. | Retained path does not fall back, the kit does. | low |
| Hover/press colour | `ButtonGroupItem` item classes (`EL/🔳️ButtonGroup/🟦️.tsx:133-141`, `chromeControlItemClass`) | Border and background use `item_bg`/`item_text(hovered)`, but the label is always `theme.text` (`control-nodes:35`), so hover never changes label colour. | Label colour does not follow hover. | low |
| Pressed / active | Click → `activate` (`display-views:58-70`) | No pressed background (`item_bg(theme,false,hovered)`, `control-nodes:22`) | No pressed visual in wgpu. React's pressed look is CSS-only and **unverified**. | low |
| Disabled | `disabled={controls.disabled(record) || state.busy}` (`display-views:~80`) | `presence_overlay` dims with `panel @ 0.35` (`WG/🖌️…/📶️progress-celebrate-and-widget-painters/🦀️.rs:17-20`). No hover when disabled (`control-nodes:21`). | Dim strength is an approximation of `disabled:opacity-50` (**unverified**). | low |
| Busy during dispatch | `state.busy` → `aria-busy`, `disabled`, `activityBorderClass` (`display-views:58-73`) | Activity border from `presence.status` (`presence_overlay`). No "busy while dispatching" state. | React disables the button while its command is in flight, wgpu does not. | med |
| Focus ring | Tailwind `focus-visible` on the item | `focus_ring_visible(flags)` → accent border, `SharedBorderState::Focus` (`control-nodes:22-23`) | Matches. | low |
| Keyboard | Native `<button>`: Enter/Space | `WG/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:37-50` (Enter/Space on focused enabled Button) | Matches. | low |
| Tone | `data-tone={record.style.tone}` (`display-views:~77`), styling is CSS-only | `EL/…` / reconcile sets `style: None` (`RU/🌳️…/🎛️widget-nodes/🦀️.rs:84`) | `tone` is dropped in wgpu. Whether React CSS uses `[data-tone]` on buttons is **unverified**. | low |
| Accessibility | `aria-label`/`aria-labelledby`/`aria-describedby`/`aria-busy` (`display-views:60-75`) | Role/label from contract (`WG/♿️accessibility/🦀️.rs`) | Shared; busy not projected. | low |

---

## 4. Separator (`Component::Separator`)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Element | `RI/🧱️node-tree/🟦️.tsx:567-568`: `<hr className="border-0 border-t border-normal">`; the separator is skipped as a child at `node-tree:122,153` | `WG/🖌️…/📝️text-and-structure-nodes/🦀️.rs:64-82` → `paint_separator` (`WG/🖌️…/📶️progress-celebrate-and-widget-painters/🦀️.rs:186-189`): a horizontal line at mid-height, `theme.separator`, `stroke_hairline` | Colour token is the same (`separator = chrome.border_normal`, `RU/🎨️styling/🎭️theme/🧩️component/🦀️.rs:262`). Line width is hairline in both. A separator in a row layout is painted horizontal in wgpu. React `<hr>` is also horizontal. | low |
| Skipping in layouts | `node-tree:122,153` excludes the separator from some slots | Reconcile emits a `UiSeparatorNode` for every non-matching record (`widget-nodes:14,50,63…`) | Fallback-only. | low |

---

## 5. Input (`Component::Input`: text, longText, number, date, color, file)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Kinds | Native `<input type=text|number|date|color|file>` (`RI/✍️input-views/🟦️.tsx:296`); longText is `<Textarea>` (`input-views:274-294`) | `paint_input_node_step` `WG/🖌️…/🎛️control-nodes/🦀️.rs:46-110`: one text painter. Colour swatch (`:70`, `color_input_swatch` `…/📶️…/🦀️.rs:~228`). date/file are plain text. No native picker. | Date and file have no picker. Colour has no picker, only a swatch. | med |
| Multi-line longText | `min-h-[4.5rem]`, multi-line textarea (`input-views:276-292`) | `text_line_view(...)` (`control-nodes:68-69,99`) shows **one line** around the caret | Multi-line content is not shown. | high |
| IME composition | Native input shows preedit (browser) | Input display uses `edit.text` only (`control-nodes:61-67`), no `edit.composition`. The stepper does insert it (`…/🎚️slider-stepper-and-progress-nodes/🦀️.rs:138-143,163-168`). Events receive `ImeEvent` (`WG/⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs:477`). | Preedit text is not painted in text inputs. | high |
| Selection highlight | Native selection (`input-views`, implicit) | The production painter draws only a caret, and only when `anchor == caret` (`control-nodes:95-107`). Selection is drawn only in the test-only `WG/🖌️paint/🔬️reference/🦀️.rs:309`. | No selection highlight in production. | high |
| Caret | Native caret | 1px caret via `retained_caret_step` (`control-nodes:95-107`) | Matches in principle; blink driven by `caret_visible`. | low |
| Placeholder | native `placeholder` (`input-views:313`), muted | `control-nodes:63` placeholder, colour `text_muted` (`:87`) | Matches. | low |
| Refusal (number) | `<span role="alert" data-slot="input-refusal">` under the field (`input-views:327-331`), `aria-invalid` (`:306`) | Border turns `theme.error` (`control-nodes:60,75`). The refusal **message is not painted**. | The refusal text is missing in wgpu. | high |
| Draft/commit | `useCommitDraft` queue, blur/Enter/Escape (`input-views:63-130, 237-247`) | `text_field_key` Enter commit / Escape revert (`WG/⚡️events/🧮️control-commit-rules/🦀️.rs:296-302`), Enter/Home/End in edit (`WG/⚡️events/✍️drag-scroll-and-edit-api/🦀️.rs:274,361-367`) | Matches by design; no drift found. | low |
| Number keys | `lawKey` → `uiNumberFieldKey`/`uiNumberKeyValue` (`input-views:251-262`) | `number_field_key` (`WG/⚡️events/🧮️control-commit-rules/🦀️.rs:309-319`) | Matches. | low |
| Invalid state | `aria-invalid` on field (`input-views:306`) | Only number refusal sets error border; field-level `invalid` is not drawn on the input | Field invalid styling missing in wgpu, but the field's error text is painted (see Container). | med |
| Kit (legacy) path | n/a | `EL/✏️Input/🎯️targets/🧊️wgpu/🦀️.rs:17-38`: text inset hard-coded `8.0` (`:30`) and cursor `bounds.y+6` (`:35`) | Hard-coded inset in the kit path. | low |
| Clipboard | React native copy/paste | Paste routed (`WG/⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs:476`). Copy/cut of input selection not verified. | **unverified** copy/cut. | med |

---

## 6. Select (`Component::Select`, appearance menu | segmented)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Segmented appearance | `RI/✍️input-views/🟦️.tsx:342-362`: `ToggleGroup` with `semantics="radio"`, one icon per option (`circle-dot`) | Reconcile drops `appearance`: `UiSelectNode` has no such field (`RU/📜️declaration/🧩️ui-graph/⌨️input-select-toggle-and-group-nodes/🦀️.rs:91-104`) and `select_node` does not map it (`RU/🌳️…/🎛️widget-nodes/🦀️.rs:49-60`). The paint always draws a menu (`control-nodes:114-275`). Events and a11y **do** read the record appearance (`WG/⚡️events/🧮️control-commit-rules/🦀️.rs:324-327`, `WG/♿️accessibility/🦀️.rs:178-215`). | Segmented select is **painted as a closed dropdown**, while its keyboard and radio semantics are segmented. | high |
| Closed trigger | `SelectTrigger` `h-medium`, `SelectValue` with `placeholder ?? interpLabel("ui.common.select")` (`input-views:363-367`) | `control-nodes:132-139` (border/hover fill `button_hover`/`input_bg`, chevron-down) | Matches on structure. | low |
| Placeholder text | `interpLabel("ui.common.select")` (`input-views:366`; `declarative-controls:84`) | Hard-coded English `"Select…"` at `control-nodes:159` and `WG/🖌️paint/🔬️reference/🦀️.rs:348` | Not localised. | med |
| Open popup | Radix `SelectContent` items (`input-views:368-374`) | Popup built by `retained_select_popup_at_bounds`, `control-nodes:168-239` (menu glass, chevrons, selected row highlight `row_hover`) | Structure matches. Exact popup geometry **unverified**. | low |
| Keyboard | Radix trigger/content handlers | `WG/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:~60-100` (arrows, typeahead, Tab closes) and the code comment at `:60-67` claims union parity | Stated parity, not re-verified line by line. | low |
| Declarative (non-interpreted) select | `declarative-controls:81-87` | n/a | — | low |

---

## 7. Toggle (`Component::Toggle`, appearance button | checkbox)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Button look | `Toggle` item with `pressed` (`input-views:382`) | `paint_toggle_node_step` `control-nodes:293-304`: `item_bg(pressed,hovered)`, `Active` border when pressed, icon + text | Matches. | low |
| Checkbox look | `TreeCheckbox` (`input-views:381`) | `paint_checkbox` `WG/🖌️…/📶️…/🦀️.rs:203-215`: 1px-stroke square, `text` fill when checked, hand-drawn tick | Tick approximates React's check icon (**unverified** pixel match). | low |
| `on` state | `pressed`/`checked` from `component.on` (`input-views:381-382`) | Reconcile sets `presence.selected = props.on` (`widget-nodes:63-65`) | Matches. | low |
| Keyboard | `Toggle` handles Enter and Space (`EL/🔀️Toggle/🟦️.tsx:159-168`) | `WG/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:181` (Enter/Space flips `on`) | Matches. | low |
| Checkbox label | Label is accessibility-only (`ariaLabel`, `input-views:381`) | Checkbox label skipped in paint (`control-nodes:313-317`) | Matches (both render no visible text inside the checkbox). | low |
| Accessibility | `aria-pressed` (`Toggle:181`) | `WG/♿️accessibility/🦀️.rs:336-338` (`pressed`/`checked`) | Matches. | low |
| Focus ring | Tailwind | `focus_ring_visible` (`control-nodes:~301`) | Matches. | low |

---

## 8. KeyValueList (`Component::KeyValueList`)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Layout | `<dl class="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs">` (`display-views:91-104`); rows auto height | Retained: fixed row pitch `theme.control_height` (`WG/🖌️…/📝️text-and-structure-nodes/🦀️.rs:104`). Kit: `EL/🔑️KeyValue/🎯️targets/🧊️wgpu/🦀️.rs:19-23` also uses `control_height`. | Row height is 22.4px fixed in wgpu, while React rows are as tall as the 11.2px text plus `gap-y-single`. Long values do not wrap in wgpu. | med |
| Label / value colours | label `text-muted-foreground`, value `tabular-nums` (`display-views:101-102`) | Label `text_muted`, value `text` (`text-and-structure-nodes:106`, kit `…/🦀️.rs:22-23`) | Colour matches. Tabular numerals are not requested in wgpu (**unverified** whether the font has tabular figures). | low |
| Label size | `text-xs` (`display-views:91`) | `font_size_small` (TEXT_XS) | Matches. | low |
| Value column x | grid auto column = widest label | `label_w + gap_standard*2` (kit `…/🦀️.rs:17-18`); retained path uses a per-row label cell | Close but not identical. | low |
| Accessibility | `<dl>` (no role) | Contract role `list` (`CT/♿️accessibility/🦀️.rs:43`) | Role differs by design, shared contract. | low |

---

## 9. Slider (`Component::Slider`, appearance track | dial; scale linear | log)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Track and range colour | rail `bg-muted h-single`, range `bg-element` (`EL/🎚️Slider/🟦️.tsx:29,525,531`) | rail `theme.muted`, range `theme.text_element` (`WG/🖌️…/🎚️slider-stepper-and-progress-nodes/🦀️.rs` via `paint_slider_travel` `WG/🖌️…/📶️…/🦀️.rs:~220-250`) | Tokens match. | low |
| Thumb states | thumb `bg-element`; `hover/group-hover: bg-emphasized`; `focus-visible: bg-active-base + border-emphasized`; `data-[dragging]: bg-active-base`; `disabled: opacity-50` (`EL/🎚️Slider/🟦️.tsx:38-45`) | Thumb always `text_element` (`paint_slider_travel`). `paint_slider_node_step` takes **no flags** (`…/🎚️slider-stepper-and-progress-nodes/🦀️.rs:7-19`), so there is no hover, focus-ring or dragging visual. Kit: `EL/🎚️Slider/…wgpu/🦀️.rs:103-117` dims with alpha 0.5 when disabled. | Focus ring, hover and dragging looks are missing in wgpu. | high |
| Readout | `aria-valuetext` (`input-views:394`); visible readout uses `uiNumberDisplayText` with `displayUnit ?? unit` (`input-views:413-420`) | Retained: `slider.readout(value)` (`…/🎚️…/🦀️.rs:47-48`) honours unit/display factor. Kit: `format_ui_number(live)` only (`EL/🎚️Slider/…wgpu/🦀️.rs:118-120`), ignoring unit, `displayFactor` and `precision`. | The kit path drops units and precision. The retained path is fine. | med (kit only) |
| Editable readout | Double-click or Enter/F2/Space opens an exact-value spinbutton (`EL/🎚️Slider/🟦️.tsx:625-630`) | Editing draft via `node.state.edit` and caret (`…/🎚️…/🦀️.rs:59-72`), trigger not verified | Trigger parity **unverified**. | low |
| Refusal | `role=alert` `slider-refusal` text (`EL/🎚️Slider/🟦️.tsx:636`) | Refusal text painted in `track_cell` with `theme.error` (`…/🎚️…/🦀️.rs:39-45`) | Matches. | low |
| Keyboard | Arrow/PageUp/PageDown/Home/End via shared law (`EL/🎚️Slider/🟦️.tsx:461`; `input-views` lane) | `slider_key_value_from` (`WG/⚡️events/🧮️control-commit-rules/🦀️.rs:262-280`) maps ArrowUp/Right ↑, Down/Left ↓, PageUp/Down, Home/End with the same contract law | Matches. | low |
| Pointer | pointer capture, `pointerValue` (`EL/🎚️Slider/🟦️.tsx:385-436`) | Hit region `HitKind::Slider`, `DragAxis::Horizontal` (kit `…/🦀️.rs:122`); retained via `slider_node_pointer_value` (`…/🎚️…/🦀️.rs` referenced by events) | Matches in design; not traced end-to-end. | low |
| Wheel | none in React | none in slider; wheel exists only for scroll (`WG/⚡️events/🧲️overlays-drag-and-commands/🦀️.rs`) | Parity. | low |
| Ticks (snaps) | `slider-tick` spans at 60% opacity (`EL/🎚️Slider/🟦️.tsx:548`) | `slider_tick_rects` with `text_element @ 0.6` (`…/🎚️…/🦀️.rs:105-107`) | Matches. | low |
| Dial appearance | SVG dial, needle, ticks at 62-92% radius (`EL/🎚️Slider/🟦️.tsx:500-510`) | `paint_slider_travel` dial branch (`…/📶️…/🦀️.rs` ~L224-238): face, ticks, needle, thumb at 0.82 | Matches in structure. | low |
| Disabled | `disabled:opacity-50`, `pointer-events-none` (`EL/🎚️Slider/🟦️.tsx:43`) | `presence_overlay` dims the whole node at 0.35 (`…/📶️…/🦀️.rs:17-20`) | Dim strength approximate. | low |
| Accessibility | `role=slider`, `aria-valuemin/max/now/text`, `aria-orientation` (`EL/🎚️Slider/🟦️.tsx:556-566`) | Contract `slider` role, value from `WG/♿️accessibility/🦀️.rs:313-328` | Matches. | low |

---

## 10. NumberStepper (`Component::NumberStepper`)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Mixed placeholder | `useLabel("ui.common.mixedValues")`, placeholder `mixedLabel ‖ "—"` when mixed and not edited (`EL/🪜️Stepper/🟦️.tsx:74,281`) | `UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed"` hard-coded English (`RU/📜️declaration/🧩️ui-graph/🔍️inspector-helpers/🦀️.rs:22`), used at `…/🎚️…/🦀️.rs:147` and `EL/🪜️Stepper/…/🦀️.rs:38` | Not localised; React falls back to `—` when the label is empty. | med |
| Chrome | `h-medium`, border, two `w-medium` buttons with `hover:bg-muted`, `focus-visible:bg-muted`, disabled `opacity-50` (`EL/🪜️Stepper/🟦️.tsx:257-354`) | `push_stepper_chrome` (`EL/🪜️Stepper/…/🦀️.rs:52-68`): hover fill `button_hover`, divider edges, focus border `accent`. Retained: `…/🎚️…/🦀️.rs:117-127`. | Hover fill colour is `button_hover` vs React `hover:bg-muted` (**unverified** token mapping). Disabled is a whole-node dim only. | low |
| Value text | `text-base md:text-sm`, centred, `formatUiNumberFixed` (`EL/🪜️Stepper`) | `stepper_value_text` → `ui_number_display_text` (`EL/🪜️Stepper/…/🦀️.rs:23-27`), retained `stepper.value_text` (`…/🎚️…/🦀️.rs:145`) | Matches. | low |
| Keys | `uiNumberFieldKey`: arrows, Shift ×10, page keys to detents, Home/End (`EL/🪜️Stepper/🟦️.tsx:182-191,306-321`) | `WG/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:187` `number_field_key` | Matches. | low |
| IME | native input | composition inserted in retained label (`…/🎚️…/🦀️.rs:138-143,163-168`) | Matches (better than Input). | low |
| Focus ring | border `accent` when focused | `NodeFlags::FOCUSED` (`…/🎚️…/🦀️.rs:149-150`) | Matches. | low |
| Accessibility | `aria-valuetext`, `aria-invalid`, `aria-describedby` on the input (`EL/🪜️Stepper/🟦️.tsx:325-330`) | `WG/♿️accessibility/🦀️.rs:332-333,368-377` `spinbutton` projection | Matches. | low |

---

## 11. Ring (`Component::Ring`, orb `t` in [0,1))

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Zero position | `angle = t·2π − π/2` (`EL/⭕️Ring/🟦️.tsx:58`): **t=0 at 12 o'clock**, clockwise | Paint: `angle = TAU·t` (`WG/🖌️…/🎚️slider-stepper-and-progress-nodes/🦀️.rs:227`). Kit: `EL/⭕️Ring/…/🦀️.rs:38-40`. Both start at **3 o'clock**. | For the same contract value the knob is drawn **90° apart** in the two renderers. Drag maths is also off by 90°: React `atan2+π/2` (`EL/⭕️Ring/🟦️.tsx:101-103`), wgpu `atan2` (`WG/🧮️layout/🦀️.rs:486-490`; kit `EL/⭕️Ring/…/🦀️.rs:32`). | high |
| Track | `stroke-muted-foreground/30`, radius 40 of a 100 viewBox (`EL/⭕️Ring/🟦️.tsx:86-87,187`) | 48-segment polyline with `theme.separator` (`…/🎚️…/🦀️.rs:208-216`, kit `…/🦀️.rs:19-27`), radius `0.4·min(w,h)` | Track colour: `muted-foreground @ 30%` vs `border_normal`. Radius ratio matches (0.4). | low |
| Orb | `r = 5` (unselected) or `7` (selected) in viewBox units, `fill-foreground` / `fill-accent` when selected, `fill-muted-foreground/40` disabled (`EL/⭕️Ring/🟦️.tsx:57-80`) | Fixed 12px square with radius 6 in `accent` (or `text_muted` disabled) (`…/🎚️…/🦀️.rs:228`, kit `…/🦀️.rs:42`) | Orb size and colour differ from React; selected/hover visuals missing. | med |
| Disabled | `pointer-events:none`, `cursor-not-allowed` (`EL/⭕️Ring/🟦️.tsx:71,75`) | Disabled suppresses drag hit only (kit `…/🦀️.rs:43-45`) | Matches in behaviour. | low |
| Keyboard | **None** in React (no `tabIndex` or key handler in `EL/⭕️Ring/🟦️.tsx`) | **None** handled. But Ring is listed as focusable (`WG/⚡️events/🧭️event-model-and-focus/🦀️.rs:306`), so it takes Tab focus with no key action and no focus paint (`paint_ring_node_step` takes no flags, `…/🎚️…/🦀️.rs:200-207`). | Shared accessibility gap. The contract says role `slider` but nothing is keyboard-operable. wgpu has the extra focus-without-visual defect. | high (a11y) |
| Accessibility | `role` from contract (`slider`) on an unfocusable SVG | contract `slider` value min 0, max 1, now t (`CT/♿️accessibility/🦀️.rs:239`) | Shared; both fail operability. | high |
| Orb id / selected | `orbs=[{id: orbId, selected: true}]` (`RI/✍️input-views/🟦️.tsx:471`) | `orb_id` is reconciled (`widget-nodes:142`) but never painted | Multi-orb is not supported in either. Parity for the single orb. | low |
| Pointer | `pointerdown/move/up` on orb and svg (`EL/⭕️Ring/🟦️.tsx:164-166`), release via `window pointerup` (`RI/✍️input-views/🟦️.tsx:459-470`) | Drag axis `DragAxis::Ring` (`EL/⭕️Ring/…/🦀️.rs:44`), `ring_t_at` (`WG/🧮️layout/🦀️.rs:486`) | Same geometry bug as above. | high (via angle) |

---

## 12. IconSelect (`Component::IconSelect`, classifier puzzle2d | default)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Control | `IconSelector` element: trigger + popover, mode select (`EL/🎴️IconSelector/🟦️.tsx:202`), grid with `aria-labelledby` (`:232`), `classifyIconSelectorMode` for puzzle2d (`:114,118`) | `paint_icon_select_node_step` `WG/🖌️…/🎛️control-nodes/🦀️.rs:331-375`: bordered box and the **value string** painted as text (`:353`). Kit: `EL/🎴️IconSelector/…/🦀️.rs:17-28` draws the icon for the value or its text. | No picker, no icon grid, no classifier mode, no uniform/mixed handling (`_uniform`, `_classifier_kind` ignored in the kit at `:17`). The user sees a text name instead of the icon. | high |
| Keyboard | Popover and grid keys in the element | Focusable (`WG/⚡️events/🧭️event-model-and-focus/🦀️.rs:306`), no key handling and no focus paint | Shared-looking gap, but wgpu is worse: focus without a visible ring. | high |
| Accessibility | `aria-labelledby` the label (`RI/✍️input-views/🟦️.tsx:487-495`), `sr-only` span | Contract `radiogroup` role and text value (`CT/♿️accessibility/🦀️.rs:46,240`) | Role exists in wgpu, but there are no radio children to match. | med |
| Disabled | `disabled` locks the picker (`input-views:490`) | Hit region always registered (kit `…/🦀️.rs:29`) | Click-through when disabled **unverified**. | med |

---

## 13. Progress (`Component::Progress`, determinate | indeterminate)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Track / fill | `bg-muted h-tiny rounded-full`; fill `bg-accent` at `fraction·100%` (`RI/🔤️display-views/🟦️.tsx:192-214`) | `paint_progress_node_step` → `paint_progress` (`WG/🖌️…/📶️…/🦀️.rs:131-138`), `progress_bar_rects_of` (`:72-83`): height `SIZE_TINY`, `muted` track, `accent` fill, `border_radius` (0 per design). Same law shared with kit (`WG/🪀️widgets/🦀️.rs:264-270`). | Matches. Fraction law is the shared `ui_contract::progress_fraction`. | low |
| Indeterminate | `w-1/3 mx-auto` fill with `motion-safe:animate-pulse` (`display-views:211-214`) | Static centred block of `PROGRESS_INDETERMINATE_SHARE` (`progress-celebrate:77-80`), **no animation** | No pulse/sweep in wgpu. | low |
| Accessibility | `role=progressbar`, `aria-valuemin/max/now/text`, `aria-busy` when indeterminate (`display-views:198-206`) | Contract `progressbar` role and `AccessibilityValue` (`CT/♿️accessibility/🦀️.rs:47,175,241`) | Matches. | low |
| Busy state | `aria-busy` | n/a | — | low |

---

## 14. Image (`Component::Image`, src, alt, sourceBodyBytes windowed resource)

| Aspect | React (file:line) | wgpu (file:line) | Gap | Sev |
|---|---|---|---|---|
| Source | `<img src>` for any URL (`RI/🔤️display-views/🟦️.tsx:107-109`) | Retained: `admit_ui_image` decodes **only `data:`** (`WG/🖌️…/🖼️media-and-slot-nodes/🦀️.rs:17-20`). Kit: `render_image` likewise (`WG/🪀️widgets/🦀️.rs:275-281`). Any other src is not fetched. | `http(s)`, `blob:` and relative URLs never load in wgpu. | high |
| Windowed bytes (`sourceBodyBytes`) | Paged resource reader with `<progress>` during load (`display-views:126-178`) | Reconcile resolves a retained body into `source_page` (`RU/🌳️…/🎞️media-transport…/🦀️.rs:198-203`); no progress bar painted | Pending-load progress not drawn. | med |
| Fit | `max-h-64 max-w-full rounded-md object-contain` (`display-views:108`) | `ui_image_content_rect` with `UI_IMAGE_MAX_BOX_HEIGHT=256` and contain (`WG/🖌️…/🖼️ui-image-sources…/🦀️.rs:33,197-200`) | Max box matches (256 = 16rem). No `rounded-md` corner in wgpu. | low |
| Fallback | Browser broken-image / empty `alt` (`display-views:108`) | Placeholder panel with `alt`, **or the node id** when no alt (`WG/🪀️widgets/🦀️.rs:283`) | The kit leaks the record key as visible text. The retained fallback is **unverified**. | med |
| Accessibility | `alt` on `<img>` (`display-views:108`) | Contract `img` role (`CT/♿️accessibility/🦀️.rs:51`) | Matches. | low |

---

## Prioritized fix work items

Severity order. Each item names the file(s) to change and the React reference to copy.

**P1 (high)**

1. **Ring angle convention (cross-renderer mismatch).**
   - Change `WG/🖌️paint/🎨️node-painting-with-driver/🎚️slider-stepper-and-progress-nodes/🦀️.rs:227` from `TAU·t` to `TAU·t − π/2`.
   - Change `EL/⭕️Ring/🎯️targets/🧊️wgpu/🦀️.rs:32` (drag: `(atan2 + π/2) rem_euclid TAU / TAU`) and `:38` (knob angle `TAU·t − π/2`).
   - Change `WG/🧮️layout/🦀️.rs:486-490` (`ring_t_at`) to add `π/2` before the modulo.
   - Reference (copy this convention): `RU/🧱️elements/⭕️Ring/🟦️.tsx:58` (`angle = t·2π − π/2`) and `:101-103` (drag `atan2 + π/2`).
   - Add a conformance test that t=0 is at 12 o'clock.

2. **Segmented Select is painted as a dropdown.**
   - Add `appearance: SelectAppearance` to `RU/📜️declaration/🧩️ui-graph/⌨️input-select-toggle-and-group-nodes/🦀️.rs:91-104` (`UiSelectNode`).
   - Map it in `RU/🌳️retained/🌳️tree/🔀️reconcile/🎛️widget-nodes/🦀️.rs:49-60`.
   - Add a segmented branch in `WG/🖌️paint/🎨️node-painting-with-driver/🎛️control-nodes/🦀️.rs:114-275` painting one row of options with the chosen one filled, matching the ToggleGroup in React.
   - Reference: `RI/✍️input-views/🟦️.tsx:342-362`, `EL/🎛️ToggleGroup` (`RU/🧱️elements/🎛️ToggleGroup/`).

3. **IconSelect shows a text value, not the icon picker.**
   - Replace the text paint in `WG/🖌️paint/🎨️…/🎛️control-nodes/🦀️.rs:353` with at least the chosen icon via `push_icon` (as `EL/🔘️Button/…/🦀️.rs:23-26` does).
   - Implement the grid/popover (open on activate, arrow keys, classifier mode puzzle2d) in the events layer `WG/⚡️events/⌨️widget-keyboard-and-commit/`, and the paint in `WG/🖌️paint/🎨️…/🎛️control-nodes/`.
   - Mirror `EL/🎴️IconSelector/🟦️.tsx:118-251` (mode select at `:202`, grid at `:232`) and `RI/✍️input-views/🟦️.tsx:476-498`.
   - Also fix the kit `EL/🎴️IconSelector/🎯️targets/🧊️wgpu/🦀️.rs:17-30`, which ignores `uniform` and `classifier_kind`.

4. **Text input: IME preedit and selection not painted.**
   - In `WG/🖌️paint/🎨️…/🎛️control-nodes/🦀️.rs:59-70`, insert `edit.composition` at the caret, as `…/🎚️…/🦀️.rs:138-143` already does for the stepper.
   - Paint the selection rectangle for `anchor != caret`, using the logic in the test-only `WG/🖌️paint/🔬️reference/🦀️.rs:276-320` promoted into the production painter.
   - Reference: native input behaviour, `RI/✍️input-views/🟦️.tsx:296-321`.

5. **Number/text refusal message not painted.**
   - In `control-nodes:60-87`, paint `edit.number_refusal` text below the field with `theme.error` at `font_size_small`, as the slider does (`…/🎚️…/🦀️.rs:39-45`).
   - Reference: `RI/✍️input-views/🟦️.tsx:327-331`.

6. **Slider: no hover, focus or drag visuals.**
   - Pass `flags` and `presence` into `paint_slider_node_step` (`…/🎚️…/🦀️.rs:7-19`) and paint the thumb `bg-emphasized` on hover, `bg-active-base` when dragging, and a focus border when focused.
   - Reference: `EL/🎚️Slider/🟦️.tsx:38-45`.

7. **Ring and IconSelect are focusable but have no focus paint or keys.**
   - Either remove them from the focusable list at `WG/⚡️events/🧭️event-model-and-focus/🦀️.rs:306` or add key handling (Arrow/Home/End/PageUp/Down, matching the slider law in `WG/⚡️events/🧮️control-commit-rules/🦀️.rs:262-280`) and a focus ring in `paint_ring_node_step` (`…/🎚️…/🦀️.rs:200`).
   - React has no keyboard for Ring either, so this is a shared gap to raise with product. Decide one behaviour and apply it to both renderers.

8. **Input: multi-line longText shows one line.**
   - Replace `text_line_view` (`control-nodes:68-69,99`) with wrapped multi-line layout (`RetainedTextFlow::Wrap`) and a minimum height of `4.5rem`.
   - Reference: `RI/✍️input-views/🟦️.tsx:276-292`.

**P2 (med)**

9. **Localise the hard-coded placeholders.**
   - `WG/🖌️…/🎛️control-nodes/🦀️.rs:159` and `WG/🖌️paint/🔬️reference/🦀️.rs:348`: `"Select…"` must use the `ui.common.select` string, which the reconcile layer can resolve like `interpLabel`. Reference `RI/✍️input-views/🟦️.tsx:366` and `RI/🎛️declarative-controls/🟦️.tsx:84`.
   - `RU/📜️declaration/🧩️ui-graph/🔍️inspector-helpers/🦀️.rs:22` (`"Mixed"`): carry the localised `ui.common.mixedValues` label into `UiNumberStepperNode` and fall back to `—`. Reference `EL/🪜️Stepper/🟦️.tsx:74,281`.

10. **Button: label hidden when `driver.labels === "icons"`, and tooltip.**
    - In `WG/🖌️…/🎛️control-nodes/🦀️.rs:35` and `EL/🔘️Button/🎯️targets/🧊️wgpu/🦀️.rs:33`, consult the UI driver's `labels` mode (see `WG/🖥️chrome/🦀️.rs:183-212`, which already models it) and draw icon-only when it is `icons`.
    - Reference: `EL/🔳️ButtonGroup/🟦️.tsx:110-121`, `EL/🏷️Label/🟦️.tsx:139-160`.

11. **Button: busy-while-dispatching state.** Show the disabled and `aria-busy` state from an in-flight action. Reference `RI/🔤️display-views/🟦️.tsx:49-89` (`state.busy`). Target `WG/🖌️…/🎛️control-nodes/🦀️.rs:6-44`.

12. **Input: field invalid styling and aria.** Use `context.field.invalid` (the container error) to set the error border, not only number refusal. Reference `RI/✍️input-views/🟦️.tsx:306`. Target `control-nodes:75`.

13. **Image: non-data sources and progress.**
    - Non-`data:` sources need a host fetch, which is a host/scene-host responsibility. Document it as a known gap, or route it through the existing scene host lane.
    - Draw the paged-load progress bar. Reference `RI/🔤️display-views/🟦️.tsx:126-178`. Target `WG/🖌️…/🖼️media-and-slot-nodes/🦀️.rs`.
    - Remove the node-id fallback in `WG/🪀️widgets/🦀️.rs:283`, using `alt` or empty.

14. **Kit path drift.** `EL/🎚️Slider/…/🦀️.rs:118-120` ignores unit, display factor and precision (use `slider.readout` logic from the retained path). Decide whether `WG/🪀️widgets` still ships. If it does, sync it with the retained path.

15. **Input selection and copy/cut.** Verify copy/cut against `WG/⚡️events/✍️drag-scroll-and-edit-api/🦀️.rs`; add it if absent. Reference: native input behaviour.

**P3 (low)**

16. **KeyValue row layout.** Use auto-height rows with wrapping in `WG/🖌️…/📝️text-and-structure-nodes/🦀️.rs:84-110` instead of fixed `control_height`. Reference `RI/🔤️display-views/🟦️.tsx:91-104`.

17. **Ring track colour and orb size.** Use `muted-foreground @ 30%` for the track and orb radii 5/7 scaled to the ring size. Targets: `…/🎚️…/🦀️.rs:208-228` and `EL/⭕️Ring/…/🦀️.rs:15-46`. Reference `EL/⭕️Ring/🟦️.tsx:57-80,187`.

18. **Progress indeterminate animation.** Add a pulse or sweep when reduced motion is off. Target `WG/🖌️…/📶️progress-celebrate-and-widget-painters/🦀️.rs:72-83`. Reference `RI/🔤️display-views/🟦️.tsx:211-214`.

19. **Button label colour on hover.** Use `item_text(theme, pressed, hovered)` for the label in `control-nodes:35`.

20. **Button tone.** Map `style.tone` from `RU/🌳️…/🎛️widget-nodes/🦀️.rs:84` only after confirming React CSS uses `[data-tone]` on buttons (**unverified**).

21. **Disabled dim strength.** Match `opacity-50` on Slider thumb and Stepper (`presence_overlay` alpha 0.35 is an approximation). Target `WG/🖌️…/📶️…/🦀️.rs:17-20`.

---

## Shared gaps (both renderers)

- **Ring has no keyboard operation** in React or wgpu, although its contract role is `slider` (`CT/♿️accessibility/🦀️.rs:44`). Raise with product (see item 7).
- **Refusal and error announcements** are present in React (`role=alert`) and absent from wgpu for the input (item 5). The contract accessibility projection covers the slider and stepper (`WG/♿️accessibility/🦀️.rs:313-377`).

## Unverified (not checked or not verifiable from source)

- Exact pixel geometry beyond token equality (Tailwind class → token mappings for `h-medium`, `px-double`, `gap-y-single`, `rounded-md`, `text-base md:text-sm`).
- React `Field` element internals (`EL/📝️Field/`) band sizes and label semibold weight.
- React `Section`/`Group` folding behaviour (lives in `RI/🧱️node-tree/🟦️.tsx`, not read in full).
- Whether React CSS keys off `data-tone` on buttons.
- Button tooltip participation in the wgpu tooltip subsystem.
- Date, file and colour native pickers in the wgpu host (none found in the painter; host overlay not searched).
- Copy and cut of text-input selection in wgpu.
- Click-through of disabled IconSelect in wgpu.
- The React Select keyboard decision table end to end (the wgpu side states parity in `WG/⚡️events/⌨️widget-keyboard-and-commit/🦀️.rs:60-67`, not re-traced).
- Slider and Ring pointer paths traced through `slider_node_pointer_value` and `ring_t_at` only at the function level.
- Accessibility value mirroring completeness for IconSelect and Image beyond the role.
