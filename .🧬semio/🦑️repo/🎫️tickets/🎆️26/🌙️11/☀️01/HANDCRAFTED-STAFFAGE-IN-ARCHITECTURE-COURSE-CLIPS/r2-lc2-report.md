# r2-lc2: bilingual on-screen labels, energy clips (follow-up of r2-lc)

## Done
- About 800 on-screen German literals in the energy clips now read `tr("<English>", "<German>")`; the German side is byte-identical. Clips: cooling (solar-radiation, heating-comparison, internal-gains, transmission-humidity, system-sizing), ventilation (mechanical, need, heat-flow, volume, handoff, natural), heating (solar-gains, convection, internal-gains, calculation, conduction, fundamentals), physics fundamentals, demand (system-losses, regulations, demand-power, primary-energy, certificate, recap, practice).
- `de_num(` became `label_number(` everywhere (about 450 calls; `de_num` is math-only, so en gets decimal point and de stays identical). Physics `_de` helper was removed in favour of `label_number`.
- Hard-coded German decimals (`0{,}8`, `1{{,}}8`, `0{{,}}133`, ...) became `tr(..."0.8"..., ..."0{,}8"...)` or `label_number(x, n)`.
- Plain-text quantities that I rewrote use U+202F before `%`, `cm`, `kWh`, `°C` on the English side only (German untouched, so it keeps its plain space).
- Tooling: `T/🧰️lc2/` (`extract.py` finder, `apply.py` AST-exact wrapper driven by `maps/*.py`, `dec.py`, `mech.py`, `revert.py`, `unwrap.py`, `scope.py`, `narr.py`, `keys.py`, `chain.sh`, `render.sh`).

## Safety checks I added (they found real bugs)
- `tr` shadowed by local variables (`tr = ValueTracker`, `bl, br, tr = ...`): renamed in cooling internal-gains, natural, conduction, primary-energy. `scope.py` now reports none (need is a false positive from a comprehension variable).
- Module-level `tr()` would raise at import (language is read lazily): none left. Module data (`AIR_CHANGE_USES`, `CARD_SPECS`, `WALLS`, physics decade words) keeps German and is translated at the display site via small `_*_EN` dicts.
- Component names used as dict keys (`Dach`, `Außenwand`, `Fenster`, `Boden`, ...) are not translated at the key, only where displayed (conduction table, recap bar and header).
- Narration keys and `subtitle_text` / `swap_caption` / `hold_for` keys are untouched (`narr.py`, `keys.py`). `ruff F405/F821` shows no unresolved `tr` / `label_number`.

## Decisions
- Symbol subscripts stay as authored (`\mathrm{Luft}`, `WRG`, `Verlust`, `Gewinn`, `Pers`, `Geräte`, `Licht`, `Raum`, `ges`, `Quelle`, `Senke` ...). The English narration already reads them in German ("Q-dot Geräte"), and half-translated formulas are worse. I first translated a few, then reverted them with `revert.py`. `\text{...}` words and `\mathrm{...}` words that are displayed as words (Day, Summer, Reserve, RH for r.F., SPF for JAZ) are translated.
- Subtitles stay German (VO scope). `TITLE_*_DE` constants untouched (metadata).
- Terms: Abluft Extract air, Fortluft Exhaust air, Außenluft Outdoor air, Zuluft Supply air, WRG HRV / heat recovery, Raffstore Venetian blind.

## Tests and renders (after the bun kill, everything was re-run)
- Unittests (`SEMIO_TEACHING_LANGUAGE=de`): ventilation architecture, volume, cooling clip, cooling internal-gains: 15 OK. `staffage_only` + `🧪️tests/🐍️labels.py`: 30 OK.
- Physics fundamentals tests: 2 failures, `test_kettle_boil_time...` (182 vs 198 s) and `test_kettle_boils_within_the_kettle_beat...` (198 vs 184). These compare kettle timing with narration data and are unrelated to labels; not touched (someone else's timing/narration change).
- en renders (ql, 15 fps, one at a time), frames read: cooling heating-comparison Beat3 and system-sizing Beat2, solar-gains Beat1, conduction Beat5 (envelope table), recap Beat4, physics Beat3, natural Beat6, mechanical Beat3. Two overflows found and fixed: mechanical "natural ventilation without heat recovery" (now "natural, no heat recovery") and the verdict line (now "Passive house: heat-recovery ventilation replaces windows"). Natural Beat6: the "ungenutzt" arrow label sits at the left screen edge (also in de, pre-existing layout), now "unused".

## Open
- `❄️cooling/🌤️summer-strategy/🐍️.py` not touched (agent ss). It still has: `de_num` calls, German labels (Sorptionsrad, Plattentauscher, Verdunstungskühlung, Außenluft, Zuluft, Abluft 26 °C, Fortluft, feucht/trocken readouts, DEC heading, question text), and probably `tr` shadowing: line 175 `bl, br, tl, tr, peak = ...` must be renamed before `tr()` is used in that function. Use `T/🧰️lc2/apply.py` with a map once ss is done.
- Not rendered in en (tests/ruff only): transmission-humidity, internal-gains (cooling), heat-flow, volume, handoff, need, convection, calculation, fundamentals (heating), primary-energy, certificate, practice, system-losses, regulations, demand-power. Longer English strings there may need shortening.
- Untranslated on purpose or missed: unit-only badges like `"18 °C"` (plain space), reference titles in physics (German book titles), narration-style English text with German symbol names, dead data names (wall layers, `STAGES`, `_ENVELOPE` in calculation). A last `left.py`-style scan should be rerun after summer-strategy lands.
- Physics kettle test failures above.
