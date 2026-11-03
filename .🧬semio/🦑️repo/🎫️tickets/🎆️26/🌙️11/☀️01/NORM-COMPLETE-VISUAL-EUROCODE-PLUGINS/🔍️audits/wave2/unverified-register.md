# Unverified NDP Register (wave 2)

Source registry: `✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (3053 rows, generated read-only from the registry). Check sites = occurrences of the row id in `.rs` files under `✏️s/🔌️plugins/📕️norm/🗿️artifacts` (artifact check crates). Printed source = the standard (code and part) plus the clause text from the registry, which names the table or figure; the document and edition are taken from the registry `source` field.

- Rows: 3053; verified 258; unverified 2795; unverified rows awaiting an annex value 868.
- Per-annex status exists only on rows with an `annexStatus` object; those rows stay unverified until both annexes are verified.

## Totals per code

| Code | Rows | Verified | Unverified | Unverified awaiting value | Unverified with check sites |
|---|---:|---:|---:|---:|---:|
| EN 1990 | 270 | 0 | 270 | 25 | 61 |
| EN 1991 | 591 | 0 | 591 | 212 | 142 |
| EN 1992 | 208 | 0 | 208 | 75 | 144 |
| EN 1993 | 332 | 3 | 329 | 104 | 96 |
| EN 1994 | 101 | 0 | 101 | 66 | 25 |
| EN 1995 | 743 | 255 | 488 | 39 | 47 |
| EN 1996 | 294 | 0 | 294 | 124 | 95 |
| EN 1997 | 126 | 0 | 126 | 49 | 3 |
| EN 1998 | 253 | 0 | 253 | 87 | 79 |
| EN 1999 | 135 | 0 | 135 | 87 | 21 |
| **All** | 3053 | 258 | 2795 | 868 | 713 |

## Top 10 unverified rows by impact per code

Impact = number of `.rs` check sites in the artifact crates.

### EN 1990

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1990.gG_sup.str.A1` | gG_sup.str.A1 | 1.35 | 1.35 | 7 |
| 2 | `ndp.en1990.gQ.str.A1` | gQ.str.A1 | 1.5 | 1.5 | 6 |
| 3 | `ndp.en1990.a2.rail.acc_slab` | a2.rail.acc_slab | 5 m/s2 | 5 m/s2 | 4 |
| 4 | `ndp.en1990.a2.rail.speed_margin` | verification speed factor | 1.2 | 1.2 | 4 |
| 5 | `ndp.en1990.a2.rail.acc_ballast` | a2.rail.acc_ballast | 3.5 m/s2 | 3.5 m/s2 | 3 |
| 6 | `ndp.en1990.a2.rail.damping_default` | default damping ratio | 0.01 | 0.01 | 3 |
| 7 | `ndp.en1990.e90-28.1` | ? | awaiting | awaiting | 3 |
| 8 | `ndp.en1990.a2.footbridge.acc_v` | a2.footbridge.acc_v | 0.7 m/s2 | 0.7 m/s2 | 2 |
| 9 | `ndp.en1990.a2.group.gr2.road_traffic` | c gr2 road_traffic | awaiting | awaiting | 2 |
| 10 | `ndp.en1990.a2.rail.deflection.ratio` | a2.rail.deflection.ratio | 600 | 600 | 2 |

### EN 1991

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1991.e3.cranes` | e3.cranes | awaiting | awaiting | 15 |
| 2 | `ndp.en1991.e4.silo` | e4.silo | awaiting | awaiting | 12 |
| 3 | `ndp.en1991.e2.na-factors` | e2.na-factors | awaiting | awaiting | 11 |
| 4 | `ndp.en1991.e2.rail` | e2.rail | awaiting | awaiting | 10 |
| 5 | `ndp.en1991.e12.annex-a` | e12.annex-a | awaiting | awaiting | 9 |
| 6 | `ndp.en1991.e17.impact` | e17.impact | awaiting | awaiting | 8 |
| 7 | `ndp.en1991.e15.shifts` | e15.shifts | awaiting | awaiting | 7 |
| 8 | `ndp.en1991.e17.robust` | e17.robust | awaiting | awaiting | 7 |
| 9 | `ndp.en1991.e16.returns` | e16.returns | awaiting | awaiting | 6 |
| 10 | `ndp.en1991.e11.na-table` | e11.na-table | awaiting | awaiting | 5 |

### EN 1992

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1992.m-01.gamma-c-persistent` | gamma_c | 1.5 | 1.5 | 62 |
| 2 | `ndp.en1992.gamma-s` | gamma_s | 1.15 | 1.15 | 41 |
| 3 | `ndp.en1992.m-01.alpha-cc-persistent` | alpha_cc | 1 | 0.85 | 38 |
| 4 | `ndp.en1992.m-01.alpha-ct` | alpha_ct | 1 | 0.85 | 15 |
| 5 | `ndp.en1992.vmin-shear` | vmin_shear | 0.035 | 0.0525 | 12 |
| 6 | `ndp.en1992.vmin-shear-d800` | vmin_shear_d800 | 0.035 | 0.0375 | 11 |
| 7 | `ndp.en1992.gamma-inst` | gamma_inst | 1 | 1 | 10 |
| 8 | `ndp.en1992.gamma-s-fat` | gamma_s_fat | 1.15 | 1.15 | 10 |
| 9 | `ndp.en1992.punch-vrdmax-factor` | punch_vrdmax_factor | 0 | 1.4 | 8 |
| 10 | `ndp.en1992.beta-interior` | beta_interior | 1.15 | 1.1 | 7 |

### EN 1993

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1993.v-026.2` | Eq. 5.2 tolerance | 0.05 | 0.05 | 4 |
| 2 | `ndp.en1993.v-300.2` | Ramberg-Osgood exponent n of the built-in stainless grades | awaiting | awaiting | 4 |
| 3 | `ndp.en1993.gamma-m1` | γ_M1 | 1 | 1.1 | 3 |
| 4 | `ndp.en1993.v-010.1` | Table 3.1 nominal f_y and f_u (t <= 40 mm) | awaiting | awaiting | 3 |
| 5 | `ndp.en1993.v-880.1` | gM0 | awaiting | awaiting | 3 |
| 6 | `ndp.en1993.1-1.alpha-cr-min` | alpha_cr min | 3 | 3 | 2 |
| 7 | `ndp.en1993.1-1.g-modulus` | G | 81000 MPa | 81000 MPa | 2 |
| 8 | `ndp.en1993.1-8.gamma-m5` | gamma_M5 | 1 | 1 | 2 |
| 9 | `ndp.en1993.5.pile-driving-limit-ratio` | sigma_driving / f_y limit | 0.9 | 0.9 | 2 |
| 10 | `ndp.en1993.5.pile-full-capacity-driving-ratio` | sigma_driving / f_y full | 0.6 | 0.6 | 2 |

### EN 1994

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1994.2.rebar-fatigue-range` | 2.rebar-fatigue-range | awaiting | awaiting | 7 |
| 2 | `ndp.en1994.1-2.heff` | h_eff | awaiting | awaiting | 5 |
| 3 | `ndp.en1994.1-2.alpha-unexposed` | alpha_c,unexposed | 9 W/m2K | 9 W/m2K | 4 |
| 4 | `ndp.en1994.1-2.rho-concrete` | rho_c | 2300 kg/m3 | 2300 kg/m3 | 4 |
| 5 | `ndp.en1994.2.min-transverse-reinforcement` | rho_t | 0.002 | awaiting | 4 |
| 6 | `ndp.en1994.2.stud-cross-spacing` | e_t/d | 2.5 | awaiting | 4 |
| 7 | `ndp.en1994.2.stud-edge-distance` | e_D | 0.02 m | awaiting | 4 |
| 8 | `ndp.en1994.1-2.kc-theta` | 1-2.kc-theta | awaiting | awaiting | 3 |
| 9 | `ndp.en1994.1-2.tables` | 1-2.tables | awaiting | awaiting | 3 |
| 10 | `ndp.en1994.2.end-plate-bearing` | F_Rdu | awaiting | awaiting | 3 |

### EN 1995

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1995.fire.clt-fall-off-factor` | CLT char rate factor after glue line fall-off | 2 | 2 | 6 |
| 2 | `ndp.en1995.creep.time-constant-years` | time constant tau of the creep deflection curve | 5 a | 5 a | 5 |
| 3 | `ndp.en1995.fire.clt-fall-off-protect` | new char depth protecting the next CLT layer | 25 mm | 25 mm | 5 |
| 4 | `ndp.en1995.floor.f1-limit` | lower limit of the floor fundamental frequency | 8 Hz | 6 Hz | 5 |
| 5 | `ndp.en1995.deck.friction-coefficient` | friction coefficient between the laminations against slip | 0.3 | 0.3 | 3 |
| 6 | `ndp.en1995.deck.min-prestress` | minimum residual prestress of a stress-laminated deck after all losses | 0.35 N/mm2 | 0.35 N/mm2 | 3 |
| 7 | `ndp.en1995.floor.unit-load-a` | unit point load deflection limit a | 1.5 mm/kN | 0.5 mm/kN | 3 |
| 8 | `ndp.en1995.kh.density-limit` | characteristic density up to which the size factor k_h of solid timber applies (rho_k <= 700 kg/m3) | 700 kg/m3 | 700 kg/m3 | 3 |
| 9 | `ndp.en1995.panel.fire-load-ratio` | reduction factor eta_fi of the load in fire | 0.6 | 0.6 | 3 |
| 10 | `ndp.en1995.annex-c.eta.bolted` | eta bolted | 6 | 6 | 2 |

### EN 1996

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1996.m07.gamma` | m07.gamma | awaiting | awaiting | 10 |
| 2 | `ndp.en1996.m10.rho2` | m10.rho2 | awaiting | awaiting | 7 |
| 3 | `ndp.en1996.m81.phis` | m81.phis | awaiting | awaiting | 7 |
| 4 | `ndp.en1996.m04.fxk` | m04.fxk | awaiting | awaiting | 6 |
| 5 | `ndp.en1996.m14.beta` | m14.beta | awaiting | awaiting | 6 |
| 6 | `ndp.en1996.m40.flexure` | m40.flexure | awaiting | awaiting | 6 |
| 7 | `ndp.en1996.m06.creep` | m06.creep | awaiting | awaiting | 5 |
| 8 | `ndp.en1996.m06.e` | m06.e | awaiting | awaiting | 5 |
| 9 | `ndp.en1996.m12.einit` | m12.einit | awaiting | awaiting | 5 |
| 10 | `ndp.en1996.m31.yield-line` | m31.yield-line | awaiting | awaiting | 5 |

### EN 1997

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1997.7.7.lateral-deflection-limit` | y_lim | 0.01 m | awaiting | 1 |
| 2 | `ndp.en1997.9.blum-embedment-increase` | t/t0 | 1.2 | awaiting | 1 |
| 3 | `ndp.en1997.9.seismic.gamma-sliding` | gamma_Rh,E | 1 | awaiting | 1 |
| 4 | `ndp.en1997.a-acc.g` | γ_G accidental | 1 | 1 | 0 |
| 5 | `ndp.en1997.a-acc.q` | γ_Q accidental | 1 | 1 | 0 |
| 6 | `ndp.en1997.a1.g` | γ_G A1 | 1.35 | 1.35 | 0 |
| 7 | `ndp.en1997.a1.q` | γ_Q A1 | 1.5 | 1.5 | 0 |
| 8 | `ndp.en1997.a2.g` | γ_G A2 | 1 | 1 | 0 |
| 9 | `ndp.en1997.a2.q` | γ_Q A2 | 1.3 | 1.3 | 0 |
| 10 | `ndp.en1997.de.act.a.g` | γ_G accidental | awaiting | 1.1 | 0 |

### EN 1998

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1998.en5.liquefaction.annex-b.csr-factor` | τ_e factor | 0.65 | 0.65 | 9 |
| 2 | `ndp.en1998.en5.liquefaction.min-safety-factor` | F_L min | 1.25 | 1.25 | 6 |
| 3 | `ndp.en1998.en2.pier.gamma-0` | γ_0 | 1.35 | 1.35 | 5 |
| 4 | `ndp.en1998.en3.chord-rotation.sd-factor` | SD factor | 0.75 | 0.75 | 5 |
| 5 | `ndp.en1998.en5.pile.stiffness-contrast` | G_2/G_1 for kinematic interaction | 6 | 6 | 5 |
| 6 | `ndp.en1998.en1.diaphragm.transfer-factor` | γ_d transfer amplification | 1.3 | 1.3 | 4 |
| 7 | `ndp.en1998.en1.modal.min-mass-participation` | Σ M_eff / M of the modes taken into account | 0.9 | 0.9 | 4 |
| 8 | `ndp.en1997.sls.angular-distortion-limit` | limit of the angular distortion / relative rotation | 0.002 | 0.002 | 3 |
| 9 | `ndp.en1998.en1.masonry.min-thickness-m` | t_ef min | 0.24 m | 0.24 m | 3 |
| 10 | `ndp.en1998.en1.masonry.slenderness-max` | h_ef/t_ef max | 12 | 12 | 3 |

### EN 1999

| # | id | symbol | EN | NA-DE | check sites |
|---:|---|---|---|---|---:|
| 1 | `ndp.en1999.gamma-mf` | gamma_mf | 1 | 1.35 | 4 |
| 2 | `ndp.en1999.deflection-ratio` | deflection_ratio | 200 | 200 | 2 |
| 3 | `ndp.en1999.fire-alpha-c` | fire_alpha_c | 25 | 25 | 2 |
| 4 | `ndp.en1999.fire-emissivity` | fire_emissivity | 0.3 | 0.3 | 2 |
| 5 | `ndp.en1999.gamma-ff` | gamma_ff | 1 | 1 | 2 |
| 6 | `ndp.en1999.gamma-g` | gamma_g | 1.35 | 1.35 | 2 |
| 7 | `ndp.en1999.gamma-m-fi` | gamma_m_fi | 1 | 1 | 2 |
| 8 | `ndp.en1999.gamma-m1` | gamma_m1 | 1.1 | 1.1 | 2 |
| 9 | `ndp.en1999.gamma-m2` | gamma_m2 | 1.25 | 1.25 | 2 |
| 10 | `ndp.en1999.gamma-mw` | gamma_mw | 1.25 | 1.25 | 2 |

## Unverified rows by code, clause and table/figure

Each clause heading is the registry clause text (table or figure reference). Document and edition are shown per heading. Rows sorted by id.

## EN 1990 (270 unverified)

### EN 1990 · 1.05

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 1.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-26.1` | ? | awaiting | awaiting | 2 |  |

### EN 1990 · 1.3.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 1.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.l167.1` | NA.A1.3.1 | awaiting | awaiting | 0 |  |
| `ndp.en1990.l167.2` | NA.A2.x | awaiting | awaiting | 0 |  |
| `ndp.en1990.l167.3` | NA.B | awaiting | awaiting | 0 |  |
| `ndp.en1990.l167.4` | NA.2.1 | awaiting | awaiting | 0 |  |

### EN 1990 · 1.5

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-24.1` | ? | awaiting | awaiting | 2 |  |
| `ndp.en1990.e90-24.2` | ? | awaiting | awaiting | 2 |  |
| `ndp.en1990.e90-24.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1990.e90-24.4` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1990.na-de.1` | valid_for_category | awaiting | awaiting | 0 |  |

### EN 1990 · 1.6

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 1.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-25.1` | ? | awaiting | awaiting | 2 |  |

### EN 1990 · 1.7

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 1.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-28.1` | ? | awaiting | awaiting | 3 |  |

### EN 1990 · 4.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e12-07.1` | ? | awaiting | awaiting | 0 |  |

### EN 1990 · 5.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-12.alpha_cr_first_order` | α_cr first-order limit | 10 | 10 | 2 |  |

### EN 1990 · 6.10

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 6.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-16.1` | ExclusiveGroup | awaiting | awaiting | 2 |  |
| `ndp.en1990.e90-16.2` | ExclusiveGroup | awaiting | awaiting | 0 |  |

### EN 1990 · 6.4.4 EN 1991-2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.4 EN 1991-2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.f_lower_a` | n0 lower limit coefficient | 23.58 | 23.58 | 1 |  |
| `ndp.en1990.a2.rail.f_lower_b` | n0 lower limit exponent | -0.592 | -0.592 | 1 |  |
| `ndp.en1990.a2.rail.f_lower_break` | lower limit break span | 20 m | 20 m | 1 |  |
| `ndp.en1990.a2.rail.f_lower_short` | n0 lower limit, short spans | 80 m/s | 80 m/s | 1 |  |
| `ndp.en1990.a2.rail.f_range_max` | span range max | 100 m | 100 m | 2 |  |
| `ndp.en1990.a2.rail.f_range_min` | span range min | 4 m | 4 m | 2 |  |
| `ndp.en1990.a2.rail.f_upper_a` | n0 upper limit coefficient | 94.76 | 94.76 | 1 |  |
| `ndp.en1990.a2.rail.f_upper_b` | n0 upper limit exponent | -0.748 | -0.748 | 1 |  |

### EN 1990 · 6.4.6.3 EN 1991-2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.6.3 EN 1991-2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.damping_default` | default damping ratio | 0.01 | 0.01 | 3 |  |

### EN 1990 · 6.7

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-15.1` | ? | awaiting | awaiting | 0 |  |

### EN 1990 · A1.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a1.override.psi.allowed` | project ψ override allowed | 1 | 1 | 1 |  |

### EN 1990 · A1.3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a1.override.gamma.allowed` | project γ override allowed | 1 | 0 | 1 |  |
| `ndp.en1990.a1.override.k_fi.allowed` | project K_FI override allowed | 0 | 0 | 1 |  |
| `ndp.en1990.a1.override.xi.allowed` | project ξ override allowed | 1 | 0 | 1 |  |

### EN 1990 · A1.3.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gP_inf` | γP,favourable | 1 | 1 | 0 |  |
| `ndp.en1990.gP_sup` | γP,unfavourable | 1 | 1 | 0 |  |

### EN 1990 · A1.4.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.sls.w3.ratio` | sls.w3.ratio | 300 | 300 | 2 |  |
| `ndp.en1990.sls.w_net.ratio` | sls.w_net.ratio | 250 | 250 | 2 |  |

### EN 1990 · A1.4.3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.sls.frame_drift.ratio` | sls.frame_drift.ratio | 500 | 500 | 2 |  |
| `ndp.en1990.sls.storey_drift.ratio` | sls.storey_drift.ratio | 300 | 300 | 2 |  |

### EN 1990 · A1.4.4

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A1.4.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.sls.floor_frequency_hz` | sls.floor_frequency_hz | 3 Hz | 3 Hz | 2 |  |

### EN 1990 · A2.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-25.2` | ? | awaiting | awaiting | 2 |  |

### EN 1990 · A2.2.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.fw.speed_cap` | F*_W wind speed cap | 35 m/s | 35 m/s | 1 |  |
| `ndp.en1990.a2.wind_thermal.simultaneous` | wind and thermal simultaneous | 0 | 0 | 1 |  |

### EN 1990 · A2.4.3.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.footbridge.acc_h` | a2.footbridge.acc_h | 0.2 m/s2 | 0.2 m/s2 | 1 |  |
| `ndp.en1990.a2.footbridge.acc_h_crowd` | a2.footbridge.acc_h_crowd | 0.4 m/s2 | 0.4 m/s2 | 1 |  |
| `ndp.en1990.a2.footbridge.acc_v` | a2.footbridge.acc_v | 0.7 m/s2 | 0.7 m/s2 | 2 |  |
| `ndp.en1990.a2.footbridge.lock_in` | a2.footbridge.lock_in | 0.1 m/s2 | 0.1 m/s2 | 1 |  |

### EN 1990 · A2.4.4.2.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.4.4.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.acc_ballast` | a2.rail.acc_ballast | 3.5 m/s2 | 3.5 m/s2 | 3 |  |
| `ndp.en1990.a2.rail.acc_slab` | a2.rail.acc_slab | 5 m/s2 | 5 m/s2 | 4 |  |
| `ndp.en1990.a2.rail.dynamic_speed` | dynamic analysis speed | 200 km/h | 200 km/h | 2 |  |
| `ndp.en1990.a2.rail.speed_margin` | verification speed factor | 1.2 | 1.2 | 4 |  |

### EN 1990 · A2.4.4.2.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.4.4.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.twist_120` | a2.rail.twist_120 | 4.5 mm/3m | 4.5 mm/3m | 2 |  |
| `ndp.en1990.a2.rail.twist_200` | a2.rail.twist_200 | 3 mm/3m | 3 mm/3m | 1 |  |
| `ndp.en1990.a2.rail.twist_length` | twist base length | 3 m | 3 m | 1 |  |
| `ndp.en1990.a2.rail.twist_max` | a2.rail.twist_max | 1.5 mm/3m | 1.5 mm/3m | 1 |  |
| `ndp.en1990.a2.rail.twist_speed_1` | v1 twist band | 120 km/h | 120 km/h | 1 |  |
| `ndp.en1990.a2.rail.twist_speed_2` | v2 twist band | 200 km/h | 200 km/h | 1 |  |

### EN 1990 · A2.4.4.2.3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.4.4.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.end_rotation_high` | end rotation, v > band | 0.005 rad | 0.005 rad | 1 |  |
| `ndp.en1990.a2.rail.end_rotation_low` | end rotation, v <= band | 0.0065 rad | 0.0065 rad | 1 |  |
| `ndp.en1990.a2.rail.end_rotation_speed` | end rotation speed band | 160 km/h | 160 km/h | 1 |  |

### EN 1990 · A2.4.4.3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure A2.4.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.deflection.ratio` | a2.rail.deflection.ratio | 600 | 600 | 2 |  |

### EN 1990 · Annex E EN 1991-2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Annex E EN 1991-2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.rail.hslm.a1.axle_load` | HSLM-A1 axle load P | 170000 N | 170000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a1.axle_spacing` | HSLM-A1 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a1.coach_length` | HSLM-A1 coach length D | 18 m | 18 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a1.coaches` | HSLM-A1 coaches | 18 | 18 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a10.axle_load` | HSLM-A10 axle load P | 210000 N | 210000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a10.axle_spacing` | HSLM-A10 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a10.coach_length` | HSLM-A10 coach length D | 27 m | 27 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a10.coaches` | HSLM-A10 coaches | 11 | 11 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a2.axle_load` | HSLM-A2 axle load P | 200000 N | 200000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a2.axle_spacing` | HSLM-A2 axle spacing d | 3.5 m | 3.5 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a2.coach_length` | HSLM-A2 coach length D | 19 m | 19 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a2.coaches` | HSLM-A2 coaches | 17 | 17 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a3.axle_load` | HSLM-A3 axle load P | 180000 N | 180000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a3.axle_spacing` | HSLM-A3 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a3.coach_length` | HSLM-A3 coach length D | 20 m | 20 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a3.coaches` | HSLM-A3 coaches | 16 | 16 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a4.axle_load` | HSLM-A4 axle load P | 190000 N | 190000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a4.axle_spacing` | HSLM-A4 axle spacing d | 3 m | 3 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a4.coach_length` | HSLM-A4 coach length D | 21 m | 21 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a4.coaches` | HSLM-A4 coaches | 15 | 15 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a5.axle_load` | HSLM-A5 axle load P | 170000 N | 170000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a5.axle_spacing` | HSLM-A5 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a5.coach_length` | HSLM-A5 coach length D | 22 m | 22 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a5.coaches` | HSLM-A5 coaches | 14 | 14 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a6.axle_load` | HSLM-A6 axle load P | 180000 N | 180000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a6.axle_spacing` | HSLM-A6 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a6.coach_length` | HSLM-A6 coach length D | 23 m | 23 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a6.coaches` | HSLM-A6 coaches | 13 | 13 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a7.axle_load` | HSLM-A7 axle load P | 190000 N | 190000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a7.axle_spacing` | HSLM-A7 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a7.coach_length` | HSLM-A7 coach length D | 24 m | 24 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a7.coaches` | HSLM-A7 coaches | 13 | 13 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a8.axle_load` | HSLM-A8 axle load P | 190000 N | 190000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a8.axle_spacing` | HSLM-A8 axle spacing d | 2.5 m | 2.5 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a8.coach_length` | HSLM-A8 coach length D | 25 m | 25 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a8.coaches` | HSLM-A8 coaches | 12 | 12 | 0 |  |
| `ndp.en1990.a2.rail.hslm.a9.axle_load` | HSLM-A9 axle load P | 210000 N | 210000 N | 0 |  |
| `ndp.en1990.a2.rail.hslm.a9.axle_spacing` | HSLM-A9 axle spacing d | 2 m | 2 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a9.coach_length` | HSLM-A9 coach length D | 26 m | 26 m | 0 |  |
| `ndp.en1990.a2.rail.hslm.a9.coaches` | HSLM-A9 coaches | 11 | 11 | 0 |  |

### EN 1990 · C.7

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure C.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.form.alpha_e` | form.alpha_e | -0.7 | -0.7 | 1 |  |
| `ndp.en1990.form.alpha_r` | form.alpha_r | 0.8 | 0.8 | 1 |  |
| `ndp.en1990.form.ratio_hi` | form.ratio_hi | 7.6 | 7.6 | 2 |  |
| `ndp.en1990.form.ratio_lo` | form.ratio_lo | 0.16 | 0.16 | 2 |  |

### EN 1990 · E16.03

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure E16.03.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e16-03.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1990.e16-03.2` | ? | awaiting | awaiting | 0 |  |

### EN 1990 · E90.27

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure E90.27.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-27.1` | BridgeSls | awaiting | awaiting | 0 |  |
| `ndp.en1990.e90-27.2` | BridgeSls | awaiting | awaiting | 1 |  |
| `ndp.en1990.e90-27.3` | BridgeSls | awaiting | awaiting | 1 |  |

### EN 1990 · EN 1990 6.3.5

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 6.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gM.other` | gM.other | 1 | 1 | 0 |  |
| `ndp.en1990.gR` | gR | 1 | 1 | 2 |  |

### EN 1990 · EN 1992-1-1 2.4.2.4

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-1 2.4.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gM.concrete` | gM.concrete | 1.5 | 1.5 | 0 |  |

### EN 1990 · EN 1993-1-1 6.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure EN 1993-1-1 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gM.steel` | gM.steel | 1 | 1 | 0 |  |

### EN 1990 · EN 1995-1-1 2.4.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 2.4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gM.timber` | gM.timber | 1.3 | 1.3 | 0 |  |

### EN 1990 · NA.A.1.3.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure NA.A.1.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.simplified.gq` | simplified.gq | 1.35 | 1.35 | 1 |  |

### EN 1990 · Table A1.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.psi.congregation.0` | ψ0 congregation | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.congregation.1` | ψ1 congregation | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.congregation.2` | ψ2 congregation | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.office.0` | ψ0 office | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.office.1` | ψ1 office | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.office.2` | ψ2 office | 0.3 | 0.3 | 0 |  |
| `ndp.en1990.psi.residential.0` | ψ0 residential | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.residential.1` | ψ1 residential | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.residential.2` | ψ2 residential | 0.3 | 0.3 | 0 |  |
| `ndp.en1990.psi.retail.0` | ψ0 retail | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.retail.1` | ψ1 retail | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.retail.2` | ψ2 retail | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.roof.0` | ψ0 roof | 0 | 0 | 0 |  |
| `ndp.en1990.psi.roof.1` | ψ1 roof | 0 | 0 | 0 |  |
| `ndp.en1990.psi.roof.2` | ψ2 roof | 0 | 0 | 0 |  |
| `ndp.en1990.psi.snow.0` | ψ0 snow | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.snow.1` | ψ1 snow | 0.2 | 0.2 | 0 |  |
| `ndp.en1990.psi.snow.2` | ψ2 snow | 0 | 0 | 0 |  |
| `ndp.en1990.psi.snow_high.0` | ψ0 snow_high | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.snow_high.1` | ψ1 snow_high | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.snow_high.2` | ψ2 snow_high | 0.2 | 0.2 | 0 |  |
| `ndp.en1990.psi.storage.0` | ψ0 storage | 1 | 1 | 0 |  |
| `ndp.en1990.psi.storage.1` | ψ1 storage | 0.9 | 0.9 | 0 |  |
| `ndp.en1990.psi.storage.2` | ψ2 storage | 0.8 | 0.8 | 0 |  |
| `ndp.en1990.psi.temperature.0` | ψ0 temperature | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.temperature.1` | ψ1 temperature | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.temperature.2` | ψ2 temperature | 0 | 0 | 0 |  |
| `ndp.en1990.psi.traffic_heavy.0` | ψ0 traffic_heavy | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.traffic_heavy.1` | ψ1 traffic_heavy | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.traffic_heavy.2` | ψ2 traffic_heavy | 0.3 | 0.3 | 0 |  |
| `ndp.en1990.psi.traffic_light.0` | ψ0 traffic_light | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.traffic_light.1` | ψ1 traffic_light | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.traffic_light.2` | ψ2 traffic_light | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.wind.0` | ψ0 wind | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.wind.1` | ψ1 wind | 0.2 | 0.2 | 0 |  |
| `ndp.en1990.psi.wind.2` | ψ2 wind | 0 | 0 | 0 |  |
| `ndp.en1990.snow_altitude_m` | snow_altitude_m | 1000 m | 1000 m | 0 |  |

### EN 1990 · Table A1.2(A)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A1.2(A).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.equ.A1` | gG_inf.equ.A1 | 0.9 | 0.9 | 0 |  |
| `ndp.en1990.gG_sup.equ.A1` | gG_sup.equ.A1 | 1.1 | 1.1 | 0 |  |
| `ndp.en1990.gQ.equ.A1` | gQ.equ.A1 | 1.5 | 1.5 | 0 |  |

### EN 1990 · Table A1.2(B)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A1.2(B).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.str.A1` | gG_inf.str.A1 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.str.A1` | gG_sup.str.A1 | 1.35 | 1.35 | 7 |  |
| `ndp.en1990.gQ.str.A1` | gQ.str.A1 | 1.5 | 1.5 | 6 |  |
| `ndp.en1990.xi.str.A1` | xi.str.A1 | 0.85 | 0.85 | 1 |  |

### EN 1990 · Table A1.2(B) via GEO-2 (DA2*)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A1.2(B) via GEO-2 (DA2*).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.geo2.A1` | gG_inf.geo2.A1 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.geo2.A1` | gG_sup.geo2.A1 | 1.35 | 1.35 | 0 |  |
| `ndp.en1990.gQ.geo2.A1` | gQ.geo2.A1 | 1.5 | 1.5 | 0 |  |

### EN 1990 · Table A1.2(C)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A1.2(C).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.geo.A1` | gG_inf.geo.A1 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.geo.A1` | gG_sup.geo.A1 | 1 | 1 | 0 |  |
| `ndp.en1990.gQ.geo.A1` | gQ.geo.A1 | 1.3 | 1.3 | 0 |  |

### EN 1990 · Table A2.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.psi.construction_load.0` | ψ0 construction_load | 1 | 1 | 0 |  |
| `ndp.en1990.psi.construction_load.1` | ψ1 construction_load | 0 | 0 | 0 |  |
| `ndp.en1990.psi.construction_load.2` | ψ2 construction_load | 1 | 1 | 0 |  |
| `ndp.en1990.psi.road_gr3_pedestrian.0` | ψ0 road_gr3_pedestrian | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_gr3_pedestrian.1` | ψ1 road_gr3_pedestrian | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.road_gr3_pedestrian.2` | ψ2 road_gr3_pedestrian | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_gr4_crowd.0` | ψ0 road_gr4_crowd | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_gr4_crowd.1` | ψ1 road_gr4_crowd | 0.75 | 0.75 | 0 |  |
| `ndp.en1990.psi.road_gr4_crowd.2` | ψ2 road_gr4_crowd | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_horizontal.0` | ψ0 road_horizontal | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_horizontal.1` | ψ1 road_horizontal | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_horizontal.2` | ψ2 road_horizontal | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_pedestrian.0` | ψ0 road_pedestrian | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.road_pedestrian.1` | ψ1 road_pedestrian | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.road_pedestrian.2` | ψ2 road_pedestrian | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_single_axle.0` | ψ0 road_single_axle | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_single_axle.1` | ψ1 road_single_axle | 0.75 | 0.75 | 0 |  |
| `ndp.en1990.psi.road_single_axle.2` | ψ2 road_single_axle | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_special.0` | ψ0 road_special | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_special.1` | ψ1 road_special | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_special.2` | ψ2 road_special | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_traffic.0` | ψ0 road_traffic | 0.75 | 0.75 | 0 |  |
| `ndp.en1990.psi.road_traffic.1` | ψ1 road_traffic | 0.75 | 0.75 | 0 |  |
| `ndp.en1990.psi.road_traffic.2` | ψ2 road_traffic | 0 | 0 | 0 |  |
| `ndp.en1990.psi.road_udl.0` | ψ0 road_udl | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.road_udl.1` | ψ1 road_udl | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.road_udl.2` | ψ2 road_udl | 0 | 0 | 0 |  |
| `ndp.en1990.psi.snow_construction.0` | ψ0 snow_construction | 0.8 | 0.8 | 0 |  |
| `ndp.en1990.psi.snow_construction.1` | ψ1 snow_construction | 0 | 0 | 0 |  |
| `ndp.en1990.psi.snow_construction.2` | ψ2 snow_construction | 0 | 0 | 0 |  |
| `ndp.en1990.psi.temperature_bridge.0` | ψ0 temperature_bridge | 0.6 | 0.8 | 0 |  |
| `ndp.en1990.psi.temperature_bridge.1` | ψ1 temperature_bridge | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.temperature_bridge.2` | ψ2 temperature_bridge | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.wind_bridge.0` | ψ0 wind_bridge | 0.6 | 0.6 | 0 |  |
| `ndp.en1990.psi.wind_bridge.1` | ψ1 wind_bridge | 0.2 | 0.2 | 0 |  |
| `ndp.en1990.psi.wind_bridge.2` | ψ2 wind_bridge | 0 | 0 | 0 |  |
| `ndp.en1990.psi.wind_bridge_execution.0` | ψ0 wind_bridge_execution | 1 | 1 | 0 |  |
| `ndp.en1990.psi.wind_bridge_execution.1` | ψ1 wind_bridge_execution | 0 | 0 | 0 |  |
| `ndp.en1990.psi.wind_bridge_execution.2` | ψ2 wind_bridge_execution | 0 | 0 | 0 |  |

### EN 1990 · Table A2.2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.psi.footbridge_crowd.0` | ψ0 footbridge_crowd | 0 | 0 | 0 |  |
| `ndp.en1990.psi.footbridge_crowd.1` | ψ1 footbridge_crowd | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.footbridge_crowd.2` | ψ2 footbridge_crowd | 0 | 0 | 0 |  |
| `ndp.en1990.psi.footbridge_service.0` | ψ0 footbridge_service | 0 | 0 | 0 |  |
| `ndp.en1990.psi.footbridge_service.1` | ψ1 footbridge_service | 0.4 | 0.4 | 0 |  |
| `ndp.en1990.psi.footbridge_service.2` | ψ2 footbridge_service | 0 | 0 | 0 |  |

### EN 1990 · Table A2.3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.psi.rail_hslm.0` | ψ0 rail_hslm | 1 | 1 | 0 |  |
| `ndp.en1990.psi.rail_hslm.1` | ψ1 rail_hslm | 1 | 1 | 0 |  |
| `ndp.en1990.psi.rail_hslm.2` | ψ2 rail_hslm | 0 | 0 | 0 |  |
| `ndp.en1990.psi.rail_sw2.0` | ψ0 rail_sw2 | 0 | 0 | 0 |  |
| `ndp.en1990.psi.rail_sw2.1` | ψ1 rail_sw2 | 1 | 1 | 0 |  |
| `ndp.en1990.psi.rail_sw2.2` | ψ2 rail_sw2 | 0 | 0 | 0 |  |
| `ndp.en1990.psi.rail_traffic.0` | ψ0 rail_traffic | 0.8 | 0.8 | 0 |  |
| `ndp.en1990.psi.rail_traffic.1` | ψ1 rail_traffic | 0.8 | 0.8 | 0 |  |
| `ndp.en1990.psi.rail_traffic.2` | ψ2 rail_traffic | 0 | 0 | 0 |  |
| `ndp.en1990.psi.rail_unloaded.0` | ψ0 rail_unloaded | 1 | 1 | 0 |  |
| `ndp.en1990.psi.rail_unloaded.1` | ψ1 rail_unloaded | 0 | 0 | 0 |  |
| `ndp.en1990.psi.rail_unloaded.2` | ψ2 rail_unloaded | 0 | 0 | 0 |  |
| `ndp.en1990.psi.wind_rail.0` | ψ0 wind_rail | 0.75 | 0.75 | 0 |  |
| `ndp.en1990.psi.wind_rail.1` | ψ1 wind_rail | 0.5 | 0.5 | 0 |  |
| `ndp.en1990.psi.wind_rail.2` | ψ2 wind_rail | 0 | 0 | 0 |  |

