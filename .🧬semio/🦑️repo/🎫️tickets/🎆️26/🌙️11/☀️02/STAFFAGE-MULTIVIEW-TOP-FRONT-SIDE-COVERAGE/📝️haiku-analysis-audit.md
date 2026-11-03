# 📝️ haiku-analysis-audit: Thermal, Transport and Structure (relayed by the coordinator)

This is a static read-only audit taken while other workers were editing the same files. Line numbers can drift. No tests were run.

- M = `🧍️staffage/🔨️modules`
- T = `🧍️staffage/🧪️tests`

## Thermal and transport (owner: sonnet-thermal)

| # | Severity | Defect | Evidence |
|---|---|---|---|
| T1 | High | Thermal for a looping clip runs on the wrapped clock, so each loop restarts from steady state and the temperature jumps at the loop boundary. The periodicity test cannot catch this: it compares two absolute times that wrap to the same state, so it passes by construction. | `M/🎞️animation:499,508`; `M/🌡️thermal:289-304`; `T/⏳️thermal-transient:91-97` |
| T2 | High | There are two `environmentAt` implementations that blend wind differently (along the shorter arc versus as a vector). | `🌡️thermal/🌤️environment:19,29`; `💨️transport/🌬️ambient:37-46` |
| T3 | High | Wind now lies in the x–z plane, but the docs and schema header are stale, and a crosswind is invisible in the elevation. | `ambient:3,67-81`; `〰️streamlines:107` |
| T4 | Med-high | The transport overlay is drawn in elevation coordinates for every viewpoint, and z is dropped. | `🎨️depiction/💨️transport-view:53-123`; `🎨️depiction:176-177` |
| T5 | Med | Transient thermal inside transport ignores the environment timeline. | `💨️transport:80`; `parcels:157` |
| T6 | Med | When the range is fixed, the neutral grey is clamped into it, but the legend text still prints the unclamped air temperature as the centre. | `🖍️palette:79`; `thermal-view:127-130` |
| T7 | Med | The painted value of merged regions is weighted by area only, but the module's rule is area × exposed fraction. | `thermal-view:71`; `🧩️shapes:57-68`; `🌡️thermal:474-488` |
| T8 | Med-low | Totals mix area bases: latent heat on skin area, net radiation on exposed area. | `🌡️thermal:385,387,438` |
| T9 | Low | The transient step size depends on τ at the query time, so the integration grid can jump by a factor of 2 between frames. | `⏳️transient:21,24` |
| T10 | Low | The chain start uses the querying time's steady temperatures for placed regions. | `🌡️thermal:319-325` |
| T11 | Low | Plume and transpiration source strength is sampled at 0.5 s steps. | `🔥️sources:48-49,70` |
| T12 | Low | There is an undisclosed parcel radius clamp. | `transport-view:22-23` |
| T13 | Low | The legend says "span" but the values use the shared ±20 K range. | `transport-view:50-153` |

## Structure (owner: sonnet-structural)

| # | Severity | Defect | Evidence |
|---|---|---|---|
| S1 | High (in progress) | The structural view still draws in elevation coordinates. Fz and the moments are never drawn. The `📽️projection` module exists but nothing imports it and it has no tests. | `🏗️structural-view:98,189-199,264,330-346` |
| S2 | — | The viewer side and handedness conflict. Resolved by D6. | — |
| S3 | Med | The drawn member legend covers only eligible members, while `structuralLegend` covers all members. | `structural-view:64-66`; `🏗️structure:684-706` |
| S4 | Med | The contact-pressure legend ignores a fixed range and has a 1 Pa floor. | `structural-view:318-319` |
| S5 | Med | The 3D residual (fz, mx, my) is not asserted. | `T/⚖️equilibrium:22-23` |
| S6 | Low-med | Accelerations are zeroed in the first and last half-step of clips, so inertial forces jump at clip ends. | `🧮️evaluation:613,620` (owner: rigs) |

## Missing tests

- Thermal, forces, moments and transport drawn in front and plan views.
- Overlay coordinates per viewpoint.
- Magnitudes that stay the same across views.
- Continuity at the loop boundary.
- Wind consistency between thermal and transport.
- Auto-range invariance.
- The neutral grey clamp.
- 3D residuals and 3D oracle cases.
- Emitter positions under spread and twist.
