# Glyph Coverage Audit

Read-only FontTools getBestCmap inspection of installed C:/Windows/Fonts files confirms missing Unicode characters in Georgia. No source/font edits, renders, or dependencies were added.

| Symbol | Unicode | Georgia all four styles | Times New Roman all four styles | Cambria regular | Cambria Math |
| --- | --- | --- | --- | --- | --- |
| ϑ | U+03D1 | absent | theta1 | theta1 | theta1 |
| → | U+2192 | absent | arrowright | arrowright | arrowright |
| ← | U+2190 | absent | arrowleft | arrowleft | arrowleft |
| ⇒ | U+21D2 | absent | absent | arrowdblright | arrowdblright |

Georgia contains ordinary θ, Δ, η, ρ, ü and ·. Cambria bold/italic/bold italic lack ⇒; regular and Math are the two faces of cambria.ttc and cover it. Times New Roman solves the three observed symbols but not every math command.

Exact Cooling prefix is "← Lüftung · Kapitel 3:" in energy/demand/4_cooling/3_transmission_humidity/scene_3.py:531, repeated at 542 and 567. Lines 576–577 use chapter 5/4 variants. _back_chip line 157 passes this prefix directly to body_text in AIR blue. Other Cooling chapters use the same arrow. The initial unresolved box corresponds to ←.

manim_visuals.py:1511 maps vartheta to ϑ. Ordinary theta maps to θ, which Georgia supports. The visible Energy Balance berechnet-arrow suffix contains →; exact full-string search did not locate a combined literal.

manim_fonts.py:92–118 selects installed family without glyph-coverage validation, preferring Georgia and reusing cached selection. manim_visuals.py:1656–1663 creates an explicitly invisible VectorizedPoint baseline marker. Cooling uses plain body_text, bypassing that wrapper: its missing initial arrow cannot arise from the math marker. Cmap absence matching observed symbols strongly supports missing glyphs over animation artifacts. Cmap coverage alone does not establish Pango fallback behavior; pipeline_execution owns the tiny render/Pango proof and fix.

All audit commands after launcher discovery ran through ticket/🗑️generated/launcher/repo.exe tools exec bun and existing run.ts to root .venv/Scripts/python.exe. FontTools was already installed and only read local fonts. This Markdown report is the only written artifact.

## Complete Authored Symbol Coverage

AST inspection extracted every _MATH_SYMBOLS value and non-docstring Unicode math/Greek/arrow literals in authored Python, excluding node_modules/.agents/.venv. Comments are absent from the AST; emoji docstrings were explicitly excluded. This conservative complete literal inventory also includes non-rendered log strings, so direct Text/body_text/math/helper arguments were tracked separately. Dynamic composed strings are bounded by the complete literal union.

Required conservative set: `%+<=>|~°±²³·×÷ΔΣΦΨηθλμπρτφχψϑ…₂←↑→↓⇒−√∞∮≈≠≤≥`.

All declared values, in command order: `· ≈ × → ⇒ ↑ ± ≤ ≥ Δ Σ η φ φ ρ λ θ ϑ μ ∞ % Φ Ψ ψ χ … ∮ π`.

Direct string arguments to Text/body_text/centered_body_text/math_label/math_text/equation_row/symbol_token/note_line/chip/_back_chip/beat_subtitle contain: `+<=>°·÷Δθτ←→↓−√≈≠`. Variable-bound strings may appear only in the conservative union.

| Font fixture | Missing declared symbols | Missing direct-call characters | Missing conservative full set |
| --- | --- | --- | --- |
| Georgia regular | ϑ ↑ → ⇒ ∮ | ← → ↓ | ϑ ← ↑ → ↓ ⇒ ∮ |
| Times New Roman regular | ⇒ ∮ | none | ⇒ ∮ |
| Cambria Math, cambria.ttc face 1 | none | none | none |
| Matplotlib DejaVu Serif regular | ∮ | none | ∮ |

The DejaVu Serif test fixture exists at C:/git/semio/.venv/Lib/site-packages/matplotlib/mpl-data/fonts/ttf/DejaVuSerif.ttf. Its cmap covers every currently observed direct arrow and vartheta plus declared double/up arrows, but does not cover contour integral ∮ U+222E (the supported oint command). It is therefore insufficient alone as the complete portable math fallback. No fonts were registered/copied or installed.

Additional direct down-arrow sources: energy/demand/4_cooling/5_systemauslegung/scene_5.py:444 and energy/demand/3_heating/5_solar_heat_gain/scene_5.py:1439. Additional Georgia-declared gaps are ↑ U+2191 and ∮ U+222E. The regular Cambria Math fixture covers the entire declared and authored set.

The original audit input is retained at ticket root as `glyph-all.py`; its derived `🗑️generated/glyph-all.json` is transient output removed during generated cleanup. Both parent and pipeline_execution received the full required set and fallback gaps.
