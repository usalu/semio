# Fix Heat Watt Caption Formula Overlap

## Change
`Beat4_WattVergleich` U-value caption shortened so `caption_bar` stays **one line** in DE and EN, clearing the formula panel.

- DE: `Jede Schicht addiert $\\frac{d}{\\lambda}$ — U-Werte $1{,}43$, $0{,}24$ und $0{,}11\\,\\mathrm{W/(m^{2}K)}$.`
- EN: `Each layer adds $\\frac{d}{\\lambda}$ — U-values $1.43$, $0.24$ and $0.11\\,\\mathrm{W/(m^{2}K)}$.`

## Tests
`🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🎪️tests/` — caption contracts + Manim/fonts line-count check.
