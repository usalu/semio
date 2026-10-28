# Font Registration Review

## Findings

1. Bundled font registration succeeds and BODY_FONT resolves to Anta on this Windows host. However, mathematical rendering remains host-dependent. fonts._SYMBOL_CANDIDATES includes Cambria Math, DejaVu, STIX, Apple Symbols, Segoe and FreeSerif; _symbol_fonts enumerates installed host families. These fonts are not fingerprinted by theme.source_files.

2. More seriously, Cairo ToyFontFace coverage queries are fallback-aware on this host. _font_supports(Anta, Greek/arrows) returns true for glyphs absent from every bundled Anta cmap. Therefore selecting only bundled candidate names does not ensure bundled glyph rendering; the coverage oracle can silently accept a system replacement before explicit fallback logic runs.

3. Native family names differ from CSS aliases: Anta Extended/Math/Symbols each declare internal family Anta; Kelly Slab Extended/Cyrillic declare Kelly Slab; every Noto Emoji CSS alias declares Noto Emoji. Registration using actual font metadata is correct, but native code must not request the CSS alias names. Registering multiple subset faces with identical family/style requires an actual cmap aggregate policy rather than assuming CSS alias behavior.

## Concrete Probe Evidence

Ran read-only probes via repo tools exec uv using the existing ticket generated Python environment. Third-party oracle: fontTools.ttLib.TTFont.getBestCmap for every theme.font_files asset, aggregated by internal family. Required symbols come from shared fonts test cases.

All bundled theme fonts combined lack: ΔΣΦΨηθλμπρτφχψϑ₂←↑→↓⇒√∞∮≠≤≥.

Cairo nevertheless reports Anta coverage for: ΔΣΦΨηθλμπρτφχψϑ←↑→↓√∞≠≤≥. A direct _glyph_fallbacks probe returned empty dictionaries for ä, ϑ, λ, Δ, → and ∑. Registered host candidates included Cambria Math, DejaVu Math TeX Gyre, Segoe UI Symbol and DejaVu Serif.

## Existing Repository Solution

`🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf` is already vendored. Internal family: Libertinus Math. fontTools verifies every required_symbols character is supported. Sibling `📖️LibertinusSerif-Regular.otf` also supports the complete set; italic serif lacks ∮. Register the exact math asset, add it to source_files, and use it deterministically for characters absent from the bundled body cmap. This needs no new runtime dependency or downloaded asset.

A repo-owned minimal SFNT cmap reader can query exact file coverage using stdlib; test against fontTools as the development oracle. If a native exact-face query is used instead, verify explicitly that it cannot resolve host fallback. Cairo ToyFontFace is demonstrably insufficient here.

## Dependency Closure

theme.source_files includes theme Python source, canonical tokens JSON, font catalog and every TTF derived from the declared face list. This correctly covers current theme inputs and registered source bytes. Add the existing Libertinus Math OTF when it joins registration. Shared fonts.py and visuals.py remain the runtime closure responsibility. No source mutation performed.

## Native QA Follow-Up: Anta Subset Collision

Viewed generated physical_fundamentals-1.png. Title and much caption text render in Anta, but caption contains tofu for characters such as ä, colon and German punctuation/space. Source caption at physics fundamentals line 701 is: Im Alltag fällt dabei oft das h weg: „4 000 kW verbraucht“.

Independent exact per-file cmap format-4 probe confirms Anta Latin alone covers äöüÄÖÜß×−0123456789:„“ and space. Other same-family subset probe coverage: latin-ext only space; symbols digits and space; math ×−digits colon and space. Registering Latin only is therefore sufficient for German body copy and multiplication, with Libertinus Math for Greek/arrows/mathematical symbols. Unneeded same-family/same-style subsets are a concrete native collision candidate, because ManimPango attaches all registered records by set iteration to each layout. Root is verifying this hypothesis with a Latin-only render; no conclusion is claimed before its output.

A queued fontTools recheck was canceled before execution due memory-capacity contention; the lightweight Node exact cmap probe completed instead. No shared QA outputs or source files were modified.

## Healthy Native Glyph Preview

Inspected generated/o/glyph/glyphs.png after the Latin-one-face registration policy. German umlauts, ß, punctuation and multiplication render cleanly in Anta. Greek letters, arrows, roots, integral and comparison operators render visibly as mathematical glyphs without tofu/zero replacement. This independently supports the native collision fix.

For the independent FontTools outline regression, exact filled-path XOR equality can fail on legitimate native60px hinting/gridfit differences. Compare uniformly scaled combined-bounds geometry using symmetric dense boundary Hausdorff distance, preserving aspect ratio and holes; allow a small pixel-scale tolerance verified against wrong-glyph/tofu negative controls. Do not normalize contours independently or remove legitimate holes. Recommendations sent directly to clip_tests; no test modifications.
