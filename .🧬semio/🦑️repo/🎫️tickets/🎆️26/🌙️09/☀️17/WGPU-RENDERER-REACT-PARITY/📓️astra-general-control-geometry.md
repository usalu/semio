# General Inline Control Geometry

Checkpoint14b physically measured React General Select triggers at 160 × 16 logical pixels and WGPU at 120 × 22.4. The root source audit identifies a shared Tree geometry gap, separate from the panel preflight/Select action failures under repair.

React Tree property rows use `STYLING_DOM.controlValueColumnUiSpacing` for their value column (`Tree/🟦️.tsx:230`). That schema token is 50 and the compact spacing is 3.2, producing 160 px. The `tree-item-control` wrapper forces every `data-detail-panel-control=fill` child to width 100%; SelectTrigger publishes that attribute, so General’s authored `w-32` is not the final used width. General authors `h-small`, which resolves to 16 px.

WGPU `layout::TreeRowMetrics::from_theme` instead assigns the literal `TREE_ROW_CONTROL_WIDTH = 120.0` and generic `Theme::control_height = 22.4`. The flex adapter positions every row control with those values. A non-retained tree painter separately repeats 120.0 and the generic height. Both need one schema-token-backed geometry authority.

Do not change every generic control globally to the small height: the React tree cell’s actual control component/authored height must be compared for Input, Select, Stepper, Toggle, and Button. First derive the Tree value-column width from the existing token, then resolve inline control heights from a language-neutral fixture and actual React geometry. Paint, mounted layout, flex, and hit ownership must share that result.

This is a source-backed execution packet, not a completed implementation. No production files were changed by this audit. The source landmarks are UI WGPU layout lines 128–152 and 223–229, flex lines 281–285, paint lines 2816–2817, React Tree lines 225–233 and 2695–2703, Select lines 347–381, and engine ChromePanels General builder lines 381–415.

## Implementation handoff

The follow-up implementation is recorded in `📓️astra-sol-settings-layout.md` under “Token-backed
inline control geometry.” The neutral schema and mounted React oracle now cover Select, Input,
NumberStepper, Toggle, and Button; the focused React receipt is 7/7. WGPU derives the Tree value
column from the shared styling token, carries small versus default control height through mounted
layout/flex, and uses the same rect in the static painter and the actual Shell hit law. Native and
browser receipts remain root-owned.
