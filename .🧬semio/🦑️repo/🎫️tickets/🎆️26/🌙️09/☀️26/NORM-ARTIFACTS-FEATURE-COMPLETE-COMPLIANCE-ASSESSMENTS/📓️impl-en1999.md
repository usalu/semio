# Impl — EN 1999 (`en1999`)

Wave D round-2 (CORRECTION 13:43): characteristic EN 1990 actions, HAZ ρ_u, cold-formed / shell / fatigue depth, remedy-law ≥2, every editable leaf influences a check.

Family: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📚️en1999`.

## Subject schema (SI: m, N, Pa, °C)

```
En1999Snapshot
├─ annex: AnnexChoice (en|de)
├─ materials[]: { id, designation }
├─ sections[]: { id, kind, …, elements[{ welded, weldPosition, … }] }
├─ members[]: {
│    id, sectionId, materialId, length, support,
│    bucklingLengthY/Z/T, ltbLength, c1, restrainedLtb,
│    actions[]: { id, kind, category, source, gKLine, qKLine, nK, vYK, vZK, mYK, mZK }  # characteristic
│  }
├─ connections[]: { …, nK, vK, bolts, welds{ fillerAlloy, throat, length, betaW, hazExtent } }
├─ fireScenarios[]: { id, memberId, thetaA, durationS }
├─ fatigueDetails[]: { id, memberId, detailCategory, deltaSigmaC, deltaSigmaEd, nCycles, m1, m2 }
├─ coldFormed[]: { id, materialId, thickness, width, span, mEd, nEd, welded }   # EN 1999-1-4
└─ shells[]: { id, materialId, radius, thickness, length, sigmaXEd, sigmaThetaEd }  # EN 1999-1-5
```

`part_en1990::{governing,fire}_member_effects` forms ULS 6.10a/b + fire ψ₂; member / fire / connection checks use them (no `actions.first()`). Field-meta labels say “characteristic”.

## Round-2 blockers closed

1. **EN 1990 action model** — characteristic load cases + governing combinations; examples / oracle / DSL / JSON schema updated.
2. **ρ_u,haz** — net-section `effective_area` uses ρ_u; wel/bending uses ρ_o; HAZ extent from `hazExtent`/throat/t; `haz_rho_u_governs_welded_net_section`.
3. **Cold-formed (1-4)** — axial, welded η/HAZ, span → support / web crippling, N–M interaction.
4. **Shells (1-5)** — χ from σ_x,Rcr / σ_θ,Rcr (geometry, E, f_o, C-class α/λ̄₀/β); `shell_chi_from_geometry_hand_value`.
5. **Fatigue (1-3)** — bi-linear S–N (m1/m2, N_C/N_D/N_L), Annex J categories, γ_Mf DE NA, damage D; memberId links.
6. **Remedy law** — ≥2 distinct applicable numeric remedies on distinct fails → Pass / u≤1.
7. **Editable leaves** — length→L_cr; V_y/M_z via governing; weldPosition/fillerAlloy/hazExtent; durationS→θ_eff; shell.length; memberId; `every_editable_leaf_influences_a_check`.

## Tests (authoritative)

| Gate | Result |
|------|--------|
| `bun nx run @semio-tech/norm-en1999-rs:test --skip-nx-cache -- --no-fail-fast` | **Summary [   0.414s] 65 tests run: 65 passed, 0 skipped** |

## Remaining gaps

None.
