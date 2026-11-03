# r3-lesson: teaching and physics audit (W04, haiku-lesson-audit, read-only; saved by coordinator)

Paths are relative to A\⚡️energy unless stated. No renders were made, no tests were run and no files were changed.

## Physics that passes

**Recomputed lesson numbers.** All of the following match:
- U-values from ISO 6946 layers: 1.43, 0.24 and 0.11 W/(m² K) (heating conduction, lines 84–109).
- Φ_T ≈ 2300 W, Φ_V ≈ 1500 W, Q_loss ≈ 16 000 kWh, Q_E ≈ 14 000 kWh.
- Latent ventilation load: 1.2 × 0.0611 × 2.45 MJ/kg × 4.5 g/kg = 808 W.
- q_V,R = 0.126 m³/s, and a duct area of 0.0505 m² at 2.5 m/s.
- Stack-effect density ratio ≈ 3.
- g-value split 0.50 + 0.10.
- Thermal mass: 240 kg × 1000 × 2 K = 0.13 kWh.
- Latent-to-sensible ratio 2257/419.
- Seated adult ≈ 120 W.

**Thermal model, `🧍️staffage/🔨️modules/🌡️thermal`.**
- Colours come from the model. Each quantity has a unit, centre, range and legend (`🎨️depiction/🌡️thermal-view`, `📏️quantities`).
- Surface temperature, skin temperature, emitted flux εσT⁴, net radiation, sensible, latent and moisture are separate quantities.
- The emitted-flux range of 230–700 W/m² matches σT⁴ at −20 °C and 60 °C.
- Breathing uses a half-sine flow with inhale, pause and exhale fractions, nose/mouth routing and a `directionDeg` emitter (`🫁️respiration:3-7, 155-170`, `🔥️sources:95-105`).
- Parcels are born at the emitter's world position and are then advected in world space, never re-anchored (`💨️transport:3-6`, `🔥️sources:129-171`).
- Transpiration is modelled as vapour parcels at ambient temperature driven by leaf latent flux, not as a hot plume (`🔥️sources:6,112-115`, `🧱️balance:93-97`).
- Nothing is drawn as fog. Vapour is a teal disc with a hatch cue, and condensation is a cyan-outlined disc.
- The physics correlations are cited:
  - Buck saturation
  - Lewis relation
  - Churchill–Chu
  - McAdams
  - Churchill–Bernstein
  - CODATA σ

**Structural machinery.**
- `🧍️staffage/🔨️modules/🏗️structure/🟦️.ts` solves N/V/M with stated sign conventions, and the solver has tests.
- `🎨️depiction/🏗️structural-view` draws forces and moments.
- The course uses none of this: the cast manifest has 0 forces or moments views.

## Findings

| id | priority | where | finding | fix | owner (r3) |
|---|---|---|---|---|---|
| F1 | M | `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:292, 337-342, 424-427` | OI-112 is resolved in the code: the collector feeds the heating coil. However, "die Sonne treibt den Kreislauf" overstates the solar role for a 70 °C regeneration air temperature. | Reword the en/de narration to "Solarwärme heizt die Regenerationsluft vor; ein Nachheizregister hält 70 °C", and show the regeneration temperature on the solar branch. | W14 |
| F2 | H | same file `:294-295, 444-446, 463-464`; fixture `V/🧪️tests/🔣️integration.json:445-449` | The clip claims that stress "causes mutations", and the visual says "Mutation". In fact, alternating stress selects for resistant strains that are already present. | Say "Temperatur- und Feuchtewechsel selektieren resistente Keime; nasse Sorptionsflächen brauchen Hygiene (VDI 6022)", label the visual "Selektion", and update the fixture. | W14 |
| F3 | M | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:147-161` | The solar peak is calibrated so that η = 0.90, and η is then shown as if it were a result. | Either derive the peak from a stated case, or label it "calibrated to η = 0.90" on screen. | W14 |
| F4 | M | `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:1863-1864` | `cop_of(t) = 3.0 + 0.08 t` is an unsourced linear fit. | Use COP = η_G·T_h/(T_h−T_c) with a stated η_G (EN 14825 context), or label the fit "illustrative". | W15 |
| F5 | L | `🧍️staffage/🔨️modules/🌡️thermal/…/💨️transport-view/🟦️.ts:6 vs 115` | The doc says the condensation veil is white, but the code fills it with ink at 0.12 opacity, i.e. dark. | Make the code and the doc agree. | W06 |
| F6 | H | `🫀️physiology/🟦️.ts:25` | `DILATION = 120`, but the Gagge two-node model uses 200 in `m_bl = (6.3 + 200·WARM_c)/(1+0.5·COLD_s)`. | Confirm the value against the pythermalcomfort source, fix it, and rerun the physiology tests. | W06 |
| F7 | M | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:301-324` | Droplets are drawn in exhaled breath at 22 °C. The breath paths are hand-drawn and have no inhale/exhale timing. | Use the canonical breathing emitter (vapour, no droplets) on the cast. | W14 |
| F8 | L | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:707-708` | The clip calls 0.5 h⁻¹ the "minimum" air change of EN 12831. It is a design rate; the hygiene minimum comes from DIN 1946-6 / EN 16798-1. | Reword after checking the clause. | W15 |
| F9 | L | convection:104, calculation:79-92, heat-flow:99-119, handoff:77-87, transmission-humidity:105 | The running-example constants are duplicated across clips and unit factors are hidden. | Put them in one shared numbers module with explicit units. | W14 and W15, with a shared module |
| F10 | L | `📖️concepts/📝️.md:573-575` | The shares ~45/35/20 % have no source. | Cite a source or remove the shares. | W15 |
| F11 | L | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:381-382` | The factors 0.49, 0.15 and 0.60 are never defined on screen. | Name each factor with its symbol. | W14 |
| F12 | L | calculation:112 | The clip uses a mean θ_e of 2 °C, while GT 3500/212 d implies about 4 °C. | Align the values or add a note. | W14 |
| F13 | L | `🏗️bim/🎬️clip/🧱️model/🧮️analysis/🐍️.py`; model JSON | The solarium is given an imposed load of 2.0 kN/m². A roof used by people needs its EN 1991-1-1 category stated. | State the category and check the value. | W07 |

## Structural scope

No clip in fit, parametric, nakagin, optimization or course teaches forces, supports or bending. The only structural content is the BIM Villa Savoye `🔌️exchange` Beat5 structural check (EN 1991 loads, tributary areas, fixed bases). It should use the existing library solver and `structural-view`.

## Not audited in depth

- Heating internal gains and solar radiation
- Cooling heating-comparison and solar-radiation
- Ventilation mechanical and need
- Demand: power, regulations, system losses, primary energy, certificate, practice
- Per-garment clo and per-pose met values
- The sorption-wheel asset
