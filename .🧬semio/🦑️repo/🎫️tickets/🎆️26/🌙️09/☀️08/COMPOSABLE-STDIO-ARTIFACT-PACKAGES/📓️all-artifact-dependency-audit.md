# All Registered Artifact Dependency Audit

This snapshot uses full offline Cargo resolved metadata after the external consumer manifest cutover. It follows normal dependency edges, excluding test/build edges, across all currently selected workspace features. Source compilation remains a separate gate.

96 artifact/contract crates are resolved. 3 closures still include plugin crates. This audit is intentionally run during extraction and does not claim the end state is complete.

| Artifact Package | Normal Closure | Stdio Crates | Plugin Backedges |
| --- | ---: | ---: | --- |
| semio-framework-artifact-workflow-run | 171 | 0 |  |
| semio-framework-artifact-workflow-workflow | 396 | 0 |  |
| semio-s-artifact-animate-presentation | 729 | 34 |  |
| semio-s-artifact-architect-program | 441 | 35 |  |
| semio-s-artifact-block-2d | 1 | 0 |  |
| semio-s-artifact-block-3d | 1 | 0 |  |
| semio-s-artifact-block-5d | 1 | 0 |  |
| semio-s-artifact-cad-cad | 439 | 33 |  |
| semio-s-artifact-dag-dag | 480 | 33 |  |
| semio-s-artifact-demonstrator-playground | 410 | 9 |  |
| semio-s-artifact-draw-drawing | 443 | 33 | semio-s-plugin-draw-fsm, semio-s-plugin-draw-fsm-macros |
| semio-s-artifact-energy-model | 441 | 35 |  |
| semio-s-artifact-fem-2d | 1 | 0 |  |
| semio-s-artifact-fem-3d | 1 | 0 |  |
| semio-s-artifact-flow-flow | 484 | 33 |  |
| semio-s-artifact-forms-forms | 484 | 33 |  |
| semio-s-artifact-gis-gismap | 1 | 0 |  |
| semio-s-artifact-gis-gisterrain | 1 | 0 |  |
| semio-s-artifact-imperative-procedure | 441 | 33 |  |
| semio-s-artifact-layout-layout | 610 | 33 |  |
| semio-s-artifact-lowpoly-lowpoly | 440 | 33 |  |
| semio-s-artifact-mathematical-equation | 439 | 33 |  |
| semio-s-artifact-norm-contract | 401 | 0 |  |
| semio-s-artifact-norm-din16798 | 402 | 0 |  |
| semio-s-artifact-norm-din18599 | 442 | 33 |  |
| semio-s-artifact-norm-din4108 | 402 | 0 |  |
| semio-s-artifact-norm-en1990 | 440 | 33 |  |
| semio-s-artifact-norm-en1991 | 441 | 33 |  |
| semio-s-artifact-norm-en1992 | 402 | 0 |  |
| semio-s-artifact-norm-en1993 | 403 | 0 |  |
| semio-s-artifact-norm-en1994 | 404 | 0 |  |
| semio-s-artifact-norm-en1995 | 444 | 33 |  |
| semio-s-artifact-norm-en1996 | 445 | 33 |  |
| semio-s-artifact-norm-en1997 | 446 | 33 |  |
| semio-s-artifact-norm-en1998 | 447 | 33 |  |
| semio-s-artifact-norm-en1999 | 448 | 33 |  |
| semio-s-artifact-norm-iso16757 | 449 | 33 |  |
| semio-s-artifact-norm-vdi3805 | 402 | 0 |  |
| semio-s-artifact-note-note | 590 | 33 |  |
| semio-s-artifact-playbook-playbook | 484 | 33 |  |
| semio-s-artifact-procedural-assembly | 1 | 0 |  |
| semio-s-artifact-procedural-generation2d | 1 | 0 |  |
| semio-s-artifact-procedural-generation3d | 1 | 0 |  |
| semio-s-artifact-process-process3d | 439 | 33 |  |
| semio-s-artifact-puzzle-2d | 1 | 0 |  |
| semio-s-artifact-puzzle-3d | 1 | 0 |  |
| semio-s-artifact-puzzle-5d | 1 | 0 |  |
| semio-s-artifact-raster-raster | 590 | 33 |  |
| semio-s-artifact-reasoning-wires | 480 | 33 |  |
| semio-s-artifact-remodel-remodeling | 591 | 33 |  |
| semio-s-artifact-sequence-sequence | 482 | 33 |  |
| semio-s-artifact-shooting-shooting | 590 | 33 |  |
| semio-s-artifact-sourcing-curation | 439 | 33 |  |
| semio-s-artifact-space-home | 1 | 0 |  |
| semio-s-artifact-space-space | 1 | 0 |  |
| semio-s-artifact-stdio-avi | 402 | 2 |  |
| semio-s-artifact-stdio-bcf | 407 | 7 |  |
| semio-s-artifact-stdio-binary | 402 | 2 |  |
| semio-s-artifact-stdio-bmp | 403 | 3 |  |
| semio-s-artifact-stdio-contract | 401 | 1 |  |
| semio-s-artifact-stdio-csv | 404 | 4 |  |
| semio-s-artifact-stdio-deflate | 403 | 3 |  |
| semio-s-artifact-stdio-docx | 407 | 7 |  |
| semio-s-artifact-stdio-dwg | 403 | 3 |  |
| semio-s-artifact-stdio-dxf | 404 | 4 |  |
| semio-s-artifact-stdio-epw | 402 | 2 |  |
| semio-s-artifact-stdio-gif | 403 | 3 |  |
| semio-s-artifact-stdio-gltf | 405 | 5 |  |
| semio-s-artifact-stdio-html | 402 | 2 |  |
| semio-s-artifact-stdio-ifc | 405 | 5 |  |
| semio-s-artifact-stdio-jpg | 403 | 3 |  |
| semio-s-artifact-stdio-json | 404 | 4 |  |
| semio-s-artifact-stdio-las | 403 | 3 |  |
| semio-s-artifact-stdio-md | 404 | 4 |  |
| semio-s-artifact-stdio-mp3 | 402 | 2 |  |
| semio-s-artifact-stdio-mp4 | 402 | 2 |  |
| semio-s-artifact-stdio-obj | 404 | 4 |  |
| semio-s-artifact-stdio-pdf | 404 | 4 |  |
| semio-s-artifact-stdio-ply | 404 | 4 |  |
| semio-s-artifact-stdio-png | 407 | 7 |  |
| semio-s-artifact-stdio-pptx | 407 | 7 |  |
| semio-s-artifact-stdio-semio | 438 | 33 |  |
| semio-s-artifact-stdio-step | 404 | 4 |  |
| semio-s-artifact-stdio-stl | 404 | 4 |  |
| semio-s-artifact-stdio-svg | 405 | 5 |  |
| semio-s-artifact-stdio-tiff | 403 | 3 |  |
| semio-s-artifact-stdio-tsv | 402 | 2 |  |
| semio-s-artifact-stdio-txt | 403 | 3 |  |
| semio-s-artifact-stdio-wav | 402 | 2 |  |
| semio-s-artifact-stdio-xlsx | 407 | 7 |  |
| semio-s-artifact-stdio-xml | 404 | 4 |  |
| semio-s-artifact-stdio-zip | 406 | 6 |  |
| semio-s-artifact-trinity-jack | 1 | 0 |  |
| semio-s-artifact-trinity-rewriting | 1 | 0 |  |
| semio-s-artifact-vcs-vcs | 444 | 37 | semio-s-plugin-stdio |
| semio-s-artifact-writer-writer | 482 | 33 | semio-s-plugin-trinity |

## Framework Normal-Dependency Followup

Cargo workspace feature union, runtime edges only (dependency tests are excluded). This metadata audit is not compilation or a package-selected feature proof.

- `semio-framework-artifact-flow-flow`: 446 packages; composition dependencies [].
- `semio-framework-artifact-infinite-dag`: 176 packages; composition dependencies [].
- `semio-framework-artifact-playbook-playbook`: 398 packages; composition dependencies [].
- `semio-framework-artifact-space-collection`: 171 packages; composition dependencies [].
- `semio-framework-artifact-space-space`: 171 packages; composition dependencies [].
- `semio-framework-artifact-workflow-run`: 171 packages; composition dependencies [].
- `semio-framework-artifact-workflow-workflow`: 398 packages; composition dependencies [].
