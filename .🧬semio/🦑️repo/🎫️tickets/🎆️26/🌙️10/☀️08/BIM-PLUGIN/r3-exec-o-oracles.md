# 🔮️ r3 exec report: `o-oracles`

Companion: `r3-oracle-pattern.md` (the recipe). `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`P` = `✏️s/🔌️plugins/🏙️bim`, `H` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

## 1. Environment

Gap found: `.venv` (python 3.14.4) lacked `ifcopenshell` and `shapely` although root `pyproject.toml` pins them (test group) and
`uv.lock` locks `ifcopenshell 0.8.4.post1` (has `py314-none-win_amd64` wheel) and `shapely 2.1.2` (`cp314-win_amd64`). No repo
setup path installs that group: `bun ./📜️script.ts setup` (`SetupScript`: `postinstall|git|native|deps|prepare|devcontainer`)
never calls `uv sync`; `.devcontainer/devcontainer.json` installs `uv` only. The repo's own zero-touch route is the Python
test host, which builds a cache venv from `oracleHostPackages` per case (`H/🖥️host/🏗️materialization/🟦️.ts`
`provisionPythonInterpreter`). So: declared both in `P/🔮️oracles/🔣️.json` (works on a clean checkout), and installed additively
into `.venv` for direct runs:

```
$ uv pip install --python .venv/Scripts/python.exe ifcopenshell==0.8.4.post1 shapely==2.1.2
Resolved 8 packages in 896ms ... Installed 3 packages in 569ms
 + ifcopenshell==0.8.4.post1
 + isodate==0.7.2
 + shapely==2.1.2
