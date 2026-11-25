# Final carve plan: process and demonstrator components under the 4,096 MB unit ceiling

Date 2026-11-25. Read-only analysis; no crate, manifest, descriptor or policy file was edited. Scripts in this folder: `final-carve-manifest-closure.py` (closure and families), `final-carve-cuts.py` (cut simulation), `final-carve-paths.py` (shortest dependency path), `final-carve-closure.py` and `final-carve-analyze.py` (first, Cargo.lock based pass: superseded, it counts dev-dependencies).

## 0. Method and limits (read first)

- `repo cargo ... metadata` could not run: every attempt died at "Unexpected }" in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:100` (another session's in-flight edit; the wrapper retried 3 times). So the closures are computed from the manifests: normal `[dependencies]` only (dev and build dependencies dropped), workspace entries resolved through `[workspace.dependencies]`, optional dependencies and features NOT evaluated (a superset of the real wasm32-wasip2 closure), ticket copies of manifests excluded.
- Closure sizes: process final 922 crates, demonstrator final 1,318 crates.
- rlib bitcode sizes from `.cache/b/r/m`: only 527 framework, ui and plugin rlibs exist there (about 1.9 GB); no `semio_s_artifact_*` or `semio_hub_*` rlib is cached. Per-crate MB for the families below therefore come from `carve-C3-final-memory.md` (the cad final's measurement, 953 libraries, 6,259 MB bitcode, 8,036 MB charged peak, ratio charged/bitcode 1.28) and are marked "C3". Everything else is a crate count; MB estimates are proportional or ratio-derived and unmeasured.
- Crate count does not predict the peak: process has fewer crates (922) than the demonstrator (1,318) yet the larger peak (5.5 vs 4.9 GB). Process links brep/step/3d heavy crates and all of `stdio-semio`; the demonstrator's extra crates are small.

## 1. What each final links that its apps do not need

### Process final (`semio-hub-process-component`, apps: process3d editor and viewer)

Real needs, from source (`grep` in `✏️s/🔌️plugins/🏭️process/🗿️artifacts`): `stdio_semio_brep` (27 uses), `stdio_semio_flow` (14), `stdio_semio_base` (4), `stdio_step` (8), `stdio_gltf`, `stdio_json` (2), `stdio_obj`, `stdio_stl`.

What it links instead: `semio-hub-process` depends on the aggregate crate `semio-s-artifact-stdio-semio` only to name one type, `SemioMembers`, in `🏭️process/🦀️.rs:14-15`, `process3d/…/👁️viewer/🦀️.rs:92` and `…/✏️editor/🪟️editor-impl/🦀️.rs:65`. That aggregate has 72 direct dependencies: 18 `*-member` roster crates and every stdio codec (`avi bcf bmp csv docx dwg dxf gif gltf ifc jpg json las md mp3 mp4 obj pdf ply png pptx step stl svg tiff txt wav xml zip`). Shortest path for each unneeded family: `semio-hub-process -> semio-s-artifact-stdio-semio -> <codec>`.

| Unneeded family reached through `stdio-semio` | Crates | Note |
|---|---|---|
| pdf (+ pdf14) | 80 | largest by count |
| dwg | 27 | read/write/snapshot; `dwg_snapshot_objects` is 229 MB (C3) but is reached only in the demonstrator, not here |
| docx, pptx, svg, xml, png, gif, dxf, bmp, tiff, mp4, jpg, mp3, avi, wav, zip, las, ply, bcf, ifc, csv, md | 5 to 12 each | |
| members and editors: `presentation-member` 165 MB, `model-member` 153, `brep-member` 145, `graph-editor` 140, `document-member` 104 (C3) | 129 crates in the `stdio-semio` family | 707 MB bitcode for these five |

Simulation (`final-carve-cuts.py`): replace the aggregate by `semio-s-artifact-stdio-semio-{base,brep,flow}` + `stdio-contract`: 922 to 793 crates (-129, all `stdio-semio` family). The simulation keeps process's own direct codecs (step, gltf, json, obj, stl, txt, binary).

Dependencies of process that are not wiring only: `semio-framework-3d` (mesh engine 54 MB), `semio-plugin` closure (`semio-plugin-actor-input` 205 MB (C3), unavoidable, every final has it), `stdio-binary-io-registry` 190 MB (C3, via `stdio_stl`).

### Demonstrator final (`semio-hub-demonstrator-component`)

Real needs: apps `playground`, `sourcing` (editor + viewer) and `process3d` (editor + viewer) while the panes Aussuchen and Bearbeiten use the bundle. The bundle also links: `cad-cad-editor`, `gis-gismap`, `procedural-generation3d`, `procedural-3d-editor`, `puzzle-3d`, `puzzle-3d-editor` for panes that run on their own components (`runtime/🔣️.json`: generator -> `generation3d`, koordinator -> `cad`, aggregator -> `puzzle3d`, verfolgen -> `gis2d`, each served by its own `🌎️hub/🧩️compositions/{🌀️procedural,📐️cad,🧩️puzzle,🌍️gis}/🧩️component` playground row).

| Unneeded family (path from `semio-hub-demonstrator`) | Crates | Note |
|---|---|---|
| `dwg_snapshot*` 12 crates, `stdio-dwg` 34 | `procedural-generation3d -> stdio-dwg-snapshot -> dwg-snapshot-objects` | 229 MB (C3) for `dwg_snapshot_objects` alone |
| pdf 80, svg 11, xml, png, `semio-drawing-export/-import`, mesh/brep import/export | `gis-gismap -> stdio-semio-drawing-export -> stdio-pdf` | |
| artifact-procedural 78, puzzle 35, cad 34, gis 14 | direct | |
| `stdio-xlsx` 9 | direct dependency of `semio-hub-demonstrator`; no source use in the demonstrator or in sourcing | |
| `*-tests` crates in the release closure: gismap, deflate, dxf, las, pdf, png, svg, xml, zip (16 in total) | via the codecs above | see 3 below: they are not test-only |

Hub crates in the closure: only `semio-hub-demonstrator(-component)`; the six `semio-hub-*` plugins are dev-dependencies (standalone parity), so they are not linked.

## 2. Ranked cuts

Charged estimate = bitcode x 1.28 (C3) or proportional share of the measured 5.5 / 4.9 GB, flagged.

| # | Cut | Unit | Crates | Estimate | Status |
|---|---|---|---|---|---|
| D1 | Remove cad, gis, puzzle, generation3d surfaces from the demonstrator bundle (6 `Cargo.toml` entries, 5 enum variants, 5 `.editor::<>` rows with rosters, 4 hosted kinds, 4 dependencies, tests) | demonstrator | 1,318 -> 966 (-352: pdf 80, procedural 78, puzzle 35, dwg 34, cad 34, gis 14, svg 11, framework 19) | about -1.3 GB proportional (352/1318 of 4.9 GB) giving about 3.6 GB; a floor of 0.3 GB from `dwg_snapshot_objects` alone (229 MB x 1.28) | proposed, see 4 |
| P1 | Replace `SemioMembers` of the aggregate by a process-specific roster crate over `stdio-semio-{base,brep,flow}` (+ the members those need) in `semio-hub-process`, the process3d viewer and editor | process | 922 -> 793 (-129) | about -0.9 GB (707 MB bitcode of five named members x 1.28) giving about 4.6 GB: not yet under 4,096 | proposed, process crates untouched |
| P2 | Same roster idea for the demonstrator after D1: sourcing needs `stdio_semio` 21 uses, so the roster is the union (process + sourcing): the demonstrator after D1 + P1 is 866 crates (-100) | demonstrator | 966 -> 866 | about -0.4 GB | optional, only if D1 alone does not fit |
| D2 | Drop unused `semio-s-artifact-stdio-xlsx` (and csv, json, zip if sourcing declares its own) from `semio-hub-demonstrator` | demonstrator | 966 -> 957 (xlsx), -> 919 (all four, upper bound) | -0.05 to -0.3 GB | proposed, verify sourcing's own deps |
| P3 | `semio-framework-3d` / `mesh-engine` (54 MB) only behind a feature that process3d viewer needs; process editor may not need it | process | few | -0.07 GB | only after P1, measure first |

Reading: D1 brings the demonstrator to about 3.6 GB on its own (before D2/P2) with the margin of the estimate unknown; P1 alone does not bring process under the ceiling (about 4.6 GB). Process needs P1 plus one more structural cut from its own heavy members (`stdio_semio_brep` family 27 uses, `process3d-model` -> `stdio-step`, `binary-io-registry` 190 MB) or a thin-LTO scope change by the carving owner; this cannot be settled without a measured per-crate bitcode list for the real wasm32-wasip2 closure.

## 3. Correction of an earlier carve hint: `*-tests` crates are not test-only

`carve-C3` lists `semio_s_artifact_stdio_txt_tests` (160 MB) as "a tests crate in a release closure" and suggests moving it. It is not movable: `semio-s-artifact-stdio-txt-tests` is the carved component crate that holds the runtime code (`TxtApps`, `ARTIFACT_DEFINITION_SCHEMA`, `artifact`, `declaration`, `native_codecs`, `formats`), re-exported by `stdio-txt/🦀️.rs:13-30`; the same shape holds for `stdio-json-tests`, `stdio-semio-graph-mut-fixture-tests` and `stdio-gltf-inferences-projection-tests`. The name comes from the carving of `🧪️tests/🔬️unit`; moving them to `[dev-dependencies]` would break the codecs. The correct carve is a rename plus a split, a separate ticket.

## 4. Demonstrator cut (task 3): not implemented

Condition was: only if every consumer is found and updated. It is not met. Consumers found for D1:

- `🌎️hub/🧩️compositions/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs` (enum variants, builder rows, 6 `use`), `🧪️tests/🔬️unit/🦀️.rs` (manifest id list, hosted kinds, setContributions consumers, window render list, genesis-children counts for cad 4, gis 2, puzzle 0, generation 0), `🧪️tests/🔬️standalone-parity/🦀️.rs` (4 of 6 standalone hosts), `🧪️tests/🔬️surface/🦀️.rs`.
- `📦️packages/🦀️rust/Cargo.toml` (5 artifact dependencies + 4 dev-dependencies + metadata comments) and `🦀️.rs`.
- `🧩️component/📦️packages/🦀️rust/Cargo.toml`: `depends-on` cad, gis, procedural, puzzle; 4 playground rows (generator 6027, koordinator 6028, aggregator 6023 with aliases `mit-bestand` and `entwerfen-mit-bestand`, verfolgen 6032) with brands and `devContribution`; 5 asset rows (`/cad-assets`, `/mesh`, `/infinite-assets`, `/osm`, `/vt`); `sources.artifacts` of four plugins; `consumes`. These dev rows and aliases would have to move to the four standalone components, which changes their brand/alias/port tables.
- `🧩️component/🔣️.json`: 59,949 lines, 227 lines mention cad, puzzle, gis or procedural; the descriptor is authored through `authored-release-descriptors.ts` in this ticket, so it has to be regenerated, not hand-cut.
- Cross-tree consumers: `📋️build-owners.json`, `🌎️hub/.config/nextest.toml`, `🌎️hub/🧩️compositions/🧪️tests/🧫️fixtures/📇️ownership/🔣️.json`, `🌎️hub/🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/…/🛫️catalog-selection-preflight/🔣️.json`, `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🔣️.json`, kernel fixtures `app-router-plugin-faults` and `consumed-topics-for-app`, `🪪️artifact-admission` fixture, `🔌️plugin-architecture` law, `capability-policy` test, repo `workspace-contract` and `test-level-budgets` fixtures, the brand file and test of the mit-bestand shell, wgpu shell tests that name the pane brands.
- Verification is not possible now either: `repo cargo` is down (other session's syntax error above) and `repo capacity status` showed 3,866 MB available (< 4.5 GiB). A cut spread over about 25 files, a regenerated descriptor and four other components' playground tables, unverified in the shared worktree two days before a redeploy, is a risk this plan does not take.

Recommended order for whoever implements: (a) fix the budget `📥️opening/🟦️.ts` syntax error, (b) move the four playground rows, aliases and asset rows to their standalone components together with their brand/port tests, (c) apply D1 in the manifest, enum, Cargo.toml, tests, (d) regenerate the descriptor, (e) `repo cargo --store=shared --configuration=prototype -- check -j 2 -p semio-hub-demonstrator` once > 4.5 GiB available, (f) release final to re-measure.
