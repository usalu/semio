# 🔣 Section 5: Parentheses rendered as brackets in on-screen text

## Verdict

**Bug, not by design.** No code maps `(` to `[`, and the bundled faces draw `(` as a curved paren. But a rendered frame shows `[` where the source has `(`. The root cause is not isolated. The frame may come from a different face path or a stale render. Re-render and zoom to confirm.

## 1. Font setup evidence

- `🏗️authoring/🔤️fonts/🐍️.py:214` builds `f"[{index}:{index + 1}]"` keys. These are Manim `t2f` index-range syntax, not glyph substitution. No `replace`, `translate` or `maketrans` on `(`/`)` exists in `energy/` (grep, no matches).
- `🔤️fonts/🐍️.py:138-161` (`_font_supports`, `_glyph_fallbacks`) substitute only characters that the requested face lacks, via cmap lookup. `(` is in the Anta latin cmap, so it is not substituted.
- `🎨️theme/🐍️.py:16,43-50`: body font `Anta` (from `fontStacks.sans`); math face `Libertinus Math`.
- `🔤️fonts/🧪️tests/🔣️.cases.json`: `required_symbols` and `rendered_glyphs` contain no `(` or `)`. The `rendered_glyphs` outline test (`🧪️tests/🐍️.py:161-177`) therefore never checks parentheses.
- Outline check (`audit2/paren_shape.py`, project venv fontTools): in the bundled faces the paren glyphs are curved (`qCurveTo`) while bracket glyphs are straight (`lineTo` only):
  - Anta latin: `parenleft` qCurveTo x4, `bracketleft` lineTo x7
  - Share Tech Mono: `parenleft` qCurveTo x4, `bracketleft` lineTo x7
  - Kelly Slab and Libertinus Math: same pattern
  - Anta `➕️latin-ext`, `🔣️symbols`, `🧮️math` and Kelly Slab `🪆️cyrillic` have no `(`/`)` cmap entries.

## 2. Parenthesis strings in on-screen text

Scope: `energy/**/🐍️.py`, minus tests, narration/speech/roadmap, and LaTeX or math strings (`$`, `\`, `^`, `_`, `{`, `}`, or math_* callers). Source: `audit2/paren-scan.jsonl` (AST scan). The `@(` debug strings in `visuals/🐍️.py:2382,2459` are excluded.

| Folder | Count | Call paths | Examples (file:line, string) |
|---|---|---|---|
| 🧲️physics | 15 | `tr` 12; citation/source literals 3 | `🧲️fundamentals/🐍️.py:1968` "Luft-WP (JAZ)"; `:735` "Shower (instantaneous heater)"; `:3185` "Willems WM, Schild K, Stricker D (2024)"; `:3188` "Bohne D, Bohne M (2026)"; `:3191` "VDI 4650 Blatt 1 (2024-02) · DIN EN 14825 · DIN EN 410" |
| 🌬️ventilation | 15 | `tr` 10; `legend` 4; 1 unclear | `⚙️mechanical/🐍️.py:821` "Zuluft (16 °C)"; `:834` "(Speichern)"; `:801` "(0 °C)"; `:835` "(" and ")"; `🪟️natural/🐍️.py:996` "Treppenhaus oder Schacht (Kamin) …" (unclear: narration or caption) |
| 🔥️heating | 33 | `tr` 24; 9 unclear | `👥️internal-gains/🐍️.py:744` "Spezifische Lastdichte (DIN V 18599-10)"; `🧮️calculation/🐍️.py:397` "Gesamtverlustleistung (DIN V 4108-6)"; `🧱️conduction/🐍️.py:126` "Gedämmt (12 cm)", "Ungedämmt (Ziegel 1960)" |
| ❄️cooling | 7 | `Text` 2 (`" ("`, `")"` in one label); `tr` 2; 3 unclear | `👥️internal-gains/🐍️.py:491` `Text(" (")` and `Text(")")`; `🌤️summer-strategy/🐍️.py:305` "Sorptionsgestützte Kühlung (DEC)"; `:165`, `:487`, `:493` narration-style tuples (unclear) |
| 📊️demand | 30 | `tr` 22; 8 unclear | `🏭️primary-energy/🐍️.py:426` "GEG seit 29.07.2026: Gebäudemodernisierungsgesetz (GModG)"; `:627` "GEG (now GModG) § 22 (1), …"; `🏷️certificate/🐍️.py:330` "… certificate (§ 80)." (unclear) |
| 🏗️authoring (visuals) | 2 | f-string `Text` labels | `🎨️visuals/🐍️.py:334` and `:338` `f"≈ 1× {name} ({unit_text(…)})"` |
| **Total** | **102** | | Of which about 14 are `Text`/`tr`/`legend` literals with a confirmed on-screen call; the rest are unclear narration/caption tuples and need a per-scene check |

Excluded (math or LaTeX, 194 strings): e.g. `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1403` `" - 1) \approx "` and `🧲️fundamentals/🐍️.py:1526` `"\right)\,"`. Also 9 narration/speech and 13 test strings.

## 3. Frames inspected

- `🗑️generated/ui-out/synthetic-posters/Heating_02_Conduction.png`, `Cooling_02_InternalGains.png`, `Ventilation_05_MechanischeLueftung.png`: solid background, no text. Not usable.
- `🗑️generated/vent-a-out/direct/frames/need1_12.png`: header citation "Pettenkofer 1858 · Bundesgesundheitsbl. 51 [2008] 1358 · DIN 1946-6". Shows `[2008]`. Source string not checked, so this is not confirmed as a bug.
- `🗑️generated/heat-b-out/direct/Beat5_Verschattung/f_005.png`: "DIN V 4108-6, Anh. B.4", no parentheses. Not informative.
- `🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png` (2560x2160 contact sheet): **bracket rendering confirmed.** The references block shows "Willems WM, Schild K, Stricker D [2024]" and "Bohne D, Bohne M [2026]" with straight-stroke brackets. Source at `🧲️fundamentals/🐍️.py:3185` and `:3188` has `(`/`)`. Zoomed crops: `audit2/crop-glyphs.png`, `audit2/crop-refs-willems.png`, `audit2/crop-refs.png`. The "VDI 4650 Blatt 1 (2024-02)" line appears with parentheses at thumbnail scale; not zoomed, so unverified.

## 4. Next steps

1. Re-render `🧲️physics` Beat8 (or the fundamentals `Beat8_Zusammenfassung` scene) and zoom on the citation line. Check whether the `[`/`(` difference persists and which face draws it.
2. Add `(`, `)`, `[`, `]` to `required_symbols` and `rendered_glyphs` in `🔤️fonts/🧪️tests/🔣️.cases.json`, so the outline test covers them.
3. Check which face the citation label uses (`Text` with `font=` override, or a `tr` path that picks a different family).

Scripts and intermediates: `audit2/paren_scan.py`, `paren_filter.py`, `paren_count.py`, `paren_show.py`, `cmap_check.py`, `paren_shape.py`, `crop_*.py`. Crops: `audit2/crop-glyphs.png`, `crop-refs-willems.png`, `crop-refs.png`.
