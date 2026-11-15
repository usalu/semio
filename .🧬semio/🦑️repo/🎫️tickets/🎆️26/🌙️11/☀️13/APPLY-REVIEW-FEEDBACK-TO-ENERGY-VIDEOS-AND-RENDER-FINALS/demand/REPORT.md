# demand report (energy balance video, seven chapters), recorded by the coordinator

Chapters 1–3 (recap, regulations, demand-power) by `demand`; system-losses, primary-energy,
certificate, practice by sub-agents `demand-a` / `demand-b` (their work was not re-checked by
`demand`). Preview renders (German) of chapters 1–3 viewed as frame sheets; recap not re-rendered
after the last spacing fix. Narration guard green.

- Units/symbols: no bare `[unit]`; units on every number; fraction bars; narration `$…$`.
- Balance per DIN V 4108-6 §3.2: Q_h = Q_l − η·Q_g, Q_l = Q_T + Q_V, Q_g = Q_S + Q_i; solar
  F_S/F_C/F_F/F_W. Cooling Q_c = Q_g − η_c·Q_l. Heating load Q̇_H ("Φ_HL in der Norm" said once).
  Q̇_V ventilation only.
- Degree days: Σ(θ_i − θ_e) over heating days with the θ_i = 20 °C line from the first mention of
  G_t,20/15; F_Gt = 0,024 kKh/Kd · 3500 Kd/a = 84 kKh/a.
- Nutzenergie / Endenergie / Primärenergie named wherever energies are compared (Q_h → Q_E →
  Kosten panel; heat-pump electricity vs delivered heat corrected). Anlagenaufwandszahl → e_E.
- ρ·c_p carries its condition (ρ ≈ 1,2 kg/m³ at 20 °C).
- Quiz and README updated.

## Numbers recomputed
Old Q_Senke = 840 kWh contradicted Φ·t ≈ 3,1 MWh → month sets from the running example:
transition month η = 0,80; January η = 0,97; July η_c = 0,61. Design load 6,08 kW (H_T = 115 W/K,
H_V = 75 W/K, Δθ = 32 K; before 10 kW with 225 + 87,5 W/K). Annual demand ≈ 11 700 kWh/a.
G_t,20/15 = 3500 Kd/a. Chain numbers 140/31/154/56 unchanged. Running example unchanged.

## References
Verified locators: DIN V 4108-6:2003-06 (3.2, 5.5, 6.1–6.5, Anh. A, D.2, D.5),
DIN V 18599-2:2018-09 5.5.3, DIN EN 12831-1:2017-09 Abschn. 6, VDI 2078:2015-06,
VDI 4645:2023-04, GEG § 15/§ 20 (gesetze-im-internet.de).
Agent claim to re-verify: GEG renamed Gebäudemodernisierungsgesetz (GModG) on 2026-07-29 (chips
say so); DIN 4108-2:2026-05 final vs pre-published unconfirmed; quiz 12,5 kWh/(m²·a) cited at
Anhang D only.

## Open points
1. Roadmap `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` lines 46, 47, 66 use `e_p` → `e_E`; G_t → G_{t,20/15}.
2. Chapter poster nearly empty (0,0 % content) → manifest `poster` moments (UI agent, done since).
3. Register new tests (`🔁️recap…🧮️balance`, `🏭️primary-energy…🐍️chain`, `🧪️tests…🔗️chain`) in `📜️script.ts`.
4. Speech for changed clauses not re-synthesised.
5. Shared font draws `(` as `[`; demand-a/b chips contain "(seit 7/2026 GModG)".
6. `🛡️guard` fixture misses the new roadmap clip (one failing case).

Files (`🎓️teaching/🏛️architecture/⚡️energy/📊️demand/`): all seven chapter `🐍️.py`, `README.md`,
`❓️quiz/🔣️.json`, `🔁️recap/🧪️tests/🧮️balance/`, chain tests under `🏭️primary-energy/🧪️tests` and
`🧪️tests/🔗️chain`. Frames: `🗑️generated/demand-out/frames`, `demand-a-out`, `demand-b-out`.
