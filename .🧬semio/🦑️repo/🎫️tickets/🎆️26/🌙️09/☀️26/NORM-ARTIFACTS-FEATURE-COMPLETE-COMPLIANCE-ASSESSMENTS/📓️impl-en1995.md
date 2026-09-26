# 🪵️ EN 1995 — Feature-Complete Compliance Assessment

Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/` only (below, `ANY` = `🏅️standards/🔖️1/🪆️subsets/✳️any/`).
Compliance core (`⚖️compliance`) and app-surface were not edited.

Wave C (fresh fixer): Round-3 **FAIL (4)** blockers closed — SLS frequent ψ₁, mutation text/binary facets (66 kinds), combined en/de localization.

## Subject sketch

```text
En1995Snapshot
├── annex: AnnexChoice (En | De)
├── members: Vec<TimberMember>                      # stable string ids, SI base units
│     id, labelEn/De, role (beam|column|floor|bridge), strengthClass, serviceClass, support
│     bM, hM, spanM, supportLengthM, bearingLengthM, bucklingLengthY/ZM, lateralRestraintSpacingM
│     notchDepthM, notchDistanceM, mCritNm, massKgPerM, massKgPerM2, dampingXi, fireDurationS
│     bridgeNObs, bridgeTLYears, bridgeBeta, bridgeA, bridgeB, bridgeCrowdPerM2
│     actions: Vec<CharacteristicAction>             # kind, category, loadDuration, qLine/fPoint or analysed mK/vK/nK/nTK/fC90K
└── connections: Vec<TimberConnection>
      id, labelEn/De, fastenerType, strengthClass, serviceClass
      diameterM, number, rows, spacingM, edgeDistanceM, endDistanceM, t1M, t2M
      steelPlate, steelPlateThicknessM, shearPlanes, fUK
      actions: Vec<ConnectionAction>                 # kind, loadDuration, fKN
