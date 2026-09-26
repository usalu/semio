# Impl — EN 1991 (Wave D)

## Status

`bun nx run @semio-tech/norm-en1991-rs:test --skip-nx-cache -- --no-fail-fast` → `Summary [   1.095s] 70 tests run: 70 passed, 0 skipped`.

Remaining gaps: **none**.


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
