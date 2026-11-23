import pathlib

text = """# fix-heat2: second heating polish pass (German video)

Files (all under `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/`):
- `🧱️conduction/🐍️.py`
- `🧲️fundamentals/🐍️.py`

## Fixes

| # | Defect | Verdict and fix |
|---|---|---|
| 1 | psi_i / psi_e rows under the red "psi_i - psi_e = U . 2d" box | Real. The two rows sat in the formula band. They now fade out (and are removed) together with the corner strips before the box appears; removed from the later plan fade-out. Held preview frames: box clean. |
| 2 | caption "l = 8 m ... 4 W/K" vs scene "l = 0,8 m ... 0,4 W/K" | Transient, not a contradiction: constants are `BALCONY_PSI = 0.5`, `BALCONY_L = 8.0` (4 W/K; corner sum 4 . 6 m . -0.08 = -1.92, total 2.1 W/K as narrated), the readout counts up while the caption is already shown. Count-up shortened from 3.0 s to 1.6 s. Scene, caption and narration (en, de) agree at 8,0 m and 4,0 W/K. |
| 3 | spectrum panel translucent, scene labels cut by its edge | Panel fully opaque; the labels it cuts (inside and outside temperature, delta theta, Raumluft, kalter Nachthimmel) fade to 0 while it is shown and return with it. Frames: nothing half-covered. |
| 4 | circle labels "3x" / "14x" vs "0,8x Toaster" / "1,4x Teelicht" (audit said internal gains) | Located in conduction watt comparison: the circles are magnifier discs (scale of the drawn item), not counts. Tag is now "3x vergroessert" / "magnified 3x" (same for 14x), taken from `ZOOM`. |
| 5a | "1 m" label touching the formula box (conduction U tile) | Real. Tile moved up 0.25. |
| 5b | "aussen 0 C" touched by the Strahlung chip (fundamentals) | Real. Route strip moved 1.0 left. |

## Verification
- Preview render (de, speaker) of Heating_02_Conduction and Heating_01_Introduction into `🗑️generated/fix-heat2/out`; held frames checked for 1, 2, 3, 4.
- Heating test module `conduction.tests`: 2 OK. Visuals integration module (preview, en): 9 OK, no pinned caption changed (narration untouched).
"""
pathlib.Path(__file__).with_name("REPORT.md").write_text(text, encoding="utf-8")
