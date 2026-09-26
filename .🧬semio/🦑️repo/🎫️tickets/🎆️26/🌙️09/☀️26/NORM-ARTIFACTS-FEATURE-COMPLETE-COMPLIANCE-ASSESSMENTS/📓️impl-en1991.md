# Impl — EN 1991 (Wave D)

## Status

`bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast` → `Summary [   1.086s] 79 tests run: 79 passed, 0 skipped`.

Remaining gaps: **none** (round-3 closed).


## Subject schema (SI base)

Flat `En1991Snapshot` with SI quantities (Pa, N, m, K, kg, J, W, s). National annex `annex: De | En`. Parts covered:

| Part | Scope | Claim gate |
|------|-------|------------|
| 1-1 | Imposed floors (qk·α_A·α_n, Qk, partitions), self-weight gk (Annex A densities) | floors / selfWeightElements non-empty |
| 1-2 | Fire thermal actions: nominal/parametric gas temp, h_net, q_f,d (Annex E / DE NA) | `fireClaimed` |
| 1-3 | Snow sk on roofs | roofs non-empty |
| 1-4 | Wind pressure + c_pe table conformance; coast→DE q_p | windFaces non-empty |
| 1-5 | Thermal ΔT from T_max/T_min/T_0 / bridge types 1–3 + ΔT_M | always |
| 1-6 | Construction actions | always |
| 1-7 | Accidental impact / explosion | accidentalCases non-empty (else N/A) |
| 2 | Bridge LM1 (tandem moment via span), UDL, LM2, LM3, LM4, footway, gr1a–gr5 | `bridgeClaimed` |
| 3 | Crane vertical + horizontal | `craneClaimed` |
| 4 | Silo Janssen / patch / wall friction / tank | `siloClaimed` |

Entity list SubjectRef/Remedy paths use `[id=…]`. Assumed ≥ required; Fail remedies write required SI into `target.path`.

## Wave D fixes

1. **EN 1991-1-2** `part_1_2` + fire snapshot fields/checks/remedies/meta/mutations; FireCurve waiver removed.
2. **Categories I/J/K** + localized NormFieldChoice labels.
3. **α_A / α_n** via `FloorArea.area` + `storeyCount`.
4. **LM3 / LM4** + LM2 + load groups gr1a–gr5; LM1 tandem uses `bridgeSpan` moment.
5. **`[id=…]` paths** + resolve test.
6. **Thermal** computed from 1-5 inputs; free `required_delta_t` removed.
7. **Human en+de enum labels** + material densities for self-weight.
8. **KINDS** synced to enum (80); oracle catalog regenerated; `kinds_catalog_tests` wired.
9. **Dual remedy-law** test (imposed + snow).
10. **Wind c_pe** vs Tables 7.1–7.4; `coast_or_island` in DE q_p.

Extras: tight DE UDL in compliant example; LM1 span moment; asset drift assertion (regen gated by `EN1991_REGEN_ASSETS=1`); example DSL decode+evaluate; taxonomy EN1991 rows merged (full generate may still be blocked by en1993).

## Requests to coordinator

- Shared `mutation-leaf-taxonomy-generate` may still fail on concurrent en1993 descriptor identity — EN1991 rows merged directly.

## Round-2 (Wave D verify closeout)

`bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast` → `Summary [   0.754s] 74 tests run: 74 passed, 0 skipped`.

`bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate` → `norm mutation-leaf taxonomy generated: 547 payloads`.

### Blocking items 1–6

| # | Fix | file:line | Tests |
|---|-----|-----------|-------|
| 1 | Scope-aware perturbation (status/computed/limit/utilization; CORRECTION 14:37) | `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:405` | `scope_aware_perturbation_of_editable_leaves` |
| 2 | Table 4.1 notional lanes; governing Table 4.4a load-group; Annex A/E fire from A_f/H; e=min(b,2h) ze | `💡️inferences/🦀️.rs:136–160,179–248,281–396`; `🧬️schema/🦀️.rs:773–776,778–798,901–917,1140` | bridge/fire example + compliance |
| 3 | Discriminated accidental impact\|explosion; FireMode; StructureKind + examples | `🏋️en1991/🦀️.rs` (`FireMode`/`StructureKind`/`AccidentalCase`); `📚️examples/🧬️subjects/🦀️.rs` | field-meta + perturbation suites |
| 4 | Field-meta en+de over all examples; lane labels; terrain 0 | `✏️editor/🏷️field-meta/🦀️.rs`; coverage `compliance-report/🦀️.rs:336` | `field_meta_coverage_all_committed_examples` |
| 5 | c_pe,1 table conformance + Fig.7.2 log interpolation | `💡️inferences/🦀️.rs:184–241`; `🧬️schema/🦀️.rs:801–854` | wind cpe checks |
| 6 | jsonschema required (no ImportError skip); oracle manifest count | `compliance-report/🦀️.rs:318`, `:568` | `snapshot_json_validates_against_schema`, `oracle_manifest_kind_count_matches_mutation_enum` |

### Extra (decisions / non-blocking)

- DE NA B.3 q_p scaled by ρ/1.25 (`🧬️schema/🦀️.rs:774`) so `airDensity` is normative.
- Load-group remedy targets the failing constituent path (tandem/UDL/LM2/LM3/LM4/footway) (`💡️inferences/🦀️.rs:367–377`).
- Default geometry width/height/z set so e=min(b,2h) changes z_e across NA B.3 breakpoints.

### CORRECTION 14:37

No epsilon/fingerprint gaming. Perturbation asserts check id/status/computed/limit/utilization only. Annex-scoped and claim-gated leaves listed per suite N/A (not dummy bindings).

## Round-3 (Wave C fixer)

`bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast` → `Summary [   1.086s] 79 tests run: 79 passed, 0 skipped`.

### Blocking items 1–4

| # | Fix | files | Tests |
|---|-----|-------|-------|
| 1 | Scope-aware perturbation walks every nested editable leaf (all array indices/depths); numeric+bool+enum; signature `(id,status,computed,limit,utilization)`; exemptions only id/name/title/labelEn/labelDe | `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` | `scope_aware_perturbation_of_editable_leaves` |
| 2 | `reference_tables()` from shared `part_1_1`/`part_1_3`/`part_1_4` consts (Table 6.1 DE/EN q_k, snow zones, NA B.3 q_p); catalogue rejects empty | `✏️editor/📌️panels/📚️catalogue/🦀️.rs` + unit | `reference_tables_cells_match_imposed_and_evaluate_limit` (B1 DE 2000 Pa = evaluate limit) |
| 3 | Duplicate entity ids Fail (floors/selfWeight/roofs/windFaces/accidentalCases) + dangling-ref helper; en+de + `one_of` remedy | `🧬️schema/💡️inferences/🦀️.rs` | `duplicate_floor_id_fails_integrity`, wind/roof/accidental analogues |
| 4 | Diff + root artifact facets: `AccidentalCase={id,impact[],explosion[]}`; drop flat kind/combined fields + `requiredDeltaT`; `structureKind` string enum; drop orphan `kind` field-meta | `🧬️schema/🔳️json`/`🟦️.ts`, `🔺️diff/*`, `🏷️field-meta/🦀️.rs` | schema/jsonschema suites |

### Normative leaf wiring (perturbation)

- Floor `area` → total force check `q_k·A` (`en1991.1-1.floor-force.*`).
- `hasParapet` / multi roof types → `shape_mu` parapet notional min height + multi-bay μ=1.6.
