# 🔬️ Audit: Carving the 79 Oversized Plugin Crates

Read-only audit. No source edited, no git writes, no builds, no `🗑️generated` folder deleted. Inputs: `🗑️generated/plugin-census.csv` (1,357 manifests), `AUDIT-REFUSED-AND-PLUGIN-GATE.md`, `📓️WAVE4-BRIEF.md`, `PRODUCER-CRATE-RECIPE.md`. Scratch scripts and machine output are in `🗑️generated/audit-plugin-carve/` (`analyze.ts`, `analyze2.ts`, `analyze3.ts`, `deps.ts`, `table.ts`; `rows.json`, `rows2.json`, `rows3.json`, `deps.json`, `classes.json`, `table.md`). Those are generated-folder files and go away with the folder.

Measurement: the canonical `readRustSourceClosureV1` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`) on each crate's lib entry, with host cfg, `debug_assertions` and default features (lib closure). The same closure with `test` added gives the test closure. Embedded `include_str!`/`include_bytes!` files count toward files and bytes, as the census does.

## 1. Headline

- **79 oversized crates**: 77 artifact crates and 2 plugin extension/engine crates (`P:wfc-engine`, `P:flow-brep`). 47 of them are `🗄️stdio`.
- **Binding limit:** files 70, bytes 7, lines 2. The file budget (32) is the wall for almost every crate.
- **Dominant cause, primary class:** (a) mount fan-out 69, (c) embedded assets 6, (b) monolithic source files 4, (d) other 0. Full table in section 2.
- **Why (a) dominates:** the lib root of an artifact, standard or subset mounts every taxonomy node beneath it. Each taxonomy directory holds at most one source file. In all 73 crates I could measure, no directory holds more than one source file, and the median crate spans 72 directories. A 32-file crate therefore fits about 32 taxonomy nodes.
- **Test closures are also over:** 72 of 79 exceed 32 files in the test closure (median +43 files).
- **Embedded files alone do not fix anything:** removing every embedded file from each closure lets 1 of 79 pass. Removing only the 1,852 schema companion files lets 0 of 79 pass. Source fan-out has to be fixed.
- **Systemic fixes** (section 5) cover most crates at once: a crate per component directory in the scaffolders, a crate per mutation leaf, schema companions out of the closure, a runtime asset channel for 8 assets of at least 256 KiB, and an enforced lib-root line budget.
- **Bottom-up order:** `stdio-contract` is the hub (108 direct and 535 transitive dependents). Five crates have no over-limit dependency and can start now (layer 0). Section 6 gives the wave plan.

## 2. Per-crate table

Sorted by ratio. Ratio = max(files/32, bytes/262144, lines/4000) on the lib closure. "Binds" is the census binding dimension. Cause: **a** mount fan-out, **b** monolithic sources, **c** embedded assets, **d** other; "(+x)" is a secondary cause (c: embedded share at least 25% of files or 20% of bytes; b: largest source at least 40 KB with top-3 files at least 35% of source bytes; a: file count over 32 with `#[path]` mounts at least the source file count). "Lib root lines" is the artifact's `🦀️.rs` root. "Over-limit deps" are direct path dependencies that are themselves in the 79. "Dependents" is the transitive count of plugin manifests that reach the crate by path. "Layer" is the bottom-up layer from the over-limit dependency graph (0 = no oversized dependency).

Paths in column "Artifact crate" are relative to `✏️s/🔌️plugins/<plugin>/`. Full manifests are in `🗑️generated/plugin-census.csv`.

