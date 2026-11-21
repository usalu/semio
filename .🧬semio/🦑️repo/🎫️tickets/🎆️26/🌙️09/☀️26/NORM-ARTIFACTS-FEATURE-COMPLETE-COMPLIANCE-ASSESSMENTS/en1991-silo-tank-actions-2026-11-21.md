# EN 1991 silo and tank actions

## Implemented cases

| Case | EN 1991-4:2006 source | Implemented scope |
| --- | --- | --- |
| E4.06 | §5.6.2 and C.10.2 | Additional normal wall pressure from relative cooling of a circular silo. The user supplies the tested unloading modulus or the unit weight and base vertical filling stress for the indirect estimate. The calculation applies the corresponding temperature factor and uses 0.30 as the permitted Poisson ratio when omitted. |
| E4.09 | Annex B B.2.9 and EN 1991-1-4 pressure conventions | Internal wind coefficients for open circular tanks, open catch basins and tanks with small vents. The calculator combines signed internal pressure with external pressure from a supplied local coefficient. Separate reference velocity pressures allow different internal and external heights. |
| E4.09.1 | Annex B B.2.2 | Records the specified signed minimum and maximum gauge pressures for an operating or test situation. The derived span describes those endpoints. |

The silo calculation was checked against the English edition and the publication hosted by IMANOR. The tank internal wind coefficients use the negative signs specified in B.2.9(2). The local external coefficient is supplied by the project. [EN 1991-4:2006](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1991.4.2006.pdf) · [IMANOR edition](https://www.imanor.gov.ma/wp-content/uploads/2021/06/10.0.051-EN-1991-4.pdf)

## Verification and gaps

The production Rust catalogue evaluators agreed with an independent SQLite implementation on all nine silo cooling pressure rows and all 11 tank wind and pressure rows.

Hot filling, restrained thermal expansion, connected-structure effects, independent estimation of effective properties from the material tests and separate load combinations remain open. The tank cases require the project to determine the external wind coefficient with any applicable shielding effect. Suction from impeded ventilation, silo fatigue and tank structural fatigue remain open. Structural steel tank fatigue follows its own Part 4-2 design code and EN 1993-1-6. [DIN EN 1991-4 edition](https://www.dinmedia.de/en/standard/din-en-1991-4/134861248) · [EN 1993-4-2:2007](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1993.4.2.2007.pdf)
