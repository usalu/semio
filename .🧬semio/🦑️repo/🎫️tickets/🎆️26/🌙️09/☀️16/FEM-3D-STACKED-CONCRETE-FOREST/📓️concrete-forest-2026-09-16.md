# 🌲️ fem3d `concrete-forest` example — two stacked Hexagonal Cut Concrete Forest Left pieces (2026-09-16)

## What was added
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🖼️assets/🌲️concrete-forest/🗣️.dsl.semio` — the document: 20 nodes, 20 `frame` elements, no solids (nodes + line elements only), `hex30` + `beam30x45` sections, C30/37, `dead` (self weight + slab tributary UDL 10 298 N/m) and `live` (4 952 N/m) cases, `uls` 1.35/1.5 and `sls` 1/1 combinations, deformation scale 300.
- `📚️examples/🌲️concrete-forest/{🦀️.rs,🟦️.ts}` (`ID = "concrete-forest"`, label Concrete Forest / Betonwald, icon `list-tree`) + `🧪️tests/🧩️example/{🦀️.rs,🟦️.ts}`.
- Registration: crate root `examples::concrete_forest` (`🗿️artifacts/🧊️3d/🦀️.rs`), subset `examples()` list (`🌐️any/🦀️.rs`), `setActiveExample` handler matches the id (`✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`), palette select option (`✏️editor/🦀️.rs`); descriptor pair + registry regenerated (`✏️s/🔌️plugins/🏗️fem/🔣️.json`, `🛂️.descriptor.semio`).

## Geometry provenance (details: `📓️model.md`)
- Member topology = the CAD play fixture's `aec.building.structure.classic` model (the CAD structural representation): 2 hexagonal columns at (2.7, 2.338) and (8.1, 2.338), 7 cantilever beams to the hexagonal-cut edges, 1 spine between the column heads, one-way slab at z 3.
- Node coordinates = the puzzle3d `🌲️concrete-forest` vortices of `seed-left-001` (v0–v6 beam tips, v7/v9 `c-b` bases, v8/v10 `c-t` heads), verbatim (`x + 0.00001` offsets included). The upper piece is the lower one shifted by (0, 0, 3): its `c-b` vortices coincide with the lower `c-t` vortices, so both storeys share the two head nodes `lc1t`/`lc2t` — the FEM form of the puzzle's `c-b ↔ c-t` compatibility.
- Sections from the `aec.building` solids: hexagon R 0.30 (A 0.2338 m², I 0.004384 m⁴, J 0.00852 m⁴), beams 0.30 × 0.45 (A 0.135, Iy 0.002278, Iz 0.0010125, J 0.00238), slab 0.265 m carried as tributary UDL over the 22.95 m of beams.

## Deformation
Engine (`fem3d_solve_all`, Rust) vs independent numpy 3D-frame oracle (`🐍️frame-oracle.py`, own DSL parser, Przemieniecki 12×12, consistent self weight): 20/20 nodes agree, worst relative difference 3.1e-7. Vertical reactions balance the applied loads (dead 684.6 kN, live 227.3 kN). Every cantilever's sag beyond its head's rigid-body motion equals wL⁴/8EI (1.628 mm on the 2.7 m arms, 0.102 mm on the 1.35 m arm at SLS) to 1e-6 relative.

SLS (1.0 G + 1.0 Q) vertical displacements, metres:

| node | uz | node | uz |
|---|---|---|---|
| lc1t / lc2t (heads) | −1.62e-4 / −1.86e-4 | uc1t / uc2t | −2.42e-4 / −2.77e-4 |
| lv0 (light side, lifted) | +2.45e-4 | uv0 | +1.17e-3 |
| lv1 | −4.14e-3 | uv1 | −5.72e-3 |
| lv2 | −4.05e-3 | uv2 | −5.20e-3 |
| lv3 (light side, lifted) | +4.22e-4 | uv3 | +1.39e-3 |
| lv4 | −4.32e-3 | uv4 | −5.94e-3 |
| lv5 | −3.83e-3 | uv5 | −4.91e-3 |
| lv6 (short arm) | −2.42e-4 | uv6 | −1.21e-4 |

Physics: the hexagonal cut is asymmetric — column 1 carries two −y cantilevers and one +y, column 2 two +y and one −y plus the short v6 arm — so each head rotates about x and the heads lean apart in y (lower ±1.34 mm, upper ±4.94 mm at SLS); the light-side tips v0/v3 rise because the head rotation lifts them more than their own sag. Peak: −5.94 mm (uv4, SLS), −8.3 mm ULS.

## Proof
- `📜️test-concrete-forest.sh 1`: `examples::concrete_forest::tests` 3/3 — asset parses + print/parse identity, topology on the vortices (shared head joint, spine, seven fans keyed to CAD members), deformation laws (finite, residual < 1e-6, reactions balance, heads shorten and stack, heads lean apart and accumulate, per-cantilever wL⁴/8EI, ULS linear superposition).
- `📜️test-set-active-example.sh 1`: 20/20 (set_active_example + example tests with `component-app-assembly`). fem-js vitest 6 files / 8 tests green.
- React lane (`📜️activate-fem3d-react.sh 2`, `📜️serve-fem3d-react.sh` on 6087, `🐍️concrete-forest-probe.mjs`): navbar combobox `No example | Demo | Concrete Forest`; picking it logs `[DEBUG] fem3d setActiveExample id=concrete-forest nodes=20 elements=20 solids=0`, `[DEBUG] fem3d results solve #3`, live_visual reconcile `meshItems=197 drawInstances=194`; both World3d windows render 234 instances. The results window's node instances, read back from `data-instances-json`, sit at model position + 300 × dead-case displacement — worst |Δ| vs the oracle 2.5e-15 m for all 20 nodes (table in `📓️status.md` log). Screenshot showed the undeformed stack left and the leaning, sagging stack right.

## Notes / left open (not this ticket's scope)
- The mounted `live_visual` session lane refuses the example with `fem3d.numerical-modal-order` (its 40-equation modal cap; the demo is refused by the same lane with a singular-stiffness fault). The engine-backed results window is the deformation lane and works.
- First serve on a freshly created `⚡️cache/vite/os-dev/fem3d-react-dev` hit a dep-optimizer race (`scheduler` not pre-bundled → `does not provide an export named 'default'`); wiping that cache dir and restarting the serve fixed it.
- The descriptor regeneration also captured the sibling ticket's in-flight fem3d editor actions (translateSelection, patch*, …) because it describes the current tree.
