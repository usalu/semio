# Differential compare notes (Wave 6 gate)

Ticket: `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`  
Lane: `1-oracle` (harness documentation only — no new Rust crate, no `brepkit-*` production deps).

## Purpose

Before Wave 6 deletes `brepkit-*` git dependencies, native kernel outputs must match reference behavior within documented tolerances. The oracle lane supplies **independent** ground truth (SDFs, closed-form mass, watertightness stubs) so tests never depend on the same code path they validate.

## Reference checkout (read-only)

Agents may read algorithms at:

`~/.cargo/git/checkouts/brepkit-760d3602f95e00d3/d470b7c`

Final tree must not link brepkit in production; comparison runs belong in ticket-local scripts or `#[cfg(test)]` harnesses under this ticket folder until the flip.

## Comparison dimensions

| Dimension | Native source (post-flip) | Oracle / reference | Tolerance |
|---|---|---|---|
| SDF classification | Kernel mesh samples / pick | `brep::oracle::Sdf::eval` / `contains` | linear `1e-6` on distance; containment via `Resolution::default().linear` |
| Solid volume / area | `brep::measure` (divergence theorem) | `ClosedFormMass::{box,sphere,cylinder}_*` on matching primitives | relative `1e-9` volume, `1e-8` area for analytic solids |
| Watertightness | `validate` + topo edge walk (future) | `watertightness_from_boundary_edge_count` fed by independent edge tally | exact on edge count; verdict enum must agree |
| Boolean / imprint | Native boolean pipeline | SDF boolean ops on analytic operands | sample grid + Monte Carlo inside/outside agreement |

## Suggested Wave 6 procedure (integrator)

1. Build the same primitive (box, sphere, cylinder, cone, torus) via native `primitives` and via legacy brepkit wrapper (pre-delete baseline capture).
2. Tessellate or sample `N` stratified points in a bbox; compare `measure` volume/area to `ClosedFormMass`.
3. For booleans, build equivalent `Sdf` expression and compare `contains` at random points against native point-in-solid.
4. Record pass/fail and max error in ticket `🧪differential/` logs; block flip until all rows in the table pass.

## Out of scope for Wave 1

- Automated brepkit runner in CI (integrator adds via `📥️integration-requests.md` if needed).
- Cone/torus closed-form mass in oracle (add when `measure` lane exposes matching solids).