$ .venv/Scripts/python.exe versions.py        (file; versions.py = import ifcopenshell, shapely; print versions)
ifcopenshell 0.8.4.post1
shapely 2.1.2
```
Proposed (NOT applied, other owners' scripts): in root `📜️script.ts` `NativeDependenciesScript` (`setup deps`) add
`uv sync --group test --frozen` when `uv` is present, so `.venv` matches `pyproject.toml` on every OS/devcontainer.

## 2. Built (all new files; nothing of f1's edited)

| File | Purpose |
|---|---|
| `P/🔮️oracles/🔣️.json` (edited, additive) | `oracleHostPackages`: python `shapely 2.1.2`, `ifcopenshell 0.8.4.post1` |
| `S/🔮️oracles/🔣️.json` (new) | oracle rows `bim-1-shapely-geometry` (family geos) and `bim-1-ifcopenshell-kernel` (family ifcopenshell), both `third-party-library`, capability `bim-1-infer`, profile `floating-point-v1`. Waves M/X APPEND `mutationCatalogs`, `mutationManifests`, `testEvidence` here (currently empty arrays) |
| `S/🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json` | 5 storeys (basement level -1, ground, first, roof, shed in 2nd building), 7 walls (lines + 1 arc, all three `TopConstraint` kinds, two location lines), 2 wall types, site 100 m / buildings +2.5 / -0.5 |
| `S/🧫️fixtures/💡️inferences/🏠️house/💡️inference/🪜️storey-levels/🔣️.json`, `…/🧱️wall-layout/🔣️.json` | expected tables, written by the oracle (`write`) |
| `S/🧪️tests/🪜️infer-bim-1-levels-and-wall-heights/{🥒️.feature,🐍️.py}` | scenarios `storey-levels`, `wall-layout`; shapely audit + parametric law |
| `S/🧪️tests/🧊️infer-bim-1-wall-solids/{🥒️.feature,🐍️.py}` | scenario `wall-solids`; ifcopenshell kernel z-extent + volume |
| `T/r3-o-oracles-validate-snapshot.py` | one-off: validates the fixture snapshot against f1's `🧬️schema/📸️snapshot` JSON Schema |
| `T/r3-oracle-pattern.md` | the recipe |

Fixture shape = r2-design §2; confirmed against f1's real `snapshot`/`entities`/`values` Rust and JSON Schema (Point2 `{x,y}`, externally
tagged `Axis`/`TopConstraint`, unit-variant strings `Center`/`New`). Result of validating it:

```
$ .venv/Scripts/python.exe r3-o-oracles-validate-snapshot.py <S> <S>/🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json
artifact $id https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json
errors: 0
```

## 3. Semantics the oracle commits (the Rust inference must match; nothing in `💡️inferences` existed yet to read)

- `🪜️storey-levels`, per storey: `elevation` = `fsum` of heights of the storeys of the SAME building with a lower `level`; the lowest
  storey of a building is 0. `top_elevation` = elevation + height. `absolute_elevation` = `site.elevation + building.elevation + elevation`
  (same for `absolute_top_elevation`). Basement (level -1) sits at 0, ground at 2.6, i.e. adding a basement shifts the datum
  (matches r2-design "parent = storey below by level"; if f1 chooses level 0 as datum, change `storey_levels` and re-`write`).
- `🧱️wall-layout`, per wall: `base_z = storey.elevation + base_offset` (building-relative); `top_z`: `Unconnected{height}` = base + height;
  `StoreyTop{offset}` = storey.top_elevation + offset; `Storey{storey,offset}` = target.elevation + offset (target must be in the same
  building); `height = top_z - base_z`; `thickness = Σ layers`; `length` closed form (arc: `|4·atan(bulge)|·chord / (2·sin(|sweep|/2))`);
  `side_area = length·height`; `footprint_area = length·thickness` (UNJOINED, any location line); `volume = footprint_area·height`.
- Field names are snake_case as in the tables. Tolerance `floating-point-v1` = 1e-9.

## 4. Commands and results

Direct (no host):
```
$ python <case>/🐍️.py write <S>/🧫️fixtures/💡️inferences       -> 🏠️house: shapely 2.1.2, 5 storeys, 7 walls / write: oracle agrees
$ python 🪜️infer-bim-1-levels-and-wall-heights/🐍️.py check …   -> check: oracle agrees   exit=0
$ python 🧊️infer-bim-1-wall-solids/🐍️.py check …                -> 🏠️house: ifcopenshell 0.8.4.post1 measured 6 of 7 walls (arcs excluded) / check: oracle agrees  exit=0
```
Negative controls (scratch copies, not committed): editing a committed `elevation` and a `top_z` in the tables made case 1 print two `[FAIL]`
lines and exit 1; raising a committed `height` made case 2 print `w-ground-south.top_z: ifcopenshell 5.8, committed 5.6` and
`.volume: ifcopenshell 7.68, committed 7.2` and exit 1; a layout whose `top_z` was raised made `kernel` report
`w-ground-south: ifcopenshell z_max 5.6 != 5.7`.

Through the platform (real host, real python env provisioning, oracle role):
```
$ cd H && bun ./📜️script.ts discover | grep bim
test-s-plugins-bim-artifacts-model-standards-1-subsets-any-692e06-🧊️infer-bim-1-wall-solids        … [python]
test-s-plugins-bim-artifacts-model-standards-1-subsets-any-692e06-🪜️infer-bim-1-levels-and-wall-heights … [python]
$ SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🪜️infer-bim-1-levels-and-wall-heights
[test] level=quick cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0        (1 m 22 s, includes env build)
$ SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🧊️infer-bim-1-wall-solids
[test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0        (44 s)
$ bun ./📜️script.ts contract --case 🪜️infer-bim-1-levels-and-wall-heights          (repo-wide, 8 m 18 s)
```
`contract` breach file `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`: no breach names `infer-bim`, `bim-1-infer`, `bim-1-shapely-geometry` or
`bim-1-ifcopenshell-kernel`. The only BIM breach is `unregistered-mutation-vocabulary` for `S/🧬️schema/🧬️mutations` (high): Wave M must
append a `mutationCatalogs` row to `S/🔮️oracles/🔣️.json`.

## 5. Findings / traps

- `ifcopenshell.geom.create_shape(...).geometry` must be bound to a variable together with the shape: chaining it frees the shape and
  `util.shape.get_vertices` returns an empty array.
- Default IFC units from `api.unit.assign_unit` are millimetres; the API converts, the kernel reports metres.
- A feature carries exactly ONE `@oracle-<id>`, so one case per oracle library (hence two cases).
- `subject`/`parity` phases were not run: the Rust adapters (`🦀️.rs` in both case dirs with `Adapter::new("rust").subject("storey-levels"|"wall-layout"|"wall-solids", …)`)
  do not exist yet because `💡️inferences/🪜️storey-levels` and `🧱️wall-layout` are not written; whoever lands them adds the adapters and
  compares the `ModelInference` projection field-for-field with the committed tables.
- `🔒️dependencies.json` (generated, `generatedAt 2026-10-03`) does not yet list the new oracle ids under `shapely` and `ifcopenshell`; regenerate with
  `bun ./📜️script.ts dependency` (not run).
- No launch.json entry was added (Wave R; also the dashboard ticket deletes `.vscode/launch.json`).
- Left in `T/🗑️generated/o-oracles/` (scratch, delete with the ticket): `versions.py`, `shape.py`, `dump.py`, `ids.py`, `ifc_probe*.py`, `debug_kernel.py`,
  `negative_kernel.py`, `patch*.py`.
