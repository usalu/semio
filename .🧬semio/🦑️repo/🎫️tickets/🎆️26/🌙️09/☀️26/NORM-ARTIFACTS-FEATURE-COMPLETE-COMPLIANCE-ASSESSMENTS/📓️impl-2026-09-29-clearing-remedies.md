# Clearing remedies 2026-09-29

Grok 4.7 High is not an available agent model, so these edits are in the coordinator chat. Not test-run yet.

- DIN 18599 `din18599.1.net-floor-area`: the A_N remedy is emitted for every overrun, not only past 5%.
- DIN 18599 `din18599.2.cooling-need`: limit is 0.5·Q_H,nd. A failing check reduces window g, or raises the cooling setpoint when there is no window.
- EN 1993 bending and SLS: when the rolled catalogue has no larger section, the remedy raises `wPlY`/`wElY` or `iy`.
- EN 1993 HSS: zero elastic resistance is no longer reported against a 1 N·m floor. Class above 3 with a real moment fails against class 3.
- EN 1994 class and shear on catalogue sections remedy `steel.designation`, filtered to a section that clears. Custom plates still change `twM`.
- EN 1994 bending `one_of` keeps only heavier HEB sections whose plastic moment covers M_Ed.
- EN 1994 §7.2.2 scales `qAreaPa` (or the span) by σ_lim/σ_a instead of writing the stress into the load field.
- VDI 3805 nesting depth is counted from products, records, and fields.
