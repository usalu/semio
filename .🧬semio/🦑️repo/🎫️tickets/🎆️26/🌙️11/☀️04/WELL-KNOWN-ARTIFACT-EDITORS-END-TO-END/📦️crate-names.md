# 📦️ Cargo Package Names: Well-Known Stdio Artifacts, Hub, OS and Energy

Generated from `Cargo.toml` `[package] name` only (TOML read; no cargo run). Index: `🗑️generated/crate-names-index.tsv`; renderer: `crate-names-render.py` (both in this ticket). Owner is the nearest ancestor `Cargo.toml` with `[workspace]` that lists the crate (ancestors are checked nearest first, so the repo-root workspace can own OS crates). `cargo -p <name>` only resolves from that owner directory (or with `--manifest-path`); the root workspace excludes artifact-level workspaces such as `🗿️artifacts/🎒️zip` and `🗿️artifacts/🗽️obj`, so the root-level command fails for those crates.

Path abbreviations: `stdio/` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, `stdio-root/` = `✏️s/🔌️plugins/🗄️stdio/`, `hub/` = `🌎️hub/🧩️compositions/🗄️stdio/`, `energy/` = `✏️s/🔌️plugins/🔋️energy/`, `os/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`, `rust` = `📦️packages/🦀️rust`. The energy section lists only the model artifact (`energy/🗿️artifacts/🔋️model`); other energy plugin crates are omitted.

Member = `yes` when the owning workspace lists the crate (and does not exclude it). `no` means `cargo -p` from that owner will not find it; an owner-less crate runs only with `--manifest-path`.

## Corrections to the fleet names (verified by TOML and workspace read)

- `semio-s-artifact-stdio-zip-editor` EXISTS (`stdio/🎒️zip/✏️editor/📦️packages/🦀️rust`). It is a member of the zip artifact's own workspace (`stdio/🎒️zip/Cargo.toml`), which the root workspace excludes. Run it from that workspace, not from the repo root: `cd <stdio/🎒️zip> && cargo test -p semio-s-artifact-stdio-zip-editor`. The earlier `did not match any packages` error came from the root-level selection. [TOML and workspace read]
- There is no `semio-s-artifact-stdio-obj-editor` package. The obj root is `semio-s-artifact-stdio-obj` (`stdio/🗽️obj/📦️packages/🦀️rust`), owned by the obj artifact workspace (`stdio/🗽️obj/Cargo.toml`), which the root workspace excludes. The obj subset `✏️editor` directory has no Cargo.toml, so no editor package exists for obj. The earlier `e-obj` run (`cargo test -p semio-s-artifact-stdio-obj editor::` from the repo root) failed for the same workspace reason: the name exists, but the root selection does not see it. Re-queue from the obj artifact workspace.
- `natural-file-lifecycle` fixture: JSON at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json`. The Rust test is `🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs`, mounted by the companion crate `semio-framework-plugin-tests-07` via `#[path]`. The OS plugin tests crates listed under `os-plugin-tests` are other suites.
- wgpu Shell relay test `🔀️wgpu-document-relay/🦀️.rs` is mounted by the companion crate `semio-framework-os-renderer-wgpu-shell-tests-docs-2` (`🥪️shell-tests-docs-2`, `#[path]` in its lib `🥪️shell-tests-docs-2/🦀️.rs`). The Shell directory itself has no Cargo.toml.
- Energy model editor: `semio-s-artifact-energy-model-editor` (`energy/🗿️artifacts/🔋️model/.../✏️editor/📦️packages/🦀️rust`), owned by the energy model artifact workspace.
- OS io: `semio-framework-os-io` (`os/🚪️io/📦️packages/🦀️rust`), owned by the repo-root workspace. OS semio: `semio-framework-os-semio` (`os/🧬️semio/📦️packages/🦀️rust`), owned by the repo-root workspace.

## Summary by artifact (stdio)

Columns: root crate(s), editor crate(s), io / snapshot / mutations crate(s), test crate(s). `-` means none in the index.

