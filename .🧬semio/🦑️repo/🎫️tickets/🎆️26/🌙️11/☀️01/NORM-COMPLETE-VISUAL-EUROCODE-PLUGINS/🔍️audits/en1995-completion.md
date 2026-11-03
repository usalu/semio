# EN 1995 norm completion report

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995` (subset root `S = 🏅️standards/🔖️1/🪆️subsets/✳️any`). Kernel root `K = S/🧬️schema/⚖️timber`.

## Status

BUILD STATE: see "Verification status" at the end. The repo MCP `show_projects` call hung for 1800 s on the capacity queue (6.4 GB request behind other agents), so builds were submitted through the `repo` CLI (`repo run @semio-tech/norm-en1995-rs:test --language en`) and queued behind other agents. Nothing below is claimed green unless the final section says so.

## Implemented catalogue ids (kernel + registry, all with `evaluate_case`)

Registry: `K/📇️catalogue/🦀️.rs` (`registry()`, `entry(id)`, `evaluate_case(id, &Case)`, `evaluate_outcomes(id, &Case)`), `Case` = annex + numeric + text inputs in N, mm, N/mm². Every entry returns `CheckResult`s with clause, utilisation, SI quantities and a step text; missing inputs yield one `NotApplicable` result naming the missing key.

| Group | Module | Ids |
|---|---|---|
| Materials, factors | `K/📚️strength-tables`, `K/🧱️cross-sections` | T-01 (EN 338 C14–C50, D18–D70, EN 14080 GL h/c, LVL, CLT lamellae table), T-02 kmod (solid/glulam/LVL/CLT/plywood/OSB/particleboard/fibreboard, shortest-duration rule, connection √), T-03 γM (EN/NA-DE per product, accidental 1.0, fire 1.0), T-04 kdef (Table 3.2, ufin, kdef connection, Efin), T-05 kh/kl/kvol, T-06 design strengths |
| 6.1/6.2/6.6 | `K/🧱️cross-sections` | T-10, T-11, T-12, T-13 (lef, kc,90 table), T-14, T-15 (kcr EN/NA, support reduction), T-16 (torsion, ksh, combined shear), T-17, T-18 (net section, holes), T-50 (ksys) |
| 6.3 | `K/📐️stability` | T-20, T-21, T-22 (Table 6.1 lef, σm,crit simplified and 6.31 exact, continuous restraint), T-23, T-24 |
| 6.4/6.5 | `K/🪚️varying-sections` | T-30, T-31, T-32 (kdis, kvol, pd term, 6.53), T-33, T-45, T-46 |
| 7 | `K/🛋️serviceability` | T-60 (closed form + numeric integration for tapered), T-61 (wfin = wG(1+kdef)+wQ(1+ψ2 kdef), wnet,fin), T-62 (slip via γ-method), T-63, T-64, T-65, T-66 (alias of T-120) |
| 8 | `K/🔩️connections` | T-70 … T-83 (all failure modes a–m single/double shear timber and steel, rope effect with share caps, interpolation of plate thickness, Tables 8.2/8.4/8.5/8.6 spacing with α, nef, nail/screw/bolt axial, combined, splitting, block shear, steel plate, split ring/shear plate, toothed plate, nail plate, kser/Ku) |
| 9 | `K/🧩️components` | T-90 (γ-method, 2–3 parts), T-91, T-92 (effective flange widths), T-93 (Annex C), T-94 (wall Method A/B, hold-down, diaphragm, chord), T-95, T-96, T-97 (CLT γ-method, rolling shear), T-98 (q factor) |
| Fire | `K/🔥️fire` | T-100 (β0/βn, panel corrections, series), T-101 (reduced cross-section with bisection t_R), T-102 (zero-strength layer with comparison), T-103, T-104, T-105 |
| Bridges | `K/🌉️bridges` | T-120, T-121 (Miner), T-122, T-123, T-124 |

## Fixed defects (from the inventory, the coordinator notes and the NDP cross-check)

1. wfin applied ψ2 to the permanent part: now `wG(1+kdef)+wQ(1+ψ2·kdef)`; winst check uses the variable part only; cantilever uses 2L.
2. V, N, Fc,90, M, Nt were taken from the moment-governing combination: every ULS combination is checked per effect (`worst()` in `K/🏗️structure-and-members`), with the kmod of that combination.
3. Johansen mode (c) used t1/t2: now `r = t2/t1` per EN 8.6 (test `johansen_mode_c_uses_t2_over_t1_for_unequal_thicknesses`); rope effect now capped per fastener family (15/25/50/100/25/0 %) instead of a flat 25 %; invented `Fax = 0.2 fh d t` removed (screws use 8.38).
4. kv (notch) was a non-EN form: now EN 6.60–6.62 with kn 4.5/5/6.5, hef = h − notch, NA-DE limits h_ef ≥ 0.5 h.
5. kc,90 was a geometric ratio capped at 1.5: now Table 6.1.5 values (1.25/1.5/1.5/1.75, hardwood 1.0).
6. Floor: ad hoc support factors, acceleration formula and velocity formula removed; f1, unit-point deflection and velocity now EN 7.5–7.7; DE f1 threshold 6 Hz, a limit per NDP (see unverified).
7. Nail a3,t was 10d: Tables 8.2/8.4/8.5 with α (15d at α = 0); ρk > 420 factor.
8. γM DE was 1.3 for every product: per product table (glulam 1.25, LVL 1.2, CLT 1.25 …).
9. kcr applied to LVL: now 1.0 for LVL/CLT; EN 0.67 and NA 2.5/fv,k for solid/glulam only.
10. Strength tables: C16/C24/C30 ft,90 etc. corrected to EN 338:2016 (ft,90 0.4, C24 ft,0 14.5), GL24h/28h/32h to EN 14080:2013 (ft,0 16.5/19.5/22.5, fc,90 2.7/3.0/3.3, G 650); the table is shared by the editor pick list (`✏️editor/🏷️field-meta` regenerated from it, 36 classes).
11. `m_crit` scaled by (span/lef)² (ad hoc) replaced by σm,crit from 6.32 with lef = restraint spacing or 0.9 L; the snapshot field `mCritNm` is kept as an explicit eigenvalue override.
12. Bridge: heuristic fatigue (kfat) and comfort formulas replaced by Miner damage (EN 1995-2 6.2) and the 7.3 single-pedestrian / crowd accelerations with separate lateral frequency; horizontal check skipped for f_lat > 2.5 Hz, vertical for f1 > 5 Hz.
13. Fire: d_ef = βn·t + k0·d0 now shared with the kernel (`d_ef_mm`); old duplicated helpers removed.
14. Python oracle no longer mirrors the Rust tables: it carries its own row table keyed by class name, uses the exact 6.31 critical stress, a different Johansen derivation (β = 1 closed forms), Newmark series for T-90, shear analogy for CLT, a Rayleigh quotient for f1 and independent charring/Miner arithmetic.

## Tests added

* `S/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (Rust): `catalogue_matches_independent_python_oracle_within_one_percent` (18 cases, ≥ 30 values, fixture `S/🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json`), `registry_covers_every_catalogue_id`, `every_entry_reports_missing_inputs_instead_of_panicking`, worked examples E2 (kc = 0.28457, NRd = 36.77 kN) and E6 (7.532 kN / 4.635 kN), mode-c regression, nail a3,t, per-product γM and kcr, wfin regression, pinned strength rows, CLT γ-method versus shear analogy. Existing tests updated for the new kernel API.
* Language-agnostic: `S/🧪️tests/📇️catalogue-en1995-1/🥒️.feature` (18 example rows plus the worked-example scenario). Step adapters for the host test platform are NOT written (host API not exercised); see gaps.
* Oracle: `S/🔮️oracles/⚖️compliance/🐍️.py` (`--json` snapshot mode rewritten for the corrected checks, new `--catalogue` mode). Run with `python3 -I`.

