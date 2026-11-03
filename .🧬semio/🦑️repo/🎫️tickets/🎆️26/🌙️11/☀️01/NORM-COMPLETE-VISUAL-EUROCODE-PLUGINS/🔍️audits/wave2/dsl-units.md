# DSL units (wave 2)

Table: `semio_framework_dsl::UnitSpec` static `UNITS` in `🧰️framework/🔨️modules/🗣️dsl/🔤️token/🦀️.rs` (symbol, SI `Dimension` vector, linear factor; `unit_by_symbol` const lookup used by `borrowed_unit`). Schema/test fixture: `🔤️token/🧪️tests/🧫️fixtures/🏷️const-units/🔣️.json` (symbol order = table index, queries incl. refusals); the unit test asserts length, order and lookups. The compliance `UnitId` vocabulary (`⚖️compliance/.../🔢️units`) already has N/m and kN/m (line load); it is separate and not checked by `borrowed_unit`.

## Scan
Every `#[dsl(unit)]` in the norm plugins used m, N, Pa, m2, s, m/s, K, J, m/s2, m4, m3, kg, deg, pct, mm, kg/m3, h (all present) plus `m2/m` (en1994 `transverse_as_top_m2_per_m`, latent E0080) and the removed `N/m`. Field-meta and JSON twins use display symbols (N/m, N/m3, N*m, m^6, Hz, ...) and are not table-checked.

## Added (appended, indices 41-49; additive)
N/m and kN/m (new DIM_LINE_LOAD kg s-2), m2/m (length dimension, factor 1), N/m3 and kN/m3 (DIM_UNIT_WEIGHT), N*m and kN*m (energy dimension), m6 (DIM_M6), Hz (DIM_FREQUENCY). Table length 41 -> 50; fixture symbols/queries extended plus a refusal query `N/n`.

## Attributes
Restored `#[dsl(unit = "N/m")]` on en1991 `assumed_rail_lm71_udl|sw0|sw2`. Also added the missing units on en1995 `q_line_n_per_m` (N/m) and `m_k_nm` (N*m). Other norm fields without `dsl(unit)` (e.g. en1997 `concrete_gamma` N/m3) were not changed: out of scope, table now supports it.

Nothing compiled. Rebuild line "ready for rebuild: dsl units" appended to integrate and build-en* latest.md.