| Artifact | Root | Editor | IO / snapshot / mutations | Tests |
|---|---|---|---|---|
| ☁️las | `semio-s-artifact-stdio-las` | - | `semio-s-artifact-stdio-las-mutations`, `semio-s-artifact-stdio-las-snapshot` | `las-1-0-any-fixture-generator`, `semio-s-artifact-stdio-las-tests`, `semio-s-artifact-stdio-las-tests-2` |
| ⚡️idf | `semio-s-artifact-stdio-idf` | - | - | - |
| 🌐️html | `semio-s-artifact-stdio-html` | - | `semio-s-artifact-stdio-html-io-registry`, `semio-s-artifact-stdio-html-mutations`, `semio-s-artifact-stdio-html-snapshot` | - |
| 🌦️epw | `semio-s-artifact-stdio-epw` | `semio-s-artifact-stdio-epw-editor` | `semio-s-artifact-stdio-epw-io-registry`, `semio-s-artifact-stdio-epw-mutations`, `semio-s-artifact-stdio-epw-snapshot` | `semio-s-artifact-stdio-epw-tests` |
| 🎒️zip | `semio-s-artifact-stdio-zip` | `semio-s-artifact-stdio-zip-editor` | `semio-s-artifact-stdio-zip-mutations` | `semio-s-artifact-stdio-zip-tests` |
| 🎞️gif | `semio-s-artifact-stdio-gif` | `semio-s-artifact-stdio-gif-editor` | `semio-s-artifact-stdio-gif-mutations`, `semio-s-artifact-stdio-gif-mutations-2`, `semio-s-artifact-stdio-gif-snapshot` | `gif-89a-any-fixture-generator`, `semio-s-artifact-stdio-gif-fixture-tests` |
| 🎥️mp4 | `semio-s-artifact-stdio-mp4` | - | `semio-s-artifact-stdio-mp4-boxes`, `semio-s-artifact-stdio-mp4-mutations`, `semio-s-artifact-stdio-mp4-text` | `semio-s-artifact-stdio-mp4-tests` |
| 🎨️svg | `semio-s-artifact-stdio-svg` | - | `semio-s-artifact-stdio-svg-mut-family`, `semio-s-artifact-stdio-svg-mutations`, `semio-s-artifact-stdio-svg-mutations-2`, `semio-s-artifact-stdio-svg-snapshot` | `semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests`, `semio-s-artifact-stdio-svg-tests` |
| 🎵️mp3 | `semio-s-artifact-stdio-mp3` | - | `semio-s-artifact-stdio-mp3-io-metadata`, `semio-s-artifact-stdio-mp3-metadata`, `semio-s-artifact-stdio-mp3-mutations` | `semio-s-artifact-stdio-mp3-editor` |
| 🏗️ifc | `semio-s-artifact-stdio-ifc` | - | `semio-s-artifact-stdio-ifc-2x3-base-details`, `semio-s-artifact-stdio-ifc-2x3-base-mutations`, `semio-s-artifact-stdio-ifc-4-mutations`, `semio-s-artifact-stdio-ifc4x3-exchange`, `semio-s-artifact-stdio-ifc4x3-fem`, `semio-s-artifact-stdio-ifc4x3-step` | - |
| 💬️bcf | `semio-s-artifact-stdio-bcf` | `semio-s-artifact-stdio-bcf-io-registry` | `semio-s-artifact-stdio-bcf-mutations` | - |
| 💾️binary | `semio-s-artifact-stdio-binary` | - | `semio-s-artifact-stdio-binary-mutations`, `semio-s-artifact-stdio-binary-snapshot` | - |
| 📊️csv | `semio-s-artifact-stdio-csv` | `semio-s-artifact-stdio-csv-editor`, `semio-s-artifact-stdio-csv-io-registry` | `semio-s-artifact-stdio-csv-mutations`, `semio-s-artifact-stdio-csv-snapshot` | - |
| 📐️step | `semio-s-artifact-stdio-step` | `semio-s-artifact-stdio-step-base-editor`, `semio-s-artifact-stdio-step-cc1-editor`, `semio-s-artifact-stdio-step-cc2-editor`, `semio-s-artifact-stdio-step-cc3-editor`, `semio-s-artifact-stdio-step-cc4-editor`, `semio-s-artifact-stdio-step-cc5-editor`, `semio-s-artifact-stdio-step-cc6-editor` | `semio-s-artifact-stdio-step-geometry`, `semio-s-artifact-stdio-step-ladder`, `semio-s-artifact-stdio-step-mutations`, `semio-s-artifact-stdio-step-snapshot` | - |
| 📑️tsv | `semio-s-artifact-stdio-tsv` | - | `semio-s-artifact-stdio-tsv-io-registry`, `semio-s-artifact-stdio-tsv-mutations`, `semio-s-artifact-stdio-tsv-snapshot` | - |
| 📕️xlsx | `semio-s-artifact-stdio-xlsx` | - | `semio-s-artifact-stdio-xlsx-base-mutations`, `semio-s-artifact-stdio-xlsx-strict-mutations` | - |
| 📖️pdf | `semio-s-artifact-stdio-pdf` | - | `semio-s-artifact-stdio-pdf-model`, `semio-s-artifact-stdio-pdf-model-profiles`, `semio-s-artifact-stdio-pdf-mut-base-3`, `semio-s-artifact-stdio-pdf-mut-base-5`, `semio-s-artifact-stdio-pdf-mut-base-6`, `semio-s-artifact-stdio-pdf-mut-base-7`, `semio-s-artifact-stdio-pdf-mut-family-2`, `semio-s-artifact-stdio-pdf-mut-family-3`, `semio-s-artifact-stdio-pdf-mut-family-4`, `semio-s-artifact-stdio-pdf-mut-family-6`, `semio-s-artifact-stdio-pdf-mut-family-7`, `semio-s-artifact-stdio-pdf-mut-vt`, `semio-s-artifact-stdio-pdf-mut-x`, `semio-s-artifact-stdio-pdf-mutations`, `semio-s-artifact-stdio-pdf-mutations-2`, `semio-s-artifact-stdio-pdf-snapshot`, `semio-s-artifact-stdio-pdf-snapshot-2`, `semio-s-artifact-stdio-pdf-snapshot-surfaces` | `pdf-1-7-base-fixture-generator`, `semio-s-artifact-stdio-pdf-mut-base-8`, `semio-s-artifact-stdio-pdf-mut-base-9`, `semio-s-artifact-stdio-pdf-mut-family-10`, `semio-s-artifact-stdio-pdf-mut-family-11`, `semio-s-artifact-stdio-pdf-mut-family-12`, `semio-s-artifact-stdio-pdf-mut-family-13`, `semio-s-artifact-stdio-pdf-mut-family-14`, `semio-s-artifact-stdio-pdf-mut-family-8`, `semio-s-artifact-stdio-pdf-mut-family-9`, `semio-s-artifact-stdio-pdf-mut-x-2`, `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`, `semio-s-artifact-stdio-pdf-tests`, `semio-s-artifact-stdio-pdf-tests-2` |
| 📚️idml | `semio-s-artifact-stdio-idml` | - | - | `semio-s-artifact-stdio-idml-tests` |
| 📜️docx | `semio-s-artifact-stdio-docx` | - | `semio-s-artifact-stdio-docx-base-edit-rules`, `semio-s-artifact-stdio-docx-base-mutations`, `semio-s-artifact-stdio-docx-base-xml-address`, `semio-s-artifact-stdio-docx-strict-mutations` | - |
| 📝️md | `semio-s-artifact-stdio-md` | - | `semio-s-artifact-stdio-md-mutations`, `semio-s-artifact-stdio-md-snapshot` | `semio-s-artifact-stdio-md-tests` |
| 📰️xml | `semio-s-artifact-stdio-xml` | - | `semio-s-artifact-stdio-xml-mutations`, `semio-s-artifact-stdio-xml-snapshot` | `semio-s-artifact-stdio-xml-tests`, `semio-s-artifact-stdio-xml-tests-2`, `semio-s-artifact-stdio-xml-tests-3` |
| 📷️png | `semio-s-artifact-stdio-png` | `semio-s-artifact-stdio-png-editor` | `semio-s-artifact-stdio-png-mutations`, `semio-s-artifact-stdio-png-mutations-facets`, `semio-s-artifact-stdio-png-mutations-io`, `semio-s-artifact-stdio-png-mutations-schema`, `semio-s-artifact-stdio-png-snapshot` | `semio-s-artifact-stdio-png-tests` |
| 📸️jpg | `semio-s-artifact-stdio-jpg` | `semio-s-artifact-stdio-jpg-io-registry` | `semio-s-artifact-stdio-jpg-mutations` | - |
| 📼️avi | `semio-s-artifact-stdio-avi` | - | `semio-s-artifact-stdio-avi-mutations`, `semio-s-artifact-stdio-avi-snapshot` | `semio-s-artifact-stdio-avi-codec-tests`, `semio-s-artifact-stdio-avi-tests` |
| 📽️pptx | `semio-s-artifact-stdio-pptx` | - | `semio-s-artifact-stdio-pptx-base-mutations`, `semio-s-artifact-stdio-pptx-strict-mutations` | - |
| 🔊️wav | `semio-s-artifact-stdio-wav` | `semio-s-artifact-stdio-wav-io-registry` | `semio-s-artifact-stdio-wav-mutations` | `semio-s-artifact-stdio-wav-codec-tests` |
| 🔤️txt | `semio-s-artifact-stdio-txt` | - | `semio-s-artifact-stdio-txt-mutations` | `semio-s-artifact-stdio-txt-tests` |
| 🔺️stl | `semio-s-artifact-stdio-stl` | - | `semio-s-artifact-stdio-stl-mutations` | - |
| 🖊️dwg | `semio-s-artifact-stdio-dwg` | `semio-s-artifact-stdio-dwg-editor`, `semio-s-artifact-stdio-dwg-editor-ac1018` | `semio-s-artifact-stdio-dwg-mutations`, `semio-s-artifact-stdio-dwg-read`, `semio-s-artifact-stdio-dwg-read-objects`, `semio-s-artifact-stdio-dwg-read-objects-dynamic-blocks`, `semio-s-artifact-stdio-dwg-read-records`, `semio-s-artifact-stdio-dwg-snapshot`, `semio-s-artifact-stdio-dwg-snapshot-document`, `semio-s-artifact-stdio-dwg-snapshot-drawing`, `semio-s-artifact-stdio-dwg-snapshot-objects`, `semio-s-artifact-stdio-dwg-sqlite-drawing`, `semio-s-artifact-stdio-dwg-sqlite-drawing-2`, `semio-s-artifact-stdio-dwg-sqlite-entities`, `semio-s-artifact-stdio-dwg-sqlite-objects`, `semio-s-artifact-stdio-dwg-write`, `semio-s-artifact-stdio-dwg-write-objects`, `semio-s-artifact-stdio-dwg-write-records` | - |
| 🖋️dxf | `semio-s-artifact-stdio-dxf` | - | `semio-s-artifact-stdio-dxf-mutations`, `semio-s-artifact-stdio-dxf-snapshot` | `dxf-r12-any-fixture-generator`, `semio-s-artifact-stdio-dxf-tests` |
| 🖼️tiff | `semio-s-artifact-stdio-tiff` | - | `semio-s-artifact-stdio-tiff-mutations`, `semio-s-artifact-stdio-tiff-snapshot`, `semio-s-artifact-stdio-tiff-snapshot-surfaces` | `semio-s-artifact-stdio-tiff-mutation-regressions` |
| 🗜️deflate | `semio-s-artifact-stdio-deflate` | - | `semio-s-artifact-stdio-deflate-mutations`, `semio-s-artifact-stdio-deflate-snapshot`, `semio-s-artifact-stdio-deflate-zlib` | `semio-s-artifact-stdio-deflate-tests` |
| 🗽️obj | `semio-s-artifact-stdio-obj` | - | `semio-s-artifact-stdio-obj-mutations` | `obj-3-0-any-fixture-generator` |
| 🧊️gltf | `semio-s-artifact-stdio-gltf` | `semio-s-artifact-stdio-gltf-editor` | `semio-s-artifact-stdio-gltf-inferences-inferences`, `semio-s-artifact-stdio-gltf-model-schema`, `semio-s-artifact-stdio-gltf-model-snapshot`, `semio-s-artifact-stdio-gltf-model-snapshot-root`, `semio-s-artifact-stdio-gltf-model-wire`, `semio-s-artifact-stdio-gltf-model-wire-2`, `semio-s-artifact-stdio-gltf-mutations`, `semio-s-artifact-stdio-gltf-mutations-add-required-extension`, `semio-s-artifact-stdio-gltf-mutations-mut-any`, `semio-s-artifact-stdio-gltf-mutations-mut-any-2`, `semio-s-artifact-stdio-gltf-mutations-mut-any-3`, `semio-s-artifact-stdio-gltf-mutations-mutations` | `semio-s-artifact-stdio-gltf-inferences-projection-tests`, `semio-s-artifact-stdio-gltf-mutations-mut-any-10`, `semio-s-artifact-stdio-gltf-mutations-mut-any-4`, `semio-s-artifact-stdio-gltf-mutations-mut-any-5`, `semio-s-artifact-stdio-gltf-mutations-mut-any-6`, `semio-s-artifact-stdio-gltf-mutations-mut-any-7`, `semio-s-artifact-stdio-gltf-mutations-mut-any-8`, `semio-s-artifact-stdio-gltf-mutations-mut-any-9` |
| 🧱️ply | `semio-s-artifact-stdio-ply` | - | `semio-s-artifact-stdio-ply-mutations` | `semio-s-artifact-stdio-ply-inferences-tests`, `semio-s-artifact-stdio-ply-io-registry` |
| 🧾️json | `semio-s-artifact-stdio-json` | - | `semio-s-artifact-stdio-json-mutations`, `semio-s-artifact-stdio-json-mutations-2`, `semio-s-artifact-stdio-json-number` | `semio-s-artifact-stdio-json-tests` |
| 🧿️semio | `semio-s-artifact-stdio-semio` | `semio-s-artifact-stdio-semio-animation-editor`, `semio-s-artifact-stdio-semio-cad-editor`, `semio-s-artifact-stdio-semio-document-editor`, `semio-s-artifact-stdio-semio-drawing-editor`, `semio-s-artifact-stdio-semio-editor`, `semio-s-artifact-stdio-semio-graph-editor`, `semio-s-artifact-stdio-semio-image-editor`, `semio-s-artifact-stdio-semio-kit-editor`, `semio-s-artifact-stdio-semio-mesh-editor`, `semio-s-artifact-stdio-semio-object-editor` | `semio-s-artifact-stdio-semio-animation-export`, `semio-s-artifact-stdio-semio-animation-import`, `semio-s-artifact-stdio-semio-animation-schema`, `semio-s-artifact-stdio-semio-animation-schema-3`, `semio-s-artifact-stdio-semio-animation-schema-4`, `semio-s-artifact-stdio-semio-audio-export`, `semio-s-artifact-stdio-semio-audio-import`, `semio-s-artifact-stdio-semio-audio-mut-family`, `semio-s-artifact-stdio-semio-audio-mutations`, `semio-s-artifact-stdio-semio-brep-export`, `semio-s-artifact-stdio-semio-brep-import`, `semio-s-artifact-stdio-semio-brep-io`, `semio-s-artifact-stdio-semio-brep-mut-family`, `semio-s-artifact-stdio-semio-brep-snapshot`, `semio-s-artifact-stdio-semio-cad-export`, `semio-s-artifact-stdio-semio-cad-import`, `semio-s-artifact-stdio-semio-cad-io`, `semio-s-artifact-stdio-semio-cad-mut-family`, `semio-s-artifact-stdio-semio-cad-snapshot`, `semio-s-artifact-stdio-semio-document-export`, `semio-s-artifact-stdio-semio-document-import`, `semio-s-artifact-stdio-semio-document-io`, `semio-s-artifact-stdio-semio-document-mut-family`, `semio-s-artifact-stdio-semio-document-mutations`, `semio-s-artifact-stdio-semio-document-snapshot`, `semio-s-artifact-stdio-semio-drawing-binary`, `semio-s-artifact-stdio-semio-drawing-export`, `semio-s-artifact-stdio-semio-drawing-import`, `semio-s-artifact-stdio-semio-drawing-io`, `semio-s-artifact-stdio-semio-drawing-mut-family`, `semio-s-artifact-stdio-semio-drawing-mut-family-2`, `semio-s-artifact-stdio-semio-drawing-png-export`, `semio-s-artifact-stdio-semio-drawing-snapshot`, `semio-s-artifact-stdio-semio-flow-export`, `semio-s-artifact-stdio-semio-flow-import`, `semio-s-artifact-stdio-semio-flow-io`, `semio-s-artifact-stdio-semio-flow-mut-family`, `semio-s-artifact-stdio-semio-flow-mutations`, `semio-s-artifact-stdio-semio-flow-set-node-position`, `semio-s-artifact-stdio-semio-flow-snapshot`, `semio-s-artifact-stdio-semio-graph-mut-family`, `semio-s-artifact-stdio-semio-graph-snapshot`, `semio-s-artifact-stdio-semio-image-export`, `semio-s-artifact-stdio-semio-image-import`, `semio-s-artifact-stdio-semio-image-io`, `semio-s-artifact-stdio-semio-image-mut-family`, `semio-s-artifact-stdio-semio-image-snapshot`, `semio-s-artifact-stdio-semio-kit-mut-family`, `semio-s-artifact-stdio-semio-kit-mutations`, `semio-s-artifact-stdio-semio-kit-snapshot`, `semio-s-artifact-stdio-semio-mesh-export`, `semio-s-artifact-stdio-semio-mesh-import`, `semio-s-artifact-stdio-semio-mesh-io`, `semio-s-artifact-stdio-semio-mesh-mut-family`, `semio-s-artifact-stdio-semio-mesh-mutations`, `semio-s-artifact-stdio-semio-mesh-snapshot`, `semio-s-artifact-stdio-semio-model-export`, `semio-s-artifact-stdio-semio-model-import`, `semio-s-artifact-stdio-semio-model-io`, `semio-s-artifact-stdio-semio-model-mut-family`, `semio-s-artifact-stdio-semio-model-mutations`, `semio-s-artifact-stdio-semio-model-snapshot`, `semio-s-artifact-stdio-semio-mut-family`, `semio-s-artifact-stdio-semio-mut-family-2`, `semio-s-artifact-stdio-semio-object-mut-family`, `semio-s-artifact-stdio-semio-object-snapshot`, `semio-s-artifact-stdio-semio-presentation-export`, `semio-s-artifact-stdio-semio-presentation-import`, `semio-s-artifact-stdio-semio-presentation-io`, `semio-s-artifact-stdio-semio-presentation-mut-family`, `semio-s-artifact-stdio-semio-presentation-mutations`, `semio-s-artifact-stdio-semio-presentation-snapshot`, `semio-s-artifact-stdio-semio-snapshot`, `semio-s-artifact-stdio-semio-table-mut-family`, `semio-s-artifact-stdio-semio-table-mutations`, `semio-s-artifact-stdio-semio-text-mutations`, `semio-s-artifact-stdio-semio-value-export`, `semio-s-artifact-stdio-semio-value-import`, `semio-s-artifact-stdio-semio-value-mutation-leaves`, `semio-s-artifact-stdio-semio-value-mutations`, `semio-s-artifact-stdio-semio-video-export`, `semio-s-artifact-stdio-semio-video-import`, `semio-s-artifact-stdio-semio-video-mutations` | `semio-s-artifact-stdio-semio-document-examples-unit-tests`, `semio-s-artifact-stdio-semio-graph-mut-fixture-tests`, `semio-s-artifact-stdio-semio-inverts-tests`, `semio-s-artifact-stdio-semio-sqlite-laws-1`, `semio-s-artifact-stdio-semio-sqlite-laws-2`, `semio-s-artifact-stdio-semio-sqlite-laws-3`, `semio-s-artifact-stdio-semio-sqlite-laws-4`, `semio-s-artifact-stdio-semio-sqlite-laws-5` |
| 🪟️bmp | `semio-s-artifact-stdio-bmp` | `semio-s-artifact-stdio-bmp-editor`, `semio-s-artifact-stdio-bmp-editor-2` | `semio-s-artifact-stdio-bmp-mutations`, `semio-s-artifact-stdio-bmp-snapshot` | `semio-s-artifact-stdio-bmp-mutation-regressions` |

