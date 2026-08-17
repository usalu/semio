# Layout Findings

The tree direction implementation already reverses section, item, and collapsible-content order for `direction="up"`. Bottom-anchored panels also pin their absolute panel root with an inline `bottom` position and dock the `WindowChrome` cap to the bottom.

The remaining mismatch was inside the panel body. `Scrollable` exposed no way to lay out its viewport from the block end, so its viewport always began at the top. `PanelTreeUnitsPane` also assigned `overflow-y-auto` to every individual tree even though the enclosing `Scrollable` already owned vertical overflow. The combination made the bottom-up semantic order render inside a top-origin, nested-scroll layout.

The durable layout is one vertical scroll owner per panel body. For a bottom anchor, its viewport has a minimum height equal to the scroller and aligns the natural-height tree stack to the block end. Below the height limit, added tree rows consume free space upward. Above the height limit, the viewport grows to its content height and the enclosing scroll area handles overflow.
