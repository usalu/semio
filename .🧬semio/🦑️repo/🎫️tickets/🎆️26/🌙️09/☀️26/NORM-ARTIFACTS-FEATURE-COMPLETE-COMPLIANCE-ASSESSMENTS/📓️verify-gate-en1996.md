# Verify Gate — EN 1996 Remedy Clearance (`🪨️en1996`)

**PASS**

**Auditor:** read-only subagent (remedy-clear gate)  
**Family:** `en1996` — located via `os.walk` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts` → `🪨️en1996`  
**Fixer claim:** `📓️impl-gate-en1996.md` — `Summary [0.507s] 145 tests run: 145 passed, 0 skipped` (prior closed round 144)  
**Gate rule:** each blocking `Fail` must leave `status ≠ Fail` after `apply_remedy_edit(..., option_index 0)` on at least one applicable scalar remedy, or after sequential application of all applicable remedies on one copy (`gate_blocking_fails_clear_via_remedy_bounds`, `⚖️compliance/🦀️.rs:117–170`). Equality at a ≤/≥ limit must `Pass` (`CheckBuilder::utilization`).

**Runners (this audit):**
- Did **not** rerun the cross-family compliance gate (per instructions).
- Targeted family test: `gate_blocking_fails_clear_via_remedy_bounds` — **passed** via `NX_DAEMON=false bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache -- gate_blocking_fails_clear_via_remedy_bounds` → `Summary [0.145s] 1 test run: 1 passed, 144 skipped`.
- Full 145-test family suite **not** independently rerun (cargo compile ~5.5 min for single test).

---

## Per-check audit (`En1996Snapshot::noncompliant_multi_fail`)

Fixture: `wall-weak` / load case `uls-bad` / concentrated `beam-A` (`📸️snapshot/🦀️.rs:104–156`). Prior symptoms from gate: material and flexure sequential still `u=1.5000`; concentrated remedy[0] only dropped to ~1.64.

| Check id | Before (symptom) | Remedy path(s) | Field(s) read by evaluate | Solo vs sequential | en/de distinct |
|----------|------------------|----------------|---------------------------|--------------------|----------------|
| `en1996.3.1.material.wall-weak` | Fail `u=1.5` — Group2 aspect `0.250/0.090≈2.78>2.5` **and** GP bed joint `0.004 m ∉ [6,15] mm` (`material_conformance_ok`, `⚖️masonry/🦀️.rs:536–571`) | `mortarStrengthPa`, `fBPa`, `unitHeightM`, `unitWidthM`, `bedJointThicknessM` (`:871–917`) | All five paths feed `material_conformance_ok` (mortar class match, `f_b` band, aspect via `unit_height_m/unit_width_m`, joint limits via `bed_joint_thickness_m`) | **Sequential** — no single scalar clears (aspect and joint both fail); test sequential branch passes | yes — e.g. bed-joint remedy EN “Set bed-joint thickness…” / DE “Lagerfugendicke auf zulässigen NM-…” (`:913–916`) |
| `en1996.6.3.flexure.wall-weak.uls-bad` | Fail high `u` (~58 pre-fix); post-fix thickness bound `t·√(M_Ed/M_Rd)·1.01` (`:1190–1194`) | `thicknessM` (primary), `asHorizontalM2`, `fYdPa` | `thickness_m` → `section_modulus_m3` (`M_Rd ∝ t²`); `as_horizontal_m2` + `f_yd_pa` → `m_rd_reinf` (`:1168–1176`) | **Solo[0] thickness** clears — loop breaks before `as_h`/`f_yd` pair; `as_h>0 ∧ f_yd≤0` still forces `u=1.5` (`:1208–1209`) but is not the first clearing path | yes — flexure explanation EN vs DE “(Plattenbiegung)” suffix (`:1187–1188`) |
| `en1996.6.1.3.concentrated.wall-weak.uls-bad.beam-A` | Fail `u≈8.68`; old length-only remedy lowered `β`, leaving `u≈1.64` | **Primary** `bearingAreaM2` at_least `area_used·(F_Ed/N_Rdc)·1.01` at fixed `β` (`:1235–1242`); support `bearingLengthM`, `thicknessM` | `c.bearing_area_m2` in `n_rdc = β·f_d·max(A, ℓ·t)` (`:1219–1219`, `:1235`) | **Solo[0] bearing area** clears — area scales capacity without shrinking `β` | yes — explanation DE adds “(Teilflächenpressung)” (`:1232–1233`) |

---

## Compliance criteria

| # | Criterion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | Remedy edits a field the check reads, by enough that `status ≠ Fail` | **PASS** | Targeted test applies `set_value_at_path` on each remedy target then re-runs `evaluate_building`; all three ids `cleared = true`. Source bounds tied to same formulas as assess (material flags, `M_Rd` thickness scaling, concentrated `area_req`). |
| 2 | No remedy that only adjusts unrelated fields is the sole applicable path | **PASS** | Material needs sequential (aspect **and** joint); flexure thickness solo clears before reinforcement branch; concentrated area is first applicable and sufficient. |
| 3 | en and de copy not identical | **PASS** | Distinct localized strings in remedy constructors and explanations (`⚖️masonry/🦀️.rs`); family test `mutation_and_check_labels_differ_en_de` still present (`⚖️compliance/🦀️.rs:383–434`). |
| 4 | No fingerprint / epsilon fold / id_score / tag_fp / dummy utilization gaming on these checks | **PASS** | `rg` over `⚖️masonry/🦀️.rs`: no `fingerprint`, `tag_fp`, `id_score`, `field_fingerprint`. Fail-branch `u=1.5` on material (`:884`) and `as_h∧¬f_yd` flexure guard (`:1209`) are explicit Fail placeholders, not remedy clearance tricks. Pass-branch material `u=0.5` when conforming (`:869`) is pre-existing weak display, not used to clear gate ids. |
| 5 | New test OK; no deleted test | **PASS** | Added `gate_blocking_fails_clear_via_remedy_bounds` (`:117`); fixer count 145 = prior 144 + 1 regression. |

---

## Source anchors (fixer claims vs code)

| Fixer claim (`📓️impl-gate-en1996.md`) | Verified |
|---------------------------------------|----------|
| Material: remedies for `unitWidthM` (aspect) and `bedJointThicknessM` (GP 10 mm) so sequential clears | `:903–917` — `unitWidthM` at_least `(unit_height/2.4).max(0.100)`; joint exactly `0.010` for GP |
| Flexure: thickness `t·√(M_Ed/M_Rd)·1.01`; always emit `f_yd` with `As,h` | `:1190–1207` |
| Concentrated: primary `bearingAreaM2` bound `A·u·1.01` at fixed `β` | `:1235–1242` — `area_req = area_used * (f_ed/n_rdc).max(1.0) * 1.01` |
| Regression mirrors cross-family gate flip/sequential rule | `:117–170` — passed at runtime |

Evaluate module: `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️masonry/🦀️.rs`.

---

## Blocking fix list

None

---

## Non-blocking observations

- Material gate id still requires **sequential** application of all five applicables; solo remedies fix only one of two independent conformance failures — acceptable per gate sequential path (`:156–167`).
- Flexure `asHorizontalM2` solo still lands in `u=1.5` Fail branch when `f_yd` stays zero; thickness remedy[0] clears first, so gate passes without relying on reinforcement pair.
- Implementer 145/145 full-suite claim not independently rerun (`--no-fail-fast`); targeted gate regression + source audit suffice for this remedy-clear audit.

---

*Verifier: read-only subagent · ticket `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS`*