## Unverified normative values

Strength rows with `verified: false` in `K/📚️strength-tables` (everything except C16, C24, C30, GL24h, GL28h, GL32h): C14, C18–C22, C27, C35–C50, all D classes, GL20h/22h/26h/30h and all GL c classes, LVL32, CLT100 lamella row. kmod rows: CLT, particleboard P5/P7, hardboard, MDF; kdef rows: CLT, particleboard, fibreboard; γM CLT 1.25 (DE); T-15 point-load reduction β = a/(2h); NA-DE a = 0.5 mm/kN and f1 ≥ 6 Hz (T-63/T-64), velocity b = 100; DE kdef/limits L/200 for wnet,fin; hole limit d ≤ 0.15 h; T-24 bow L/300 and L/500; slenderness 150; T-33 `Ft,90 = 0.5 σt,90 b hap`; T-46 coefficient 1.3; kc,90 applicability limits (l1 ≥ 2h, l ≤ 400); screw fax,k with ρa = 350, kd, fhead = 8 N/mm²; nail kef interpolation and ρk > 420 factor; screw My with d_root = 0.7 d; split ring/shear plate k-factors, toothed plate 18·k·dc^1.5, nail plate; Annex C η (glued 1, nailed 4, connectors 3, bolted 6); Method A/B factors; T-91 flange/web limits 20 hf / 25 hf and web slenderness 70; CLT γ-method d_i, Gr = 50 N/mm², fr,k = 1.0, fv,CLT = 5.0; T-98 q table; fire: hardwood interpolation, panel kρ/kh, CLT 0.65, t_ch = 2.8 hp − 14, tf = 0.8 tch, k2 = 1 − 0.018 hp, kflux 1.5, kfi connections 1.15/1.05, T-105 layer times; bridges: 200/100/50 Mζ accelerations, crowd factors 0.23/0.18, limits 0.7/0.2/0.4, Miner S-N defaults a = 15, b = 4, stress-laminated 0.35 N/mm², loss 0.55, μ = 0.3, wheel spread; floor: assumed 22 mm deck for (EI)b and square panel; spec example E11 arithmetic (1.249e6 Nm²/m) is a factor 10 too small for 80×220 @ 625 mm (correct 1.25e7, f1 ≈ 22 Hz).

