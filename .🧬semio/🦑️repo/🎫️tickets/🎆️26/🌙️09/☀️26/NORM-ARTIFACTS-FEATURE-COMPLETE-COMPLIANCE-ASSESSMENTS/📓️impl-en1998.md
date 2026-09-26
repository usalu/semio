# Impl — EN 1998 (`en1998` / `semio-s-artifact-norm-en1998`)

Wave C rebuild + Wave D Round 1 + **Round 2** — **DONE**.

## Round 2 (7 blocking)

1. **Seismic mass from actions** — Removed `storeys[].massKg` / `weightN`. Storeys carry `permanentGkN` + `variables[]` (`category`, `qkN`) and `correlatedOccupancy`. `evaluate()` forms `m = (ΣG_k + Σψ_E,i·Q_k,i)/g` with `ψ_E = φ·ψ₂` (EN 1998-1 §3.2.4 / Table 4.2; EN 1990 DE NA ψ₂) before base shear / storey forces / P-Δ.
2. **Parts 2–6 demands from spectrum + model** — Bridge shear/bearing from T + S_d; silo/tank anchorage from impulsive/convective V; tower overturning from V·(2/3)H; foundation sliding from supported-building V_b; assessment E_d from supported building × LS a_g factor. Hand-typed `bearingDEdM` / `vEdN` / `hEdN` / `mEdNm` / `eDN` removed (resistances remain).
3. **`multipleResistingSystems`** — Real §5.2.2.1 dual (frame+wall) + §4.2.3.1 plan-regularity check with Pass/Fail + remedies.
4. **`assessments[].limitState`** — Selects NC/SD/DL (Table 2.1 + DE NA): a_g factor 1.72 / 1.0 / 0.5; CF suppressed for DL; capacity check uses derived E_d.
5. **Facets** — `🟦️.ts` / `🔗️.graphql` / `🛰️.proto` regenerated to match `🔣️.json` (typed nested interfaces; no `Record<string, unknown>`).
6. **Plan dimensions** — Removed `plan_w = 24.0` fallback; missing/zero `planWidthM`/`planLengthM` → Fail + remedy.
7. **Foundation sliding** — H_Ed = supported building base shear (EN 1998-5 §5.3), not hand-typed `hEdN`.

## Subject

- Nested `En1998Snapshot` (buildings/systems/storeys/members + parts 2–6), SI throughout.
- `DeSeismicZone` / `DeGroundCombo`; `site.aGr` Warning-on-mismatch in DE.
- Storey characteristic actions; bridge `fundamentalPeriodS`; assessment `supportedBuildingId`; tower `heightM`.

## Tests

**Runner:** `bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast`

```text
Summary [   2.349s] 63 tests run: 63 passed, 0 skipped
```

## Remaining gaps

None.

## Round 3 — Wave D blockers + coordinator tightenings

**Runner:** `bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   0.571s] 68 tests run: 68 passed, 0 skipped`  
**Also:** `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` (success)

### Blocking items

