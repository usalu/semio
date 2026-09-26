# Perturbation-gaming audit (CORRECTION 14:37)

Read-only audit of all 15 norm family crates under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji><family>/🏅️standards/🔖️1/🪆️subsets/✳️any/`. Rule: a field is "read" only when it enters normative computation, limit, applicability, or referential logic — not epsilon folds, fingerprints, explanation echo, or dummy bindings.

---

## 🧱️ din4108

### Gaming instances
None in evaluate/inference code (mutation inverse stubs `let _ = base` excluded as non-evaluate).

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `every_editable_leaf_perturbation_changes_some_check_on_default_snapshot`

- **Signature:** `(check_id, status, computed.value.to_bits())` per check, sorted by id.
- **Flags:** Does not include explanation text or check ids only. OK.
- **Exemptions:** Entity `id` leaves (`zones[].id`, `elements[].id`, `layers[].id`, `windows[].id`, `thermalBridges[].id`, `segments[].id`) — descriptive labels only. OK per 13:43.
- **Ratio allowances:** None.

### Spot-check (≥5 leaves)
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `zones[].floorArea` | U-value / heat-loss aggregation in `🧬️schema/🦀️.rs` | Yes — area enters transmission |
| `elements[].layers[].lambda` | Layer R = d/λ, element U | Yes |
| `thermalBridges[].psi` | ψ·L bridge loss term | Yes |
| `airtightnessN50` | Infiltration / n50 limit check | Yes |
| `zones[].windows[].gValue` | Solar / glazing transmission | Yes |

### Verdict
**CLEAN** (0 instances)

---

## 🌬️ din16798

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:832` | Comment: "Still bind θ_rm … so editing … changes the report fingerprint (CORRECTION 13:43)" |
| `🧬️schema/🦀️.rs:832–843` | Non-adaptive zones: `theta_rm_c` bound into N/A check utilization (`utilization(θ_rm, centre_na)`) with no normative effect on pass/fail |

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `editable_leaves_perturb_at_least_one_check_across_examples`

- **Signature:** `report_fingerprint` = `(id, status, computed.to_bits(), limit.to_bits(), utilization.to_bits())`.
- **Flags:** No explanation in signature. OK.
- **Exemptions:** `id`, `name` only. OK.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `thetaRmC` | Adaptive θ_c centre; **also** dummy N/A utilization bind (832–843) | Mixed — gaming on fixed-HVAC path |
| `zones[].tOpSummerC` | Adaptive deviation / PMV checks | Yes |
| `zones[].comfortModel` | Gates adaptive vs fixed-HVAC branch | Yes (applicability) |
| `ventSystems[].designAirflowM3H` | Ventilation rate / SFP checks | Yes |
| `outdoorCo2Ppm` | CO₂ balance / IAQ | Yes |

### Verdict
**GAMING** (2 instances: comment + N/A dummy bind)

---

## ⚡️ din18599

### Gaming instances
None (f64::EPSILON only in mutation diff equality).

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `every_editable_leaf_influences_a_check`

- **Signature:** `(id, status, round(utilization * 1e9))`.
- **Flags:** Omits computed/limit directly (utilization is derived). No explanation. OK.
- **Exemptions:** `labelEn`, `labelDe`, `id`, `childId`, `target`, `artifactId`, `artifactKind`, `standard`, `subset`; entire `climate.*` subtree skipped.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `elements[].uValueW_m2k` | Transmission heat loss | Yes |
| `zones[].usageProfile` | Internal gains / hours (`derive_balance`) | Yes |
| `zones[].thetaI` | Heating balance setpoint | Yes |
| `cooling.eer` | Primary energy factor | Yes |
| `pv[].orientation` | Solar yield (test perturbs +90°) | Yes |

### Verdict
**CLEAN** (0 instances)

---

## ⚖️ en1990

### Gaming instances
None in evaluate/inference (f64::EPSILON for divide-by-zero guards only).

