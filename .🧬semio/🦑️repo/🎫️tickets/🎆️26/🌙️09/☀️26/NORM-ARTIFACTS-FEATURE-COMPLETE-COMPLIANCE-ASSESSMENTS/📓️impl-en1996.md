# Impl — EN 1996 (`🧱en1996`) Wave D closeout

**Family:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱en1996/`  
**Runner:** `bun nx run @semio-tech/norm-en1996-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `Summary [   0.710s] 138 tests run: 138 passed, 0 skipped`

## Wave D blockers addressed

1. **Characteristic actions + EN 1990/DE NA combinations** — `WallLoadCase` carries G_k slab, q_k imposed (+ category), snow, wind q_p·c_pe, earth H_k, tributary/span; `design_effects()` forms ULS max-N / min-N·max-H combinations with γ_G/γ_Q/ψ₀; governing combo reported in explanations. Wall self-weight (density×geometry) enters N at mid/bottom.
2. **Sliding §6.2** — V_Rd = min(masonry shear, μ·N_Ed/γ_M) with `walls[].mu`.
3. **Fire EN 1996-1-2 DE NA** — α = N_Ed,fi/N_Rd; tabulated t_min by REI / unit group / mortar / α.
4. **EN 1996-3/NA** — Table NA.A.1 applicability (storeys, q_k, span, thickness/height, load-bearing); Φ_s·f_d·A; basement earth-pressure §4.5; `NotApplicable` + reason out of scope.
5. **f_vk DE NA caps** — tensile/unit limits; no dead `_cap` / 1 MPa hardcode.
6. **Material conformance** — declared mortar class vs `mortarStrengthPa`, unit group/dims, bed joint, f_b band (replaces tautological f_k≥0.5 MPa).
7. **All editable leaves live** — slab bearing → eccentricity; unit dims → δ shape factor + group rules; openings → area/ρ_n; `asHorizontalM2`/`asVerticalM2`/`fYd`/`reinforced` → §6.6 / flexure; labels/ids excepted. Perturb-every-leaf test green.
8. **e_mk creep** — e_k = 0.002·φ_∞·(h_ef/t_ef)·√(t·e_m) with `phiInfinity`.
9. **UX** — `designSituation` NormFieldChoice en+de; `mortarStrengthPa` SI Pa (renamed from fMPa).

## Remaining gaps

None for the Wave D blocking list.