| # | Plugin | Artifact crate | Crate | Files | Bytes | Lines | Ratio | Binds | Cause | Emb files / bytes | Lib root lines | Over-limit deps (direct) | Dependents (transitive) | Layer |
|---:|---|---|---|---:|---:|---:|---:|---|---|---|---:|---|---:|---:|
| 1 | 🏙️bim | 🗿️artifacts/🏢️model | `semio_s_artifact_bim_model` | 1288 | 6,280,428 | 88,421 | 40.25 | files | **a** (+c) | 41 / 1268 KB | 128 | stdio-csv, stdio-gltf-model, stdio-json, stdio-pdf, stdio-svg, stdio-xml | 1 | 5 |
| 2 | 🗄️stdio | 🗿️artifacts/📖️pdf | `semio_s_artifact_stdio_pdf` | 721 | 10,539,688 | 52,626 | 40.21 | bytes | **c** (+a) | 62 / 7077 KB | 1411 | stdio-binary, stdio-contract, stdio-deflate | 86 | 3 |
| 3 | 🧩️puzzle | 🗿️artifacts/🖐️5d | `semio_s_artifact_puzzle_5d` | 323 | 5,900,517 | 26,646 | 22.51 | bytes | **c** (+a) | 51 / 4179 KB | 2509 | stdio-json, stdio-obj, stdio-png, stdio-semio-drawing, stdio-semio-kit, stdio-stl, stdio-txt, stdio-zip | 1 | 6 |
| 4 | 🗄️stdio | 🗿️artifacts/🎞️gif | `semio_s_artifact_stdio_gif` | 158 | 5,047,093 | 9,264 | 19.25 | bytes | **c** (+a) | 61 / 4428 KB | 509 | stdio-binary, stdio-contract | 15 | 2 |
| 5 | 🗄️stdio | 🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences | `semio_s_artifact_stdio_gltf_inferences` | 362 | 522,511 | 6,626 | 11.31 | files | **c** (+ba) | 270 / 156 KB | 89 | stdio-gltf-model | 21 | 5 |
| 6 | 🏛️architect | 🗿️artifacts/🏛️program | `semio_s_artifact_architect_program` | 176 | 2,828,975 | 23,387 | 10.79 | bytes | **c** (+a) | 40 / 1444 KB | 563 | stdio-csv, stdio-json, stdio-semio, stdio-tsv, stdio-txt, stdio-zip | 1 | 5 |
| 7 | 🧩️puzzle | 🗿️artifacts/◻️2d | `semio_s_artifact_puzzle_2d` | 343 | 2,684,738 | 25,883 | 10.72 | files | **a** (+c) | 75 / 962 KB | 1491 | stdio-dxf, stdio-json, stdio-pdf, stdio-semio-drawing, stdio-png, stdio-svg, stdio-txt | 1 | 6 |
| 8 | 🗒️note | 🗿️artifacts/🗒️note | `semio_s_artifact_note_note` | 248 | 819,779 | 12,501 | 7.75 | files | **a** | 36 / 100 KB | 1530 | stdio-dxf, stdio-pdf, stdio-semio-drawing, stdio-svg | 1 | 5 |
| 9 | 🎥️shooting | 🗿️artifacts/🎥️shooting | `semio_s_artifact_shooting_shooting` | 219 | 662,055 | 9,695 | 6.84 | files | **a** (+b) | 41 / 111 KB | 1479 | stdio-bmp, stdio-gif, stdio-jpg, stdio-json, stdio-pdf, stdio-png, stdio-semio-drawing, stdio-semio-image, stdio-svg, stdio-tiff, stdio-txt | 1 | 7 |
| 10 | 📋️forms | 🗿️artifacts/📋️forms | `semio_s_artifact_forms_forms` | 195 | 717,250 | 10,410 | 6.09 | files | **a** | 38 / 72 KB | 984 | stdio-csv, stdio-zip, stdio-json, stdio-semio | 1 | 5 |
| 11 | 🌍️gis | 🗿️artifacts/🗺️gismap | `semio_s_artifact_gis_gismap` | 190 | 1,043,823 | 12,035 | 5.94 | files | **a** (+c) | 43 / 309 KB | 1018 | stdio-dxf, stdio-json, stdio-pdf, stdio-png, stdio-semio, stdio-semio-drawing, stdio-semio-image, stdio-svg, stdio-txt | 3 | 6 |
| 12 | 🔱️trinity | 🗿️artifacts/♻️rewriting | `semio_s_artifact_trinity_rewriting` | 161 | 499,048 | 7,068 | 5.03 | files | **a** | 31 / 61 KB | 813 | stdio-json, stdio-md, stdio-pdf, stdio-txt, stdio-semio, stdio-semio-graph, trinity-jack | 1 | 5 |
| 13 | ✒️writer | 🗿️artifacts/✒️writer | `semio_s_artifact_writer_writer` | 155 | 428,042 | 7,483 | 4.84 | files | **a** (+b) | 33 / 20 KB | 688 | stdio-json, stdio-md, stdio-pdf, stdio-semio, stdio-semio-document, stdio-txt, trinity-jack | 1 | 5 |
| 14 | 🎞️animate | 🗿️artifacts/🎬️presentation | `semio_s_artifact_animate_presentation` | 152 | 734,115 | 14,018 | 4.75 | files | **a** (+c) | 39 / 59 KB | 856 | stdio-gif, stdio-html, stdio-json, stdio-md, stdio-mp4, stdio-pdf, stdio-png, stdio-semio, stdio-semio-animation, stdio-semio-document, stdio-semio-presentation, stdio-svg, stdio-xml | 1 | 6 |
| 15 | 🗄️stdio | 🗿️artifacts/🧿️semio | `semio_s_artifact_stdio_semio` | 150 | 513,940 | 4,880 | 4.69 | files | **a** (+c) | 62 / 188 KB | 413 | stdio-contract, stdio-semio-animation, stdio-semio-brep, stdio-semio-cad, stdio-semio-document, stdio-semio-drawing, stdio-semio-flow, stdio-semio-graph, stdio-semio-image, stdio-semio-kit, stdio-semio-mesh, stdio-semio-model, stdio-semio-object, stdio-semio-presentation | 56 | 4 |
| 16 | 🗄️stdio | 🗿️artifacts/🎨️svg | `semio_s_artifact_stdio_svg` | 145 | 644,677 | 9,703 | 4.53 | files | **a** | 31 / 99 KB | 605 | stdio-contract, stdio-xml | 96 | 4 |
| 17 | ➗️mathematical | 🗿️artifacts/➗️equation | `semio_s_artifact_mathematical_equation` | 143 | 865,774 | 14,763 | 4.47 | files | **a** (+b) | 31 / 80 KB | 1047 | stdio-csv, stdio-json, stdio-md, stdio-semio | 1 | 5 |
| 18 | 🀄️wfc | 🗿️artifacts/🧊️3d | `semio_s_artifact_wfc_3d` | 139 | 537,663 | 8,800 | 4.34 | files | **a** (+b) | 29 / 62 KB | 673 | stdio-semio-mesh, stdio-txt, P:wfc-engine | 2 | 3 |
| 19 | 🀄️wfc | 🗿️artifacts/◻️2d | `semio_s_artifact_wfc_2d` | 139 | 573,682 | 8,719 | 4.34 | files | **a** | 28 / 98 KB | 532 | stdio-png, stdio-semio-image, stdio-txt, P:wfc-engine | 2 | 6 |
| 20 | 🔱️trinity | 🗿️artifacts/🔌️jack | `semio_s_artifact_trinity_jack` | 132 | 716,017 | 12,082 | 4.13 | files | **a** | 30 / 38 KB | 1038 | - | 5 | 0 |
| 21 | 🌊️flow | 🗿️artifacts/🌊️flow | `semio_s_artifact_flow_flow` | 131 | 530,302 | 8,480 | 4.09 | files | **a** (+b) | 29 / 17 KB | 783 | stdio-csv, stdio-json, stdio-md, stdio-semio, stdio-semio-flow, stdio-txt | 1 | 5 |
| 22 | 🗄️stdio | 🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations | `semio_s_artifact_stdio_gltf_mutations` | 131 | 732,261 | 12,910 | 4.09 | files | **a** | 2 / 1 KB | 310 | stdio-gltf-model | 22 | 5 |
| 23 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph | `semio_s_artifact_stdio_semio_graph` | 127 | 579,170 | 8,750 | 3.97 | files | **a** | 30 / 81 KB | 347 | stdio-contract | 57 | 1 |
| 24 | 🌿️vcs | 🗿️artifacts/🌿️vcs | `semio_s_artifact_vcs_vcs` | 123 | 315,978 | 4,770 | 3.84 | files | **a** (+cb) | 40 / 44 KB | 549 | stdio-csv, stdio-json, stdio-zip | 1 | 5 |
| 25 | 🪵️sourcing | 🗿️artifacts/🗂️curation | `semio_s_artifact_sourcing_curation` | 123 | 486,506 | 6,907 | 3.84 | files | **a** (+cb) | 40 / 65 KB | 814 | stdio-json, stdio-obj, stdio-png, stdio-semio, stdio-semio-kit, stdio-stl, stdio-txt, stdio-zip | 7 | 6 |
| 26 | 🗄️stdio | 🗿️artifacts/📰️xml | `semio_s_artifact_stdio_xml` | 122 | 634,759 | 9,359 | 3.81 | files | **a** (+c) | 33 / 115 KB | 544 | stdio-contract, stdio-txt | 187 | 3 |
| 27 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing | `semio_s_artifact_stdio_semio_drawing` | 120 | 516,806 | 6,757 | 3.75 | files | **a** | 29 / 96 KB | 308 | stdio-contract | 189 | 1 |
| 28 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh | `semio_s_artifact_stdio_semio_mesh` | 118 | 448,535 | 6,306 | 3.69 | files | **a** (+c) | 30 / 72 KB | 312 | stdio-contract | 189 | 1 |
| 29 | 🗄️stdio | 🗿️artifacts/🧾️json | `semio_s_artifact_stdio_json` | 116 | 563,173 | 8,135 | 3.63 | files | **a** (+c) | 32 / 57 KB | 538 | stdio-contract, stdio-txt | 152 | 3 |
| 30 | 🀄️wfc | 🗿️artifacts/🖼️bitmap | `semio_s_artifact_wfc_bitmap` | 115 | 527,670 | 8,689 | 3.59 | files | **a** | 28 / 51 KB | 669 | P:wfc-engine | 2 | 1 |
| 31 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document | `semio_s_artifact_stdio_semio_document` | 114 | 459,717 | 6,703 | 3.56 | files | **a** (+c) | 30 / 57 KB | 141 | stdio-contract | 62 | 1 |
| 32 | 🗄️stdio | 🗿️artifacts/🖼️tiff | `semio_s_artifact_stdio_tiff` | 114 | 484,672 | 5,804 | 3.56 | files | **a** (+c) | 32 / 82 KB | 526 | stdio-png, stdio-jpg, stdio-deflate, stdio-binary, stdio-contract | 11 | 6 |
| 33 | 🕸️dag | 🗿️artifacts/🕸️dag | `semio_s_artifact_dag_dag` | 112 | 316,526 | 5,041 | 3.50 | files | **a** (+cb) | 38 / 29 KB | 701 | stdio-csv, stdio-json, stdio-md, stdio-png, stdio-semio, stdio-semio-graph, stdio-svg, stdio-xml | 1 | 6 |
| 34 | 🪐️space | 🗿️artifacts/🏠️home | `semio_s_artifact_space_home` | 112 | 368,287 | 5,467 | 3.50 | files | **a** (+cb) | 39 / 23 KB | 438 | stdio-csv, stdio-json, stdio-txt, stdio-zip | 2 | 5 |
| 35 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit | `semio_s_artifact_stdio_semio_kit` | 111 | 380,214 | 4,892 | 3.47 | files | **a** (+c) | 30 / 80 KB | 287 | stdio-contract, stdio-semio-model, stdio-semio-object | 80 | 3 |
| 36 | 🀄️wfc | 🗿️artifacts/🔲️grid2d | `semio_s_artifact_wfc_grid2d` | 111 | 518,771 | 8,443 | 3.47 | files | **a** | 26 / 58 KB | 576 | stdio-json, stdio-semio-image, stdio-txt, P:wfc-engine | 2 | 4 |
| 37 | 🀄️wfc | 🗿️artifacts/🧱️grid3d | `semio_s_artifact_wfc_grid3d` | 110 | 499,849 | 7,841 | 3.44 | files | **a** (+c) | 28 / 73 KB | 560 | stdio-semio-mesh, stdio-txt, P:wfc-engine | 2 | 3 |
| 38 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep | `semio_s_artifact_stdio_semio_brep` | 109 | 549,460 | 7,143 | 3.41 | files | **a** (+c) | 30 / 98 KB | 275 | stdio-contract | 135 | 1 |
| 39 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad | `semio_s_artifact_stdio_semio_cad` | 108 | 353,233 | 4,786 | 3.38 | files | **a** (+c) | 30 / 60 KB | 136 | stdio-contract, stdio-semio-drawing | 59 | 2 |
| 40 | 📜️imperative | 🗿️artifacts/📜️procedure | `semio_s_artifact_imperative_procedure` | 108 | 319,542 | 5,080 | 3.38 | files | **a** (+cb) | 37 / 30 KB | 737 | stdio-csv, stdio-json, stdio-md, stdio-semio, stdio-semio-flow, stdio-txt | 1 | 5 |
| 41 | 📖️playbook | 🗿️artifacts/📖️playbook | `semio_s_artifact_playbook_playbook` | 107 | 296,461 | 4,571 | 3.34 | files | **a** (+cb) | 40 / 37 KB | 823 | stdio-json, stdio-semio, stdio-semio-flow | 1 | 5 |
| 42 | 🗄️stdio | 🗿️artifacts/🎒️zip | `semio_s_artifact_stdio_zip` | 106 | 502,879 | 7,706 | 3.31 | files | **a** (+c) | 31 / 69 KB | 547 | stdio-binary, stdio-contract, stdio-deflate, stdio-xml | 169 | 4 |
| 43 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation | `semio_s_artifact_stdio_semio_presentation` | 105 | 418,340 | 5,841 | 3.28 | files | **a** (+c) | 30 / 63 KB | 140 | stdio-contract, stdio-semio-document | 59 | 2 |
| 44 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow | `semio_s_artifact_stdio_semio_flow` | 102 | 323,915 | 4,979 | 3.19 | files | **a** (+c) | 30 / 44 KB | 137 | stdio-contract, stdio-semio-drawing | 82 | 2 |
| 45 | 🌍️gis | 🗿️artifacts/🏔️gisterrain | `semio_s_artifact_gis_gisterrain` | 100 | 294,554 | 4,431 | 3.13 | files | **a** (+cb) | 33 / 28 KB | 555 | gis-gismap, stdio-json, stdio-las, stdio-obj, stdio-ply, stdio-png, stdio-semio-mesh, stdio-stl, stdio-txt | 1 | 7 |
| 46 | 🗄️stdio | 🗿️artifacts/📸️jpg | `semio_s_artifact_stdio_jpg` | 99 | 417,041 | 6,384 | 3.09 | files | **a** (+c) | 31 / 45 KB | 527 | stdio-binary, stdio-contract | 33 | 2 |
| 47 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation | `semio_s_artifact_stdio_semio_animation` | 99 | 314,732 | 4,586 | 3.09 | files | **a** (+c) | 30 / 50 KB | 134 | stdio-contract | 59 | 1 |
| 48 | 💡️reasoning | 🗿️artifacts/🔌️wires | `semio_s_artifact_reasoning_wires` | 99 | 331,388 | 4,830 | 3.09 | files | **a** (+cb) | 28 / 26 KB | 661 | stdio-csv, stdio-md, stdio-semio, stdio-semio-graph | 1 | 5 |
| 49 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image | `semio_s_artifact_stdio_semio_image` | 97 | 284,705 | 4,462 | 3.03 | files | **a** (+c) | 29 / 40 KB | 131 | stdio-contract | 85 | 1 |
| 50 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model | `semio_s_artifact_stdio_semio_model` | 97 | 388,165 | 5,522 | 3.03 | files | **a** (+c) | 30 / 59 KB | 139 | stdio-contract | 107 | 1 |
| 51 | 🗄️stdio | 🗿️artifacts/📷️png | `semio_s_artifact_stdio_png` | 96 | 417,665 | 5,198 | 3.00 | files | **a** (+cb) | 31 / 75 KB | 463 | stdio-binary, stdio-contract, stdio-deflate, stdio-zip | 96 | 5 |
| 52 | 🗄️stdio | 🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object | `semio_s_artifact_stdio_semio_object` | 93 | 236,059 | 3,509 | 2.91 | files | **a** (+c) | 30 / 36 KB | 223 | stdio-contract, stdio-semio-brep, stdio-semio-mesh | 81 | 2 |
| 53 | 🗄️stdio | 🗿️artifacts/🔤️txt | `semio_s_artifact_stdio_txt` | 89 | 263,625 | 3,977 | 2.78 | files | **a** (+c) | 32 / 41 KB | 363 | stdio-binary, stdio-contract | 257 | 2 |
| 54 | 🗄️stdio | 🗿️artifacts/🗽️obj | `semio_s_artifact_stdio_obj` | 88 | 508,478 | 6,629 | 2.75 | files | **a** (+cb) | 31 / 111 KB | 409 | stdio-contract, stdio-txt | 70 | 3 |
| 55 | 🗄️stdio | 🗿️artifacts/🪟️bmp | `semio_s_artifact_stdio_bmp` | 87 | 282,634 | 3,448 | 2.72 | files | **a** (+c) | 31 / 49 KB | 333 | stdio-binary, stdio-contract | 11 | 2 |
| 56 | 🎬️sequence | 🗿️artifacts/🎬️sequence | `semio_s_artifact_sequence_sequence` | 87 | 462,132 | 7,610 | 2.72 | files | **a** (+cb) | 29 / 29 KB | 802 | stdio-csv, stdio-json, stdio-md, stdio-semio, stdio-semio-flow | 1 | 5 |
| 57 | 🗄️stdio | 🗿️artifacts/☁️las | `semio_s_artifact_stdio_las` | 85 | 412,343 | 6,339 | 2.66 | files | **a** (+c) | 31 / 62 KB | 407 | stdio-binary, stdio-contract | 56 | 2 |
| 58 | 🗄️stdio | 🗿️artifacts/🖋️dxf | `semio_s_artifact_stdio_dxf` | 84 | 607,320 | 8,626 | 2.63 | files | **a** (+cb) | 31 / 108 KB | 430 | stdio-contract, stdio-txt | 78 | 3 |
| 59 | 🗄️stdio | 📇️registry/🧬️contract | `semio_s_artifact_stdio_contract` | 19 | 546,250 | 9,967 | 2.49 | lines | **b** | 1 / 6 KB | 1743 | - | 535 | 0 |
| 60 | 🗄️stdio | 🗿️artifacts/💬️bcf | `semio_s_artifact_stdio_bcf` | 79 | 386,428 | 5,748 | 2.47 | files | **a** (+c) | 31 / 62 KB | 394 | stdio-binary, stdio-contract, stdio-xml, stdio-zip | 3 | 5 |
| 61 | 🗄️stdio | 🗿️artifacts/🌦️epw | `semio_s_artifact_stdio_epw` | 77 | 316,901 | 3,815 | 2.41 | files | **a** (+c) | 32 / 100 KB | 289 | stdio-contract | 1 | 1 |
| 62 | 🗄️stdio | 🗿️artifacts/📊️csv | `semio_s_artifact_stdio_csv` | 77 | 277,040 | 3,956 | 2.41 | files | **a** (+cb) | 34 / 46 KB | 382 | stdio-contract, stdio-txt | 40 | 3 |
| 63 | 🗄️stdio | 🗿️artifacts/📼️avi | `semio_s_artifact_stdio_avi` | 76 | 275,933 | 4,078 | 2.38 | files | **a** (+cb) | 31 / 49 KB | 273 | stdio-contract | 13 | 1 |
| 64 | 🗄️stdio | 🗿️artifacts/📝️md | `semio_s_artifact_stdio_md` | 76 | 438,279 | 5,921 | 2.38 | files | **a** (+c) | 31 / 81 KB | 423 | stdio-contract, stdio-txt | 21 | 3 |
| 65 | 🗄️stdio | 🗿️artifacts/🎥️mp4 | `semio_s_artifact_stdio_mp4` | 75 | 477,859 | 5,350 | 2.34 | files | **a** (+cb) | 31 / 175 KB | 273 | stdio-contract | 17 | 1 |
| 66 | 🗄️stdio | 🗿️artifacts/📑️tsv | `semio_s_artifact_stdio_tsv` | 75 | 235,729 | 3,292 | 2.34 | files | **a** (+cb) | 33 / 40 KB | 286 | stdio-contract | 2 | 1 |
| 67 | 🪐️space | 🗿️artifacts/🪐️space | `semio_s_artifact_space_space` | 75 | 212,827 | 3,369 | 2.34 | files | **a** | 10 / 10 KB | 328 | - | 2 | 0 |
| 68 | 🀄️wfc | ⚙️engine | `semio_s_plugin_wfc_engine` | 43 | 398,750 | 9,356 | 2.34 | lines | **b** (+a) | 0 / 0 KB | 98 | - | 11 | 0 |
| 69 | 🗄️stdio | 🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema | `semio_s_artifact_stdio_gltf_model` | 55 | 609,045 | 8,304 | 2.32 | bytes | **b** (+ca) | 14 / 63 KB | 100 | stdio-json | 68 | 4 |
| 70 | 🎪️demonstrator | 🗿️artifacts/🎪️playground | `semio_s_artifact_demonstrator_playground` | 74 | 158,754 | 2,435 | 2.31 | files | **a** (+c) | 31 / 28 KB | 395 | stdio-csv, stdio-json, stdio-zip | 1 | 5 |
| 71 | 🗄️stdio | 🗿️artifacts/🔺️stl | `semio_s_artifact_stdio_stl` | 74 | 272,419 | 3,709 | 2.31 | files | **a** (+c) | 31 / 50 KB | 427 | stdio-binary, stdio-contract, stdio-txt | 70 | 3 |
| 72 | 🗄️stdio | 🗿️artifacts/🧱️ply | `semio_s_artifact_stdio_ply` | 74 | 379,856 | 5,245 | 2.31 | files | **a** (+cb) | 31 / 73 KB | 403 | stdio-contract, stdio-txt | 56 | 3 |
| 73 | 🗄️stdio | 🗿️artifacts/🗜️deflate | `semio_s_artifact_stdio_deflate` | 73 | 351,572 | 5,691 | 2.28 | files | **a** (+cb) | 31 / 51 KB | 416 | stdio-binary, stdio-contract | 176 | 2 |
| 74 | 🗄️stdio | 🗿️artifacts/💾️binary | `semio_s_artifact_stdio_binary` | 73 | 205,670 | 2,800 | 2.28 | files | **a** (+c) | 31 / 37 KB | 375 | stdio-contract | 301 | 1 |
| 75 | 🗄️stdio | 🗿️artifacts/🌐️html | `semio_s_artifact_stdio_html` | 72 | 308,051 | 4,358 | 2.25 | files | **a** (+c) | 31 / 55 KB | 267 | stdio-contract | 2 | 1 |
| 76 | 🗄️stdio | 🗿️artifacts/🔊️wav | `semio_s_artifact_stdio_wav` | 67 | 374,145 | 4,709 | 2.09 | files | **a** (+cb) | 28 / 94 KB | 267 | stdio-contract | 3 | 1 |
| 77 | 🗄️stdio | 🗿️artifacts/🎵️mp3 | `semio_s_artifact_stdio_mp3` | 66 | 269,227 | 2,988 | 2.06 | files | **a** (+c) | 28 / 98 KB | 275 | stdio-contract | 3 | 1 |
| 78 | 🗄️stdio | 🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor | `semio_s_artifact_stdio_gltf_editor` | 16 | 358,157 | 1,306 | 1.37 | bytes | **c** | 2 / 279 KB | 99 | stdio-contract, stdio-gltf-model, stdio-gltf-mutations | 0 | 6 |
| 79 | 🌊️flow | 🧩️extensions/📐️brep | `semio_s_plugin_flow_brep` | 7 | 303,671 | 4,039 | 1.16 | bytes | **b** (+c) | 2 / 3 KB | 6 | - | 3 | 0 |

