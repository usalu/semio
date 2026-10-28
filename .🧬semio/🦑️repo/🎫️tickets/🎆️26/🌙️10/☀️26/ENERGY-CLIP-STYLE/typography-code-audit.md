# Typography Code Audit

Read-only review of shared font cmap parsing, glyph fallback, text SVG cache salting, and existing neutral/reference tests. No expensive tests ran and no source was edited. Findings sent to root; cmap branch coverage suggestions also sent to clip_tests.

## Concrete Edge Case: Format 4 Direct U+0000 Mapping

In `🏗️authoring/🔤️fonts/🐍️.py`, `_cmap_supports` around lines 94–98 assigns `glyph = codepoint` for idRangeOffset zero, then requires glyph nonzero before adding idDelta. A valid direct segment mapping U+0000 with nonzero idDelta is therefore falsely reported missing. Direct mapping must evaluate `(codepoint + idDelta) & 0xffff` directly. The glyph-zero-before-delta rule applies only to indirect glyphIdArray entries. Ordinary authored strings do not use NUL, so this does not imply visible current clip damage, but the SFNT algorithm should be exact.

Other reviewed format 4 arithmetic is correct: end/start/delta/range arrays use correct offsets; indirect addressing is relative to its own idRangeOffset word; nonzero indirect glyph values receive signed delta modulo 65536, and mapped zero is missing. Format 12 correctly checks inclusive Unicode group boundaries and derives sequential glyph indices, including astral codepoints.

## Concrete Default-Font Consistency

`_kerned_init` sets `requested` to the bundled sans default around line 153, but its fallback call around line 170 independently defaults to the generic name `sans`. Without a font keyword or applied Manim defaults, it registers the bundled sans yet checks generic sans as unsupported and assigns mathematical fallbacks to ordinary supported text. Use the same resolved requested face for registration, coverage, and the default constructor font. Normal authored scenes call apply_body_font, so current render paths often mask this discrepancy.

## Cache Review

The original Manim hash is salted with bundled outline bytes and source paths. Low-resolution output gets a separate minimum-canvas salt; ordinary canvas profiles retain their shared base identity. Explicit text-to-font mappings are preserved and reflected in the original hash. No concrete cache identity collision or restoration bug was found by source inspection. Font data and digest are cached for the process lifetime; existing render source fingerprints reject output when authored inputs change during a render.

## Neutral Test Coverage

Existing cases compare German text, mathematics, arrows, emoji, and astral mathematical Q against FontTools getBestCmap and native/Pango output. This is useful third-party validation and exercises real bundled assets. It does not force the exact direct/indirect format 4 branches or all boundary semantics. Add small neutral synthetic SFNT/cmap vectors for direct zero plus delta, indirect zero remaining missing, indirect nonzero plus delta wrapping to zero, indirect nonzero delta mapping, and astral format 12 first/last/outside boundaries, and validate those same vectors through FontTools.

The long-title concern in final-audit.md is resolved by the current scene_title maximum width of 13.2 and square title chip; this review did not repeat visual QA.
