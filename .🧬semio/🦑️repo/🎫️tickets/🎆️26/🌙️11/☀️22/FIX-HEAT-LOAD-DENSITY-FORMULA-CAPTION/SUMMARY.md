# Fix Heat Load Density Formula Caption Overlap

## Change
`Beat5_SummeUndDichte` (Spezifische Lastdichte): formula panel no longer overlaps the subtitle.

1. Raised module `FORMULA_EDGE_BUFF` from `1.2` → `1.85` so `math_panel` clears a caption_bar.
2. Shortened the `din` caption to **one line** in DE and EN:
   - DE: `Tabellarisch in DIN/TS 18599-10 — über die Heizperiode wird daraus $Q_{\mathrm{i}}$.`
   - EN: `Tabulated in DIN/TS 18599-10 — over the heating season it becomes $Q_{\mathrm{i}}$.`

## Tests
`🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🎪️tests/` — `formulaEdgeBuff` + `beat5DinCaption` (Manim/fonts line-count check).

## Verified
Preview Beat5_SummeUndDichte re-rendered; formula panel clears the one-line DIN caption (~43px at 480p). Chapter: videos/preview/de/speaker/3/v/Heating_04_InternalGains.mp4.
