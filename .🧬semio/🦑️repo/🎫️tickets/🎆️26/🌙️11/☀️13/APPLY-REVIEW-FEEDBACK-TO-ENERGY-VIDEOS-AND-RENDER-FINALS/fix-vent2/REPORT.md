# fix-vent2: four layout defects in the German ventilation video

All paths under `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/`.

## Fixes
1. Stack effect (`🪟️natural/🐍️.py`, Beat4_Auftrieb): "Zuluftöffnung" and "A = 0,1 m²" now sit left of the inlet wall, above and below the supply arrow, outside the outline. The comfort ring around the inlet at the end of the beat now encloses both labels instead of cutting through them.
2. "Was Lüftung leisten muss" (`❓️need/🐍️.py`): the Pettenkofer tag is two lines ("Grenzwert nach" / "Pettenkofer (1858)"; en "Limit after" / "Pettenkofer (1858)") with the 1000 ppm line below, so it no longer reaches the Feuchte column.
3. "Wärmekapazität der Luft" (`🌡️heat-flow/🐍️.py`): left column rows spaced by the fraction height (ρ 20 °C at 2.55, ρ 5 °C at 1.75, m at 1.0, c_p at 0.10, θ at -0.55). The 5 °C row now reads "ρ ≈ 1,27 kg/m³ (5 °C)" at body size directly under the 20 °C row.
4. Two different cases, now labelled: a case tag above the Δp / v / V̇ readouts reads "Sommernacht · h = 2,5 m" (0,8 Pa, 1,15 m/s, 250 m³/h), "Sommernacht · h = 4,4 m" (1,4 Pa, 1,53 m/s, 330 m³/h) and "Winter · h = 4,4 m" (3,8 Pa, 2,52 m/s, 540 m³/h). Numbers come from one `_stack_chain`; the 0,6 · 0,1 · 1,15 = 0,069 m³/s chain is the 2,5 m summer case.

## Verification
Preview renders (speaker, de) of Ventilation_01_WarumLueften, _03_WaermestromLuft and _04_NatuerlicheLueftung into `🗑️generated/fix-vent2/out*`; held frames inspected, all four defects gone.
Tests (preview, en, SEMIO_TEST_MANIM=1): ventilation architecture 8, numbers 4, transitions 7, volume 2: OK.

## Left
Nothing in scope. Generated renders remain in `🗑️generated/fix-vent2`.
