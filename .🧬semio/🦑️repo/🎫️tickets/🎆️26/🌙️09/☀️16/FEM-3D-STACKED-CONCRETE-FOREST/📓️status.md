# 🌲️ Fem 3D stacked concrete forest example — status

App under work: `fem3d` of plugin `✏️s/🔌️plugins/🏗️fem` (artifact `🗿️artifacts/🧊️3d`, crate `semio-s-artifact-fem-3d`). Sibling live ticket: `FEM-3D-INTERACTIVE-FEATURE-COMPLETE` (session ⚪9dc2b27f, same day) — it owns the editor/interaction/house work; this ticket only adds the `concrete-forest` example and its proof. Repo MCP down; bookkeeping manual on disk. Start commit: `🗑️generated/start-commit.txt`.

## Definition of done
1. `📚️examples/🌲️concrete-forest` + `🖼️assets/🌲️concrete-forest/🗣️.dsl.semio`: two Hexagonal Cut Concrete Forest Left pieces stacked (lower z 0→3, upper z 3→6), nodes = puzzle3d vortex coordinates, frames = CAD structure-classic members (2 columns, 7 cantilevers, 1 spine per piece), hexagonal column + 30×45 beam sections, C30/37, self weight + slab dead UDL + live UDL, ULS/SLS combinations.
2. Registered: crate `examples` module, subset `examples()` list, `setActiveExample` handler + palette option.
3. Deformation solved by `fem3d_solve_all` in a unit test (finite, residual tiny, reactions balance the applied load, upper tips deflect more than lower tips) and validated by an independent numpy 3D frame oracle in the ticket folder.
4. Native check + nextest green for the touched crate; React lane shows the example's deformed shape (console proof).

## Closed 2026-09-16 20:53 — see `📓️concrete-forest-2026-09-16.md`.

## Log
- 2026-09-16 20:20 open. Read CAD play fixture model 3 (`aec.building.structure.classic`), puzzle3d `🌲️concrete-forest` vortices, fem3d demo example wiring.
- 20:27 Authored `🖼️assets/🌲️concrete-forest/🗣️.dsl.semio` (20 nodes, 20 frames, hex30/beam30x45, dead/live UDLs, uls/sls), example module + TS mirror + tests; registered in crate root `examples`, subset `examples()`, `setActiveExample` (match on id), palette select option. Derivation: `📓️model.md`.
- 20:33 numpy oracle `🐍️frame-oracle.py` solves the asset: reactions dead 684.6 kN / live 227.3 kN; SLS peak −5.94 mm at uv4; every cantilever sag beyond its head's rigid motion equals wL⁴/8EI (1.628 mm / 0.102 mm) — the asymmetric hexagonal cut leans the heads apart in y (lower ±1.34 mm, upper ±4.94 mm) and lifts the light-side tips v0/v3. Rust example test rewritten to assert exactly those laws. `📜️test-concrete-forest.sh 1` queued behind a fleet of peer cargo runs (artifact-dir lock).
- 20:32 `📜️test-concrete-forest.sh 1`: 3/3 green (parse+round-trip, topology on vortices, deformation laws). Rust SLS displacements vs numpy oracle: 20/20 nodes, worst relative diff 3.1e-7. `📜️test-set-active-example.sh 1`: 20/20 green (set_active_example + example tests with the editor feature). fem-js vitest: 6 files / 8 tests green incl. the new `🧩️example/🟦️.ts`.
- 20:34 `📜️activate-fem3d-react.sh 1` running (wasm guest rebuild with the new example), then serve on 6087 + `🐍️concrete-forest-probe.mjs`.
- 20:36 Activation 1 green (fem wasm 80.7 MB, descriptor lists `concrete-forest` for `s.fem.fem3d`). First serve on 6087 hit a vite dep-optimizer race (`scheduler` missing from the fresh `fem3d-react-dev` deps cache → `does not provide an export named 'default'`); wiped `⚡️cache/vite/os-dev/fem3d-react-dev` and restarted — boots.
- 20:44 Probe 3: navbar combobox lists `No example|Demo|Concrete Forest`; picking it dispatches `setActiveExample id=concrete-forest nodes=20 elements=20 solids=0`, live_visual reconcile spawns `meshItems=197 drawInstances=194`, `results solve #3` runs. BUT the whole fem3d actor is down since boot: results window `DuplicateSiblingKey parent=#0 key=#0` on the DEMO (sibling ticket's in-flight results-window caption change; the peer patched `with_caption` at 20:45). The mounted `live_visual` lane also faults `fem3d.numerical-modal-order` (its 40-equation modal cap — the same class of refusal it gives the demo; the engine results window is the real proof lane). Rebuilding the wasm with the peer's fix: `📜️activate-fem3d-react.sh 2`.
- 20:48 Activation 2 (peer's `with_caption` fix in) → probe 4/5: `ready=fem3d`, both windows render (234 instances each after the switch); results-window node offsets / 300 match the oracle dead case to 2.5e-15 m on all 20 nodes:
```
node     model z  results z  ui uz(mm)  oracle uz(mm)  ui uy(mm)  oracle uy(mm)
lc1t      3.0000     2.9636    -0.1214        -0.1214    -0.9781        -0.9781
lc2t      3.0000     2.9585    -0.1383        -0.1383     0.9781         0.9781
lv0       3.0000     3.0530     0.1766         0.1766    -0.4262        -0.4262
lv1       3.0000     2.0909    -3.0303        -3.0303     0.4262         0.4262
lv2       3.0000     2.1107    -2.9643        -2.9643     1.5301         1.5301
lv3       3.0000     3.0918     0.3060         0.3060     0.4262         0.4262
lv4       3.0000     2.0521    -3.1597        -3.1597    -0.4262        -0.4262
lv5       3.0000     2.1597    -2.8011        -2.8011    -1.5301        -1.5301
lv6       3.0000     2.9461    -0.1797        -0.1797     1.5301         1.5301
uc1t      6.0000     5.9459    -0.1804        -0.1804    -3.6143        -3.6143
uc2t      6.0000     5.9382    -0.2060        -0.2060     3.6143         3.6143
uv0       6.0000     6.2559     0.8530         0.8530    -2.4508        -2.4508
uv1       6.0000     4.7444    -4.1854        -4.1854     2.4507         2.4507
uv2       6.0000     4.8575    -3.8085        -3.8085     4.7779         4.7779
uv3       6.0000     6.3045     1.0149         1.0149     2.4507         2.4507
uv4       6.0000     4.6958    -4.3473        -4.3473    -2.4508        -2.4508
uv5       6.0000     4.9214    -3.5955        -3.5955    -4.7779        -4.7779
uv6       6.0000     5.9724    -0.0920        -0.0920     4.7779         4.7779
```
- 20:52 `📜️describe-and-registry.sh 1`: describe + registry green; `✏️s/🔌️plugins/🏗️fem/🔣️.json` lists `concrete-forest`. Report: `📓️concrete-forest-2026-09-16.md`. Closing on disk (repo MCP down); serve on 6087 left running for the dev.