```

Characteristic actions are combined inside `evaluate()` per EN 1990 eq. 6.10 (+ ψ₀/ψ₁/ψ₂ from Table A1.1 / DE NA `snow_high`). Design effects are never the only action input. `ComboKind` = ULS, SLS characteristic, **SLS frequent (ψ₁ lead + ψ₂ accompanying)**, SLS quasi-permanent, accidental. `lateralRestraintSpacingM` scales `M_crit` (∝ 1/ℓ_ef²). Connection `rows` enters `n_ef`. Steel-plate / shear-plane Johansen uses §8.2.2–8.2.3. Bridge ULS uses crowd LM from `bridgeCrowdPerM2`, not the building beam’s design moment. Floor §7.3 derives f₁ and a_vert from mass/stiffness/span; NotApplicable when role ≠ Floor.

## Check catalogue

| Part | Clause | Check id | Remedy levers |
|---|---|---|---|
| EN 1995-1-1 | §6.1.6 | `en1995.6.1.6.bending.<member>` | ↑ `hM`, ↑ `mCritNm`, ↓ `lateralRestraintSpacingM` |
| EN 1995-1-1 | §6.1.7 | `en1995.6.1.7.shear.<member>` | ↑ `hM` / ↑ `bM` |
| EN 1995-1-1 | §6.1.2 | `en1995.6.1.2.tension.<member>` | ↑ `bM` |
| EN 1995-1-1 | §6.3.2 | `en1995.6.3.2.compression.<member>` | ↑ `hM` / ↑ `bM` (λ from Y and Z buckling lengths) |
| EN 1995-1-1 | §6.2.4 | `en1995.6.2.4.combined.<member>` | ↑ `hM` / ↑ `bM` — en/de prose differs („Kombination Druck und Biegung“) |
| EN 1995-1-1 | §6.1.5 | `en1995.6.1.5.c90.<member>` | ↑ `bearingLengthM` (`supportLengthM` in k_c,90) |
| EN 1995-1-1 | §7.2 | `en1995.7.2.winst/wfin/wfreq.<member>` | ↑ `hM` — wfreq uses SLS frequent (ψ₁) combo |
| EN 1995-1-1 | §7.3 | `en1995.7.3.f1/stiffness/velocity|acceleration.<member>` | Floor role; ↑ `hM` / mass / damping |
| EN 1995-1-2 | §4.2 | `en1995.1-2.4.fire.<member>` | ↑ `hM` / ↑ `bM` |
| EN 1995-2 | Annex A | `en1995.2.a.fatigue.<member>` | Wohler N_R / k_fat damage; ↑ `hM` |
| EN 1995-2 | Annex B | `en1995.2.b.avert/ahor.<member>` | ↑ `hM` / crowd |
| EN 1995-2 | §5 / §7 | `en1995.2.uls.bending` / `….sls.deflection` | Crowd LM; ↑ `hM` |
| EN 1995-1-1 | §8.2.2/8.2.3 | `en1995.8.*.johansen.<conn>` | steel plate / planes / rows; ↑ `diameterM` |
| EN 1995-1-1 | §8.3–8.6 | `en1995.8.spacing.*` | ↑ spacing / end / edge |
| integrity | id | `en1995.integrity.duplicate.*` / material unknown | `Remedy::one_of` free ids / tabulated classes |

## Mutations (66 hierarchical kinds)

Rust `En1995Mutation` / `KINDS` = 66 id-addressed kinds (memberId / connectionId / actionId; insert/remove keep list index).

- `🧬️mutations/🟦️.ts` and `🧬️mutations/📝️text/🟦️.ts` — typed union; no `Record<string, unknown>` / `unknown` nests.
- `🧬️mutations/💾️binary/📡️.protocol.semio` — wire tags 0..65 for all 66 kinds; legacy flat scalars (`change-m-ed-knm`, `change-w-mm3`, …) removed.
- Python oracle `🔮️oracles/🐍️evaluate.py` mirrors SLS frequent + `en1995.7.2.wfreq.*`.

## DE / EN differences

| Parameter | EN | DE |
|---|---|---|
| γ_M timber | solid 1.3 / glulam 1.25 / LVL 1.2 | 1.3 all (NA Table NA.2) |
| k_cr | 0.67 | min(1, 2.5/f_v,k[MPa]) |
| w_fin | L/250 | L/200 |
| floor w(1 kN) | 1.7 mm | 1.5 mm |
| floor a_vert (f₁<8) | 0.10 m/s² | 0.05 m/s² |
| snow ψ₁ | 0.2 (`snow`) | 0.5 (`snow_high`, sites >1000 m) |

## Examples

1. `🏠️glulam-floor-beam` — compliant floor + connection (`complies()`).
2. `🌉️glulam-footbridge` — compliant bridge (`complies()`).
3. `❌️multi-fail-timber` — `fail_count ≥ 2` with named ids.
4. `⚠️overloaded-footbridge` — ≥2 EN 1995-2 fails.

## Catalogue / anti-gaming

- `reference_tables()` publishes strength classes, k_mod (Table 3.1), fasteners, roles from the same consts `evaluate()` reads.
- Test `reference_table_k_mod_cell_equals_evaluated_modification_factor` asserts table cell == governing k_mod.
- Perturbation signature `(id, status, computed, limit, utilization)` only; exemptions `id`/`labelEn`/`labelDe` (+ role-gated bridge leaves / steel thickness when plate off).
- Duplicate member/connection/action ids Fail with `one_of`. Unknown strength class Fail with `one_of` (member + connection).
- No fingerprints / epsilon gaming; no evaluate-path `let _ =` dummy binds on editable leaves.
- `bucklingLengthZM` retained and used in compression buckling λ_z.

## Wave C FAIL (4) closeout

1. **SLS frequent** — `ComboKind::SlsFrequent` + `enumerate_combos()` emits `sls.freq.*` with ψ₁ lead / ψ₂ accompanying; `evaluate()` emits real §7.2 `wfreq` deflection check. Char + QP enumeration and ULS from characteristic actions unchanged.
2. **Text mutations** — regenerated 66-kind typed facet from Rust (no `Record<string, unknown>`).
3. **Binary protocol** — regenerated tags for all 66 hierarchical kinds.
4. **Combined interaction de** — German explanation uses „Kombination Druck und Biegung“, not a copied English formula string.

New test: `sls_frequent_uses_psi1_and_de_snow_high_diverges`.

## Tests

Command: `bun nx run @semio-tech/norm-en1995-rs:test --skip-nx-cache -- --no-fail-fast`

```text
Summary [   2.831s] 177 tests run: 177 passed, 0 skipped
```

Command: `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache`

```text
Summary [   0.154s] 51 tests run: 51 passed, 0 skipped
```

Added / retained gates: ψ₁ frequent combo + DE `snow_high` divergence, two fail→pass remedies, path-resolve, field-meta leaf walk, python oracle ±0.5 %, jsonschema, example DSL→`complies()`/`fail_count≥2`, perturbation walk, duplicate/dangling integrity, lateral-restraint + rows influence, steel-plate capacity.

## Remaining gaps

None for the Wave D Round-3 **FAIL (4)** blocking list.

Non-blocking residuals:

- Integer fastener-count remedies stay `applicable = false` until app-surface accepts u32 writes.
- Framework `UNITS` table still lacks N·m / N/m / kg/m / kg/m² (DSL uses plain `NUM`).
- Plugin-wide `NormMutationLeafTaxonomy` `rows.maxItems: 392` while generate emits 547 payloads (not EN 1995-only).

## Requests to coordinator

- Raise `NormMutationLeafTaxonomy` JSON Schema `rows.maxItems` (or split the fixture) so `mutation-leaf-taxonomy-check` can pass.