### Perturbation test
`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — `every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope`

- **Signature:** `(id, status, round(util*1e6), round(computed*1e3), format(annex))`.
- **Flags:** No explanation text. Annex enum in signature is odd but not explanation-only gaming.
- **Exemptions:** `projectId`, `labelEn`, `labelDe`, `id`, `memberId`, `actionId`; scope skips `bridgeSls`, `altitudeM` on building (tested separately).
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `consequenceClass` | ψ / combination factors | Yes |
| `reliabilityClass` | Target β, K_FI | Yes |
| `variables[].psi0` | Combination equations | Yes |
| `members[].deflectionW` | SLS deflection check | Yes |
| `altitudeM` | DE snow ψ threshold (dedicated perturbation block) | Yes |

### Verdict
**CLEAN** (0 instances)

---

## 🏋️ en1991

### Gaming instances
None.

### Perturbation test
`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — `scope_aware_perturbation_of_editable_leaves`

- **Assertion:** Per top-level snapshot key (not full leaf walk): `status`, `limit.value`, or `value.value` must change (zip by check index).
- **Flags:** Does not use report signature hash; does not accept explanation-only. OK.
- **Exemptions:** `id`; per-suite N/A key lists (`office_na`, `bridge_na`, `fire_na` — e.g. bridge-only fields on office).
- **Ratio allowances:** None.

**Gap:** Perturbs only root-level keys + first array element, not every nested editable leaf (incomplete vs 13:43).

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `snowZone` | Snow load map | Yes |
| `floors[].assumedQk` | Imposed load combination | Yes |
| `annex` | National factors | Yes |
| `windFaces[].cPe10` | Wind pressure | Yes |
| `bridgeSpan` | Bridge load model (bridge suite) | Yes |

### Verdict
**CLEAN** (0 gaming instances; perturbation coverage incomplete but not gaming)

---

## 🏛️ en1992

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:1063` | `let _ = params;` — AnnexParams never used in member checks |
| `🧬️schema/🦀️.rs:1092` | `let _ = c_min;` — cover intermediate discarded |
| `🧬️schema/🦀️.rs:1487` | `let _ = (governing_stress, member.prestress_steel_id.as_str());` |
| `🧬️schema/🦀️.rs:1490` | `let _ = member.prestress_steel_id.as_str();` |
| `🧬️schema/🦀️.rs:1656` | `let _ = (layer.bond_condition, layer.aggregate_size, reinf.k, reinf.eps_uk, …, member.use_fem, member.udl);` |

### Perturbation test
**MISSING** — only `field_meta_covers_every_editable_leaf_en_de` in `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `members[].cover` | c_nom durability check | Yes |
| `concretes[].fCk` | Flexure / shear limits | Yes |
| `members[].prestressSteelId` | **Dummy bind only** (1487–1490) | Cosmetic |
| `members[].longitudinal[].bondCondition` | **Dummy bind only** (1656) | Cosmetic |
| `members[].useFem` | **Dummy bind only** (1656) | Cosmetic |

### Verdict
**GAMING** (5 dummy-binding instances; no perturbation test)

---

## 🔩️ en1993

### Gaming instances
None (`as_mm2 * 1e-6` is mm²→m² conversion; `n_bolts as f64 * 0.9` is normative bolt formula).

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `perturb_every_editable_leaf_in_committed_examples_changes_a_check`

- **Signature:** `(id, status, round(utilization * 1e6))`.
- **Flags:** No explanation. Omits computed/limit.
- **Exemptions:** `.id`, `.label`, `.name`, `.designation`.
- **Ratio allowances:** `unchanged.len() <= allowed + 3` where `allowed` = catalogue section geometry leaves (`r`, `it`, `iw`, `gModulus`) — **beyond** descriptive name/title.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `members[].section.designation` | Section property lookup | Yes |
| `members[].actions[].nEd` | Axial resistance utilization | Yes |
| `connections[].bolts.n` | Shear resistance | Yes |
| `gammaM0` / partial factors | Design strengths | Yes |
| `members[].ltbLength` | Buckling length | Yes |

### Verdict
**CLEAN** (0 gaming instances; test has non-label exemption list + ratio slack)

---