## 3. Cause clusters

Counts are primary causes from the table. 47 of the 79 crates are in `🗄️stdio`; 22 other plugin families contribute the rest.

| Cluster | Crates | What it is | Members (primary) |
|---|---:|---|---|
| (a) Mount fan-out | 69 | One lib root mounts every taxonomy leaf below it (standards, subsets, schema, io, mutations, inferences, viewer, editor, examples). Every taxonomy directory holds at most one source file (measured), so the file count tracks the node count. | stdio 41; wfc 5 (`3d`, `2d`, `bitmap`, `grid2d`, `grid3d`); trinity 2 (`jack`, `rewriting`); gis 2; space 2 (`home`, `space`); bim, puzzle-2d, note, shooting, forms, writer, animate, mathematical, flow-flow, vcs, sourcing, dag, imperative, playbook, reasoning, sequence, demonstrator (1 each) |
| (b) Monolithic sources | 4 | A few source files of 74 KB to 145 KB and 1,300 to 2,500 lines dominate the bytes or lines budget. | `stdio-contract` (4 files of 74 to 104 KB, 9,967 lines, a hub); `stdio-gltf-model` (`🔺️diff/🦀️.rs`, 101,882 bytes, 2,385 lines); `P:wfc-engine` (`💼️job/🦀️.rs`, 2,182 lines); `P:flow-brep` (`📐️brep/🦀️.rs`, 144,558 bytes, 2,506 lines) |
| (c) Embedded assets | 6 | Embedded payloads (examples, `.pdf`, `.gif`, `.glb`, schema JSON, or the four schema companion files per node) dominate the bytes or file budget. | `stdio-pdf` (6.3 MB PDF example in the lib closure, 7.1 MB embedded); `stdio-gif` (4.4 MB GIF example); `puzzle-5d` (3.1 MB and 0.86 MB example data); `architect-program` (schema snapshot JSON, 375 KB and 318 KB); `stdio-gltf-inferences` (270 embedded files, about 67 inference nodes with four companions each); `stdio-gltf-editor` (284 KB `.glb`) |
| (d) Other | 0 | none found | none |

