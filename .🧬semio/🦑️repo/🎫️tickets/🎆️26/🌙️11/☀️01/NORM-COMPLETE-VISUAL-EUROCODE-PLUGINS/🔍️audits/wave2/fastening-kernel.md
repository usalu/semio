# Fastening kernel (wave 2): one domain-neutral EN 1992-4 kernel

Nothing compiled (rule). No git writes. No new crate (crate budget): the kernel is a public module `fastening` of the shared mechanics crate `semio-s-plugin-norm-mechanics` (`🧮️mechanics/🔩️fastening/🦀️.rs`, 11 KB), which en1992-checks and en1993-checks-special-frames already depend on.

## Kernel API (N, mm, MPa, all annex rows passed in)
- Cone: `ConeInput`/`Region`/`Cone`, `cone_breakout` = N0_Rk,c A_c,N/A0 psi_s psi_re psi_ec psi_M (footprint union of the group clipped to the free region, `ConeInput::single` for one anchor with one free edge), parts `cone_n0_rk_c`, `psi_s_n`, `psi_re_n`, `psi_ec_n`, `psi_m_n`, `cone_area_ratio`, `edge_distance`, `union_area`. `k1` is an argument.
- Pull-out: `head_bearing_area`, `pullout_n_rk_p(k2, ...)`, `pullout_n_rk_p_scaled`. Splitting: `psi_h_sp`, `splitting_n_rk_sp`.
- Steel: `SteelPartialFactors` (row with limits and slopes; `EN_1992_4` base-text row, unverified) with `gamma_ms`, `steel_n_rk_s`, `steel_v_rk_s` (k7), `steel_v_rk_s_lever`.
- Shear edge breakout: `EdgeInput`/`Edge`, `edge_breakout` (k9 argument), `pryout_k8`, `pryout_v_rk_cp`. Interaction: `interaction(.., exponent)`.

## Replaced (old code deleted)
- en1992 `part_4` (helpers-and-annex-parts) is now a thin SI wrapper on the kernel; the hard-wired `AnnexChoice::De` is gone: `cone/pullout/splitting/pryout/edge_resistance_n` take `annex: AnnexChoice` (callers in anchor-evaluation pass `doc.annex`, catalogue test passes `DE`).
- en1992 `fastenings` (FA-02 steel, FA-03/11 cone, FA-04 pull-out, FA-05 splitting, FA-07 steel shear, FA-08/13 pry-out, FA-09 edge, FA-10 interaction): own `union_area`, `Group::ratio/edge`, the cone psi chain, edge formulas deleted; `gamma_ms` delegates to `SteelPartialFactors::EN_1992_4`. Case ids and outputs unchanged (same formulas; only FA-10 negative actions now count as zero).
- en1993 `plate-checks`: `Cone` and `cone_breakout` deleted, the `.cone.<id>` check calls `fastening::cone_breakout(&ConeInput::single(..))`. The wiring test uses the kernel.
- Cargo: `semio-s-plugin-norm-mechanics` moved from dev-dependency to dependency of en1992-checks (lock edge already present). No new crate, so `build-owners` needs no row.

## Tests
- Rust: `🧮️mechanics/🧪️tests/🔩️fastening/🦀️.rs` (worked cone example by hand, psi_ec/psi_M/psi_re, overlapping footprints, splitting/lever/interaction helpers, and the language-agnostic cases vs oracle).
- Language-agnostic cases: `🧪️tests/🔩️fastening/🧾️cases.json` (10 cases, flat JSON, `expect_*` closed forms).
- Python oracle twin: `🔮️oracles/🔩️fastening/🐍️.py` (closed forms written independently; `shapely` `unary_union` is the third-party cross-check of the cone footprint area; exits 1 on a stated-value mismatch; run OK).
- Feature: bullet added to `🧪️tests/🧮️mechanics/🥒️.feature`.

## Gates
Spec-trace `en1992 159 ok`, `en1993 142 ok`. Oracle script run: no mismatch. Remaining gates (handle-paths, check-cargo-authority, source budget, build-owners sync) were queued behind other agents in `heavy.sh`; results below. No fixture, schema or handle changes, so the fixture checkers are unaffected.

## Open / unverified
- Never compiled: `impl ConeInput<'static>::single` with a `&[[0.0, 0.0]]` constant promotion, struct-update `..ConeInput::single(..)` in the test, `pub struct Group` now holding `kernel::Region`.
- `SteelPartialFactors::EN_1992_4` (800 MPa, 0.8, 1.2/1.4, 1.0/1.25, 1.5), k9 (1.7/2.4), k7 (0.8 grout), k8 are base-text constants, unverified against the standard; k1, k2, gamma_Mc come from registry rows.
- Pull-out N_Rk,p is available in the kernel but en1993 still does not call it (head bearing area missing in the model). EN 1994 can now call cone, pull-out, steel and edge directly.

## Gate results
Handle paths en1992 0 uncovered, en1993 0 uncovered. `check-cargo-authority.py`: 30 crates, 0 errors. Source budget: mechanics 92 % (147 KB, 21 files, warn, not over; kernel adds 11 KB, a later split of `section`/`statics` should make room); the 6 crates over the boundary are foreign (en1994/others, not touched). `verify build-owners sync` not run: no crate, member or dependency row was added (mechanics already owned). Rebuild lines appended to build-en1992, build-en1993 and mechanics `latest.md`.
