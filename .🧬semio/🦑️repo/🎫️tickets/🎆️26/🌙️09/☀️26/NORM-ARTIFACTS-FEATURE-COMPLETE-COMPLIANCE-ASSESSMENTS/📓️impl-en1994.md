# Impl — EN 1994 (`🧩️en1994`) — Round 2

## Subject (SI, hierarchical)
- `En1994Snapshot`: annex, structureKind, steelFYPa, beams[], columns[], slabs[], fireRating, insulationThicknessM, fatigueDetail
- **Beams**: span, spacing, support (`simply_supported` | `continuous_2_span`), construction (propped/unpropped), steel section, slab, sheeting, studs (incl. `spacingM`), crack fields, `actions[]`
- **Columns**: kind (encased / concrete_filled / partially_encased), outerSizeM, wallThicknessM, sections, `actions[]`
- **Slabs**: support, sheeting (thickness → A_p), m-k, `actions[]`
- **CharacteristicAction**: kind/category/stage + qAreaPa / qLineNPerM / mKNm / vKN / nKN / fatigue Δσ_k / Δτ_k
- Hand-typed M_Ed / V_Ed / N_Ed **removed**

## Combinations (`part_en1990`)
- ULS 6.10 (composite + construction stages), SLS characteristic / quasi-permanent, fire 6.11 with ψ_fi
- Span + support analysis (simply supported / continuous 2-span) or external m_k/v_k/n_k
- Governing combination label reported per check; propped vs unpropped (construction steel-alone + LTB)
- Bridges: FLM3 Δσ / Δτ (or fatigue action overrides)

## Checks (clause · remedy)
| Part | Clause | Check id | Remedy |
|------|--------|----------|--------|
| 1-1 | §5.4.1.2 | beff | spacing |
| 1-1 | §5.5 / EN 1993-1-1 Table 5.2 | class | tw/tf |
| 1-1 | §6.2.1.3 | mrd | actions / slab / steel OneOf |
| 1-1 | §6.2.1 / §9.3 | construction | construction q |
| 1-1 | §6.6.5.5 | spacing | studs.spacingM |
| 1-1 | §6.6.3.1 | prd | stud count / spacing |
| 1-1 | §6.6.1.2 | etamin | stud count |
| 1-1 | §6.2.2 | vpl | tw (+ shear buckling) |
| 1-1 | §6.6.6 | vlrd | transverse As |
| 1-1 | §6.4 | ltb | ltbLength (unpropped/hogging) |
| 1-1 | §7.3.1 | deflection | SLS QP actions |
| 1-1 | §7.4 | crack | As / bar spacing |
| 1-1 | §6.7.3 | npl / mn | kind, wallThickness, actions N_k |
| 1-1 | §9.3 / §9.7 | sheeting / mrd / mk / vrd | thickness, As, concrete |
| 1-2 | §4.2 / §4.3 | insulation / theta | insulation |
| 2 | §6.8 | delta-sigma / stud | FLM3 / diameter |

## DE/EN
- γ_Mf bridge fatigue: DE 1.35 vs EN 1.15; other NDPs aligned in AnnexParams

## Examples
- `🏢composite-floor-beam` complies; `🏢composite-floor-beam-failing` fail≥2; `🌉️composite-bridge-girder` runs EN 1994-2 fatigue

## Tests
- Leaf perturbation: every editable leaf (excl. name/title, catalogue plate geometry under resolve, bridge-only fatigue, unused zero companions, propped-only LTB length) changes ≥1 check
- Oracle ±0.5 % + jsonschema; mutation kinds renamed to action/geometry verbs

## Runner
```
Summary [   2.918s] 72 tests run: 72 passed, 0 skipped
```
`bun nx run @semio-tech/norm-en1994-rs:test --skip-nx-cache -- --no-fail-fast`

## Round-2 CORRECTION 13:43 closeout
- Characteristic actions + EN 1990(+DE NA) combos; governing label; propped/unpropped; FLM3
- `studs.spacingM` → §6.6.5.5 + η; `columns[].kind` → §6.7.3; `twM`/`tfM` → class/A_v/buckling/M_pl; `sheeting.thicknessM` → A_p / construction / m-k·τ_u / §9.7
- Leaf mutation coverage test green; parallel-rib k_t = 1.0×t_fac; default rib width 35 mm so non-parallel k_t < 1 for leaf sensitivity

## Remaining gaps
None

## Requests to coordinator
None
