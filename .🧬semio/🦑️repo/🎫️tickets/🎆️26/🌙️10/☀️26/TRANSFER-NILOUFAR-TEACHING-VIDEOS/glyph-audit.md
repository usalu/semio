# Portable Mathematical Glyph Audit

## Confirmed Cause

The movie defects are missing font glyphs, rather than animation or math baseline markers. The static unanimated Manim/Pango probe reproduces the same solid white boxes with Georgia. Independent FontTools cmap inspection in glyph-coverage.md and native Cairo glyph-index queries agree that installed Georgia lacks exactly these required symbols: ϑ ← ↑ → ↓ ⇒ ∮. Cooling's plain body_text prefix contains ← and bypasses the math wrapper entirely; Energy Balance's plain axis label ends in →. Physical Fundamentals maps vartheta to ϑ in math_text. The shared math baseline marker is an invisible VectorizedPoint, which does not explain these static plain-text defects.

The probe inventories declared math commands and non-docstring arrow literals. The language-neutral production fixture covers all 44 required symbols, including source math literals and arrows:

%+<=>|~°±²³·×÷ΔΣΦΨηθλμπρτφχψϑ…₂←↑→↓⇒−√∞∮≈≠≤≥

## Implemented Policy

Only manim_fonts.py changed visual behavior. The existing Text kerning interface now queries glyph coverage through Cairo, which is already part of Manim, and applies Manim's existing t2f mapping only to characters absent from the requested face. Installed symbol-family priority starts with Cambria Math, then DejaVu Math TeX Gyre. Other installed mathematical/symbol families remain available as candidates. FontTools is a test dependency only; no runtime font library, bundled font, font download, source scene change, PDF change, timing change, or manim_visuals.py change was introduced.

Georgia remains the body font on this Windows host. The seven missing characters use installed Cambria Math; every supported Georgia/body glyph retains its original face. Explicit caller font substrings and character ranges remain authoritative, including both t2f and text2font forms. A character without any installed supporting candidate fails clearly instead of silently drawing a replacement box.

Parent setup provisions DejaVu Math TeX Gyre on other supported environments. Plain DejaVu Serif is not a complete mathematical fallback because it lacks ∮; DejaVu Math TeX Gyre supplies that glyph while keeping fallback limited to missing characters.

## Test-Driven Evidence

The new language-neutral glyph_fallback.tests.json and test_manim_fonts.py were written before the production change. The baseline produced 22 failing checks and 44 missing-helper errors across 7 tests. After the symbol-only implementation, all 7 tests passed in 6.001 seconds with actual Manim/Pango/Cairo and independent FontTools validation.

Tests cover all 44 symbols, native Cairo coverage against Georgia's real FontTools cmap, actual Pango SVG validity, vector geometry matching a directly rendered supporting-font glyph, German/body cache and geometry stability, affected-string cache invalidation, explicit range overrides, and the shared math_text wrapper. This verifies actual glyph shapes, rather than relying solely on successful imports or PNG existence.

The corrected static frame was independently opened and inspected. It shows every required symbol plus the physical temperature formula, blue Cooling backlink, and Energy Balance arrow label without replacement boxes. It also shows the original German umlauts and ß in Georgia. Its command completed with exit 0.

## SVG Cache Identity

Supported German text retains its exact old cache identity: aeb6907d7f0b131a.svg before and after, with no t2f entries. The originally broken full symbol probe changes from e8f1ae752d215901.svg to 816c6642b0edb509.svg when its seven explicit fallback entries are applied. The production tests independently compare affected strings against the raw previous Text constructor and require changed cache identities; all four supported sample strings require identical cache identities and vector geometry. Old boxed-symbol SVGs therefore are not selected for the repaired strings.

Corrected source-label examples:

| Label | Only Fallback Character | New SVG Cache Name |
| --- | --- | --- |
| ← Lüftung · Kapitel 3: | ← | 17d68d4bdeca4d94.svg |
| berechnet  → | → | a11b95d5e904b645.svg |

## Frozen Helper and Rerender Boundary

manim_fonts.py is frozen at SHA-256 00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2. Parent and portability coordination received this digest before starting the corrected render run. Old full movies must not be relabeled as newly rendered after this visual change. The corrected run owns fresh simulation using the new helper and existing supported text SVGs; affected strings receive distinct new cache keys.

No further helper edits are planned. All owned diagnostic/test commands finished; this agent owns no long render and did not stop any course render or alter a live cache/controller.

## Retained Evidence

Product files: manim_fonts.py, glyph_fallback.tests.json, test_manim_fonts.py. Ticket inputs: glyph-probe.py, glyph-after.py, glyph-cases.json, run-glyph-probe.ts, run-glyph-tests.ts. Generated SVGs, logs, and images stay beneath 🗑️generated/glyph-probe, 🗑️generated/glyph-after, and 🗑️generated/glyph-tests, and may be removed during final generated-output cleanup.

Baseline comparison image SHA-256: 9f424d981e7536a9b6d5072bb956adeef231c677acb30a7c043913e3ec3cf3d2.

Corrected actual-label image SHA-256: a333d783d815ed6b6fb78153f0c4322a980393fc0c14db59690542e8842e5f03.
