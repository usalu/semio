# 📓️ Native Summary Audit — every plugin crate's native test suite, current measured state

Scope: 163 plugin crates (from `🗑️generated/plugin-crates.json`, 09-21 14:00). Re-verified against
disk (`ls ✏️s/🔌️plugins/*/📦️packages/🦀️rust/Cargo.toml`, `…/🗿️artifacts/*/…`, `…/🧩️extensions/*/…`,
`…/🧱️modules/*/…`): no new top-level plugin/artifact/extension/module crate has appeared since —
the only disk entries the JSON omits are (a) deep `🏅️standards/*/🪆️subsets/*/{🏭️generator,🔬️probes}/*`
sub-crates (reader/codec/engine/oracle-probe leaves inside an artifact, e.g. `png-codec`,
`pdf-1-7-e-lopdf-engine`) and (b) per-plugin `🔮️oracles/📦️packages/🦀️rust` reference-implementation
crates (13 of them, e.g. `semio-s-plugin-stdio-test-oracle`) that a generated test host links by path
and that are never a fleet test target themselves — neither category is a "plugin crate" in the
ticket's sense, so the 163-crate list stands unchanged.

## 1. Totals

**green=100 · red=21 · abort=6 · compile-error=5 · never-run=31**  (measured=132 of 163 plugin crates)

Method: latest-by-log-mtime `test result:` / abort / compile-error line per crate, across every
`*.txt` under `🗑️generated/{raster,design,block-puzzle,knowledge,knowledge-children,media,engineering,
cad-content,stdio-a,stdio-b,stdio-examples,flow,baseline,console-spam,default-example,
registry-projection,play-stdio,xcut-dict}` (310 files scanned; console-spam/default-example/
registry-projection/play-stdio/xcut-dict produced zero native-crate results — they are TS/e2e/registry
topics), plus `🗑️generated/watchdog.txt` for 30-min-timeout kills and `🗑️generated/baseline/summary.tsv`
as a fallback for the one baseline log (`semio-s-plugin-fem.txt`) whose `Running unittests` header was
truncated out of the raw log.

## 2. Per-crate table (sorted by plugin)