## energy

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-energy-model` | `energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-commands` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-config` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-edit` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-inspector` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️inspector/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-interaction` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-panels` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| editor | `semio-s-artifact-energy-model-editor-results` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📊️results/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| io | `semio-s-artifact-energy-model-export` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| io | `semio-s-artifact-energy-model-import` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| io | `semio-s-artifact-energy-model-io` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| snapshot | `semio-s-artifact-energy-model-snapshot` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| snapshot | `semio-s-artifact-energy-model-snapshot-bridge` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/📦️packages/🦀️rust` | `(none)` | no |
| snapshot | `semio-s-artifact-energy-model-snapshot-sqlite-admission` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/📏️cells/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-air-loop` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-air-loop-nodes` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️air-loop/🔀️nodes/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-battery` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪫️battery/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-boundary` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️boundary/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-construction` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️construction/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-day-schedule` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒️day-schedule/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-daylight-zone` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞️daylight-zone/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-enclosure` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️enclosure/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-equipment-gain` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖥️equipment-gain/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-fault` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️fault/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-fenestration` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-fenestration-geometry` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟️fenestration/📐️geometry/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-finish` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️finish/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-fixed-schedule` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕐️fixed-schedule/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-glazing` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️glazing/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-humidistat` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️humidistat/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-ideal-loads` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-ideal-loads-limits` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💯️ideal-loads/🚧️limits/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-infiltration` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️infiltration/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-leakage` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️leakage/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-lighting-gain` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-lighting-gain-emission` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💡️lighting-gain/🔆️emission/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-load-center` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️load-center/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-material` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️material/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-model` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-model-simulation` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔋️model/🧮️simulation/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-outdoor-air` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃️outdoor-air/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-people-gain` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-people-gain-fractions` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️people-gain/🍰️fractions/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-plant-loop` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-plant-loop-design` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️plant-loop/🧭️design/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-pv-system` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-pv-system-performance` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡️pv-system-performance/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-refrigeration` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️refrigeration/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-room-air` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️room-air/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-run` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏯️run/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-setpoint` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️setpoint/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-shading` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳️shading/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-shading-device` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛱️shading-device/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-shw-system` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️shw-system/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-site` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-site-ground` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site/🌋️ground/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-sizing` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️sizing/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-solar-thermal` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥️solar-thermal/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-space` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️space/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-space-list` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️space-list/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-surface` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-surface-properties` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️surface/🛠️properties/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-thermostat` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️thermostat/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-time-series` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️time-series/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-ventilation` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌪️ventilation/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-water-system` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️water-system/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-week-schedule` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️week-schedule/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-window-layer` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-window-layer-optics` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️window-layer-optics/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-window-layer-optics-visible` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️window-layer-optics/🔭️visible/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-year-schedule` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-year-schedule-holidays` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📆️year-schedule/🏖️holidays/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-zone` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏡️zone/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mut-zone-equipment` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️zone-equipment/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-mutations` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-native-derive` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️derive/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-native-mutations` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-native-mutations-lifecycle` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native-lifecycle/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| mutations | `semio-s-artifact-energy-model-native-mutations-singletons` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native-singletons/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| schema | `semio-s-artifact-energy-model-diff` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| schema | `semio-s-artifact-energy-model-diff-patch` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patch/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| schema | `semio-s-artifact-energy-model-diff-splice` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/✂️splice/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| schema | `semio-s-artifact-energy-model-inferences` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| schema | `semio-s-artifact-energy-model-schema` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| test | `semio-s-artifact-energy-model-editor-mounted-tests` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mounted-bestest/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| test | `semio-s-artifact-energy-model-mut-membership-tests` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️membership/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| test | `semio-s-artifact-energy-model-mutation-fixtures` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixtures/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| test | `semio-s-artifact-energy-model-snapshot-tests` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| other | `semio-s-artifact-energy-model-examples` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| other | `semio-s-artifact-energy-model-initialization` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🌱️initialization/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| other | `semio-s-artifact-energy-model-session` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |
| other | `semio-s-artifact-energy-model-viewer` | `energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust` | `energy/🗿️artifacts/🔋️model` | yes |

## hub-stdio

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-hub-stdio` | `hub/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-bim` | `hub/🧩️extensions/🏠️bim/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-binary` | `hub/🧩️extensions/🔢️binary/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-cad` | `hub/🧩️extensions/🛠️cad/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-catalog` | `hub/🔌️plugin/📇️catalog/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-component` | `hub/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-image` | `hub/🧩️extensions/🖼️image/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-media` | `hub/🧩️extensions/🎵️media/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-mesh` | `hub/🧩️extensions/🔺️mesh/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-mutation-bridge` | `hub/🏭️bridge/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-office` | `hub/🧩️extensions/💼️office/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-pdf` | `hub/🧩️extensions/📘️pdf/📦️packages/🦀️rust` | `🌎️hub` | yes |
| root | `semio-hub-stdio-semio` | `hub/🧩️extensions/🧿️semio/📦️packages/🦀️rust` | `🌎️hub` | yes |
| oracle | `semio-hub-stdio-test-oracle` | `hub/🔮️oracles/📦️packages/🦀️rust` | `hub/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-hub-stdio-bim-component` | `hub/🧩️extensions/🏠️bim/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-binary-component` | `hub/🧩️extensions/🔢️binary/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-cad-component` | `hub/🧩️extensions/🛠️cad/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-catalog-definition` | `hub/🔌️plugin/📇️catalog/📜️definition/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-catalog-projection` | `hub/🔌️plugin/📇️catalog/🏗️projection/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-image-component` | `hub/🧩️extensions/🖼️image/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-media-component` | `hub/🧩️extensions/🎵️media/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-mesh-component` | `hub/🧩️extensions/🔺️mesh/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-office-component` | `hub/🧩️extensions/💼️office/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-pdf-component` | `hub/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |
| other | `semio-hub-stdio-semio-component` | `hub/🧩️extensions/🧿️semio/🧩️component/📦️packages/🦀️rust` | `🌎️hub` | yes |

## os-io

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| io | `semio-framework-os-io` | `os/🚪️io/📦️packages/🦀️rust` | `.` | yes |

## os-plugin-tests

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| test | `semio-framework-plugin-child-group-wire-laws` | `os/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/⚖️laws/📦️packages/🦀️rust` | `.` | yes |
| test | `semio-framework-plugin-command-page-laws` | `os/🔌️plugin/🧪️tests/📑️command-page/📦️packages/🦀️rust` | `🧰️framework` | yes |
| test | `semio-framework-plugin-surface-carrier-laws` | `os/🔌️plugin/🧪️tests/🪟️surface-carriers/📦️packages/🦀️rust` | `.` | yes |
| test | `semio-framework-plugin-surface-fixture` | `os/🔌️plugin/🧪️tests/🪟️surface-fixture/📦️packages/🦀️rust` | `.` | yes |
| test | `semio-framework-plugin-testing-acceptance` | `os/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/📦️packages/🦀️rust` | `.` | yes |

## os-semio

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-framework-os-semio` | `os/🧬️semio/📦️packages/🦀️rust` | `.` | yes |

