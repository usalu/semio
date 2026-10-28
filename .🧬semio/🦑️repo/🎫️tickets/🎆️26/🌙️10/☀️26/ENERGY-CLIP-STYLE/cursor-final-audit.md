# Final Native Interaction Audit

Independent read-only source and image review on 2026-10-26.

## Native Frames

Inspected hover, press, clicked, tree-hover and navbar-hover PNGs in generated/cursor-tests/native-frames. Canonical two-part arrow cursor remains aligned with its actual control hotspot. Hover uses gray fill and cream emphasis; press/active uses primary. The Maximize/Minimize control fits its fused cap without overlap. Unrelated lesson windows remain gray while the hovered window is cream or selected primary. Tree hovered leaf and ancestor label/guide emphasize while sibling rows remain gray. Navbar trigger hover correctly opens the tree; it is absent from ordinary window interactions.

## Event And Geometry Review

The pure model chooses one deepest/topmost enabled target clipped by all real ancestors. Its ancestry path includes root to target and excludes siblings. Click commits only after same-target press/release and explicit click; fullscreen zoom remains zero until a committed fullscreen effect. Cursor paths load actual dark UI SVG assets without recentering their hotspot. Native authored helper similarly draws local cursor paths around the unshifted pointer anchor, emits enter/move/press/release/click/leave and invokes equipment callbacks only after click produces an effect.

## Actionable Finding

Reported interaction/🎬️scene _draw_control opacity ordering to the coordinating agent. Its final mob.set_opacity overwrites the frame fill opacity set immediately before it, turning canonical transparent normal control cells opaque. Apply overall family opacity before explicit stroke/fill paints, or use relative opacity without accumulating it. This does not affect the already inspected static shell frames, but does affect authored equipment control rendering.

Final authored equipment and tour visual review pending.

## First Tour Review

Independently decoded cursor-ui.mp4 at 0.5, 2, 4, 6, 8, 10 and 11.5 seconds. Gestures and cursor placement are coherent. Fullscreen activation precedes expansion. Heater stays at 0 W before committed click, then shows An, 1500 W and heat flow; System and Heizgerät ancestor frames emphasize while the pointer is inside. Outgoing restoration returns to the overview without obstructing Navbar/Footer.

Additional visual finding reported: at 10 seconds rising red heat waves overlap the end of the Heizgerät title label. Move or constrain the plume within its device body so moving scientific marks cannot cross the title cap. This pertains to the ticket demonstration asset.

Opacity finding resolved in current authored helper source: frame fill uses canonical fill_opacity multiplied by disabled opacity; label family base fill/stroke alpha is captured and scaled without accumulation. The blanket group opacity reset is removed. Final refreshed tour visual remains pending.

## Refreshed Final Review

Decoded the refreshed cursor-ui.mp4 independently with PyAV and inspected 1.5, 2.2, 4, 6, 7.2, 8 and 10 seconds. The tree remains open under its hovered row, closes after the pointer leaves, and emphasizes only the actual row ancestry. Fullscreen icons remain hollow. Parent window/control hover and selected primary treatment remain clear. At 6 seconds the heater is off/0 W; the committed state shows An/1500 W afterward. Heat waves now stay below the Heizgerät cap label throughout the inspected active samples; the reported overlap is resolved.

The event report confirms release and click share 1.700 seconds; the physical equipment callback follows the committed click. Independently inspected cursor-physics.png: actual production Staubsauger Start/An states preserve cursor placement and scientific chart labels, and the activated chart advances from 0 to 1.00 kWh. No clipping or collision was found.

All identified findings are resolved. No actionable source or visual concerns remain in the final reviewed assets. SVG nominal viewport icon scaling is under a separate coordinating-agent geometry check; it does not change control hit rectangles or the interaction hierarchy reviewed here.

## Final SVG Viewport Review

Read-only review of the final renderer confirms a nominal 24×24 invisible SVG viewport is retained, centered with the glyph and scaled to iconSize. Stroke width follows the native 2×iconSize/24 ratio, hollow paths retain zero fill, and the separate control cell hit rectangle is unchanged. The final retained cursor-ui-click.png and cursor-ui.png show the maximize glyph centered inside each cap control with the expected diagonal orientation and clear inset. Pointer hotspot remains at the clicked control, with cream hovered ancestry and primary pressed control. No clipping, unintended fill or sibling emphasis appears.

Final conclusion: the refreshed retained assets and canonical source integration pass this bounded independent audit. No actionable findings remain.
