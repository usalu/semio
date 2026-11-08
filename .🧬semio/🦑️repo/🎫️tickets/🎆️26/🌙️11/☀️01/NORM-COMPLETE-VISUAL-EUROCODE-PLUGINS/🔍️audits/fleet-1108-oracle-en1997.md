# Oracle lane report: EN 1997 SLOPE-02, SLOPE-04, SLS-04, DEW-02, DEW-03

Oracle: `🔮️oracles/🌍️geotechnics/🐍️.py` (numpy, scipy; groundhog 0.15.0 has no Janbu, wedge, tilt or dewatering-array routine, so these are independent implementations). Row ids are literal in the docstrings. Fixture: `🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json` (11 new blessed cases, 49 total; `--self-check` passes).

| Row | Kind | Method |
|---|---|---|
| SLOPE-02 | en1997.G.slope-janbu | 4000 midpoint slices (numpy), arc span by brentq, Janbu 1973 f0, F by brentq; cross-check 400 slices with scipy fixed_point |
| SLOPE-04 | en1997.G.slope-wedge | force resolution along/normal to the plane, minimize_scalar after coarse scan; closed-form Culmann check for phi = 0 |
| SLS-04 | en1997.G.footing-tilt | numpy polyfit slope through two settlements |
| DEW-02 | en1997.G.drawdown-superposition | Dupuit/Thiem by scipy quad of Darcy law, Sichardt radius, superposed confined drawdown |
| DEW-03 | en1997.G.drawdown-settlement | oedometer strain integrated by quad |

## Comparison with the in-repo Python proxy of the Rust catalogue (Rust not run)
Agreement: Janbu dry 2.14437 vs 2.14429 (4e-5), phi-only 2.28850 vs 2.28849, groundwater at toe level 2.04796 vs 2.04788; wedge identical (theta 33.2413 vs 33.2415 deg, grid step); tilt, DEW-02, DEW-03 identical.

## Discrepancy (modelling convention, clause EN 1997-1 11.5 / Janbu 1973)
Inputs h 6, beta 30, xc 6, yc 10, r 11, gamma 18000, gamma_sat 20000, phi 30, c 5000, gwl_level 2 (above the toe ground): oracle F = 1.7930 (pore pressure gamma_w (gwl - y_base) plus the weight of ponded water on the ground); Rust/catalogue F = 1.7270 (water above ground is clipped: no ponded weight, pore pressure only below ground). Not fixture-blessed; the fixture uses gwl_level 0 where both conventions agree. Decision for en1997: Rust ignores ponded water at the toe.

## Open
Rust adapters for five new checks: see `📌️important/requests.md`, section `## fleet-1108 → en1997`.
