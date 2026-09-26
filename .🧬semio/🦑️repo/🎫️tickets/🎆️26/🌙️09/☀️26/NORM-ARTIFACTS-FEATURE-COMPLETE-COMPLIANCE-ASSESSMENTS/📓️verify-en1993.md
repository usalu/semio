# Verify — EN 1993 (`🔩️en1993`)

**Verifier:** Wave D adversarial (read-only)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1993.md` — 144/144, no gaps  
**Test run (fresh, `--skip-nx-cache -- --no-fail-fast`):** `Summary [   0.614s] 144 tests run: 144 passed, 0 skipped`  
**Logs:** `🗑️generated/verify-en1993/`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| R1 | **FAIL** | 11 |
| R2 | **FAIL** | 8 |

R1 blockers on remedies, oracle gate, field meta, examples, facets, paths, slip, LTB curve, and localization are largely fixed (144/144, taxonomy 49 en1993 leaves). R2 fails on **scope-aware perturbation coverage** (parts 2–6 entity leaves never appear in committed examples), **special-entity action combination** (γ_Q scaling instead of EN 1990), **ignored editable fields**, residual **localization/tautology** issues, and **fatigue without N**.

---

## VERDICT: FAIL (8 blocking)

---

## Check table (brief §1–10 + 7b)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PARTIAL FAIL** | Hierarchical `En1993Snapshot` with load cases + characteristic `memberActions` combined in `combine_member_actions` (`🧬️schema/🦀️.rs:1126–1243`). **Gap:** tower/pile/crane/bridge/joint checks use characteristic scalars × `gamma_q(annex)` only (`:1256–1258`, `:1763+`, `:2420–2422`, `:2465`, `:2522`, `:2541`) — not EN 1990 governing combinations. Several editable fields never read (see 13:43). |
| 2 | Clause coverage | **PARTIAL FAIL** | Core 1-1 checks, slip §3.9 (`:1866+`), LTB Table 6.4 (`:504–514`, `:1333`), M+N 6.31–6.41 (`:577+`), fire k_y,θ/χ_fi (`:663+`, `:2037–2048`). **Missing:** fatigue limit ignores `cycles` N — no (N/2×10⁶)^(1/m) scaling (`:1983–2014` uses Δσ_C/γ_Mf only). Classification u=class/3 with `warn_above(0.99)` cannot fail for class 1–3 (`:1300–1316`). |
| 3 | Numerics (≥3 hand checks) | **PASS** | Hand recompute in `🗑️generated/verify-en1993/hand-numerics.txt`: N_Rd=3763 kN, χ≈0.597, LTB curve A, bolt Fb≈247 kN, fatigue cat 71, ULS N≈200 kN, θ_cr≈584.7 °C. Oracle test `python_oracle_and_jsonschema_agree_within_half_percent` asserts ±0.5 %. |
| 4 | Applicability | **PASS** | Empty subject → `NotApplicable` (`:1275+`); members without ULS actions gated. |
| 5 | National annex DE vs EN | **PASS** (partial) | `AnnexParams::de()` γ_M1=1.1 vs EN 1.0; test `de_gamma_m1_raises_buckling_utilization_vs_en`. **Note:** `design_gamma` ignores annex (`:1121–1123` `let _ = annex`) — combination γ_G/γ_Q identical EN/DE (material partial factors still diverge). |
| 6 | Report quality | **PARTIAL FAIL** | `[id=…]` paths + `every_emitted_path_parses_and_resolves` (`🧪️tests/⚖️compliance/🦀️.rs:186–208`). Two remedy-law tests pass. **Residual:** identical en/de explanations e.g. bridge η (`🦀️.rs:2432`), crane σ (`:2552–2554`). |
| 7 | Examples | **PARTIAL FAIL** | DSL decode + verdict tests pass (`📚️examples/*/🧪️tests/🧩️example/🦀️.rs`). **Gap:** both committed assets leave 8 part-scoped entity lists empty (`🖼️assets/*/snapshot.json` — `coldFormedMembers`…`craneRunways` all `[]`); mutation fixtures populate them but are not committed examples. |
| 7b | Inputs UX | **PASS** | `lookup_norm_field_meta` + human `NormFieldChoice` (`✏️editor/🏷️field-meta/🦀️.rs:9–70`, `:242–244`); `field_meta_covers_every_editable_leaf_en_de_unit`. Minor: some id labels `"Id"`/`"Id"` (`:223`, `:232`). |
| 8 | Mutations & schema | **PASS** | 49 en1993 taxonomy leaves (`🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` — 49× `"artifact": "🔩️en1993"`). Snapshot `🔣️.json` uses `definitions` + `$ref` (no bare array-item objects). TS snapshot typed (`📸️snapshot/🟦️.ts`). Text-guard `Record<string, unknown>` is runtime guard only, not snapshot API. |
| 9 | Tests | **PARTIAL FAIL** | **144/144 passed, 0 skipped** (`test-r2.txt`). Oracle + jsonschema in gate (`python_oracle_and_jsonschema_agree_within_half_percent`). **Residual trivial:** `✏️editor/🧪️tests/🔬️unit/🦀️.rs:230` `!report.checks.is_empty()` only. Perturbation test allows ≤3 unchanged leaves (`:386–393`) but does not cover part-scoped entities at all. |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!`. Catalogue panel uses `render_catalogue` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:18–20`). Mutation transport placeholder comments in `🧪️tests/🔩️mutate-en1993-1/` are envelope-level, not evaluate stubs. |

---

## Round-1 blocking re-check (11 items)

| # | R1 item | R2 | Evidence |
|---|---------|-----|----------|
| 1 | `next_section_options` emits section ids | **FIXED** | `🧬️schema/🦀️.rs:1080–1085` `.map(\|s\| s.id)`; test `remedy_law_oneof_section_id_flips_axial_or_buckling_to_pass` `:139` |
| 2 | Second fail→pass remedy test | **FIXED** | `remedy_law_bolt_shear_rows_flips_to_pass` `🧪️tests/⚖️compliance/🦀️.rs:158–182` |
| 3 | Oracle + jsonschema in nx gate | **FIXED** | `python_oracle_and_jsonschema_agree_within_half_percent` `:229–275` |
| 4 | Human annex + enum `NormFieldChoice` | **FIXED** | `🏷️field-meta/🦀️.rs:9–70` |
| 5 | `lookup_norm_field_meta` + `[]` + leaf walk | **FIXED** | `:242–244`, `field_meta_covers_every_editable_leaf_en_de_unit` |
| 6 | Example DSL decode + verdicts | **FIXED** | `📚️examples/✅️heb240-compliant/…/🦀️.rs`, `🔩️high-strength-connection/…/🦀️.rs` |
| 7 | Nested snapshot JSON facets | **FIXED** | `📸️snapshot/🔣️.json` `definitions` + `$ref` for all entity types |
| 8 | Emitted path resolution test | **FIXED** | `every_emitted_path_parses_and_resolves` |
| 9 | Slip-resistant bolt check | **FIXED** | `slip_resistant_category_c_produces_slip_check`; `part_1_8::slip_resistance_n` `:917–918` |
| 10 | LTB Table 6.4 curve selection | **FIXED** | `ltb_curve_table_6_4` `:504–514`; test `:211–214` |
| 11 | Distinct de explanations / labels | **NOT FIXED** | Bridge η identical en/de `🦀️.rs:2432`; crane/web stress `:2552–2554`; silo label `:2261` "Silo"/"Silo" |

---

## CORRECTION 13:27 — recurring failure causes (12)

| # | Cause | Result | Evidence |
|---|--------|--------|----------|
| 1 | Human en+de choice labels | **PASS** | `ANNEX`, `MEMBER_TYPE`, `JOINT_CATEGORY`, etc. `🏷️field-meta/🦀️.rs:9–70` |
| 2 | Every editable leaf has meta + leaf test | **PASS** | `field_meta_covers_every_editable_leaf_en_de_unit`; `every_default_snapshot_editable_leaf_has_en_de_meta` |
| 3 | Structured inputs, not JSON dump | **PASS** | `render_document_editor` |
| 4 | Entity paths `[id=…]` + path resolve test | **PASS** | `every_emitted_path_parses_and_resolves` |
| 5 | ≥2 distinct fail→pass remedy-apply tests | **PASS** | Section OneOf + bolt rows + buckling length (`:88–182`) |
| 6 | Example tests decode DSL + assert verdicts | **PASS** | Both example tests |
| 7 | Rust runs Python oracle + jsonschema | **PASS** | `python_oracle_and_jsonschema_agree_within_half_percent` |
| 8 | Facets regenerated, no bare object | **PASS** | `📸️snapshot/🔣️.json` nested definitions |
| 9 | No tautologies / ignored fields | **FAIL** | Classification u=class/3 (`:1314–1315`); unread: `pitch`, `memberType`, `momentDiagram`, `cycles`, `designTemperature` (audit below) |
| 10 | No trivially-true tests | **PARTIAL FAIL** | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:230` |
| 11 | Semantic mutation verbs | **PASS** | `update-bolt-inputs`, `change-annex`, etc. |
| 12 | Dynamic text localized (no copy) | **PARTIAL FAIL** | `🦀️.rs:2432`, `:2552–2554`, `:2261` |

