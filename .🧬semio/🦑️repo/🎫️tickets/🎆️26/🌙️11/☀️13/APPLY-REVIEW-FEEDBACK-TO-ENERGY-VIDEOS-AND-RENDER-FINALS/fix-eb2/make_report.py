from pathlib import Path

TEXT = """# Energy balance final: second audit pass

All edits are under `📊️demand/🎬️clip/`.

## Findings and fixes
1. **Q_h 11500 vs 11700 (real).** The 11500 at 21:52 is not a demand-chapter label: chapter 4 opens with the heating
   clip `🔥️heating/🎬️clip/🧮️calculation` (`AnlagenVerluste`), where `_Q_H = round(Q_loss) - 0.90 * (Q_sol + Q_int)
   = 16000 - 4500 = 11500`. The 11700 came from the synthetic duration-curve year in `⚡️demand-power`
   (`round(sum(load) * PEAK_KW, -2)` with a hand-picked `GAIN_LIMIT = 12`). Fix: `Q_H_KWH_A` in `⚡️demand-power`
   is derived from the running example the same way (`ETA_UTILISATION = 0.90`), and `GAIN_LIMIT` is now solved
   by bisection (`_solve_gain_limit`) so the synthetic year integrates to exactly `Q_H_KWH_A`. Every label in that
   chapter (`Q_h ≈ ...`, `t_VL`) follows. Test `test_annual_heating_demand_is_one_value_from_the_running_example`
   checks the 11500 against a plain numpy sum. Narration of the demand chapters quotes no annual figure.
2. **Primary-energy overlap (31:25-31:30).** The settled state overlapped: the equation box (y -1.36..-2.4) cut
   through the axis labels Nutzenergie/Endenergie/Primaerenergie, and the final 0.6 s `FadeOut(everything)` re-added
   mobjects that were already gone (legend words, unit legend, the three transitioned value labels) because
   Manim re-adds anything that is animated and not in the scene, so they flashed over the e_P note. Fix in
   `🏭️primary-energy`: the category labels fade out together with the legend before the formula morph, and
   `everything` no longer lists faded or morphed-away mobjects. Preview frames 252-272 s: no label under the
   box, flight clean, final fade clean.
3. **Law name.** Narration/captions now say Gebaeudemodernisierungsgesetz / Building Modernisation Act in all
   chapters (regulations: three clauses, primary energy: three clauses). The former name is mentioned twice:
   once in the regulations intro clause, once in the primary-energy DIN note ("vormals GEG"), plus the law card.
4. **"1 Monate".** `_count_label` in `🏷️certificate`: singular for exactly one (`1 Monat` / `1 month`), plural
   otherwise; only counted unit in the demand chapters that follows a variable count (others are constants:
   `7 Tage`, `12 Monate`).
5. **Hard-coded numbers.** `🏭️primary-energy`: `E_P_GAS = Q_PRIM_GAS / Q_NUTZ`, `E_P_WP` drive the e_P note and
   its narration (`_de` for the decimal comma); the neighbouring narration values (100, 140, 31, 74, 26, 120, 0,55,
   66, 56) now come from `Q_NUTZ`, `Q_END_GAS`, `Q_END_WP`, `Q_AMBIENT_WP`, `Q_POWER_SHARE`, `Q_REF`, `REF_SHARE`,
   `Q_LIMIT`, `Q_PRIM_WP`. `🧪️practice`: the narration of the series review reads `CHAIN_NUTZ`, `CHAIN_END`,
   `CHAIN_PRIM`, `CHAIN_END_WP`, `CHAIN_PRIM_WP`, `CHAIN_E_E` and the f_P constants via `_num`.

## Tests
`🧪️tests/🔗️chain` gained three tests (annual demand 11500 from the fixture and a numpy oracle, e_P numbers and
narration, singular month label) and the fixture key `annual_heating_demand`. Demand modules run green:
guard, chain (8), primary-energy chain and scale, recap balance, regulations sun (30 tests in the first batch,
8 in chain afterwards).

## Verification renders (preview, de, private root `🗑️generated/fix-eb2/out`)
Chapter 05: frames 252-272 s inspected (box clean, flight clean, e_P caption 1,54 / 0,56 on two lines). Chapter 06 rendered, contact sheet looked at (counter chapter intact; the singular frame itself covered by the unit test, not eyeballed). Chapter 03 rendered (exit 1 only from the shared presentation handoff step, 'UI sources changed during handoff capture', caused by other agents editing; the chapter itself completed): contact sheet shows Q_h = 11500 kWh/a in every beat and t_VL = 11500/6,08 = 1891 h. German captions of the four touched chapters: none wraps beyond two lines (caption_bar raises otherwise; `captions.py`).

## Leftovers
- The heating clip `🔥️heating/🎬️clip/🧮️calculation` (outside this scope) still hard-codes `11500` in its narration
  and derives `_Q_H` from its own solved utilisation; ideally the running example exposes `Q_H_KWH_A` and both
  read it. The demand constant duplicates `0.90` for that reason.
- The speech for changed clauses is not re-synthesised.
"""

Path(__file__).with_name("REPORT.md").write_text(TEXT, encoding="utf-8")