### EN 1990 · Table A2.4

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.a2.group.gr1a.road_pedestrian` | c gr1a road_pedestrian | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr1a.road_traffic` | c gr1a road_traffic | 1 | 1 | 1 |  |
| `ndp.en1990.a2.group.gr1a.road_udl` | c gr1a road_udl | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr1b.road_single_axle` | c gr1b road_single_axle | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr2.road_horizontal` | c gr2 road_horizontal | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr2.road_traffic` | c gr2 road_traffic | awaiting | awaiting | 2 |  |
| `ndp.en1990.a2.group.gr3.road_gr3_pedestrian` | c gr3 road_gr3_pedestrian | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr4.road_gr4_crowd` | c gr4 road_gr4_crowd | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr5.road_special` | c gr5 road_special | 1 | 1 | 0 |  |
| `ndp.en1990.a2.group.gr5.road_traffic` | c gr5 road_traffic | awaiting | awaiting | 1 |  |

### EN 1990 · Table A2.4(A)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.4(A).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.equ.A2` | gG_inf.equ.A2 | 0.95 | 0.95 | 0 |  |
| `ndp.en1990.gG_sup.equ.A2` | gG_sup.equ.A2 | 1.05 | 1.05 | 0 |  |
| `ndp.en1990.gQ.other.equ.A2` | gQ.other.equ.A2 | 1.5 | 1.5 | 0 |  |
| `ndp.en1990.gQ.rail.equ.A2` | gQ.rail.equ.A2 | 1.45 | 1.45 | 0 |  |
| `ndp.en1990.gQ.road.equ.A2` | gQ.road.equ.A2 | 1.35 | 1.35 | 0 |  |

### EN 1990 · Table A2.4(B)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.4(B).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.str.A2` | gG_inf.str.A2 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.str.A2` | gG_sup.str.A2 | 1.35 | 1.35 | 0 |  |
| `ndp.en1990.gQ.other.str.A2` | gQ.other.str.A2 | 1.5 | 1.5 | 0 |  |
| `ndp.en1990.gQ.rail.str.A2` | gQ.rail.str.A2 | 1.45 | 1.45 | 0 |  |
| `ndp.en1990.gQ.road.str.A2` | gQ.road.str.A2 | 1.35 | 1.35 | 0 |  |
| `ndp.en1990.xi.str.A2` | xi.str.A2 | 0.85 | 0.85 | 0 |  |

### EN 1990 · Table A2.4(B) via GEO-2 (DA2*)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.4(B) via GEO-2 (DA2*).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.geo2.A2` | gG_inf.geo2.A2 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.geo2.A2` | gG_sup.geo2.A2 | 1.35 | 1.35 | 0 |  |
| `ndp.en1990.gQ.other.geo2.A2` | gQ.other.geo2.A2 | 1.5 | 1.5 | 0 |  |
| `ndp.en1990.gQ.rail.geo2.A2` | gQ.rail.geo2.A2 | 1.45 | 1.45 | 0 |  |
| `ndp.en1990.gQ.road.geo2.A2` | gQ.road.geo2.A2 | 1.35 | 1.35 | 0 |  |

### EN 1990 · Table A2.4(C)

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table A2.4(C).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.gG_inf.geo.A2` | gG_inf.geo.A2 | 1 | 1 | 0 |  |
| `ndp.en1990.gG_sup.geo.A2` | gG_sup.geo.A2 | 1 | 1 | 0 |  |
| `ndp.en1990.gQ.other.geo.A2` | gQ.other.geo.A2 | 1.3 | 1.3 | 0 |  |
| `ndp.en1990.gQ.rail.geo.A2` | gQ.rail.geo.A2 | 1.25 | 1.25 | 0 |  |
| `ndp.en1990.gQ.road.geo.A2` | gQ.road.geo.A2 | 1.15 | 1.15 | 0 |  |

### EN 1990 · Table B2

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table B2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.beta1.1` | beta1.1 | 4.2 | 4.2 | 0 |  |
| `ndp.en1990.beta1.2` | beta1.2 | 4.7 | 4.7 | 0 |  |
| `ndp.en1990.beta1.3` | beta1.3 | 5.2 | 5.2 | 0 |  |
| `ndp.en1990.beta50.1` | beta50.1 | 3.3 | 3.3 | 0 |  |
| `ndp.en1990.beta50.2` | beta50.2 | 3.8 | 3.8 | 0 |  |
| `ndp.en1990.beta50.3` | beta50.3 | 4.3 | 4.3 | 0 |  |

### EN 1990 · Table B3

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table B3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.e90-02.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1990.kfi.1` | kfi.1 | 0.9 | 0.9 | 0 |  |
| `ndp.en1990.kfi.2` | kfi.2 | 1 | 1 | 0 |  |
| `ndp.en1990.kfi.3` | kfi.3 | 1.1 | 1.1 | 0 |  |

### EN 1990 · Table NA.A.1.1

Printed source: DIN EN 1990/NA ed. to be recorded at verification. Verify: printed table or figure Table NA.A.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1990.psi.settlement.0` | ψ0 settlement | 1 | 1 | 0 |  |
| `ndp.en1990.psi.settlement.1` | ψ1 settlement | 1 | 1 | 0 |  |
| `ndp.en1990.psi.settlement.2` | ψ2 settlement | 1 | 1 | 0 |  |
| `ndp.en1990.psi.variable_other.0` | ψ0 variable_other | 0.8 | 0.8 | 0 |  |
| `ndp.en1990.psi.variable_other.1` | ψ1 variable_other | 0.7 | 0.7 | 0 |  |
| `ndp.en1990.psi.variable_other.2` | ψ2 variable_other | 0.5 | 0.5 | 0 |  |

## EN 1991 (591 unverified)

### EN 1991 · ?

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l482.1` | eurocodepy | awaiting | awaiting | 0 |  |
| `ndp.en1991.l491.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1991 · Table 6.1

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-14.1` | ? | awaiting | awaiting | 2 |  |
| `ndp.en1991.e2-14.2` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1991.e2-14.3` | ? | awaiting | awaiting | 1 |  |

### EN 1991 · e11

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11.alpha` | e11.alpha | awaiting | awaiting | 2 |  |
| `ndp.en1991.e11.barrier` | e11.barrier | awaiting | awaiting | 2 |  |
| `ndp.en1991.e11.density` | e11.density | awaiting | awaiting | 3 |  |
| `ndp.en1991.e11.forklift` | e11.forklift | awaiting | awaiting | 1 |  |
| `ndp.en1991.e11.na-table` | e11.na-table | awaiting | awaiting | 5 |  |
| `ndp.en1991.e11.partition` | e11.partition | awaiting | awaiting | 2 |  |

### EN 1991 · e12

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e12.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e12.annex-a` | e12.annex-a | awaiting | awaiting | 9 |  |
| `ndp.en1991.e12.annex-c` | e12.annex-c | awaiting | awaiting | 4 |  |
| `ndp.en1991.e12.annex-e` | e12.annex-e | awaiting | awaiting | 3 |  |
| `ndp.en1991.e12.annex-f` | e12.annex-f | awaiting | awaiting | 2 |  |
| `ndp.en1991.e12.firecurves` | e12.firecurves | awaiting | awaiting | 2 |  |

### EN 1991 · e13

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e13.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13.drift` | e13.drift | awaiting | awaiting | 4 |  |
| `ndp.en1991.e13.en-annex-c` | e13.en-annex-c | awaiting | awaiting | 1 |  |
| `ndp.en1991.e13.na-zones` | e13.na-zones | awaiting | awaiting | 4 |  |
| `ndp.en1991.e13.overhang` | e13.overhang | awaiting | awaiting | 3 |  |
| `ndp.en1991.e13.weights` | e13.weights | awaiting | awaiting | 1 |  |

### EN 1991 · e14

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e14.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14.annex-b` | e14.annex-b | awaiting | awaiting | 3 |  |
| `ndp.en1991.e14.annex-e` | e14.annex-e | awaiting | awaiting | 3 |  |
| `ndp.en1991.e14.bridge` | e14.bridge | awaiting | awaiting | 1 |  |
| `ndp.en1991.e14.cpe-flat` | e14.cpe-flat | awaiting | awaiting | 1 |  |
| `ndp.en1991.e14.cpe-pitched` | e14.cpe-pitched | awaiting | awaiting | 2 |  |
| `ndp.en1991.e14.na-profile` | e14.na-profile | awaiting | awaiting | 2 |  |

### EN 1991 · e15

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e15.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15.altitude` | e15.altitude | awaiting | awaiting | 2 |  |
| `ndp.en1991.e15.shifts` | e15.shifts | awaiting | awaiting | 7 |  |

### EN 1991 · e16

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e16.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e16.returns` | e16.returns | awaiting | awaiting | 6 |  |

### EN 1991 · e17

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e17.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e17.blast` | e17.blast | awaiting | awaiting | 5 |  |
| `ndp.en1991.e17.impact` | e17.impact | awaiting | awaiting | 8 |  |
| `ndp.en1991.e17.robust` | e17.robust | awaiting | awaiting | 7 |  |

### EN 1991 · e2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2.na-factors` | e2.na-factors | awaiting | awaiting | 11 |  |
| `ndp.en1991.e2.rail` | e2.rail | awaiting | awaiting | 10 |  |

### EN 1991 · e3

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3.cranes` | e3.cranes | awaiting | awaiting | 15 |  |

### EN 1991 · e4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure e4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e4.silo` | e4.silo | awaiting | awaiting | 12 |  |

### EN 1991-1 · 4.3.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.k-r.coef` | k_r coefficient | 0.19 | 0.19 | 1 |  |
| `ndp.en1991.1-4.k-r.exp` | k_r exponent | 0.07 | 0.07 | 1 |  |
| `ndp.en1991.1-4.k-r.z0-ref` | k_r reference roughness | 0.05 m | 0.05 m | 1 |  |

### EN 1991-1 · 5.3.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.mu2.alpha-ref` | mu_2 reference angle | 30 deg | 30 deg | 1 |  |
| `ndp.en1991.1-3.mu2.base` | mu_2 base | 0.8 | 0.8 | 1 |  |
| `ndp.en1991.1-3.mu2.multi-span-cap` | mu_2 multi-span cap | 1.6 | 1.6 | 1 |  |
| `ndp.en1991.1-3.mu2.slope` | mu_2 slope | 0.8 | 0.8 | 1 |  |

### EN 1991-1 · 5.3.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.mu3.base` | mu_3 base | 0.2 | 0.2 | 1 |  |
| `ndp.en1991.1-3.mu3.slope` | mu_3 slope | 10 | 10 | 1 |  |

### EN 1991-1 · 7.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.cf.signboard-default` | c_f signboard default | 1.8 | 1.8 | 1 |  |

### EN 1991-1 · Annex F

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex F.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.delta-s.concrete` | structural damping delta_s concrete | 0.1 | 0.1 | 0 |  |
| `ndp.en1991.1-4.delta-s.mixed` | structural damping delta_s mixed | 0.08 | 0.08 | 0 |  |
| `ndp.en1991.1-4.delta-s.steel` | structural damping delta_s steel | 0.05 | 0.05 | 0 |  |
| `ndp.en1991.1-4.mode-exponent.cantilever` | mode shape exponent cantilever | 1.5 | 1.5 | 0 |  |
| `ndp.en1991.1-4.mode-exponent.core` | mode shape exponent core | 1 | 1 | 0 |  |
| `ndp.en1991.1-4.mode-exponent.framed` | mode shape exponent framed | 0.6 | 0.6 | 0 |  |
| `ndp.en1991.1-4.mode-exponent.lattice` | mode shape exponent lattice | 2 | 2 | 0 |  |

### EN 1991-1 · NA.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure NA.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.exceptional-factor` | s_Ad/s_k | 2.3 | 2.3 | 1 |  |
| `ndp.en1991.1-3.snow-altitude-offset` | altitude offset | 140 m | 140 m | 1 |  |
| `ndp.en1991.1-3.snow-altitude-scale` | altitude scale | 760 m | 760 m | 1 |  |

### EN 1991-1 · NA.4 ground snow

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure NA.4 ground snow.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.snow-zone.z1.a` | s_k zone z1 a | awaiting | 0.19 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1.b` | s_k zone z1 b | awaiting | 0.91 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1.factor` | s_k zone z1 zone factor | awaiting | 1 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1.min` | s_k zone z1 s_k minimum | awaiting | 0.65 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1a.a` | s_k zone z1a a | awaiting | 0.19 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1a.b` | s_k zone z1a b | awaiting | 0.91 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1a.factor` | s_k zone z1a zone factor | awaiting | 1.25 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z1a.min` | s_k zone z1a s_k minimum | awaiting | 0.65 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2.a` | s_k zone z2 a | awaiting | 0.25 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2.b` | s_k zone z2 b | awaiting | 1.91 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2.factor` | s_k zone z2 zone factor | awaiting | 1 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2.min` | s_k zone z2 s_k minimum | awaiting | 0.85 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2a.a` | s_k zone z2a a | awaiting | 0.25 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2a.b` | s_k zone z2a b | awaiting | 1.91 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2a.factor` | s_k zone z2a zone factor | awaiting | 1.25 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z2a.min` | s_k zone z2a s_k minimum | awaiting | 0.85 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z3.a` | s_k zone z3 a | awaiting | 0.31 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z3.b` | s_k zone z3 b | awaiting | 2.91 kN/m2 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z3.factor` | s_k zone z3 zone factor | awaiting | 1 | 0 |  |
| `ndp.en1991.1-3.snow-zone.z3.min` | s_k zone z3 s_k minimum | awaiting | 1.1 kN/m2 | 0 |  |

### EN 1991-1 · NA.B simplified profile

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure NA.B simplified profile.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.na-profile.coast.c1` | q_p profile coast c1 | 1.8 | 1.8 | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.c2` | q_p profile coast c2 | 2.3 | 2.3 | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.c3` | q_p profile coast c3 | 2.6 | 2.6 | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.e2` | q_p profile coast e2 | 0.27 | 0.27 | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.e3` | q_p profile coast e3 | 0.19 | 0.19 | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.limit1` | q_p profile coast limit1 | 4 m | 4 m | 0 |  |
| `ndp.en1991.1-4.na-profile.coast.limit2` | q_p profile coast limit2 | 50 m | 50 m | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.c1` | q_p profile inland c1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.c2` | q_p profile inland c2 | 1.7 | 1.7 | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.c3` | q_p profile inland c3 | 2.1 | 2.1 | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.e2` | q_p profile inland e2 | 0.37 | 0.37 | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.e3` | q_p profile inland e3 | 0.24 | 0.24 | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.limit1` | q_p profile inland limit1 | 7 m | 7 m | 0 |  |
| `ndp.en1991.1-4.na-profile.inland.limit2` | q_p profile inland limit2 | 50 m | 50 m | 0 |  |

### EN 1991-1 · Table 4.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.terrain.i.z0` | z_0 category i | 0.01 m | 0.01 m | 0 |  |
| `ndp.en1991.1-4.terrain.i.zmin` | z_min category i | 1 m | 1 m | 0 |  |
| `ndp.en1991.1-4.terrain.ii.z0` | z_0 category ii | 0.05 m | 0.05 m | 0 |  |
| `ndp.en1991.1-4.terrain.ii.zmin` | z_min category ii | 2 m | 2 m | 0 |  |
| `ndp.en1991.1-4.terrain.iii.z0` | z_0 category iii | 0.3 m | 0.3 m | 0 |  |
| `ndp.en1991.1-4.terrain.iii.zmin` | z_min category iii | 5 m | 5 m | 0 |  |
| `ndp.en1991.1-4.terrain.iv.z0` | z_0 category iv | 1 m | 1 m | 0 |  |
| `ndp.en1991.1-4.terrain.iv.zmin` | z_min category iv | 10 m | 10 m | 0 |  |
| `ndp.en1991.1-4.terrain.zero.z0` | z_0 category zero | 0.003 m | 0.003 m | 0 |  |
| `ndp.en1991.1-4.terrain.zero.zmin` | z_min category zero | 1 m | 1 m | 0 |  |

### EN 1991-1 · Table 5.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.mu1.a0` | mu_1 at 0 deg | 0.8 | 0.8 | 1 |  |
| `ndp.en1991.1-3.mu1.a30` | mu_1 at 30 deg | 0.8 | 0.8 | 1 |  |
| `ndp.en1991.1-3.mu1.a60` | mu_1 at 60 deg | 0 | 0 | 1 |  |

### EN 1991-1 · Table 7.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.cpe.wall.a.h0-25.c1` | c_pe,1 zone A h/d h0-25 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.a.h0-25.c10` | c_pe,10 zone A h/d h0-25 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.a.h1.c1` | c_pe,1 zone A h/d h1 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.a.h1.c10` | c_pe,10 zone A h/d h1 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.a.h5.c1` | c_pe,1 zone A h/d h5 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.a.h5.c10` | c_pe,10 zone A h/d h5 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h0-25.c1` | c_pe,1 zone B h/d h0-25 | -1.1 | -1.1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h0-25.c10` | c_pe,10 zone B h/d h0-25 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h1.c1` | c_pe,1 zone B h/d h1 | -1.1 | -1.1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h1.c10` | c_pe,10 zone B h/d h1 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h5.c1` | c_pe,1 zone B h/d h5 | -1.1 | -1.1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.b.h5.c10` | c_pe,10 zone B h/d h5 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h0-25.c1` | c_pe,1 zone C h/d h0-25 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h0-25.c10` | c_pe,10 zone C h/d h0-25 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h1.c1` | c_pe,1 zone C h/d h1 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h1.c10` | c_pe,10 zone C h/d h1 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h5.c1` | c_pe,1 zone C h/d h5 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.c.h5.c10` | c_pe,10 zone C h/d h5 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h0-25.c1` | c_pe,1 zone D h/d h0-25 | 1 | 1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h0-25.c10` | c_pe,10 zone D h/d h0-25 | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h1.c1` | c_pe,1 zone D h/d h1 | 1 | 1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h1.c10` | c_pe,10 zone D h/d h1 | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h5.c1` | c_pe,1 zone D h/d h5 | 1 | 1 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.d.h5.c10` | c_pe,10 zone D h/d h5 | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h0-25.c1` | c_pe,1 zone E h/d h0-25 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h0-25.c10` | c_pe,10 zone E h/d h0-25 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h1.c1` | c_pe,1 zone E h/d h1 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h1.c10` | c_pe,10 zone E h/d h1 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h5.c1` | c_pe,1 zone E h/d h5 | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.wall.e.h5.c10` | c_pe,10 zone E h/d h5 | -0.7 | -0.7 | 0 |  |

### EN 1991-1 · Table 7.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-05.c1` | c_pe,1 flat roof curved zone F r0-05 | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-05.c10` | c_pe,10 flat roof curved zone F r0-05 | -1 | -1 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-10.c1` | c_pe,1 flat roof curved zone F r0-10 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-10.c10` | c_pe,10 flat roof curved zone F r0-10 | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-20.c1` | c_pe,1 flat roof curved zone F r0-20 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.f.r0-20.c10` | c_pe,10 flat roof curved zone F r0-20 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-05.c1` | c_pe,1 flat roof curved zone G r0-05 | -1.8 | -1.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-05.c10` | c_pe,10 flat roof curved zone G r0-05 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-10.c1` | c_pe,1 flat roof curved zone G r0-10 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-10.c10` | c_pe,10 flat roof curved zone G r0-10 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-20.c1` | c_pe,1 flat roof curved zone G r0-20 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.g.r0-20.c10` | c_pe,10 flat roof curved zone G r0-20 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-05.c1` | c_pe,1 flat roof curved zone H r0-05 | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-05.c10` | c_pe,10 flat roof curved zone H r0-05 | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-10.c1` | c_pe,1 flat roof curved zone H r0-10 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-10.c10` | c_pe,10 flat roof curved zone H r0-10 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-20.c1` | c_pe,1 flat roof curved zone H r0-20 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.curved.h.r0-20.c10` | c_pe,10 flat roof curved zone H r0-20 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.i.minus` | c_pe flat roof zone I suction | -0.2 | -0.2 | 2 |  |
| `ndp.en1991.1-4.cpe.flat.i.plus` | c_pe flat roof zone I pressure | 0.2 | 0.2 | 2 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a30.c1` | c_pe,1 flat roof mansard zone F a30 | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a30.c10` | c_pe,10 flat roof mansard zone F a30 | -1 | -1 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a45.c1` | c_pe,1 flat roof mansard zone F a45 | -1.8 | -1.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a45.c10` | c_pe,10 flat roof mansard zone F a45 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a60.c1` | c_pe,1 flat roof mansard zone F a60 | -1.9 | -1.9 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.f.a60.c10` | c_pe,10 flat roof mansard zone F a60 | -1.3 | -1.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a30.c1` | c_pe,1 flat roof mansard zone G a30 | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a30.c10` | c_pe,10 flat roof mansard zone G a30 | -1 | -1 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a45.c1` | c_pe,1 flat roof mansard zone G a45 | -1.9 | -1.9 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a45.c10` | c_pe,10 flat roof mansard zone G a45 | -1.3 | -1.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a60.c1` | c_pe,1 flat roof mansard zone G a60 | -1.9 | -1.9 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.g.a60.c10` | c_pe,10 flat roof mansard zone G a60 | -1.3 | -1.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a30.c1` | c_pe,1 flat roof mansard zone H a30 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a30.c10` | c_pe,10 flat roof mansard zone H a30 | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a45.c1` | c_pe,1 flat roof mansard zone H a45 | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a45.c10` | c_pe,10 flat roof mansard zone H a45 | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a60.c1` | c_pe,1 flat roof mansard zone H a60 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.mansard.h.a60.c10` | c_pe,10 flat roof mansard zone H a60 | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-025.c1` | c_pe,1 flat roof parapet zone F hp0-025 | -2.2 | -2.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-025.c10` | c_pe,10 flat roof parapet zone F hp0-025 | -1.6 | -1.6 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-05.c1` | c_pe,1 flat roof parapet zone F hp0-05 | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-05.c10` | c_pe,10 flat roof parapet zone F hp0-05 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-10.c1` | c_pe,1 flat roof parapet zone F hp0-10 | -1.8 | -1.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0-10.c10` | c_pe,10 flat roof parapet zone F hp0-10 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0.c1` | c_pe,1 flat roof parapet zone F hp0 | -2.5 | -2.5 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.f.hp0.c10` | c_pe,10 flat roof parapet zone F hp0 | -1.8 | -1.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-025.c1` | c_pe,1 flat roof parapet zone G hp0-025 | -1.8 | -1.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-025.c10` | c_pe,10 flat roof parapet zone G hp0-025 | -1.1 | -1.1 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-05.c1` | c_pe,1 flat roof parapet zone G hp0-05 | -1.6 | -1.6 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-05.c10` | c_pe,10 flat roof parapet zone G hp0-05 | -0.9 | -0.9 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-10.c1` | c_pe,1 flat roof parapet zone G hp0-10 | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0-10.c10` | c_pe,10 flat roof parapet zone G hp0-10 | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0.c1` | c_pe,1 flat roof parapet zone G hp0 | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.g.hp0.c10` | c_pe,10 flat roof parapet zone G hp0 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-025.c1` | c_pe,1 flat roof parapet zone H hp0-025 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-025.c10` | c_pe,10 flat roof parapet zone H hp0-025 | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-05.c1` | c_pe,1 flat roof parapet zone H hp0-05 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-05.c10` | c_pe,10 flat roof parapet zone H hp0-05 | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-10.c1` | c_pe,1 flat roof parapet zone H hp0-10 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0-10.c10` | c_pe,10 flat roof parapet zone H hp0-10 | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0.c1` | c_pe,1 flat roof parapet zone H hp0 | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.flat.parapet.h.hp0.c10` | c_pe,10 flat roof parapet zone H hp0 | -0.7 | -0.7 | 0 |  |

### EN 1991-1 · Tables 7.3a/7.4a

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Tables 7.3a/7.4a.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m15.c1` | c_pe,1 pitched roof zone F primary -15 deg | -2.8 | -2.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m15.c10` | c_pe,10 pitched roof zone F primary -15 deg | -2.5 | -2.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m30.c1` | c_pe,1 pitched roof zone F primary -30 deg | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m30.c10` | c_pe,10 pitched roof zone F primary -30 deg | -1.1 | -1.1 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m45.c1` | c_pe,1 pitched roof zone F primary -45 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m45.c10` | c_pe,10 pitched roof zone F primary -45 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m5.c1` | c_pe,1 pitched roof zone F primary -5 deg | -2.5 | -2.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.m5.c10` | c_pe,10 pitched roof zone F primary -5 deg | -2.3 | -2.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p15.c1` | c_pe,1 pitched roof zone F primary 15 deg | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p15.c10` | c_pe,10 pitched roof zone F primary 15 deg | -0.9 | -0.9 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p30.c1` | c_pe,1 pitched roof zone F primary 30 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p30.c10` | c_pe,10 pitched roof zone F primary 30 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p45.c1` | c_pe,1 pitched roof zone F primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p45.c10` | c_pe,10 pitched roof zone F primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p5.c1` | c_pe,1 pitched roof zone F primary 5 deg | -2.5 | -2.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p5.c10` | c_pe,10 pitched roof zone F primary 5 deg | -1.7 | -1.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p60.c1` | c_pe,1 pitched roof zone F primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p60.c10` | c_pe,10 pitched roof zone F primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p75.c1` | c_pe,1 pitched roof zone F primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.primary.p75.c10` | c_pe,10 pitched roof zone F primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p15.c1` | c_pe,1 pitched roof zone F secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p15.c10` | c_pe,10 pitched roof zone F secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p30.c1` | c_pe,1 pitched roof zone F secondary 30 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p30.c10` | c_pe,10 pitched roof zone F secondary 30 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p45.c1` | c_pe,1 pitched roof zone F secondary 45 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p45.c10` | c_pe,10 pitched roof zone F secondary 45 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p5.c1` | c_pe,1 pitched roof zone F secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.f.secondary.p5.c10` | c_pe,10 pitched roof zone F secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m15.c1` | c_pe,1 pitched roof zone G primary -15 deg | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m15.c10` | c_pe,10 pitched roof zone G primary -15 deg | -1.3 | -1.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m30.c1` | c_pe,1 pitched roof zone G primary -30 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m30.c10` | c_pe,10 pitched roof zone G primary -30 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m45.c1` | c_pe,1 pitched roof zone G primary -45 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m45.c10` | c_pe,10 pitched roof zone G primary -45 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m5.c1` | c_pe,1 pitched roof zone G primary -5 deg | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.m5.c10` | c_pe,10 pitched roof zone G primary -5 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p15.c1` | c_pe,1 pitched roof zone G primary 15 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p15.c10` | c_pe,10 pitched roof zone G primary 15 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p30.c1` | c_pe,1 pitched roof zone G primary 30 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p30.c10` | c_pe,10 pitched roof zone G primary 30 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p45.c1` | c_pe,1 pitched roof zone G primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p45.c10` | c_pe,10 pitched roof zone G primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p5.c1` | c_pe,1 pitched roof zone G primary 5 deg | -2 | -2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p5.c10` | c_pe,10 pitched roof zone G primary 5 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p60.c1` | c_pe,1 pitched roof zone G primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p60.c10` | c_pe,10 pitched roof zone G primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p75.c1` | c_pe,1 pitched roof zone G primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.primary.p75.c10` | c_pe,10 pitched roof zone G primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p15.c1` | c_pe,1 pitched roof zone G secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p15.c10` | c_pe,10 pitched roof zone G secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p30.c1` | c_pe,1 pitched roof zone G secondary 30 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p30.c10` | c_pe,10 pitched roof zone G secondary 30 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p45.c1` | c_pe,1 pitched roof zone G secondary 45 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p45.c10` | c_pe,10 pitched roof zone G secondary 45 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p5.c1` | c_pe,1 pitched roof zone G secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.g.secondary.p5.c10` | c_pe,10 pitched roof zone G secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m15.c1` | c_pe,1 pitched roof zone H primary -15 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m15.c10` | c_pe,10 pitched roof zone H primary -15 deg | -0.9 | -0.9 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m30.c1` | c_pe,1 pitched roof zone H primary -30 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m30.c10` | c_pe,10 pitched roof zone H primary -30 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m45.c1` | c_pe,1 pitched roof zone H primary -45 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m45.c10` | c_pe,10 pitched roof zone H primary -45 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m5.c1` | c_pe,1 pitched roof zone H primary -5 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.m5.c10` | c_pe,10 pitched roof zone H primary -5 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p15.c1` | c_pe,1 pitched roof zone H primary 15 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p15.c10` | c_pe,10 pitched roof zone H primary 15 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p30.c1` | c_pe,1 pitched roof zone H primary 30 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p30.c10` | c_pe,10 pitched roof zone H primary 30 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p45.c1` | c_pe,1 pitched roof zone H primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p45.c10` | c_pe,10 pitched roof zone H primary 45 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p5.c1` | c_pe,1 pitched roof zone H primary 5 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p5.c10` | c_pe,10 pitched roof zone H primary 5 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p60.c1` | c_pe,1 pitched roof zone H primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p60.c10` | c_pe,10 pitched roof zone H primary 60 deg | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p75.c1` | c_pe,1 pitched roof zone H primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.primary.p75.c10` | c_pe,10 pitched roof zone H primary 75 deg | 0.8 | 0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p15.c1` | c_pe,1 pitched roof zone H secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p15.c10` | c_pe,10 pitched roof zone H secondary 15 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p30.c1` | c_pe,1 pitched roof zone H secondary 30 deg | 0.4 | 0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p30.c10` | c_pe,10 pitched roof zone H secondary 30 deg | 0.4 | 0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p45.c1` | c_pe,1 pitched roof zone H secondary 45 deg | 0.6 | 0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p45.c10` | c_pe,10 pitched roof zone H secondary 45 deg | 0.6 | 0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p5.c1` | c_pe,1 pitched roof zone H secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.h.secondary.p5.c10` | c_pe,10 pitched roof zone H secondary 5 deg | 0 | 0 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m15.c1` | c_pe,1 pitched roof zone I primary -15 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m15.c10` | c_pe,10 pitched roof zone I primary -15 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m30.c1` | c_pe,1 pitched roof zone I primary -30 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m30.c10` | c_pe,10 pitched roof zone I primary -30 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m45.c1` | c_pe,1 pitched roof zone I primary -45 deg | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m45.c10` | c_pe,10 pitched roof zone I primary -45 deg | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m5.c1` | c_pe,1 pitched roof zone I primary -5 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.m5.c10` | c_pe,10 pitched roof zone I primary -5 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p15.c1` | c_pe,1 pitched roof zone I primary 15 deg | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p15.c10` | c_pe,10 pitched roof zone I primary 15 deg | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p30.c1` | c_pe,1 pitched roof zone I primary 30 deg | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p30.c10` | c_pe,10 pitched roof zone I primary 30 deg | -0.4 | -0.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p45.c1` | c_pe,1 pitched roof zone I primary 45 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p45.c10` | c_pe,10 pitched roof zone I primary 45 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p5.c1` | c_pe,1 pitched roof zone I primary 5 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p5.c10` | c_pe,10 pitched roof zone I primary 5 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p60.c1` | c_pe,1 pitched roof zone I primary 60 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p60.c10` | c_pe,10 pitched roof zone I primary 60 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p75.c1` | c_pe,1 pitched roof zone I primary 75 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.i.primary.p75.c10` | c_pe,10 pitched roof zone I primary 75 deg | -0.2 | -0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m15.c1` | c_pe,1 pitched roof zone J primary -15 deg | -1.2 | -1.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m15.c10` | c_pe,10 pitched roof zone J primary -15 deg | -0.7 | -0.7 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m30.c1` | c_pe,1 pitched roof zone J primary -30 deg | -1.4 | -1.4 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m30.c10` | c_pe,10 pitched roof zone J primary -30 deg | -0.8 | -0.8 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m45.c1` | c_pe,1 pitched roof zone J primary -45 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m45.c10` | c_pe,10 pitched roof zone J primary -45 deg | -1 | -1 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m5.c1` | c_pe,1 pitched roof zone J primary -5 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.m5.c10` | c_pe,10 pitched roof zone J primary -5 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p15.c1` | c_pe,1 pitched roof zone J primary 15 deg | -1.5 | -1.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p15.c10` | c_pe,10 pitched roof zone J primary 15 deg | -1 | -1 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p30.c1` | c_pe,1 pitched roof zone J primary 30 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p30.c10` | c_pe,10 pitched roof zone J primary 30 deg | -0.5 | -0.5 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p45.c1` | c_pe,1 pitched roof zone J primary 45 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p45.c10` | c_pe,10 pitched roof zone J primary 45 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p5.c1` | c_pe,1 pitched roof zone J primary 5 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p5.c10` | c_pe,10 pitched roof zone J primary 5 deg | -0.6 | -0.6 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p60.c1` | c_pe,1 pitched roof zone J primary 60 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p60.c10` | c_pe,10 pitched roof zone J primary 60 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p75.c1` | c_pe,1 pitched roof zone J primary 75 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.primary.p75.c10` | c_pe,10 pitched roof zone J primary 75 deg | -0.3 | -0.3 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.secondary.p5.c1` | c_pe,1 pitched roof zone J secondary 5 deg | 0.2 | 0.2 | 0 |  |
| `ndp.en1991.1-4.cpe.pitched0.j.secondary.p5.c10` | c_pe,10 pitched roof zone J secondary 5 deg | 0.2 | 0.2 | 0 |  |

### EN 1991-1-1 · 1.0

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-04.1` | allow_area_reduction | awaiting | awaiting | 0 |  |
| `ndp.en1991.e11-04.2` | allow_area_reduction | awaiting | awaiting | 0 |  |
| `ndp.en1991.e11-05.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e11-05.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e11-05.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · 2.0

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-03.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · 5.00

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.00.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l520.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · 6.3.1.2 NA

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.2 NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.alpha-a.de-c0` | α_A c0 | awaiting | 0.5 | 1 |  |
| `ndp.en1991.alpha-a.de-c1` | α_A c1 | awaiting | 10 m | 1 |  |
| `ndp.en1991.alpha-a.de-max` | α_A max | awaiting | 1 | 1 |  |
| `ndp.en1991.alpha-a.de-min` | α_A min | awaiting | 0.7 | 1 |  |

### EN 1991-1-1 · 6.3.1.2(10)

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.2(10).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.alpha-a.en-a0` | A0 | 10 m2 | awaiting | 2 |  |
| `ndp.en1991.alpha-a.en-min-cd` | α_A min C D | 0.6 | awaiting | 1 |  |

### EN 1991-1-1 · 6.3.1.2(8)

Printed source: DIN EN 1991/NA ed. to be recorded at verification; DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.2(8).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-02.1` | LineLoad | awaiting | awaiting | 0 |  |
| `ndp.en1991.partition.band1.limit` | g1 | 1 kN/m | 3 kN/m | 0 |  |
| `ndp.en1991.partition.band1.q` | q1 | 0.5 kN/m2 | 0.8 kN/m2 | 0 |  |
| `ndp.en1991.partition.band2.limit` | g2 | 2 kN/m | 5 kN/m | 0 |  |
| `ndp.en1991.partition.band2.q` | q2 | 0.8 kN/m2 | 1.2 kN/m2 | 0 |  |
| `ndp.en1991.partition.band3.limit` | g3 | 3 kN/m | 6 kN/m | 0 |  |
| `ndp.en1991.partition.band3.q` | q3 | 1.2 kN/m2 | 1.6 kN/m2 | 0 |  |

### EN 1991-1-1 · Annex A Table A.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex A Table A.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.density.steel` | γ steel | 77 kN/m3 | 78.5 kN/m3 | 2 |  |

### EN 1991-1-1 · Annex B

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex B.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.snow.drift-mu2-cap` | μ2 max | 4 | 2 | 2 |  |

### EN 1991-1-1 · E11.01

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure E11.01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e11-01.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · E11.06

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure E11.06.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-06.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · E11.07

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure E11.07.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-07.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · E11.09

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure E11.09.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-09.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · Table 6.1DE

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.1DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-03.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · Table 6.2

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-03.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-1 · Table 6.5

Printed source: DIN EN 1991-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e11-07.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · 1.0

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e90-17.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · 1.8

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 1.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.c-44.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · 2.2

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e12-03.1` | fire_load_density_qf | awaiting | awaiting | 0 |  |
| `ndp.en1991.e12-03.2` | fire_load_density_qf | awaiting | awaiting | 0 |  |
| `ndp.en1991.e12-03.3` | fire_load_density_qf | awaiting | awaiting | 0 |  |
| `ndp.en1991.e12-03.4` | fire_load_density_qf | awaiting | awaiting | 0 |  |
| `ndp.en1991.e12-10.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · 3.1 Eq. 3.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 3.1 Eq. 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-2.configuration-factor` | Phi | 1 | 1 | 1 |  |
| `ndp.en1991.1-2.flame-emissivity` | eps_f | 1 | 1 | 1 |  |
| `ndp.en1991.1-2.member-emissivity` | eps_m | 0.8 | 0.8 | 1 |  |

