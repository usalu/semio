# Leaves and hygiene (wave 2, 2026-11-03)

Nothing compiled, no cargo, no git write.

## 1. Editable leaves
- `kind-convention-v2/leaf_coverage.py` (venv python, `-I`) over en1990 to en1999: 0 content leaves without a `SetFieldRule` kind everywhere (content leaves 102, 165, 117, 476, 106, 119, 111, 132, 213, 157). The "about 93" and "17 handle-only" figures of status-matrix-3 were rule-count approximations; the schema walk shows full coverage. No generation was needed, so no owner area was touched and `verify build-owners sync` passes.
- New check `🧰️scripts/handle-paths-check.py <code>...`: every handle or binding path literal (`x[id={}].y`) in the scene dirs, normalised to `[]`, against the `SetFieldRule` paths. en1990, en1991, en1993, en1994, en1997, en1998, en1999: 0 uncovered. en1992 (8), en1995 (1), en1996 (2): all member-relative paths (`bars[id=..].y` under `members[]`, rules exist as `members[].bars[].y`) or insert/remove collection paths; no gap.

## 2. requests.md reconciliation
Evidence is in `📌️important/requests.md` (new section at the end, plus single-item lines 9, 381, 383, 888, 1047 marked done). All checked in code, none wrong:
- EN 1992 asw `🧰️helpers-and-annex-parts/🦀️.rs:158` `* 1.0e3`; sigma_cp `:134` with real `a_c`.
- EN 1993 prying `🔗️joint-components/🦀️.rs:86`; HEB 240 to 300 `it`/`iw` at `📸️snapshot/🦀️.rs:993-1072`.
- EN 1995 two-span `⚖️timber/🧮️load-combinations/🦀️.rs:32`. EN 1997 capacity 0 `📇️catalogue/🦀️.rs:148-156`.
- EN 1990 gamma sign: `combinations_dir` direction logic, tests `review_demo_a/b` in `🧪️tests/⚖️compliance/🦀️.rs:228-241`.
- EN 1994: continuous solver with 9qL²/128 test; `delta_sigma_ts` on the cracked section with test.
- EN 1996: f_vk0 by material via NDP cells, f_vlt f_bt,cal 0.032/0.026/0.020, regression test `review_regressions_...`.
- EN 1998: `fundamental_period_s` calls the stiffness-based `rayleigh_period_shear_building`; Table 4.2 phi 1.0 for D to F.
- EN 1999 eta: applied only to internal webs (`element_beta_over_eps`); had no regression test, so `stress_gradient_eta_scales_internal_webs_and_never_internal_flanges` was added to `🧪️tests/⚖️compliance/🔬️resistance-worked-examples/🦀️.rs`.
- Still open by design (not S1-verified here): EN 1995 `enumerate_combos` permanent-only combination, all S2 items, structuralcodes `t0_adj`.

## 3. Hygiene
- 13 `Command::new("python3")` spawns (vdi3805 4, din18599 2, iso16757 3, din16798 2, din4108 2) now use `semio_s_plugin_norm_compliance::python_oracle::command()` (venv interpreter, `-I`). Added the dev-dependency `semio-s-plugin-norm-compliance` (feature `compliance-testing`) to the schema crates of vdi3805, din18599 and din4108 (Cargo.toml, plus Cargo.lock for vdi3805 and din18599). Three bare spawns with `-I` remain in `🎬️structural-scene` (colour-maps, diagram-kit) and app-surface `📈️diagrams2d` tests; those crates do not depend on the compliance kernel.
- `[DEBUG]` `eprintln!` removed from `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs`; an unwritable report directory is skipped silently.
- Seven `-2`/`-3` dirs: kept `blatt-3` x2 (VDI 3805 Blatt 3) and `change-floor-psi-2` x2 (ψ₂, per rule-guard.md). Renamed three crate splits: din4108 `mut-elements-2` to `mut-elements-rows`, iso16757 `mut-products-2` to `mut-products-taxonomy`, vdi3805 `examples-failing-2` to `examples-failing-blatt-4-to-19` (dirs, crate names, workspace Cargo.toml, parent re-exports, Cargo.lock of the artifact, hub, fixture-sweep and bridge, `📋️build-owners.json`, `🔒️dependencies.json`). Lock package and dependency order stays sorted. `verify build-owners sync` passes.

Ready for rebuild: leaves and hygiene (din4108, iso16757, vdi3805, din18599, din16798, en1999 tests).