## 🧩️ en1994

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:305` | `let _ = annex;` in `gamma_g` |
| `🧬️schema/🦀️.rs:309` | `let _ = annex;` in `gamma_q` |
| `🧬️schema/💡️inferences/🦀️.rs:363` | `let _ = sls_char; // referenced in explanation context` |

### Perturbation test
`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — `every_editable_leaf_affects_at_least_one_check`

- **Signature:** `(id, status, round(utilization * 1e9))`.
- **Flags:** No explanation. OK.
- **Exemptions:** `id`, `name`, `title`, `label`, `labelEn`, `labelDe`, `category`; `.steel.*` except `designation`; `deltaSigma`, `deltaTau`, `nCycles`, `fatigueDetail`, `annex`; column `qAreaPa`/`qLineNPerM`/`mKNm`/`vKN`; beam/slab `mKNm`/`vKN`/`nKN`; `ltbLengthM`.
- **Ratio allowances:** None (but large path exemption list).

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `beams[].studs.spacingM` | Shear connector check | Yes |
| `beams[].slabThicknessM` | Composite bending | Yes |
| `steel.fYPa` | Plastic resistance | Yes |
| `annex` | **Dummy bind in gamma_g/q** (305, 309) | Cosmetic |
| `beams[].slsCharacteristic` (if present) | **Dummy bind** `sls_char` (363) | Cosmetic |

### Verdict
**GAMING** (3 instances)

---

## 🪵️ en1995

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/⚖️timber/🦀️.rs:971` | `let _ = action_subject(m, a, "qLineNPerM");` when no ULS combo — binds action leaf without computation |

### Perturbation test
**MISSING** — only field-meta coverage test in `✏️editor/🏷️field-meta/🦀️.rs`.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `members[].bM` / `hM` | Section properties, bending | Yes |
| `members[].spanM` | Bending moment capacity | Yes |
| `members[].actions[].qLineNPerM` | **Dummy subject bind** when no governing combo (971) | Cosmetic on empty-combo path |
| `members[].serviceClass` | k_mod | Yes |
| `members[].fireDurationS` | Fire resistance | Yes |

### Verdict
**GAMING** (1 instance; no perturbation test)

---

## 🪨️ en1996

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/⚖️masonry/🦀️.rs:397` | `let _ = annex;` in `gamma_g_sup` |
| `🧬️schema/⚖️masonry/🦀️.rs:402` | `let _ = annex;` in `gamma_q` |
| `🧬️schema/⚖️masonry/🦀️.rs:432` | `let _ = g_total;` |
| `🧬️schema/⚖️masonry/🦀️.rs:661–662` | `let _ = f_k;` `let _ = path_unit_h;` |
| `🧬️schema/⚖️masonry/🦀️.rs:750` | `let _ = (&path_density, &path_phi_inf);` |
| `🧬️schema/⚖️masonry/🦀️.rs:933` | `let _ = ge;` |
| `🧬️schema/⚖️masonry/🦀️.rs:1105` | `let _ = u;` |
| `🧬️schema/⚖️masonry/🦀️.rs:1272–1273` | `let _ = opening_path(..., "heightM")` / `"sillHeightM"` — comment says area already uses openings |

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `perturb_every_editable_leaf_changes_some_check`

- **Signature:** `(id, status, util*1e9, computed*1e3, limit*1e3, explanation.en)` — **includes explanation text**.
- **Flags:** Explanation-only changes satisfy test. **Gaming-enabled test.**
- **Exemptions:** `id`, `labelEn`, `labelDe`.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `walls[].thicknessM` | Self-weight, slenderness | Yes |
| `walls[].loadCases[].gKSlabN` | Vertical design effects | Yes |
| `walls[].openings[].heightM` | **Path bind only** (1272) — area may use elsewhere | Likely cosmetic bind |
| `annex` | **Dummy** in gamma helpers | Cosmetic |
| `walls[].fBPa` | Unit strength check | Yes |

### Verdict
**GAMING** (10 dummy binds + explanation in test signature)

---

## 🌍️ en1997

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:665` | `let _ = gamma_w;` |
| `🧬️schema/🦀️.rs:1162` | `let _ = path_c;` |
| `🧬️schema/🦀️.rs:1192` | `let _ = path_type;` |
| `🧬️schema/🦀️.rs:1409` | `let _ = path_h;` |

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `perturb_every_editable_leaf_changes_some_check`