### EN 1991-1-2 · Annex E

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex E.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-2.combustion-factor` | m | 0.8 | 0.8 | 4 |  |

### EN 1991-1-2 · Annex E Table E.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex E Table E.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-2.fire-load-fractile.dwelling` | q_f,k 80 % fractile dwelling | 948 MJ/m2 | 948 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.hospital` | q_f,k 80 % fractile hospital | 280 MJ/m2 | 280 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.hotel` | q_f,k 80 % fractile hotel | 377 MJ/m2 | 377 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.library` | q_f,k 80 % fractile library | 1824 MJ/m2 | 1824 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.office` | q_f,k 80 % fractile office | 511 MJ/m2 | 511 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.school` | q_f,k 80 % fractile school | 347 MJ/m2 | 347 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.shopping` | q_f,k 80 % fractile shopping | 730 MJ/m2 | 730 MJ/m2 | 1 |  |
| `ndp.en1991.1-2.fire-load-fractile.theatre` | q_f,k 80 % fractile theatre | 365 MJ/m2 | 365 MJ/m2 | 0 |  |
| `ndp.en1991.1-2.fire-load-fractile.transport` | q_f,k 80 % fractile transport | 122 MJ/m2 | 122 MJ/m2 | 0 |  |

### EN 1991-1-2 · E12.04

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure E12.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e12-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · E12.05

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure E12.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e12-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-2 · E12.08

Printed source: DIN EN 1991-1-2/NA ed. to be recorded at verification. Verify: printed table or figure E12.08.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e12-08.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · 1.6

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure 1.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-03.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · 2.3

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure 2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-06.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.na-de.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.na-de.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · 5.3.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.cylindrical-mu3-cap` | mu3 cap cylindrical | 2 | 2 | 1 |  |

### EN 1991-1-3 · 5.3.6

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-3.drift-mu-w-reading` | mu_w limit reading | 0 | 0 | 1 |  |
| `ndp.en1991.1-3.sliding-share` | sliding share | 0.5 | 0.5 | 1 |  |

### EN 1991-1-3 · E13.01

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure E13.01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e13-01.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · E13.04

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure E13.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · E13.05

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure E13.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-3 · E13.11

Printed source: DIN EN 1991-1-3/NA ed. to be recorded at verification. Verify: printed table or figure E13.11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e13-11.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 1.10

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure 1.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-02.4` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-02.5` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-02.6` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 1.25

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure 1.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l569.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 2.4

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure 2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-02.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-02.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-02.3` | mixed_terrain_upwind | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-07.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-08.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-12.5` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-14.1` | ? | awaiting | awaiting | 2 |  |
| `ndp.en1991.e14-14.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 5.3

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-11.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-11.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-11.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 6.1

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-12.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1991.e14-12.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-12.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-12.4` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · 6.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.cscd-simple-height` | h limit c_s c_d = 1 | 15 m | 15 m | 1 |  |

### EN 1991-1-4 · 7.2.9

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.c-pi.1` | c_pi overpressure | 0.2 | 0.2 | 1 |  |
| `ndp.en1991.1-4.c-pi.2` | c_pi suction | -0.3 | -0.3 | 1 |  |

### EN 1991-1-4 · 8.3.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 8.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.8.traffic-rail-add` | d rail traffic | 4 m | 4 m | 1 |  |
| `ndp.en1991.8.traffic-road-add` | d road traffic | 2 m | 2 m | 1 |  |

### EN 1991-1-4 · 8.3.3

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 8.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.8.cfz` | c_f,z | 0.9 | 0.9 | 1 |  |

### EN 1991-1-4 · 8.3.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 8.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.8.longitudinal` | F_Wy/F_Wx | 0.25 | 0.25 | 1 |  |

### EN 1991-1-4 · E.1.3.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure E.1.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.kinematic-viscosity` | nu air | 1.5e-05 m2/s | 1.5e-05 m2/s | 1 |  |
| `ndp.en1991.1-4.vortex-onset` | v_crit/v_m onset factor | 1.25 | 1.25 | 1 |  |

### EN 1991-1-4 · E14.01

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-01.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-01.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-01.4` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · E14.03

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.03.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-03.1` | orography_factor | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · E14.04

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · E14.09

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.09.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-09.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · E14.13

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.13.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-13.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1991.e14-13.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · E14.15

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure E14.15.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-15.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · Fig. 8.3

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Fig. 8.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.8.cfx0-b-narrow` | b/d_tot upper plateau limit | 0.5 | 0.5 | 1 |  |
| `ndp.en1991.8.cfx0-b-wide` | b/d_tot lower plateau limit | 4 | 4 | 1 |  |
| `ndp.en1991.8.cfx0-narrow` | c_f,x0 at b/d_tot <= 0.5 | 2.4 | 2.4 | 1 |  |
| `ndp.en1991.8.cfx0-wide` | c_f,x0 at b/d_tot >= 4 | 1.3 | 1.3 | 1 |  |

### EN 1991-1-4 · NA.A.1 Table NA.A.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure NA.A.1 Table NA.A.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.vb0.zone1` | v_b,0 zone 1 | awaiting | 22.5 m/s | 2 |  |
| `ndp.en1991.vb0.zone2` | v_b,0 zone 2 | awaiting | 25 m/s | 1 |  |
| `ndp.en1991.vb0.zone3` | v_b,0 zone 3 | awaiting | 27.5 m/s | 1 |  |
| `ndp.en1991.vb0.zone4` | v_b,0 zone 4 | awaiting | 30 m/s | 1 |  |

### EN 1991-1-4 · Table 7.

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure Table 7..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-11.5` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e14-16.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · Table 7.13

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.13.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-11.4` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · Table 7.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-4.hipped-cpe1.f` | c_pe,1 zone F hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.g` | c_pe,1 zone G hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.h` | c_pe,1 zone H hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.i` | c_pe,1 zone I hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.j` | c_pe,1 zone J hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.k` | c_pe,1 zone K hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.l` | c_pe,1 zone L hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.m` | c_pe,1 zone M hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe1.n` | c_pe,1 zone N hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.f` | c_pe,10 zone F hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.g` | c_pe,10 zone G hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.h` | c_pe,10 zone H hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.i` | c_pe,10 zone I hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.j` | c_pe,10 zone J hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.k` | c_pe,10 zone K hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.l` | c_pe,10 zone L hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.m` | c_pe,10 zone M hipped roof | awaiting | awaiting | 0 |  |
| `ndp.en1991.1-4.hipped-cpe10.n` | c_pe,10 zone N hipped roof | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · Table 7.9

Printed source: DIN EN 1991-1-4/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e14-10.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-4 · Table 8.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 8.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.8.parapet-open-add` | d open parapet per side | 0.3 m | 0.3 m | 1 |  |

### EN 1991-1-5 · 2.1

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.v-900.1` |  approx. with  | awaiting | awaiting | 0 |  |
| `ndp.en1991.v-900.2` | Tmd | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · 2.5

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e15-01.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e15-01.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · 6.1.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-5.omega-m` | omega_M | 0.75 | 0.75 | 1 |  |
| `ndp.en1991.1-5.omega-n` | omega_N | 0.35 | 0.35 | 1 |  |

### EN 1991-1-5 · 7.6

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 7.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l603.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · Annex A

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex A.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-5.isotherm-lapse` | altitude correction | 0.5 K/100 m | 0.5 K/100 m | 1 |  |
| `ndp.en1991.1-5.isotherm-t-max-base` | T_max base | 37 degC | 37 degC | 1 |  |
| `ndp.en1991.1-5.isotherm-t-min-base` | T_min base | -24 degC | -24 degC | 1 |  |

### EN 1991-1-5 · E15.02

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure E15.02.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-02.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e15-02.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e15-02.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · E15.03

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure E15.03.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-03.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · E15.05

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure E15.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · E15.07

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure E15.07.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-07.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · Table 5.1

Printed source: DIN EN 1991-1-5/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e15-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-5 · Table 6.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-5.dtm-cool.1` | dT_M,cool steel deck | 13 K | 13 K | 0 |  |
| `ndp.en1991.1-5.dtm-cool.2` | dT_M,cool composite deck | 18 K | 18 K | 0 |  |
| `ndp.en1991.1-5.dtm-cool.3` | dT_M,cool concrete box girder | 5 K | 5 K | 0 |  |
| `ndp.en1991.1-5.dtm-cool.4` | dT_M,cool concrete slab | 8 K | 8 K | 0 |  |
| `ndp.en1991.1-5.dtm-cool.5` | dT_M,cool concrete beam | 8 K | 8 K | 0 |  |
| `ndp.en1991.1-5.dtm-heat.1` | dT_M,heat steel deck | 18 K | 18 K | 0 |  |
| `ndp.en1991.1-5.dtm-heat.2` | dT_M,heat composite deck | 15 K | 15 K | 0 |  |
| `ndp.en1991.1-5.dtm-heat.3` | dT_M,heat concrete box girder | 10 K | 10 K | 0 |  |
| `ndp.en1991.1-5.dtm-heat.4` | dT_M,heat concrete slab | 15 K | 15 K | 0 |  |
| `ndp.en1991.1-5.dtm-heat.5` | dT_M,heat concrete beam | 15 K | 15 K | 0 |  |

### EN 1991-1-5 · Table 6.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-5.ksur-cool.0` | k_sur cooling surfacing 0 | 0.9 | 0.9 | 0 |  |
| `ndp.en1991.1-5.ksur-cool.1` | k_sur cooling surfacing 1 | 1.1 | 1.1 | 0 |  |
| `ndp.en1991.1-5.ksur-cool.2` | k_sur cooling surfacing 2 | 1 | 1 | 0 |  |
| `ndp.en1991.1-5.ksur-cool.3` | k_sur cooling surfacing 3 | 1.2 | 1.2 | 0 |  |
| `ndp.en1991.1-5.ksur-cool.4` | k_sur cooling surfacing 4 | 1.2 | 1.2 | 0 |  |
| `ndp.en1991.1-5.ksur-heat.0` | k_sur heating surfacing 0 | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-5.ksur-heat.1` | k_sur heating surfacing 1 | 1.6 | 1.6 | 0 |  |
| `ndp.en1991.1-5.ksur-heat.2` | k_sur heating surfacing 2 | 1 | 1 | 0 |  |
| `ndp.en1991.1-5.ksur-heat.3` | k_sur heating surfacing 3 | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-5.ksur-heat.4` | k_sur heating surfacing 4 | 0.7 | 0.7 | 0 |  |
| `ndp.en1991.1-5.ksur-other` | k_sur other decks | 1 | 1 | 2 |  |

### EN 1991-1-6 · 1.0

Printed source: DIN EN 1991-1-6/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e16-02.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e16-02.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e16-02.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-6 · Annex A.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex A.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-6.combination-psi0` | psi_0 of construction loads | 1 | 1 | 3 |  |

### EN 1991-1-6 · E16.01

Printed source: DIN EN 1991-1-6/NA ed. to be recorded at verification. Verify: printed table or figure E16.01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e16-01.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-6 · Table 3.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e6-01.1` | T | awaiting | awaiting | 2 |  |

### EN 1991-1-6 · Table 3.2

Printed source: DIN EN 1991-1-6/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e16-01.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-7 · 4.3

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.impact-height-max` | impact height max | 1.5 m | 1.5 m | 1 |  |
| `ndp.en1991.1-7.impact-height-min` | impact height min | 0.5 m | 0.5 m | 1 |  |

### EN 1991-1-7 · 4.4 Tables 4.4-4.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.4 Tables 4.4-4.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.derailment-table` | derailment forces F_dx, F_dy | 0 | 0 | 3 |  |

### EN 1991-1-7 · 4.5

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.forklift-factor` | F_d,x/W forklift | 5 | 5 | 1 |  |

### EN 1991-1-7 · 4.7

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.helicopter-c` | C in F_d = C sqrt(m) | 3000 N/sqrt(kg) | 3000 N/sqrt(kg) | 1 |  |

### EN 1991-1-7 · 6.0

Printed source: DIN EN 1991-1-7/NA ed. to be recorded at verification. Verify: printed table or figure 6.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l621.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.l621.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-7 · 7.8

Printed source: DIN EN 1991-1-7/NA ed. to be recorded at verification. Verify: printed table or figure 7.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l631.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-7 · Annex A.5.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex A.5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.tie-cap` | tie force cap | 75000 N | 75000 N | 1 |  |
| `ndp.en1991.1-7.tie-internal` | internal tie factor | 0.8 | 0.8 | 1 |  |
| `ndp.en1991.1-7.tie-peripheral` | peripheral tie factor | 0.4 | 0.4 | 1 |  |

### EN 1991-1-7 · Annex C

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex C.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.vehicle-stiffness` | k | 300000 N/m | 300000 N/m | 1 |  |

### EN 1991-1-7 · Annex D

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex D.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.gas-base` | p_d base | 3 kN/m2 | 3 kN/m2 | 2 |  |
| `ndp.en1991.1-7.gas-vent-coefficient` | p_d vent term | 0.04 kN/m2 | 0.04 kN/m2 | 1 |  |
| `ndp.en1991.1-7.gas-volume-max` | V max | 1000 m3 | 1000 m3 | 2 |  |
| `ndp.en1991.1-7.gas-volume-min` | V min | 5 m3 | 5 m3 | 2 |  |

### EN 1991-1-7 · E17.02

Printed source: DIN EN 1991-1-7/NA ed. to be recorded at verification. Verify: printed table or figure E17.02.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e17-02.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e17-02.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e17-02.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e17-02.4` | ? | awaiting | awaiting | 0 |  |

### EN 1991-1-7 · Table 4.1

