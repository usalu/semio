# Compile review H, lane 90I: EN 1990 inferences crate and all submodules

Static review only (no cargo/rustc). Reviewer: lane 90I. Review window 02:05-02:30 on 2026-11-03; the owner edited `⚖️checks/📜️ndp-tables`, `📐️annex-criteria`, the compliance-report tests, and the shared `⚖️compliance` (`action_factors`, per-annex NDP status, `Remedy::insert`) while the review ran. Files touched after the first read were re-read (ndp-tables, annex-criteria, tests tail, shared check-results/ndp/action-factors).

## Scope read (all in full)

`B/🧬️schema/💡️inferences` (31 `.rs`, 2 `Cargo.toml`):
- crate root `🦀️.rs`, `🧾outline` (+unit test), `🧮️units`, `📋️compliance-evaluation`, `🧰️report-helpers`, `🏗️ultimate-limit-state`, `📏️serviceability-and-fatigue`, `🌍️working-life-and-bridge-annexes`, `📐️annex-criteria`, `📐️material-geometry-analysis`, `🎚️factor-overrides`, `🚆️rail-and-frame-sls`, `📊️annex-analyses`, `🧮️partial-factor-sets`, `⛓️referential-integrity`, tests `🔬️unit`, `🎯️oracle-wiring`, `🔬️compliance-report` (1398 lines).
- `⚖️checks` crate: root, `📈️reliability-math`, `📜️ndp-tables`, `🧮️combination-generator`, `🚄️rail-dynamics`, `📇️catalogue` (+`📚️inputs`, `🗂️cases`, `🌱️demo`, `📐️geometry`, test modules), mounted `🧭️spec-trace` and the mounted checks test `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`.
- Both `Cargo.toml` against the artifact workspace (`⚖️en1990/Cargo.toml` `[workspace.dependencies]`: every `workspace = true` key exists, lib path, workspace pointer, framework path deps resolve).

Verified against the CURRENT definitions of: `⚖️compliance` (`CheckBuilder`/`CheckResult`/`Remedy` incl. `remove`/`insert`, `Quantity`/`QuantityKind`, `NearLimit`, NDP table API `ndp`, `ndp_number_opt`, `record_ndp_reads`, `ndp_is_verified(_for)`, `Scope`/`Rows`/`UnitSource`/`CheckUnit`, `live_report!`, `design_case` (`CaseCatalogue`, `InputSpec` const fns, `CaseRow`, `geometry_in`), `spec_trace` (`SpecTrace::new` arity 5, `SpecTraceTable`, `Emitted`), `reasons`, `action_factors`), `🧮️mechanics` (`Model`, `Support`, `Member`, `Analysis::new`, `modal`, `ModalMode`), `oracle_driver` (`Registry`, `Adapter`, `KnownDifference`), model crate fields (incl. `factor_overrides`, `traffic_groups`, `ReliabilityClass` now re-exported from the facade).

Mechanical checks (scripts in `🗑️generated/compile-review-h/90I/`): `ndpcheck.py`, `dyn.py` (every `ndp.en1990.*` literal and every id built by `format!` exists in the en1990 NDP table: 270 rows, 34 psi keys x 3, gG/gQ/xi/kfi/beta tags, hslm a1..a10 x 4, a2.group cells), `fmtcheck.py` (580 `format!`/`assert*!`/`panic!` calls: placeholder count equals argument count, every inline `{name}` is a binding of the enclosing fn), float-literal method scan (E0689), `push_*` call/definition arity table, let-chain scan (edition 2021).

## Fixed

None. No error that I am certain of was found; nothing was edited in the tree.

## Findings that are NOT errors (checked and cleared)

- rsck "Registry literal at `🧪️tests/🎯️oracle-wiring/🦀️.rs:107` misses `adapters`, `artifact`, `known_differences`": false positive. The literal is `Registry { artifact: "en1990", adapters: &ADAPTERS, known_differences: &KNOWN }`; the struct in `oracle_driver` has exactly those three fields. `Adapter { check, evaluator, run }` and `[KnownDifference; 0]` match their definitions; `ADAPTERS` has 6 entries for `[Adapter; 6]`.
- `let mut total = 0.0; ... total.abs()` in `⚖️checks/🚄️rail-dynamics` (peak_acceleration): not E0689, `total += f64` before the call unifies the float variable (builtin binop rule).
- `SpecTrace::new(id, clause, entity, realisation, note)` calls in the generated `🧭️spec-trace` (29 rows, 29 ids) match the arity-5 const fn.
- `NationalAnnexes` `dyn_enum_close!` with `__semio_dispatch_NationalAnnex`: carried over from the previously compiled monolith; trait items implemented by `NaDe`/`NaEn` match `NationalAnnex`.
- All cross-lane consumers (`scene`: `inferences::{evaluate, factor_overrides::{Verdict, verdicts, deviations, annex_value, annex_for_action, targets}, group_coefficient, effective_group}`, `checks::{...}`; editor `En1990Units`, `live::{report, responsive, progress, cancel}`; oracle comparison `oracle_wiring::registry`) refer to items that exist and are public.
- Match exhaustiveness: `Equation` (8 variants) in the factor/psi_slot matches, `Verdict` (7), `DynamicReason` (5), `ActionClass`: complete. `RemedyBound` is not matched anywhere in the lane (new `Remove`/`Insert` variants are harmless).

## Open (routed to `📌️important/requests.md`)

1. Dev-dependency cycle `inferences -dev-> io/examples/schema -> inferences` (cargo accepts it; may bite `:test` through the nx graph). Not a `:build` problem.
2. Logic notes in `📐️annex-criteria` `push_traffic_group_rows` (status overwritten by `.utilization()` when a coefficient exceeds 1.0; remedy options spelled with `{:?}`). Not compile errors, not changed.

## Not read / limits

- Everything above was read in full; the shared compliance, mechanics and model crates were read only where the lane calls into them. The owner was still editing `ndp-tables`, `annex-criteria` and the compliance-report tests at 02:24; the final state of those three files was read after the last edit seen (02:24:40). Any later edit is unreviewed.
- Macro bodies that other lanes invoke (`incremental_laws!` in the editor, `norm_live_*`) were not part of this lane; `live_report!` expansion in the inferences root was checked.
- Test code was type-checked by reading; runtime expectations (numeric values, check statuses) were not verified.
