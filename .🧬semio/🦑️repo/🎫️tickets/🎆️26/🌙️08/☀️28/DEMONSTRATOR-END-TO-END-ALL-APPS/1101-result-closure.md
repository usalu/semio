STATUS: PATCH READY, NOT APPLIED (activation freeze 19:50; my edits reverted from the tree, my checks killed). UNVERIFIED: no check of the patched manifests has finished.

## Method
Closures read from each pane workspace `Cargo.lock` (the pane tests use `<pane ws>/Cargo.toml`, not `🌎️hub`). Build closure rebuilt from manifests (normal+build deps for every crate, dev-deps only for the test root, since Cargo.lock lists dev-deps of all members). Unused = dependency key never mentioned (`\bsnake_name\b`) in the crate's sources, following `[lib]/[[test]] path` roots. Tools: `C/1101-tools/closure/{real,unused,prune,sizes}.py`, logs `C/🗑️generated/1101/closure-*`.

## Findings (accident vs real need)
- No `docx`/`tiff` crate is in any pane lock (only `🌎️hub/Cargo.lock` has them; hub carries every app).
- ACCIDENT (fixed): gismap carve. All 12 `gis-gismap-*` component crates (+ `gis2d-unit-tests`) copy one 15-dep stdio list (dwg dxf json pdf png semio semio-base semio-drawing(-export/-import) semio-image semio-value-model/-mutations svg txt). Not one is referenced in their sources (real users live in the `🚪️io` crate, which is not in the verfolgen closure). This alone pulled pdf(55), dwg(26), dxf, png, svg, xml, zip, deflate.
- ACCIDENT (fixed): `fem-3d-io` / `fem-2d-io` / `fem-3d-engine` declared `semio-mesh-export`, `csv json obj stl semio-base semio-mesh` unused. `semio-mesh-export -> drawing-export -> pdf/dwg/png/svg/...` was the whole statik pdf path.
- ACCIDENT (fixed): curation carve (7 crates x obj png stl txt zip json semio*), process3d (`step-geometry`, `semio-brep`, `semio-flow` in host/io/model/mut-stock), generator pieces (`generation3d` step png; 5 `sequence-*` crates csv json md semio*; `plugin-flow-brep` brep-export/import/step-geometry), and six `stdio-semio-{brep,drawing,mesh}-{export,import}` crates that each declared unused `dwg-model`.
- REAL need (kept): process3d gltf/obj/stl/step (its `process3d-io` imports them); energy epw/xml/zip/deflate via energy-model-snapshot; cad pdf/dwg/dxf/png/svg: `cad-cad-io -> semio-mesh-export -> semio-drawing-export`, whose sources really contain pdf/svg/png/dxf/dwg serializers, so the cad app needs them; generator = `semio-s-composition-laws` composes every app, so all families are real. `stdio-semio` (139 crates) is directly used by process3d/curation/energy.
- STRUCTURAL (not fixed, needs design): `stdio-semio-drawing-export` bundles every drawing serializer in one crate, so any consumer pulls pdf+dwg+png+svg together. Splitting it per format is the real fix; pdf crates are stdio2's.
- NOT DONE (candidates): ~100 stdio-internal cross-family unused deps (`C/🗑️generated/1101/closure-removed-AB.txt`, e.g. png-* unused zip/deflate, svg-* unused xml, many families unused txt, pdf-* unused deflate). Worth about -15 crates for cad, -8 for generator, 0 elsewhere; left to the stdio lanes.

## Crates in the build closure (lock closure from before/after manifests; `Cargo.lock` of the pane refreshes on the first `repo cargo` run)
| pane | crate | before | after | stdio families after |
|---|---|---|---|---|
| bearbeiten | process3d-editor | 1122 | 1121 | semio gltf step obj stl json txt contract binary (no pdf) |
| aussuchen | sourcing-curation | 676 | 654 | semio xml zip deflate json txt (png obj stl gone) |
| verfolgen | gis2d-unit-tests | 1034 | 724 | none (pdf 55, dwg 26 and all other families gone) |
| koordinator | cad-cad | 1250 | 1250 | unchanged, real cad io |
| aggregator | puzzle-3d-editor | 905 | 905 | none, nothing to remove |
| statik | fem-3d-editor | 740 | 518 | none (pdf, dwg, gltf, png, svg, zip, xml... gone) |
| energie | energy-mounted-tests | 736 | 736 | semio epw, nothing to remove |
| generator | composition-laws | 1437 | 1423 | all, real (composition of every app) |

## Edited manifests (40, deps only, no source change)
gis: `gis-gismap-{config,gis2d,gis2d-unit-tests,gismap,inferences,mut-any,mut-family,mutations,schema,snapshot,tests,tile-coordinates,windows}`; fem: `fem-2d-io, fem-3d-engine, fem-3d-io`; process3d: `-host -io -model -mut-stock`; curation: `sourcing-curation(+ -domain -examples -inferences -operations -schema -tests)`; generator: `procedural-generation3d`, `sequence-sequence-{editor,inferences,schema,tests,v1}`, `plugin-flow-brep`; stdio: `stdio-semio-{brep,drawing,mesh}-{export,import}`.

## Checks (logs `C/🗑️generated/1101/closure-<pane>.{txt,rc}`)
Serial runner `C/1101-tools/closure/seq.sh` (`check --tests` per pane, native). Results below.

## Freeze handover (19:50)
- Patch: `C/1101-tools/closure/patches/closure-manifest-deps.patch` (32 Cargo.toml, 204 dependency lines removed, 0 added). Apply after the lift from the repo root with `patch -p1 < <patch>`; revert with `patch -R -p1`. Tree is back to its pre-patch state.
- First attempt (302 deps, 40 files) was WRONG: my unused detector ignored `#[path]` chains into sibling `🚪️io` trees, so `cargo check` on verfolgen failed in `gis-gismap-snapshot` (E0433 semio_base, drawing, value_model, drawing_export, svg, image; log `C/🗑️generated/1101/closure-run1/closure-verfolgen.txt`). Peers were building those manifests for a few minutes before I reverted (~17:50); if anything gis/fem/cad/process3d/curation failed around then, that is why. Detector now follows `#[path]` (tools `unused.py`), giving the 204-dep patch; the second attempt never got through the capacity queue (3400 s wait).
- Corrections to the findings above: `fem-3d-io`/`fem-2d-io` DO use `semio-mesh-export` (stl ascii and obj serializers), so statik stays 740. Its pdf path is the `stdio-semio-mesh-export` umbrella (dwg gltf las obj ply png stl + drawing-export in one crate). Splitting it per format (crates for stl, obj, ... under `🚪️io/📤️export/🧵️serializers`) is the only way to take pdf out of statik (518 crates without it); not done, it is a registry/composition refactor.
- Expected sizes with the patch (manifest-derived, lock not yet refreshed): bearbeiten 1122 to 1121, aussuchen 676 to 654, verfolgen 1034 to 724 (no stdio at all), statik 740 (no change), koordinator 1250 (no change, real cad io), aggregator 905, energie 736, generator 1437 (no change).
- Verification plan after the lift: `C/1101-tools/closure/seq.sh` (serial native `check --tests`: verfolgen, statik, aussuchen, bearbeiten, koordinator, generator; logs `C/🗑️generated/1101/closure-<pane>.{txt,rc}`).