Secondary causes: embedded share is at least 25% of files or 20% of bytes in 60 of 79 crates. A monolithic file (at least 40 KB, top-3 at least 35% of source bytes) appears in 29 of 79.

Embedded files in the 79 closures total 2,737 (25% of the 10,917 closure files). Of those, 1,852 (68%) are schema companions: `🔗️.graphql`, `🛰️.proto`, `🔣️.json` and `🟦️.ts` beside a schema or inference node `🦀️.rs`. Eight embedded assets of at least 262,144 bytes (256 KiB) sit in 5 crates. Each one alone exceeds any crate's byte budget, so they need a runtime channel, not a carve.

Test closures (`test` cfg added): 72 of 79 exceed 32 files, 72 exceed 262,144 bytes and 69 exceed 4,000 lines. The median test closure has 43 more files than its lib closure.

## 4. Dependency structure and bottom-up order

Path-dependency graph from the 1,357 manifests (workspace-inherited path deps resolved through the owning `[workspace]`; dev-dependencies excluded). Dependents are plugin manifests and deps reaching the crate.

| Oversized crate | Direct dependents | Transitive dependents | Oversized direct deps |
|---|---:|---:|---|
| `stdio-contract` | 108 | 535 | none (layer 0) |
| `stdio-binary` | 20 | 301 | contract |
| `stdio-txt` | 34 | 257 | binary, contract |
| `stdio-xml` | 32 | 187 | contract, txt |
| `stdio-json` | 49 | 152 | contract, txt |
| `stdio-deflate` | 5 | 176 | binary, contract |
| `stdio-semio-drawing` | 26 | 189 | contract |
| `stdio-semio-mesh` | 21 | 189 | contract |
| `stdio-semio-brep` | 14 | 135 | contract |
| `stdio-png` | 20 | 96 | binary, contract, deflate, zip |
| `stdio-gltf-model` | 16 | 68 | json |