## Remaining gaps (blocked or not done)

1. No snapshot entity / mutation leaves for catalogue cases (needs hand-edited 5 schema twins, diff, SQLite, 8+ mutation leaves with fixtures and the label census); the kernel is reachable only through `registry()`/`evaluate_case`, so the editor cannot create cases yet (requests added to `📌️important/requests.md`).
2. `mCritNm` field still exists (as override) and 216 fixture/twin files reference it; removal requires the entity work above.
3. ULS 6.10a/b with ξ = 0.85 is not selectable (no snapshot field for the equation choice); only 6.10 is used.
4. Gherkin step adapters (Rust/Python) for the new `.feature` are missing.
5. Field-meta for new fields: none, because no snapshot field was added.
6. NDP registration in `⚖️compliance/📜️annex-and-tables` (not owned; requested).
7. Visual/editor work belongs to other agents.

## Update 2026-11-02 (build loop, review fixes)

Build history (`🗑️generated/build-en1995/`): run 1 and 2 failed on a foreign manifest (`🧮️mechanics` `[lints] workspace = "../../.."`, fixed to `true`); run 3 hit the crate source boundary (`semio-s-artifact-norm-en1995-schema` 4211/4000 lines) so the verification kernel was split into the new crate `semio-s-artifact-norm-en1995-kernel` (`🧬️schema/⚖️kernel`, registered in the artifact workspace; `checks::timber` re-exports it with `pub use semio_s_artifact_norm_en1995_kernel::*`); source admission of the schema crate then passed (434/447). Runs 4–11 failed only on infrastructure (NX daemon terminated, graph cache capacity exhausted, SIGKILL by OOM) and one foreign `E0283` in app-surface live-editor (reported, owner fixed). No rustc error of the EN 1995 crates has been seen yet because compilation of the artifact never completed; the first completed compile will show them. A rebuild is requested in `latest.md` (central runner).

Review fixes applied (uncompiled): two-span internals now `ql²/8 + 3Fl/16` and `5ql/8 + 11F/16` (member, bridge, oracle); permanent-only and duration-class ULS combinations (`uls.dur.*`, kmod of the class) in code and oracle; columns: point load is axial only, line load transverse only; strength tables: C-classes EN 338:2016 (C16 8.5, C30 19/24 …) and glulam EN 14080:2013 (GL24h 19.2/0.5/24/2.5, GL28h, GL32h, G = 650), verified flags only for C16/C24/C30/GL24h/28h/32h, the oracle rows retyped independently; DE γM 1.3 for all products (rows unverified); member 6.35 term with kcrit; lateral buckling default `l_ef = L`; k_cr 1.0 for LVL/CLT; predrilled-nail k_ef 0.7/0.85/1.0, n_ef angle interpolation α/90°, staple Kser /80, toothed plate ×1.5; connectors and Johansen require `kmod` and `f_u` (no silent 1.0/400), `kmod_2` gives √(k1 k2); notch geometry and negative shear rejected; zero-capacity utilisation = ∞; T-64 normalised by force; `charring_rates`, `cladding_times`, `deflection_m` exported for the scene.

Still open: snapshot entities for catalogue cases plus the visual agent's fields (tapered/curved/pitched geometry, CLT layers, split rings, trusses, diaphragms, footbridge decks, Placement, connection member reference, a₂), mutations, twins, diff, codecs, fixtures; selectable ULS 6.10a/b with ξ = 0.85 (snapshot field); shear in `deflection_m` (kept bending-only because `deflection_at` in the effect profile shares its coefficients); psi categories F/G and unknown-duration rejection; T-23 NA form; plywood bolt embedment; DE velocity check for 6 ≤ f1 < 8 Hz. These need a green compile first and are not started.
