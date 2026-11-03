# NDP wave 2, round 3: per-Eurocode registry crates, typed tables, exact gamma citations

Nothing was compiled or run (builds are the infra agent's). Edits checked by reading and by a delimiter scan (touched files balanced). Rebuild requested in `integrate/latest.md` and all `build-en*/latest.md`. The registry JSON has 2754 entries now (2565 unverified); the registry Python oracle passes; the adoption oracle fails only on six ids of the en1993 owner (below).

## 1. Registry split into crates
- New crates under `⚖️compliance/📜️annex-and-tables/🔖️ndp/`: `🧱️model` (`semio-s-plugin-norm-ndp-model`: `NdpEntry`, `NdpValue`, `NdpSource`, `NdpStatus`, constructors `c`/`d`, `.at/.ed/.ver`) and `🗃️table/en1990 … en1999/` (`semio-s-plugin-norm-ndp-enXXXX`, each exporting `pub const ROWS`). The facade `semio-s-plugin-norm-ndp` keeps `ndp()`, `ndp_is_verified`, `unverified_ndp_entries`, `NDP_TABLE` (joined at compile time from the ten crates) and re-exports the model types explicitly (not `c`/`d`), so `semio_s_plugin_norm_compliance::*` call sites are unchanged. Largest crate en1995 at 73.7 KB; no total cap any more.
- `🧰️ndp/codegen.py` emits the part crates (lib + manifest) and the facade table; the `Number` scope error came from the old flat per-part files (fixed: each crate imports `NdpValue::*` from the model). Manifest workspace path is 7 levels (`../../../../../../..`), facade 5.
- Registered: norm `Cargo.toml` members (11 new), `Cargo.lock` entries by hand (deps listed), `repo ./📜️script.ts verify build-owners sync` (added 11, then 1, passed).
- Typed lookups in `📜️annex-and-tables/🦀️.rs`: `ndp_cell(table, keys, annex)`, `ndp_cell_common`, `ndp_cell_opt`, `ndp_interpolate(table, lead, axis, x, annex)` (linear, clamped); row and column keys are id segments; all reads are logged for citation.

## 2. Migrated tables (values as the code used them, all unverified)
- EN 1996 (`wave3_en1996_tables.py`, `📚️normative-tables`): `k.<gp|tl|lw>.<clay|casi|concrete|aac>.<g1..g4>` (48, EN and DE columns), `alpha.<mortar>`, `beta.<mortar>`, `fvk0.<mortar>.<material>.<band>` (28; bands ge15, ge10, ge5, ge2-5, lt2-5 for general purpose), `fxk.<x1|x2>.<material>.<strong|weak>` (16, EN = DE), `k-e`. `fk_factors`, `f_vk0_mpa`, `f_xk_mpa`, `k_e` read them.
- EN 1998 (`wave3_en1998_tables.py`): `en1.gamma-i.<i..iv>`, `rc.column-min-b`, `rc.rho-min|rho-max.<column|beam|other>.<dch|dcm|low>`, `rc.omega-wd-min`, `masonry-wall-ratio.*` (bands and importance factor 1.25), `plan-gap-ratio`; `part_1::{gamma_i, rc_column_min_b_m, rc_rho_limits, rc_omega_wd_min, masonry_wall_ratio_limit, plan_envelope_utilization}`.
- EN 1993 (`wave3_en1993_tables.py`): `1-1.e-modulus|g-modulus|poisson|alpha-t` (`e_steel()/g_steel()` replace the consts `E_STEEL/G_STEEL`, `E_MPA`, `E_SHELL`, `NU` and the `210000.0` literals, tests renamed), stainless grades (`1-4.grade.<name>.<fy|fu|e|n>`, `stainless_grade` returns an owned value), rho constants, flexural/ltb/torsional (alpha, lambda0), stainless fire factors (`1-2.stainless-fire.1-4301.<k02p|ke|ku>.t<theta>`, interpolated through `ndp_interpolate`), fatigue category ladder (`1-9.category.cNNN`), orthotropic deck details, S-N corner points and slopes (`1-9.sn.*`, also used by the parts-crate Miner sum which now calls `fatigue_fire::endurance`), size effect, web crippling coefficients.
- EN 1991 (`wave3_en1991_tables.py`): snow zones (`1-3.snow-zone.<z>.<a|b|min|factor>`, altitude offset/scale, exceptional factor, mu1 nodes, mu2, mu3), terrain z0/zmin, k_r constants, NA-DE q_p profile, c_pe walls (Table 7.1, linear in h/d), flat roofs (parapet, curved, mansard, zone I), pitched roofs θ = 0 (primary and secondary series per angle and zone), signboard c_f default, mode exponents and structural damping. The bridge c_f rows were already registry values.
- Adoption fixture `🧫️fixtures/🔖️ndp-annex-differences/🔣️.json` regenerated (153 ids; it also contains two EN 1992 ids of another owner). Not migrated: EN 1996 gamma_S, psi and slenderness limit, EN 1991 orography thresholds, EN 1998 steel section class thresholds.

## 3. EN 1993 special elements cite their gamma ids
`Case` got `gammas`, and `emit` cites `ndps` plus `gammas`; every `Case` in `🔭️special-parts-checks` and `🏛️annex-block-checks` lists exactly the ids its resistance uses (cold-formed distortional/shear M0, crippling M1; shells M1 or M0 by resistance; plated M1; joints M0/M1/M2, base plate M0/M2; tension rod M2, pin M0/M2; pile/crane M0; fatigue MF). The builder-chain checks (`🔩️joint-checks` M2/M3, `🧵️weld-group-checks` M2, `🧷️restraint-checks` M1, `🏛️shell-and-structure-checks` M0/M1/MF, `⏳️fatigue-fire-and-thin-walled-checks`, `🪜️built-up-checks` M1, `🕳️hollow-joint-checks` gamma-m5, `🔩️joint-row-checks` M0+M1+M2) call `.ndp(AnnexParams::GAMMA_*)`. The assignment was read from the code per check (helper closures included), not derived from the whole function.

## 4. Request to the en1996 owner
Appended to `📌️important/requests.md` ("from: ndp v3"). No reply yet; resolve when `[done] en1996 → ndp v3` appears.

## 5. Worklist
`📌️important/ndp-verification-worklist.md` regenerated with `🧰️ndp/worklist2.py` (2754 entries, 2565 unverified, 189 verified).

## Open
- En1993 owner: `v-258.1`, `v-376.1`, `v-806.1`, `v-881.1/.2/.3` are cited but not registered (adoption oracle).
- Cargo.lock edits for the new crates were made by hand; `synchronize` may rewrite them.
- `semio-s-plugin-norm-compliance` closure grew by about 35 lines (typed lookups); the owner should still split the annex module.