Layers over the 79 (layer = 1 + max layer of over-limit direct deps; 0 = none): L0 = 5 (stdio-contract, trinity-jack, wfc-engine, space-space, flow-brep); L1 = 17; L2 = 10; L3 = 12; L4 = 5; L5 = 19; L6 = 9; L7 = 2.

- The stdio codecs are the common substrate. 27 of the 30 oversized non-stdio crates depend directly on an oversized stdio crate (bim, puzzle-2d and -5d, architect, gis x2, trinity-rewriting, writer, animate, mathematical, flow-flow, wfc artifacts, vcs, sourcing, dag, space-home, imperative, playbook, reasoning, sequence, demonstrator, note, shooting, forms).
- Carving a crate in place keeps its public path (a facade with `pub use`). Dependents then need no edits. The order still matters for the gate: a layer's crates should be within limits before their dependents are measured as carvable leaves (the census `carvableLeaf` rule).

## 5. Systemic fixes and where they live

Each fix is listed with its evidence and generator. Items S1 to S3 remove the (a) cause in bulk. S4 and S5 are needed for the (c) and (b) crates regardless.

**S1. Scaffolders stop at leaf files, so no crate boundary exists below an artifact, standard or subset.** The authoring CLI is `CleanMechanismNewScript` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts` (usage at lines 33 to 37, `bun ./📜️script.ts new artifact|standard|subset|mutation`). Its leaf writers are in `🏗️authoring/🗿️artifact-tree/🟦️.ts`: `newScaffoldArtifactTree` (root doc comment: "mounts standards"), `newScaffoldStandardTree` ("mounts subsets") and `newScaffoldSubsetTree` ("mounts schema/io/viewer/editor/examples"). They write one stub `🦀️.rs` per node and never a crate boundary. The taxonomy already declares the boundaries (`🔣️taxonomy.json`, in `📚️library`): `artifactComponentDirs` = schema, io; `standardComponentDirs` = subsets; `subsetComponentDirs` = schema, io; `subsetSurfaceDirs` = viewer, editor. `🏗️builder/🟦️.ts:61` and `:75` already branch on these component dirs. Fix: when a component dir is created, emit its own `📦️packages/🦀️rust/Cargo.toml` and lib root, and let the parent re-export it with `pub use`. The taxonomy gives those boundaries, so no new dirs are needed.

**S2. Mutations: one crate per leaf, not one mount per leaf.** `🏗️authoring/🧬️mutation-tree/🟦️.ts:194` writes `#[path = "<name>/🦀️.rs"] pub mod <m>;` plus an enum variant into the aggregate root (`newMutationUpdateAggregate`, lines 181 to 222). The closures hold 869 mutation directories (40 crates have at least 10). The taxonomy comment `_mutationOwnershipComment` says each mutation dir owns one `🦀️.rs`, and `mutationAggregateSources` already declares the physical roots. Fix: each mutation leaf becomes a producer crate, and the aggregate uses `pub use <crate>::Mutation as <Variant>`.

