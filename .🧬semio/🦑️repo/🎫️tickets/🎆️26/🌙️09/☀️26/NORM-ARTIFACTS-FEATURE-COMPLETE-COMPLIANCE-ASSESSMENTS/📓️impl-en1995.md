# EN 1995 Wave C / Wave D Implementation

## Subject schema

```text
En1995Snapshot
├── annex: AnnexChoice (EN | DE)
├── members: Vec<TimberMember>        # stable string ids
│     id, labelEn/De, strengthClass (C14–C50, GL20h–GL32c, LVL32, CLT100)
│     serviceClass (1–3), loadDuration
│     bM, hM, spanM, supportLengthM, bearingLengthM
│     bucklingLengthYM/ZM, lateralRestraintSpacingM
│     notchDepthM, notchDistanceM
│     mEdNm, vEdN, nEdN, nTEdN, fC90EdN, mCritNm
│     wInstM, psi2, floorAVert, fireDurationS, bridgeNCycles
└── connections: Vec<TimberConnection>
      id, labelEn/De, fastenerType (nail|screw|bolt|dowel)
      strengthClass, serviceClass, loadDuration
      diameterM, number, rows, spacingM, edgeDistanceM, endDistanceM
      t1M, t2M, steelPlate, steelPlateThicknessM, shearPlanes
      fEdN, fUK
```

All quantities are SI base units (m, N, Pa, s). Strength properties resolve from EN 338 / EN 14080 / LVL / CLT tables in `🧬️schema/⚖️timber/🦀️.rs`.

## Check catalogue

| Part | Clause | Check id pattern | Remedy strategy |
|------|--------|------------------|-----------------|
| EN 1995-1-1 | §6.1.6 | `en1995.6.1.6.bending.<id>` | ↑ `hM` (analytic cube-root) |
| EN 1995-1-1 | §6.1.7 | `en1995.6.1.7.shear.<id>` | ↑ `hM`; k_cr EN 0.67 / DE min(1,2.5/f_v,k); k_v notches |
| EN 1995-1-1 | §6.1.2 | `en1995.6.1.2.tension.<id>` | ↑ `bM` = A_req/h (not √A) |
| EN 1995-1-1 | §6.3.2 | `en1995.6.3.2.compression.<id>` | ↑ `bM` = A_req/h; k_c from λ_rel (β_c=0.2) |
| EN 1995-1-1 | §6.2.4 | `en1995.6.2.4.combined.<id>` | ↑ `hM` via bounded search inverting (σ_c/(k_c·f_c,d))²+σ_m/f_m,d ≤ 1 |
| EN 1995-1-1 | §6.1.5 | `en1995.6.1.5.c90.<id>` | ↑ `bearingLengthM`; k_c,90 |
| EN 1995-1-1 | §7.2 | `en1995.7.2.winst/wfin.<id>` | ↑ `hM`; k_def×ψ2; DE L/200 vs EN L/250 fin |
| EN 1995-1-1 | §7.3 | `en1995.7.3.vibration.<id>` | ↑ `hM`; DE 0.05 / EN 0.10 m/s² (only when bridgeNCycles≤0) |
| EN 1995-1-1 | §8.2.2 | `en1995.8.2.2.johansen.<id>` | ↑ `number` / `diameterM`; EYM + rope; n_ef |
| EN 1995-1-1 | §8.3 | `en1995.8.spacing.<id>` | u=max(a1,min/a1, a3,t,min/a3,t, a4,t,min/a4,t); Table 8.2/8.4/8.5 by fastener |
| EN 1995-1-2 | §4.2 | `en1995.1-2.4.fire.<id>` | ↑ `hM`; d_ef from β_n(product); k_fi/k_mod,fi/γ_M,fi via AnnexParams |
| EN 1995-2 | Annex A | `en1995.2.a.fatigue.<id>` | gated on bridgeNCycles; k_fat(N); ↑ `hM` |
| EN 1995-2 | Annex B | `en1995.2.b.vibration.<id>` | pedestrian a_vert vs bridge comfort (DE 0.5 / EN 0.7) |
| EN 1995-2 | §5 | `en1995.2.uls.bending.<id>` | bridge ULS bending |
| EN 1995-2 | §7 | `en1995.2.sls.deflection.<id>` | bridge SLS L/400 |

SubjectRef / Remedy paths use `members[id=<id>].…` / `connections[id=<id>].…` (spec v1.2).

Applicability: empty members → NotApplicable; tension/compression/c90/SLS/vibration/fire/bridge gated on inputs.

## DE / EN differences

| Parameter | EN | DE-NA |
|-----------|----|-------|
| k_cr | 0.67 | min(1, 2.5/f_v,k[MPa]) |
| w_fin limit | L/250 | L/200 |
| floor a_vert | 0.10 m/s² | 0.05 m/s² |
| bridge a_vert | 0.70 m/s² | 0.50 m/s² |
| γ_M | solid 1.3 / glulam 1.25 / conn 1.3 | same |
| fire k_fi / k_mod,fi / γ_M,fi / β_n | AnnexParams (product tables) | same routing |

## Mutations

46 hierarchical kinds (id-keyed scalars + insert/remove for members/connections). Legacy flat-scalar triads removed. Each leaf has 🔣️.json + payload schema, diff/inverse, and text/unit coverage. Semantic kinds match `#[derive(dsl::Mutations)]` kebab variants (`change-member-fc90-ed`, `change-member-nt-ed`, `change-connection-fuk`). Plugin mutation-leaf taxonomy regenerated.

## Examples

1. **Compliant** — `🌉️glulam-footbridge` / `compliant_glulam_beam()` (default): GL28h 200×400, L=6 m, M_Ed=28 kNm, DE SC1 medium, bridgeNCycles=2e6. Spacing margins above Table 8.4 minima.
2. **Non-compliant** — `❌️multi-fail-timber` / `noncompliant_multi_fail()`: C24 beam + slender column + weak nail group + fire + bridge overload; multiple Fail with remedies.

## B2 field metadata

`🏷️field-meta/🦀️.rs` → `en1995_field_meta` with `NormFieldChoice { value, label_en, label_de }` (not `&[&str]`). Exact-path + `[]` wildcard lookup; en+de labels; SI display units on quantity leaves; enum choices for annex/strength/service/load-duration/fastener/steel-plate.

## Facets / regen

Snapshot/diff/mutations/text/binary/JSON/TS/GQL/proto leaves for hierarchical subject. Oracle catalog `🔮️oracles/🔣️.json` lists all 46 kinds. Independent Python evaluate at `🔮️oracles/🐍️evaluate.py` (also via `⚖️evaluate-en1995-1/🐍️.py`).

## Tests

| Suite | Status |
|-------|--------|
| Python oracle smoke | **PASS** |
| JSON Schema (third-party `jsonschema`) on compliant + noncompliant snapshots | **PASS** |
| Rust↔Python utilization parity ≤ 0.5 % on both examples | **PASS** |
| `bun nx run @semio-tech/norm-en1995-rs:test -- --no-fail-fast` | **Summary [   0.438s] 84 tests run: 84 passed, 0 skipped** |

Editor unit tests call `NormHost::evaluate()` before paint-path assertions (revision-keyed report cache).

## Remaining gaps

_(none)_

## Requests to coordinator

_(none)_
