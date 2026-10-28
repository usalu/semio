# Window Name Chip Fullscreen Icons

Every teaching fullscreen icon now belongs to its window name chip. Overview topic and video cards display the maximize icon after their localized name; the right-side Ansehen/Watch chip contains only text. Opened video windows display an icon-only native restore button after the localized name, inside the same title chip. The separate right-side restore chip is removed. Body framing and content camera behavior are unchanged.

The shared OverviewCard interface accepts an optional titleAdornment rendered inside its existing name chip. Teaching passes a decorative maximize icon without nesting buttons inside the native card button. Existing callers retain their presentation. The existing OpenChip interface already distinguishes omitted icons from explicit null.

The opened icon uses the canonical windowChromeControlButtonClass and an inset keyboard focus ring. The title chip removes redundant vertical padding around the canonical icon button, preserving its original cap height and body geometry. Long names truncate inside the chip while retaining the full localized title as a tooltip, and the icon button keeps its native width. Only this native icon button restores the window. It keeps the stable `:fullscreen` target, localized Restore/Wiederherstellen accessible label, pointer handlers, and pressed fill on the enclosing title chip.

Browser metadata measures name chips, names, native buttons, and actual SVGs separately, including containment, accessible names, button counts, and absence of right-side fullscreen controls. The title chip supplies typography and pressed-background measurements. The obsolete right-cap rectangle field is removed. Hierarchy probes and virtual fullscreen targets use the actual icon geometry for pointer choreography.

## Validation

The test agent confirmed failures before both implementation stages: the old standalone Restore label and the overview footer maximize icon. The renderer JavaScript syntax check passed through the repo tool interface. Portable Chromium contracts and native browser opening/cancellation at both overview title icons passed. Native bilingual opened-window name/icon and production overview checks passed. Geometry verification caught redundant vertical padding around the canonical icon button; the final layout removes that padding and contains long names. The final frozen-source geometry and pixel regression passed in 8.651 seconds. All focused portable, native overview/opened interaction, two-level production, and geometry checks passed.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🖌️render/🐍️.py`
