# Independent Surface Level Review

## Initial findings

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:93`: every interactive ink block directly paints `bg-background/90`. This bypasses level and fill ownership, including on text and math blocks, and hard-codes base paint on top of the canvas. Determine whether blocks are content or independent overlays; inherit the host in the former case, or publish the appropriate shared surface level in the latter. The resize handle at line 191 is an interaction affordance and should be assessed separately.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🔷️diagram-node/🟦️.tsx:94`: workflow node bodies directly paint `bg-background` under a base-level diagram host (line 190). A node needs an explicit owner and suitable level if it must obscure edges; a direct base fill does not participate in the shared level contract. This is an editor node with ports and controls, distinct from an authored image.

These are source findings, not a claim of a reproduced visual regression. Both should be checked against overlap and contrast in light and dark when changed.

## Cross-check

- Inspected the generated dark Excel and light Grasshopper quiz runs. Ordinary rows, lists, grid areas and task bodies visually inherit the base. Red cells and figures are selected exercise content, not neutral UI fills. This supports the narrower browser findings in `web-levels.md`; it does not independently reproduce every interaction or all ten quizzes.
- Shared `WindowChrome` computes a content level and a next-level chip and publishes a `SurfaceScope`; `Surface` suppresses redundant same-level in-flow paints while permitting explicit overlays. The language-agnostic `surface-levels.feature` describes single-owner, next-level chip and transient repaint laws, and `frame-structure.test.tsx` exercises the React frame. This review did not rerun those suites or independently verify a third-party comparison.
- OS renderer CSS imports shared styles and contains no independent declarations; its remaining relevant direct fills are in the editor components above. The animate presentation stylesheet paints a base root and a presentation canvas; the latter is presentation content. The archived Mit Bestand presentation includes third-party copied webpage CSS with white fills; it is not evidence of a maintained application surface.
- The web evidence reports actual Staffage and course browser runs, while Play and demonstrator acceptance updates lack fresh release/wasm verification. This review did not verify those additional running surfaces.


## Resolution

Both editor findings were addressed after the review. Ink blocks, handles and workflow nodes use the opaque ambient surface when they must cover underlying canvas content. The editable canvas hosts share their enclosing window. The ink editor, selection toolbar and presence chips use the next level, while completion and context menus publish menu glass to their children. Browser DOM tests check an ink document inside a real window scope, including its open text editor.

Chromium subsequently loaded the Play and demonstrator landing pages from their production composition configs. Across both appearances, every floating overview card on both sites published dialog glass and menu-level chips. The Play page contained 150 cards and the demonstrator eight. Full per-application plugin activation and release distribution were outside that landing check.