## os-tests

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| test | `semio-framework-plugin-tests-07` | `os/🔌️plugin/🧷️companions/🍇️tests-07/📦️packages/🦀️rust` | `.` | yes |

## wgpu-shell

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| test | `semio-framework-os-renderer-wgpu-shell-tests-docs-2` | `os/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🥪️shell-tests-docs-2/📦️packages/🦀️rust` | `.` | yes |

## ☁️las

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-las` | `stdio/☁️las/📦️packages/🦀️rust` | `stdio/☁️las` | yes |
| snapshot | `semio-s-artifact-stdio-las-snapshot` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/☁️las` | yes |
| mutations | `semio-s-artifact-stdio-las-mutations` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/☁️las` | yes |
| schema | `semio-s-artifact-stdio-las-schema` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📦️packages/🦀️rust` | `stdio/☁️las` | yes |
| oracle | `semio-s-artifact-stdio-las-test-oracle` | `stdio/☁️las/🔮️oracles/📦️packages/🦀️rust` | `stdio/☁️las/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `las-1-0-any-fixture-generator` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-las-tests` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/☁️las` | yes |
| test | `semio-s-artifact-stdio-las-tests-2` | `stdio/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/☁️las` | yes |

## ⚡️idf

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-idf` | `stdio/⚡️idf/📦️packages/🦀️rust` | `stdio/⚡️idf` | yes |

## 🌐️html

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-html` | `stdio/🌐️html/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| io | `semio-s-artifact-stdio-html-io-registry` | `stdio/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| snapshot | `semio-s-artifact-stdio-html-snapshot` | `stdio/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| mutations | `semio-s-artifact-stdio-html-mutations` | `stdio/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| schema | `semio-s-artifact-stdio-html-inferences` | `stdio/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| schema | `semio-s-artifact-stdio-html-schema` | `stdio/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🌐️html` | yes |
| oracle | `semio-s-artifact-stdio-html-test-oracle` | `stdio/🌐️html/🔮️oracles/📦️packages/🦀️rust` | `stdio/🌐️html/🔮️oracles/📦️packages/🦀️rust` | yes |

## 🌦️epw

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-epw` | `stdio/🌦️epw/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| editor | `semio-s-artifact-stdio-epw-editor` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| io | `semio-s-artifact-stdio-epw-io-registry` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| snapshot | `semio-s-artifact-stdio-epw-snapshot` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| mutations | `semio-s-artifact-stdio-epw-mutations` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| schema | `semio-s-artifact-stdio-epw-schema` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |
| oracle | `semio-s-artifact-stdio-epw-test-oracle` | `stdio/🌦️epw/🔮️oracles/📦️packages/🦀️rust` | `stdio/🌦️epw/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-epw-tests` | `stdio/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🌦️epw` | yes |

## 🎒️zip

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-zip` | `stdio/🎒️zip/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| editor | `semio-s-artifact-stdio-zip-editor` | `stdio/🎒️zip/✏️editor/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| mutations | `semio-s-artifact-stdio-zip-mutations` | `stdio/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| schema | `semio-s-artifact-stdio-zip-inferences` | `stdio/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| schema | `semio-s-artifact-stdio-zip-schema` | `stdio/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| schema | `semio-s-artifact-stdio-zip-schema-2` | `stdio/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| oracle | `semio-s-artifact-stdio-zip-test-oracle` | `stdio/🎒️zip/🔮️oracles/📦️packages/🦀️rust` | `stdio/🎒️zip/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-zip-tests` | `stdio/🎒️zip/📦️opc/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |
| other | `semio-s-artifact-stdio-zip-opc` | `stdio/🎒️zip/📦️opc/📦️packages/🦀️rust` | `stdio/🎒️zip` | yes |