---

## CORRECTION 13:43 — scope-aware perturbation & structural actions

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Members use EN 1990 combinations | **PASS** | `combine_member_actions` + `governing_effects`; test `en1993_combinations_form_uls_from_characteristic_actions` |
| Special entities (tower/pile/crane/bridge/joint) use combinations | **FAIL** | `tower.n_k * gamma_q` `:2465`; `pile.n_k * gamma_q` `:2522`; `crane.wheel_force_k * gamma_q` `:2541`; `joint.shear_force` × γ_Q `:1763+`; bridge uses raw `action_row.action` `:2420–2422` |
| Perturb every editable leaf in committed example where it applies | **FAIL** | Test `perturb_every_editable_leaf_in_committed_examples_changes_a_check` only walks `compliant_heb240_frame` + `noncompliant_overloaded_frame` (`:350–395`). Both builders + both `🖼️assets/*/snapshot.json` have **empty** `coldFormedMembers`, `platedPanels`, `siloShells`, `tensionComponents`, `bridgeFatigue`, `towerLegs`, `piles`, `craneRunways`. Mutation fixture snapshots (e.g. `🧫️fixtures/…/update-bridge-inputs/…/before/🔣️.json`) populate these but are **not** committed examples or perturbation subjects. |
| Every editable field read by ≥1 check | **FAIL** | Static audit `🗑️generated/verify-en1993/ignored-fields-audit.txt`: `joints[].pitch` (bearing uses `end_distance`/`edge_distance` only `:1763–1764`), `members[].memberType`, `members[].momentDiagram` (not in `m_cr_rolled_nm` `:480–499`), `fatigueDetails[].cycles` (`:1987–1997` no N), `fireExposures[].designTemperature` (computed `steel_temperature_c` `:2020–2027`, input ignored) |
| N/A-in-default leaves not exempt | **FAIL** | 8 part-scoped entity kinds never instantiated in committed examples despite declared scope parts 2–6 |

