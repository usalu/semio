# Fidelity: actions, geotechnics, seismic (EN 1990, 1991, 1997, 1998)

Nothing was compiled (build loop: "ready for rebuild: scene fidelity" appended to `build-en{1990,1991,1997,1998}/latest.md`). Scene goldens need one `SEMIO_UPDATE_GOLDEN=1` run. Requests: `requests.md` section "fidelity-actions-geo-seismic".

## 1. Station emission (checks, pure additions, peak = utilisation so verdicts never change)

| Artifact | Check / field | Stations |
|---|---|---|
| EN 1990 | `en1990.a14.deflection`, `deflection-w3` (member `s`) | 11, elastic line 16/5(s-2s^3+s^4) x w, [w(s), limit] |
| EN 1991 | `en1991.1-3.snow.*` | 11 along roof, drift triangle mu2 -> mu1 over l_s, [s(x), assumed] |
| EN 1991 | `en1991.1-4.wind.*` (zone D) | 11 over height, q_p(z)/q_p(z_e) |
| EN 1997 | `en1997.6.5.bearing.*`, field `soil-pressure` | 9x3 grid, trapezoid for e<=B/6 else triangle over 3(B/2-e) |
| EN 1997 | `en1997.9.sliding.*`, field `earth-pressure` | 11 over wall height, k(gamma z + q)cos(delta) |
| EN 1997 | `en1997.11.bishop.*`, field `slip-slice` | 60 slices of the critical circle, mobilisation F d_i/r_i scaled to 1/F |
| EN 1997 | `en1997.7.6.2.compression.*`, field `pile-shaft` | one per soil layer, [R_s,i, N(z)] |
| EN 1998 | `…storeyForces.*` (`storey-shear`), `…drift.*` (`storey-drift`), `…pdelta.*` (`storey-pdelta`) | one per storey, s = i/(n-1) |

Not possible yet: EN 1991 cpe per face point and hipped roofs (no face polygons), EN 1998 joint capacity ratios (no M_Rd inputs), EN 1990 bridge modes via the mechanics kernel (members carry no EI/mass; the simply supported idealisation gives sin(k pi s) exactly).
Limitation to state honestly: the EN 1997 bearing, wall and slope distributions are shapes whose peak is scaled to the governing utilisation (law of the contract); pile utilisation is uniform per layer because the snapshot has one unit shaft resistance.

## 2. Scenes

- EN 1998: modes are the real shear-building eigenmodes (both directions, 3 each): `response::Mode { period, shape, gamma, m_eff_ratio, amplitude_m = Gamma gamma_I Se(T)(T/2pi)^2 }`; mode 1 (x) animates on slabs and columns with physical amplitude, participation and frequency, visual `scale` chosen per mode; modes x2, x3, y1..y3 are hidden ghost-slab layers; new `modes:<building>` diagram with period and M_eff; torsion/e_ai now use the perpendicular plan side, both e_ai axes drawn.
- EN 1997: footing `:contact-pressure` grid coloured from `soil-pressure` stations (`field_at_points`), wall pressure surface coloured from `earth-pressure`, slope slices coloured from `slip-slice` stations (`field_along`), pile shaft glyphs from `pile-shaft` stations; each falls back to the old drawing when stations are absent.
- EN 1990: deflected shape uses the elastic line (was a parabola); member colour follows deflection stations through `binding_of_check().per_station`.
- EN 1991: stations only (scene already binds checks); zone geometry blocked on fields.

## 3. Tests / oracles
EN 1998 oracle (numpy `eigh`) now returns all modes with gamma, M_eff, amplitude; scene tests compare them and add torsion-perpendicular and mode-ordering tests. EN 1997 oracle gets a numpy trapezoid check that contact stations integrate to V_d; scene test for the contact field and slip-slice peak. Feature files extended. Goldens regenerate on first run.

## 4. Suspicions
- EN 1998 torsion L_e swap: CONFIRMED. See requests.md (evidence: e0 uses perpendicular offsets, `plan_regularity_metrics` Lx = plan_width_m, standard measures L_e perpendicular to the action). Scene fixed, check left to the norm agent.
- EN 1999 support factor: CONFIRMED as double application for explicit buckling lengths (`member-checks:328-330` and scene `k_support`); snapshot docs define `buckling_length_*` as L_cr. Reported to the norm agent.

## 5. Files
EN 1998 checks `…/🏬️storey-forces-and-drift/🦀️.rs`; scene `🧮️response`, `🏢️buildings`, `🧰️kit`, `📈️diagrams`, tests, oracle. EN 1997 checks `📍️station-fields/🦀️.rs` (new), `🦀️.rs`, `🏗️project-checks`, `🧱️pile-wall-slope-and-uplift-checks`, `📐️geotechnical-parts`; scene `🦶️footings`, `🧱️walls`, `⛰️slopes`, `🏗️piles`, tests, oracle, feature. EN 1990 `📏️serviceability-and-fatigue`, scene `🏗️members`. EN 1991 inferences `🦀️.rs`.