## 🎞️gif

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-gif` | `stdio/🎞️gif/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| editor | `semio-s-artifact-stdio-gif-editor` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| snapshot | `semio-s-artifact-stdio-gif-snapshot` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| mutations | `semio-s-artifact-stdio-gif-mutations` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| mutations | `semio-s-artifact-stdio-gif-mutations-2` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| schema | `semio-s-artifact-stdio-gif-inferences` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| schema | `semio-s-artifact-stdio-gif-schema` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| schema | `semio-s-artifact-stdio-gif-schema-2` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| oracle | `semio-s-artifact-stdio-gif-test-oracle` | `stdio/🎞️gif/🔮️oracles/📦️packages/🦀️rust` | `stdio/🎞️gif/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `gif-89a-any-fixture-generator` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-gif-fixture-tests` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/📦️packages/🦀️rust` | `stdio/🎞️gif` | yes |
| other | `gif-87a-any-reader` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |
| other | `gif-89a-extension-reader` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |

## 🎥️mp4

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-mp4` | `stdio/🎥️mp4/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| io | `semio-s-artifact-stdio-mp4-boxes` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/📦️boxes/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| io | `semio-s-artifact-stdio-mp4-text` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| mutations | `semio-s-artifact-stdio-mp4-mutations` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| schema | `semio-s-artifact-stdio-mp4-inferences` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| schema | `semio-s-artifact-stdio-mp4-schema` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| oracle | `semio-s-artifact-stdio-mp4-test-oracle` | `stdio/🎥️mp4/🔮️oracles/📦️packages/🦀️rust` | `stdio/🎥️mp4/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-mp4-tests` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |
| other | `semio-s-artifact-stdio-mp4-viewer` | `stdio/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust` | `stdio/🎥️mp4` | yes |

## 🎨️svg

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-svg` | `stdio/🎨️svg/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| snapshot | `semio-s-artifact-stdio-svg-snapshot` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| mutations | `semio-s-artifact-stdio-svg-mut-family` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/➕insert-tiny-element/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| mutations | `semio-s-artifact-stdio-svg-mutations` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| mutations | `semio-s-artifact-stdio-svg-mutations-2` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| schema | `semio-s-artifact-stdio-svg-diff` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| schema | `semio-s-artifact-stdio-svg-schema` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| schema | `semio-s-artifact-stdio-svg-schema-2` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| schema | `semio-s-artifact-stdio-svg-schema-3` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🧬️schema/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| oracle | `semio-s-artifact-stdio-svg-test-oracle` | `stdio/🎨️svg/🔮️oracles/📦️packages/🦀️rust` | `stdio/🎨️svg/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| test | `semio-s-artifact-stdio-svg-tests` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🎨️svg` | yes |
| other | `quick-xml-svg-codec` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 🎵️mp3

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-mp3` | `stdio/🎵️mp3/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| io | `semio-s-artifact-stdio-mp3-io-metadata` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🏷️metadata/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| snapshot | `semio-s-artifact-stdio-mp3-metadata` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🏷️metadata/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| mutations | `semio-s-artifact-stdio-mp3-mutations` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| schema | `semio-s-artifact-stdio-mp3-inferences` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| schema | `semio-s-artifact-stdio-mp3-schema` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |
| oracle | `semio-s-artifact-stdio-mp3-test-oracle` | `stdio/🎵️mp3/🔮️oracles/📦️packages/🦀️rust` | `stdio/🎵️mp3/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-mp3-editor` | `stdio/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🎵️mp3` | yes |

## 🏗️ifc

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-ifc` | `stdio/🏗️ifc/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| io | `semio-s-artifact-stdio-ifc-2x3-base-details` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| io | `semio-s-artifact-stdio-ifc4x3-exchange` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🌉️exchange/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| io | `semio-s-artifact-stdio-ifc4x3-fem` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🏗️fem/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| io | `semio-s-artifact-stdio-ifc4x3-step` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧾️step/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| mutations | `semio-s-artifact-stdio-ifc-2x3-base-mutations` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| mutations | `semio-s-artifact-stdio-ifc-4-mutations` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-2x3-base-descriptor` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-2x3-base-inferences` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-2x3-base-model` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-4-descriptor` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-4-inferences` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc-4-model` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-00` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-01` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/01/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-02` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/02/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-03` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/03/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-04` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/04/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-05` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/05/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-entities-06` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🏢️entities/06/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-functions-00` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/⚙️functions/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-rules-00` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/⚖️rules/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc2x3-catalog-types-00` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🧩️components/🔀️types/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-00` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-01` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/01/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-02` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/02/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-03` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/03/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-04` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/04/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-05` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/05/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-06` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/06/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-07` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/07/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-entities-08` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/08/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-functions-00` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/⚙️functions/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-rules-00` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/⚖️rules/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4-catalog-types-00` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🔀️types/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-00` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-01` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/01/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-02` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/02/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-03` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/03/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-04` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/04/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-05` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/05/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-06` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/06/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-07` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/07/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-08` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/08/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-09` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🏢️entities/09/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-functions-00` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/⚙️functions/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-rules-00` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/⚖️rules/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-add2-catalog-types-00` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧩️components/🔀️types/00/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-evaluator` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧮️evaluator/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-express` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🌳️express/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-machine` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/⚙️machine/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-mvd` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-mvd-catalog` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/📖️catalog/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-population` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/👥️population/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-rules` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/📚️rules/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-schema` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧭️schema/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| schema | `semio-s-artifact-stdio-ifc4x3-templates` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| oracle | `semio-s-artifact-stdio-ifc-test-oracle` | `stdio/🏗️ifc/🔮️oracles/📦️packages/🦀️rust` | `stdio/🏗️ifc/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-ifc-2x3-base` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-2x3-cobie` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-2x3-cv20` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-2x3-sav` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-2x3-viewer` | `stdio/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-4` | `stdio/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-4x3` | `stdio/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-catalog` | `stdio/🏗️ifc/🧩️composition/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-declarations` | `stdio/🏗️ifc/📚️catalog/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details` | `stdio/🏗️ifc/🔎️details/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-edit` | `stdio/🏗️ifc/🔎️details/✏️edit/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-guidance` | `stdio/🏗️ifc/🔎️details/🧶️guidance/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-laws-1` | `stdio/🏗️ifc/🔎️details/🧪️laws-1/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-laws-2` | `stdio/🏗️ifc/🔎️details/🧪️laws-2/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-profile` | `stdio/🏗️ifc/🔎️details/🧭️profile/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-provider` | `stdio/🏗️ifc/🔎️details/👁️provider/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-type` | `stdio/🏗️ifc/🔎️details/🔤️type/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-details-wire` | `stdio/🏗️ifc/🔎️details/📨️wire/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-domain` | `stdio/🏗️ifc/📚️catalog/⚖️domain/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-geometry` | `stdio/🏗️ifc/📐️geometry/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-geometry-rings` | `stdio/🏗️ifc/📐️geometry/🕳️rings/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |
| other | `semio-s-artifact-stdio-ifc-json-pack` | `stdio/🏗️ifc/📚️catalog/🧾️json-pack/📦️packages/🦀️rust` | `stdio/🏗️ifc` | yes |

## 💬️bcf

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-bcf` | `stdio/💬️bcf/📦️packages/🦀️rust` | `stdio/💬️bcf` | yes |
| editor | `semio-s-artifact-stdio-bcf-io-registry` | `stdio/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/📦️packages/🦀️rust` | `stdio/💬️bcf` | yes |
| mutations | `semio-s-artifact-stdio-bcf-mutations` | `stdio/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/💬️bcf` | yes |
| schema | `semio-s-artifact-stdio-bcf-inferences` | `stdio/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/💬️bcf` | yes |
| schema | `semio-s-artifact-stdio-bcf-schema` | `stdio/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📦️packages/🦀️rust` | `stdio/💬️bcf` | yes |
| oracle | `semio-s-artifact-stdio-bcf-test-oracle` | `stdio/💬️bcf/🔮️oracles/📦️packages/🦀️rust` | `stdio/💬️bcf/🔮️oracles/📦️packages/🦀️rust` | yes |