**S3. Schema companions out of the closure.** Each schema or inference leaf's Rust file carries `FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!(…), proto: include_str!("🛰️.proto") }`. Example: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs:63`. The taxonomy lists the five formats in `schemaFacetKinds.🧬️data.formats` and `schemaFormats`. The facet loader is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗿️artifact/📚️facet-leaves/🟦️.ts`. The schema generator (`bun ./📜️script.ts schema generate`, `SchemaScript.generate` at `📜️script.ts:14496`) renders only `🔣️schema-catalog.json`. I found no emitter for the `FacetLeaves` block in `📚️library` or in `📜️script.ts`, so these leaves are hand-authored per node. Fix: serve companion text from the generated catalog or from the node's own crate. This removes 1,852 embedded files but does not clear any crate alone (section 3), so it must go with S1.

**S4. Runtime asset channel for embedded payloads.** Eight embedded assets are at least 262,144 bytes and cannot fit any crate's byte budget: PDF example 6.3 MB and PDF schema snapshot 0.30 MB (`stdio-pdf`), GIF example 4.4 MB (`stdio-gif`), puzzle-5d DSL 3.1 MB and JSON 0.86 MB, architect schema JSON 0.38 MB and 0.32 MB, glb 0.28 MB (`stdio-gltf-editor`). Fix: move them behind a runtime port, the same way `CompilerFonts` and print's `ChartSchemaAssets` work (per WAVE4-BRIEF step 2). The 25 assets of at least 64 KB should move with them.