**Exempt descriptive labels (explicit):** `.id`, `.label`, `.name`, `.designation` — per perturbation test `:364–367`.

---

## Taxonomy findings

| Check | Result |
|-------|--------|
| `mutation-leaf-taxonomy-generate` | **PASS** — `537 payloads`, no en1993 skip (`taxonomy.txt`) |
| en1993 leaves in fixture | **49** rows `"artifact": "🔩️en1993"` in `📇️mutation-leaf-taxonomy-v1/🔣️.json` |
| en1991 rows intact after regen | **PASS** — **80** `"artifact": "🏋️en1991"` rows remain (unchanged count class) |

---

## Mandatory family checks (DIN EN 1993 + NA)

| Requirement | Result | Notes |
|-------------|--------|-------|
| §5.2 section classification | **PARTIAL** | Real Table 5.2 class; check limit class/3 is informational only for class 1–3 |
| §6.2.1–6.2.6 N/V/M | **PASS** | From governing ULS combinations |
| §6.2.9/6.2.10 M+N | **PASS** | `mn_interaction_eta` 6.31–6.41 `:577+` |
| §6.3.1 χ + curves | **PASS** | Computed, not user-supplied |
| §6.3.2 LTB χ_LT Table 6.4 | **PASS** | `ltb_curve_table_6_4` |
| §6.3.3 interaction 6.61/6.62 | **PASS** | `interaction_kij` + `interaction_eta` |
| γ_M0/γ_M1/γ_M2 DE | **PASS** | `AnnexParams` |
| Slip-resistant bolts | **PASS** | Category B/C + `slip_resistance_n` |
| Fatigue Δσ_C, γ_Mf, **N** | **PARTIAL FAIL** | γ_Mf + category OK; **N/cycles not in limit** |
| Parts 2/3/4/5/6 checks in evaluate | **PASS** (code) / **FAIL** (examples) | Logic present; no committed subject exercises them |
| EN 1990 combinations for all action inputs | **PARTIAL FAIL** | Members yes; special entities γ_Q-only |

