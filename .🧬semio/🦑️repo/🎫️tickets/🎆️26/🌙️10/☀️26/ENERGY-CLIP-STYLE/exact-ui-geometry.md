# Exact UI Geometry

Canonical source review for the existing energy clip ticket.

| Metric | Exact CSS Pixels | Source |
| --- | ---: | --- |
| Navbar / Footer | 28.8 | ui styling palette CSS lines 224–225 |
| Window Cap | 22.4 | palette CSS lines 226–227 |
| Compact Padding / Gap | 3.2 | palette CSS lines 223, 229–231 |
| Small Icon Viewport | 16 | palette CSS line 228 |
| Title Font Size | 11.2 | palette CSS line 240; chromeControlItemBaseClass text-xs |
| Title Line Height | 11.2 | chromeControlItemBaseClass leading-none |
| Border Path Inset | 1 | WindowSilhouette WINDOW_SILHOUETTE_PATH_INSET |
| Normal Border Stroke | 1 | WindowChromeSilhouetteBorder stroke-hairline |

## Placement

WindowChrome cap is full width. Its children are a shrink-wrapped title chip, a transparent flex-1 gap, and a separate right controls chip. The current teaching renderer instead packs its fullscreen button next to an oversized title, making a long transparent strip outside its entire cap. This is the key silhouette mismatch. WindowChrome cap min-height is 22.4 pixels; the current authored cap is 44 pixels. No source supports navbar 60 pixels or footer 34 pixels.

Title sizing must use the shaped Anta layout advance at CSS font-size 11.2 and weight 500, with 3.2 pixels padding on each horizontal side. Ink bounds cannot determine either font-size or cap width. An icon-only windowChromeControlButtonClass is size-medium (22.4 square) and belongs at the far right. Generic WindowChrome enlarge action instead includes a label and uses natural width. Root will select the actual matching instance.

## Proposed Shared Contract

A geometry module takes an explicit rectangle and pre-shaped text advance. It returns title/control/body rectangles, two top chip intervals, content outline at inset 0, and border outline at inset 1. The module uses no drawing libraries. Canonical metrics are read from UI token JSON. A language-neutral schema describes serialized output; independent tests compare it to the actual TypeScript WindowSilhouette implementation and browser CSS measurements.

## Source Files

- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx
- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/🟦️.tsx
- 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/🟦️.ts
- 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx
- 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css
- 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css

## Final Geometry

The shared geometry module and JSON schema are authored at presentation/📐️geometry. `window_geometry(rect, title_advance)` returns pure dictionary data: exact title/control rectangles, body surface, inset content viewport, unclipped and one-pixel-inset border outlines, top-chip intervals, CSS title placement, and canonical metrics. Inputs must be finite; title advance comes from the bundled font shaping interface. The chosen control is the real icon-only `windowChromeControlButtonClass` placed inside WindowChrome `capRightChips`.

The canvas layout follows actual `Canvas` compact padding and HorizontalWindows/VerticalWindows gap: 3.2 pixels for both. Five windows fill the available area in three top tiles and two bottom tiles. Top tile width is (1593.6 − 6.4) / 3 = 529.0666666666666; lower tile width is 795.2. Both rows are 416.4 high, with y origins 32 and 451.6. Focus rectangle is (3.2, 32, 1593.6, 836); inset lesson viewport is (6.4, 57.6, 1587.2, 807.2). Authored content keeps its own aspect ratio through letterboxing.

An initial proposal to center poster-sized windows with larger margins and gaps was rejected after inspecting Canvas and window tiling source. The committed contract uses the source-derived full canvas tiling above.

## Validation

Pure module imports and computes a real top-row window under registered Python with no Manim, NumPy or rendering dependencies. Independent portable and browser parity verification is owned by the ticket test agent; final outcomes will be recorded below after that run.

## Independent Geometry and Typography Validation