**S5. Source-level splits for monoliths.** Largest files in the closures (lines): `mathematical-equation` `💡️inferences/🌿️cas-internals` 5,072; `bim-model` `🔺️diff/🩹️patches` 3,752; `sequence` editor 3,738; `stdio-pdf` editor `🖼️page` 2,991; `flow-flow` editor 2,939; `P:flow-brep` `📐️brep` 2,506; `stdio-gltf-model` `🔺️diff` 2,385; `stdio-pdf` snapshot 2,298 and diff 2,285; `P:wfc-engine` `💼️job` 2,182; `stdio-dxf` diff 2,133; `stdio-contract` `✏️editing/🪟️details` 2,118. Twenty-eight source files of 1,500 or more active lines appear among the top four per crate. Split with the component rule from S1 and S2.

**S6. Enforce the lib-root budget.** `libWiringLineBudget` = 150 in `🔣️taxonomy.json` is declared (typed at `🔍️discovery/🟦️.ts:1158`) and asserted in `🧪️tests/🔬️workspace-contract/🟦️.ts:2781`, but nothing enforces it on lib roots. Lib roots in the 79 are a median 423 lines (66 exceed 150), and carry 20 to 439 `#[path]` mounts each (e.g. gif root 508 lines, 120 mounts; pdf root 1,411 lines, 369 mounts). A carved crate's root also needs a `pub use` facade that keeps the consumer paths, e.g. `crate::standards::v87a::subsets::any::schema::snapshot` in the gif schema leaf (line 4).

**S7. Mount text emitters.** The only `#[path = …] pub mod` emitters I found in `📚️library` are the mutation aggregate writer (S2) and the relocation rewrite `rust-mod` in `🧹️normalization/🟦️.ts` (about lines 3465 to 3469). The nested `#[path = "."]` trees in the artifact roots are therefore hand-maintained or produced by relocation rewrites, not by `new artifact|standard|subset`. The plugin registry generator (`bun ./📜️script.ts generate plugin-glue`, `📜️script.ts:453`) writes only `📦️packages/🦀️rust/🤖️generated-subset-inventory.txt`. It emits no mounts.