| Plugin | Crate | Status | Passed | Failed | Log | Timestamp |
|---|---|---|---|---|---|---|
| ✒️writer | `semio-s-artifact-writer-writer` | ABORT (SIGABRT) — 24 FAILED lines seen before crash, 69 ok | 69 | 24 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| ✒️writer | `semio-s-plugin-writer` | green | 6 | 0 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| ➗️mathematical | `semio-s-artifact-mathematical-equation` | RED (28 failing) | 356 | 28 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| ➗️mathematical | `semio-s-plugin-mathematical` | RED (1 failing) | 3 | 1 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🀄️wfc | `semio-s-artifact-wfc-2d` | COMPILE-ERROR | — | — | `design/run-d4.txt` | 2026-09-22 04:40 |
| 🀄️wfc | `semio-s-artifact-wfc-bitmap` | COMPILE-ERROR | — | — | `design/run-b1.txt` | 2026-09-21 15:38 |
| 🌊️flow | `semio-s-artifact-flow-flow` | green | 254 | 0 | `flow/run5.txt` | 2026-09-21 17:38 |
| 🌊️flow | `semio-s-plugin-flow` | RED (1 failing) | 3 | 1 | `flow/final.txt` | 2026-09-21 16:32 |
| 🌊️flow | `semio-s-plugin-flow-extension-bim` | green | 10 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-brep` | green | 30 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-dictionary` | green | 7 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-draw` | green | 42 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-list` | green | 10 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-logic` | green | 5 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-math` | green | 10 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-primitive` | green | 6 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌊️flow | `semio-s-plugin-flow-extension-text` | green | 5 | 0 | `flow/run3.txt` | 2026-09-21 15:50 |
| 🌍️gis | `semio-s-artifact-gis-gismap` | RED (1 failing) | 262 | 1 | `engineering/first-assembly.txt` | 2026-09-21 15:35 |
| 🌍️gis | `semio-s-artifact-gis-gisterrain` | RED (1 failing) | 91 | 1 | `engineering/retry2-assembly.txt` | 2026-09-21 21:43 |
| 🌍️gis | `semio-s-plugin-gis` | RED (1 failing) | 5 | 1 | `engineering/s6-noassembly.txt` | 2026-09-22 03:43 |
| 🌿️vcs | `semio-s-artifact-vcs-vcs` | green | 121 | 0 | `media/run13.txt` | 2026-09-21 21:00 |
| 🌿️vcs | `semio-s-plugin-vcs` | green | 3 | 0 | `media/run12.txt` | 2026-09-21 20:48 |
| 🎞️animate | `semio-s-artifact-animate-presentation` | ABORT (SIGABRT) — 14 FAILED lines seen before crash, 186 ok | 186 | 14 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| 🎞️animate | `semio-s-plugin-animate` | green | 3 | 0 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| 🎥️shooting | `semio-s-artifact-shooting-shooting` | green | 357 | 0 | `media/run12.txt` | 2026-09-21 20:48 |
| 🎥️shooting | `semio-s-plugin-shooting` | green | 4 | 0 | `media/run12.txt` | 2026-09-21 20:48 |
| 🎪️demonstrator | `semio-s-artifact-demonstrator-playground` | green | 40 | 0 | `media/run12.txt` | 2026-09-21 20:48 |
| 🎪️demonstrator | `semio-s-plugin-demonstrator` | green | 10 | 0 | `media/run12.txt` | 2026-09-21 20:48 |
| 🎬️sequence | `semio-s-artifact-sequence-sequence` | RED (7 failing) | 200 | 7 | `media/run23.txt` | 2026-09-22 04:38 |
| 🎬️sequence | `semio-s-plugin-sequence` | green | 3 | 0 | `media/run23.txt` | 2026-09-22 04:38 |
| 🏗️fem | `semio-s-artifact-fem-2d` | RED (2 failing) | 1259 | 2 | `engineering/retry2-assembly.txt` | 2026-09-21 21:43 |
| 🏗️fem | `semio-s-artifact-fem-3d` | RED (1 failing) | 1130 | 1 | `engineering/first-assembly.txt` | 2026-09-21 15:35 |
| 🏗️fem | `semio-s-plugin-fem` | green | 5 | 0 | `baseline/summary.tsv` | 2026-09-21 14:00 |
| 🏛️architect | `semio-s-artifact-architect-program` | RED (1 failing) | 2084 | 1 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| 🏛️architect | `semio-s-plugin-architect` | RED (1 failing) | 2 | 1 | `knowledge-children/pass2-5.txt` | 2026-09-22 03:54 |
| 🏭️process | `semio-s-artifact-process-process3d` | RED (4 failing) | 355 | 4 | `stdio-b/fmt-process3d.txt` | 2026-09-22 04:44 |
| 🏭️process | `semio-s-plugin-process` | green | 1 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🏭️process | `semio-s-plugin-process-concrete` | green | 5 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🏭️process | `semio-s-plugin-process-metal` | green | 5 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🏭️process | `semio-s-plugin-process-robotic` | green | 5 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🏭️process | `semio-s-plugin-process-wood` | green | 6 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 💠️lowpoly | `semio-s-artifact-lowpoly-lowpoly` | green | 299 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 💠️lowpoly | `semio-s-plugin-lowpoly` | green | 3 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 💡️reasoning | `semio-s-artifact-reasoning-wires` | RED (18 failing) | 169 | 18 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 💡️reasoning | `semio-s-plugin-reasoning` | green | 4 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📋️forms | `semio-s-artifact-forms-forms` | green | 198 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📋️forms | `semio-s-plugin-forms` | green | 1 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📏️layout | `semio-s-artifact-layout-layout` | COMPILE-ERROR | — | — | `design/run-d3.txt` | 2026-09-22 03:49 |
| 📏️layout | `semio-s-plugin-layout` | green | 3 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 📐️cad | `semio-s-artifact-cad-cad` | RED (1 failing) | 429 | 1 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📐️cad | `semio-s-plugin-cad` | green | 5 | 0 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📐️cad | `semio-s-plugin-cad-aec-building` | green | 8 | 0 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📐️cad | `semio-s-plugin-cad-aec-building-energy` | green | 2 | 0 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📐️cad | `semio-s-plugin-cad-aec-building-structure` | green | 2 | 0 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📐️cad | `semio-s-plugin-cad-spatial-shape` | green | 2 | 0 | `cad-content/run15.txt` | 2026-09-22 04:41 |
| 📜️imperative | `semio-s-artifact-imperative-procedure` | RED (19 failing) | 125 | 19 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative` | green | 3 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative-control` | green | 1 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative-effect` | green | 3 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative-logic` | green | 1 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative-math` | green | 1 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📜️imperative | `semio-s-plugin-imperative-text` | green | 4 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 📸️remodel | `semio-s-artifact-remodel-remodeling` | ABORT (SIGKILL) — 8 FAILED lines seen before crash, 349 ok | 349 | 8 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 📸️remodel | `semio-s-plugin-remodel` | green | 3 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 🔋️energy | `semio-s-artifact-energy-model` | green | 6292 | 0 | `engineering/s6-noassembly.txt` | 2026-09-22 03:43 |
| 🔋️energy | `semio-s-plugin-energy` | green | 4 | 0 | `engineering/s6-noassembly.txt` | 2026-09-22 03:43 |
| 🔱️trinity | `semio-s-artifact-trinity-jack` | green | 210 | 0 | `knowledge/batch1.txt` | 2026-09-21 18:08 |
| 🔱️trinity | `semio-s-artifact-trinity-rewriting` | green | 157 | 0 | `knowledge/batch1.txt` | 2026-09-21 18:08 |
| 🔱️trinity | `semio-s-plugin-trinity` | green | 5 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🔱️trinity | `semio-s-plugin-trinity-jack-lsp` | green | 0 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🔱️trinity | `semio-s-plugin-trinity-jack-shell` | green | 1 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🕸️dag | `semio-s-artifact-dag-dag` | RED (9 failing) | 199 | 9 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🕸️dag | `semio-s-plugin-dag` | green | 3 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🖍️draw | `semio-s-artifact-draw-drawing` | ABORT (SIGABRT) — 2 FAILED lines seen before crash, 25 ok | 25 | 2 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 🖍️draw | `semio-s-plugin-draw` | green | 3 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 🖍️draw | `semio-s-plugin-draw-fsm` | green | 26 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 🖍️draw | `semio-s-plugin-draw-fsm-macros` | green | 9 | 0 | `design/run-a1.txt` | 2026-09-21 15:17 |
| 🖨️raster | `semio-s-artifact-raster-raster` | RED (5 failing) | 219 | 5 | `raster/test-26.txt` | 2026-09-22 04:37 |
| 🖨️raster | `semio-s-plugin-raster` | green | 3 | 0 | `raster/test-26.txt` | 2026-09-22 04:37 |
| 🗄️stdio | `semio-s-artifact-stdio-avi` | green | 44 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-bcf` | green | 48 | 0 | `stdio-b/run10.txt` | 2026-09-21 17:23 |
| 🗄️stdio | `semio-s-artifact-stdio-binary` | green | 56 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-bmp` | green | 76 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-contract` | green | 2 | 0 | `stdio-b/fmt-contract.txt` | 2026-09-22 04:41 |
| 🗄️stdio | `semio-s-artifact-stdio-csv` | green | 54 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-deflate` | green | 64 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-docx` | green | 111 | 0 | `stdio-b/run10.txt` | 2026-09-21 17:23 |
| 🗄️stdio | `semio-s-artifact-stdio-dwg` | green | 85 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-dxf` | green | 45 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-epw` | green | 38 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-gif` | green | 116 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-gltf` | green | 269 | 0 | `stdio-b/fmt-stdio.txt` | 2026-09-22 04:43 |
| 🗄️stdio | `semio-s-artifact-stdio-html` | green | 54 | 0 | `stdio-b/run10.txt` | 2026-09-21 17:23 |
| 🗄️stdio | `semio-s-artifact-stdio-ifc` | green | 179 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-jpg` | green | 135 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-json` | green | 119 | 0 | `stdio-examples/test-run2.txt` | 2026-09-21 15:48 |
| 🗄️stdio | `semio-s-artifact-stdio-las` | green | 55 | 0 | `stdio-a/run2.txt` | 2026-09-21 15:15 |
| 🗄️stdio | `semio-s-artifact-stdio-md` | green | 58 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-mp3` | green | 40 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-mp4` | green | 54 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-obj` | green | 53 | 0 | `stdio-b/fmt-stdio.txt` | 2026-09-22 04:43 |
| 🗄️stdio | `semio-s-artifact-stdio-pdf` | green | 558 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-ply` | green | 55 | 0 | `stdio-b/fmt-stdio.txt` | 2026-09-22 04:43 |
| 🗄️stdio | `semio-s-artifact-stdio-png` | green | 150 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-pptx` | green | 115 | 0 | `stdio-b/run10.txt` | 2026-09-21 17:23 |
| 🗄️stdio | `semio-s-artifact-stdio-semio` | RED (1 failing) | 3033 | 1 | `stdio-b/run13.txt` | 2026-09-22 03:58 |
| 🗄️stdio | `semio-s-artifact-stdio-step` | green | 224 | 0 | `stdio-b/fmt-stdio.txt` | 2026-09-22 04:43 |
| 🗄️stdio | `semio-s-artifact-stdio-stl` | green | 53 | 0 | `stdio-b/fmt-stdio.txt` | 2026-09-22 04:43 |
| 🗄️stdio | `semio-s-artifact-stdio-svg` | green | 113 | 0 | `stdio-b/run12-family.txt` | 2026-09-21 21:15 |
| 🗄️stdio | `semio-s-artifact-stdio-tiff` | COMPILE-ERROR | — | — | `raster/test-22.txt` | 2026-09-21 22:12 |
| 🗄️stdio | `semio-s-artifact-stdio-tsv` | green | 41 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-txt` | green | 70 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-wav` | green | 39 | 0 | `stdio-b/run9.txt` | 2026-09-21 17:03 |
| 🗄️stdio | `semio-s-artifact-stdio-xlsx` | green | 105 | 0 | `stdio-b/run10.txt` | 2026-09-21 17:23 |
| 🗄️stdio | `semio-s-artifact-stdio-xml` | green | 97 | 0 | `stdio-b/run12-family.txt` | 2026-09-21 21:15 |
| 🗄️stdio | `semio-s-artifact-stdio-zip` | green | 83 | 0 | `stdio-b/run12-family.txt` | 2026-09-21 21:15 |
| 🗄️stdio | `semio-s-plugin-stdio` | COMPILE-ERROR | — | — | `media/run24.txt` | 2026-09-22 11:12 |
| 🗒️note | `semio-s-artifact-note-note` | RED (32 failing) | 364 | 32 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🗒️note | `semio-s-plugin-note` | green | 4 | 0 | `knowledge/pass1.txt` | 2026-09-22 04:02 |
| 🧩️puzzle | `semio-s-artifact-puzzle-2d` | ABORT (SIGABRT) — 19 FAILED lines seen before crash, 123 ok | 123 | 19 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧩️puzzle | `semio-s-artifact-puzzle-3d` | RED (6 failing) | 741 | 6 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧩️puzzle | `semio-s-artifact-puzzle-5d` | ABORT (SIGKILL) — 27 FAILED lines seen before crash, 554 ok | 554 | 27 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧩️puzzle | `semio-s-plugin-puzzle` | green | 7 | 0 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧱️block | `semio-s-artifact-block-2d` | green | 262 | 0 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧱️block | `semio-s-artifact-block-3d` | RED (2 failing) | 350 | 2 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧱️block | `semio-s-artifact-block-5d` | green | 366 | 0 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🧱️block | `semio-s-plugin-block` | green | 8 | 0 | `block-puzzle/run12.txt` | 2026-09-22 04:35 |
| 🪵️sourcing | `semio-s-artifact-sourcing-curation` | green | 152 | 0 | `engineering/s6-noassembly.txt` | 2026-09-22 03:43 |
| 🪵️sourcing | `semio-s-plugin-sourcing` | green | 3 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🪵️sourcing | `semio-s-plugin-sourcing-beams` | green | 2 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🪵️sourcing | `semio-s-plugin-sourcing-slabs` | green | 2 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🪵️sourcing | `semio-s-plugin-sourcing-windows` | green | 2 | 0 | `engineering/first-noassembly.txt` | 2026-09-21 14:45 |
| 🀄️wfc | `semio-s-artifact-wfc-3d` | never run | — | — | — | — |
| 🀄️wfc | `semio-s-artifact-wfc-grid2d` | never run | — | — | — | — |
| 🀄️wfc | `semio-s-artifact-wfc-grid3d` | never run | — | — | — | — |
| 🀄️wfc | `semio-s-plugin-wfc` | never run | — | — | — | — |
| 🀄️wfc | `semio-s-plugin-wfc-engine` | never run | — | — | — | — |
| 🌀️procedural | `semio-s-artifact-procedural-generation2d` | never run | — | — | — | — |
| 🌀️procedural | `semio-s-artifact-procedural-generation3d` | never run | — | — | — | — |
| 🌀️procedural | `semio-s-plugin-procedural` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-contract` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-din16798` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-din18599` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-din4108` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1990` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1991` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1992` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1993` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1994` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1995` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1996` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1997` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1998` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-en1999` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-iso16757` | never run | — | — | — | — |
| 📕️norm | `semio-s-artifact-norm-vdi3805` | never run | — | — | — | — |
| 📕️norm | `semio-s-plugin-norm` | never run | — | — | — | — |
| 📖️playbook | `semio-s-artifact-playbook-playbook` | never run | — | — | — | — |
| 📖️playbook | `semio-s-plugin-playbook` | never run | — | — | — | — |
| 📖️playbook | `semio-s-plugin-playbook-procedural` | never run | — | — | — | — |
| 🪐️space | `semio-s-artifact-space-home` | never run | — | — | — | — |
| 🪐️space | `semio-s-artifact-space-space` | never run | — | — | — | — |
| 🪐️space | `semio-s-plugin-space` | never run | — | — | — | — |

## 3. Per-topic: what is still red (failing tests / compile errors)

### block-puzzle

- **`semio-s-artifact-puzzle-2d`** (🧩️puzzle) — ABORT (SIGABRT) — 19 FAILED lines seen before crash, 123 ok — `block-puzzle/run12.txt` @ 2026-09-22 04:35
  - FAIL `editor::puzzle2d::component::clipboard_tests::copy_then_paste_round_trips_the_concrete_forest_seed`
  - FAIL `editor::puzzle2d::commands::engagement_submit::tests::engagement_line_carries_its_arguments_verbatim`
  - FAIL `editor::puzzle2d::component::unit_tests::a_node_rotate_board_event_commits_one_rotate_selection_edit`
  - FAIL `editor::puzzle2d::component::unit_tests::add_node_action_emits_upsert_op_and_appends_node`
  - FAIL `editor::puzzle2d::component::unit_tests::apply_board_events_camera_event_commits`
  - FAIL `editor::puzzle2d::component::unit_tests::cohort_hostile_static_law_rejects_one_grant_complex_routes_and_missing_cursors`
  - FAIL `editor::puzzle2d::component::unit_tests::context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last`
  - FAIL `editor::puzzle2d::component::unit_tests::hover_id_reaches_the_board_scene_for_every_granularity_and_pane`
  - FAIL `editor::puzzle2d::component::unit_tests::exact_overview_window_cameras_isolate_render_and_reload_through_registered_app`
  - FAIL `editor::puzzle2d::component::unit_tests::open_hover_accept_places_one_node_on_concrete_forest_and_reselects_it`
  - FAIL `editor::puzzle2d::component::unit_tests::exact_overview_window_transient_isolates_abort_and_resets_on_reload`
  - FAIL `editor::puzzle2d::component::unit_tests::set_active_example_loads_concrete_forest_via_operations`
  - FAIL `editor::puzzle2d::component::unit_tests::set_camera_is_session_only_and_never_undoable`
  - FAIL `editor::puzzle2d::component::unit_tests::set_transform_gumball_flag_composes_the_handles_without_touching_the_document`
  - FAIL `editor::puzzle2d::component::unit_tests::transform_gesture_ticks_coalesce_into_one_undo_step`
  - FAIL `editor::puzzle2d::engine::board_host::unit_tests::board_host_minimap_preselect_matches_selected_chrome`
  - FAIL `editor::puzzle2d::engine::brush::tests::board_fill_candidate_acceptance_exposes_every_retained_field_stage`
  - FAIL `editor::puzzle2d::engine::brush::tests::board_fill_worker_refusal_and_unclaimed_complete_close_exact_owners`
  - FAIL `editor::puzzle2d::engine::brush::tests::board_host_brush_fill_checkpoint_restore_matches_uninterrupted_replay`
- **`semio-s-artifact-puzzle-3d`** (🧩️puzzle) — RED (6 failing) — `block-puzzle/run12.txt` @ 2026-09-22 04:35
  - FAIL `editor::puzzle3d::component::mutation_latency::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`
  - FAIL `editor::puzzle3d::component::unit_tests::a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle`
  - FAIL `editor::puzzle3d::component::unit_tests::the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement`
  - FAIL `editor::puzzle3d::component::unit_tests::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`
  - FAIL `editor::puzzle3d::component::unit_tests::window_options_are_local_to_the_window_instance_not_shared_across_split_panes`
  - FAIL `editor::puzzle3d::precompute::fill::tests::fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin`
- **`semio-s-artifact-puzzle-5d`** (🧩️puzzle) — ABORT (SIGKILL) — 27 FAILED lines seen before crash, 554 ok — `block-puzzle/run12.txt` @ 2026-09-22 04:35
  - FAIL `editor::puzzle5d::component::puzzle5d_retained_retirement_laws::cut_completion_rejection_retains_and_incrementally_closes_exact_cut_mutations`
  - FAIL `editor::puzzle5d::component::puzzle5d_retained_retirement_laws::copy_completion_rejection_retains_and_incrementally_closes_clipboard_ephemeral_and_fault_owners`
  - FAIL `editor::puzzle5d::component::puzzle5d_retained_retirement_laws::import_completion_rejection_never_repages_and_closes_catalog_mutations_incrementally`
  - FAIL `editor::puzzle5d::component::puzzle5d_retained_retirement_laws::paste_completion_rejection_retains_original_flattened_mutation_vector_until_bounded_close`
  - FAIL `editor::puzzle5d::component::unit_tests::add_part_dialog_enumerates_live_part_kinds`
  - FAIL `editor::puzzle5d::component::unit_tests::add_part_kind_materializes_the_declared_kind_default`
  - FAIL `editor::puzzle5d::component::unit_tests::engagement_submit_switches_utility_via_host_effect_for_both_windows`
  - FAIL `editor::puzzle5d::component::unit_tests::engagements_expose_no_utility_switch_options_for_either_window`
  - FAIL `editor::puzzle5d::component::unit_tests::exact_window_cameras_isolate_render_and_reload_without_document_or_app_config_changes`
  - FAIL `editor::puzzle5d::component::unit_tests::exact_window_transient_isolated_abort_and_reload_reset_through_registered_app`
  - FAIL `editor::puzzle5d::component::unit_tests::gumball_translate_drag_coalesces_into_one_edit`
  - FAIL `editor::puzzle5d::component::unit_tests::cut_removes_selected_part_and_undo_restores_it`
  - FAIL `editor::puzzle5d::component::unit_tests::every_context_menu_row_resolves_to_a_live_verb`
  - FAIL `editor::puzzle5d::component::unit_tests::import_stages_every_chunk_and_only_the_closing_one_edits_the_document`
  - FAIL `editor::puzzle5d::component::unit_tests::paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids`
  - FAIL `editor::puzzle5d::component::unit_tests::set_active_example_swaps_the_document_and_undo_restores_it`
  - FAIL `editor::puzzle5d::component::unit_tests::patch_fastener_updates_transform_offsets_and_undoes`
  - FAIL `editor::puzzle5d::component::unit_tests::window_engagements_cover_both_windows`
  - FAIL `editor::puzzle5d::component::unit_tests::window_owner_hostile_static_law_rejects_missing_owner_boundaries_and_app_config_leaks`
  - FAIL `editor::puzzle5d::precompute::fill::tests::aborting_a_fill_run_leaves_the_document_byte_identical`
  - FAIL `editor::puzzle5d::precompute::fill::tests::fill_revalidate_job_translates_provisional_placements_and_their_conflicts`
  - FAIL `editor::puzzle5d::precompute::fill::tests::fill_run_finalize_publishes_one_edit_with_every_provisional_placement`
  - FAIL `editor::puzzle5d::precompute::fill::tests::fill_run_job_matches_the_language_neutral_fill_run_fixture`
  - FAIL `editor::puzzle5d::precompute::fill::tests::fill_run_job_step_stays_below_the_interactive_ceiling_on_the_largest_examples`
  - FAIL `editor::puzzle5d::precompute::fill::tests::lowering_the_fill_count_during_a_run_retracts_the_tail_of_the_same_run`
  - FAIL `editor::puzzle5d::precompute::fill::tests::raising_the_fill_count_during_a_run_reconfigures_the_same_run`
  - FAIL `editor::puzzle5d::component::unit_tests::set_active_example_switches_the_document_and_never_faults_on_capacity`
  - watchdog (30-min) killed a test binary of this crate at: 15:52:23 (pid 96931), 18:41:34 (pid 90083), 21:50:45 (pid 38382), 04:35:52 (pid 17124)
- **`semio-s-artifact-block-3d`** (🧱️block) — RED (2 failing) — `block-puzzle/run12.txt` @ 2026-09-22 04:35
  - FAIL `viewer::block3d::component::tests::noop_command_round_trips_and_never_mutates`
  - FAIL `viewer::block3d::component::tests::viewer_boots_with_at_least_one_representation`

### cad-content

- **`semio-s-artifact-cad-cad`** (📐️cad) — RED (1 failing) — `cad-content/run15.txt` @ 2026-09-22 04:41
  - FAIL `editor::cad::modes::edit::windows::config::tests::cad_document_contract_world_window_runtime_isolates_commands_and_restores_exact_owner`

### design

- **`semio-s-artifact-wfc-2d`** (🀄️wfc) — COMPILE-ERROR — `design/run-d4.txt` @ 2026-09-22 04:40
  - (compile error — see log; no test binary was produced for this measurement)
- **`semio-s-artifact-wfc-bitmap`** (🀄️wfc) — COMPILE-ERROR — `design/run-b1.txt` @ 2026-09-21 15:38
  - (compile error — see log; no test binary was produced for this measurement)
- **`semio-s-artifact-layout-layout`** (📏️layout) — COMPILE-ERROR — `design/run-d3.txt` @ 2026-09-22 03:49
  - (compile error — see log; no test binary was produced for this measurement)
  - watchdog (30-min) killed a test binary of this crate at: 15:07:33 (pid 69343)
- **`semio-s-artifact-remodel-remodeling`** (📸️remodel) — ABORT (SIGKILL) — 8 FAILED lines seen before crash, 349 ok — `design/run-a1.txt` @ 2026-09-21 15:17
  - FAIL `editor::remodeling::component::unit_tests::retained_route_dispositions_are_exact_and_exhaustive`
  - FAIL `editor::remodeling::engine::component::tests::maximum_terminal_raster_png_is_worker_step_bounded`
  - FAIL `editor::remodeling::engine::images::tests::maximum_admitted_png_scanline_stays_below_the_worker_ceiling`
  - FAIL `editor::remodeling::engine::images::tests::production_png_decoder_worker_steps_are_scanline_bounded`
  - FAIL `editor::remodeling::engine::mesh::tests::accepted_texture_bake_and_png_publication_steps_stay_below_hard_ceiling`
  - FAIL `editor::remodeling::engine::images::tests::accepted_worst_envelope_jpeg_and_malformed_entropy_steps_are_timed`
  - FAIL `editor::remodeling::engine::mesh::tests::accepted_tsdf_extraction_and_envelope_rejection_steps_stay_below_hard_ceiling`
  - FAIL `editor::remodeling::engine::reconstruction::tests::adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded`
  - watchdog (30-min) killed a test binary of this crate at: 15:17:39 (pid 81349)
- **`semio-s-artifact-draw-drawing`** (🖍️draw) — ABORT (SIGABRT) — 2 FAILED lines seen before crash, 25 ok — `design/run-a1.txt` @ 2026-09-21 15:17
  - FAIL `editor::drawing::component::archive_load_tests::demo_example_load_settles_through_the_host_document_archive_door`
  - FAIL `editor::drawing::component::unit_tests::add_layer_undo_round_trip_through_wrapper`

### engineering

- **`semio-s-artifact-gis-gismap`** (🌍️gis) — RED (1 failing) — `engineering/first-assembly.txt` @ 2026-09-21 15:35
  - FAIL `editor::gis2d::component::unit_tests::gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`
- **`semio-s-artifact-gis-gisterrain`** (🌍️gis) — RED (1 failing) — `engineering/retry2-assembly.txt` @ 2026-09-21 21:43
  - FAIL `editor::gis3d::modes::view::windows::terrain::config::mutations::set_camera::tests::direct_leaf_neutral_schema_codec_and_outcome_laws`
- **`semio-s-plugin-gis`** (🌍️gis) — RED (1 failing) — `engineering/s6-noassembly.txt` @ 2026-09-22 03:43
  - FAIL `descriptor_is_fresh`
- **`semio-s-artifact-fem-2d`** (🏗️fem) — RED (2 failing) — `engineering/retry2-assembly.txt` @ 2026-09-21 21:43
  - FAIL `analyses::tests::assembly_job_one_fuel_steps_stay_below_eight_milliseconds`
  - FAIL `mesh::tests::mesh_job_large_boundary_never_runs_to_completion_in_one_step`
- **`semio-s-artifact-fem-3d`** (🏗️fem) — RED (1 failing) — `engineering/first-assembly.txt` @ 2026-09-21 15:35
  - FAIL `editor::fem3d::commands::result_animation_tick::tests::long::result_animation_frame_cost_stays_flat_across_a_long_run`

### flow

- **`semio-s-plugin-flow`** (🌊️flow) — RED (1 failing) — `flow/final.txt` @ 2026-09-21 16:32
  - FAIL `plugin::surface_tests::flow_actual_surface_factories_close_all_owners_under_neutral_grants`

### knowledge

- **`semio-s-artifact-mathematical-equation`** (➗️mathematical) — RED (28 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `cas::integrate::tests::integrate_simple_partial_fraction`
  - FAIL `cas::limits::tests::limit_at_infinity_of_rational_function`
  - FAIL `cas::ode::tests::bernoulli_ode`
  - FAIL `cas::ode::tests::linear_first_order_ode`
  - FAIL `cas::sums::tests::fourier_coefficients_of_a_polynomial_smoke_test`
  - FAIL `cas::sums::tests::sum_of_k_from_1_to_n_is_gauss_formula`
  - FAIL `cas::sums::tests::sum_of_k_squared_matches_known_hand_values`
  - FAIL `editor::equation::commands::set_algorithm::tests::ingest_operations_is_idempotent_for_equation`
  - FAIL `editor::equation::commands::set_algorithm::tests::set_algorithm_updates_graph_and_seed`
  - FAIL `editor::equation::commands::set_algorithm::tests::set_directed_toggles_the_graph`
  - FAIL `editor::equation::commands::set_algorithm::tests::two_instances_converge_disjoint_edits_via_backbone`
  - FAIL `editor::equation::commands::set_artifact::tests::set_artifact_replaces_graph_and_geometry`
  - FAIL `editor::equation::modes::edit::windows::graph::config::mutation_vectors::language_neutral_mutations_match_json_oracle_and_restore_base`
  - FAIL `editor::equation::modes::edit::windows::graph::config::window_config_ownership::equation_graph_window_config_retained_publications_isolate_and_reload_two_windows`
  - FAIL `polynomial::algebraic::tests::cbrt2_times_cbrt4_equals_2`
  - FAIL `polynomial::algebraic::tests::neg_and_inv_hand_cases`
  - FAIL `polynomial::algebraic::tests::root_of_selects_correct_irreducible_factor`
  - FAIL `polynomial::algebraic::tests::sqrt2_plus_sqrt3_has_minimal_poly_degree_4`
  - FAIL `polynomial::finite::tests::is_irreducible_hand_cases`
  - FAIL `polynomial::univariate::tests::interpolate_reconstructs_quadratic`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::math_document_text_round_trips_through_store`
  - FAIL `standards::v1::subsets::any::io::snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::connect_then_disconnect_nodes_round_trips`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::delete_node_inverse_recreates_node_and_severed_edges`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::delete_nodes_plural_cascades_like_the_singular_form`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::insert_point_inverse_is_remove_point_at_same_index`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::move_point_inverse_restores_old_position`
  - FAIL `viewer::equation::modes::view::windows::geometry::tests::render_produces_a_table_scene_with_one_row_per_point`
