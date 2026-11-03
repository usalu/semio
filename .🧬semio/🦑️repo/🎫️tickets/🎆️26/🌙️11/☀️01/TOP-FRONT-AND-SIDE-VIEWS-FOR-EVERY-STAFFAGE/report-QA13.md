# Report QA13: Visual QA of C1 Components

- **Checks:** check and consistency ok for 8 of 8. `coverage.py --consistency` reports complete 8 of 8. The three-views vitest passes 16 of 16.
- **Section conventions:**
  - The curtain-wall cut aluminium is now filled solid dark at 1:200–1:50.
  - The masonry 1:100 cut brick gets a 45° hatch in front and plan.
  - The decisions state the section planes (en/de).
- **garage-door:** the same-region fix exposed hinge bands and the seal below the 0.2 mm minimum, so they were thickened.
- **Writes:** atomic.
- **Inspected:** a subset per asset. The 1:100/1:200 garage-door and blind sheets were mostly unread; a Haiku reader follows.
- **Open:**
  - The sheets label section views "front" and "plan". Decision: schema-level section metadata, so viewer and preview label them "Section A-A" and "Section B-B" (FW).
  - The radiator gradient threshold is already aligned by FW (the feature reads `minSpreadK`).

## QA15 Round

Fixes for the C1 defects of `audit-sheets-C1.md`; patch script `🧰️QA15/patch.py` (atomic writes), driver `🧰️QA15/all.ts`.

- **masonry-wall-with-lintel 1:500 section A-A:** added the light `reveal` block (region `masonry-field`, z 0.5, between sill top and lintel bottom). The 2.75 m wall now reads as one column. Decisions (en/de) extended.
- **glulam-beam 1:500 front:** the 0.16 x 0.62 m end face stays at true extent (same height as the side view) but is now a bold block (outline pen 0.3 mm closes the 0.32 mm face), no longer a hollow glyph. Decisions extended.
- **glulam-beam height:** the overall height is now 0.62 m at every scale (beam 0.60 m plus 0.02 m plate). Fixed the 1:200 bearing lines (elevation y = -0.32, front y = 0) and the 1:100 plates (elevation y = -0.32, front y = 0.0). The dimension label reads 0.62 m at 1:50, 1:100, 1:200 and 1:500. At 1:200 the label now sits clear of the outline, so no framework change is needed.
- **panel-radiator thermal red:** not an asset defect. The thermal view uses the same diverging ramp everywhere and the default range is AUTO (`scaleOf` normalises to the largest deviation present in the figure, the legend says so). The radiator at about 70 degC is the hottest region of its figure, so it gets the extreme red, while other assets peak at orange. A shared range option for the preview would be a framework or preview change; nothing changed in the asset.
- **Checks:** check (office-laser) ok for both edited assets, consistency ok at all four scales, three-views and front-projection vitest 10 of 10 for glulam, masonry and radiator. Sheets read: glulam and masonry self-weight and dead-load at 1:500, 1:200, 1:100 and 1:50, plus a 600 dpi zoom of the 1:500 glulam front.