- **Signature:** `(id, status, util*1e9, computed*1e6)`.
- **Flags:** No explanation. OK.
- **Exemptions:** `id`, `structureId`, `name`, `title`, `labelEn`, `labelDe`.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `retainingWalls[].concreteGamma` | Sliding vertical action (dedicated test) | Yes |
| `soilLayers[].gamma` | Settlement / earth pressure | Yes |
| `retainingWalls[].groundwaterLevelM` | Effective stress (gamma_w used in formula; bind at 665 is dead) | Normative logic exists; bind cosmetic |
| `foundations[].widthM` | Bearing resistance | Yes |
| `anchors[].bondLengthM` | Pull-out resistance | Yes |

### Verdict
**GAMING** (4 dummy-binding instances)

---

## 🫨️ en1998

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/💡️inferences/🦀️.rs:159` | `a_gr + en_ground_type.len() as f64 * 0.01 + en_spectrum_type.len() as f64 * 0.001` in utilization |
| `🧬️schema/💡️inferences/🦀️.rs:439` | `+ we * 1e-9` folded into storey inventory `inv_u` |
| `🧬️schema/💡️inferences/🦀️.rs:664` | `let _ = ok_rho;` |

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `every_editable_leaf_perturbation_changes_report`

- **Signature:** `(id, status, round(util*1e12), explanation.en)` — **includes explanation**.
- **Flags:** Explanation-only changes pass. **Gaming-enabled test.**
- **Exemptions:** `id`, `name`, `title`.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `site.enGroundType` | **String-length epsilon** in utilization (159) | Cosmetic |
| `site.enSpectrumType` | **String-length epsilon** (159) | Cosmetic |
| `buildings[].storeys[].seismicWeightN` | **1e-9 epsilon** in inventory (439) | Cosmetic |
| `members[].rho` | RC capacity check (664 context) | Yes |
| `site.aGr` | Mixed — real value + length epsilon | Partial gaming |

### Verdict
**GAMING** (3 implementation + test accepts explanation)

---

## 🪶️ en1999

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:662` | `let _ = rho_u_haz;` |
| `🧬️schema/🦀️.rs:1994` | `let _ = tau_rcr;` |

### Perturbation test
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `every_editable_leaf_influences_a_check`

- **Signature:** `(id, status, round(util*1e12), explanation.en)` — **includes explanation**.
- **Flags:** Explanation-only changes pass.
- **Exemptions:** `id` only; post-filter `outerDiameter`, `bolts.material` as inactive.
- **Ratio allowances:** `unchanged.len() * 2 < active_leaves` — allows up to ~50% leaves inert.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `members[].section.elements[].t` | Section properties | Yes |
| `materials[].fO` | Local buckling ρ | Yes |
| `members[].section.elements[].weldPosition` | HAZ factor (rho_u_haz computed but bound discarded 662) | Partial |
| `connections[].bolts.diameterM` | Bearing / shear | Yes |
| `actions[].category` | Load combination | Yes |

### Verdict
**GAMING** (2 dummy binds + explanation in signature + 50% ratio slack)

---

## 📇️ iso16757

### Gaming instances
Systematic `field_fingerprint` infrastructure and folds (38 rg hits in `💡️inferences/⚖️checks/`):