- **`semio-s-plugin-mathematical`** (➗️mathematical) — RED (1 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `plugin::surface_tests::equation_viewer_instantiates_through_new_viewer`
- **`semio-s-artifact-reasoning-wires`** (💡️reasoning) — RED (18 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `editor::wires::commands::add_relationship::tests::add_relationship_appends_edge_and_selects`
  - FAIL `editor::wires::commands::canvas_pointer_down::tests::pointer_down_on_empty_space_requests_no_select_effect`
  - FAIL `editor::wires::commands::canvas_pointer_down::tests::pointer_drag_translates_node_by_screen_delta`
  - FAIL `editor::wires::commands::delete_selection::tests::handle_alone_deletes_nothing_without_a_live_selection`
  - FAIL `editor::wires::commands::set_active_example::tests::set_active_example_metabolism_loads_seven_nodes`
  - FAIL `editor::wires::commands::set_active_example::tests::set_active_example_unknown_id_loads_empty_document`
  - FAIL `editor::wires::component::window_transient::tests::wires_pointer_move_uses_only_the_captured_canvas_and_publishes_document_positions`
  - FAIL `editor::wires::component::window_transient::tests::wires_window_transient_retained_pointer_lifecycle_is_partitioned`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::document_text_round_trip_with_operation_applied`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::op_binary_round_trips_and_agrees_with_text`
  - FAIL `standards::v1::subsets::any::schema::mutations::change_node_shape::tests_reports_a_no_op_when_the_shape_already_reads_circle::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::change_node_shape::tests_reports_a_no_op_when_the_shape_already_reads_circle::produces_committed_diff`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::op_text_round_trip_create_node`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::set_node_root_round_trip`
  - FAIL `standards::v1::subsets::any::schema::mutations::move_node::tests_reports_a_no_op_when_a_y_less_node_is_moved_to_y_zero::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::resize_node::tests_reports_a_no_op_when_the_radius_already_matches::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::resize_node::tests_reports_a_no_op_when_the_radius_already_matches::committed_json_is_canonical`
- **`semio-s-artifact-imperative-procedure`** (📜️imperative) — RED (19 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `editor::procedure::component::unit_tests::every_declared_action_is_carried_by_a_window_kind`
  - FAIL `editor::procedure::component::unit_tests::two_instances_converge_disjoint_edits_via_backbone`
  - FAIL `editor::procedure::component::unit_tests::undo_after_add_step_restores_original_document_exactly`
  - FAIL `editor::procedure::engine::tests::host_runs_default_snapshot`
  - FAIL `editor::procedure::modes::edit::windows::main::tests::run_command_expands_scope_into_readable_rows_without_truncation`
  - FAIL `editor::procedure::panels::inspection::semantic_contract::imperative_semantic_panels_match_the_json_oracle`
  - FAIL `standards::v1::subsets::any::schema::mutations::binary::tests::document_text_round_trip_with_applied_operation`
  - FAIL `standards::v1::subsets::any::schema::mutations::binary::tests::op_text_round_trips_edit_step_params`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::structural_correspondence_tests::direct_owners_descriptors_surfaces_and_catalog_correspond`
  - FAIL `standards::v1::subsets::any::schema::mutations::create_step::tests_rejects_a_duplicate_step_id_at_the_root_path::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::declared_outcome_holds`
  - FAIL `standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::produces_committed_diff`
  - FAIL `standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::the_idempotent_edit_carries_before_to_an_identical_after`
  - FAIL `standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::the_inverse_resends_the_identical_dictionary`
  - FAIL `standards::v1::subsets::any::schema::mutations::reorder_steps::tests_warns_that_an_over_clamped_index_leaves_the_tail_step_in_place::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::operations::tests::delete_step_inverse_law`
  - FAIL `standards::v1::subsets::any::schema::operations::tests::edit_step_params_inverse_law`
  - FAIL `viewer::procedure::modes::view::windows::main::tests::columns_resolve_from_the_shared_view_locale`
- **`semio-s-artifact-dag-dag`** (🕸️dag) — RED (9 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `editor::dag::commands::add_node::tests::remove_node_deletes_node_and_connected_edges`
  - FAIL `editor::dag::commands::node_graph_edit::tests::node_graph_edit_batches_connect_then_delete_selection`
  - FAIL `editor::dag::component::unit_tests::every_declared_action_is_registered`
  - FAIL `standards::v1::subsets::any::io::snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
  - FAIL `standards::v1::subsets::any::schema::mutations::component::tests::connect_disconnect_nodes_inverse_law`
  - FAIL `standards::v1::subsets::any::schema::mutations::create_node::tests_rejects_a_duplicate_node_id::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::move_node::tests_rejects_moving_a_missing_node::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::replace_node_kind::tests_rejects_rekinding_a_missing_node::committed_json_is_canonical`
  - FAIL `standards::v1::subsets::any::schema::mutations::resize_node::tests_rejects_resizing_a_missing_node::committed_json_is_canonical`
- **`semio-s-artifact-note-note`** (🗒️note) — RED (32 failing) — `knowledge/pass1.txt` @ 2026-09-22 04:02
  - FAIL `editor::note::commands::add_block::tests::add_block_action_emits_one_op_and_grows_projection`
  - FAIL `editor::note::commands::ink_apply_events::tests::gesture_begin_live_commit_produces_single_undo_step`
  - FAIL `editor::note::commands::set_camera::tests::camera_drag_never_creates_a_document_undo_step`
  - FAIL `editor::note::commands::set_camera::tests::set_camera_writes_config_and_emits_no_artifact_mutations`
  - FAIL `editor::note::commands::set_camera::tests::set_camera_zoom_updates_zoom_and_keeps_pan_via_config`
  - FAIL `editor::note::component::unit_tests::utility_registry_declares_canvas_utilities_scoped_to_composite_window`
  - FAIL `editor::note::modes::edit::windows::composite::component::tests::renders_composite_canvas`
  - FAIL `editor::note::modes::edit::windows::navigator::component::tests::renders_navigator_canvas`
  - FAIL `editor::note::panels::document::tests::renders_document_tree`
  - FAIL `editor::note::panels::inspection::semantic_contract::note_semantic_panels_match_the_json_oracle`
  - FAIL `editor::note::panels::inspection::tests::renders_the_document_wide_summary`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::note_document_text_round_trips_store_with_applied_operation`
  - FAIL `standards::v1::subsets::any::io::snapshot::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
  - FAIL `standards::v1::subsets::block::schema::mutations::create_block::tests_inserts_a_photo_block_at_root_index_2::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::block::schema::mutations::create_block::tests_inserts_a_photo_block_at_root_index_2::produces_committed_diff`
  - FAIL `standards::v1::subsets::block::schema::mutations::duplicate_block::tests_copies_the_math_block_right_after_its_source::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::block::schema::mutations::duplicate_block::tests_copies_the_math_block_right_after_its_source::produces_committed_diff`
  - FAIL `standards::v1::subsets::block::schema::mutations::duplicate_blocks::tests_copies_ink_and_table_with_shifting_indices::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::block::schema::mutations::duplicate_blocks::tests_copies_ink_and_table_with_shifting_indices::produces_committed_diff`
  - FAIL `standards::v1::subsets::block::schema::mutations::move_block_to_container::tests_reparents_ink_into_the_callout_group::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::block::schema::mutations::move_block_to_container::tests_reparents_ink_into_the_callout_group::produces_committed_diff`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::tests_widens_grid_spacing::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::tests_widens_grid_spacing::produces_committed_diff`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::tests_doubles_grid_subdivisions::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::tests_doubles_grid_subdivisions::produces_committed_diff`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::tests_halves_snap_grid_spacing::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::tests_halves_snap_grid_spacing::produces_committed_diff`
  - FAIL `standards::v1::subsets::ink::schema::mutations::change_eraser_radius::tests_enlarges_eraser::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::ink::schema::mutations::change_eraser_radius::tests_enlarges_eraser::produces_committed_diff`
  - FAIL `standards::v1::subsets::ink::schema::mutations::change_pencil_width::tests_thickens_pencil::committed_diff_is_canonical`
  - FAIL `standards::v1::subsets::ink::schema::mutations::change_pencil_width::tests_thickens_pencil::produces_committed_diff`

### knowledge-children

- **`semio-s-artifact-writer-writer`** (✒️writer) — ABORT (SIGABRT) — 24 FAILED lines seen before crash, 69 ok — `knowledge-children/pass2-5.txt` @ 2026-09-22 03:54
  - FAIL `editor::writer::commands::engagement_submit::tests::engagement_submit_parses_font_size`
  - FAIL `editor::writer::commands::engagement_submit::tests::engagement_submit_parses_separatorless_drafts`
  - FAIL `editor::writer::commands::set_camera::tests::set_camera_command_writes_config_not_operations`
  - FAIL `editor::writer::commands::lint_document::tests::lint_is_a_view_action_and_example_default_materializes`
  - FAIL `editor::writer::commands::text_edit::tests::set_active_example_falls_back_to_empty_document`
  - FAIL `editor::writer::commands::text_edit::tests::commit_rename_renames_all_spans_at_the_config_selection`
  - FAIL `editor::writer::commands::text_edit::tests::format_artifact_reformats_jack_query`
  - FAIL `editor::writer::commands::text_edit::tests::format_document_without_change_emits_no_operation`
  - FAIL `editor::writer::commands::text_edit::tests::set_active_example_loads_dag_jack_fixture`
  - FAIL `editor::writer::commands::text_edit::tests::set_active_example_loads_jack_fixture`
  - FAIL `editor::writer::commands::text_edit::tests::set_text_action_updates_projection`
  - FAIL `editor::writer::component::unit_tests::command_surface_has_the_expected_row_count_and_distinct_wire_keywords`
  - FAIL `editor::writer::component::unit_tests::writer_artifact_store_preparation_is_exact_bounded_and_reversible`
  - FAIL `editor::writer::component::unit_tests::writer_labels_resolve_native_english_by_default_across_every_surface`
  - FAIL `editor::writer::component::unit_tests::writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed`
  - FAIL `editor::writer::commands::text_edit::tests::set_text_undo_redo_round_trips_through_the_wrapper`
  - FAIL `editor::writer::commands::text_edit::tests::text_edit_burst_coalesces_into_one_undo_step`
  - FAIL `editor::writer::commands::toggle_line_numbers::tests::view_action_emits_no_operations`
  - FAIL `editor::writer::modes::edit::windows::main::component::tests::scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack`
  - FAIL `editor::writer::panels::document::tests::a_closed_ast_level_stamps_its_total_and_builds_no_children`
  - FAIL `editor::writer::panels::document::tests::a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_ast_id`
  - FAIL `editor::writer::panels::document::tests::document_lists_the_ast_section_for_jack_documents`
  - FAIL `editor::writer::panels::inspection::tests::writer_labels_resolve_native_by_default`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::command_envelope_round_trip_holds_for_an_applied_operation`
- **`semio-s-artifact-animate-presentation`** (🎞️animate) — ABORT (SIGABRT) — 14 FAILED lines seen before crash, 186 ok — `knowledge-children/pass2-5.txt` @ 2026-09-22 03:54
  - FAIL `editor::animate::commands::add_tile::tests::app_manifest_declares_expected_operations`
  - FAIL `editor::animate::commands::copy_prompt::tests::export_video_from_deck_reports_no_scene_hashes_as_download_error`
  - FAIL `editor::animate::commands::delete_selection::tests::delete_selection_with_no_selection_is_a_no_op`
  - FAIL `editor::animate::component::unit_tests::app_manifest_declares_expected_operations_and_shell_actions`
  - FAIL `editor::animate::component::unit_tests::import_media_frames_in_inserts_a_new_tile`
  - FAIL `editor::animate::component::unit_tests::import_media_frames_in_places_repeated_imports_in_distinct_cells`
  - FAIL `editor::animate::panels::catalogue::tests::catalogue_lists_templates`
  - FAIL `editor::animate::panels::inspection::semantic_contract::presentation_semantic_panels_match_the_json_oracle`
  - FAIL `editor::animate::panels::artifact::tests::document_lists_seeded_tiles`
  - FAIL `editor::animate::panels::inspection::tests::details_panel_reports_schema_and_tile_count`
  - FAIL `editor::animate::terminology::tests::animate_presentation_labels_resolve_native_by_default`
  - FAIL `editor::animate::terminology::tests::animate_presentation_labels_translate_panels_in_german`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::document_text_round_trip_with_operation_applied`
  - FAIL `standards::v1::subsets::any::io::mutations::binary::tests::presentation_deck_materializes`
- **`semio-s-artifact-architect-program`** (🏛️architect) — RED (1 failing) — `knowledge-children/pass2-5.txt` @ 2026-09-22 03:54
  - FAIL `editor::architect::component::unit_tests::interaction_select_stamps_the_picked_element_as_selected_in_the_document_panel`
- **`semio-s-plugin-architect`** (🏛️architect) — RED (1 failing) — `knowledge-children/pass2-5.txt` @ 2026-09-22 03:54
  - FAIL `descriptor_is_fresh`

### media

- **`semio-s-artifact-sequence-sequence`** (🎬️sequence) — RED (7 failing) — `media/run23.txt` @ 2026-09-22 04:38
  - FAIL `editor::sequence::component::unit_tests::import_media_steps_in_inserts_a_new_step_from_an_object_payload`
  - FAIL `editor::sequence::component::unit_tests::import_media_steps_in_wraps_a_bare_scalar_payload`
  - FAIL `editor::sequence::component::unit_tests::repeated_drops_after_replace_snapshot_use_distinct_ids`
  - FAIL `editor::sequence::component::unit_tests::replace_snapshot_preserves_next_serial_and_selection`
  - FAIL `editor::sequence::component::unit_tests::set_step_params_json_updates_step_params`
  - FAIL `editor::sequence::modes::edit::windows::main::config::tests::sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows`
  - FAIL `standards::v1::subsets::any::schema::operations::tests::store_applies_and_undoes_step_create`
- **`semio-s-plugin-stdio`** (🗄️stdio) — COMPILE-ERROR — `media/run24.txt` @ 2026-09-22 11:12
  - (compile error — see log; no test binary was produced for this measurement)

### raster

- **`semio-s-artifact-raster-raster`** (🖨️raster) — RED (5 failing) — `raster/test-26.txt` @ 2026-09-22 04:37
  - FAIL `editor::raster::component::unit_tests::composite_scene_syncs_document_and_assets`
  - FAIL `editor::raster::component::unit_tests::raster_import_media_appends_layer_from_incoming_image`
  - FAIL `standards::v1::subsets::any::schema::mutations::binary::unit_tests::raster_standalone_control_max_plus_one_returns_exact_owner_and_resumes_after_full_saturation`
  - FAIL `viewer::raster::modes::view::windows::composite::component::tests::render_produces_a_scene_node_for_the_default_document`
  - FAIL `viewer::raster::modes::view::windows::navigator::component::tests::render_produces_a_scene_node_for_the_default_document`
  - watchdog (30-min) killed a test binary of this crate at: 14:30:20 (pid 21767), 15:58:55 (pid 5946)
- **`semio-s-artifact-stdio-tiff`** (🗄️stdio) — COMPILE-ERROR — `raster/test-22.txt` @ 2026-09-21 22:12
  - (compile error — see log; no test binary was produced for this measurement)

### stdio-b

- **`semio-s-artifact-process-process3d`** (🏭️process) — RED (4 failing) — `stdio-b/fmt-process3d.txt` @ 2026-09-22 04:44
  - FAIL `editor::process3d::component::unit_tests::vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed`
  - FAIL `editor::process3d::wasm::mounted_registry::mounted_laws::authoritative_publication_rejects_stale_generation_aba_and_parent`
  - FAIL `standards::v1::subsets::any::schema::mutations::binary::retained_laws::actual_atomic_publication_is_fail_closed_and_retires_stale_candidate`
  - FAIL `standards::v1::subsets::any::schema::mutations::binary::retained_laws::every_store_replacement_phase_unit_fits_the_interactive_step_budget`
- **`semio-s-artifact-stdio-semio`** (🗄️stdio) — RED (1 failing) — `stdio-b/run13.txt` @ 2026-09-22 03:58
  - FAIL `tests::returned_read_leases_retire_before_the_displaced_owners_that_alias_them`

### Compile-error detail (headline error per crate)

- `semio-s-artifact-wfc-bitmap` (design/run-b1.txt): `error[E0599]: no method named `is_empty` found for struct `LocalizedLabel``
- `semio-s-artifact-wfc-2d` (design/run-d4.txt): `error[E0425]: cannot find type `Wfc2dRetainedCommandJobFactory` in module `crate::editor::wfc2d``
- `semio-s-artifact-layout-layout` (design/run-d3.txt): `error[E0277]: the trait bound `LayoutApp: PluginApp` is not satisfied`
- `semio-s-artifact-stdio-tiff` (raster/test-22.txt): `error[E0463]: can't find crate for `semio_framework_io_base64`` (chain of `can't find crate` — looks like a broken/incomplete private target dir in that specific run, not a source defect; no later attempt at this crate exists in the logs)
- `semio-s-plugin-stdio` (media/run24.txt, **09-22 11:13 — the newest log in the whole ticket, i.e. current**): `error[E0425]: cannot find function `native_codec_factory_receipts_for` in this scope` + `error[E0061]: this function takes 2 arguments but 1 argument was supplied`

Note: `semio-s-artifact-wfc-3d`, `semio-s-artifact-wfc-grid2d`, `semio-s-artifact-wfc-grid3d`,
`semio-s-plugin-wfc`, `semio-s-plugin-wfc-engine` are **not** compile-errors — the two design runs that
touched them (`run-d3.txt`, `run-d4.txt`) both died on an *earlier* `-p` crate's compile error
(`layout` / `wfc-2d`) before cargo ever reached these crates' own test binaries (no `--keep-going`,
confirmed by `Compiling` lines for these crates present but no matching `Running unittests`/`error: could
not compile` for them specifically) — they are counted as **never run**, not compile-error.

## 4. Never-run crates, grouped by plugin

All 31 never-run crates belong to the four plugins the fleet briefs mark peer-owned (S10 /
`End-to-end repo completion`): norm, space, procedural, playbook — plus every wfc crate that a design
run's compile error blocked from ever reaching its own test binary (see note above; wfc is *not* in the
brief's peer-owned list, so this is a fleet gap, not an ownership exclusion).

### 🀄️wfc (5) — NOT peer-owned — blocked by an unrelated compile error (layout in run-d3, wfc-2d in run-d4) earlier in the same `-p` list before cargo reached these crates' own test binaries; needs a dedicated re-run once layout/wfc-2d/wfc-bitmap compile

- `semio-s-artifact-wfc-3d`
- `semio-s-artifact-wfc-grid2d`
- `semio-s-artifact-wfc-grid3d`
- `semio-s-plugin-wfc`
- `semio-s-plugin-wfc-engine`

### 🌀️procedural (3) — peer-owned (brief v4/v5; generation2d guest code specifically called out as peer-owned)

- `semio-s-artifact-procedural-generation2d`
- `semio-s-artifact-procedural-generation3d`
- `semio-s-plugin-procedural`

### 📕️norm (17) — peer-owned (brief v4/v5: norm excluded from the fleet's describe/test passes)

- `semio-s-artifact-norm-contract`
- `semio-s-artifact-norm-din16798`
- `semio-s-artifact-norm-din18599`
- `semio-s-artifact-norm-din4108`
- `semio-s-artifact-norm-en1990`
- `semio-s-artifact-norm-en1991`
- `semio-s-artifact-norm-en1992`
- `semio-s-artifact-norm-en1993`
- `semio-s-artifact-norm-en1994`
- `semio-s-artifact-norm-en1995`
- `semio-s-artifact-norm-en1996`
- `semio-s-artifact-norm-en1997`
- `semio-s-artifact-norm-en1998`
- `semio-s-artifact-norm-en1999`
- `semio-s-artifact-norm-iso16757`
- `semio-s-artifact-norm-vdi3805`
- `semio-s-plugin-norm`

### 📖️playbook (3) — peer-owned (brief v4/v5: "playbook = peer S10")

- `semio-s-artifact-playbook-playbook`
- `semio-s-plugin-playbook`
- `semio-s-plugin-playbook-procedural`

### 🪐️space (3) — peer-owned (brief v4/v5)

- `semio-s-artifact-space-home`
- `semio-s-artifact-space-space`
- `semio-s-plugin-space`

---

Raw TSV: `🗑️generated/audit-native-summary/summary.tsv`. Files scanned: 310.

_Audit generated 2026-09-22 11:15 by audit-native-summary (read-only, no cargo/wasm run)._
