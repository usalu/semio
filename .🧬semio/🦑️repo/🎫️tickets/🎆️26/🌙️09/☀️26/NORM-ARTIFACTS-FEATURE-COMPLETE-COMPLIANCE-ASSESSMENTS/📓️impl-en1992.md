# Impl — EN 1992 (🏛️) Wave D close-out

**Family:**   
**Runner:** 
 NX   Running target test for project @semio-tech/norm-en1992-rs and 4 tasks it depends on:


✔  nx run @semio-tech/ui-rs:generate
✔  nx run @semio-tech/framework-schema:generate
✔  nx run @semio-tech/ui-styling-tokens:generate
✔  nx run @semio-tech/framework-graph:generate

> nx run @semio-tech/norm-en1992-rs:test --no-fail-fast

> bun ./📜️script.ts test --no-fail-fast




 NX   Successfully ran target test for project @semio-tech/norm-en1992-rs and 4 tasks it depends on


Output of 4 successful tasks were not shown. Run with --verbose or --output-style=static to see it.

  Run duration:      27.1s
  Cache:             Skipped (--skip-nx-cache)
  Critical path:     23.3s (2 tasks)
  Recoverable time:  <1ms  
**Summary:** 

## Wave D blockers closed

1. **Characteristic actions + EN 1990 combinations** —  carries G/Q/S/W/A with ψ category, line/point/external sources;  forms ULS 6.10a/b, accidental 6.11, SLS char/freq/qp; governing combination reported in check explanations.
2. **Durability** —  +  adjust structural class /  (Table 4.3N/DE).
3. ** / ** — bond diameter (+aggregate >32 mm), , .
4. **Materials catalogue** —  Table 3.1 C12–C100; ; accidental  γ_c=1.3 / γ_s=1.0.
5. **SLS §7.2** — cracked-section σ_s / σ_c characteristic + 0.45 f_ck quasi-permanent creep.
6. **Anchorage §8.4 / laps §8.7** — f_bd (η1/η2), l_b,rqd, l_bd, l_0 with remedies on embedment / diameter / lap.
7. **DE deflection** — Table 7.4N ∩ K·35 ∩ K²·150/l when .
8. **λ_lim DE NA** — 25 if n≥0.41 else 16/√n + §5.8.8 second-order moment when exceeded.
9. **/** — FEM path uses external M_k/V_k; otherwise udl/line loads derive effects; both read in evaluate.
10. **PrestressSpec** — force, area, eccentricity, lossRatio aligned across Rust/JSON/GraphQL/proto/field-meta; P_m,∞ + transfer stress limits.
11. **Facets** — hierarchical snapshot + mutation GraphQL/TS/proto; outline regenerated; flat  stubs removed.
12. **Localized explanations** — distinct en/de strings on all major checks (no copy(x,x)).
13. **Punching DE NA** — β from column position, u₀/u₁ from geometry, C_Rd,c=0.18/γ_c with u₀/d reduction, asw contribution.
14. **Fire Tables 5.2a–5.11** — columns A/B, walls, beams SS/cont, slabs one-/two-way/flat/ribbed via .

## Extra

- Python oracle covers full ULS/SLS overlapping set (parity ≥10 / all py checks, ±0.5%).
- Catalogue panel exposes C12/15–C100/115 + B500A/B markdown tables.

## Remaining gaps

_none_