Printed source: DIN EN 1991-1-7/NA ed. to be recorded at verification; DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Table 4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.1-7.lateral-ratio` | F_dy/F_dx | 0.5 | 0.5 | 1 |  |
| `ndp.en1991.e17-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e17-01.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e17-01.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-2 · 1.9

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure 1.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.c-48.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-2 · 4.3.2 (3)

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.3.2 (3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.alpha-q-udl.1` | alpha_q1 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q-udl.2` | alpha_q2 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q-udl.3` | alpha_q3 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q-udl.4` | alpha_q4 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q.1` | alpha_Q1 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q.2` | alpha_Q2 | 1 | 1 | 0 |  |
| `ndp.en1991.2.alpha-q.3` | alpha_Q3 | 1 | 1 | 0 |  |

### EN 1991-2 · 4.3.2 Fig. 4.2b

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.3.2 Fig. 4.2b.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.lm1-axle-spacing` | axle spacing | 1.2 m | 1.2 m | 2 |  |

### EN 1991-2 · 4.4.1 (2)

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1 (2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.braking-max` | 900 | 900000 N | 900000 N | 1 |  |
| `ndp.en1991.2.braking-min-per-alpha` | 180 alpha_Q1 | 180000 N | 180000 N | 1 |  |

### EN 1991-2 · 4.4.1 (2) Eq. 4.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1 (2) Eq. 4.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.braking-q-factor` | 0.6 | 0.6 | 0.6 | 1 |  |
| `ndp.en1991.2.braking-udl-factor` | 0.10 | 0.1 | 0.1 | 1 |  |

### EN 1991-2 · 4.4.2 Table 4.3

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.2 Table 4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.centrifugal-factor-low` | 0.2 | 0.2 | 0.2 | 1 |  |
| `ndp.en1991.2.centrifugal-numerator` | 40 | 40 m | 40 m | 1 |  |
| `ndp.en1991.2.centrifugal-r-high` | r upper | 1500 m | 1500 m | 1 |  |
| `ndp.en1991.2.centrifugal-r-low` | r lower | 200 m | 200 m | 1 |  |

### EN 1991-2 · 5.4

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-01.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-2 · 6.3.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.rail-alpha.1` | alpha 1 | 0.75 | 0.75 | 1 |  |
| `ndp.en1991.2.rail-alpha.2` | alpha 2 | 0.83 | 0.83 | 1 |  |
| `ndp.en1991.2.rail-alpha.3` | alpha 3 | 0.91 | 0.91 | 1 |  |
| `ndp.en1991.2.rail-alpha.4` | alpha 4 | 1 | 1 | 1 |  |
| `ndp.en1991.2.rail-alpha.5` | alpha 5 | 1.1 | 1.1 | 1 |  |
| `ndp.en1991.2.rail-alpha.6` | alpha 6 | 1.21 | 1.21 | 1 |  |
| `ndp.en1991.2.rail-alpha.7` | alpha 7 | 1.33 | 1.33 | 1 |  |
| `ndp.en1991.2.rail-alpha.8` | alpha 8 | 1.46 | 1.46 | 1 |  |

### EN 1991-2 · ?

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.t-122.1` | Qd | awaiting | awaiting | 0 |  |

### EN 1991-2 · Annex A

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure Annex A.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.lm3-axle-spacing` | LM3 axle line spacing | 1.5 m | 1.5 m | 1 |  |
| `ndp.en1991.2.lm3-vehicle-width` | LM3 vehicle width | 3 m | 3 m | 2 |  |

### EN 1991-2 · E2.02

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure E2.02.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-02.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-2 · E2.04

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure E2.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-2 · E2.07

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure E2.07.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-07.1` | fem | awaiting | awaiting | 0 |  |
| `ndp.en1991.e2-07.2` | fem | awaiting | awaiting | 0 |  |
| `ndp.en1991.e2-07.3` | fem | awaiting | awaiting | 0 |  |
| `ndp.en1991.e2-07.5` | fem | awaiting | awaiting | 0 |  |

### EN 1991-2 · EN 1990 A2.4.3.2(1)

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 A2.4.3.2(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.footbridge-band-horizontal-max` | upper bound of the horizontal and torsional natural frequency band needing the comfort check | 2.5 Hz | 2.5 Hz | 2 |  |
| `ndp.en1991.2.footbridge-band-vertical-max` | upper bound of the vertical natural frequency band needing the comfort check | 5 Hz | 5 Hz | 2 |  |

### EN 1991-2 · EN 1991-2 5.7

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure EN 1991-2 5.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.2.footbridge-band-jogging-max` | upper bound of the jogging excitation band | 3.5 Hz | 3.5 Hz | 2 |  |
| `ndp.en1991.2.footbridge-band-jogging-min` | lower bound of the jogging excitation band | 2 Hz | 2 Hz | 2 |  |

### EN 1991-2 · Table 6.11

Printed source: DIN EN 1991-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e2-07.4` | fem | awaiting | awaiting | 0 |  |

### EN 1991-3 · 1.0

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e3-01.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e3-01.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · 1.05

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure 1.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.l653.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · 2.7

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 2.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.3.phi-5` | phi_5 | 1.5 | 1.5 | 1 |  |

### EN 1991-3 · 5.3.2

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.3.drive-friction-mu` | mu | 0.2 | 0.2 | 1 |  |

### EN 1991-3 · E3.02

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure E3.02.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-02.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · E3.03

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure E3.03.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-03.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e3-03.3` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · E3.04

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure E3.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · E3.05

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure E3.05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · E3.07

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure E3.07.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-07.1` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · Table 2.

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure Table 2..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-03.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e3-07.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-3 · Table 2.x

Printed source: DIN EN 1991-3/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.x.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e3-01.4` | ? | awaiting | awaiting | 2 |  |

### EN 1991-4 · 2.10

Printed source: DIN EN 1991-4/NA ed. to be recorded at verification. Verify: printed table or figure 2.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e4-01.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e4-01.4` | ? | awaiting | awaiting | 0 |  |

### EN 1991-4 · 5.2.4

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure 5.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.4.patch-factor` | p_pf / p_hf | 0.2 | 0.2 | 1 |  |

### EN 1991-4 · E4.01

Printed source: DIN EN 1991-4/NA ed. to be recorded at verification. Verify: printed table or figure E4.01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e4-01.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e4-01.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-4 · E4.04

Printed source: DIN EN 1991-4/NA ed. to be recorded at verification. Verify: printed table or figure E4.04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e4-04.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1991.e4-04.2` | ? | awaiting | awaiting | 0 |  |

### EN 1991-4 · EN 1991-4 2.2 Table 2.1

Printed source: DIN EN 1991/NA ed. to be recorded at verification. Verify: printed table or figure EN 1991-4 2.2 Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.4.aac-class1-below` | silo capacity below which the action assessment class is 1 | 100 t | 100 t | 2 |  |
| `ndp.en1991.4.aac-class3-above` | silo capacity above which the action assessment class is 3 | 10000 t | 10000 t | 2 |  |

### EN 1991-4 · §5.2.4 / §6

Printed source: DIN EN 1991-4/NA ed. to be recorded at verification. Verify: printed table or figure §5.2.4 / §6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1991.e4-02.1` | ? | awaiting | awaiting | 2 |  |

## EN 1992 (208 unverified)

### EN 1992 · 1.0

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.b-04.1` | <= 50% | awaiting | awaiting | 1 |  |
| `ndp.en1992.sl-04.1` | fctm,min = 2.9 | awaiting | awaiting | 0 |  |
| `ndp.en1992.t-04.1` | . Box sections:  | awaiting | awaiting | 2 |  |

### EN 1992 · 1.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.e-03.1` | nu1 = 0.75*nu2 = 0.75 | awaiting | awaiting | 1 |  |
| `ndp.en1992.p-02.1` | beta = 1.1 | awaiting | awaiting | 0 |  |

### EN 1992 · 1.4

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-03.3` | [V] | awaiting | awaiting | 2 |  |

### EN 1992 · 1.5

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.e-05.1` | CRd,c = 0.18/1.5 = 0.12 | awaiting | awaiting | 0 |  |
| `ndp.en1992.e-05.2` | [V] | awaiting | awaiting | 0 |  |
| `ndp.en1992.p-04.1` | [V] | awaiting | awaiting | 3 |  |
| `ndp.en1992.sl-07.1` |  ( | awaiting | awaiting | 1 |  |
| `ndp.en1992.v-02.1` | 0.035*k^1.5*sqrt(fck) | awaiting | awaiting | 1 |  |

### EN 1992 · 12.7

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 12.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.e-08.1` |  (DE  | awaiting | awaiting | 0 |  |

### EN 1992 · 2.0

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 2.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-01.1` | uout | awaiting | awaiting | 0 |  |
| `ndp.en1992.p-03.1` |  and  | awaiting | awaiting | 1 |  |

### EN 1992 · 2.12

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 2.12.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pr-04.1` | ;  | awaiting | awaiting | 0 |  |

### EN 1992 · 2.2

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.d-02.1` | XC1 C16/20 | awaiting | awaiting | 1 |  |
| `ndp.en1992.d-03.1` | Dc,dur,gamma = 0 | awaiting | awaiting | 2 |  |

### EN 1992 · 2.5

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.v-03.1` | 1.2 | awaiting | awaiting | 1 |  |
| `ndp.en1992.v-05.1` | c | awaiting | awaiting | 0 |  |

### EN 1992 · 2.7

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 2.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.e17-02.5` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1992.e17-02.6` | ? | awaiting | awaiting | 0 |  |

### EN 1992 · 3.0

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 3.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pa-01.1` | alpha_cc = 0.85 | awaiting | awaiting | 0 |  |

### EN 1992 · 3.1.7(3)

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.7(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.m-07.1` | 0.9 | awaiting | awaiting | 0 |  |

### EN 1992 · 3.3

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.m-09.1` | sigma_p = fp0.1k/gamma_s | awaiting | awaiting | 0 |  |

### EN 1992 · 4.4.1.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.d-04.1` | [V] | awaiting | awaiting | 2 |  |

### EN 1992 · 5.10.2.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pr-01.1` | Pmax = Ap*sigma_p,max | awaiting | awaiting | 1 |  |

### EN 1992 · 5.10.8

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pr-06.1` | Ap | awaiting | awaiting | 0 |  |

### EN 1992 · 5.10.9

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pr-07.1` | 1.10/0.90 | awaiting | awaiting | 0 |  |

### EN 1992 · 5.5

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.a-06.1` | xu/d <= 0.45 | awaiting | awaiting | 0 |  |
| `ndp.en1992.a-06.2` | delta >= 0.64 + 0.8*xu/d | awaiting | awaiting | 0 |  |

### EN 1992 · 5.7

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.a-08.1` | gamma_R | awaiting | awaiting | 0 |  |

### EN 1992 · 5.8.3.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.8.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.a-09.1` | lambda_lim = 25 | awaiting | awaiting | 1 |  |

### EN 1992 · 5.8.6

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 5.8.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.a-14.1` | ;  | awaiting | awaiting | 0 |  |

### EN 1992 · 6.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.s-09.1` | hw >= 120 mm | awaiting | awaiting | 0 |  |

### EN 1992 · 6.2.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.v-01.1` | av/d >= 0.5 | awaiting | awaiting | 1 |  |

### EN 1992 · 6.2.3(103)

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.3(103).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.v-04.1` | alpha_cw = 1.0 | awaiting | awaiting | 0 |  |

### EN 1992 · 6.4.4(1)

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.4(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-03.2` | CRd,c | 0.1 | 0.12 | 1 |  |

### EN 1992 · 6.4.5(4)

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.5(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-05.1` | vRd,max | awaiting | awaiting | 1 |  |

### EN 1992 · 6.5.4

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.5.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.st-03.1` | k1 | awaiting | awaiting | 0 |  |

### EN 1992 · 6.8.4

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 6.8.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.f-03.1` | Delta sigma_Rsk = 162.5 | awaiting | awaiting | 1 |  |

### EN 1992 · 7.2(5)

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 7.2(5).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pr-05.1` | Pm,t(x,t) | awaiting | awaiting | 2 |  |
| `ndp.en1992.sl-02.1` | 0.65 fpk | awaiting | awaiting | 5 |  |

### EN 1992 · 7.3.3

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.sl-05.1` | wk = 0.2 | awaiting | awaiting | 0 |  |
| `ndp.en1992.sl-05.2` | wk = 0.2 | awaiting | awaiting | 0 |  |

### EN 1992 · 8.4.2

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 8.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.b-01.1` | [V] | awaiting | awaiting | 1 |  |

### EN 1992 · 9.10

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 9.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.dt-14.1` |  or DE ( | awaiting | awaiting | 0 |  |

### EN 1992 · 9.2.2

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 9.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.v-03.2` |  (DE:  | awaiting | awaiting | 1 |  |

### EN 1992 · 9.3.1

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 9.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.dt-06.1` | As,min | awaiting | awaiting | 0 |  |

### EN 1992 · 9.6

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure 9.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.dt-10.1` | As,v,min = 0.002*Ac | awaiting | awaiting | 0 |  |
| `ndp.en1992.dt-10.2` | h >= 100 mm | awaiting | awaiting | 0 |  |

### EN 1992 · Table 4.4N

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure Table 4.4N.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.d-03.2` | cmin,dur | awaiting | awaiting | 2 |  |

### EN 1992 · Table 5.5

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.e-12.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1992 · Table 7.1DE

Printed source: DIN EN 1992/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.1DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.sl-06.1` | 0.4 | awaiting | awaiting | 1 |  |

### EN 1992-1-1 · 12.3.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 12.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.alpha-cc-pl` | alpha_cc_pl | 0.8 | 0.7 | 2 |  |

### EN 1992-1-1 · 2.4.2.4

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.4.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-c-fat` | gamma_c_fat | 1.5 | 1.5 | 0 |  |
| `ndp.en1992.gamma-s` | gamma_s | 1.15 | 1.15 | 41 |  |
| `ndp.en1992.gamma-s-acc` | gamma_s_acc | 1 | 1 | 6 |  |
| `ndp.en1992.gamma-s-fat` | gamma_s_fat | 1.15 | 1.15 | 10 |  |
| `ndp.en1992.m-01.gamma-c-accidental` | gamma_c_acc | 1.2 | 1.3 | 4 |  |
| `ndp.en1992.m-01.gamma-c-persistent` | gamma_c | 1.5 | 1.5 | 62 |  |

### EN 1992-1-1 · 3.1(4)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.1(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-05.2` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · 3.1.6

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.bridge-alpha-cc-lt` | bridge_alpha_cc_lt | 1 | 0.8 | 3 |  |

### EN 1992-1-1 · 3.1.6(1)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.6(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.m-01.alpha-cc-accidental` | alpha_cc_acc | 1 | 0.85 | 5 |  |
| `ndp.en1992.m-01.alpha-cc-persistent` | alpha_cc | 1 | 0.85 | 38 |  |

### EN 1992-1-1 · 3.1.6(2)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.6(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.m-01.alpha-ct` | alpha_ct | 1 | 0.85 | 15 |  |

### EN 1992-1-1 · 4.4.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-inst` | gamma_inst | 1 | 1 | 10 |  |
| `ndp.en1992.liquid-cover-add` | liquid_cover_add | 0 | 5 | 2 |  |

### EN 1992-1-1 · 4.4.1.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.bridge-cover-chloride` | bridge_cover_chloride | 0 | 10 | 2 |  |
| `ndp.en1992.cmin-dur-table` | cmin_dur_table | 0 | 0 | 5 |  |

### EN 1992-1-1 · 4.4.1.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.delta-c-dev` | delta_c_dev | 10 | 15 | 3 |  |

### EN 1992-1-1 · 4.5.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.spalling-add-mm` | spalling_add_mm | 10 | 10 | 5 |  |

### EN 1992-1-1 · 5.10.2.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-k1` | p_k1 | 0.8 | 0.8 | 4 |  |
| `ndp.en1992.p-k2` | p_k2 | 0.9 | 0.9 | 4 |  |

### EN 1992-1-1 · 5.10.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-k7` | p_k7 | 0.75 | 0.75 | 5 |  |
| `ndp.en1992.p-k8` | p_k8 | 0.85 | 0.85 | 5 |  |

### EN 1992-1-1 · 5.10.5.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.friction-k` | friction_k | 0.005 | 0.005 | 2 |  |
| `ndp.en1992.friction-mu` | friction_mu | 0.19 | 0.19 | 2 |  |

### EN 1992-1-1 · 5.10.8

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.unbonded-dsigma` | unbonded_dsigma | 100 | 100 | 2 |  |

### EN 1992-1-1 · 5.10.9

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.10.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.p-r-inf` | p_r_inf | 0.95 | 0.95 | 2 |  |
| `ndp.en1992.p-r-inf-unbonded` | p_r_inf_unbonded | 0.9 | 0.9 | 2 |  |
| `ndp.en1992.p-r-sup` | p_r_sup | 1.05 | 1.05 | 2 |  |
| `ndp.en1992.p-r-sup-unbonded` | p_r_sup_unbonded | 1.1 | 1.1 | 2 |  |

### EN 1992-1-1 · 5.5(4)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.5(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.redistribution-k1` | redistribution_k1 | 0.44 | 0.64 | 2 |  |
| `ndp.en1992.redistribution-k2` | redistribution_k2 | 1.25 | 0.8 | 2 |  |

### EN 1992-1-1 · 5.6.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.plastic-rotation-model` | plastic_rotation_model | 0 | 0 | 1 |  |

### EN 1992-1-1 · 5.7

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-o-nl` | gamma_o_nl | 1.2 | 1.2 | 2 |  |

### EN 1992-1-1 · 5.8.3.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.8.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lambda-lim-de-n` | lambda_lim_de_n | 0 | 0.41 | 4 |  |

### EN 1992-1-1 · 5.8.3.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.8.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.k1-global` | k1_global | 0.31 | 0.31 | 2 |  |

### EN 1992-1-1 · 5.8.6

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.8.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-ce` | gamma_cE | 1.2 | 1.2 | 2 |  |

### EN 1992-1-1 · 6.2.1.4(6)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.4(6).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-10.1` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · 6.2.2(1)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.2(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.crd-c-shear` | crd_c_shear | 0.18 | 0.15 | 6 |  |
| `ndp.en1992.k1-shear` | k1_shear | 0.15 | 0.12 | 5 |  |
| `ndp.en1992.vmin-shear` | vmin_shear | 0.035 | 0.0525 | 12 |  |
| `ndp.en1992.vmin-shear-d800` | vmin_shear_d800 | 0.035 | 0.0375 | 11 |  |

### EN 1992-1-1 · 6.2.3(103)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.3(103).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.alpha-cw` | alpha_cw | 1 | 1 | 5 |  |
| `ndp.en1992.nu1-factor` | nu1_factor | 1 | 0.75 | 6 |  |

### EN 1992-1-1 · 6.2.3(2)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.3(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.cot-theta-a` | cot_theta_a | 2.5 | 1.2 | 3 |  |
| `ndp.en1992.cot-theta-b` | cot_theta_b | 0 | 1.4 | 3 |  |
| `ndp.en1992.cot-theta-max` | cot_theta_max | 2.5 | 3 | 6 |  |
| `ndp.en1992.vrdcc-c` | vrdcc_c | 0 | 0.5 | 3 |  |

### EN 1992-1-1 · 6.4.3(6)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.3(6).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.beta-corner` | beta_corner | 1.5 | 1.5 | 5 |  |
| `ndp.en1992.beta-edge` | beta_edge | 1.4 | 1.4 | 5 |  |
| `ndp.en1992.beta-interior` | beta_interior | 1.15 | 1.1 | 7 |  |
| `ndp.en1992.beta-wall-end` | beta_wall_end | 1.35 | 1.35 | 3 |  |

### EN 1992-1-1 · 6.4.4(1)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.4(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.crd-c-punch` | crd_c_punch | 0.18 | 0.18 | 7 |  |
| `ndp.en1992.k1-punch` | k1_punch | 0.1 | 0.1 | 7 |  |

### EN 1992-1-1 · 6.4.5(3)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.5(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.punch-vrdmax-factor` | punch_vrdmax_factor | 0 | 1.4 | 8 |  |

### EN 1992-1-1 · 6.4.5(4)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.5(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.punch-k-out` | punch_k_out | 1.5 | 1.5 | 4 |  |

### EN 1992-1-1 · 6.5.2(2)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.5.2(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.strut-cracked` | strut_cracked | 0.6 | 0.75 | 4 |  |

### EN 1992-1-1 · 6.5.4(4)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.5.4(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.node-k1` | node_k1 | 1 | 1.1 | 2 |  |
| `ndp.en1992.node-k2` | node_k2 | 0.85 | 0.75 | 2 |  |
| `ndp.en1992.node-k3` | node_k3 | 0.75 | 0.75 | 2 |  |

### EN 1992-1-1 · 6.6.6.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-19.1` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · 6.8.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.8.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-21.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1992.c-21.2` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · 6.8.4

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.8.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fatigue-coupler` | fatigue_coupler | 35 | 35 | 3 |  |
| `ndp.en1992.fatigue-straight` | fatigue_straight | 162.5 | 162.5 | 7 |  |
| `ndp.en1992.fatigue-welded` | fatigue_welded | 58.5 | 58.5 | 3 |  |

### EN 1992-1-1 · 7.2(2)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.sigma-c-char` | sigma_c_char | 0.6 | 0.6 | 3 |  |

### EN 1992-1-1 · 7.2(3)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.sigma-c-qp` | sigma_c_qp | 0.45 | 0.45 | 3 |  |

### EN 1992-1-1 · 7.2(5)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2(5).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.sigma-p-qp` | sigma_p_qp | 0.75 | 0.65 | 4 |  |
| `ndp.en1992.sigma-s-char` | sigma_s_char | 0.8 | 0.8 | 3 |  |
| `ndp.en1992.sigma-s-imposed` | sigma_s_imposed | 1 | 1 | 2 |  |

### EN 1992-1-1 · 7.2.1.4

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.cone-k-cast-cracked` | cone_k_cast_cracked | 8.9 | 8.9 | 7 |  |
| `ndp.en1992.cone-k-cast-uncracked` | cone_k_cast_uncracked | 12.7 | 12.7 | 7 |  |
| `ndp.en1992.cone-k-post-cracked` | cone_k_post_cracked | 7.7 | 7.7 | 5 |  |
| `ndp.en1992.cone-k-post-uncracked` | cone_k_post_uncracked | 11 | 11 | 5 |  |
| `ndp.en1992.psi-m-n` | psi_m_n | 2 | 2 | 1 |  |

### EN 1992-1-1 · 7.2.1.5

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.pullout-k2-cracked` | pullout_k2_cracked | 7.5 | 7.5 | 3 |  |
| `ndp.en1992.pullout-k2-uncracked` | pullout_k2_uncracked | 10.5 | 10.5 | 3 |  |

### EN 1992-1-1 · 7.2.2.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.anchor-shear-k` | anchor_shear_k | 0.5 | 0.5 | 1 |  |

### EN 1992-1-1 · 7.2.2.5

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.psi-re-edge` | psi_re_edge | 1 | 1 | 0 |  |

### EN 1992-1-1 · 7.3.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.wk-lim` | wk_lim | 0.3 | 0.3 | 5 |  |
| `ndp.en1992.wk-lim-prestress` | wk_lim_prestress | 0.2 | 0.2 | 3 |  |
| `ndp.en1992.wk-lim-xc1` | wk_lim_xc1 | 0.4 | 0.4 | 4 |  |

### EN 1992-1-1 · 7.3.1(110)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.1(110).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.tightness-wk-high` | tightness_wk_high | 0.2 | 0.2 | 5 |  |
| `ndp.en1992.tightness-wk-low` | tightness_wk_low | 0.05 | 0.05 | 4 |  |

### EN 1992-1-1 · 7.3.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.bridge-fct-min` | bridge_fct_min | 3 | 3 | 2 |  |
| `ndp.en1992.fct-eff-factor-slab` | fct_eff_factor_slab | 1 | 0.5 | 2 |  |

### EN 1992-1-1 · 7.3.4

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.k3-crack` | k3_crack | 3.4 | 3.4 | 4 |  |
| `ndp.en1992.k4-crack` | k4_crack | 0.425 | 0.425 | 4 |  |
| `ndp.en1992.kt-long` | kt_long | 0.4 | 0.4 | 3 |  |
| `ndp.en1992.kt-short` | kt_short | 0.6 | 0.6 | 2 |  |

### EN 1992-1-1 · 7.4

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.tank-sigma-min` | tank_sigma_min | 1 | 1 | 2 |  |

### EN 1992-1-1 · 7.4.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.ld-de-cap` | ld_de_cap | 0 | 35 | 3 |  |

### EN 1992-1-1 · 7.4.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-43.1` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · 8.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 8.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.k-mandrel-large` | k_mandrel_large | 7 | 7 | 2 |  |
| `ndp.en1992.k-mandrel-small` | k_mandrel_small | 4 | 4 | 2 |  |

### EN 1992-1-1 · 8.4.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 8.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fbd-eta` | fbd_eta | 2.25 | 2.25 | 5 |  |

### EN 1992-1-1 · 9.10.2.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.10.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.tie-q1` | tie_q1 | 10 | 10 | 2 |  |
| `ndp.en1992.tie-q2` | tie_q2 | 70 | 70 | 2 |  |

### EN 1992-1-1 · 9.10.2.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.10.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.tie-internal` | tie_internal | 20 | 20 | 2 |  |

### EN 1992-1-1 · 9.10.2.5

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.10.2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.tie-column` | tie_column | 150 | 150 | 2 |  |

### EN 1992-1-1 · 9.3.1.1(3)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.3.1.1(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.slab-smax-cracked` | slab_smax_cracked | 400 | 250 | 2 |  |

### EN 1992-1-1 · 9.5.2(2)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.5.2(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.col-asmin-n` | col_asmin_n | 0.1 | 0.15 | 2 |  |

### EN 1992-1-1 · 9.7.6

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 9.7.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.c-38.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1992.c-38.2` | ? | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · ?

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.l642.1` | [V] | awaiting | awaiting | 0 |  |
| `ndp.en1992.l642.2` | [V] | awaiting | awaiting | 0 |  |

### EN 1992-1-1 · Annex J.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Annex J.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.surface-reinf` | surface_reinf | 0.01 | 0.01 | 2 |  |
| `ndp.en1992.surface-smax` | surface_smax | 150 | 150 | 2 |  |

### EN 1992-1-1 · Annex J.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Annex J.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.corner-links-share` | corner_links_share | 0.5 | 0.5 | 2 |  |
| `ndp.en1992.corner-rho-open` | corner_rho_open | 0.02 | 0.02 | 2 |  |

### EN 1992-1-1 · Annex J.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Annex J.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.corbel-k1` | corbel_k1 | 0.25 | 0.25 | 2 |  |

### EN 1992-1-1 · DIN EN 1992-1-1/NA

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure DIN EN 1992-1-1/NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.na-de-edition` | edition year | 0 year | 2013 year | 5 |  |

### EN 1992-1-1 · EN 1992-1-1 9.5.3(3) / NA-DE

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-1 9.5.3(3) / NA-DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.col-link-spacing-de` | k_s,cl | 20 | 12 | 4 |  |
| `ndp.en1992.col-link-spacing-max` | s_cl,t,max | 400 mm | 400 mm | 2 |  |

### EN 1992-1-1 · EN 1992-1-1 9.5.3(4)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-1 9.5.3(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.col-link-near-factor` | k_near | 0.6 | 0.6 | 2 |  |

### EN 1992-1-1 · EN 1992-1-1 Table 7.4N note 2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-1 Table 7.4N note 2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.ld-flange-factor` | k_flange | 0.8 | 0.8 | 1 |  |
| `ndp.en1992.ld-flange-ratio` | b_f/b_w | 3 | 3 | 1 |  |

### EN 1992-1-1 · EN 1992-1-2 3.2-3.3

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-2 3.2-3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fire-ks-kc` | fire_ks_kc | 0 | 0 | 7 |  |

### EN 1992-1-1 · EN 1992-1-2 5

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-2 5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fire-tables` | fire_tables | 0 | 0 | 2 |  |

### EN 1992-1-1 · EN 1992-2 Annex KK

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-2 Annex KK.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.kk-ageing-chi` | kk_ageing_chi | 0.8 | 0.8 | 5 |  |

### EN 1992-1-1 · EN 1992-4 5

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-4 5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-ms-fat` | gamma_ms_fat | 1.35 | 1.35 | 2 |  |

### EN 1992-1-1 · EN 1992-4 7.2.1.8

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-4 7.2.1.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.gamma-ms-re` | gamma_ms_re | 1.15 | 1.15 | 2 |  |

### EN 1992-1-1 · Table 3.1

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.t3-1.ecm.c12-15` | E_cm C12/15 | 27 GPa | 27 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c16-20` | E_cm C16/20 | 29 GPa | 29 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c20-25` | E_cm C20/25 | 30 GPa | 30 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c25-30` | E_cm C25/30 | 31 GPa | 31 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c30-37` | E_cm C30/37 | 33 GPa | 33 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c35-45` | E_cm C35/45 | 34 GPa | 34 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c40-50` | E_cm C40/50 | 35 GPa | 35 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c45-55` | E_cm C45/55 | 36 GPa | 36 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c50-60` | E_cm C50/60 | 37 GPa | 37 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c55-67` | E_cm C55/67 | 38 GPa | 38 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c60-75` | E_cm C60/75 | 39 GPa | 39 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c70-85` | E_cm C70/85 | 41 GPa | 41 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c80-95` | E_cm C80/95 | 42 GPa | 42 GPa | 0 |  |
| `ndp.en1992.t3-1.ecm.c90-105` | E_cm C90/105 | 44 GPa | 44 GPa | 0 |  |

### EN 1992-1-1 · Table 7.105

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.105.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-01.1` | hD/h <= 5: 0.20 | awaiting | awaiting | 1 |  |

### EN 1992-1-2 · 5.1

Printed source: DIN EN 1992-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.r-01.1` | fck > C80/95 | awaiting | awaiting | 2 |  |

### EN 1992-1-2 · EN 1990 6.5.3 / EN 1992-1-2 2.4.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 6.5.3 / EN 1992-1-2 2.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.psi-fire-lead` | psi_fi selector | 1 | 1 | 1 |  |

### EN 1992-1-2 · EN 1992-1-2 B.2

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-2 B.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fire-zone-exponent` | e_az | 1 | 1 | 4 |  |
| `ndp.en1992.fire-zones` | n | 5 | 5 | 2 |  |

### EN 1992-1-2 · Table 3.1

Printed source: DIN EN 1992-1-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.r-07.1` | kc(theta) | awaiting | awaiting | 0 |  |

### EN 1992-1-2 · Table 3.2a

Printed source: DIN EN 1992-1-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.2a.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.r-03.1` | ks(theta) | awaiting | awaiting | 0 |  |

### EN 1992-2 · 3.1.2

Printed source: DIN EN 1992-2/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.br-01.1` | alpha_cc = 0.85 | awaiting | awaiting | 0 |  |

### EN 1992-2 · 6.2.2

Printed source: DIN EN 1992-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.br-03.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1992-2 · 7.3

Printed source: DIN EN 1992-2/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.br-05.1` | wk,lim | awaiting | awaiting | 0 |  |

### EN 1992-2 · Table 4.4N

Printed source: DIN EN 1992-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 4.4N.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.br-02.1` | cmin,dur | awaiting | awaiting | 0 |  |

### EN 1992-3 · 4.4

Printed source: DIN EN 1992-3/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-05.1` | cmin | awaiting | awaiting | 0 |  |

### EN 1992-3 · 7.4

Printed source: DIN EN 1992-3/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-06.1` | p = n*P/(R*...) | awaiting | awaiting | 0 |  |

### EN 1992-3 · EN 1992-3 7.3.1(105)

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-3 7.3.1(105).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-04-head-ratio` | h_D/h | 5 | 5 | 3 |  |
| `ndp.en1992.lq-04-wk-selfhealing` | w_k,lim | 0.2 mm | 0.2 mm | 2 |  |

### EN 1992-3 · EN 1992-3 Table 7.105DE

Printed source: DIN EN 1992-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-3 Table 7.105DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-04-wk-above` | w_k,lim | 0.1 mm | 0.1 mm | 2 |  |

### EN 1992-3 · Table 7.105DE

Printed source: DIN EN 1992-3/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.105DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.lq-04.1` | wk,lim | awaiting | awaiting | 0 |  |

### EN 1992-4 · 1.5

Printed source: DIN EN 1992-4/NA ed. to be recorded at verification. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fa-03.1` |  [N, fck MPa, hef mm];  | awaiting | awaiting | 3 |  |

### EN 1992-4 · 7.2.1.4

Printed source: DIN EN 1992-4/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1992.fa-04.1` | NRk,p = psi_c*A_h*8*fck | awaiting | awaiting | 1 |  |

## EN 1993 (329 unverified)

### EN 1993 · 1.0

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l396.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.v-802.1` | σx,Rcr = 0.605 E Cx t/r | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-880.2` | gM0 | awaiting | awaiting | 1 |  |

### EN 1993 · 1.00

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 1.00.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l401.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 1.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l398.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 1.5

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l403.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.v-860.1` | FEd <= FRd = Fuk/γR | awaiting | awaiting | 1 |  |

### EN 1993 · 10.5

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 10.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-372.1` | ρ_x | awaiting | awaiting | 1 |  |

### EN 1993 · 3.2.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-250.1` | ,  | awaiting | awaiting | 1 |  |

### EN 1993 · 3.2.6

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-1.g-modulus` | G | 81000 MPa | 81000 MPa | 2 |  |

### EN 1993 · 3.25

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 3.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-890.1` | σoz,Ed = Fz,Ed/(leff tw) | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-890.2` | σoz,Ed = Fz,Ed/(leff tw) | awaiting | awaiting | 1 |  |

### EN 1993 · 4.4

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 4.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-362.1` | σcr,p = kσ,p σE | awaiting | awaiting | 1 |  |

### EN 1993 · 5.3.2(3)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.2(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l383.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 5.3.4

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-026.1` | ? | awaiting | awaiting | 1 |  |

### EN 1993 · 5.3.4(3)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.4(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l384.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 5.5

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 5.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-028.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.v-369.1` | A_end >= ... | awaiting | awaiting | 1 |  |

### EN 1993 · 5.5.2

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-4.ltb.other.alpha` | alpha ltb other | 0.34 | 0.34 | 0 |  |
| `ndp.en1993.1-4.ltb.other.lambda0` | lambda_0 ltb other | 0.4 | 0.4 | 0 |  |
| `ndp.en1993.1-4.ltb.welded-open.alpha` | alpha ltb welded-open | 0.76 | 0.76 | 0 |  |
| `ndp.en1993.1-4.ltb.welded-open.lambda0` | lambda_0 ltb welded-open | 0.2 | 0.2 | 0 |  |
| `ndp.en1993.1-4.torsional.all.alpha` | alpha torsional all | 0.34 | 0.34 | 0 |  |
| `ndp.en1993.1-4.torsional.all.lambda0` | lambda_0 torsional all | 0.2 | 0.2 | 0 |  |

### EN 1993 · 6.1.7.2 Table 6.? values

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.7.2 Table 6.? values.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-3.web-crippling.cat1.alpha` | alpha category 1 | 0.075 | 0.075 | 1 |  |
| `ndp.en1993.1-3.web-crippling.cat1.la` | l_a category 1 | 10 | 10 | 1 |  |
| `ndp.en1993.1-3.web-crippling.cat2.alpha` | alpha category 2 | 0.15 | 0.15 | 1 |  |

### EN 1993 · 6.2.4

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-420.1` | FT,1,Rd = 4 Mpl,1,Rd/m | awaiting | awaiting | 2 |  |

### EN 1993 · 6.2.9.1

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.9.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-068.1` | MEd <= MN,Rd | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-113a.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.1

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-431.2` |  per Table 6.11  | awaiting | awaiting | 1 |  |

### EN 1993 · 6.3.1.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l385.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.1.4

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-850.1` | λ̄eff = 0.9 + ... | awaiting | awaiting | 1 |  |

### EN 1993 · 6.3.2.3

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-111.1` | lambda_bar_LT,0 = 0.4 | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-111.2` | lambda_bar_LT,0 = 0.4 | awaiting | awaiting | 1 |  |

### EN 1993 · 6.3.2.3(1)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.3(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l387.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.2.4(1)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.4(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l390.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.3

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.na-de.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.3(5)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.3(5).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-121.1` | NDP 6.3.3(5) | awaiting | awaiting | 0 |  |

### EN 1993 · 6.3.5

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-124.1` | Ls = (C_n 600 + 60 ...) | awaiting | awaiting | 0 |  |

### EN 1993 · 6.4.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-131.1` | ,  | awaiting | awaiting | 0 |  |

### EN 1993 · 7.1 / Figure 7.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.1 / Figure 7.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-9.sn.m-1` | m1 | 3 | 3 | 0 |  |
| `ndp.en1993.1-9.sn.m-2` | m2 | 5 | 5 | 0 |  |
| `ndp.en1993.1-9.sn.n-c` | N_C | 2e+06 | 2e+06 | 0 |  |
| `ndp.en1993.1-9.sn.n-d` | N_D | 5e+06 | 5e+06 | 0 |  |
| `ndp.en1993.1-9.sn.n-l` | N_L | 1e+08 | 1e+08 | 0 |  |

### EN 1993 · 7.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-371.1` | η2 + 0.8 η1 <= 1.4 | awaiting | awaiting | 1 |  |

### EN 1993 · 7.2.1(1)

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.1(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l393.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993 · 8.5

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 8.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-801.1` | λ̄ = sqrt(Rpl/Rcr) | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-801.2` | λ̄ = sqrt(Rpl/Rcr) | awaiting | awaiting | 2 |  |

### EN 1993 · 8.7

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure 8.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-803.1` | nx = μ·pn ...  | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-803.2` | nx = μ·pn ...  | awaiting | awaiting | 1 |  |

### EN 1993 · ?

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l394.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.l5.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.v-255.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-256.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-257.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-305.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-800.1` | ? | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-851.1` | Vcrit = b fn/St | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-851.2` | Vcrit = b fn/St | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-905.1` | t = p d/(2 fy/gM0) | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-905.2` | t = p d/(2 fy/gM0) | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-905.3` | t = p d/(2 fy/gM0) | awaiting | awaiting | 1 |  |

### EN 1993 · EN 1990 Table A1.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 Table A1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.psi-self.0` | ψ0 permanent | 1 | 1 | 0 |  |
| `ndp.en1993.psi-self.1` | ψ1 permanent | 1 | 1 | 0 |  |
| `ndp.en1993.psi-self.2` | ψ2 permanent | 1 | 1 | 0 |  |

### EN 1993 · Table 2.1

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l402.1` | ? | awaiting | awaiting | 1 |  |

### EN 1993 · Table 2.1 / Annex C

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.1 / Annex C.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-4.grade.1-4003.e` | e grade 1.4003 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4003.fu` | fu grade 1.4003 | 450 MPa | 450 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4003.fy` | fy grade 1.4003 | 250 MPa | 250 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4003.n` | n grade 1.4003 | 14 | 14 | 0 |  |
| `ndp.en1993.1-4.grade.1-4301.e` | e grade 1.4301 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4301.fu` | fu grade 1.4301 | 520 MPa | 520 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4301.fy` | fy grade 1.4301 | 210 MPa | 210 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4301.n` | n grade 1.4301 | 6 | 6 | 0 |  |
| `ndp.en1993.1-4.grade.1-4401.e` | e grade 1.4401 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4401.fu` | fu grade 1.4401 | 520 MPa | 520 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4401.fy` | fy grade 1.4401 | 220 MPa | 220 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4401.n` | n grade 1.4401 | 7 | 7 | 0 |  |
| `ndp.en1993.1-4.grade.1-4404.e` | e grade 1.4404 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4404.fu` | fu grade 1.4404 | 520 MPa | 520 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4404.fy` | fy grade 1.4404 | 220 MPa | 220 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4404.n` | n grade 1.4404 | 7 | 7 | 0 |  |
| `ndp.en1993.1-4.grade.1-4462.e` | e grade 1.4462 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4462.fu` | fu grade 1.4462 | 640 MPa | 640 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4462.fy` | fy grade 1.4462 | 460 MPa | 460 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4462.n` | n grade 1.4462 | 5 | 5 | 0 |  |
| `ndp.en1993.1-4.grade.1-4571.e` | e grade 1.4571 | 200000 MPa | 200000 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4571.fu` | fu grade 1.4571 | 520 MPa | 520 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4571.fy` | fy grade 1.4571 | 220 MPa | 220 MPa | 0 |  |
| `ndp.en1993.1-4.grade.1-4571.n` | n grade 1.4571 | 7 | 7 | 0 |  |

### EN 1993 · Table 5.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-870.1` | βD | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-870.2` | βD | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-870.3` | βD | awaiting | awaiting | 1 |  |

### EN 1993 · Table 5.3

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-4.rho-internal.a` | rho internal element a | 0.772 | 0.772 | 1 |  |
| `ndp.en1993.1-4.rho-internal.b.cold` | rho internal element b cold-formed | 0.125 | 0.125 | 1 |  |
| `ndp.en1993.1-4.rho-internal.b.welded` | rho internal element b welded | 0.079 | 0.079 | 1 |  |
| `ndp.en1993.1-4.rho-outstand.a` | rho outstand element a | 1 | 1 | 1 |  |
| `ndp.en1993.1-4.rho-outstand.b.cold` | rho outstand element b cold-formed | 0.231 | 0.231 | 1 |  |
| `ndp.en1993.1-4.rho-outstand.b.welded` | rho outstand element b welded | 0.242 | 0.242 | 1 |  |

### EN 1993 · Table 5.4

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-4.flexural.other.alpha` | alpha flexural other | 0.49 | 0.49 | 0 |  |
| `ndp.en1993.1-4.flexural.other.lambda0` | lambda_0 flexural other | 0.4 | 0.4 | 0 |  |
| `ndp.en1993.1-4.flexural.welded-open.alpha` | alpha flexural welded-open | 0.49 | 0.49 | 0 |  |
| `ndp.en1993.1-4.flexural.welded-open.lambda0` | lambda_0 flexural welded-open | 0.2 | 0.2 | 0 |  |

### EN 1993 · Table 6.2

Printed source: DIN EN 1993/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-100.1` | Ncr = pi^2 E I / Lcr^2 | awaiting | awaiting | 0 |  |

### EN 1993 · Table 8.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 8.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-9.category.c100` | detail category 100 | 100 MPa | 100 MPa | 0 |  |
| `ndp.en1993.1-9.category.c112` | detail category 112 | 112 MPa | 112 MPa | 0 |  |
| `ndp.en1993.1-9.category.c125` | detail category 125 | 125 MPa | 125 MPa | 0 |  |
| `ndp.en1993.1-9.category.c140` | detail category 140 | 140 MPa | 140 MPa | 0 |  |
| `ndp.en1993.1-9.category.c160` | detail category 160 | 160 MPa | 160 MPa | 0 |  |
| `ndp.en1993.1-9.category.c36` | detail category 36 | 36 MPa | 36 MPa | 0 |  |
| `ndp.en1993.1-9.category.c40` | detail category 40 | 40 MPa | 40 MPa | 0 |  |
| `ndp.en1993.1-9.category.c45` | detail category 45 | 45 MPa | 45 MPa | 0 |  |
| `ndp.en1993.1-9.category.c50` | detail category 50 | 50 MPa | 50 MPa | 0 |  |
| `ndp.en1993.1-9.category.c56` | detail category 56 | 56 MPa | 56 MPa | 0 |  |
| `ndp.en1993.1-9.category.c63` | detail category 63 | 63 MPa | 63 MPa | 0 |  |
| `ndp.en1993.1-9.category.c71` | detail category 71 | 71 MPa | 71 MPa | 0 |  |
| `ndp.en1993.1-9.category.c80` | detail category 80 | 80 MPa | 80 MPa | 0 |  |
| `ndp.en1993.1-9.category.c90` | detail category 90 | 90 MPa | 90 MPa | 0 |  |

### EN 1993 · Table 8.1 notes

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 8.1 notes.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-9.size-effect.exponent` | size effect exponent | 0.2 | 0.2 | 1 |  |
| `ndp.en1993.1-9.size-effect.t-ref` | reference thickness | 25 mm | 25 mm | 1 |  |

### EN 1993 · Table 8.8

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 8.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-9.deck.rib-splice-butt-weld` | orthotropic deck detail rib-splice-butt-weld | 90 MPa | 90 MPa | 0 |  |
| `ndp.en1993.1-9.deck.rib-to-crossbeam-web-cutout` | orthotropic deck detail rib-to-crossbeam-web-cutout | 71 MPa | 71 MPa | 0 |  |
| `ndp.en1993.1-9.deck.rib-to-deck-full-penetration` | orthotropic deck detail rib-to-deck-full-penetration | 125 MPa | 125 MPa | 0 |  |
| `ndp.en1993.1-9.deck.rib-to-deck-partial-penetration-plate` | orthotropic deck detail rib-to-deck-partial-penetration-plate | 71 MPa | 71 MPa | 0 |  |
| `ndp.en1993.1-9.deck.rib-to-deck-partial-penetration-throat` | orthotropic deck detail rib-to-deck-partial-penetration-throat | 36 MPa | 36 MPa | 0 |  |

### EN 1993 · Table C.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table C.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t100` | k02p grade 1.4301 at 100 C | 0.82 | 0.82 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t1000` | k02p grade 1.4301 at 1000 C | 0.06 | 0.06 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t1100` | k02p grade 1.4301 at 1100 C | 0.03 | 0.03 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t1200` | k02p grade 1.4301 at 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t20` | k02p grade 1.4301 at 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t200` | k02p grade 1.4301 at 200 C | 0.68 | 0.68 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t300` | k02p grade 1.4301 at 300 C | 0.64 | 0.64 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t400` | k02p grade 1.4301 at 400 C | 0.6 | 0.6 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t500` | k02p grade 1.4301 at 500 C | 0.54 | 0.54 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t600` | k02p grade 1.4301 at 600 C | 0.49 | 0.49 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t700` | k02p grade 1.4301 at 700 C | 0.4 | 0.4 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t800` | k02p grade 1.4301 at 800 C | 0.27 | 0.27 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.k02p.t900` | k02p grade 1.4301 at 900 C | 0.14 | 0.14 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t100` | ke grade 1.4301 at 100 C | 0.96 | 0.96 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t1000` | ke grade 1.4301 at 1000 C | 0.2 | 0.2 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t1100` | ke grade 1.4301 at 1100 C | 0.1 | 0.1 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t1200` | ke grade 1.4301 at 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t20` | ke grade 1.4301 at 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t200` | ke grade 1.4301 at 200 C | 0.92 | 0.92 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t300` | ke grade 1.4301 at 300 C | 0.88 | 0.88 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t400` | ke grade 1.4301 at 400 C | 0.84 | 0.84 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t500` | ke grade 1.4301 at 500 C | 0.8 | 0.8 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t600` | ke grade 1.4301 at 600 C | 0.76 | 0.76 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t700` | ke grade 1.4301 at 700 C | 0.71 | 0.71 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t800` | ke grade 1.4301 at 800 C | 0.63 | 0.63 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ke.t900` | ke grade 1.4301 at 900 C | 0.45 | 0.45 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t100` | ku grade 1.4301 at 100 C | 0.87 | 0.87 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t1000` | ku grade 1.4301 at 1000 C | 0.07 | 0.07 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t1100` | ku grade 1.4301 at 1100 C | 0.03 | 0.03 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t1200` | ku grade 1.4301 at 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t20` | ku grade 1.4301 at 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t200` | ku grade 1.4301 at 200 C | 0.77 | 0.77 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t300` | ku grade 1.4301 at 300 C | 0.73 | 0.73 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t400` | ku grade 1.4301 at 400 C | 0.72 | 0.72 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t500` | ku grade 1.4301 at 500 C | 0.67 | 0.67 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t600` | ku grade 1.4301 at 600 C | 0.58 | 0.58 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t700` | ku grade 1.4301 at 700 C | 0.43 | 0.43 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t800` | ku grade 1.4301 at 800 C | 0.27 | 0.27 | 0 |  |
| `ndp.en1993.1-2.stainless-fire.1-4301.ku.t900` | ku grade 1.4301 at 900 C | 0.15 | 0.15 | 0 |  |

### EN 1993-1-1 · 1.0

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-902.1` | fu/fy >= 1.05 | awaiting | awaiting | 0 |  |

### EN 1993-1-1 · 1.00

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.00.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-001.1` | non-normative | awaiting | awaiting | 0 |  |

### EN 1993-1-1 · 1.7

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.c-40.2` | ? | awaiting | awaiting | 0 |  |

### EN 1993-1-1 · 10.1.5

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 10.1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-113c.1` | ? | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 3.2.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-010.1` | Table 3.1 nominal f_y and f_u (t <= 40 mm) | awaiting | awaiting | 3 |  |

### EN 1993-1-1 · 3.2.2

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-011.1` | Ductility f_u/f_y | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 3.2.6

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-013.1` | Design values of constants | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 5.2.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-1.alpha-cr-min` | alpha_cr min | 3 | 3 | 2 |  |
| `ndp.en1993.v-026.2` | Eq. 5.2 tolerance | 0.05 | 0.05 | 4 |  |

### EN 1993-1-1 · 5.3.2(6)

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.2(6).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-024.1` | Bow imperfection inclusion rule | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 6.1(1)

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.gamma-m0` | γ_M0 | 1 | 1 | 2 |  |
| `ndp.en1993.gamma-m1` | γ_M1 | 1 | 1.1 | 3 |  |
| `ndp.en1993.gamma-m2` | γ_M2 | 1.25 | 1.25 | 1 |  |
| `ndp.en1993.gamma-m3` | γ_M3 | 1.25 | 1.25 | 1 |  |

### EN 1993-1-1 · 6.2.6(3)

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.6(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-050.1` | Shear area A_v of rolled and welded I sections | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 6.3.1.4

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-101.1` | Torsional buckling length and curve | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 6.3.2.2

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-113b.1` | W_y class dependent | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 6.3.2.4(1)B

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.4(1)B.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-112.1` | Simplified LTB method k_fl and lambda_c0 | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 6.3.4

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-123.1` | alpha_cr,op estimate | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · 7.2

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-090.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993-1-1 · 7.2.3

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-091.1` | Floor natural frequency limit | awaiting | awaiting | 1 |  |

### EN 1993-1-1 · EN 1993-1-9 Table 3.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1993-1-9 Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.gamma-mf` | γ_Mf | 1.15 | 1.15 | 1 |  |

### EN 1993-1-1 · Table 5.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-1.t51.a.elastic` | L/e0 curve a elastic | 300 | 300 | 0 |  |
| `ndp.en1993.1-1.t51.a.plastic` | L/e0 curve a plastic | 250 | 250 | 0 |  |
| `ndp.en1993.1-1.t51.a0.elastic` | L/e0 curve a0 elastic | 350 | 350 | 0 |  |
| `ndp.en1993.1-1.t51.a0.plastic` | L/e0 curve a0 plastic | 300 | 300 | 0 |  |
| `ndp.en1993.1-1.t51.b.elastic` | L/e0 curve b elastic | 250 | 250 | 0 |  |
| `ndp.en1993.1-1.t51.b.plastic` | L/e0 curve b plastic | 200 | 200 | 0 |  |
| `ndp.en1993.1-1.t51.c.elastic` | L/e0 curve c elastic | 200 | 200 | 0 |  |
| `ndp.en1993.1-1.t51.c.plastic` | L/e0 curve c plastic | 150 | 150 | 0 |  |
| `ndp.en1993.1-1.t51.d.elastic` | L/e0 curve d elastic | 150 | 150 | 0 |  |
| `ndp.en1993.1-1.t51.d.plastic` | L/e0 curve d plastic | 100 | 100 | 0 |  |

### EN 1993-1-10 · Table 2.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-10.t21.s235-j0.t-10` | t_max S235-J0 -10 C | 65 mm | 65 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t-20` | t_max S235-J0 -20 C | 55 mm | 55 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t-30` | t_max S235-J0 -30 C | 45 mm | 45 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t-40` | t_max S235-J0 -40 C | 40 mm | 40 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t-50` | t_max S235-J0 -50 C | 35 mm | 35 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t0` | t_max S235-J0 0 C | 75 mm | 75 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j0.t10` | t_max S235-J0 10 C | 90 mm | 90 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t-10` | t_max S235-J2 -10 C | 90 mm | 90 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t-20` | t_max S235-J2 -20 C | 75 mm | 75 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t-30` | t_max S235-J2 -30 C | 65 mm | 65 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t-40` | t_max S235-J2 -40 C | 55 mm | 55 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t-50` | t_max S235-J2 -50 C | 45 mm | 45 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t0` | t_max S235-J2 0 C | 105 mm | 105 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-j2.t10` | t_max S235-J2 10 C | 125 mm | 125 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t-10` | t_max S235-JR -10 C | 40 mm | 40 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t-20` | t_max S235-JR -20 C | 35 mm | 35 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t-30` | t_max S235-JR -30 C | 30 mm | 30 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t-40` | t_max S235-JR -40 C | 25 mm | 25 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t-50` | t_max S235-JR -50 C | 20 mm | 20 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t0` | t_max S235-JR 0 C | 50 mm | 50 mm | 0 |  |
| `ndp.en1993.1-10.t21.s235-jr.t10` | t_max S235-JR 10 C | 60 mm | 60 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t-10` | t_max S355-J0 -10 C | 45 mm | 45 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t-20` | t_max S355-J0 -20 C | 40 mm | 40 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t-30` | t_max S355-J0 -30 C | 35 mm | 35 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t-40` | t_max S355-J0 -40 C | 30 mm | 30 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t-50` | t_max S355-J0 -50 C | 25 mm | 25 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t0` | t_max S355-J0 0 C | 55 mm | 55 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j0.t10` | t_max S355-J0 10 C | 65 mm | 65 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t-10` | t_max S355-J2 -10 C | 65 mm | 65 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t-20` | t_max S355-J2 -20 C | 55 mm | 55 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t-30` | t_max S355-J2 -30 C | 45 mm | 45 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t-40` | t_max S355-J2 -40 C | 40 mm | 40 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t-50` | t_max S355-J2 -50 C | 35 mm | 35 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t0` | t_max S355-J2 0 C | 75 mm | 75 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-j2.t10` | t_max S355-J2 10 C | 90 mm | 90 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t-10` | t_max S355-JR -10 C | 30 mm | 30 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t-20` | t_max S355-JR -20 C | 25 mm | 25 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t-30` | t_max S355-JR -30 C | 20 mm | 20 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t-40` | t_max S355-JR -40 C | 15 mm | 15 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t-50` | t_max S355-JR -50 C | 15 mm | 15 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t0` | t_max S355-JR 0 C | 35 mm | 35 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-jr.t10` | t_max S355-JR 10 C | 40 mm | 40 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t-10` | t_max S355-K2 -10 C | 90 mm | 90 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t-20` | t_max S355-K2 -20 C | 75 mm | 75 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t-30` | t_max S355-K2 -30 C | 65 mm | 65 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t-40` | t_max S355-K2 -40 C | 55 mm | 55 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t-50` | t_max S355-K2 -50 C | 45 mm | 45 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t0` | t_max S355-K2 0 C | 105 mm | 105 mm | 0 |  |
| `ndp.en1993.1-10.t21.s355-k2.t10` | t_max S355-K2 10 C | 125 mm | 125 mm | 0 |  |

### EN 1993-1-2 · 3.4

Printed source: DIN EN 1993-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-601.1` | ;  | awaiting | awaiting | 1 |  |

### EN 1993-1-2 · 4.2.3.3

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.2.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-607.1` | Lateral-torsional buckling in fire | awaiting | awaiting | 1 |  |

### EN 1993-1-2 · 4.2.3.5

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.2.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-608.1` | Bending and axial compression in fire | awaiting | awaiting | 1 |  |

### EN 1993-1-2 · 4.3

Printed source: DIN EN 1993-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.c-47.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993-1-2 · Table 3.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-2.t31.ke.100` | k_E,theta 100 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ke.1000` | k_E,theta 1000 C | 0.045 | 0.045 | 0 |  |
| `ndp.en1993.1-2.t31.ke.1100` | k_E,theta 1100 C | 0.0225 | 0.0225 | 0 |  |
| `ndp.en1993.1-2.t31.ke.1200` | k_E,theta 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.t31.ke.20` | k_E,theta 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ke.200` | k_E,theta 200 C | 0.9 | 0.9 | 0 |  |
| `ndp.en1993.1-2.t31.ke.300` | k_E,theta 300 C | 0.8 | 0.8 | 0 |  |
| `ndp.en1993.1-2.t31.ke.400` | k_E,theta 400 C | 0.7 | 0.7 | 0 |  |
| `ndp.en1993.1-2.t31.ke.500` | k_E,theta 500 C | 0.6 | 0.6 | 0 |  |
| `ndp.en1993.1-2.t31.ke.600` | k_E,theta 600 C | 0.31 | 0.31 | 0 |  |
| `ndp.en1993.1-2.t31.ke.700` | k_E,theta 700 C | 0.13 | 0.13 | 0 |  |
| `ndp.en1993.1-2.t31.ke.800` | k_E,theta 800 C | 0.09 | 0.09 | 0 |  |
| `ndp.en1993.1-2.t31.ke.900` | k_E,theta 900 C | 0.0675 | 0.0675 | 0 |  |
| `ndp.en1993.1-2.t31.kp.100` | k_p,theta 100 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.kp.1000` | k_p,theta 1000 C | 0.025 | 0.025 | 0 |  |
| `ndp.en1993.1-2.t31.kp.1100` | k_p,theta 1100 C | 0.0125 | 0.0125 | 0 |  |
| `ndp.en1993.1-2.t31.kp.1200` | k_p,theta 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.t31.kp.20` | k_p,theta 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.kp.200` | k_p,theta 200 C | 0.807 | 0.807 | 0 |  |
| `ndp.en1993.1-2.t31.kp.300` | k_p,theta 300 C | 0.613 | 0.613 | 0 |  |
| `ndp.en1993.1-2.t31.kp.400` | k_p,theta 400 C | 0.42 | 0.42 | 0 |  |
| `ndp.en1993.1-2.t31.kp.500` | k_p,theta 500 C | 0.36 | 0.36 | 0 |  |
| `ndp.en1993.1-2.t31.kp.600` | k_p,theta 600 C | 0.18 | 0.18 | 0 |  |
| `ndp.en1993.1-2.t31.kp.700` | k_p,theta 700 C | 0.075 | 0.075 | 0 |  |
| `ndp.en1993.1-2.t31.kp.800` | k_p,theta 800 C | 0.05 | 0.05 | 0 |  |
| `ndp.en1993.1-2.t31.kp.900` | k_p,theta 900 C | 0.0375 | 0.0375 | 0 |  |
| `ndp.en1993.1-2.t31.ky.100` | k_y,theta 100 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ky.1000` | k_y,theta 1000 C | 0.04 | 0.04 | 0 |  |
| `ndp.en1993.1-2.t31.ky.1100` | k_y,theta 1100 C | 0.02 | 0.02 | 0 |  |
| `ndp.en1993.1-2.t31.ky.1200` | k_y,theta 1200 C | 0 | 0 | 0 |  |
| `ndp.en1993.1-2.t31.ky.20` | k_y,theta 20 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ky.200` | k_y,theta 200 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ky.300` | k_y,theta 300 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ky.400` | k_y,theta 400 C | 1 | 1 | 0 |  |
| `ndp.en1993.1-2.t31.ky.500` | k_y,theta 500 C | 0.78 | 0.78 | 0 |  |
| `ndp.en1993.1-2.t31.ky.600` | k_y,theta 600 C | 0.47 | 0.47 | 0 |  |
| `ndp.en1993.1-2.t31.ky.700` | k_y,theta 700 C | 0.23 | 0.23 | 0 |  |
| `ndp.en1993.1-2.t31.ky.800` | k_y,theta 800 C | 0.11 | 0.11 | 0 |  |
| `ndp.en1993.1-2.t31.ky.900` | k_y,theta 900 C | 0.06 | 0.06 | 0 |  |

### EN 1993-1-3 · 6.1.4

Printed source: DIN EN 1993-1-3/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-254.1` | αLT = 0.34 | awaiting | awaiting | 0 |  |

### EN 1993-1-3 · 9.3.1

Printed source: DIN EN 1993-1-3/NA ed. to be recorded at verification. Verify: printed table or figure 9.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.c-33.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993-1-3 · ?

Printed source: DIN EN 1993-1-3/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-252.1` | u = 2 σa⁴ bs⁴/(E² t² z²) | awaiting | awaiting | 1 |  |

### EN 1993-1-3 · Annex E

Printed source: EN 1993-1-3:2006 ed. to be recorded at verification. Verify: printed table or figure Annex E.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-258.1` | Purlin free flange on the sheeting spring K = C_D/h^2 | awaiting | awaiting | 1 |  |

### EN 1993-1-3 · Table 6.

Printed source: DIN EN 1993-1-3/NA ed. to be recorded at verification. Verify: printed table or figure Table 6..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-254.2` | ,  | awaiting | awaiting | 0 |  |

### EN 1993-1-4 · Annex C

Printed source: EN 1993-1-4:2006 ed. to be recorded at verification. Verify: printed table or figure Annex C.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-300.2` | Ramberg-Osgood exponent n of the built-in stainless grades | awaiting | awaiting | 4 |  |

### EN 1993-1-4 · Table 5.

Printed source: DIN EN 1993-1-4/NA ed. to be recorded at verification. Verify: printed table or figure Table 5..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-300.1` | kyy | awaiting | awaiting | 2 |  |

### EN 1993-1-5 · 3.2

Printed source: DIN EN 1993-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-360.1` | β_ult = β^(κ?) | awaiting | awaiting | 1 |  |

### EN 1993-1-5 · 5.3

Printed source: DIN EN 1993-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 5.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-367.1` | η = 1.2 | awaiting | awaiting | 1 |  |

### EN 1993-1-5 · 6.2

Printed source: DIN EN 1993-1-5/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.c-24.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993-1-5 · Annex A.1

Printed source: EN 1993-1-5:2006 ed. to be recorded at verification. Verify: printed table or figure Annex A.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-376.a1` | Smeared equal stiffeners: k_sigma,p = 2((1+alpha^2)^2+gamma-1)/(alpha^2(psi+1)(1+delta)) for alpha <= 4 sqrt(gamma), else 4(1+sqrt(gamma))/((psi+1)(1+delta)); gamma = I_sl/I_p with I_p = b t^3/10.92, delta = sum A_sl/(b t); sigma_cr,p = k_sigma,p sigma_E | awaiting | awaiting | 2 |  |

### EN 1993-1-5 · Annex A.2.2.1

Printed source: EN 1993-1-5:2006 ed. to be recorded at verification. Verify: printed table or figure Annex A.2.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-376.a2` | Single stiffener: a_c = 4.33 (I_sl b1^2 b2^2/(t^3 b))^(1/4); sigma_cr,sl = 1.05 E sqrt(I_sl t^3 b)/(A_sl b1 b2) for a >= a_c, else pi^2 E I_sl/(A_sl a^2) + E t^3 b a^2/(4 pi^2 (1-nu^2) A_sl b1^2 b2^2); governing = min(sigma_cr,sl, 4 sigma_E of the wider sub-panel) | awaiting | awaiting | 2 |  |

### EN 1993-1-5 · Annex B

Printed source: EN 1993-1-5:2006 ed. to be recorded at verification. Verify: printed table or figure Annex B.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-376.b1` | Non-uniform members: not encoded | awaiting | awaiting | 0 |  |

### EN 1993-1-5 · Annex C

Printed source: EN 1993-1-5:2006 ed. to be recorded at verification. Verify: printed table or figure Annex C.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-376.c1` | Finite element cross-check: ratio of the FE load factor alpha_cr to sigma_cr,p/sigma_Ed of Annex A; mesh, imperfection, material and partial factor rules of C.5 to C.9 are not encoded | awaiting | awaiting | 1 |  |

### EN 1993-1-5 · Annex D

Printed source: EN 1993-1-5:2006 ed. to be recorded at verification. Verify: printed table or figure Annex D.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-376.1` | Corrugated web shear: tau_cr,l (k_l = 5.34), tau_cr,g = 32.4/(t h^2) (B_x B_z^3)^(1/4), chi_c,l = 1.15/(0.9 + lambda), chi_c,g = 1.5/(0.5 + lambda^2), chi_c = min | awaiting | awaiting | 1 |  |

### EN 1993-1-6 · ?

Printed source: DIN EN 1993-1-6/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-804.1` | σ = 0.9 ρ g H | awaiting | awaiting | 1 |  |

### EN 1993-1-6 · Annex D

Printed source: EN 1993-1-6:2007 ed. to be recorded at verification. Verify: printed table or figure Annex D.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-806.1` | Required elastic factor of the ring general instability p_cr / p_Ed | 3 1 | 3 1 | 1 |  |

### EN 1993-1-7 · ?

Printed source: DIN EN 1993-1-7/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-903.1` | w <= b/200 | awaiting | awaiting | 0 |  |

### EN 1993-1-8 · 2.2 Table 2.1

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.2 Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-8.gamma-m5` | gamma_M5 | 1 | 1 | 2 |  |

### EN 1993-1-8 · 3.10.2

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.10.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-405.1` | Block tearing geometry | awaiting | awaiting | 1 |  |

### EN 1993-1-8 · 3.6

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-402.1` | Punching shear of bolt heads and nuts | awaiting | awaiting | 1 |  |

### EN 1993-1-8 · 4.10

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 4.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-413.1` | Effective width of a plate welded to an unstiffened flange | awaiting | awaiting | 1 |  |

### EN 1993-1-8 · 6.2.7.2

Printed source: EN 1993-1-8:2005 ed. to be recorded at verification. Verify: printed table or figure 6.2.7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-420.2` | group effect of bolt rows | end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp = 2p, nc = p | end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp = 2p, nc = p | 1 |  |

### EN 1993-1-8 · 6.3.1

Printed source: DIN EN 1993-1-8/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-431.1` |  per Table 6.11  | awaiting | awaiting | 1 |  |

### EN 1993-1-8 · 6.3.1 Table 6.8

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.1 Table 6.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.1-8.psi-end-plate` | psi | 2.7 | 2.7 | 1 |  |

### EN 1993-1-8 · 7.4.1

Printed source: EN 1993-1-8:2005 ed. to be recorded at verification. Verify: printed table or figure 7.4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-440.3` | Table 7.8 validity range of the K/N gap joint formulas | awaiting | awaiting | 1 |  |

### EN 1993-1-8 · 7.4.2

Printed source: EN 1993-1-8:2005 ed. to be recorded at verification. Verify: printed table or figure 7.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-440.1` | Table 7.11 N_1,Rd chord face | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-440.2` | Table 7.11 N_i,Rd brace effective width | awaiting | awaiting | 1 |  |
| `ndp.en1993.v-440.4` | Table 7.11 k_n chord stress function | awaiting | awaiting | 0 |  |

### EN 1993-1-9 · 6.3

Printed source: DIN EN 1993-1-9/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-502.1` | gM,f | awaiting | awaiting | 1 |  |

### EN 1993-1-9 · 6.8

Printed source: DIN EN 1993-1-9/NA ed. to be recorded at verification. Verify: printed table or figure 6.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.c-52.1` | ? | awaiting | awaiting | 0 |  |

### EN 1993-2 · 1.0

Printed source: DIN EN 1993-2/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-880.1` | gM0 | awaiting | awaiting | 3 |  |

### EN 1993-2 · 5.1

Printed source: DIN EN 1993-2/NA ed. to be recorded at verification. Verify: printed table or figure 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-501.1` |  for  | awaiting | awaiting | 0 |  |

### EN 1993-2 · 9.4.2

Printed source: EN 1993-2:2006 ed. to be recorded at verification. Verify: printed table or figure 9.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.v-881.1` | Minimum orthotropic deck plate thickness | 14 mm; 12 mm below a surfacing of at least 70 mm | 14 mm; 12 mm below a surfacing of at least 70 mm | 1 |  |
| `ndp.en1993.v-881.2` | Minimum trough wall thickness of closed stiffeners | 0.006 m | 0.006 m | 1 |  |

### EN 1993-2 · 9.5

Printed source: DIN EN 1993-2/NA ed. to be recorded at verification; EN 1993-2:2006 ed. to be recorded at verification. Verify: printed table or figure 9.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.l405.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1993.v-881.3` | Deck plate wheel strip model | awaiting | awaiting | 1 |  |

### EN 1993-5 · 7.? driving

Printed source: DIN EN 1993-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.? driving.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1993.5.pile-driving-limit-ratio` | sigma_driving / f_y limit | 0.9 | 0.9 | 2 |  |
| `ndp.en1993.5.pile-full-capacity-driving-ratio` | sigma_driving / f_y full | 0.6 | 0.6 | 2 |  |

## EN 1994 (101 unverified)

### EN 1994 · 1.0

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-19.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1994.c-30.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 1.1

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-01.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 1.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-11.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 1.25

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-16.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 1.3

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-17.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 1.6

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 1.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-36.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 5.4.1.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-03.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 5.4.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-04.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 5.4.2.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.1.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-07.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.1.2(2)

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.2(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-08.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.1.3

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-12.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.1.3(3)

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.3(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-13.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.1.4

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-06.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1994.c-09.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.2.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-22.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.6

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.l27.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.6.1.1

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-15.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.6.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-18.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.6.5.1

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-20.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.7.1

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-25.1` | in-scope\|out-of-scope | awaiting | awaiting | 0 |  |
| `ndp.en1994.c-25.2` | in-scope\|out-of-scope | awaiting | awaiting | 0 |  |

### EN 1994 · 6.7.3.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-26.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.7.3.3

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-27.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 6.7.3.6

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-29.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 7.2.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-41.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 7.3

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-40.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 7.4.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 7.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-42.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 9.7.2

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 9.7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-34.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 9.7.3(4)

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 9.7.3(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-35.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 9.7.5

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 9.7.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-37.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · 9.8

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure 9.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-39.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · ?

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.e1-12.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1994.e1-13.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1994.e2-12.3` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · EN 1994-1-1 9.7.2 / EN 1993-1-3

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-1 9.7.2 / EN 1993-1-3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.gamma-ap` | γ_ap | 1 | 1 | 2 |  |

### EN 1994 · EN 1994-1-1 9.7.3(4)

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-1 9.7.3(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.gamma-vs` | γ_VS | 1.25 | 1.25 | 2 |  |

### EN 1994 · Table 4.

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure Table 4..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-32.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · Table 5.2.

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure Table 5.2..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-02.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · Table 6.

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure Table 6..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.e1-13.2` | ? | awaiting | awaiting | 0 |  |

### EN 1994 · Table 6.6

Printed source: DIN EN 1994/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-31.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-1-1 · EN 1990 A1.4

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 A1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.deflection-divisor` | deflection span divisor | 250 | 300 | 1 |  |

### EN 1994-1-1 · EN 1994-1-1 2.4.1.2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-1 2.4.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.gamma-c` | γ_C | 1.5 | 1.5 | 1 |  |
| `ndp.en1994.gamma-s` | γ_S | 1.15 | 1.15 | 1 |  |
| `ndp.en1994.gamma-v` | γ_V | 1.25 | 1.25 | 1 |  |

### EN 1994-1-2 · 1.0

Printed source: DIN EN 1994-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.l29.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-1-2 · 3.2

Printed source: DIN EN 1994-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-45.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-1-2 · EN 1991-1-2 3.2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1991-1-2 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.1-2.alpha-unexposed` | alpha_c,unexposed | 9 W/m2K | 9 W/m2K | 4 |  |

### EN 1994-1-2 · EN 1992-1-2 Table 3.2a

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-2 Table 3.2a.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.1-2.ks.cold.100` | k_s,theta cold worked 100 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.cold.1000` | k_s,theta cold worked 1000 C | 0.05 | 0.05 | 0 |  |
| `ndp.en1994.1-2.ks.cold.1100` | k_s,theta cold worked 1100 C | 0.03 | 0.03 | 0 |  |
| `ndp.en1994.1-2.ks.cold.1200` | k_s,theta cold worked 1200 C | 0 | 0 | 0 |  |
| `ndp.en1994.1-2.ks.cold.20` | k_s,theta cold worked 20 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.cold.200` | k_s,theta cold worked 200 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.cold.300` | k_s,theta cold worked 300 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.cold.400` | k_s,theta cold worked 400 C | 0.94 | 0.94 | 0 |  |
| `ndp.en1994.1-2.ks.cold.500` | k_s,theta cold worked 500 C | 0.67 | 0.67 | 0 |  |
| `ndp.en1994.1-2.ks.cold.600` | k_s,theta cold worked 600 C | 0.4 | 0.4 | 0 |  |
| `ndp.en1994.1-2.ks.cold.700` | k_s,theta cold worked 700 C | 0.12 | 0.12 | 0 |  |
| `ndp.en1994.1-2.ks.cold.800` | k_s,theta cold worked 800 C | 0.11 | 0.11 | 0 |  |
| `ndp.en1994.1-2.ks.cold.900` | k_s,theta cold worked 900 C | 0.08 | 0.08 | 0 |  |
| `ndp.en1994.1-2.ks.hot.100` | k_s,theta hot rolled 100 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.hot.1000` | k_s,theta hot rolled 1000 C | 0.04 | 0.04 | 0 |  |
| `ndp.en1994.1-2.ks.hot.1100` | k_s,theta hot rolled 1100 C | 0.02 | 0.02 | 0 |  |
| `ndp.en1994.1-2.ks.hot.1200` | k_s,theta hot rolled 1200 C | 0 | 0 | 0 |  |
| `ndp.en1994.1-2.ks.hot.20` | k_s,theta hot rolled 20 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.hot.200` | k_s,theta hot rolled 200 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.hot.300` | k_s,theta hot rolled 300 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.hot.400` | k_s,theta hot rolled 400 C | 1 | 1 | 0 |  |
| `ndp.en1994.1-2.ks.hot.500` | k_s,theta hot rolled 500 C | 0.78 | 0.78 | 0 |  |
| `ndp.en1994.1-2.ks.hot.600` | k_s,theta hot rolled 600 C | 0.47 | 0.47 | 0 |  |
| `ndp.en1994.1-2.ks.hot.700` | k_s,theta hot rolled 700 C | 0.23 | 0.23 | 0 |  |
| `ndp.en1994.1-2.ks.hot.800` | k_s,theta hot rolled 800 C | 0.11 | 0.11 | 0 |  |
| `ndp.en1994.1-2.ks.hot.900` | k_s,theta hot rolled 900 C | 0.06 | 0.06 | 0 |  |

### EN 1994-1-2 · EN 1994-1-2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.1-2.buckling-curve` | 1-2.buckling-curve | awaiting | awaiting | 1 |  |
| `ndp.en1994.1-2.concrete-conduction` | 1-2.concrete-conduction | awaiting | awaiting | 2 |  |
| `ndp.en1994.1-2.eps-c1-theta` | 1-2.eps-c1-theta | awaiting | awaiting | 1 |  |
| `ndp.en1994.1-2.interaction` | 1-2.interaction | awaiting | awaiting | 1 |  |
| `ndp.en1994.1-2.kc-theta` | 1-2.kc-theta | awaiting | awaiting | 3 |  |
| `ndp.en1994.1-2.ke-theta` | 1-2.ke-theta | awaiting | awaiting | 1 |  |
| `ndp.en1994.1-2.stiffness-phi` | 1-2.stiffness-phi | awaiting | awaiting | 1 |  |
| `ndp.en1994.1-2.surface-exchange` | 1-2.surface-exchange | awaiting | awaiting | 2 |  |
| `ndp.en1994.1-2.tables` | 1-2.tables | awaiting | awaiting | 3 |  |

### EN 1994-1-2 · EN 1994-1-2 3.3.2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-2 3.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.1-2.rho-concrete` | rho_c | 2300 kg/m3 | 2300 kg/m3 | 4 |  |

### EN 1994-1-2 · EN 1994-1-2 4.3.2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-1-2 4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.1-2.heff` | h_eff | awaiting | awaiting | 5 |  |

### EN 1994-1-2 · Table 4.1

Printed source: DIN EN 1994-1-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-46.1` | ηfi | awaiting | awaiting | 0 |  |

### EN 1994-1-2 · Table 6.2

Printed source: DIN EN 1994-1-2/NA ed. to be recorded at verification. Verify: printed table or figure Table 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.l694.1` | [T] | awaiting | awaiting | 0 |  |

### EN 1994-2 · 5.4

Printed source: DIN EN 1994-2/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-49.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-2 · 6.2

Printed source: DIN EN 1994-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-50.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-2 · 6.4

Printed source: DIN EN 1994-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-51.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-2 · 6.6.2

Printed source: DIN EN 1994-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.c-53.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1994.c-53.2` | ? | awaiting | awaiting | 0 |  |

### EN 1994-2 · 6.6.5

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification; DIN EN 1994-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.2.end-plate-bearing` | F_Rdu | awaiting | awaiting | 3 |  |
| `ndp.en1994.2.stud-cross-spacing` | e_t/d | 2.5 | awaiting | 4 |  |
| `ndp.en1994.2.stud-edge-distance` | e_D | 0.02 m | awaiting | 4 |  |
| `ndp.en1994.c-54.1` | ? | awaiting | awaiting | 0 |  |

### EN 1994-2 · 6.6.6.3

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.6.6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.2.min-transverse-reinforcement` | rho_t | 0.002 | awaiting | 4 |  |

### EN 1994-2 · EN 1994-2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.2.rebar-fatigue-range` | 2.rebar-fatigue-range | awaiting | awaiting | 7 |  |
| `ndp.en1994.2.shear-lag` | 2.shear-lag | awaiting | awaiting | 2 |  |

### EN 1994-2 · EN 1994-2 6.8.2

Printed source: DIN EN 1994-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1994-2 6.8.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1994.gamma-mf` | γ_Mf bridge steel fatigue | 1.15 | 1.35 | 1 |  |

## EN 1995 (488 unverified)

### EN 1995 · 1.0

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-15.1` | kcr = 1.0 | awaiting | awaiting | 0 |  |
| `ndp.en1995.t-20.1` | λ | awaiting | awaiting | 0 |  |

### EN 1995 · 1.25

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 1.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-82.1` | … | awaiting | awaiting | 0 |  |

### EN 1995 · 1.3

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-46.1` | x | awaiting | awaiting | 0 |  |

### EN 1995 · 1.5

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-81.1` | kt = min(1.0, …) | awaiting | awaiting | 0 |  |

### EN 1995 · 2.6

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 2.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-70.1` | d = deff = 1.1 dc? | awaiting | awaiting | 0 |  |

### EN 1995 · 3.4.2

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 3.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-100.1` | 0.9/-  | awaiting | awaiting | 0 |  |
| `ndp.en1995.t-100.2` | k_ρ | awaiting | awaiting | 0 |  |

### EN 1995 · 4.5

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 4.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.e3-bearing-perpendicular.1` | f1 ≥ 6 Hz | awaiting | awaiting | 0 |  |

### EN 1995 · 5.2

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-18.1` | Anet | awaiting | awaiting | 0 |  |

### EN 1995 · 5.4.4

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-24.1` | ec = βc · lef/200 | awaiting | awaiting | 0 |  |

### EN 1995 · 6.1.5

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-13.1` | kc,90 | awaiting | awaiting | 0 |  |

### EN 1995 · 6.3.3

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-22.1` | 0.4 h | awaiting | awaiting | 0 |  |

### EN 1995 · 6.5.2

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 6.5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-45.1` | kv = 1 | awaiting | awaiting | 0 |  |

### EN 1995 · 7.3.3

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-64.1` | a = 0.5 mm/kN | awaiting | awaiting | 0 |  |

### EN 1995 · 8.1.4

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 8.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-78.1` | α | awaiting | awaiting | 0 |  |

### EN 1995 · 9.1

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 9.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-76.1` | ScrewApproval | awaiting | awaiting | 0 |  |

### EN 1995 · 9.1.1

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure 9.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-91.1` | hw/bw > 70 | awaiting | awaiting | 0 |  |

### EN 1995 · Table 3.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.kmod.clt.sc1.instant` | k_mod clt SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.clt.sc1.long` | k_mod clt SC1 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.clt.sc1.medium` | k_mod clt SC1 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.clt.sc1.permanent` | k_mod clt SC1 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.clt.sc1.short` | k_mod clt SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.clt.sc2.instant` | k_mod clt SC2 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.clt.sc2.long` | k_mod clt SC2 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.clt.sc2.medium` | k_mod clt SC2 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.clt.sc2.permanent` | k_mod clt SC2 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.clt.sc2.short` | k_mod clt SC2 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.clt.sc3.instant` | k_mod clt SC3 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.clt.sc3.long` | k_mod clt SC3 long | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.clt.sc3.medium` | k_mod clt SC3 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.clt.sc3.permanent` | k_mod clt SC3 permanent | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.clt.sc3.short` | k_mod clt SC3 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc1.instant` | k_mod fibreboard-hb-hla2 SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc1.long` | k_mod fibreboard-hb-hla2 SC1 long | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc1.medium` | k_mod fibreboard-hb-hla2 SC1 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc1.permanent` | k_mod fibreboard-hb-hla2 SC1 permanent | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc1.short` | k_mod fibreboard-hb-hla2 SC1 short | 0.85 | 0.85 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc2.instant` | k_mod fibreboard-hb-hla2 SC2 instant | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc2.long` | k_mod fibreboard-hb-hla2 SC2 long | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc2.medium` | k_mod fibreboard-hb-hla2 SC2 medium | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc2.permanent` | k_mod fibreboard-hb-hla2 SC2 permanent | 0.2 | 0.2 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc2.short` | k_mod fibreboard-hb-hla2 SC2 short | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc3.instant` | k_mod fibreboard-hb-hla2 SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc3.long` | k_mod fibreboard-hb-hla2 SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc3.medium` | k_mod fibreboard-hb-hla2 SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc3.permanent` | k_mod fibreboard-hb-hla2 SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-hb-hla2.sc3.short` | k_mod fibreboard-hb-hla2 SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc1.instant` | k_mod fibreboard-mdf-hls SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc1.long` | k_mod fibreboard-mdf-hls SC1 long | 0.4 | 0.4 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc1.medium` | k_mod fibreboard-mdf-hls SC1 medium | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc1.permanent` | k_mod fibreboard-mdf-hls SC1 permanent | 0.2 | 0.2 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc1.short` | k_mod fibreboard-mdf-hls SC1 short | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc2.instant` | k_mod fibreboard-mdf-hls SC2 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc2.long` | k_mod fibreboard-mdf-hls SC2 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc2.medium` | k_mod fibreboard-mdf-hls SC2 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc2.permanent` | k_mod fibreboard-mdf-hls SC2 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc2.short` | k_mod fibreboard-mdf-hls SC2 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc3.instant` | k_mod fibreboard-mdf-hls SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc3.long` | k_mod fibreboard-mdf-hls SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc3.medium` | k_mod fibreboard-mdf-hls SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc3.permanent` | k_mod fibreboard-mdf-hls SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.fibreboard-mdf-hls.sc3.short` | k_mod fibreboard-mdf-hls SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.glulam.sc1.instant` | k_mod glulam SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.glulam.sc1.long` | k_mod glulam SC1 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.glulam.sc1.medium` | k_mod glulam SC1 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.glulam.sc1.permanent` | k_mod glulam SC1 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.glulam.sc1.short` | k_mod glulam SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.glulam.sc2.instant` | k_mod glulam SC2 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.glulam.sc2.long` | k_mod glulam SC2 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.glulam.sc2.medium` | k_mod glulam SC2 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.glulam.sc2.permanent` | k_mod glulam SC2 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.glulam.sc2.short` | k_mod glulam SC2 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.glulam.sc3.instant` | k_mod glulam SC3 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.glulam.sc3.long` | k_mod glulam SC3 long | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.glulam.sc3.medium` | k_mod glulam SC3 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.glulam.sc3.permanent` | k_mod glulam SC3 permanent | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.glulam.sc3.short` | k_mod glulam SC3 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.lvl.sc1.instant` | k_mod lvl SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.lvl.sc1.long` | k_mod lvl SC1 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.lvl.sc1.medium` | k_mod lvl SC1 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.lvl.sc1.permanent` | k_mod lvl SC1 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.lvl.sc1.short` | k_mod lvl SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.lvl.sc2.instant` | k_mod lvl SC2 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.lvl.sc2.long` | k_mod lvl SC2 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.lvl.sc2.medium` | k_mod lvl SC2 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.lvl.sc2.permanent` | k_mod lvl SC2 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.lvl.sc2.short` | k_mod lvl SC2 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.lvl.sc3.instant` | k_mod lvl SC3 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.lvl.sc3.long` | k_mod lvl SC3 long | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.lvl.sc3.medium` | k_mod lvl SC3 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.lvl.sc3.permanent` | k_mod lvl SC3 permanent | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.lvl.sc3.short` | k_mod lvl SC3 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.osb3.sc1.instant` | k_mod osb3 SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.osb3.sc1.long` | k_mod osb3 SC1 long | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.osb3.sc1.medium` | k_mod osb3 SC1 medium | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.osb3.sc1.permanent` | k_mod osb3 SC1 permanent | 0.4 | 0.4 | 0 |  |
| `ndp.en1995.kmod.osb3.sc1.short` | k_mod osb3 SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.osb3.sc2.instant` | k_mod osb3 SC2 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.osb3.sc2.long` | k_mod osb3 SC2 long | 0.4 | 0.4 | 0 |  |
| `ndp.en1995.kmod.osb3.sc2.medium` | k_mod osb3 SC2 medium | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.osb3.sc2.permanent` | k_mod osb3 SC2 permanent | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.osb3.sc2.short` | k_mod osb3 SC2 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.osb3.sc3.instant` | k_mod osb3 SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb3.sc3.long` | k_mod osb3 SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb3.sc3.medium` | k_mod osb3 SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb3.sc3.permanent` | k_mod osb3 SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb3.sc3.short` | k_mod osb3 SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb4.sc1.instant` | k_mod osb4 SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.osb4.sc1.long` | k_mod osb4 SC1 long | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.osb4.sc1.medium` | k_mod osb4 SC1 medium | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.osb4.sc1.permanent` | k_mod osb4 SC1 permanent | 0.4 | 0.4 | 0 |  |
| `ndp.en1995.kmod.osb4.sc1.short` | k_mod osb4 SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.osb4.sc2.instant` | k_mod osb4 SC2 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.osb4.sc2.long` | k_mod osb4 SC2 long | 0.4 | 0.4 | 0 |  |
| `ndp.en1995.kmod.osb4.sc2.medium` | k_mod osb4 SC2 medium | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.osb4.sc2.permanent` | k_mod osb4 SC2 permanent | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.osb4.sc2.short` | k_mod osb4 SC2 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.osb4.sc3.instant` | k_mod osb4 SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb4.sc3.long` | k_mod osb4 SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb4.sc3.medium` | k_mod osb4 SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb4.sc3.permanent` | k_mod osb4 SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.osb4.sc3.short` | k_mod osb4 SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc1.instant` | k_mod particleboard-p5 SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc1.long` | k_mod particleboard-p5 SC1 long | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc1.medium` | k_mod particleboard-p5 SC1 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc1.permanent` | k_mod particleboard-p5 SC1 permanent | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc1.short` | k_mod particleboard-p5 SC1 short | 0.85 | 0.85 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc2.instant` | k_mod particleboard-p5 SC2 instant | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc2.long` | k_mod particleboard-p5 SC2 long | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc2.medium` | k_mod particleboard-p5 SC2 medium | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc2.permanent` | k_mod particleboard-p5 SC2 permanent | 0.2 | 0.2 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc2.short` | k_mod particleboard-p5 SC2 short | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc3.instant` | k_mod particleboard-p5 SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc3.long` | k_mod particleboard-p5 SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc3.medium` | k_mod particleboard-p5 SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc3.permanent` | k_mod particleboard-p5 SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p5.sc3.short` | k_mod particleboard-p5 SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc1.instant` | k_mod particleboard-p7 SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc1.long` | k_mod particleboard-p7 SC1 long | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc1.medium` | k_mod particleboard-p7 SC1 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc1.permanent` | k_mod particleboard-p7 SC1 permanent | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc1.short` | k_mod particleboard-p7 SC1 short | 0.85 | 0.85 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc2.instant` | k_mod particleboard-p7 SC2 instant | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc2.long` | k_mod particleboard-p7 SC2 long | 0.3 | 0.3 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc2.medium` | k_mod particleboard-p7 SC2 medium | 0.45 | 0.45 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc2.permanent` | k_mod particleboard-p7 SC2 permanent | 0.2 | 0.2 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc2.short` | k_mod particleboard-p7 SC2 short | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc3.instant` | k_mod particleboard-p7 SC3 instant | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc3.long` | k_mod particleboard-p7 SC3 long | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc3.medium` | k_mod particleboard-p7 SC3 medium | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc3.permanent` | k_mod particleboard-p7 SC3 permanent | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.particleboard-p7.sc3.short` | k_mod particleboard-p7 SC3 short | 0 | 0 | 0 |  |
| `ndp.en1995.kmod.plywood.sc1.instant` | k_mod plywood SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.plywood.sc1.long` | k_mod plywood SC1 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.plywood.sc1.medium` | k_mod plywood SC1 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.plywood.sc1.permanent` | k_mod plywood SC1 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.plywood.sc1.short` | k_mod plywood SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.plywood.sc2.instant` | k_mod plywood SC2 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.plywood.sc2.long` | k_mod plywood SC2 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.plywood.sc2.medium` | k_mod plywood SC2 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.plywood.sc2.permanent` | k_mod plywood SC2 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.plywood.sc2.short` | k_mod plywood SC2 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.plywood.sc3.instant` | k_mod plywood SC3 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.plywood.sc3.long` | k_mod plywood SC3 long | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.plywood.sc3.medium` | k_mod plywood SC3 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.plywood.sc3.permanent` | k_mod plywood SC3 permanent | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.plywood.sc3.short` | k_mod plywood SC3 short | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.solid.sc1.instant` | k_mod solid SC1 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.solid.sc1.long` | k_mod solid SC1 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.solid.sc1.medium` | k_mod solid SC1 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.solid.sc1.permanent` | k_mod solid SC1 permanent | 0.6 | 0.6 | 1 |  |
| `ndp.en1995.kmod.solid.sc1.short` | k_mod solid SC1 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.solid.sc2.instant` | k_mod solid SC2 instant | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.kmod.solid.sc2.long` | k_mod solid SC2 long | 0.7 | 0.7 | 0 |  |
| `ndp.en1995.kmod.solid.sc2.medium` | k_mod solid SC2 medium | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kmod.solid.sc2.permanent` | k_mod solid SC2 permanent | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kmod.solid.sc2.short` | k_mod solid SC2 short | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.solid.sc3.instant` | k_mod solid SC3 instant | 0.9 | 0.9 | 0 |  |
| `ndp.en1995.kmod.solid.sc3.long` | k_mod solid SC3 long | 0.55 | 0.55 | 0 |  |
| `ndp.en1995.kmod.solid.sc3.medium` | k_mod solid SC3 medium | 0.65 | 0.65 | 0 |  |
| `ndp.en1995.kmod.solid.sc3.permanent` | k_mod solid SC3 permanent | 0.5 | 0.5 | 0 |  |
| `ndp.en1995.kmod.solid.sc3.short` | k_mod solid SC3 short | 0.7 | 0.7 | 0 |  |

### EN 1995 · Table 3.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.kdef.clt.sc1` | k_def clt SC1 | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kdef.clt.sc2` | k_def clt SC2 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.clt.sc3` | k_def clt SC3 | 2 | 2 | 0 |  |
| `ndp.en1995.kdef.fibreboard-hb-hla2.sc1` | k_def fibreboard-hb-hla2 SC1 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.fibreboard-hb-hla2.sc2` | k_def fibreboard-hb-hla2 SC2 | 3 | 3 | 0 |  |
| `ndp.en1995.kdef.fibreboard-hb-hla2.sc3` | k_def fibreboard-hb-hla2 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.fibreboard-mdf-hls.sc1` | k_def fibreboard-mdf-hls SC1 | 3 | 3 | 0 |  |
| `ndp.en1995.kdef.fibreboard-mdf-hls.sc2` | k_def fibreboard-mdf-hls SC2 | 4 | 4 | 0 |  |
| `ndp.en1995.kdef.fibreboard-mdf-hls.sc3` | k_def fibreboard-mdf-hls SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.glulam.sc1` | k_def glulam SC1 | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kdef.glulam.sc2` | k_def glulam SC2 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.glulam.sc3` | k_def glulam SC3 | 2 | 2 | 0 |  |
| `ndp.en1995.kdef.lvl.sc1` | k_def lvl SC1 | 0.6 | 0.6 | 0 |  |
| `ndp.en1995.kdef.lvl.sc2` | k_def lvl SC2 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.lvl.sc3` | k_def lvl SC3 | 2 | 2 | 0 |  |
| `ndp.en1995.kdef.osb2.sc1` | k_def osb2 SC1 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.osb2.sc2` | k_def osb2 SC2 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.osb2.sc3` | k_def osb2 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.osb3.sc1` | k_def osb3 SC1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1995.kdef.osb3.sc2` | k_def osb3 SC2 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.osb3.sc3` | k_def osb3 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.osb4.sc1` | k_def osb4 SC1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1995.kdef.osb4.sc2` | k_def osb4 SC2 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.osb4.sc3` | k_def osb4 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.particleboard-p4.sc1` | k_def particleboard-p4 SC1 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.particleboard-p4.sc2` | k_def particleboard-p4 SC2 | 3 | 3 | 0 |  |
| `ndp.en1995.kdef.particleboard-p4.sc3` | k_def particleboard-p4 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.particleboard-p5.sc1` | k_def particleboard-p5 SC1 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.particleboard-p5.sc2` | k_def particleboard-p5 SC2 | 3 | 3 | 0 |  |
| `ndp.en1995.kdef.particleboard-p5.sc3` | k_def particleboard-p5 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.particleboard-p6.sc1` | k_def particleboard-p6 SC1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1995.kdef.particleboard-p6.sc2` | k_def particleboard-p6 SC2 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.particleboard-p6.sc3` | k_def particleboard-p6 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.particleboard-p7.sc1` | k_def particleboard-p7 SC1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1995.kdef.particleboard-p7.sc2` | k_def particleboard-p7 SC2 | 2.25 | 2.25 | 0 |  |
| `ndp.en1995.kdef.particleboard-p7.sc3` | k_def particleboard-p7 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.plywood-636-1.sc1` | k_def plywood-636-1 SC1 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.plywood-636-1.sc2` | k_def plywood-636-1 SC2 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.plywood-636-1.sc3` | k_def plywood-636-1 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.plywood-636-2.sc1` | k_def plywood-636-2 SC1 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.plywood-636-2.sc2` | k_def plywood-636-2 SC2 | 1 | 1 | 0 |  |
| `ndp.en1995.kdef.plywood-636-2.sc3` | k_def plywood-636-2 SC3 | 0 | 0 | 0 |  |
| `ndp.en1995.kdef.plywood-636-3.sc1` | k_def plywood-636-3 SC1 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.plywood-636-3.sc2` | k_def plywood-636-3 SC2 | 1 | 1 | 0 |  |
| `ndp.en1995.kdef.plywood-636-3.sc3` | k_def plywood-636-3 SC3 | 2.5 | 2.5 | 0 |  |
| `ndp.en1995.kdef.solid.sc1` | k_def solid SC1 | 0.6 | 0.6 | 1 |  |
| `ndp.en1995.kdef.solid.sc2` | k_def solid SC2 | 0.8 | 0.8 | 0 |  |
| `ndp.en1995.kdef.solid.sc3` | k_def solid SC3 | 2 | 2 | 0 |  |

### EN 1995 · Table 5

Printed source: EN 14080:2013 Table 5 ed. to be recorded at verification. Verify: printed table or figure Table 5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.strength.gl20c.e005` | E_0,05 GL20c | 8600 N/mm2 | 8600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.e0mean` | E_0,mean GL20c | 10400 N/mm2 | 10400 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.e90mean` | E_90,mean GL20c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.fc0k` | f_c,0,k GL20c | 15.5 N/mm2 | 15.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.fc90k` | f_c,90,k GL20c | 2.4 N/mm2 | 2.4 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.fmk` | f_m,k GL20c | 20 N/mm2 | 20 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.ft0k` | f_t,0,k GL20c | 11.5 N/mm2 | 11.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.ft90k` | f_t,90,k GL20c | 0.35 N/mm2 | 0.35 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.fvk` | f_v,k GL20c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.gmean` | G_mean GL20c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20c.rhok` | ρ_k GL20c | 335 kg/m3 | 335 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl20c.rhomean` | ρ_mean GL20c | 370 kg/m3 | 370 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl20h.e005` | E_0,05 GL20h | 7000 N/mm2 | 7000 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.e0mean` | E_0,mean GL20h | 8400 N/mm2 | 8400 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.e90mean` | E_90,mean GL20h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.fc0k` | f_c,0,k GL20h | 20 N/mm2 | 20 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.fc90k` | f_c,90,k GL20h | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.fmk` | f_m,k GL20h | 20 N/mm2 | 20 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.ft0k` | f_t,0,k GL20h | 16 N/mm2 | 16 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.ft90k` | f_t,90,k GL20h | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.fvk` | f_v,k GL20h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.gmean` | G_mean GL20h | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl20h.rhok` | ρ_k GL20h | 340 kg/m3 | 340 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl20h.rhomean` | ρ_mean GL20h | 370 kg/m3 | 370 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl22c.e005` | E_0,05 GL22c | 8600 N/mm2 | 8600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.e0mean` | E_0,mean GL22c | 10400 N/mm2 | 10400 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.e90mean` | E_90,mean GL22c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.fc0k` | f_c,0,k GL22c | 18.5 N/mm2 | 18.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.fc90k` | f_c,90,k GL22c | 2.4 N/mm2 | 2.4 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.fmk` | f_m,k GL22c | 22 N/mm2 | 22 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.ft0k` | f_t,0,k GL22c | 13 N/mm2 | 13 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.ft90k` | f_t,90,k GL22c | 0.35 N/mm2 | 0.35 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.fvk` | f_v,k GL22c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.gmean` | G_mean GL22c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22c.rhok` | ρ_k GL22c | 355 kg/m3 | 355 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl22c.rhomean` | ρ_mean GL22c | 390 kg/m3 | 390 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl22h.e005` | E_0,05 GL22h | 8800 N/mm2 | 8800 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.e0mean` | E_0,mean GL22h | 10500 N/mm2 | 10500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.e90mean` | E_90,mean GL22h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.fc0k` | f_c,0,k GL22h | 22 N/mm2 | 22 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.fc90k` | f_c,90,k GL22h | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.fmk` | f_m,k GL22h | 22 N/mm2 | 22 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.ft0k` | f_t,0,k GL22h | 17.6 N/mm2 | 17.6 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.ft90k` | f_t,90,k GL22h | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.fvk` | f_v,k GL22h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.gmean` | G_mean GL22h | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl22h.rhok` | ρ_k GL22h | 370 kg/m3 | 370 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl22h.rhomean` | ρ_mean GL22h | 410 kg/m3 | 410 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl24c.e005` | E_0,05 GL24c | 9100 N/mm2 | 9100 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.e0mean` | E_0,mean GL24c | 11000 N/mm2 | 11000 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.e90mean` | E_90,mean GL24c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.fc0k` | f_c,0,k GL24c | 21.5 N/mm2 | 21.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.fc90k` | f_c,90,k GL24c | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.fmk` | f_m,k GL24c | 24 N/mm2 | 24 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.ft0k` | f_t,0,k GL24c | 17 N/mm2 | 17 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.ft90k` | f_t,90,k GL24c | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.fvk` | f_v,k GL24c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.gmean` | G_mean GL24c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24c.rhok` | ρ_k GL24c | 350 kg/m3 | 350 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl24c.rhomean` | ρ_mean GL24c | 385 kg/m3 | 385 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl24h.e005` | E_0,05 GL24h | 9600 N/mm2 | 9600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24h.e0mean` | E_0,mean GL24h | 11500 N/mm2 | 11500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24h.e90mean` | E_90,mean GL24h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24h.fmk` | f_m,k GL24h | 24 N/mm2 | 24 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24h.fvk` | f_v,k GL24h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl24h.rhok` | ρ_k GL24h | 385 kg/m3 | 385 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl24h.rhomean` | ρ_mean GL24h | 420 kg/m3 | 420 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl26c.e005` | E_0,05 GL26c | 9600 N/mm2 | 9600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.e0mean` | E_0,mean GL26c | 11500 N/mm2 | 11500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.e90mean` | E_90,mean GL26c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.fc0k` | f_c,0,k GL26c | 23 N/mm2 | 23 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.fc90k` | f_c,90,k GL26c | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.fmk` | f_m,k GL26c | 26 N/mm2 | 26 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.ft0k` | f_t,0,k GL26c | 19 N/mm2 | 19 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.ft90k` | f_t,90,k GL26c | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.fvk` | f_v,k GL26c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.gmean` | G_mean GL26c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26c.rhok` | ρ_k GL26c | 385 kg/m3 | 385 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl26c.rhomean` | ρ_mean GL26c | 420 kg/m3 | 420 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl26h.e005` | E_0,05 GL26h | 10100 N/mm2 | 10100 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.e0mean` | E_0,mean GL26h | 12100 N/mm2 | 12100 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.e90mean` | E_90,mean GL26h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.fc0k` | f_c,0,k GL26h | 26 N/mm2 | 26 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.fc90k` | f_c,90,k GL26h | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.fmk` | f_m,k GL26h | 26 N/mm2 | 26 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.ft0k` | f_t,0,k GL26h | 20.8 N/mm2 | 20.8 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.ft90k` | f_t,90,k GL26h | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.fvk` | f_v,k GL26h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.gmean` | G_mean GL26h | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl26h.rhok` | ρ_k GL26h | 405 kg/m3 | 405 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl26h.rhomean` | ρ_mean GL26h | 445 kg/m3 | 445 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl28c.e005` | E_0,05 GL28c | 10400 N/mm2 | 10400 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.e0mean` | E_0,mean GL28c | 12500 N/mm2 | 12500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.e90mean` | E_90,mean GL28c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.fc0k` | f_c,0,k GL28c | 24 N/mm2 | 24 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.fc90k` | f_c,90,k GL28c | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.fmk` | f_m,k GL28c | 28 N/mm2 | 28 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.ft0k` | f_t,0,k GL28c | 19.5 N/mm2 | 19.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.ft90k` | f_t,90,k GL28c | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.fvk` | f_v,k GL28c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.gmean` | G_mean GL28c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28c.rhok` | ρ_k GL28c | 390 kg/m3 | 390 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl28c.rhomean` | ρ_mean GL28c | 425 kg/m3 | 425 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl28h.e005` | E_0,05 GL28h | 10500 N/mm2 | 10500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28h.e0mean` | E_0,mean GL28h | 12600 N/mm2 | 12600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28h.e90mean` | E_90,mean GL28h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28h.fmk` | f_m,k GL28h | 28 N/mm2 | 28 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28h.fvk` | f_v,k GL28h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl28h.rhok` | ρ_k GL28h | 425 kg/m3 | 425 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl28h.rhomean` | ρ_mean GL28h | 465 kg/m3 | 465 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl30c.e005` | E_0,05 GL30c | 10800 N/mm2 | 10800 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.e0mean` | E_0,mean GL30c | 13000 N/mm2 | 13000 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.e90mean` | E_90,mean GL30c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.fc0k` | f_c,0,k GL30c | 24.5 N/mm2 | 24.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.fc90k` | f_c,90,k GL30c | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.fmk` | f_m,k GL30c | 30 N/mm2 | 30 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.ft0k` | f_t,0,k GL30c | 19.5 N/mm2 | 19.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.ft90k` | f_t,90,k GL30c | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.fvk` | f_v,k GL30c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.gmean` | G_mean GL30c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30c.rhok` | ρ_k GL30c | 390 kg/m3 | 390 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl30c.rhomean` | ρ_mean GL30c | 430 kg/m3 | 430 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl30h.e005` | E_0,05 GL30h | 11300 N/mm2 | 11300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.e0mean` | E_0,mean GL30h | 13600 N/mm2 | 13600 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.e90mean` | E_90,mean GL30h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.fc0k` | f_c,0,k GL30h | 30 N/mm2 | 30 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.fc90k` | f_c,90,k GL30h | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.fmk` | f_m,k GL30h | 30 N/mm2 | 30 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.ft0k` | f_t,0,k GL30h | 24 N/mm2 | 24 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.ft90k` | f_t,90,k GL30h | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.fvk` | f_v,k GL30h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.gmean` | G_mean GL30h | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl30h.rhok` | ρ_k GL30h | 430 kg/m3 | 430 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl30h.rhomean` | ρ_mean GL30h | 480 kg/m3 | 480 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl32c.e005` | E_0,05 GL32c | 11200 N/mm2 | 11200 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.e0mean` | E_0,mean GL32c | 13500 N/mm2 | 13500 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.e90mean` | E_90,mean GL32c | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.fc0k` | f_c,0,k GL32c | 24.5 N/mm2 | 24.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.fc90k` | f_c,90,k GL32c | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.fmk` | f_m,k GL32c | 32 N/mm2 | 32 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.ft0k` | f_t,0,k GL32c | 19.5 N/mm2 | 19.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.ft90k` | f_t,90,k GL32c | 0.5 N/mm2 | 0.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.fvk` | f_v,k GL32c | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.gmean` | G_mean GL32c | 650 N/mm2 | 650 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32c.rhok` | ρ_k GL32c | 400 kg/m3 | 400 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl32c.rhomean` | ρ_mean GL32c | 440 kg/m3 | 440 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl32h.e005` | E_0,05 GL32h | 11800 N/mm2 | 11800 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32h.e0mean` | E_0,mean GL32h | 14200 N/mm2 | 14200 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32h.e90mean` | E_90,mean GL32h | 300 N/mm2 | 300 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32h.fmk` | f_m,k GL32h | 32 N/mm2 | 32 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32h.fvk` | f_v,k GL32h | 3.5 N/mm2 | 3.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.gl32h.rhok` | ρ_k GL32h | 440 kg/m3 | 440 kg/m3 | 0 |  |
| `ndp.en1995.strength.gl32h.rhomean` | ρ_mean GL32h | 490 kg/m3 | 490 kg/m3 | 0 |  |

### EN 1995 · Table 7.2

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure Table 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-61.1` | winst,Q ≤ L/300 | awaiting | awaiting | 0 |  |

### EN 1995 · Table 8.8

Printed source: DIN EN 1995/NA ed. to be recorded at verification. Verify: printed table or figure Table 8.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-80.1` |  ⇒  | awaiting | awaiting | 0 |  |

### EN 1995 · material

Printed source: EN 16351 lamellae C24 ed. to be recorded at verification; EN 14374 / ETA ed. to be recorded at verification. Verify: printed table or figure material.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.strength.clt100.e005` | E_0,05 CLT100 | 7400 N/mm2 | 7400 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.e0mean` | E_0,mean CLT100 | 11000 N/mm2 | 11000 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.e90mean` | E_90,mean CLT100 | 370 N/mm2 | 370 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.fc0k` | f_c,0,k CLT100 | 21 N/mm2 | 21 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.fc90k` | f_c,90,k CLT100 | 2.5 N/mm2 | 2.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.fmk` | f_m,k CLT100 | 24 N/mm2 | 24 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.ft0k` | f_t,0,k CLT100 | 14.5 N/mm2 | 14.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.ft90k` | f_t,90,k CLT100 | 0.4 N/mm2 | 0.4 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.fvk` | f_v,k CLT100 | 4 N/mm2 | 4 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.gmean` | G_mean CLT100 | 690 N/mm2 | 690 N/mm2 | 0 |  |
| `ndp.en1995.strength.clt100.rhok` | ρ_k CLT100 | 350 kg/m3 | 350 kg/m3 | 0 |  |
| `ndp.en1995.strength.clt100.rhomean` | ρ_mean CLT100 | 420 kg/m3 | 420 kg/m3 | 0 |  |
| `ndp.en1995.strength.lvl32.e005` | E_0,05 LVL32 | 11600 N/mm2 | 11600 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.e0mean` | E_0,mean LVL32 | 14000 N/mm2 | 14000 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.e90mean` | E_90,mean LVL32 | 500 N/mm2 | 500 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.fc0k` | f_c,0,k LVL32 | 35 N/mm2 | 35 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.fc90k` | f_c,90,k LVL32 | 6 N/mm2 | 6 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.fmk` | f_m,k LVL32 | 36 N/mm2 | 36 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.ft0k` | f_t,0,k LVL32 | 26 N/mm2 | 26 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.ft90k` | f_t,90,k LVL32 | 0.9 N/mm2 | 0.9 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.fvk` | f_v,k LVL32 | 4.5 N/mm2 | 4.5 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.gmean` | G_mean LVL32 | 600 N/mm2 | 600 N/mm2 | 0 |  |
| `ndp.en1995.strength.lvl32.rhok` | ρ_k LVL32 | 510 kg/m3 | 510 kg/m3 | 0 |  |
| `ndp.en1995.strength.lvl32.rhomean` | ρ_mean LVL32 | 540 kg/m3 | 540 kg/m3 | 0 |  |

### EN 1995-1-1 · 1.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-50.1` | n | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 1.3

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-03.1` | γM = 1.25 | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 2.00

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.00.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-04.1` | KdefTable | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 2.3.2.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.3.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-04.2` | kser | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 2.4.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-06.1` | Ed = Emean | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 5.0

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-97.1` | fr,k | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 5.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-18.2` | HoleRules | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 6.1.7

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-15.2` | Vd | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · 6.1.7(2)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.7(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.k-cr.en-constant` | k_cr EN | 0.67 | awaiting | 1 |  |

### EN 1995-1-1 · 7.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.w-fin-divisor` | w_fin limit L/n | 250 | 200 | 1 |  |
| `ndp.en1995.w-inst-divisor` | w_inst limit L/n | 300 | 300 | 1 |  |

### EN 1995-1-1 · 7.3.3

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-63.1` | f1 < 6 Hz | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · ?

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.e3-bearing-perpendicular.2` | kl | awaiting | awaiting | 0 |  |
| `ndp.en1995.t-33.1` | reinforcement | awaiting | awaiting | 0 |  |

### EN 1995-1-1 · EN 16351 / ETA (product value)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 16351 / ETA (product value).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.clt.gross-shear-strength` | characteristic gross shear strength f_v,CLT,k of CLT | 5 N/mm2 | 5 N/mm2 | 1 |  |
| `ndp.en1995.clt.rolling-shear-modulus` | rolling shear modulus G_R of the cross layers of CLT | 50 N/mm2 | 50 N/mm2 | 1 |  |
| `ndp.en1995.clt.rolling-shear-strength` | characteristic rolling shear strength f_r,k of the cross layers of CLT | 1 N/mm2 | 1 N/mm2 | 1 |  |

### EN 1995-1-1 · EN 1995-1-1 2.3.2.2 and 3.2 (model assumption)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 2.3.2.2 and 3.2 (model assumption).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.creep.time-constant-years` | time constant tau of the creep deflection curve | 5 a | 5 a | 5 |  |

### EN 1995-1-1 · EN 1995-1-1 3.2(3)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 3.2(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.kh.density-limit` | characteristic density up to which the size factor k_h of solid timber applies (rho_k <= 700 kg/m3) | 700 kg/m3 | 700 kg/m3 | 3 |  |

### EN 1995-1-1 · EN 1995-1-1 7.3.3 (3) and NA-DE

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 7.3.3 (3) and NA-DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.floor.f1-limit` | lower limit of the floor fundamental frequency | 8 Hz | 6 Hz | 5 |  |

### EN 1995-1-1 · EN 1995-1-1 7.3.3 (4) and NA-DE

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 7.3.3 (4) and NA-DE.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.floor.unit-load-a` | unit point load deflection limit a | 1.5 mm/kN | 0.5 mm/kN | 3 |  |

### EN 1995-1-1 · EN 1995-1-1 9.2.4.2 (Method B)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 9.2.4.2 (Method B).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.diaphragm.method-b-k-d` | Method B modification factor k_d of the racking resistance | 1 | 1 | 1 |  |
| `ndp.en1995.diaphragm.method-b-k-n` | Method B modification factor k_n of the racking resistance | 1 | 1 | 1 |  |
| `ndp.en1995.diaphragm.method-b-k-op` | Method B modification factor k_op of the racking resistance | 1 | 1 | 1 |  |
| `ndp.en1995.diaphragm.method-b-k-q` | Method B modification factor k_q of the racking resistance | 1 | 1 | 1 |  |

### EN 1995-1-1 · EN 1995-1-1 9.2.5.2(3)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 9.2.5.2(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.bracing.force-divisor-engineered` | k_f,2 of the stabilising force F_d = N_d / k_f,2 for glued laminated timber and LVL (9.35) | 80 | 80 | 1 |  |
| `ndp.en1995.bracing.force-divisor-solid` | k_f,1 of the stabilising force F_d = N_d / k_f,1 for solid timber (9.35) | 50 | 50 | 1 |  |

### EN 1995-1-1 · EN 1995-1-1 9.2.5.3(1)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 9.2.5.3(1).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.bracing.kl-reference-length` | reference length of k_l = min(1, sqrt(15 / l)) (9.38) | 15 m | 15 m | 1 |  |
| `ndp.en1995.bracing.stabilising-divisor` | k_f,3 of the stabilising load q_d = k_l n N_d / (k_f,3 l) (9.37) | 30 | 30 | 2 |  |

### EN 1995-1-1 · EN 1995-1-1 9.2.5.3(2)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 9.2.5.3(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.bracing.deflection-limit-divisor` | bracing deflection limit l / divisor | 500 | 500 | 1 |  |

### EN 1995-1-1 · EN 1995-1-1 Table C.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1 Table C.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.annex-c.eta.bolted` | eta bolted | 6 | 6 | 2 |  |
| `ndp.en1995.annex-c.eta.connectors` | eta connectors | 3 | 3 | 1 |  |
| `ndp.en1995.annex-c.eta.glued` | eta glued | 1 | 1 | 1 |  |
| `ndp.en1995.annex-c.eta.nailed` | eta nailed | 4 | 4 | 2 |  |

### EN 1995-1-1 · EN 1995-1-1/NA 7.3.3 NCI

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-1/NA 7.3.3 NCI.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.floor.na-de-additional-criterion` | NA-DE additional unit-load criterion applies for f1 below | 8 Hz | 8 Hz | 1 |  |

### EN 1995-1-1 · NA 6.1.7(2)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure NA 6.1.7(2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.k-cr.de-numerator` | k_cr DE numerator | awaiting | 2.5 N/mm2 | 1 |  |

### EN 1995-1-1 · Table 2.3 / NA.2.4.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.3 / NA.2.4.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.gamma-m.clt` | γ_M clt | 1.25 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.connection` | γ_M connection | 1.3 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.fibreboard` | γ_M fibreboard | 1.3 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.glulam` | γ_M glulam | 1.25 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.lvl` | γ_M lvl | 1.2 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.osb` | γ_M osb | 1.2 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.particleboard` | γ_M particleboard | 1.3 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.plywood` | γ_M plywood | 1.2 | 1.3 | 0 |  |
| `ndp.en1995.gamma-m.punched-plate` | γ_M punched-plate | 1.25 | 1.25 | 0 |  |
| `ndp.en1995.gamma-m.solid` | γ_M solid | 1.3 | 1.3 | 0 |  |

### EN 1995-1-1 · Table 3.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-02.1` | KmodTable | awaiting | awaiting | 0 |  |
| `ndp.en1995.t-02.2` | H > 1000 m | awaiting | awaiting | 0 |  |

### EN 1995-1-2 · 3.4.3

Printed source: DIN EN 1995-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 3.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-103.1` | tf | awaiting | awaiting | 0 |  |

### EN 1995-1-2 · ?

Printed source: DIN EN 1995-1-2/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-105.1` | , with  | awaiting | awaiting | 0 |  |

### EN 1995-1-2 · EN 1990-1-2 / EN 1991-1-2 2.4.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990-1-2 / EN 1991-1-2 2.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.panel.fire-load-ratio` | reduction factor eta_fi of the load in fire | 0.6 | 0.6 | 3 |  |

### EN 1995-1-2 · EN 1995-1-2 4.2.3

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-2 4.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.fire.reduced-bending-divisor` | k_mod,fi bending divisor | 200 | 200 | 2 |  |
| `ndp.en1995.fire.reduced-compression-divisor` | k_mod,fi compression divisor | 125 | 125 | 1 |  |
| `ndp.en1995.fire.reduced-tension-divisor` | k_mod,fi tension and stiffness divisor | 330 | 330 | 1 |  |
| `ndp.en1995.fire.reduced-time-limit` | t of linear k_mod,fi growth | 20 min | 20 min | 2 |  |

### EN 1995-1-2 · EN 1995-1-2 6.2

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-2 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.fire.connection-reduced-load-k` | k of t_fi,d = -ln(eta)/k | awaiting | awaiting | 2 |  |

### EN 1995-1-2 · EN 1995-1-2 CLT research model

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-2 CLT research model.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.fire.clt-fall-off-factor` | CLT char rate factor after glue line fall-off | 2 | 2 | 6 |  |
| `ndp.en1995.fire.clt-fall-off-protect` | new char depth protecting the next CLT layer | 25 mm | 25 mm | 5 |  |

### EN 1995-1-2 · EN 1995-1-2 Table 2.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-2 Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.k-fi.clt` | k_fi clt | 1.15 | 1.15 | 0 |  |
| `ndp.en1995.k-fi.glulam` | k_fi glulam | 1.15 | 1.15 | 1 |  |
| `ndp.en1995.k-fi.lvl` | k_fi lvl | 1.1 | 1.1 | 0 |  |
| `ndp.en1995.k-fi.solid` | k_fi solid | 1.25 | 1.25 | 0 |  |

### EN 1995-1-2 · EN 1995-1-2 Table 3.1

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-1-2 Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.beta-0` | β_0 | 0.65 mm/min | 0.65 mm/min | 1 |  |
| `ndp.en1995.beta-n.clt` | β_n clt | 0.65 mm/min | 0.65 mm/min | 0 |  |
| `ndp.en1995.beta-n.glulam` | β_n glulam | 0.7 mm/min | 0.7 mm/min | 0 |  |
| `ndp.en1995.beta-n.lvl` | β_n lvl | 0.7 mm/min | 0.7 mm/min | 0 |  |
| `ndp.en1995.beta-n.solid` | β_n solid | 0.8 mm/min | 0.8 mm/min | 0 |  |

### EN 1995-2 · 6.2

Printed source: DIN EN 1995-2/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.t-121.1` | Δσ > 0.2 fd | awaiting | awaiting | 0 |  |

### EN 1995-2 · EN 1995-2 7.3 / Setra

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-2 7.3 / Setra.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.bridge.crowd-factor` | crowd synchronisation factor | 10.8 | 10.8 | 1 |  |
| `ndp.en1995.bridge.crowd-force` | dynamic force per pedestrian | 280 N | 280 N | 2 |  |
| `ndp.en1995.bridge.crowd-psi-end` | psi vanishes above frequency | 2.1 Hz | 2.1 Hz | 1 |  |
| `ndp.en1995.bridge.crowd-psi-full-from` | psi equals 1 from frequency | 1 Hz | 1 Hz | 1 |  |
| `ndp.en1995.bridge.crowd-psi-full-to` | psi equals 1 up to frequency | 1.7 Hz | 1.7 Hz | 1 |  |
| `ndp.en1995.bridge.crowd-psi-start` | psi starts above frequency | 0.5 Hz | 0.5 Hz | 1 |  |

### EN 1995-2 · EN 1995-2 stress-laminated decks (Annex A cited by the catalogue)

Printed source: DIN EN 1995-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1995-2 stress-laminated decks (Annex A cited by the catalogue).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1995.deck.default-loss` | total prestress loss assumed when no staged losses are given | 0.55 | 0.55 | 1 |  |
| `ndp.en1995.deck.friction-coefficient` | friction coefficient between the laminations against slip | 0.3 | 0.3 | 3 |  |
| `ndp.en1995.deck.k-c90` | factor k_c,90 on the compressive strength perpendicular to the grain under the anchor plate | 1.5 | 1.5 | 2 |  |
| `ndp.en1995.deck.min-prestress` | minimum residual prestress of a stress-laminated deck after all losses | 0.35 N/mm2 | 0.35 N/mm2 | 3 |  |

## EN 1996 (294 unverified)

### EN 1996 · 1.0

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-10.1` | ρ2 = 1.0 | awaiting | awaiting | 0 |  |

### EN 1996 · 1.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-01.2` | SFK | awaiting | awaiting | 0 |  |

### EN 1996 · 3.1

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-01.1` | ) **[V]**,  | awaiting | awaiting | 0 |  |

### EN 1996 · 3.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-02.2` | KTable | awaiting | awaiting | 0 |  |

### EN 1996 · 3.6.1.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-02.1` | NM I, II, IIa, III, IIIa | awaiting | awaiting | 0 |  |
| `ndp.en1996.m-02.3` | fk | awaiting | awaiting | 0 |  |

### EN 1996 · 3.6.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-03.3` | fvk = fvk0 + 0.4 σDd | awaiting | awaiting | 0 |  |

### EN 1996 · 3.6.3

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-04.2` | fxk1 | awaiting | awaiting | 0 |  |

### EN 1996 · 3.7

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 3.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-06.1` | G = 0.4 E | awaiting | awaiting | 0 |  |

### EN 1996 · 5.4

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 5.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-10.2` | ρn | awaiting | awaiting | 0 |  |

### EN 1996 · 6.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-03.2` | fvlt | awaiting | awaiting | 0 |  |
| `ndp.en1996.m-20.2` | ULS: ε | awaiting | awaiting | 0 |  |

### EN 1996 · 8.5.1

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure 8.5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-51.1` | Fd = 1.5%? | awaiting | awaiting | 0 |  |

### EN 1996 · ?

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.l901.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1996.x1-fk-for-clay-group-1-g.1` | α | awaiting | awaiting | 0 |  |
| `ndp.en1996.x1-fk-for-clay-group-1-g.4` | t = 365 mm | awaiting | awaiting | 0 |  |

### EN 1996 · EN 1998-1 9.3 Table 9.1

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1998-1 9.3 Table 9.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m95.q.confined.max` | q max confined | 3 | 3 | 1 |  |
| `ndp.en1996.m95.q.confined.min` | q min confined | 2 | 2 | 1 |  |
| `ndp.en1996.m95.q.reinforced.max` | q max reinforced | 3 | 3 | 1 |  |
| `ndp.en1996.m95.q.reinforced.min` | q min reinforced | 2.5 | 2.5 | 1 |  |
| `ndp.en1996.m95.q.unreinforced.max` | q max unreinforced | 2.5 | 2.5 | 2 |  |
| `ndp.en1996.m95.q.unreinforced.min` | q min unreinforced | 1.5 | 1.5 | 2 |  |

### EN 1996 · Table 2.3

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-07.1` | . NA-DE:  | awaiting | awaiting | 0 |  |

### EN 1996 · Table 3.2

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-06.2` | φ∞ = φ(∞) | awaiting | awaiting | 0 |  |

### EN 1996 · Table 3.4

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-03.1` | 0.15 | awaiting | awaiting | 0 |  |

### EN 1996 · Table 3.6

Printed source: DIN EN 1996/NA ed. to be recorded at verification. Verify: printed table or figure Table 3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-04.1` | 0.05/0.10 … | awaiting | awaiting | 0 |  |

### EN 1996-1 · 3.6.1.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.alpha.gp` | alpha general purpose | 0.7 | 0.65 | 0 |  |
| `ndp.en1996.alpha.lw` | alpha lightweight | 0.7 | 0.65 | 0 |  |
| `ndp.en1996.alpha.tl` | alpha thin layer | 0.85 | 0.85 | 0 |  |
| `ndp.en1996.beta.gp` | beta general purpose | 0.3 | 0.25 | 0 |  |
| `ndp.en1996.beta.lw` | beta lightweight | 0.3 | 0.25 | 0 |  |
| `ndp.en1996.beta.tl` | beta thin layer | 0 | 0 | 0 |  |

### EN 1996-1 · 3.6.1.2 Table 3.3 / NA

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.1.2 Table 3.3 / NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.k.gp.aac.g1` | K general purpose aac g1 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.aac.g2` | K general purpose aac g2 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.aac.g3` | K general purpose aac g3 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.aac.g4` | K general purpose aac g4 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.casi.g1` | K general purpose casi g1 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.casi.g2` | K general purpose casi g2 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.casi.g3` | K general purpose casi g3 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.casi.g4` | K general purpose casi g4 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.clay.g1` | K general purpose clay g1 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.clay.g2` | K general purpose clay g2 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.clay.g3` | K general purpose clay g3 | 0.35 | 0.45 | 0 |  |
| `ndp.en1996.k.gp.clay.g4` | K general purpose clay g4 | 0.35 | 0.4 | 0 |  |
| `ndp.en1996.k.gp.concrete.g1` | K general purpose concrete g1 | 0.55 | 0.6 | 0 |  |
| `ndp.en1996.k.gp.concrete.g2` | K general purpose concrete g2 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.concrete.g3` | K general purpose concrete g3 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.gp.concrete.g4` | K general purpose concrete g4 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.aac.g1` | K lightweight aac g1 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.aac.g2` | K lightweight aac g2 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.aac.g3` | K lightweight aac g3 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.aac.g4` | K lightweight aac g4 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.casi.g1` | K lightweight casi g1 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.casi.g2` | K lightweight casi g2 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.casi.g3` | K lightweight casi g3 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.casi.g4` | K lightweight casi g4 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.clay.g1` | K lightweight clay g1 | 0.3 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.clay.g2` | K lightweight clay g2 | 0.3 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.clay.g3` | K lightweight clay g3 | 0.25 | 0.45 | 0 |  |
| `ndp.en1996.k.lw.clay.g4` | K lightweight clay g4 | 0.25 | 0.4 | 0 |  |
| `ndp.en1996.k.lw.concrete.g1` | K lightweight concrete g1 | 0.45 | 0.6 | 0 |  |
| `ndp.en1996.k.lw.concrete.g2` | K lightweight concrete g2 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.concrete.g3` | K lightweight concrete g3 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.lw.concrete.g4` | K lightweight concrete g4 | 0.45 | 0.5 | 0 |  |
| `ndp.en1996.k.tl.aac.g1` | K thin layer aac g1 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.aac.g2` | K thin layer aac g2 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.aac.g3` | K thin layer aac g3 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.aac.g4` | K thin layer aac g4 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.casi.g1` | K thin layer casi g1 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.casi.g2` | K thin layer casi g2 | 0.65 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.casi.g3` | K thin layer casi g3 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.casi.g4` | K thin layer casi g4 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.clay.g1` | K thin layer clay g1 | 0.75 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.clay.g2` | K thin layer clay g2 | 0.7 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.clay.g3` | K thin layer clay g3 | 0.5 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.clay.g4` | K thin layer clay g4 | 0.35 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.concrete.g1` | K thin layer concrete g1 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.concrete.g2` | K thin layer concrete g2 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.concrete.g3` | K thin layer concrete g3 | 0.8 | 0.7 | 0 |  |
| `ndp.en1996.k.tl.concrete.g4` | K thin layer concrete g4 | 0.8 | 0.7 | 0 |  |

### EN 1996-1 · 3.6.2 Table 3.4 / NA

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.2 Table 3.4 / NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.fvk0.gp.aac.ge10` | f_vk0 general purpose aac ge10 | 0.2 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.aac.ge15` | f_vk0 general purpose aac ge15 | 0.2 MPa | 0.26 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.aac.ge2-5` | f_vk0 general purpose aac ge2-5 | 0.15 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.aac.ge5` | f_vk0 general purpose aac ge5 | 0.15 MPa | 0.18 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.aac.lt2-5` | f_vk0 general purpose aac lt2-5 | 0.1 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.casi.ge10` | f_vk0 general purpose casi ge10 | 0.2 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.casi.ge15` | f_vk0 general purpose casi ge15 | 0.2 MPa | 0.26 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.casi.ge2-5` | f_vk0 general purpose casi ge2-5 | 0.15 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.casi.ge5` | f_vk0 general purpose casi ge5 | 0.15 MPa | 0.18 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.casi.lt2-5` | f_vk0 general purpose casi lt2-5 | 0.1 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.clay.ge10` | f_vk0 general purpose clay ge10 | 0.3 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.clay.ge15` | f_vk0 general purpose clay ge15 | 0.3 MPa | 0.26 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.clay.ge2-5` | f_vk0 general purpose clay ge2-5 | 0.2 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.clay.ge5` | f_vk0 general purpose clay ge5 | 0.2 MPa | 0.18 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.clay.lt2-5` | f_vk0 general purpose clay lt2-5 | 0.1 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.concrete.ge10` | f_vk0 general purpose concrete ge10 | 0.2 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.concrete.ge15` | f_vk0 general purpose concrete ge15 | 0.2 MPa | 0.26 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.concrete.ge2-5` | f_vk0 general purpose concrete ge2-5 | 0.15 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.concrete.ge5` | f_vk0 general purpose concrete ge5 | 0.15 MPa | 0.18 MPa | 0 |  |
| `ndp.en1996.fvk0.gp.concrete.lt2-5` | f_vk0 general purpose concrete lt2-5 | 0.1 MPa | 0.08 MPa | 0 |  |
| `ndp.en1996.fvk0.lw.aac.all` | f_vk0 lightweight aac all | 0.15 MPa | 0.15 MPa | 0 |  |
| `ndp.en1996.fvk0.lw.casi.all` | f_vk0 lightweight casi all | 0.15 MPa | 0.15 MPa | 0 |  |
| `ndp.en1996.fvk0.lw.clay.all` | f_vk0 lightweight clay all | 0.15 MPa | 0.15 MPa | 0 |  |
| `ndp.en1996.fvk0.lw.concrete.all` | f_vk0 lightweight concrete all | 0.15 MPa | 0.15 MPa | 0 |  |
| `ndp.en1996.fvk0.tl.aac.all` | f_vk0 thin layer aac all | 0.3 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.tl.casi.all` | f_vk0 thin layer casi all | 0.4 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.tl.clay.all` | f_vk0 thin layer clay all | 0.3 MPa | 0.22 MPa | 0 |  |
| `ndp.en1996.fvk0.tl.concrete.all` | f_vk0 thin layer concrete all | 0.3 MPa | 0.22 MPa | 0 |  |

### EN 1996-1 · 3.6.3 Table 3.6

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.3 Table 3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.fxk.x1.aac.strong` | f_xk1 aac f_m >= 5 | 0.1 MPa | 0.1 MPa | 0 |  |
| `ndp.en1996.fxk.x1.aac.weak` | f_xk1 aac f_m < 5 | 0.05 MPa | 0.05 MPa | 0 |  |
| `ndp.en1996.fxk.x1.casi.strong` | f_xk1 casi f_m >= 5 | 0.1 MPa | 0.1 MPa | 0 |  |
| `ndp.en1996.fxk.x1.casi.weak` | f_xk1 casi f_m < 5 | 0.05 MPa | 0.05 MPa | 0 |  |
| `ndp.en1996.fxk.x1.clay.strong` | f_xk1 clay f_m >= 5 | 0.1 MPa | 0.1 MPa | 0 |  |
| `ndp.en1996.fxk.x1.clay.weak` | f_xk1 clay f_m < 5 | 0.1 MPa | 0.1 MPa | 0 |  |
| `ndp.en1996.fxk.x1.concrete.strong` | f_xk1 concrete f_m >= 5 | 0.1 MPa | 0.1 MPa | 0 |  |
| `ndp.en1996.fxk.x1.concrete.weak` | f_xk1 concrete f_m < 5 | 0.05 MPa | 0.05 MPa | 0 |  |
| `ndp.en1996.fxk.x2.aac.strong` | f_xk2 aac f_m >= 5 | 0.2 MPa | 0.2 MPa | 0 |  |
| `ndp.en1996.fxk.x2.aac.weak` | f_xk2 aac f_m < 5 | 0.2 MPa | 0.2 MPa | 0 |  |
| `ndp.en1996.fxk.x2.casi.strong` | f_xk2 casi f_m >= 5 | 0.4 MPa | 0.4 MPa | 0 |  |
| `ndp.en1996.fxk.x2.casi.weak` | f_xk2 casi f_m < 5 | 0.2 MPa | 0.2 MPa | 0 |  |
| `ndp.en1996.fxk.x2.clay.strong` | f_xk2 clay f_m >= 5 | 0.4 MPa | 0.4 MPa | 0 |  |
| `ndp.en1996.fxk.x2.clay.weak` | f_xk2 clay f_m < 5 | 0.2 MPa | 0.2 MPa | 0 |  |
| `ndp.en1996.fxk.x2.concrete.strong` | f_xk2 concrete f_m >= 5 | 0.4 MPa | 0.4 MPa | 0 |  |
| `ndp.en1996.fxk.x2.concrete.weak` | f_xk2 concrete f_m < 5 | 0.2 MPa | 0.2 MPa | 0 |  |

### EN 1996-1 · 3.7.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.k-e` | K_E | 1000 | 950 | 1 |  |

### EN 1996-1-1 · 2.5

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-50.1` | n ≥ 7 | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · 3.6.1.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 3.6.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-02.4` | HLz 12 NM IIa | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · 6.1.2.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m12.eccentricity-limit` | e_decl <= x t | 0.2 t | 0.2 t | 2 |  |

### EN 1996-1-1 · 6.17

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.17.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-32.1` | qlat,d = fd t²/ la² | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · 6.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-20.1` | bef | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · 6.3.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-31.1` | α | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · 7.3.2

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 7.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-61.1` | l/d | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · ?

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.sei-52.1` | claims_simple_masonry | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · Annex E

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Annex E.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.annex-e.ffff` | alpha1 cells FFFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fffs` | alpha1 cells FFFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fffx` | alpha1 cells FFFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ffss` | alpha1 cells FFSS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ffsx` | alpha1 cells FFSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ffxx` | alpha1 cells FFXX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fsff` | alpha1 cells FSFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fsfs` | alpha1 cells FSFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fsfx` | alpha1 cells FSFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fsss` | alpha1 cells FSSS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fssx` | alpha1 cells FSSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fsxx` | alpha1 cells FSXX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxff` | alpha1 cells FXFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxfs` | alpha1 cells FXFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxfx` | alpha1 cells FXFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxss` | alpha1 cells FXSS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxsx` | alpha1 cells FXSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.fxxx` | alpha1 cells FXXX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ssff` | alpha1 cells SSFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ssfs` | alpha1 cells SSFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ssfx` | alpha1 cells SSFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ssss` | alpha1 cells SSSS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sssx` | alpha1 cells SSSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.ssxx` | alpha1 cells SSXX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sxff` | alpha1 cells SXFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sxfs` | alpha1 cells SXFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sxfx` | alpha1 cells SXFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sxss` | alpha1 cells SXSS | awaiting | awaiting | 2 |  |
| `ndp.en1996.annex-e.sxsx` | alpha1 cells SXSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.sxxx` | alpha1 cells SXXX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxff` | alpha1 cells XXFF | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxfs` | alpha1 cells XXFS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxfx` | alpha1 cells XXFX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxss` | alpha1 cells XXSS | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxsx` | alpha1 cells XXSX | awaiting | awaiting | 1 |  |
| `ndp.en1996.annex-e.xxxx` | alpha1 cells XXXX | awaiting | awaiting | 1 |  |

### EN 1996-1-1 · EN 1990 Table A1.1

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 Table A1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.psi` | psi | awaiting | awaiting | 1 |  |

### EN 1996-1-1 · EN 1996-1-1 3.1 / DE NA

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1996-1-1 3.1 / DE NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m01.conformance` | conformance rules | awaiting | awaiting | 1 |  |

### EN 1996-1-1 · EN 1998-1 4.3.1(4)

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1998-1 4.3.1(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m95.diaphragm-ratio` | u_max / u_rigid | 1.1 1 | 1.1 1 | 3 |  |

### EN 1996-1-1 · M-01

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-01.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m01.delta` | m01.delta | awaiting | awaiting | 4 |  |

### EN 1996-1-1 · M-02

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-02.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m02.caps` | m02.caps | awaiting | awaiting | 3 |  |
| `ndp.en1996.m02.k-de` | m02.k-de | awaiting | awaiting | 1 |  |
| `ndp.en1996.m02.k-en` | m02.k-en | awaiting | awaiting | 1 |  |

### EN 1996-1-1 · M-03

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-03.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m03.fvk-cap-en` | m03.fvk-cap-en | awaiting | awaiting | 1 |  |
| `ndp.en1996.m03.fvk0` | m03.fvk0 | awaiting | awaiting | 2 |  |
| `ndp.en1996.m03.fvlt-de` | m03.fvlt-de | awaiting | awaiting | 1 |  |
| `ndp.en1996.m03.head-joint` | m03.head-joint | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · M-04

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-04.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m04.fxk` | m04.fxk | awaiting | awaiting | 6 |  |

### EN 1996-1-1 · M-05

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-05.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m05.fbok` | m05.fbok | awaiting | awaiting | 4 |  |

### EN 1996-1-1 · M-06

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-06.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m06.creep` | m06.creep | awaiting | awaiting | 5 |  |
| `ndp.en1996.m06.e` | m06.e | awaiting | awaiting | 5 |  |

### EN 1996-1-1 · M-07

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-07.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m07.gamma` | m07.gamma | awaiting | awaiting | 10 |  |

### EN 1996-1-1 · M-10

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m10.rho2` | m10.rho2 | awaiting | awaiting | 7 |  |
| `ndp.en1996.m10.stiffener` | m10.stiffener | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-11

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m11.beta-floor` | m11.beta-floor | awaiting | awaiting | 2 |  |
| `ndp.en1996.m11.limit` | m11.limit | awaiting | awaiting | 4 |  |

### EN 1996-1-1 · M-12

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-12.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m12.einit` | m12.einit | awaiting | awaiting | 5 |  |

### EN 1996-1-1 · M-13

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-13.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m13.phi1` | m13.phi1 | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-14

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-14.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m14.beta` | m14.beta | awaiting | awaiting | 6 |  |

### EN 1996-1-1 · M-15

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-15.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m15.lintel` | m15.lintel | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-20

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-20.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m20.flange` | m20.flange | awaiting | awaiting | 3 |  |
| `ndp.en1996.m20.stability` | m20.stability | awaiting | awaiting | 4 |  |

### EN 1996-1-1 · M-21

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-21.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m21.torsion` | m21.torsion | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · M-31

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-31.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m31.fixed-hogging` | m31.fixed-hogging | awaiting | awaiting | 1 |  |
| `ndp.en1996.m31.yield-line` | m31.yield-line | awaiting | awaiting | 5 |  |

### EN 1996-1-1 · M-32

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-32.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m32.arching` | m32.arching | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-34

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-34.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m34.basement` | m34.basement | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-40

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-40.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m40.flexure` | m40.flexure | awaiting | awaiting | 6 |  |

### EN 1996-1-1 · M-41

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-41.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m41.shear` | m41.shear | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-42

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-42.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m42.deep` | m42.deep | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-43

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-43.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m43.confined` | m43.confined | awaiting | awaiting | 0 |  |
| `ndp.en1996.m43.prestress` | m43.prestress | awaiting | awaiting | 0 |  |

### EN 1996-1-1 · M-45

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-45.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m45.bond` | m45.bond | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-50

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-50.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m50.ties` | m50.ties | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-51

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-51.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m51.anchor` | m51.anchor | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-52

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-52.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m52.joints` | m52.joints | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-60

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-60.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m60.exposure` | m60.exposure | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-61

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-61.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m61.slender` | m61.slender | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-70

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-70.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m70.execution` | m70.execution | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · M-80

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-80.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m80.limits` | m80.limits | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-81

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-81.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m81.phis` | m81.phis | awaiting | awaiting | 7 |  |

### EN 1996-1-1 · M-82

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-82.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m82.shear` | m82.shear | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · M-90

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-90.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m90.fire` | m90.fire | awaiting | awaiting | 4 |  |

### EN 1996-1-1 · M-91

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure M-91.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m91.fire` | m91.fire | awaiting | awaiting | 2 |  |

### EN 1996-1-1 · NA.2.4.3

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure NA.2.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.gamma-m.accidental` | γ_M accidental | awaiting | 1.3 | 2 |  |

### EN 1996-1-1 · Table 2.1

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.durability.mortar` | durability.mortar | awaiting | awaiting | 3 |  |

### EN 1996-1-1 · Table 2.3 / NA.2.4.3

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table 2.3 / NA.2.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.gamma-m.c1` | γ_M class 1 | 1.5 | 1.5 | 2 |  |
| `ndp.en1996.gamma-m.c2` | γ_M class 2 | 1.7 | 1.7 | 1 |  |
| `ndp.en1996.gamma-m.c3` | γ_M class 3 | 2 | 1.7 | 1 |  |
| `ndp.en1996.gamma-m.c4` | γ_M class 4 | 2.2 | 2 | 1 |  |
| `ndp.en1996.gamma-m.c5` | γ_M class 5 | 2.5 | 2 | 1 |  |

### EN 1996-1-1 · Table A.1

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Table A.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.delta.h100.w100` | delta h=100 w=100 | 1 | 1 | 0 |  |
| `ndp.en1996.delta.h100.w150` | delta h=100 w=150 | 0.9 | 0.9 | 0 |  |
| `ndp.en1996.delta.h100.w200` | delta h=100 w=200 | 0.8 | 0.8 | 0 |  |
| `ndp.en1996.delta.h100.w250` | delta h=100 w=250 | 0.75 | 0.75 | 0 |  |
| `ndp.en1996.delta.h100.w50` | delta h=100 w=50 | 1.15 | 1.15 | 0 |  |
| `ndp.en1996.delta.h150.w100` | delta h=150 w=100 | 1.2 | 1.2 | 0 |  |
| `ndp.en1996.delta.h150.w150` | delta h=150 w=150 | 1.1 | 1.1 | 0 |  |
| `ndp.en1996.delta.h150.w200` | delta h=150 w=200 | 1 | 1 | 0 |  |
| `ndp.en1996.delta.h150.w250` | delta h=150 w=250 | 0.95 | 0.95 | 0 |  |
| `ndp.en1996.delta.h150.w50` | delta h=150 w=50 | 1.3 | 1.3 | 0 |  |
| `ndp.en1996.delta.h200.w100` | delta h=200 w=100 | 1.35 | 1.35 | 0 |  |
| `ndp.en1996.delta.h200.w150` | delta h=200 w=150 | 1.25 | 1.25 | 0 |  |
| `ndp.en1996.delta.h200.w200` | delta h=200 w=200 | 1.15 | 1.15 | 0 |  |
| `ndp.en1996.delta.h200.w250` | delta h=200 w=250 | 1.1 | 1.1 | 0 |  |
| `ndp.en1996.delta.h200.w50` | delta h=200 w=50 | 1.45 | 1.45 | 0 |  |
| `ndp.en1996.delta.h250.w100` | delta h=250 w=100 | 1.45 | 1.45 | 0 |  |
| `ndp.en1996.delta.h250.w150` | delta h=250 w=150 | 1.35 | 1.35 | 0 |  |
| `ndp.en1996.delta.h250.w200` | delta h=250 w=200 | 1.25 | 1.25 | 0 |  |
| `ndp.en1996.delta.h250.w250` | delta h=250 w=250 | 1.15 | 1.15 | 0 |  |
| `ndp.en1996.delta.h250.w50` | delta h=250 w=50 | 1.55 | 1.55 | 0 |  |
| `ndp.en1996.delta.h50.w100` | delta h=50 w=100 | 0.75 | 0.75 | 0 |  |
| `ndp.en1996.delta.h50.w150` | delta h=50 w=150 | 0.7 | 0.7 | 0 |  |
| `ndp.en1996.delta.h50.w200` | delta h=50 w=200 | 0.7 | 0.7 | 0 |  |
| `ndp.en1996.delta.h50.w250` | delta h=50 w=250 | 0.7 | 0.7 | 0 |  |
| `ndp.en1996.delta.h50.w50` | delta h=50 w=50 | 0.85 | 0.85 | 0 |  |
| `ndp.en1996.delta.h65.w100` | delta h=65 w=100 | 0.85 | 0.85 | 0 |  |
| `ndp.en1996.delta.h65.w150` | delta h=65 w=150 | 0.75 | 0.75 | 0 |  |
| `ndp.en1996.delta.h65.w200` | delta h=65 w=200 | 0.7 | 0.7 | 0 |  |
| `ndp.en1996.delta.h65.w250` | delta h=65 w=250 | 0.65 | 0.65 | 0 |  |
| `ndp.en1996.delta.h65.w50` | delta h=65 w=50 | 0.95 | 0.95 | 0 |  |

### EN 1996-1-2 · 3.3

Printed source: DIN EN 1996-1-2/NA ed. to be recorded at verification. Verify: printed table or figure 3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-91.1` | 5 % rule | awaiting | awaiting | 0 |  |

### EN 1996-1-2 · ?

Printed source: DIN EN 1996-1-2/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-90.1` |  and  | awaiting | awaiting | 0 |  |

### EN 1996-1-2 · Annex B

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure Annex B.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.fire.aac.rei120` | t_min aac REI 120 | 200 mm | 200 mm | 0 |  |
| `ndp.en1996.fire.aac.rei180` | t_min aac REI 180 | 250 mm | 250 mm | 0 |  |
| `ndp.en1996.fire.aac.rei240` | t_min aac REI 240 | 300 mm | 300 mm | 0 |  |
| `ndp.en1996.fire.aac.rei30` | t_min aac REI 30 | 115 mm | 115 mm | 0 |  |
| `ndp.en1996.fire.aac.rei60` | t_min aac REI 60 | 140 mm | 140 mm | 0 |  |
| `ndp.en1996.fire.aac.rei90` | t_min aac REI 90 | 175 mm | 175 mm | 0 |  |
| `ndp.en1996.fire.casi.rei120` | t_min casi REI 120 | 175 mm | 175 mm | 0 |  |
| `ndp.en1996.fire.casi.rei180` | t_min casi REI 180 | 240 mm | 240 mm | 0 |  |
| `ndp.en1996.fire.casi.rei240` | t_min casi REI 240 | 300 mm | 300 mm | 0 |  |
| `ndp.en1996.fire.casi.rei30` | t_min casi REI 30 | 90 mm | 90 mm | 0 |  |
| `ndp.en1996.fire.casi.rei60` | t_min casi REI 60 | 115 mm | 115 mm | 0 |  |
| `ndp.en1996.fire.casi.rei90` | t_min casi REI 90 | 140 mm | 140 mm | 0 |  |
| `ndp.en1996.fire.clay.rei120` | t_min clay REI 120 | 175 mm | 175 mm | 0 |  |
| `ndp.en1996.fire.clay.rei180` | t_min clay REI 180 | 240 mm | 240 mm | 0 |  |
| `ndp.en1996.fire.clay.rei240` | t_min clay REI 240 | 240 mm | 240 mm | 0 |  |
| `ndp.en1996.fire.clay.rei30` | t_min clay REI 30 | 100 mm | 100 mm | 0 |  |
| `ndp.en1996.fire.clay.rei60` | t_min clay REI 60 | 100 mm | 100 mm | 0 |  |
| `ndp.en1996.fire.clay.rei90` | t_min clay REI 90 | 150 mm | 150 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei120` | t_min concrete REI 120 | 170 mm | 170 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei180` | t_min concrete REI 180 | 240 mm | 240 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei240` | t_min concrete REI 240 | 300 mm | 300 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei30` | t_min concrete REI 30 | 90 mm | 90 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei60` | t_min concrete REI 60 | 100 mm | 100 mm | 0 |  |
| `ndp.en1996.fire.concrete.rei90` | t_min concrete REI 90 | 140 mm | 140 mm | 0 |  |

### EN 1996-2 · 8.6

Printed source: DIN EN 1996-2/NA ed. to be recorded at verification. Verify: printed table or figure 8.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-70.2` | chase_ok | awaiting | awaiting | 0 |  |

### EN 1996-2 · ?

Printed source: DIN EN 1996-2/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-60.1` | Frost-Beständigkeit | awaiting | awaiting | 0 |  |
| `ndp.en1996.m-70.1` | ? | awaiting | awaiting | 0 |  |

### EN 1996-2 · EN 1996-2 3.6 / 8

Printed source: DIN EN 1996-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1996-2 3.6 / 8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m70.bed-joint.general.max` | t_bed max (general) | 0.015 m | 0.015 m | 0 |  |
| `ndp.en1996.m70.bed-joint.general.min` | t_bed min (general) | 0.006 m | 0.006 m | 0 |  |
| `ndp.en1996.m70.bed-joint.thin.max` | t_bed max (thin) | 0.003 m | 0.003 m | 0 |  |
| `ndp.en1996.m70.bed-joint.thin.min` | t_bed min (thin) | 0.0005 m | 0.0005 m | 0 |  |

### EN 1996-3 · 2.5

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.x1-fk-for-clay-group-1-g.3` | t = 0.365 | awaiting | awaiting | 0 |  |

### EN 1996-3 · 2.6

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 2.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-34.1` | N1,Rd | awaiting | awaiting | 0 |  |

### EN 1996-3 · 2.75

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 2.75.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.x1-fk-for-clay-group-1-g.2` | t = 240 mm | awaiting | awaiting | 0 |  |

### EN 1996-3 · 4.2.2.3

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 4.2.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-81.1` | Φ1, Φ2, Φ3 | awaiting | awaiting | 0 |  |

### EN 1996-3 · 5.5.1.4

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 5.5.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-11.1` | hef/t ≤ 21 | awaiting | awaiting | 0 |  |

### EN 1996-3 · 6.0

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure 6.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.m-80.1` | H ≤ 20 m | awaiting | awaiting | 0 |  |

### EN 1996-3 · ?

Printed source: DIN EN 1996-3/NA ed. to be recorded at verification. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1996.l903.1` | kmod | awaiting | awaiting | 0 |  |

## EN 1997 (126 unverified)

### EN 1997 · 1.0

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l136.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · 1.25

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 1.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l316.1` | F_s >= 1/1.4 | awaiting | awaiting | 0 |  |

### EN 1997 · 1.5

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l318.1` | η >= 1.5 | awaiting | awaiting | 0 |  |

### EN 1997 · 2.14

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 2.14.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l323.1` | R = 575 s sqrt(H k) | awaiting | awaiting | 0 |  |

### EN 1997 · 2.5

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l145.1` | μ = E_dst,d/E_stb,d <= 1 | awaiting | awaiting | 0 |  |

### EN 1997 · 2.8

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 2.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l166.1` | σ0 = q - γ d | awaiting | awaiting | 0 |  |
| `ndp.en1997.l170.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · 7.1

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 7.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l183.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · 9.81

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 9.81.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l97.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · ?

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l212.1` | s >= 3 D | awaiting | awaiting | 0 |  |
| `ndp.en1997.rw-01.1` | δa = 2/3 φ' | awaiting | awaiting | 0 |  |
| `ndp.en1997.rw-02.1` | ; cohesion  | awaiting | awaiting | 0 |  |
| `ndp.en1997.rw-03.1` | [V] | awaiting | awaiting | 0 |  |
| `ndp.en1997.rw-05.1` |  (strip), line load  | awaiting | awaiting | 0 |  |
| `ndp.en1997.rw-07.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · Table A2.1

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure Table A2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l45.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · Table A2.2

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure Table A2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l62.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997 · Table A2.3

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure Table A2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l72.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1997-1 · 1.60

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 1.60.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l220.1` | R_c;m | awaiting | awaiting | 0 |  |

### EN 1997-1 · 2.3

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l125.1` | m | awaiting | awaiting | 0 |  |

### EN 1997-1 · 6.10

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 6.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l128.1` | μ <= 1 | awaiting | awaiting | 0 |  |

### EN 1997-1 · 7.7

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure 7.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.7.7.lateral-deflection-limit` | y_lim | 0.01 m | awaiting | 1 |  |

### EN 1997-1 · 9.7

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure 9.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.9.blum-embedment-increase` | t/t0 | 1.2 | awaiting | 1 |  |

### EN 1997-1 · DIN 1054 GZ 1C

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure DIN 1054 GZ 1C.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.de.geo3.a.q` | γ_Q GEO-3 accidental | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.geo3.p.q` | γ_Q GEO-3 persistent | awaiting | 1.3 | 0 |  |
| `ndp.en1997.de.geo3.t.q` | γ_Q GEO-3 transient | awaiting | 1.2 | 0 |  |

### EN 1997-1 · DIN 1054 Table A 2.1

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure DIN 1054 Table A 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.de.act.a.g` | γ_G accidental | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.act.a.q` | γ_Q accidental | awaiting | 1.15 | 0 |  |
| `ndp.en1997.de.act.p.g` | γ_G persistent | awaiting | 1.35 | 0 |  |
| `ndp.en1997.de.act.p.q` | γ_Q persistent | awaiting | 1.5 | 0 |  |
| `ndp.en1997.de.act.t.g` | γ_G transient | awaiting | 1.2 | 0 |  |
| `ndp.en1997.de.act.t.q` | γ_Q transient | awaiting | 1.3 | 0 |  |

### EN 1997-1 · DIN 1054 Table A 2.2

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure DIN 1054 Table A 2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.de.soil.a.f` | γ_φ=γ_c=γ_cu accidental | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.soil.p.f` | γ_φ=γ_c=γ_cu persistent | awaiting | 1.25 | 0 |  |
| `ndp.en1997.de.soil.t.f` | γ_φ=γ_c=γ_cu transient | awaiting | 1.15 | 0 |  |

### EN 1997-1 · DIN 1054 Table A 2.3

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure DIN 1054 Table A 2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.de.res.a` | γ_a DE | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.res.a.re` | γ_R,e accidental | awaiting | 1.2 | 0 |  |
| `ndp.en1997.de.res.a.rv` | γ_R,v accidental | awaiting | 1.2 | 0 |  |
| `ndp.en1997.de.res.b` | γ_b DE | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.res.p.re` | γ_R,e persistent | awaiting | 1.4 | 0 |  |
| `ndp.en1997.de.res.p.rv` | γ_R,v persistent | awaiting | 1.4 | 0 |  |
| `ndp.en1997.de.res.rh` | γ_rh DE | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.res.s` | γ_s DE | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.res.st` | γ_st DE | awaiting | 1.15 | 0 |  |
| `ndp.en1997.de.res.t` | γ_t DE | awaiting | 1.1 | 0 |  |
| `ndp.en1997.de.res.t.re` | γ_R,e transient | awaiting | 1.3 | 0 |  |
| `ndp.en1997.de.res.t.rv` | γ_R,v transient | awaiting | 1.3 | 0 |  |

### EN 1997-1 · DIN 1054 Table A.6.10

Printed source: DIN EN 1997/NA ed. to be recorded at verification. Verify: printed table or figure DIN 1054 Table A.6.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.gk1.presumptive.clay` | GK1 presumptive bearing clay | 120000 Pa | 120000 Pa | 0 |  |
| `ndp.en1997.gk1.presumptive.embedment-cap` | GK1 presumptive bearing embedment-cap | 3 m | 3 m | 0 |  |
| `ndp.en1997.gk1.presumptive.embedment-gain` | GK1 presumptive bearing embedment-gain | 0.1 1/m | 0.1 1/m | 0 |  |
| `ndp.en1997.gk1.presumptive.gravel` | GK1 presumptive bearing gravel | 400000 Pa | 400000 Pa | 0 |  |
| `ndp.en1997.gk1.presumptive.other` | GK1 presumptive bearing other | 180000 Pa | 180000 Pa | 0 |  |
| `ndp.en1997.gk1.presumptive.sand` | GK1 presumptive bearing sand | 200000 Pa | 200000 Pa | 0 |  |
| `ndp.en1997.gk1.presumptive.silt` | GK1 presumptive bearing silt | 150000 Pa | 150000 Pa | 0 |  |

### EN 1997-1 · EN 1997-1 2.4.7

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure EN 1997-1 2.4.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.a-acc.g` | γ_G accidental | 1 | 1 | 0 |  |
| `ndp.en1997.a-acc.q` | γ_Q accidental | 1 | 1 | 0 |  |

### EN 1997-1 · EN 1998-5 7.3.2

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 7.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.9.seismic.gamma-sliding` | gamma_Rh,E | 1 | awaiting | 1 |  |

### EN 1997-1 · Table A.3/A.4

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure Table A.3/A.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.a1.g` | γ_G A1 | 1.35 | 1.35 | 0 |  |
| `ndp.en1997.a1.q` | γ_Q A1 | 1.5 | 1.5 | 0 |  |
| `ndp.en1997.a2.g` | γ_G A2 | 1 | 1 | 0 |  |
| `ndp.en1997.a2.q` | γ_Q A2 | 1.3 | 1.3 | 0 |  |

### EN 1997-1 · Table A.4

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure Table A.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.m1.c` | γ_c M1 | 1 | 1 | 0 |  |
| `ndp.en1997.m1.cu` | γ_cu M1 | 1 | 1 | 0 |  |
| `ndp.en1997.m1.gamma` | γ_gamma M1 | 1 | 1 | 0 |  |
| `ndp.en1997.m1.phi` | γ_phi M1 | 1 | 1 | 0 |  |
| `ndp.en1997.m2.c` | γ_c M2 | 1.25 | 1.25 | 0 |  |
| `ndp.en1997.m2.cu` | γ_cu M2 | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.m2.gamma` | γ_gamma M2 | 1 | 1 | 0 |  |
| `ndp.en1997.m2.phi` | γ_phi M2 | 1.25 | 1.25 | 0 |  |

### EN 1997-1 · Tables A.1, A.2, A.17 / A 2.1

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure Tables A.1, A.2, A.17 / A 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.upl.a.equdst` | equdst accidental | 1 | 1 | 0 |  |
| `ndp.en1997.upl.a.equq` | equq accidental | 1 | 1.15 | 0 |  |
| `ndp.en1997.upl.a.equstb` | equstb accidental | 1 | 0.95 | 0 |  |
| `ndp.en1997.upl.a.gdst` | gdst accidental | 1 | 1 | 0 |  |
| `ndp.en1997.upl.a.gstb` | gstb accidental | 1 | 0.95 | 0 |  |
| `ndp.en1997.upl.a.hyd` | hyd accidental | 1 | 1.3 | 0 |  |
| `ndp.en1997.upl.a.hydstb` | hydstb accidental | 1 | 0.95 | 0 |  |
| `ndp.en1997.upl.a.qdst` | qdst accidental | 1 | 1.15 | 0 |  |
| `ndp.en1997.upl.p.equdst` | equdst persistent | 1.1 | 1.05 | 0 |  |
| `ndp.en1997.upl.p.equq` | equq persistent | 1.5 | 1.5 | 0 |  |
| `ndp.en1997.upl.p.equstb` | equstb persistent | 0.9 | 0.95 | 0 |  |
| `ndp.en1997.upl.p.gdst` | gdst persistent | 1 | 1 | 0 |  |
| `ndp.en1997.upl.p.gstb` | gstb persistent | 0.9 | 0.9 | 0 |  |
| `ndp.en1997.upl.p.hyd` | hyd persistent | 1.35 | 1.8 | 0 |  |
| `ndp.en1997.upl.p.hydstb` | hydstb persistent | 0.9 | 0.9 | 0 |  |
| `ndp.en1997.upl.p.qdst` | qdst persistent | 1.5 | 1.5 | 0 |  |
| `ndp.en1997.upl.t.equdst` | equdst transient | 1.1 | 1.05 | 0 |  |
| `ndp.en1997.upl.t.equq` | equq transient | 1.5 | 1.3 | 0 |  |
| `ndp.en1997.upl.t.equstb` | equstb transient | 0.9 | 0.95 | 0 |  |
| `ndp.en1997.upl.t.gdst` | gdst transient | 1 | 1 | 0 |  |
| `ndp.en1997.upl.t.gstb` | gstb transient | 0.9 | 0.9 | 0 |  |
| `ndp.en1997.upl.t.hyd` | hyd transient | 1.35 | 1.45 | 0 |  |
| `ndp.en1997.upl.t.hydstb` | hydstb transient | 0.9 | 0.9 | 0 |  |
| `ndp.en1997.upl.t.qdst` | qdst transient | 1.5 | 1.3 | 0 |  |

### EN 1997-1 · Tables A.5-A.14

Printed source: DIN 1054:2021-04 ed. to be recorded at verification. Verify: printed table or figure Tables A.5-A.14.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.r1.a` | γ_a;t R1 (prestressed anchorage, temporary) | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r1.b` | γ_b R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r1.re` | γ_re R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r1.rh` | γ_rh R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r1.rv` | γ_rv R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r1.s` | γ_s R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r1.st` | γ_st R1 | 1.25 | 1.25 | 0 |  |
| `ndp.en1997.r1.t` | γ_t R1 | 1 | 1 | 0 |  |
| `ndp.en1997.r2.a` | γ_a;t R2 (prestressed anchorage, temporary) | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r2.b` | γ_b R2 | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r2.re` | γ_re R2 | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.r2.rh` | γ_rh R2 | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r2.rv` | γ_rv R2 | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.r2.s` | γ_s R2 | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r2.st` | γ_st R2 | 1.15 | 1.15 | 0 |  |
| `ndp.en1997.r2.t` | γ_t R2 | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r3.a` | γ_a;t R3 (prestressed anchorage, temporary) | 1 | 1.1 | 0 | EN verified; NA-DE unverified |
| `ndp.en1997.r3.b` | γ_b R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r3.re` | γ_re R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r3.rh` | γ_rh R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r3.rv` | γ_rv R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r3.s` | γ_s R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r3.st` | γ_st R3 | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r3.t` | γ_t R3 | 1 | 1 | 0 |  |
| `ndp.en1997.r4.a` | γ_a;t R4 (prestressed anchorage, temporary) | 1.1 | 1.1 | 0 |  |
| `ndp.en1997.r4.b` | γ_b R4 | 1.3 | 1.3 | 0 |  |
| `ndp.en1997.r4.re` | γ_re R4 | 1 | 1 | 0 |  |
| `ndp.en1997.r4.rh` | γ_rh R4 | 1 | 1 | 0 |  |
| `ndp.en1997.r4.rv` | γ_rv R4 | 1 | 1 | 0 |  |
| `ndp.en1997.r4.s` | γ_s R4 | 1.3 | 1.3 | 0 |  |
| `ndp.en1997.r4.st` | γ_st R4 | 1.6 | 1.6 | 0 |  |
| `ndp.en1997.r4.t` | γ_t R4 | 1.3 | 1.3 | 0 |  |

### EN 1997-2 · 2.2

Printed source: DIN 1054:2021-04 ed. 2021-04 (spec cites DIN 1054:2010-12, withdrawn; re-base then print check). Verify: printed table or figure 2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.l292.1` | z_a >= max(6 m, 2 B) | awaiting | awaiting | 0 |  |

## EN 1998 (253 unverified)

### EN 1998 · 1.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-46.1` | V_diaph,d = γd 1.3 ... | awaiting | awaiting | 0 |  |

### EN 1998 · 1.5

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-30.1` | q >= 1.5 | awaiting | awaiting | 0 |  |
| `ndp.en1998.sei-30.3` | q <= 1.5 | awaiting | awaiting | 0 |  |
| `ndp.en1998.sei-49.1` | F_a = 1.5 γ_a α S W_a | awaiting | awaiting | 0 |  |

### EN 1998 · 3.2.2.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 3.2.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.vertical-spectrum.avg-ratio` | a_vg/a_g | 0.9 | 0.9 | 1 |  |

### EN 1998 · 3.2.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-11.1` | ψ2 = 0 | awaiting | awaiting | 0 |  |

### EN 1998 · 4.2.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 4.2.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-20.1` | r_x >= l_s | awaiting | awaiting | 0 |  |

### EN 1998 · 4.3.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 4.3.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-40.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998 · 4.3.3.2.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.3.3.2.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en.torsion.simplified-bound` | δ max | 1.6 | 1.6 | 1 |  |

### EN 1998 · 4.4.2.7

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 4.4.2.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-47.1` | d_old = 0.01 H_old | awaiting | awaiting | 0 |  |

### EN 1998 · 4.4.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 4.4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-45.1` | ν = 0.5 | awaiting | awaiting | 0 |  |

### EN 1998 · 8.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 8.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.timber.max-diameter-mm` | d max | 12 mm | 12 mm | 3 |  |
| `ndp.en1998.en1.timber.min-thickness-factor` | t/d min | 8 | 8 | 3 |  |

### EN 1998 · ?

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l779.1` | [V] | awaiting | awaiting | 0 |  |
| `ndp.en1998.l791.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998 · EN 1992-1-1 3.2.7(4)

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1992-1-1 3.2.7(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en3.annex-a.steel-modulus-mpa` | E_s | 200000 MPa | 200000 MPa | 2 |  |

### EN 1998 · EN 1997-1 Annex H.2

Printed source: DIN EN 1997-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1997-1 Annex H.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.sls.angular-distortion-limit` | limit of the angular distortion / relative rotation | 0.002 | 0.002 | 3 |  |
| `ndp.en1997.sls.tilt-limit` | limit of the structure tilt | 0.004 | 0.004 | 2 |  |

### EN 1998 · EN 1997-1 Table A.10

Printed source: DIN EN 1997-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1997-1 Table A.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.xi.ground.n1.3` | xi3, n = 1 (ground-test profiles) | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.xi.ground.n1.4` | xi4, n = 1 (ground-test profiles) | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.xi.ground.n10.3` | xi3, n = 10 (ground-test profiles) | 1.25 | 1.25 | 0 |  |
| `ndp.en1997.xi.ground.n10.4` | xi4, n = 10 (ground-test profiles) | 1.08 | 1.08 | 0 |  |
| `ndp.en1997.xi.ground.n2.3` | xi3, n = 2 (ground-test profiles) | 1.35 | 1.35 | 0 |  |
| `ndp.en1997.xi.ground.n2.4` | xi4, n = 2 (ground-test profiles) | 1.27 | 1.27 | 0 |  |
| `ndp.en1997.xi.ground.n3.3` | xi3, n = 3 (ground-test profiles) | 1.33 | 1.33 | 0 |  |
| `ndp.en1997.xi.ground.n3.4` | xi4, n = 3 (ground-test profiles) | 1.23 | 1.23 | 0 |  |
| `ndp.en1997.xi.ground.n4.3` | xi3, n = 4 (ground-test profiles) | 1.31 | 1.31 | 0 |  |
| `ndp.en1997.xi.ground.n4.4` | xi4, n = 4 (ground-test profiles) | 1.2 | 1.2 | 0 |  |
| `ndp.en1997.xi.ground.n5.3` | xi3, n = 5 (ground-test profiles) | 1.29 | 1.29 | 0 |  |
| `ndp.en1997.xi.ground.n5.4` | xi4, n = 5 (ground-test profiles) | 1.15 | 1.15 | 0 |  |
| `ndp.en1997.xi.ground.n7.3` | xi3, n = 7 (ground-test profiles) | 1.27 | 1.27 | 0 |  |
| `ndp.en1997.xi.ground.n7.4` | xi4, n = 7 (ground-test profiles) | 1.12 | 1.12 | 0 |  |

### EN 1998 · EN 1997-1 Table A.11

Printed source: DIN EN 1997-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1997-1 Table A.11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.xi.dynamic.n10.5` | xi5, n = 10 (dynamic tests) | 1.45 | 1.45 | 0 |  |
| `ndp.en1997.xi.dynamic.n10.6` | xi6, n = 10 (dynamic tests) | 1.3 | 1.3 | 0 |  |
| `ndp.en1997.xi.dynamic.n15.5` | xi5, n = 15 (dynamic tests) | 1.42 | 1.42 | 0 |  |
| `ndp.en1997.xi.dynamic.n15.6` | xi6, n = 15 (dynamic tests) | 1.25 | 1.25 | 0 |  |
| `ndp.en1997.xi.dynamic.n2.5` | xi5, n = 2 (dynamic tests) | 1.6 | 1.6 | 0 |  |
| `ndp.en1997.xi.dynamic.n2.6` | xi6, n = 2 (dynamic tests) | 1.5 | 1.5 | 0 |  |
| `ndp.en1997.xi.dynamic.n20.5` | xi5, n = 20 (dynamic tests) | 1.4 | 1.4 | 0 |  |
| `ndp.en1997.xi.dynamic.n20.6` | xi6, n = 20 (dynamic tests) | 1.25 | 1.25 | 0 |  |
| `ndp.en1997.xi.dynamic.n5.5` | xi5, n = 5 (dynamic tests) | 1.5 | 1.5 | 0 |  |
| `ndp.en1997.xi.dynamic.n5.6` | xi6, n = 5 (dynamic tests) | 1.35 | 1.35 | 0 |  |

### EN 1998 · EN 1997-1 Table A.9 / DIN 1054 Table A 7.1

Printed source: DIN EN 1997-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1997-1 Table A.9 / DIN 1054 Table A 7.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1997.xi.static.n1.1` | xi1, n = 1 (static load tests) | 1.4 | 1.35 | 0 |  |
| `ndp.en1997.xi.static.n1.2` | xi2, n = 1 (static load tests) | 1.4 | 1.35 | 0 |  |
| `ndp.en1997.xi.static.n2.1` | xi1, n = 2 (static load tests) | 1.3 | 1.25 | 0 |  |
| `ndp.en1997.xi.static.n2.2` | xi2, n = 2 (static load tests) | 1.2 | 1.15 | 0 |  |
| `ndp.en1997.xi.static.n3.1` | xi1, n = 3 (static load tests) | 1.2 | 1.15 | 0 |  |
| `ndp.en1997.xi.static.n3.2` | xi2, n = 3 (static load tests) | 1.05 | 1.05 | 0 |  |
| `ndp.en1997.xi.static.n4.1` | xi1, n = 4 (static load tests) | 1.1 | 1.05 | 0 |  |
| `ndp.en1997.xi.static.n4.2` | xi2, n = 4 (static load tests) | 1 | 1 | 0 |  |
| `ndp.en1997.xi.static.n5.1` | xi1, n = >= 5 (static load tests) | 1 | 1 | 0 |  |
| `ndp.en1997.xi.static.n5.2` | xi2, n = >= 5 (static load tests) | 1 | 1 | 0 |  |

### EN 1998 · EN 1998-1 3.2.3.1.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-1 3.2.3.1.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.time-history.mean-spectrum-ratio` | mean spectrum over elastic spectrum | 0.9 | 0.9 | 2 |  |
| `ndp.en1998.en1.time-history.min-records` | minimum number of accelerograms | 3 | 3 | 1 |  |

### EN 1998 · EN 1998-1 4.4.2.5

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-1 4.4.2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.diaphragm.transfer-factor` | γ_d transfer amplification | 1.3 | 1.3 | 4 |  |

### EN 1998 · EN 1998-1 4.4.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-1 4.4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.importance.nu-i-ii` | ν importance I, II | 0.5 | 0.5 | 1 |  |
| `ndp.en1998.en1.importance.nu-iii-iv` | ν importance III, IV | 0.4 | 0.4 | 1 |  |

### EN 1998 · EN 1998-2 5.6.3.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-2 5.6.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en2.pier.gamma-0` | γ_0 | 1.35 | 1.35 | 5 |  |

### EN 1998 · EN 1998-3 A.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-3 A.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en3.annex-a.theta-um.axial` | θ_um axial load base | 0.3 | 0.3 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-um.base` | θ_um base value | 0.016 | 0.016 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-um.confinement` | θ_um confinement base | 25 | 25 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-um.diagonal` | θ_um diagonal reinforcement base | 1.25 | 1.25 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-um.reinforcement` | θ_um reinforcement exponent | 0.225 | 0.225 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-um.shear-span` | θ_um shear span exponent | 0.35 | 0.35 | 2 |  |
| `ndp.en1998.en3.chord-rotation.sd-factor` | SD factor | 0.75 | 0.75 | 5 |  |

### EN 1998 · EN 1998-3 A.10b

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-3 A.10b.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en3.annex-a.theta-y.bar-slip` | θ_y bar slip factor | 0.13 | 0.13 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-y.base` | θ_y constant term | 0.0013 | 0.0013 | 2 |  |
| `ndp.en1998.en3.annex-a.theta-y.slenderness` | θ_y h/L_V factor | 1.5 | 1.5 | 2 |  |

### EN 1998 · EN 1998-5 4.1.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 4.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en5.liquefaction.min-safety-factor` | F_L min | 1.25 | 1.25 | 6 |  |
| `ndp.en1998.en5.liquefaction.screen-ag-s-g` | a_g S screening limit in g | 0.1 | 0.15 | 3 |  |
| `ndp.en1998.en5.liquefaction.screen-depth-m` | screening depth | 15 m | 15 m | 3 |  |

### EN 1998 · EN 1998-5 5.4.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 5.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en5.pile.stiffness-contrast` | G_2/G_1 for kinematic interaction | 6 | 6 | 5 |  |

### EN 1998 · EN 1998-5 Annex B

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 Annex B.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en5.liquefaction.annex-b.csr-factor` | τ_e factor | 0.65 | 0.65 | 9 |  |

### EN 1998 · EN 1998-5 Annex B (chart), Youd et al. 2001

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 Annex B (chart), Youd et al. 2001.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.ref.liquefaction.youd-2001` | reference model | r_d, CRR 7.5, MSF, fines correction | r_d, CRR 7.5, MSF, fines correction | 3 |  |

### EN 1998 · EN 1998-5 Annex F

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 Annex F.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en5.annex-f.gamma-rd` | γ_Rd | 1 | 1 | 3 |  |

### EN 1998 · EN 1998-5 Table F.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure EN 1998-5 Table F.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en5.annex-f.sand.a` | a | 0.92 | 0.92 | 2 |  |
| `ndp.en1998.en5.annex-f.sand.b` | b | 1.25 | 1.25 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.beta` | β | 2.57 | 2.57 | 2 |  |
| `ndp.en1998.en5.annex-f.sand.c` | c | 0.92 | 0.92 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.c-m` | c_M | 2.57 | 2.57 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.c-m-prime` | c'_M | 1.85 | 1.85 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.c-t` | c_T | 2.14 | 2.14 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.c-t-prime` | c'_T | 1.81 | 1.81 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.d` | d | 1.25 | 1.25 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.e` | e | 0.92 | 0.92 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.f` | f | 0.41 | 0.41 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.gamma` | γ | 4 | 4 | 2 |  |
| `ndp.en1998.en5.annex-f.sand.k` | k | 0.39 | 0.39 | 1 |  |
| `ndp.en1998.en5.annex-f.sand.k-prime` | k' | 0.39 | 0.39 | 2 |  |
| `ndp.en1998.en5.annex-f.sand.m` | m | 0.96 | 0.96 | 1 |  |

### EN 1998 · Nikolaou et al. 2001

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Nikolaou et al. 2001.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.ref.pile.nikolaou-2001.coefficient` | M = c τ_c d³ ... | 0.042 | 0.042 | 2 |  |
| `ndp.en1998.ref.pile.nikolaou-2001.length-exponent` | (L/d)^p | 0.3 | 0.3 | 1 |  |
| `ndp.en1998.ref.pile.nikolaou-2001.modulus-exponent` | (E_p/E_1)^q | 0.65 | 0.65 | 1 |  |
| `ndp.en1998.ref.pile.nikolaou-2001.velocity-exponent` | (v_s2/v_s1)^r | 0.5 | 0.5 | 2 |  |

### EN 1998 · Table 8.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure Table 8.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-30.2` | 2.0 | awaiting | awaiting | 0 |  |

### EN 1998 · Table 9.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Table 9.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.masonry.min-thickness-m` | t_ef min | 0.24 m | 0.24 m | 3 |  |
| `ndp.en1998.en1.masonry.slenderness-max` | h_ef/t_ef max | 12 | 12 | 3 |  |

### EN 1998-1 · 1.00

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 1.00.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-01.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998-1 · 1.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-50.1` | ΣM_Rc | awaiting | awaiting | 0 |  |

### EN 1998-1 · 3.1 Table 3.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 3.1 Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.ground.cu-b-min` | c_u lower bound of ground type B | 250 kPa | 250 kPa | 1 |  |
| `ndp.en1998.en1.ground.cu-c-min` | c_u lower bound of ground type C | 70 kPa | 70 kPa | 1 |  |
| `ndp.en1998.en1.ground.e.substratum-vs-min` | Ground type E substratum v_s lower bound | 800 m/s | 800 m/s | 1 |  |
| `ndp.en1998.en1.ground.e.thickness-max-m` | Ground type E alluvium maximum thickness | 20 m | 20 m | 1 |  |
| `ndp.en1998.en1.ground.e.thickness-min-m` | Ground type E alluvium minimum thickness | 5 m | 5 m | 1 |  |
| `ndp.en1998.en1.ground.s1.plasticity-index-min` | Ground type S1 plasticity index lower bound | 40 | 40 | 1 |  |
| `ndp.en1998.en1.ground.s1.thickness-min-m` | Ground type S1 soft clay minimum thickness | 10 m | 10 m | 1 |  |
| `ndp.en1998.en1.ground.spt-b-min` | N_SPT lower bound of ground type B | 50 blows/30cm | 50 blows/30cm | 1 |  |
| `ndp.en1998.en1.ground.spt-c-min` | N_SPT lower bound of ground type C | 15 blows/30cm | 15 blows/30cm | 1 |  |
| `ndp.en1998.en1.ground.vs-a-min` | v_s,30 lower bound of ground type A | 800 m/s | 800 m/s | 1 |  |
| `ndp.en1998.en1.ground.vs-b-min` | v_s,30 lower bound of ground type B | 360 m/s | 360 m/s | 1 |  |
| `ndp.en1998.en1.ground.vs-c-min` | v_s,30 lower bound of ground type C | 180 m/s | 180 m/s | 1 |  |

### EN 1998-1 · 3.2.2.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.2.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l343.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998-1 · 3.2.3.1.2(4)

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 3.2.3.1.2(4).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.time-history.zero-period-ratio` | mean zero-period acceleration over a_g·S | 1 | 1 | 2 |  |

### EN 1998-1 · 4.2.5 / NA

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.2.5 / NA.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.gamma-i.i` | gamma_I class I | 0.8 | 0.8 | 0 |  |
| `ndp.en1998.en1.gamma-i.ii` | gamma_I class II | 1 | 1 | 0 |  |
| `ndp.en1998.en1.gamma-i.iii` | gamma_I class III | 1.2 | 1.2 | 0 |  |
| `ndp.en1998.en1.gamma-i.iv` | gamma_I class IV | 1.4 | 1.4 | 0 |  |

### EN 1998-1 · 4.3.3.1(3)

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.3.3.1(3).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.modal.min-mass-participation` | Σ M_eff / M of the modes taken into account | 0.9 | 0.9 | 4 |  |

### EN 1998-1 · 4.4.2 (overturning stability)

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.4.2 (overturning stability).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.overturning.stabilising-factor` | γ stabilising on G_k in the seismic combination | 1 | 1 | 3 |  |

### EN 1998-1 · 4.4.2.2 plan envelope

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.4.2.2 plan envelope.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.plan-gap-ratio` | plan gap ratio limit | 0.05 | 0.05 | 1 |  |

### EN 1998-1 · 4.4.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.4.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.drift-theta.brittle` | drift limit ratio brittle | 0.005 | 0.005 | 0 |  |
| `ndp.en1998.en1.drift-theta.ductile` | drift limit ratio ductile | 0.0075 | 0.0075 | 0 |  |
| `ndp.en1998.en1.drift-theta.isolated` | drift limit ratio isolated | 0.01 | 0.01 | 0 |  |

### EN 1998-1 · 5.4.2.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 5.4.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-50.2` | [V] | awaiting | awaiting | 0 |  |

### EN 1998-1 · 5.4.3 / 5.5.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 5.4.3 / 5.5.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.rc.rho-max.beam.dch` | rho_max beam DCH | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.beam.dcm` | rho_max beam DCM | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.beam.low` | rho_max beam LOW | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.column.dch` | rho_max column DCH | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.column.dcm` | rho_max column DCM | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.column.low` | rho_max column LOW | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.other.dch` | rho_max other DCH | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.other.dcm` | rho_max other DCM | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-max.other.low` | rho_max other LOW | 0.04 | 0.04 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.beam.dch` | rho_min beam DCH | 0.0035 | 0.0035 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.beam.dcm` | rho_min beam DCM | 0.0025 | 0.0025 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.beam.low` | rho_min beam LOW | 0.0025 | 0.0025 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.column.dch` | rho_min column DCH | 0.01 | 0.01 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.column.dcm` | rho_min column DCM | 0.01 | 0.01 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.column.low` | rho_min column LOW | 0.01 | 0.01 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.other.dch` | rho_min other DCH | 0.002 | 0.002 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.other.dcm` | rho_min other DCM | 0.002 | 0.002 | 0 |  |
| `ndp.en1998.en1.rc.rho-min.other.low` | rho_min other LOW | 0.002 | 0.002 | 0 |  |

### EN 1998-1 · 5.4.3.2.1 / 5.5.3.2.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 5.4.3.2.1 / 5.5.3.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.rc.column-min-b.dch` | RC column minimum width DCH | 0.25 m | 0.25 m | 0 |  |
| `ndp.en1998.en1.rc.column-min-b.dcm` | RC column minimum width DCM | 0.25 m | 0.25 m | 0 |  |
| `ndp.en1998.en1.rc.column-min-b.low` | RC column minimum width LOW | 0.2 m | 0.2 m | 0 |  |

### EN 1998-1 · 5.4.3.2.2 / 5.5.3.2.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 5.4.3.2.2 / 5.5.3.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.rc.omega-wd-min.dch` | omega_wd min DCH | 0.12 | 0.12 | 0 |  |
| `ndp.en1998.en1.rc.omega-wd-min.dcm` | omega_wd min DCM | 0.08 | 0.08 | 0 |  |
| `ndp.en1998.en1.rc.omega-wd-min.low` | omega_wd min LOW | 0 | 0 | 0 |  |

### EN 1998-1 · NA.12 (unverified)

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure NA.12 (unverified).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.masonry-wall-ratio.ge0-7` | masonry wall area ratio a_gR band ge0-7 | 0.04 | 0.04 | 1 |  |
| `ndp.en1998.en1.masonry-wall-ratio.importance-iii-iv` | masonry wall ratio factor importance III/IV | 1.25 | 1.25 | 2 |  |
| `ndp.en1998.en1.masonry-wall-ratio.lt0-5` | masonry wall area ratio a_gR band lt0-5 | 0.02 | 0.02 | 1 |  |
| `ndp.en1998.en1.masonry-wall-ratio.lt0-7` | masonry wall area ratio a_gR band lt0-7 | 0.03 | 0.03 | 1 |  |
| `ndp.en1998.en1.masonry-wall-ratio.none` | masonry wall area ratio a_gR band none | 0 | 0 | 1 |  |

### EN 1998-1 · Table 3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Table 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.t32.a.s` | S ground A | 1 | awaiting | 0 |  |
| `ndp.en1998.t32.a.tb` | T_B ground A | 0.15 s | awaiting | 0 |  |
| `ndp.en1998.t32.a.tc` | T_C ground A | 0.4 s | awaiting | 0 |  |
| `ndp.en1998.t32.a.td` | T_D ground A | 2 s | awaiting | 0 |  |
| `ndp.en1998.t32.b.s` | S ground B | 1.2 | awaiting | 1 |  |
| `ndp.en1998.t32.b.tb` | T_B ground B | 0.15 s | awaiting | 0 |  |
| `ndp.en1998.t32.b.tc` | T_C ground B | 0.5 s | awaiting | 0 |  |
| `ndp.en1998.t32.b.td` | T_D ground B | 2 s | awaiting | 0 |  |
| `ndp.en1998.t32.c.s` | S ground C | 1.15 | awaiting | 0 |  |
| `ndp.en1998.t32.c.tb` | T_B ground C | 0.2 s | awaiting | 0 |  |
| `ndp.en1998.t32.c.tc` | T_C ground C | 0.6 s | awaiting | 0 |  |
| `ndp.en1998.t32.c.td` | T_D ground C | 2 s | awaiting | 0 |  |
| `ndp.en1998.t32.d.s` | S ground D | 1.35 | awaiting | 0 |  |
| `ndp.en1998.t32.d.tb` | T_B ground D | 0.2 s | awaiting | 0 |  |
| `ndp.en1998.t32.d.tc` | T_C ground D | 0.8 s | awaiting | 0 |  |
| `ndp.en1998.t32.d.td` | T_D ground D | 2 s | awaiting | 0 |  |
| `ndp.en1998.t32.e.s` | S ground E | 1.4 | awaiting | 0 |  |
| `ndp.en1998.t32.e.tb` | T_B ground E | 0.15 s | awaiting | 0 |  |
| `ndp.en1998.t32.e.tc` | T_C ground E | 0.5 s | awaiting | 0 |  |
| `ndp.en1998.t32.e.td` | T_D ground E | 2 s | awaiting | 0 |  |

### EN 1998-1 · Table 3.3

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Table 3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.t33.a.s` | S ground A | 1 | awaiting | 0 |  |
| `ndp.en1998.t33.a.tb` | T_B ground A | 0.05 s | awaiting | 0 |  |
| `ndp.en1998.t33.a.tc` | T_C ground A | 0.25 s | awaiting | 0 |  |
| `ndp.en1998.t33.a.td` | T_D ground A | 1.2 s | awaiting | 0 |  |
| `ndp.en1998.t33.b.s` | S ground B | 1.35 | awaiting | 0 |  |
| `ndp.en1998.t33.b.tb` | T_B ground B | 0.05 s | awaiting | 0 |  |
| `ndp.en1998.t33.b.tc` | T_C ground B | 0.25 s | awaiting | 0 |  |
| `ndp.en1998.t33.b.td` | T_D ground B | 1.2 s | awaiting | 0 |  |
| `ndp.en1998.t33.c.s` | S ground C | 1.5 | awaiting | 0 |  |
| `ndp.en1998.t33.c.tb` | T_B ground C | 0.1 s | awaiting | 0 |  |
| `ndp.en1998.t33.c.tc` | T_C ground C | 0.25 s | awaiting | 0 |  |
| `ndp.en1998.t33.c.td` | T_D ground C | 1.2 s | awaiting | 0 |  |
| `ndp.en1998.t33.d.s` | S ground D | 1.8 | awaiting | 0 |  |
| `ndp.en1998.t33.d.tb` | T_B ground D | 0.1 s | awaiting | 0 |  |
| `ndp.en1998.t33.d.tc` | T_C ground D | 0.3 s | awaiting | 0 |  |
| `ndp.en1998.t33.d.td` | T_D ground D | 1.2 s | awaiting | 0 |  |
| `ndp.en1998.t33.e.s` | S ground E | 1.6 | awaiting | 0 |  |
| `ndp.en1998.t33.e.tb` | T_B ground E | 0.05 s | awaiting | 0 |  |
| `ndp.en1998.t33.e.tc` | T_C ground E | 0.25 s | awaiting | 0 |  |
| `ndp.en1998.t33.e.td` | T_D ground E | 1.2 s | awaiting | 0 |  |

### EN 1998-1 · Table NA.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Table NA.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.na4.ar.s` | S AR | awaiting | 1 | 0 |  |
| `ndp.en1998.na4.ar.tb` | T_B AR | awaiting | 0.05 s | 0 |  |
| `ndp.en1998.na4.ar.tc` | T_C AR | awaiting | 0.2 s | 0 |  |
| `ndp.en1998.na4.ar.td` | T_D AR | awaiting | 2 s | 0 |  |
| `ndp.en1998.na4.br.s` | S BR | awaiting | 1.25 | 0 |  |
| `ndp.en1998.na4.br.tb` | T_B BR | awaiting | 0.05 s | 0 |  |
| `ndp.en1998.na4.br.tc` | T_C BR | awaiting | 0.25 s | 0 |  |
| `ndp.en1998.na4.br.td` | T_D BR | awaiting | 2 s | 0 |  |
| `ndp.en1998.na4.bt.s` | S BT | awaiting | 1.25 | 0 |  |
| `ndp.en1998.na4.bt.tb` | T_B BT | awaiting | 0.1 s | 0 |  |
| `ndp.en1998.na4.bt.tc` | T_C BT | awaiting | 0.3 s | 0 |  |
| `ndp.en1998.na4.bt.td` | T_D BT | awaiting | 2 s | 1 |  |
| `ndp.en1998.na4.cr.s` | S CR | awaiting | 1.5 | 0 |  |
| `ndp.en1998.na4.cr.tb` | T_B CR | awaiting | 0.05 s | 0 |  |
| `ndp.en1998.na4.cr.tc` | T_C CR | awaiting | 0.3 s | 0 |  |
| `ndp.en1998.na4.cr.td` | T_D CR | awaiting | 2 s | 0 |  |
| `ndp.en1998.na4.cs.s` | S CS | awaiting | 0.75 | 0 |  |
| `ndp.en1998.na4.cs.tb` | T_B CS | awaiting | 0.1 s | 0 |  |
| `ndp.en1998.na4.cs.tc` | T_C CS | awaiting | 0.5 s | 0 |  |
| `ndp.en1998.na4.cs.td` | T_D CS | awaiting | 2 s | 0 |  |
| `ndp.en1998.na4.ct.s` | S CT | awaiting | 1.5 | 0 |  |
| `ndp.en1998.na4.ct.tb` | T_B CT | awaiting | 0.1 s | 0 |  |
| `ndp.en1998.na4.ct.tc` | T_C CT | awaiting | 0.4 s | 0 |  |
| `ndp.en1998.na4.ct.td` | T_D CT | awaiting | 2 s | 0 |  |

### EN 1998-1 · Tables 5.1/6.2/7.2/8.1/9.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure Tables 5.1/6.2/7.2/8.1/9.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en1.q-max.composite.dch` | q_max composite DCH | 6 | 6 | 0 |  |
| `ndp.en1998.en1.q-max.composite.dcm` | q_max composite DCM | 4 | 4 | 0 |  |
| `ndp.en1998.en1.q-max.composite.low` | q_max composite LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.default` | q_max default | 1.5 | 1.5 | 1 |  |
| `ndp.en1998.en1.q-max.masonry.dch` | q_max masonry DCH | 2.5 | 2.5 | 0 |  |
| `ndp.en1998.en1.q-max.masonry.dcm` | q_max masonry DCM | 2.5 | 2.5 | 0 |  |
| `ndp.en1998.en1.q-max.masonry.low` | q_max masonry LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.rc.dual.dch` | q_max rc.dual DCH | 5.2 | 5.2 | 0 |  |
| `ndp.en1998.en1.q-max.rc.dual.dcm` | q_max rc.dual DCM | 3.6 | 3.6 | 0 |  |
| `ndp.en1998.en1.q-max.rc.dual.low` | q_max rc.dual LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.rc.frame.dch` | q_max rc.frame DCH | 5.85 | 5.85 | 0 |  |
| `ndp.en1998.en1.q-max.rc.frame.dcm` | q_max rc.frame DCM | 3.9 | 3.9 | 0 |  |
| `ndp.en1998.en1.q-max.rc.frame.low` | q_max rc.frame LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.rc.wall.dch` | q_max rc.wall DCH | 4.4 | 4.4 | 0 |  |
| `ndp.en1998.en1.q-max.rc.wall.dcm` | q_max rc.wall DCM | 3 | 3 | 0 |  |
| `ndp.en1998.en1.q-max.rc.wall.low` | q_max rc.wall LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.steel.frame.dch` | q_max steel.frame DCH | 6.5 | 6.5 | 0 |  |
| `ndp.en1998.en1.q-max.steel.frame.dcm` | q_max steel.frame DCM | 4 | 4 | 0 |  |
| `ndp.en1998.en1.q-max.steel.frame.low` | q_max steel.frame LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.steel.wall.dch` | q_max steel.wall DCH | 5 | 5 | 0 |  |
| `ndp.en1998.en1.q-max.steel.wall.dcm` | q_max steel.wall DCM | 4 | 4 | 0 |  |
| `ndp.en1998.en1.q-max.steel.wall.low` | q_max steel.wall LOW | 1.5 | 1.5 | 0 |  |
| `ndp.en1998.en1.q-max.timber.dch` | q_max timber DCH | 5 | 5 | 0 |  |
| `ndp.en1998.en1.q-max.timber.dcm` | q_max timber DCM | 2.5 | 2.5 | 0 |  |
| `ndp.en1998.en1.q-max.timber.low` | q_max timber LOW | 1.5 | 1.5 | 0 |  |

### EN 1998-2 · 3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l514.1` | 0.9 | awaiting | awaiting | 0 |  |

### EN 1998-2 · 3.2.2.4 Eq. 3.12

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 3.2.2.4 Eq. 3.12.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en2.seat.ground-displacement-coefficient` | c in d_g = c a_g S T_C T_D | 0.025 | 0.025 | 2 |  |

### EN 1998-2 · 6.6.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 6.6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en2.seat.ground-displacement-factor` | factor of d_eg = f d_g | 2 | 2 | 2 |  |

### EN 1998-4 · 1.7

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 1.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l546.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998-4 · 3.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l545.1` | q_nominal | awaiting | awaiting | 0 |  |

### EN 1998-4 · A.3.3.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure A.3.3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en4.tank.slosh-height-coefficient` | c in d_max = c R S_e(T_c) / g | 0.84 | 0.84 | 3 |  |

### EN 1998-5 · 3.1

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.sei-01.2` | ag S <= 0.4 g | awaiting | awaiting | 0 |  |

### EN 1998-5 · 3.5

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l559.1` |  (informative,  | awaiting | awaiting | 0 |  |

### EN 1998-5 · 4.1.4

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 4.1.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l555.1` | ag S <= 0.15 g | awaiting | awaiting | 0 |  |

### EN 1998-5 · 5.4.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. 2023-11 (spectral response acceleration S_aP,R; a_gR zones obsolete). Verify: printed table or figure 5.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.l561.1` | [V] | awaiting | awaiting | 0 |  |

### EN 1998-6 · 4.2

Printed source: DIN EN 1998-1/NA:2023-11 ed. to be recorded at verification. Verify: printed table or figure 4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1998.en6.interaction.max-period-lengthening` | T_1(k_theta) / T_1(fixed) accepted for a fixed base | 1.2 | 1.2 | 2 |  |

## EN 1999 (135 unverified)

### EN 1999 · 1.0

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 1.0.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-30.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 1.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l454.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.l455.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 1.25

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 1.25.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-32.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 1.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l448.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 2.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-01.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-01.3` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-01.4` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-02.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-02.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-04.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-05.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 2.3.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 2.3.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-81.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.1.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.1.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-05.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.1.6.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.1.6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-03.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-03.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-10.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.11

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.11.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-19.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-19.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-11.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.7

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-16.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.8

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-18.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.2.9

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-17.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-17.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.3.1.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.3.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-21.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-21.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-21.3` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.3.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-23.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.3.3.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.3.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-24.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.6.4.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.6.4.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l441.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.6.6.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.6.6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l442.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-26.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.3.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l444.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.3.4

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l443.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.3.6

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.3.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l445.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.4

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-25.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-25.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-27.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.7.6

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.7.6.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-28.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 6.8.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.8.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l449.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 7.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 7.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-80.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 7.2.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 7.2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l446.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 7.3.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 7.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l447.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 8.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 8.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.l452.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 8.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 8.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-31.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 8.5.10

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 8.5.10.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-33.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 8.5.2.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 8.5.2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-12.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-30.3` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · 8.8

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 8.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-30.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · ?

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure ?.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.e2-10.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.e2-11.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.e2-12.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.e2-12.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.l453.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.l693.1` | [T] | awaiting | awaiting | 0 |  |

### EN 1999 · EN 1990 Table A1.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 Table A1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.psi-default.0` | ψ0 default | 0.4 | 0.4 | 0 |  |
| `ndp.en1999.psi-default.1` | ψ1 default | 0.3 | 0.3 | 0 |  |
| `ndp.en1999.psi-default.2` | ψ2 default | 0.2 | 0.2 | 0 |  |
| `ndp.en1999.psi-fire.0` | ψ0 fire | 0 | 0 | 0 |  |
| `ndp.en1999.psi-fire.1` | ψ1 fire | 0 | 0 | 0 |  |
| `ndp.en1999.psi-fire.2` | ψ2 fire | 0 | 0 | 0 |  |

### EN 1999 · Table 3.2a

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure Table 3.2a.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-01.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · Table 6.

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure Table 6..

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-23.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · Table 6.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure Table 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-04.2` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-04.3` | ? | awaiting | awaiting | 0 |  |

### EN 1999 · Table 8.8

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure Table 8.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-32.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-1 · 6.1.4.3

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.1.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-06.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-1 · 6.7.5 (transverse load on the web)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.5 (transverse load on the web).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.transverse-load.chi-const` | χ_F constant | 0.5 | 0.5 | 0 |  |
| `ndp.en1999.transverse-load.fcr-factor` | F_cr factor | 0.9 | 0.9 | 0 |  |
| `ndp.en1999.transverse-load.kf-base` | k_F,0 | 6 | 6 | 0 |  |
| `ndp.en1999.transverse-load.kf-slope` | k_F,1 | 2 | 2 | 0 |  |
| `ndp.en1999.transverse-load.m2-factor` | m_2 factor | 0.02 | 0.02 | 0 |  |

### EN 1999-1-1 · 6.7.6 (effective web strip of a transverse stiffener)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 (effective web strip of a transverse stiffener).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-stiffener.strip-factor` | 15 | 15 | 15 | 1 |  |

### EN 1999-1-1 · 6.7.6 (stiffener stiffness, a/h_w < aspect limit)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 (stiffener stiffness, a/h_w < aspect limit).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-stiffener.inertia-factor-short` | 1.5 | 1.5 | 1.5 | 0 |  |

### EN 1999-1-1 · 6.7.6 (stiffener stiffness, a/h_w >= aspect limit)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 (stiffener stiffness, a/h_w >= aspect limit).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-stiffener.inertia-factor-long` | 0.75 | 0.75 | 0.75 | 0 |  |

### EN 1999-1-1 · 6.7.6 (stiffener stiffness, panel aspect ratio)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 (stiffener stiffness, panel aspect ratio).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-stiffener.aspect-limit` | √2 | 1.41421 | 1.41421 | 0 |  |

### EN 1999-1-1 · 6.7.6 (stiffener strut buckling length)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 (stiffener strut buckling length).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-stiffener.buckling-length-factor` | 0.75 | 0.75 | 0.75 | 0 |  |

### EN 1999-1-1 · 6.7.7 (longitudinal stiffeners, edge stiffener flexure)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.7 (longitudinal stiffeners, edge stiffener flexure).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.stiffener.edge-slenderness-limit` | λ̄_s,max | 1 | 1 | 2 |  |

### EN 1999-1-1 · EN 1990 Table A1.2(B)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1990 Table A1.2(B).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-g` | gamma_g | 1.35 | 1.35 | 2 |  |
| `ndp.en1999.gamma-q` | gamma_q | 1.5 | 1.5 | 2 |  |
| `ndp.en1999.xi` | xi | 0.85 | 0.85 | 2 |  |

### EN 1999-1-1 · EN 1991-1-2 §3.2

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1991-1-2 §3.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.fire-alpha-c` | fire_alpha_c | 25 | 25 | 2 |  |

### EN 1999-1-1 · EN 1999-1-1 Table 6.4

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 Table 6.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.alpha-max` | alpha_max | 1.25 | 1.25 | 1 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.1.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.1.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-m1` | gamma_m1 | 1.1 | 1.1 | 2 |  |
| `ndp.en1999.gamma-m2` | gamma_m2 | 1.25 | 1.25 | 2 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.1.4.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.1.4.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.edge-stiffener-min-depth-ratio` | edge_stiffener_min_depth_ratio | 5 | 5 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.1.6.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.1.6.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.haz-t-junction-factor` | haz_t_junction_factor | 1.33 | 1.33 | 0 |  |
| `ndp.en1999.haz-tig-factor` | haz_tig_factor | 1.5 | 1.5 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.2.9.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.2.9.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.psi-section-i` | psi_section_i | 2 | 2 | 0 |  |
| `ndp.en1999.psi-section-tube` | psi_section_tube | 1.5 | 1.5 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.3.1.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.3.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.kappa-end-lambda` | kappa_end_lambda | 1.2 | 1.2 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.3.3.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.3.3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.eta-0` | eta_0 | 0.8 | 0.8 | 0 |  |
| `ndp.en1999.gamma-0` | gamma_0 | 0.8 | 0.8 | 0 |  |
| `ndp.en1999.xi-0` | xi_0 | 0.8 | 0.8 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.3.3.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.3.3.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.omega-x-exponent` | omega_x_exponent | 1 | 1 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.7.4

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.7.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.shear-buckling-k-tau` | shear_buckling_k_tau | 5.34 | 5.34 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §6.7.5

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §6.7.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.patch-load-k-f` | patch_load_k_f | 6 | 6 | 0 |  |

### EN 1999-1-1 · EN 1999-1-1 §7.2.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §7.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.deflection-ratio` | deflection_ratio | 200 | 200 | 2 |  |

### EN 1999-1-1 · EN 1999-1-1 §8.1.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-1 §8.1.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-mw` | gamma_mw | 1.25 | 1.25 | 2 |  |

### EN 1999-1-1 · EN 1999-1-2 §2.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-2 §2.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-m-fi` | gamma_m_fi | 1 | 1 | 2 |  |

### EN 1999-1-1 · EN 1999-1-2 §4.2.5

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-2 §4.2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.fire-emissivity` | fire_emissivity | 0.3 | 0.3 | 2 |  |

### EN 1999-1-1 · EN 1999-1-3 Table 2.1

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-3 Table 2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-mf` | gamma_mf | 1 | 1.35 | 4 |  |

### EN 1999-1-1 · EN 1999-1-3 §2.2

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-3 §2.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.gamma-ff` | gamma_ff | 1 | 1 | 2 |  |

### EN 1999-1-1 · EN 1999-1-5 §5.3

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure EN 1999-1-5 §5.3.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.shell-alpha-hoop-shear` | shell_alpha_hoop_shear | 0.65 | 0.65 | 0 |  |
| `ndp.en1999.shell-quality-q` | shell_quality_q | 25 | 25 | 2 |  |
| `ndp.en1999.shell-quality-q-class-a` | shell_quality_q_class_a | 40 | 40 | 1 |  |
| `ndp.en1999.shell-quality-q-class-c` | shell_quality_q_class_c | 16 | 16 | 1 |  |

### EN 1999-1-1 · project limit (not in EN 1999-1-1 §7.2)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure project limit (not in EN 1999-1-1 §7.2).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.stress-ratio` | stress_ratio | 0.8 | 0.8 | 2 |  |

### EN 1999-1-2 · 2.7

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 2.7.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-50.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-50.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-2 · 4.2.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 4.2.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-52.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-2 · 4.2.5

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 4.2.5.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-51.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-51.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-2 · Table 3.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure Table 3.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-50.3` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-3 · 3.4

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 3.4.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-40.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-3 · 6.2

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.2.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-41.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-4 · 2.8

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 2.8.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-62.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-4 · 5.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 5.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-60.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-4 · 6.1

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 6.1.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-61.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-61.2` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-4 · 6.1.7 (web crippling)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.1.7 (web crippling).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.web-crippling.end.alpha` | α | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.end.c-a` | c_a | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.end.c-l` | c_l | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.end.c-phi` | c_φ | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.end.c-r` | c_r | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.interior.alpha` | α | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.interior.c-a` | c_a | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.interior.c-l` | c_l | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.interior.c-phi` | c_φ | awaiting | awaiting | 0 |  |
| `ndp.en1999.web-crippling.interior.c-r` | c_r | awaiting | awaiting | 0 |  |

### EN 1999-1-5 · 2.9

Printed source: DIN EN 1999-1-1/NA ed. 2013-05 and 2017-05 are marked withdrawn; current edition to be checked. Verify: printed table or figure 2.9.

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.a-70.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-71.1` | ? | awaiting | awaiting | 0 |  |
| `ndp.en1999.a-72.1` | ? | awaiting | awaiting | 0 |  |

### EN 1999-1-5 · 5 (ring stiffener, effective shell width)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 5 (ring stiffener, effective shell width).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.ring-stiffener.effective-width-factor` | b_e/√(r·t) | 0.78 | 0.78 | 1 |  |

### EN 1999-1-5 · 6.7.6 and 5 (ring stiffener stiffness)

Printed source: DIN EN 1999-1-1/NA ed. to be recorded at verification. Verify: printed table or figure 6.7.6 and 5 (ring stiffener stiffness).

| id | symbol | EN | NA-DE | check sites | note |
|---|---|---|---|---:|---|
| `ndp.en1999.ring-stiffener.required-inertia-factor` | γ_r | awaiting | awaiting | 1 |  |

## Awaiting values (unset rows)

Rows with an empty EN or NA-DE value. Each one needs the printed number before it can be verified. Counts per code, then the named examples from the brief.

| Code | Awaiting rows |
|---|---:|
| EN 1990 | 25 |
| EN 1991 | 212 |
| EN 1992 | 75 |
| EN 1993 | 104 |
| EN 1994 | 66 |
| EN 1995 | 39 |
| EN 1996 | 124 |
| EN 1997 | 49 |
| EN 1998 | 87 |
| EN 1999 | 87 |

### Named: hipped roof c_pe (18 matching unset rows)

- `ndp.en1991.1-4.hipped-cpe1.f` c_pe,1 zone F hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.g` c_pe,1 zone G hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.h` c_pe,1 zone H hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.i` c_pe,1 zone I hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.j` c_pe,1 zone J hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.k` c_pe,1 zone K hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.l` c_pe,1 zone L hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.m` c_pe,1 zone M hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe1.n` c_pe,1 zone N hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe10.f` c_pe,10 zone F hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe10.g` c_pe,10 zone G hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting
- `ndp.en1991.1-4.hipped-cpe10.h` c_pe,10 zone H hipped roof · Table 7.5 · EN awaiting · NA-DE awaiting

### Named: Table 9.3 (2 matching unset rows)

- `ndp.en1992.dt-06.1` As,min · 9.3.1 · EN awaiting · NA-DE awaiting
- `ndp.en1993.c-33.1` ? · 9.3.1 · EN awaiting · NA-DE awaiting

### Named: web crippling (10 matching unset rows)

- `ndp.en1999.web-crippling.end.alpha` α · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.end.c-a` c_a · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.end.c-l` c_l · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.end.c-phi` c_φ · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.end.c-r` c_r · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.interior.alpha` α · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.interior.c-a` c_a · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.interior.c-l` c_l · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.interior.c-phi` c_φ · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting
- `ndp.en1999.web-crippling.interior.c-r` c_r · 6.1.7 (web crippling) · EN awaiting · NA-DE awaiting

### Named: gr2/gr5 (1 matching unset rows)

- `ndp.en1990.a2.group.gr2.road_traffic` c gr2 road_traffic · Table A2.4 · EN awaiting · NA-DE awaiting

### Named: gr2/gr5 (1 matching unset rows)

- `ndp.en1990.a2.group.gr5.road_traffic` c gr5 road_traffic · Table A2.4 · EN awaiting · NA-DE awaiting