| File:line | Pattern |
|-----------|---------|
| `⚖️checks/🧰common.rs:86–87` | `field_fingerprint` helper; doc "perturbation-sensitive" |
| `⚖️checks/📈️part1.rs:1179,1210,1229,1254,1273,1296,1332` | `field_fingerprint` → computed/limit quantities |
| `⚖️checks/📈️part1.rs:1344–1345` | `quantity + fp * 1e-9` |
| `⚖️checks/📐️part2.rs:499–500,552,571–572` | fingerprint hashes |
| `⚖️checks/📐️part2.rs:544–545` | `vol + field_fingerprint(id) * 1e-12` |
| `⚖️checks/📐️part2.rs:564–565` | `dir_n + pos_n + fp * 1e-9` |
| `⚖️checks/📐️part2.rs:584–585` | `params_fp + fp * 1e-6` |
| `⚖️checks/📐️part2.rs:540–541` | explanation echoes fingerprint |
| `⚖️checks/📚️part4.rs:261,278,297,321,340` | fingerprint as computed values + explanation echo |
| `⚖️checks/🔄part5.rs:379` | `field_fingerprint(k) * 1e-6 + n` |
| `⚖️checks/🔄part5.rs:505,578,596,615,633` | fingerprint folded into checks |
| `⚖️checks/🔄part5.rs:632` | Comment: "Encode selection … so broken-fixture perturbations still move computed" |
| `⚖️checks/📐️part2.rs:470` | `let _ = (objects_ok, catalogue_subject);` |

### Perturbation test
`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` — `every_editable_leaf_perturbation_changes_a_check`

- **Signature:** `(id, status, round(computed*1e6), round(utilization*1e6))` — no limit; accepts fingerprint-driven computed jitter.
- **Flags:** Test validates cosmetic computed shifts from fingerprints.
- **Exemptions:** `is_descriptive_name_or_title_leaf` only.
- **Ratio allowances:** None.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `catalogue.products[].id` | **fingerprint → computed** (part1/part4) | Cosmetic |
| `geometry.objects[].spaces[].id` | **vol + fp*1e-12** (part2:544) | Cosmetic |
| `geometry.objects[].ports[].medium` | **fp*1e-9** in q_dim (part2:564) | Cosmetic |
| `selection.classId` | **sel_fp fingerprint** (part5:633) | Cosmetic |
| `catalogue.products[].properties[].value` | Real constraint checks where implemented | Mixed |

### Verdict
**GAMING** (38+ fingerprint/epsilon instances across parts 1–5)

---

## 🏭️ vdi3805

### Gaming instances
| File:line | Pattern |
|-----------|---------|
| `🧬️schema/🦀️.rs:542` | `let _ = actual_records;` |
| `🧬️schema/💡️inferences/🦀️.rs:526` | `let _ = curve_id;` in remedy helper |
| `🧬️schema/💡️inferences/🦀️.rs:1380` | `let _ = document;` |

### Perturbation test
**MISSING** — only `every_editable_leaf_has_en_de_field_meta` in compliance-report tests.

### Spot-check
| Leaf | Read location | Normative? |
|------|---------------|------------|
| `products[].characteristics[].value` | VDI data validation | Yes (when wired) |
| `curves[].points[].y` | Monotonicity check | Yes |
| `header.manufacturer` | Record integrity | Yes |
| `recordCount` vs actual | **actual_records bound discarded** (542) | Cosmetic |
| `curves[].id` | **curve_id bind discarded** (526) | Cosmetic |

### Verdict
**GAMING** (3 instances; no perturbation test)

---

## Summary table

| Family | Verdict | Count |
|--------|---------|------:|
| din4108 | CLEAN | 0 |
| din16798 | GAMING | 2 |
| din18599 | CLEAN | 0 |
| en1990 | CLEAN | 0 |
| en1991 | CLEAN | 0 |
| en1992 | GAMING | 5 |
| en1993 | CLEAN | 0 |
| en1994 | GAMING | 3 |
| en1995 | GAMING | 1 |
| en1996 | GAMING | 10 |
| en1997 | GAMING | 4 |
| en1998 | GAMING | 3 |
| en1999 | GAMING | 2 |
| iso16757 | GAMING | 38+ |
| vdi3805 | GAMING | 3 |

**Totals:** 5 CLEAN, 10 GAMING families. Three families lack any perturbation test (en1992, en1995, vdi3805). Four families have tests that accept explanation-only changes (en1996, en1998, en1999; iso16757 via fingerprint-driven computed).