The pure geometry module matches the actual React/Chromium fixture at geometry/🧪️tests/🔣️.cases.json within its 0.04 CSS pixel layout tolerance. Title/control rectangles and content/border vertices were checked independently; fractional differences follow browser layout quantization (22.4 CSS pixels becomes 22.390625 layout pixels).

The new typography module at presentation/🔤️typography reads bundled Anta SFNT tables with the standard library: cmap formats 4 and 12, hhea/hmtx, loca/glyf bounds, and GPOS kern lookup selection with glyph-pair/class-pair positioning. No fontTools runtime dependency is introduced. All 12 independently measured Chromium label advances agree to less than 0.000217 CSS pixels. This includes spaces, German umlauts, precomposed accented glyphs, and short strings with very different ink heights.

Anta units per em are 2048; hhea ascent is 1966 and descent is -534. At font-size 11.2 these yield ascent 10.7515625 and descent 2.9203125; Chromium Canvas reports rounded font bounds 11 and 3. Native text keeps the existing bundled Pango outline interface, scales ink to exact SFNT bounds, and positions its alphabetic baseline from font metrics instead of centering glyph ink. A centered CSS line box places the baseline (ascent - descent) / 2 below its center. Public serializable metrics include advance, font size, font ascent/descent, baseline-relative signed ink bounds, and units per em. Native Manim objects remain behind the private helper.

## Final Native Typography

The final native implementation preserves the original Pango glyph vertices using one uniform font scale calibrated from the native H glyph at size 60 and the same bundled font H outline height. It never stretches width or height independently. Cached native glyph copies are positioned individually with exact cmap/hmtx/GPOS layout and baseline-relative glyf bounds. This removes native shaping differences from glyph spacing while preserving the original outline shapes. The earlier whole-label ink-fit approach was replaced before integration.

Pure metrics were rerun after this change: all 12 Chromium reference advances still pass within 0.001 pixels, with observed maximum error below 0.000217 pixels. Geometry and typography declared source files all exist. Root owns final rendered-frame verification and permanent test execution.

## Exact Surface Paint

Actual ui-glass derives each surface from the six canonical level names. The CSS uses Oklab interpolation from appearance base to foreground by level depth times 5%, alpha 1 minus depth times 0.12, backdrop blur depth times 8 pixels, and backdrop saturation 1.45. The earlier native hardcoded dark-8-9 (#07181d) is a different color and does not represent the actual window surface.

The new renderer-independent presentation/🎨️surface module implements Oklab conversion with standard matrices, encoded-sRGB saturation, and alpha composition over an explicit uniform opaque parent. It reads canonical tokens and lists the actual glass stylesheet as a cache input. Uniform backgrounds need no image blur because a constant-color blur remains constant. The returned paint includes floating source RGB, source alpha, blur and saturation, filtered backdrop RGB, and the final opaque composite. Only actual surface regions receive this paint; silhouette U-gaps remain uncovered.

Independent actual React/Chromium measurements over dark base match every level exactly at final 8-bit RGB: base (0,17,23), window (6,25,31), pane (13,31,37), panel (18,35,41), dialog (20,37,43), menu (20,37,43). The window paint is #06191f, correcting the previous #07181d. The module also accepts an explicit parent hex color or RGB channels for overlays. Actual nested CSS glass has an additional backdrop-root and intermediate raster-compositing step: a measured window-over-window red channel differs by one 8-bit unit (browser 7 versus floating uniform-parent formula 6), while green and blue match 26 and 32. The pure API models an opaque uniform parent and does not apply an empirical channel correction for nested browser rasterization.

The independent browser oracle additionally placed an actual WindowChrome over an explicitly opaque #06191f host. That pixel is exactly (6,26,32), matching surface_paint(window, #06191f). The actual nested WindowChrome pixel remains (7,26,32) despite identical computed CSS source colors. This isolates the difference to nested backdrop-root composition rather than the pure color formula. Seven browser cases now match exact 8-bit RGB: six levels over the base plus a window over an explicit opaque parent. The module purpose and API are uniform opaque-parent paint; nested backdrop-root graphs remain a separate renderer responsibility.
