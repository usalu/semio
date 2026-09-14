# Icon Hover Chrome Accessory Scope

## Symptom

Hovering navbar toggles, panel/mode tabs, and composite chrome buttons ran icon micro-animations on every nested glyph — including drag grips, fullscreen/focus/close actions, and toggle secondary actions — even when the pointer was only over the primary label/icon.

## Root cause

`🎨️ui.css` `🫨️IconAnim` (and `🔧️IconPartAnim`) propagate hover to all `[data-icon]` descendants of interactive parents (`button:hover :where([data-icon])`, etc.). Chrome accessories share those parents (e.g. `window-pane-chrome-toggle`, `panel-tab-button`) or sit as siblings inside grouped controls.

## Fix

Added `🫨️IconAnimChromeAccessory` rules: when a parent control is hovered, icons inside chrome accessory slots (`drag-handle`, mode-dock tab actions, `action`, `toggle-group-item-action`) stay still unless that accessory itself is `:hover`. Direct `:hover` on `[data-icon]` is unchanged.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
