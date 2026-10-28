# Final Independent Source Audit

Reviewed latest shared visuals, theme, fonts, intro, and heating calculation while implementation and representative visual QA proceed. This audit made no source changes and did not run long tests. Findings were sent to root and clip_style.

## Reviewed Correctness

- Canonical window outline uses pixel coordinates before scaling to scene coordinates, so its half-pixel edge tolerance is not accidentally applied to full scene units. Fused top-left title chip geometry and intro content placement are coherent: heading centers within chip depth, content occupies the body beneath it, and arranged card groups preserve their internal placement.
- Theme chrome surface is dark-8-9 #07181d; default foreground light #f7f3e3 and secondary #34d1bf have clear contrast. Caption backgrounds inherit the shared surface and retain fill while dimming. Formula backgrounds use the same surface and sit at z-index -1 behind formula fragments. Representative rendering should confirm the intended layering against other scene graphics.
- Heating calculation `_heading` now delegates to `scene_title`; all five callsites supply only text, so removal of its previous size/buff parameters has no stale-keyword callsites. Its label helper uses the common face and label size.
- Intro now reads shared typography and palette tokens, uses window_frame, and has a fused series cap. Independent old palette and literal hierarchy drift identified in the earlier audit is resolved.
- Shared font handling registers checked-in faces and hashes bundled font bytes. Mathematical glyph coverage is resolved from bundled OpenType cmap records rather than host font enumeration.

## Remaining Actionable Style Coverage

Local backlink and math chips remain unfilled SurroundingRectangle wrappers with stroke 1.8, while shared chip uses filled window_frame and canonical chrome border 1. Examples: cooling summer-strategy line 122, transmission-humidity 196, system-sizing 166, ventilation mechanical 173. Their corners are square now, but their surface/border spacing still differs. Route these wrappers through a shared frame while preserving the existing `[box, content]` group shape.

Cooling internal-gains `source_card` around 850 retains an independent square card and free heading; only shared stat_card/watt_anchor currently fuse title caps. Decide whether these pedagogical source cards belong to the same chrome contract. Real equipment outlines and temporary highlight rings should retain their illustrative role.

## Visual QA Target

The heating calculation title around line 1329, `Anlagenverluste: vom Heizwärmebedarf zur Endenergie`, now uses the common sans title helper. That helper has no maximum width/wrapping, so verify this long title fits the video frame; this is a measured-render concern, not a confirmed overflow. Representative intro and formula/caption inspection remains the authority for visible spacing.