## 💾️binary

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-binary` | `stdio/💾️binary/📦️packages/🦀️rust` | `stdio/💾️binary` | yes |
| snapshot | `semio-s-artifact-stdio-binary-snapshot` | `stdio/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/💾️binary` | yes |
| mutations | `semio-s-artifact-stdio-binary-mutations` | `stdio/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/💾️binary` | yes |
| schema | `semio-s-artifact-stdio-binary-schema` | `stdio/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/💾️binary` | yes |
| oracle | `semio-s-artifact-stdio-binary-test-oracle` | `stdio/💾️binary/🔮️oracles/📦️packages/🦀️rust` | `stdio/💾️binary/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-binary-io-registry` | `stdio/💾️binary/🏅️standards/🔖️raw/📦️packages/🦀️rust` | `stdio/💾️binary` | yes |

## 📊️csv

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-csv` | `stdio/📊️csv/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| editor | `semio-s-artifact-stdio-csv-editor` | `stdio/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| editor | `semio-s-artifact-stdio-csv-io-registry` | `stdio/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| snapshot | `semio-s-artifact-stdio-csv-snapshot` | `stdio/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| mutations | `semio-s-artifact-stdio-csv-mutations` | `stdio/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| schema | `semio-s-artifact-stdio-csv-schema` | `stdio/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/📊️csv` | yes |
| oracle | `semio-s-artifact-stdio-csv-test-oracle` | `stdio/📊️csv/🔮️oracles/📦️packages/🦀️rust` | `stdio/📊️csv/🔮️oracles/📦️packages/🦀️rust` | yes |

## 📐️step

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-step` | `stdio/📐️step/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-base-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc1-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc2-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc3-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc4-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc5-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| editor | `semio-s-artifact-stdio-step-cc6-editor` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| io | `semio-s-artifact-stdio-step-geometry` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| io | `semio-s-artifact-stdio-step-ladder` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪜️ladder/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| snapshot | `semio-s-artifact-stdio-step-snapshot` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| mutations | `semio-s-artifact-stdio-step-mutations` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| schema | `semio-s-artifact-stdio-step-inferences` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| oracle | `semio-s-artifact-stdio-step-test-oracle` | `stdio/📐️step/🔮️oracles/📦️packages/🦀️rust` | `stdio/📐️step/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-step-base` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc1` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc2` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc3` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc4` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc5` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/📦️packages/🦀️rust` | `stdio/📐️step` | yes |
| other | `semio-s-artifact-stdio-step-cc6` | `stdio/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/📦️packages/🦀️rust` | `stdio/📐️step` | yes |

## 📑️tsv

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-tsv` | `stdio/📑️tsv/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| io | `semio-s-artifact-stdio-tsv-io-registry` | `stdio/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| snapshot | `semio-s-artifact-stdio-tsv-snapshot` | `stdio/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| mutations | `semio-s-artifact-stdio-tsv-mutations` | `stdio/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| schema | `semio-s-artifact-stdio-tsv-inferences` | `stdio/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| schema | `semio-s-artifact-stdio-tsv-schema` | `stdio/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/📑️tsv` | yes |
| oracle | `semio-s-artifact-stdio-tsv-test-oracle` | `stdio/📑️tsv/🔮️oracles/📦️packages/🦀️rust` | `stdio/📑️tsv/🔮️oracles/📦️packages/🦀️rust` | yes |

## 📕️xlsx

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-xlsx` | `stdio/📕️xlsx/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| mutations | `semio-s-artifact-stdio-xlsx-base-mutations` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| mutations | `semio-s-artifact-stdio-xlsx-strict-mutations` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| schema | `semio-s-artifact-stdio-xlsx-base-descriptor` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| schema | `semio-s-artifact-stdio-xlsx-base-inferences` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| schema | `semio-s-artifact-stdio-xlsx-base-model` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| oracle | `semio-s-artifact-stdio-xlsx-test-oracle` | `stdio/📕️xlsx/🔮️oracles/📦️packages/🦀️rust` | `stdio/📕️xlsx/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-xlsx-base` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| other | `semio-s-artifact-stdio-xlsx-strict` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |
| other | `semio-s-artifact-stdio-xlsx-transitional` | `stdio/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust` | `stdio/📕️xlsx` | yes |

## 📖️pdf

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-pdf` | `stdio/📖️pdf/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| io | `semio-s-artifact-stdio-pdf-mut-base-6` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| io | `semio-s-artifact-stdio-pdf-mut-vt` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| io | `semio-s-artifact-stdio-pdf-mut-x` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| snapshot | `semio-s-artifact-stdio-pdf-model` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| snapshot | `semio-s-artifact-stdio-pdf-model-profiles` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model/🧩️profiles/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| snapshot | `semio-s-artifact-stdio-pdf-snapshot` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| snapshot | `semio-s-artifact-stdio-pdf-snapshot-2` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| snapshot | `semio-s-artifact-stdio-pdf-snapshot-surfaces` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪟️surfaces/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-base-3` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-base-5` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-base-7` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-family-2` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🧺️remove-catalog-entry/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-family-3` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🚀️insert-launch-action/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-family-4` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🔧️set-object-value/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-family-6` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/💾️binary/🧬️mutations/🏳️set-output-intent/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mut-family-7` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/💾️binary/🧬️mutations/🗑️remove-mark-info/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mutations` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| mutations | `semio-s-artifact-stdio-pdf-mutations-2` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-conformance-support` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🏅️conformance-support/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-diff` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-diff-surfaces` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🪟️surfaces/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-graph-projection` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔎️graph-projection/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-inferences` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-mut-family-15` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-schema` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-schema-2` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-schema-3` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-schema-4` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-schema-5` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| schema | `semio-s-artifact-stdio-pdf-testkit` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧪️testkit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| oracle | `semio-s-artifact-stdio-pdf-test-oracle` | `stdio/📖️pdf/🔮️oracles/📦️packages/🦀️rust` | `stdio/📖️pdf/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `pdf-1-7-base-fixture-generator` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-base-8` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕️append-page-content/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-base-9` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🎚️remove-ext-g-state/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-10` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-11` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🪄️replace-page/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-12` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗂️set-catalog-entry/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-13` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-14` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-8` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-family-9` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-mut-x-2` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-tests` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| test | `semio-s-artifact-stdio-pdf-tests-2` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `pdf-1-4-a-lopdf-engine` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-4-base-lopdf-engine` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-4-x-lopdf-engine` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-a-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-base-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-e-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-h-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-ua-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-vt-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `pdf-1-7-x-lopdf-engine` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-pdf-fonts` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-glyph-list` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🔡️glyph-list/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-lift` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-lower` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-mut-base` | `stdio/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-mut-base-2` | `stdio/📖️pdf/🏅️standards/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-mut-base-4` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-mut-family` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-mut-family-5` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-tables` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🅰️tables/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |
| other | `semio-s-artifact-stdio-pdf-viewer` | `stdio/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/📦️packages/🦀️rust` | `stdio/📖️pdf` | yes |

## 📚️idml

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-idml` | `stdio/📚️idml/📦️packages/🦀️rust` | `stdio/📚️idml` | yes |
| schema | `semio-s-artifact-stdio-idml-schema` | `stdio/📚️idml/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/📚️idml` | yes |
| test | `semio-s-artifact-stdio-idml-tests` | `stdio/📚️idml/🧪️tests/📦️packages/🦀️rust` | `stdio/📚️idml` | yes |
| other | `semio-s-artifact-stdio-idml-any` | `stdio/📚️idml/🏅️standards/🔖️8.0/🪆️subsets/✳️any/📦️packages/🦀️rust` | `stdio/📚️idml` | yes |

## 📜️docx

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-docx` | `stdio/📜️docx/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| mutations | `semio-s-artifact-stdio-docx-base-edit-rules` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️edit-rules/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| mutations | `semio-s-artifact-stdio-docx-base-mutations` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| mutations | `semio-s-artifact-stdio-docx-base-xml-address` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| mutations | `semio-s-artifact-stdio-docx-strict-mutations` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| schema | `semio-s-artifact-stdio-docx-base-descriptor` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| schema | `semio-s-artifact-stdio-docx-base-diff` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧱️diff-codecs/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| schema | `semio-s-artifact-stdio-docx-base-inferences` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| schema | `semio-s-artifact-stdio-docx-base-model` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| oracle | `semio-s-artifact-stdio-docx-test-oracle` | `stdio/📜️docx/🔮️oracles/📦️packages/🦀️rust` | `stdio/📜️docx/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-docx-base` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| other | `semio-s-artifact-stdio-docx-strict` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |
| other | `semio-s-artifact-stdio-docx-transitional` | `stdio/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/📦️packages/🦀️rust` | `stdio/📜️docx` | yes |

## 📝️md

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-md` | `stdio/📝️md/📦️packages/🦀️rust` | `stdio/📝️md` | yes |
| snapshot | `semio-s-artifact-stdio-md-snapshot` | `stdio/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📝️md` | yes |
| mutations | `semio-s-artifact-stdio-md-mutations` | `stdio/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📝️md` | yes |
| schema | `semio-s-artifact-stdio-md-diff` | `stdio/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/📝️md` | yes |
| schema | `semio-s-artifact-stdio-md-schema` | `stdio/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/📝️md` | yes |
| oracle | `semio-s-artifact-stdio-md-test-oracle` | `stdio/📝️md/🔮️oracles/📦️packages/🦀️rust` | `stdio/📝️md/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-md-tests` | `stdio/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📝️md` | yes |

## 📰️xml

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-xml` | `stdio/📰️xml/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| snapshot | `semio-s-artifact-stdio-xml-snapshot` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| mutations | `semio-s-artifact-stdio-xml-mutations` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| schema | `semio-s-artifact-stdio-xml-inferences` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| schema | `semio-s-artifact-stdio-xml-schema` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| schema | `semio-s-artifact-stdio-xml-schema-2` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| oracle | `semio-s-artifact-stdio-xml-test-oracle` | `stdio/📰️xml/🔮️oracles/📦️packages/🦀️rust` | `stdio/📰️xml/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-xml-tests` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| test | `semio-s-artifact-stdio-xml-tests-2` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| test | `semio-s-artifact-stdio-xml-tests-3` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📰️xml` | yes |
| other | `quick-xml-oracle-codec` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 📷️png

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-png` | `stdio/📷️png/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| editor | `semio-s-artifact-stdio-png-editor` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🧬️publication/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| io | `semio-s-artifact-stdio-png-mutations-facets` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🎹️facets/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| io | `semio-s-artifact-stdio-png-mutations-io` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| snapshot | `semio-s-artifact-stdio-png-snapshot` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| mutations | `semio-s-artifact-stdio-png-mutations` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| mutations | `semio-s-artifact-stdio-png-mutations-schema` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| schema | `semio-s-artifact-stdio-png-diff` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| schema | `semio-s-artifact-stdio-png-schema` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| oracle | `semio-s-artifact-stdio-png-test-oracle` | `stdio/📷️png/🔮️oracles/📦️packages/🦀️rust` | `stdio/📷️png/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-png-tests` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📷️png` | yes |
| other | `png-codec` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 📸️jpg

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-jpg` | `stdio/📸️jpg/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| editor | `semio-s-artifact-stdio-jpg-io-registry` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| mutations | `semio-s-artifact-stdio-jpg-mutations` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| schema | `semio-s-artifact-stdio-jpg-inferences` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| schema | `semio-s-artifact-stdio-jpg-operations` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/⚙️operations/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| schema | `semio-s-artifact-stdio-jpg-schema` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| schema | `semio-s-artifact-stdio-jpg-schema-2` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/📦️packages/🦀️rust` | `stdio/📸️jpg` | yes |
| oracle | `semio-s-artifact-stdio-jpg-test-oracle` | `stdio/📸️jpg/🔮️oracles/📦️packages/🦀️rust` | `stdio/📸️jpg/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `jpeg-jfif-codec` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 📼️avi

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-avi` | `stdio/📼️avi/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| snapshot | `semio-s-artifact-stdio-avi-snapshot` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| mutations | `semio-s-artifact-stdio-avi-mutations` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| schema | `semio-s-artifact-stdio-avi-schema` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| oracle | `semio-s-artifact-stdio-avi-test-oracle` | `stdio/📼️avi/🔮️oracles/📦️packages/🦀️rust` | `stdio/📼️avi/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-avi-codec-tests` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🧪️tests/🔬️codec/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| test | `semio-s-artifact-stdio-avi-tests` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/📼️avi` | yes |
| other | `riff-avi-codec` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 📽️pptx

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-pptx` | `stdio/📽️pptx/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| mutations | `semio-s-artifact-stdio-pptx-base-mutations` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| mutations | `semio-s-artifact-stdio-pptx-strict-mutations` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| schema | `semio-s-artifact-stdio-pptx-base-descriptor` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| schema | `semio-s-artifact-stdio-pptx-base-inferences` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| schema | `semio-s-artifact-stdio-pptx-base-model` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| schema | `semio-s-artifact-stdio-pptx-base-vocabulary` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🏷️vocabulary/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| oracle | `semio-s-artifact-stdio-pptx-test-oracle` | `stdio/📽️pptx/🔮️oracles/📦️packages/🦀️rust` | `stdio/📽️pptx/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-pptx-base` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| other | `semio-s-artifact-stdio-pptx-strict` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |
| other | `semio-s-artifact-stdio-pptx-transitional` | `stdio/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust` | `stdio/📽️pptx` | yes |