---

## Blocking fix list

1. **`📚️examples/` + `🖼️assets/` + `perturb_every_editable_leaf_in_committed_examples_changes_a_check`**  
   Add committed part-scoped examples (at minimum one each for bridge fatigue, tower leg, pile, crane runway, cold-formed, plated panel, silo shell, tension component) with realistic data; include them in the perturbation test loop. Both current examples keep eight entity lists empty (`snapshot.json` lines 151–158).

2. **`🧬️schema/🦀️.rs` — special-entity design actions**  
   Route `towerLegs[].nK`, `piles[].nK`, `craneRunways[].wheelForceK`, `joints[].shearForce`/`tensionForce`, and bridge `memberActions` through EN 1990 (+ DE NA) combination logic (or linked load cases per entity), not `× gamma_q(annex)` alone. Report governing combination in explanation.

3. **`🧬️schema/🦀️.rs` — unread editable fields**  
   - `joints[].pitch` → use in `bearing_k1` / `bearing_alpha_b` (Table 3.4 p₁) or remove from schema.  
   - `members[].momentDiagram` → feed `m_cr_rolled_nm` (§6.3.2.3 diagram factors).  
   - `members[].memberType` → gate applicable checks or drive curve selection.  
   - `fatigueDetails[].cycles` → limit = Δσ_C·(N_ref/N)^(1/m)·γ_Mf⁻¹.  
   - `fireExposures[].designTemperature` → use in check or drop from editable snapshot.

4. **`🧬️schema/🦀️.rs:1983–2014` — fatigue with N**  
   Implement Palmgren-Miner / S-N curve with `cycles` input; add numeric test at N=2×10⁶.

5. **`🧬️schema/🦀️.rs:1300–1316` — classification check**  
   Replace u=class/3 informational tautology with real limit (class ≤ 3 required) so class 4 fails with section remedy.

6. **`🧬️schema/🦀️.rs:2432`, `:2552–2554`, `:2261` — localization**  
   Replace identical en/de strings (bridge η, crane σ, silo labels) with proper German engineering text.

7. **`✏️editor/🧪️tests/🔬️unit/🦀️.rs:230`**  
   Replace `!report.checks.is_empty()` with assertion on specific check id/count from default evaluate.

8. **`🧬️schema/🦀️.rs:1121–1123` — `design_gamma(annex)`**  
   Apply DE NA combination factors where they differ from EN recommended values (currently `let _ = annex`); add DE-vs-EN combination test if factors diverge.

---

## Non-blocking observations

- Impl claim 144/144 accurate for executed tests; "no gaps" overclaimed.
- Mutation fixture snapshots contain full multi-part subjects — good seed for committed examples.
- Fire path now uses `k_y_theta`/`k_e_theta`/`chi_fi` incremental `steel_temperature_c` (R1 surrogates addressed).
- `piles[].soilReduction` is derived via `pile_soil_reduction` from driving stress/geometry when checks run (`:2512`).
- Hand numerics: `🗑️generated/verify-en1993/hand-numerics.txt`
- Ignored-field audit: `🗑️generated/verify-en1993/ignored-fields-audit.txt`
- Test log: `🗑️generated/verify-en1993/test-r2.txt`
- Taxonomy log: `🗑️generated/verify-en1993/taxonomy.txt`

---

## Manual verification log

| Command | Result |
|---------|--------|
| `bun nx run @semio-tech/norm-en1993-rs:test --skip-nx-cache -- --no-fail-fast` | `144 tests run: 144 passed, 0 skipped` |
| `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` | `537 payloads`; en1993 not skipped; en1991 80 rows intact |
