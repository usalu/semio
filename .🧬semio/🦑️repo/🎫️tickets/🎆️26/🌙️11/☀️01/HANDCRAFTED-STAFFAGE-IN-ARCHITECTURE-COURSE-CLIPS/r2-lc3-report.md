# r2-lc3: summer-strategy localisation and English overflow sweep

Scripts: `T/🧰️lc3/` (`apply_ss.py` exact-string wrapper, `count.py`, `frame.py` busiest-frame extractor with PyAV, `render_en.sh`, `render_all.sh`). Generated frames and logs deleted.

## 1. summer-strategy (`E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`)
- Local `tr` (house corner, line 175, used at 182) renamed to `eave_right`; the module `tr` is no longer shadowed.
- All remaining German on-screen labels now `tr("<en>", "<de>")` with byte-identical German: three headings, "Kühllast" meter, the three step rows, four back-link chips, tags (Sorptionsrad, Plattentauscher, Verdunstungskühlung, Außenluft, Zuluft, Abluft 26 °C via `unit_text`, Fortluft), the `feucht` / `trocken` readouts (inside `\text{}`), verdict, the whole decision chart (Sommertag, question, chips, ja/nein, Komfort). `unit_text` is exported from the module.
- Numbers are integers, so no `label_number` was needed. `de_num` is gone.

## 2. English sweep (ql, 15 fps, one scene at a time, busiest frame read)
18 scenes: summer-strategy Beat2 and Beat3, transmission-humidity Beat3, internal-gains Beat2, solar-radiation Beat1, heat-flow Beat3, volume Beat3, handoff Beat2, need Beat3, convection Beat1, calculation Scene4, heating fundamentals Beat6, primary-energy Beat1, certificate Beat1, practice Beat1, system-losses Beat4, regulations Beat1, demand-power Beat2.

Fixed:
- `🌬️ventilation/❓️need` line 683: lc2 had wrapped the narration key as `tr("mechanical", "mechanisch")`, so en raised `KeyError: 'mechanical'` in `Beat3_Fahrplan` (en render crashed). Key restored to `"mechanisch"`.
- `🤝️handoff`: "Auslegung:" left German in en (now "Design:"); en text overflowed the right edge and the left bar ("with heat recovery:", "Ventilation term of the heating load") now "with HRV:" and "Ventilation term". German untouched.
- `🌬️convection`: three back-link chips "Ventilation · Ch. n" touched each other; en now "Vent. · Ch. n".
- `☀️solar-radiation`: cube face and curve label `O` shown German in en; now `tr("E", "O")`; curve `Horiz` now `tr("Horizontal", "Horiz")`.
- `🏷️certificate`: `\text{Monate}` readout was German in en; now `tr('months', 'Monate')`.
- Test harness: cooling clip test `solar()` executes module-level label code, so it now patches `SEMIO_TEACHING_LANGUAGE=de` (`❄️cooling/🎬️clip/🧪️tests/🐍️.py`).

Re-rendered after fixes and read: need Beat3, handoff Beat2, convection Beat1: clean.

Checked without change: summer-strategy Beat2 (wheel and plate-exchanger tags sit close, not overlapping), Beat3, the other 11 scenes: no overflow or overlap. Left as authored: math subscripts (`A_{Gewinn}`, `θ_e`, `c_{Luft}`), `e_p` plain title, demand-power Beat2 info card drawn over the plotted curve (same layout in de, pre-existing).

## Tests (`T/m9_tests.sh`, language de)
- `staffage_only`, `labels`, cooling clip, volume, internal-gains: 39 tests, 1 error (the solar test above); after the test fix the cooling module is 4 OK. Earlier run before edits: 34 OK.

## Open
- Not rendered: scenes that are not the most label-dense in each clip; subtitles stay German by design.