## 🔊️wav

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-wav` | `stdio/🔊️wav/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |
| editor | `semio-s-artifact-stdio-wav-io-registry` | `stdio/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |
| mutations | `semio-s-artifact-stdio-wav-mutations` | `stdio/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |
| schema | `semio-s-artifact-stdio-wav-inferences` | `stdio/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |
| schema | `semio-s-artifact-stdio-wav-schema` | `stdio/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |
| oracle | `semio-s-artifact-stdio-wav-test-oracle` | `stdio/🔊️wav/🔮️oracles/📦️packages/🦀️rust` | `stdio/🔊️wav/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-wav-codec-tests` | `stdio/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/📦️packages/🦀️rust` | `stdio/🔊️wav` | yes |

## 🔤️txt

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-txt` | `stdio/🔤️txt/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |
| mutations | `semio-s-artifact-stdio-txt-mutations` | `stdio/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |
| schema | `semio-s-artifact-stdio-txt-inferences` | `stdio/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |
| schema | `semio-s-artifact-stdio-txt-schema` | `stdio/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |
| oracle | `semio-s-artifact-stdio-txt-test-oracle` | `stdio/🔤️txt/🔮️oracles/📦️packages/🦀️rust` | `stdio/🔤️txt/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-txt-tests` | `stdio/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |
| other | `semio-s-artifact-stdio-txt-examples` | `stdio/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️packages/🦀️rust` | `stdio/🔤️txt` | yes |

## 🔺️stl

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-stl` | `stdio/🔺️stl/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |
| mutations | `semio-s-artifact-stdio-stl-mutations` | `stdio/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |
| schema | `semio-s-artifact-stdio-stl-inferences` | `stdio/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |
| schema | `semio-s-artifact-stdio-stl-schema` | `stdio/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |
| oracle | `semio-s-artifact-stdio-stl-test-oracle` | `stdio/🔺️stl/🔮️oracles/📦️packages/🦀️rust` | `stdio/🔺️stl/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-stl-examples` | `stdio/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |
| other | `semio-s-artifact-stdio-stl-viewer` | `stdio/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust` | `stdio/🔺️stl` | yes |

## 🖊️dwg

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-dwg` | `stdio/🖊️dwg/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| editor | `semio-s-artifact-stdio-dwg-editor` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| editor | `semio-s-artifact-stdio-dwg-editor-ac1018` | `stdio/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-read` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-read-objects` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-read-objects-dynamic-blocks` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/🎛️dynamic-blocks/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-read-records` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧱️records/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-write` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-write-objects` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/📦️objects/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| io | `semio-s-artifact-stdio-dwg-write-records` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧱️records/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-snapshot` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-snapshot-document` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📄️document/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-snapshot-drawing` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/✏️drawing/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-snapshot-objects` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️objects/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-sqlite-drawing` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/✏️drawing/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-sqlite-drawing-2` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🪜️drawing-2/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-sqlite-entities` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧿️entity-codecs/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| snapshot | `semio-s-artifact-stdio-dwg-sqlite-objects` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/📦️objects/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| mutations | `semio-s-artifact-stdio-dwg-mutations` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-diff` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-exports-graphql` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📤️exports/🔗️graphql/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-exports-json` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📤️exports/🔣️json/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-exports-proto` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📤️exports/🛰️proto/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-exports-rust` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📤️exports/🦀️rust/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-exports-typescript` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📤️exports/🟦️typescript/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-geometry` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-inferences` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| schema | `semio-s-artifact-stdio-dwg-model` | `stdio/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |
| oracle | `semio-s-artifact-stdio-dwg-test-oracle` | `stdio/🖊️dwg/🔮️oracles/📦️packages/🦀️rust` | `stdio/🖊️dwg/🔮️oracles/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-dwg-examples` | `stdio/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust` | `stdio/🖊️dwg` | yes |

## 🖋️dxf

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-dxf` | `stdio/🖋️dxf/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| snapshot | `semio-s-artifact-stdio-dxf-snapshot` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| mutations | `semio-s-artifact-stdio-dxf-mutations` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| schema | `semio-s-artifact-stdio-dxf-diff` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| schema | `semio-s-artifact-stdio-dxf-diff-algebra` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/🧮️collection-algebra-core/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| schema | `semio-s-artifact-stdio-dxf-inferences` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| schema | `semio-s-artifact-stdio-dxf-schema` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |
| oracle | `semio-s-artifact-stdio-dxf-test-oracle` | `stdio/🖋️dxf/🔮️oracles/📦️packages/🦀️rust` | `stdio/🖋️dxf/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `dxf-r12-any-fixture-generator` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-dxf-tests` | `stdio/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🖋️dxf` | yes |

## 🖼️tiff

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-tiff` | `stdio/🖼️tiff/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| snapshot | `semio-s-artifact-stdio-tiff-snapshot` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| snapshot | `semio-s-artifact-stdio-tiff-snapshot-surfaces` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🪟️surfaces/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| mutations | `semio-s-artifact-stdio-tiff-mutations` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| schema | `semio-s-artifact-stdio-tiff-inferences` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| schema | `semio-s-artifact-stdio-tiff-schema` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| schema | `semio-s-artifact-stdio-tiff-schema-2` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| oracle | `semio-s-artifact-stdio-tiff-test-oracle` | `stdio/🖼️tiff/🔮️oracles/📦️packages/🦀️rust` | `stdio/🖼️tiff/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-tiff-mutation-regressions` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧪️tests/🛡️mutation-regressions/📦️packages/🦀️rust` | `stdio/🖼️tiff` | yes |
| other | `tiff-6-0-byte-order-reader` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |
| other | `tiff-ifd-codec` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 🗜️deflate

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-deflate` | `stdio/🗜️deflate/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| io | `semio-s-artifact-stdio-deflate-zlib` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| snapshot | `semio-s-artifact-stdio-deflate-snapshot` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| mutations | `semio-s-artifact-stdio-deflate-mutations` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| schema | `semio-s-artifact-stdio-deflate-schema` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| oracle | `semio-s-artifact-stdio-deflate-test-oracle` | `stdio/🗜️deflate/🔮️oracles/📦️packages/🦀️rust` | `stdio/🗜️deflate/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-deflate-tests` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |
| other | `semio-s-artifact-stdio-deflate-examples` | `stdio/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️packages/🦀️rust` | `stdio/🗜️deflate` | yes |

## 🧊️gltf

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-gltf` | `stdio/🧊️gltf/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| editor | `semio-s-artifact-stdio-gltf-editor` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| io | `semio-s-artifact-stdio-gltf-model-wire` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/🔺️diff/🧵️wire/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| io | `semio-s-artifact-stdio-gltf-model-wire-2` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/🔺️diff/🧵️wire/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| snapshot | `semio-s-artifact-stdio-gltf-inferences-inferences` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/📸️snapshot/🧮️decoded-accessors/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| snapshot | `semio-s-artifact-stdio-gltf-model-schema` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/📸️snapshot/🧪️serde-oracle/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| snapshot | `semio-s-artifact-stdio-gltf-model-snapshot` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| snapshot | `semio-s-artifact-stdio-gltf-model-snapshot-root` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🧳️snapshot-root/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations-add-required-extension` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required/➕️add/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations-mut-any` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🪪️asset/🧾️change-extras/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations-mut-any-2` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🖼️image/🌱️create/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations-mut-any-3` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎚️morph/🔗️bind/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| mutations | `semio-s-artifact-stdio-gltf-mutations-mutations` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔤️primitive/🔗️bind/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-exports-graphql` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🔗️graphql/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-exports-json` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🔣️json/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-exports-proto` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🛰️proto/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-exports-rust` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🦀️rust/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-exports-typescript` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🟦️typescript/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-adjacency` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🤝️adjacency/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-aspect-ratios` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-centroid` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/⚖️mass-distribution/🎯️centroid/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-clearance` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↔️clearance/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-compactness` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🗜️compactness/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-concavity` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕳️concavity/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-connected-components` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🤝️adjacency/🧩️connected-components/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-curvature` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🌀️curvature/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-exposed-area` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🧱️area-volume/☀️exposed-area/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-flatness` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-footprint-area` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/🦶️footprint-area/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-geometry-core` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-kernel/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-repetition-ratio` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🪞️symmetry/🔂️repetition-ratio/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-roughness` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🌊️roughness/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-symmetry` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🪞️symmetry/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-thickness` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↕️thickness/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-topology` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-inferences-volume` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🧱️area-volume/📦️volume/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-model` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| schema | `semio-s-artifact-stdio-gltf-model-diff-aggregate` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🧊️gltf-diff-aggregate/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| oracle | `semio-s-artifact-stdio-gltf-test-oracle` | `stdio/🧊️gltf/🔮️oracles/📦️packages/🦀️rust` | `stdio/🧊️gltf/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-gltf-inferences-projection-tests` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/📸️snapshot/🧮️decoded-accessors/🧪️tests/🧮️decoded-accessor/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-10` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🧪️tests/🔀️flips/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-4` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧪️tests/🧪️fixture-corpus/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-5` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required/➕️add/🧪️tests/🔬️t041/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-6` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🧪️tests/🔬️t053/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-7` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph/🌱️create/🧪️tests/🔬️t065/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-8` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🧪️tests/🚫️removes/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| test | `semio-s-artifact-stdio-gltf-mutations-mut-any-9` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🚚️move/🧪️tests/⬇️demotes/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| other | `gltf-2-0-any-resource-reader` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-gltf-inferences-modules` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔨️modules/💡️inference-measures/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |
| other | `semio-s-artifact-stdio-gltf-model-v2-0` | `stdio/🧊️gltf/🏅️standards/🔖️2.0/📦️packages/🦀️rust` | `stdio/🧊️gltf` | yes |

## 🧱️ply

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-ply` | `stdio/🧱️ply/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |
| mutations | `semio-s-artifact-stdio-ply-mutations` | `stdio/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |
| schema | `semio-s-artifact-stdio-ply-inferences` | `stdio/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |
| schema | `semio-s-artifact-stdio-ply-schema` | `stdio/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |
| oracle | `semio-s-artifact-stdio-ply-test-oracle` | `stdio/🧱️ply/🔮️oracles/📦️packages/🦀️rust` | `stdio/🧱️ply/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-ply-inferences-tests` | `stdio/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |
| test | `semio-s-artifact-stdio-ply-io-registry` | `stdio/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🧱️ply` | yes |

## 🧾️json

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-json` | `stdio/🧾️json/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| snapshot | `semio-s-artifact-stdio-json-number` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔢️number/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| mutations | `semio-s-artifact-stdio-json-mutations` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| mutations | `semio-s-artifact-stdio-json-mutations-2` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| schema | `semio-s-artifact-stdio-json-schema` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| schema | `semio-s-artifact-stdio-json-schema-2` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| oracle | `semio-s-artifact-stdio-json-test-oracle` | `stdio/🧾️json/🔮️oracles/📦️packages/🦀️rust` | `stdio/🧾️json/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-json-tests` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust` | `stdio/🧾️json` | yes |
| other | `json-rfc8259-base-serde-json-engine` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |

## 🧿️semio

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-semio` | `stdio/🧿️semio/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-animation-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-cad-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-document-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-drawing-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-graph-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-image-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-kit-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-mesh-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| editor | `semio-s-artifact-stdio-semio-object-editor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-animation-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-animation-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-audio-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-audio-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-brep-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-brep-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-brep-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-cad-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-cad-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-cad-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-document-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-document-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-document-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-drawing-binary` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/💾️binary/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-drawing-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-drawing-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-drawing-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-drawing-png-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-flow-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-flow-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-flow-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-image-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-image-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-image-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-mesh-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-mesh-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-mesh-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-model-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-model-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-model-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-presentation-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-presentation-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-presentation-io` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-value-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-value-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-video-export` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/📤️export/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| io | `semio-s-artifact-stdio-semio-video-import` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/📥️import/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-animation-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-brep-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-cad-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-document-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-drawing-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-flow-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-graph-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-image-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-kit-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-mesh-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-model-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-object-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-presentation-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| snapshot | `semio-s-artifact-stdio-semio-snapshot` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-animation-schema-3` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/📻insert-channel/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-animation-schema-4` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-audio-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/🎚️set-sample-rate/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-audio-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-brep-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-cad-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-document-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-document-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-drawing-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/➕create-node/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-drawing-mut-family-2` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/📐change-stroke-width/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-flow-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-flow-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-flow-set-node-position` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📍️set-node-position/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-graph-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🌉️create-edge/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-image-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-kit-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-kit-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-mesh-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-mesh-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-model-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/🧱insert-element/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-model-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-mut-family-2` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🕸️apply-mesh/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-object-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-presentation-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/🧩insert-layout/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-presentation-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-table-mut-family` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/🏗️create-column/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-table-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-text-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-value-mutation-leaves` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/🍃️leaves/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-value-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| mutations | `semio-s-artifact-stdio-semio-video-mutations` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-animation-schema-2` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-audio-descriptor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-audio-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-audio-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-brep-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-cad-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-document-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-drawing-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-drawing-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-flow-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-graph-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-graph-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-image-diff` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-image-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-kit-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-mesh-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-model-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-object-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-presentation-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-schema` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-applied` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🌀️entropy-internals/🗂️applied/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-causal` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🧭️causal-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-dependence` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🌀️entropy-internals/🤝️dependence/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-descriptor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-entropy` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🌀️entropy-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-probability` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🎲️probability-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-series` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/🌀️entropy-internals/📈️series/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-statistics` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📊️statistics-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-table-tabular` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📋️tabular-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-text-descriptor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-text-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-text-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-value-algebra` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/➕️algebra-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-value-descriptor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-value-fuzzy` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🌫️fuzzy-internals/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-value-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-value-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-video-descriptor` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📇️descriptor/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-video-inferences` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| schema | `semio-s-artifact-stdio-semio-video-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| oracle | `semio-s-artifact-stdio-semio-test-oracle` | `stdio/🧿️semio/🔮️oracles/📦️packages/🦀️rust` | `stdio/🧿️semio/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-semio-document-examples-unit-tests` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/📚️examples/🗒️memo/🧪️tests/🔬️laws/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-graph-mut-fixture-tests` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-inverts-tests` | `stdio/🧿️semio/🧪️tests/🥣️inverts-tests/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-sqlite-laws-1` | `stdio/🧿️semio/🧪️tests/🪶️sqlite/🧫️subset-laws-1/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-sqlite-laws-2` | `stdio/🧿️semio/🧪️tests/🪶️sqlite/🧬️subset-laws-2/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-sqlite-laws-3` | `stdio/🧿️semio/🧪️tests/🪶️sqlite/🧲️subset-laws-3/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-sqlite-laws-4` | `stdio/🧿️semio/🧪️tests/🪶️sqlite/🧱️subset-laws-4/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| test | `semio-s-artifact-stdio-semio-sqlite-laws-5` | `stdio/🧿️semio/🧪️tests/🪶️sqlite/🧮️subset-laws-5/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `drawing-v1-svg-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |
| other | `semio-brep-json-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |
| other | `semio-cad-json-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |
| other | `semio-cad-oracle-probe` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |
| other | `semio-document-json-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |
| other | `semio-drawing-json-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |
| other | `semio-mesh-json-engine` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-semio-animation` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-animation-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/📚️examples/🚶️walk/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-audio` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-base` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-base-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-base-examples-2` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/🪜️examples-2/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-brep` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-brep-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/📚️examples/🧊️solid/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-brep-viewer` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/👁️viewer/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-cad` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-cad-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/📚️examples/📐️drawing/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-catalog` | `stdio/🧿️semio/🧩️composition/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-document` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-document-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/📚️examples/🗒️memo/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-drawing` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-flow` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-flow-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/📚️examples/🌊️pipeline/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-graph` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-graph-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/📚️examples/🕸️wires/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-image` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-kit` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-kit-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/📚️examples/🪑️furniture/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-mesh` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-mesh-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/📚️examples/🧊️cube/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-model` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-model-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/📚️examples/🏢️building/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-object` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-object-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/📚️examples/📦️crate/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-presentation` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-presentation-examples` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/📚️examples/📽️deck/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-table` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-text` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-value` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-s-artifact-stdio-semio-video` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/📦️packages/🦀️rust` | `stdio/🧿️semio` | yes |
| other | `semio-semio-v1-base-bridge` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge` | `(none)` | no |
| other | `semio-stdio-drawing-oracle-probe` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust` | `stdio/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust` | yes |

## 🪟️bmp

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-bmp` | `stdio/🪟️bmp/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| editor | `semio-s-artifact-stdio-bmp-editor` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🧬️publication/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| editor | `semio-s-artifact-stdio-bmp-editor-2` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| snapshot | `semio-s-artifact-stdio-bmp-snapshot` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| mutations | `semio-s-artifact-stdio-bmp-mutations` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| schema | `semio-s-artifact-stdio-bmp-inferences` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| schema | `semio-s-artifact-stdio-bmp-schema` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| oracle | `semio-s-artifact-stdio-bmp-test-oracle` | `stdio/🪟️bmp/🔮️oracles/📦️packages/🦀️rust` | `stdio/🪟️bmp/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `semio-s-artifact-stdio-bmp-mutation-regressions` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️tests/🧬️mutation-regressions/📦️packages/🦀️rust` | `stdio/🪟️bmp` | yes |
| other | `image-bmp-codec` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust` | `stdio/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust` | yes |

## 🗽️obj

| Role | Package | Directory (relative to repo root) | Owning workspace | Member |
|---|---|---|---|---|
| root | `semio-s-artifact-stdio-obj` | `stdio/🗽️obj/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| mutations | `semio-s-artifact-stdio-obj-mutations` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🧬️mutations/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| schema | `semio-s-artifact-stdio-obj-diff` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| schema | `semio-s-artifact-stdio-obj-inferences` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/💡️inferences/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| schema | `semio-s-artifact-stdio-obj-schema` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| oracle | `semio-s-artifact-stdio-obj-test-oracle` | `stdio/🗽️obj/🔮️oracles/📦️packages/🦀️rust` | `stdio/🗽️obj/🔮️oracles/📦️packages/🦀️rust` | yes |
| test | `obj-3-0-any-fixture-generator` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust` | yes |
| other | `semio-s-artifact-stdio-obj-io-registry` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/👁️viewer/📦️packages/🦀️rust` | `stdio/🗽️obj` | yes |
| other | `tobj-obj-reader` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust` | `stdio/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust` | yes |