**S8. Counter refusals to fix before measuring test closures.** Six test closures are refused by the counter: `bim-model` (unsupported `include_str` expression, line 13), `stdio-pdf` (generated input, test fixture `⚖️lopdf-vectors`), `space-home` (generated input, `🧪️tests/🔬️unit`), `puzzle-2d`, `sourcing-curation` and `P:flow-brep` (missing source input under `test`: the fixture or data path does not exist where the counter looks; checked `📐️brep/` and the `import-snapshot` fixtures). Whether `cargo test` breaks on these is unverified, because no builds ran.

## 6. Recommended wave plan

Waves follow the bottom-up layers. Packages in one wave run in parallel when their plugin families differ; waves 3 to 6 are one stdio lane. Shared files (root `Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `🔒️dependencies.json`) take one integrator per wave. Each crate gets the PRODUCER-CRATE-RECIPE steps and a census row for lib and test.

- **Wave 0 (serial, generator owner, no plugin edits):** S1, S2, S3 (catalog-served companions), S4 (asset port), S6 (budget gate), S7 (emitter audit), S8 (counter fixes). Re-run the census after each step.
- **Wave 1 (serial):** WP-HUB `stdio-contract` (layer 0). Carve its four monolith files and the `📇️registry/🧬️contract` children. Keep the `pub use` facade so the 108 direct dependents need no edits.
- **Wave 2 (parallel, disjoint plugins):** WP-WFC-ENGINE `P:wfc-engine`; WP-FLOW-BREP `P:flow-brep` (flow plugin, extension only); WP-TRINITY-JACK `trinity-jack`; WP-SPACE-SPACE `space-space`. All layer 0.
- **Wave 3 (stdio, layer 1):** binary and the codecs and semio composites that depend only on the contract, in the stdio lane. 17 crates: binary, tsv, html, mp4, epw, avi, wav, mp3, semio-drawing, -model, -brep, -mesh, -animation, -document, -graph, -image, and `wfc-bitmap` (WP-WFC after wave 2). Within the lane, crates in one layer are disjoint directories and may be split across executors only with one stdio integrator.
- **Wave 4 (stdio, layer 2):** txt, deflate, semio-object, gif (c), semio-cad, semio-flow, semio-presentation, bmp, jpg, las (10 crates).
- **Wave 5 (stdio, layer 3):** csv, json, pdf (c), xml, obj, semio-kit, stl, dxf, md, ply, wfc-3d, wfc-grid3d (12 crates).
- **Wave 6 (stdio, layer 4):** gltf-model, svg, zip, semio, wfc-grid2d (5 crates).
- **Wave 7 (layer 5, one package per plugin family, parallel):** WP-BIM (`bim-model`); WP-ARCH (`architect-program`, c); WP-NOTE, WP-FORMS, WP-WRITER, WP-MATH, WP-VCS, WP-IMPERATIVE, WP-PLAYBOOK, WP-REASONING, WP-SEQUENCE, WP-DEMO (single-plugin packages); WP-FLOW (`flow-flow`); WP-TRINITY (`trinity-rewriting`); WP-SPACE (`space-home`). The stdio members of layer 5 (`stdio-png`, `stdio-gltf-inferences` c, `stdio-gltf-mutations`, `stdio-bcf`) stay in the stdio lane.
- **Wave 8 (layer 6, parallel):** WP-PUZZLE (`puzzle-5d` c, `puzzle-2d`); WP-GIS-MAP (`gis-gismap`); WP-ANIMATE (`animate-presentation`); WP-WFC-2D (`wfc-2d`); WP-SOURCING (`sourcing-curation`); WP-DAG (`dag-dag`); `stdio-tiff`; `stdio-gltf-editor` (c).
- **Wave 9 (layer 7):** WP-SHOOTING (`shooting-shooting`); WP-GIS-TERRAIN (`gis-gisterrain`).

Parallel capacity is set by the stdio lane, which is serial by layer. Waves 0 and 1 are the critical path. Measured closures hold 5,845 taxonomy directories across the 73 measurable crates. A crate per component directory would therefore yield a few thousand small crates, many of them mutation or schema leaves, so grouping rules belong in the scaffolder (S1, S2) rather than in hand carves.

## 7. Caveats and unverified points

- **Plugin gate applicability:** the AUDIT-REFUSED report (section 2) could not confirm that the live plugin catalog build runs the boundary gate. These 79 are over the boundary by measurement, but whether the current build refuses them is unverified. No builds were run.
- **Test closure refusals (S8):** six crates cannot be measured in test cfg. Their `cargo test` status is unknown.
- **Census mismatch:** six crates differ slightly from `plugin-census.csv` on lines or bytes (bim 88,421 against 88,458; puzzle-2d 25,883 against 25,838 lines; puzzle-5d, pdf, space-home by a few units). `gis-gismap` differs on files (190 against 188) and ratio (5.94 against 5.88). The likely cause is the cfg or feature snapshot, since `plugin-census.ts` was not re-run. The table uses my recomputation. No classification changes.
- **Feature sets:** default features only. Feature-unified units may differ.
- **Dependency graph:** path dependencies only. Dev-dependencies are excluded, so test-only producers are not in the layer order.
- **Counter measurements:** the counts use the canonical reader as-is. The `selectRustSourceV1` trailing-cfg defect noted in AUDIT-REFUSED (db closure) was not re-checked here.