| # | Item | Mapping |
|---|------|---------|
| 1 | `every_editable_leaf_perturbation_changes_report` (CORRECTION 13:43) | Test `every_editable_leaf_perturbation_changes_report` in `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — walks leaves of `compliant_de_office` + `compliant_de_multipart`, typed ±10%/bool/enum swap via serde_json, asserts status/utilization/explanation change. Helpers: `assert_perturbation_for_snapshot`, `walk_leaves`, `set_*_at_path`. Inventory checks in `💡️inferences/🦀️.rs` ensure previously N/A-scoped leaves (masonry ratio, EN site params, member/storey detailing) always influence the report. |
| 2 | Localize every `explanation` en/de pair + test | `💡️inferences/🦀️.rs` — all identical `&format!(…), &format!(…)` pairs diverged (German engineering terms). Test `committed_examples_have_localized_explanations_and_remedies` asserts en≠de for explanation/remedy text (numbers-only excepted) on all four committed snapshots. |

### Coordinator tightenings

| # | Item | Mapping |
|---|------|---------|
| A | DSL examples with parts 2–6 (compliant + failing) | Fixtures `En1998Snapshot::compliant_de_multipart` / `noncompliant_de_multipart` in `📸️snapshot/🦀️.rs`; modules `📚️examples/🏢️seismic-multipart`, `⚠️seismic-multipart-fail`; assets under `🖼️assets/…/🗣️.dsl.semio`; tests `multipart_examples_evaluate_expected_verdicts`. Editor catalogue lists all four examples (`✏️editor/🦀️.rs`). |
| B | Parts 2–6 `massKg` → G_k + ψ_E·Q_k (+ fill) | Types `En1998Bridge`/`Silo`/`Tank`/`Tower` in `🦀️.rs`: `permanentGkN` + `variables[]` or `contentQkN`+`contentCategory`+`fillingRatio`. Helpers `entity_seismic_mass_kg`, `filled_content_seismic_mass_kg` in `🧬️schema/🦀️.rs`; used in `evaluate_bridges` / silos / tanks / towers (`💡️inferences/🦀️.rs`). Facets + field-meta regenerated. |
| C | Python oracle harden + C-S + all DE combos | `🔮️oracles/🐍️.py`: C-S `S=0.75` per DIN EN 1998-1/NA Table NA.4 (Rust BT/CT S also corrected to 1.25/1.50); `--combo-table`; soft `continue` removed in `python_oracle_matches_within_half_percent`. Test `de_ground_combo_table_matches_oracle_for_all_combos` covers A-R…C-S ±0.5 %. |

### Key tests (round 3)

- `every_editable_leaf_perturbation_changes_report`
- `committed_examples_have_localized_explanations_and_remedies`
- `multipart_examples_evaluate_expected_verdicts`
- `de_ground_combo_table_matches_oracle_for_all_combos`
- prior: `python_oracle_matches_within_half_percent`, `field_meta_covers_every_editable_leaf_of_default_snapshot`, remedy/example decode tests


## Round 3b — anti-gaming (14:37 / 14:42)

Removed string-length / inventory / `let _ =` gaming. Ground & spectrum leaves drive S, T_B, T_C, T_D via `AnnexParams` (DE: `deGroundCombo`; EN: `enGroundType` + `enSpectrumType`). Storey masses m_i from G_k + ψ_E·Q_k only (`correlated_occupancy` → φ in ψ_E,i). Perturbation signature is `(id, status, computed, limit, utilization)` — no explanation. Plan-irregular / torsion examples committed.

### Clause → leaf → check

| Leaf / input | Clause | Check id | File:line |
|---|---|---|---|
| `storeys[].centreOfMass*`, `centreOfStiffness*`, `stiffnessX/Y`, plan L | EN 1998-1 §4.2.3.2 | `en1998.1.{bldg}.storey.{id}.planRegularity` | `💡️inferences/🦀️.rs:267` (`plan_regularity_metrics` in `🧬️schema/🦀️.rs:232`) |
| `storeys[].stiffness*`, masses via G_k/ψ_E/φ | EN 1998-1 §4.2.3.3 | `en1998.1.{bldg}.elevationRegularity.{lo}-{hi}` | `💡️inferences/🦀️.rs:305` |
| `accidentalEccentricityRatio`, CM/CS, plan L | EN 1998-1 §4.3.3.2.4 / §4.3.3.3 | `en1998.1.{bldg}.{sys}.torsion` (+ δ·F_i) | `💡️inferences/🦀️.rs:548–596` |
| `storeys[].correlatedOccupancy` (roof φ=1) | EN 1998-1 §4.2.4 Table 4.2 | `en1998.1.{bldg}.storey.{id}.roofPhi` | `💡️inferences/🦀️.rs:601` |
| `systems[].direction` | EN 1998-1 §4.3.3.2.1 | `en1998.1.{bldg}.{sys}.direction` | `💡️inferences/🦀️.rs:443` |
| `site.enGroundType`, `site.enSpectrumType`, `site.aGr` (EN annex) | EN 1998-1 §3.2.2 | spectrum via `resolve_annex` → `AnnexParams::En` | `💡️inferences/🦀️.rs:77–91` |
| `site.deGroundCombo`, `site.seismicZone` (DE annex) | DIN EN 1998-1/NA Table NA.4 | `AnnexParams::De` | `💡️inferences/🦀️.rs:77–86` |
| `supportedBuildingId` | EN 1998-5 / EN 1998-3 | `en1998.5/3.{id}.supportedBuilding` | foundations/assessments loops |

### Removed gaming

| Former site | Action |
|---|---|
| `enSpectrumParams` `a_gr + len()*0.01…` | Deleted |
| `storey.*.inventory` (`we * 1e-9`) | Deleted; replaced by §4.2.3.2 / §4.2.3.3 / §4.3.3.2.4 |
| `member.*.inventory` | Deleted; RC/steel detailing + ρ′ / ω_wd |
| `let _ = ok_rho` | Status uses `ok_rho` |

### Examples / tests

- `compliant_en_office` / `noncompliant_en_office`
- `compliant_de_torsion_regular` / `noncompliant_de_torsion_irregular`
- `every_editable_leaf_perturbation_changes_report` (office, multipart, EN, torsion-regular)
- `torsion_irregular_examples_evaluate_expected_verdicts`
- `en_annex_examples_evaluate_expected_verdicts`

### Runner summary

`Summary [   4.288s] 70 tests run: 70 passed, 0 skipped`

Taxonomy: `bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → 547 payloads.

## Round 4 — catalogue + referential integrity

See `📓️fix-en1998-r4.md`.

**Runner:** `bun nx run @semio-tech/norm-en1998-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   0.991s] 74 tests run: 74 passed, 0 skipped`  
**Contract:** `51 tests run: 51 passed, 0 skipped`

| # | Item | Mapping |
|---|------|---------|
| 6 | `reference_tables()` NA.1 + NA.4 from `na_de` | `✏️editor/📌️panels/📚️catalogue/🦀️.rs`; tests `reference_tables_cells_match_na4_spectrum_params`, `catalogue_na4_cell_matches_evaluated_spectrum_params` |
| 7 | Dangling `supportedBuildingId` `Remedy::one_of` + duplicate ids | `💡️inferences/🦀️.rs` `push_referential_integrity`; tests `dangling_supported_building_fails_with_one_of_remedy`, `duplicate_building_id_fails_integrity` |

Anti-gaming R4 items 1–5 / 8–9 not regresssed.
