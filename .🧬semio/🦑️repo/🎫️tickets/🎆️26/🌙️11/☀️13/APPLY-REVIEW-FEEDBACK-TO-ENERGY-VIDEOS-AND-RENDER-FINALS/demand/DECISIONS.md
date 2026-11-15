# Shared decisions of the `demand` content team (video `energy_balance`)

Binding for everyone who edits `📊️demand/**`. They refine `CONVENTIONS.md` for this video.

## Chapter ownership
- `demand` (lead): `🔁️recap`, `⚖️regulations`, `⚡️demand-power`, `README.md`, `❓️quiz/🔣️.json`, final REPORT.
- `demand-a`: `♨️system-losses`, `🏭️primary-energy` (+ their `🔊️speech`, `🏗️composition`, `🔀️transitions`, `🧪️tests`).
- `demand-b`: `🏷️certificate`, `🧪️practice` (same sub-folders).
Never edit a neighbour's chapter; message the lead via your report if a shared value must change.

## Symbols (DIN V 4108-6 / DIN V 4701-10 / DIN V 18599)
- Heating balance: `Q_h = Q_l − η · Q_g`, `Q_l = Q_T + Q_V`, `Q_g = Q_S + Q_i` (TeX `Q_{h}`, `Q_{l}`, `Q_{g}`, `Q_{S}`, `Q_{i}`, `\eta`). The old `Q_Senke`/`Q_Quelle`, `Q_I`, `η_h` are gone.
- Cooling balance: `Q_c = Q_g − η_c · Q_l` (TeX `Q_{c}`, `\eta_{c}`); the old `Q_K`, `η_k` are gone.
- Loads (power, W/kW): `\dot{Q}` generic power at a moment, `\dot{Q}_{H}` heating load (DIN EN 12831-1 calls it `Φ_HL`, said once, never `Φ_HL` as our own symbol), `\dot{Q}_{V}` only ventilation. `Φ` only as generic heat flow in the physical definitions.
- Energy chain (the lesson of this video): `Q_h` Nutzenergie (heating demand at the room) → `Q_E` Endenergie (what the meter / delivery bill shows) → `Q_P` Primärenergie (incl. upstream chain). `e_p` Anlagenaufwandszahl (`e_{p}`), `f_P` primary energy factor (`f_{P}`) — verify the exact symbols of DIN V 4701-10 / DIN V 18599-1 by WebSearch before you rely on this; document the evidence.
- Heat-pump seasonal figure: `Jahresarbeitszahl (JAZ)` (VDI 4650 Blatt 1), `SCOP` (DIN EN 14825). One term everywhere.
- Nutzenergie / Endenergie / Primärenergie must be separated verbally AND visually wherever energy amounts are compared. Heat-pump electricity is Endenergie, the heat it delivers is Nutzenergie (not the same number).
- Units per CONVENTIONS §1: no bare `[unit]`; `Q in kWh`, `Q = 120 kWh`; every number in a substitution carries its unit; compound units with `\frac{\mathrm{kWh}}{\mathrm{m^2\,a}}`. Dimensionless factors are said to be dimensionless (no unit) – then ALSO the product line shows the units on the result only if every factor shows its own.
- Narration `(key, en, de)`: every symbol / value-with-unit as inline TeX `$…$` (raw strings), never "Delta Theta", "eta h", "Rho".

## References (CONVENTIONS §6)
Every `_din_ref(...)` and on-screen citation needs edition/year AND a locator (§, Anlage, Teil, Abschnitt, Tabelle) verified with WebSearch for the CURRENT edition (today is 2026-11-13). Examples of what is wanted: `GEG 2024, Anlage 4 (Primärenergiefaktoren)`, `DIN V 18599-1:2018-09, Tab. …` — verify, do not guess. Record every reference with evidence URL in your report.

## Shared example numbers (source of truth)
- Building constants come from `🏗️authoring/📏️running-example` (import `_example` the way `🔥️heating/…/🧮️calculation` does). The lead recomputes the chapter-1 month numbers from it; the energy-chain numbers (`Q_h = 100 kWh/(m²a)`, `e_p`, `f_P`, `Q_E`, `Q_P`) keep living in `♨️system-losses` / `🏭️primary-energy` and are mirrored in `🏷️certificate` / `🧪️practice`. If a chain number changes, `demand-a` owns the change and `demand-b` mirrors it (re-read the sources before editing).

## Render lock (one render at a time for the whole team)
Before a render: `mkdir "<TK>/demand/render.lock"` (atomic; if it exists wait 60 s and retry). After the render and frame extraction: `rmdir` it. Output roots: `<TK>/🗑️generated/<your-name>-out`.
