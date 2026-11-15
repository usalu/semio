# Pipeline report: volumes consumed per scale and mode

Scope: everything in `S` = `🎓️teaching/🏛️architecture/🧍️staffage` that consumes `asset.volumes`. Scripts in `T` carry the prefix `pipeline-`, output is under `T/🗑️generated/pipeline/`.

## Result against the 22 gaps of `audit-pipeline.md`

| Gap | State |
|---|---|
| 1, 2, 3, 4 (one tessellation and one solid set for every scale) | Fixed. `qualityOf(asset, scale)` (`S/🔨️modules/🧊️volume/⏱️job`) is the single rule of the viewer, every download and the budget oracle; every scale has its own authored solids (no `maxDenominator`); scenes are tessellated one by one at the quality of their own scale (`S/🔨️modules/📤️export/🗿️model/🧩️scenes`). Feature-edge visibility stays per scene (edges off in the solid views above 1:100). |
| 5, 6 (pose, rig, analyses identical) | By design identical: bone frames and the thermal, structural and transport solutions are physical, so they are solved once per instant and shared by every scene (`instantsOf`). Only glyph sizes (paper mm times scale) are per scene. |
| 7 (authored representations never consulted) | Fixed. `authoredAt` refuses `model-scale-unknown` and `model-scale-unavailable`; `modelOfAsset` refuses with `model-no-authored-scale`; no fallback anywhere (depiction raises `ViewMissingError`, the viewer says which scales exist). |
| 8, 9, 22 (no pens or seams in the 3D document) | Unchanged by design: pens are a property of the 2D drawing; the 3D document keeps one feature-edge material per scene. |
| 10 (static colours ignore theme) | Unchanged (documented). |
| 11, 12, 13 (one scale per file, one view per FBX) | Fixed for scales: a GLB and the model JSON hold every authored scale and mode as scenes (all seven views as variants); an FBX is one clip, one scale, one view, one scene (kernel limit), named `<asset>.<clip>.s<scale>.<view>.fbx`. |
| 14 (surface ids renumber) | Fixed: surface ids are per scene and per list, and material ids carry the scene prefix, so they cannot clash or shift across scales. |
| 15 (root motion differs viewer vs files) | Fixed. The viewer no longer cuts root travel (`inPlace` removed); it uses the kernel's own `follow: bounds`. Files and viewer play the same document. |
| 16 (batch deletes other scales: data loss) | Fixed and tested (below). |
| 17 (two naming conventions) | Fixed: one module `S/🔨️modules/📤️export/🗿️model/🏷️names` is used by the library, batch, CLI and React downloads. |
| 18 (all views solve for every build) | Fixed: a mode solves only its own analysis (`body` none, `thermal` the field and emitters, `structure` the member result); `modelOfAsset` solves each once. |
| 19, 20, 21 (modes share geometry) | Fixed: `body`, `thermal` and `structure` are different lists; forces and moments glyphs exist only in the structure scenes, emitter cones in the thermal scenes; the 2D and 3D colours are compared per region and member in `model-looks` and the axonometric tests. |

## What was built

- `S/🔨️modules/🧊️volume`: `modelOf(asset, { scale, view, … })` (one scale, the mode of the view, its views as variants), `modelOfAsset(asset, { scales? })` (one kernel scene per authored scale and mode over one skeleton, scene-specific identity child nodes of the bones, the seven views as variants, poses and clips once). `listOf` resolves `{ of }` to the body geometry. Thermal look: one material per region and layer, inner shells of a multi-layer region show the skin value. Structure look: member and joint solids take the colour of their member bone, supports, masses and non-members are neutral. Mode, scene id, `authoredScales` live in `🔨️modules/🔗️contracts` (entry-safe).
- Exports: `writeGlb` (all scales as glTF scenes), `writeFbx*` (`writeFbxViews` builds and tessellates once per clip, scale and mode and encodes every view of the mode), `writeModel` (multi-scene document, resolved on request), `writeModelBundle`, CLI (`--scale` optional for glb, model, model-zip), `plannedFiles` as the one plan of the file set. Kernel `scenes` are used as the kernel agent specified.
- Batch (`🔨️modules/📤️export/📦️batch`): every authored scale in one run; `--scale a,b` and `--view v,w` restrict only the FBX files; a file is kept when its own content `key` (digest of asset document, producing code, schema and kernel, presets, thermal options) matches and its bytes hash to the manifest; only files that the asset's full plan no longer names are deleted, so other scales' files are never removed, whatever their key; assets without an authored scale are listed in the summary (and refused when named). `producer` option fixes the code digest for tests because the tree is edited live. Deterministic, progress and cancellation as before.
- Axonometric depiction: the list of the view's mode at the drawn scale (`body`, `thermal`, `structure`), `ViewMissingError` for an unauthored scale; support and mass solids are neutral in the member views exactly as in the model look.
- React: one model per asset, pose or clip and analytic setting (`modelOfAsset`); `sceneOf({scale, view})` selects `s<scale>-<mode>` with `viewer.setScene` and the variant, so scale and view never rebuild; unauthored scales show a bilingual notice naming the scales that exist and the scale control marks them (`viewer.scaleNoModel`); downloads use the shared tessellation and names. Site: `model-split` still keeps the 3D code out of the entry chunk (the file names live in a dependency-free module for that reason).
- Fixtures and oracles: `S/🧫️fixtures/🧊️volume-scales` (handcrafted four-scale sample figure, source `T/pipeline-fixture.py`), `S/🧫️fixtures/🧊️model-digests` (regenerated by `T/pipeline-digests.ts`), `S/🧫️fixtures/🧊️model-catalog` (pending list, regenerated by `T/pipeline-pending.py`).

