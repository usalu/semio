# Final Style Regression Review

Read-only review of the latest theme, shared fonts and visual chrome. No heavy tests or source changes.

## Result

No new blocking regression found in the reviewed source. Native rendering QA remains responsible for actual SVG/font-map behavior; this review does not claim visual approval of every scene.

## Typography and Registration

BODY_FONT resolves directly to canonical fontStacks.sans first family (Anta). Native font_sources groups declarations by catalog family instead of CSS aliases, so Anta Extended/Math/Symbols correctly register as Anta. Requested and explicit override families register once per exact Path; mathematical fallbacks are chosen from bundled cmap coverage and registered lazily. Host list_fonts and Cairo toy coverage are absent from current admission logic. warn_missing_font=False avoids the known cold enumeration path.

The format 4 index arithmetic matches the SFNT layout: end/start arrays, signed delta, idRangeOffset word-relative glyph address and zero glyph rejection. Format 12 checks sequential glyph ranges. Coverage accepts Unicode platform records and Windows Unicode BMP/full records. The implementation is intentionally limited to supported bundled font cmap formats; oracle tests should continue to verify all registered assets.

The text SVG hash includes every exact bundled outline byte plus repository-relative path, in addition to the existing Manim text hash and small-canvas salt. That invalidates old host-font SVGs. Byte digest caching is process-local; renderer source-state checks prevent successful publication if inputs change during a render.

## Source Closure and Token Parity

theme.source_files includes theme.py, canonical tokens JSON, source-first font catalog, every declared bundled outline and the existing Libertinus Math OTF. Runtime must retain shared fonts.py/visuals.py as reviewed previously. Primary/secondary/tertiary/light/dark/success/warning palette values resolve canonical roles with no duplicate hex table. Type roles read canonical typography metrics at 1.5 presentation scale; chrome stroke reads chromeBorderDefault. UI_PIXEL conversion is consistent with the .2rem spacing scale and rootRemPx.

The closure conservatively includes unused families, so cache may invalidate more often than necessary but will not reuse stale registered font bytes. Shared visual architecture JSON was already imported before this style change and remains a separate existing dependency consideration outside the added theme closure.

## Title Group Compatibility

Searched all authored energy clip Python sources for scene_title calls, title indices, text/font attributes, exact type checks and matching transforms. No caller indexes a title as glyphs or expects Text-only fields. The only direct title attribute operation found is generic to_edge; callers otherwise add/write/fade/group the object or place subtitles relative to it. These accept VGroup. Calculation title helper delegates to scene_title and preserves generic transforms. ReplacementTransform occurrences found affect subtitles, not title glyphs.

window_chip preserves the existing frame/content two-item group contract used by dim_chip. caption_bar preserves background/label grouping. formula_panel preserves its row/box return shape. Square window polygons and fills use shared theme constants.

## Residual QA Boundaries

Multiple Anta subset outlines share native family/style names, so native platform selection still needs actual SVG/render evidence; cmap aggregation alone proves available bytes, not which face the host Pango map picks. The running native QA is the appropriate check. No speculative compatibility layer or source edits were added by this audit.

## Follow-Up: Exact Spacing Outliers

- Shared `🎨️visuals/🐍️.py` math_panel line 2001 still defaults inter-part UI spacing to .18; the adjacent shared chrome uses UI_SPACING=.1 and multiples. This is formula layout, so changing it may alter line fit; use a token multiple only after its representative formula render passes.
- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` line 1038 frames the eta readout with buff=.05 and implicit default stroke, unlike the shared highlight_param buff=UI_SPACING and explicit stroke. This is clearly a text highlight, not physical diagram geometry.
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` line 611 frames r_read with buff=.08 and implicit stroke. Same text-highlight discrepancy.
- `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` lines 3358–3359 frames a table of topic/formula rows with literal buff=.3 and stroke=2.5. Padding is numerically 3*UI_SPACING already, so token expression + canonical chrome stroke would remove future drift without changing initial padding.

Caption/title border padding, subtitle gap and line spacing are already token-driven. Fixed safe bands, panel edge positioning, maximum widths, physical equipment roundings and diagram measurement gaps were deliberately excluded from outlier suggestions.