## Tests (final targeted run: 10 files, 237 tests green: model-scales, -scale-files, -batch, -exports, -looks, -digests, -skeleton, -frame, axonometric-depiction and -visibility; earlier green runs: -bounds, -glyph-edges, -silhouette, -catalog on authored assets, React model-view, model-downloads, model-viewer-page, site model-split)

| Test | What it proves |
|---|---|
| `🧪️tests/🧊️model-scales` (38) | every scale and view yields a valid kernel document of exactly the list of the view's mode; lists and scales differ; `of` shares geometry; thermal materials per region; structure colours; refusals by name; scenes order, partial scales, scene-wise tessellation with falling triangles, determinism, cancellation. |
| `🧪️tests/🧊️model-scale-files` (2) | the whole file set of the fixture through every oracle: gltf-validator, three `GLTFLoader` and `FBXLoader`, glTF-Transform `NodeIO`, assimp, Ajv, fflate. |
| `🧪️tests/🧊️model-batch` (11) | layout, manifest, single-export equality, **scale 50 then scale 100 into one directory keeps both byte for byte (the missing test)**, stale key keeps other scales, determinism, resume, producer key, unauthored assets, cancellation, command line. |
| `🧪️tests/🧊️model-exports`, `-looks`, `-skeleton`, `-bounds`, `-glyph-edges`, `-frame`, `-silhouette`, `-digests`, `🧊️axonometric-*` | migrated to the new API and semantics (2D/3D colours compared by bone, region and member; digests per pose and per clip, scale and checked view). |
| React `model-view`, `model-downloads`, `model-viewer-page` (46+) | scene selection without rebuild, scale refusal and recovery, one tessellation rule, new file names, no-volume refusals. |
| Site `model-split` | 3D code stays out of the entry chunk. |

## Catalog guard (`🧪️tests/🧊️model-catalog`)

Extended to every authored scale and every scene: valid multi-scene model, every scene tessellated at its scale's quality and watertight/finite, falling budgets per mode, GLB scenes (gltf-validator, three, glTF-Transform), FBX per clip, scale and checked view (three and assimp, triangles equal to the GLB scene for body and thermal), model JSON scenes, per-scale body extent against `dimensions`, ZIP equals files. FBX views default to one per mode (`light-solid`, `thermal`, `forces`); `SEMIO_STAFFAGE_MODEL_VIEWS=all` checks all seven; `SEMIO_STAFFAGE_MODELS_DIR` checks an existing batch output.

Not-yet-re-authored assets without masking: an asset with no authored scale must appear in `🧫️fixtures/🧊️model-catalog` `pending`; a pending asset that is authored fails the single test "names exactly the assets without an authored scale as pending" until the line is removed (ratchet, run `T/pipeline-pending.py` to regenerate), while the per-asset test already runs the full guard on it. No test is skipped. Last regeneration: 140 pending of 152 (12 authored then, growing); when the list is empty the guard is complete. Authored assets checked so far (hornbeam-hedge, dining-table, expansion-valve): clean.

## Decisions

- Scenes need scene-specific mesh nodes; solids ride on `<scene>-<bone>` identity children so the skeleton is shared and nothing else.
- Custom denominators are no longer available in 3D (only authored preset scales have a model); the 2D scale control stays free.
- One FBX per clip, scale and view (design) is large for the whole catalog: the batch shares tessellation per mode, but a full run with all seven views is big; `--view` restricts it.
- `qualityOf(asset)` without a scale remains as the finest reference quality for the silhouette validators of the schema agent.
- The axonometric depiction keeps its own paper-resolution chord (0.04 mm of paper, `qualityAt`): it draws vectors, not meshes, and is not a viewer or download.

## Open and honest limits

- No type check was run (`tsc` over the package); only vitest (esbuild) executed, so type errors in new code would not have been caught. Run `repo run @teaching/staffage:typecheck`.
- Not run: `repo run @semio-tech/framework-3d-js:test`, the full `@teaching/staffage:test`, `@teaching/staffage-react:test`, `@teaching/staffage-site:test`, `@teaching/staffage:export-models` (no whole-repo graph runs while others run them).
- No visual check of the viewer or an axonometric render (no GPU; the React tests use the software backend). The follow-the-figure root-motion rule in the viewer is untested visually.
- Python consumer under `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage` does not read volumes (checked by search), so nothing changed there.
- `S/🧪️tests/🧊️volumes-rules` and `🧊️volume-schema` belong to the schema agent and were not touched.
- Silhouette overlays per scale and mode (`volumes-check` rendering) are the authoring tool's, not this pipeline's.
