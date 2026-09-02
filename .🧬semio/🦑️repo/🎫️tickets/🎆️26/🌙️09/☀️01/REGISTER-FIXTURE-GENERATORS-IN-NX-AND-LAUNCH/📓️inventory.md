# Fixture Generator Inventory

Enumerated with the repository taxonomy under `✏️s` and `🧰️framework`. Each generator owns a colocated `📋️project.json`; the repository's emoji-project plugin supplies the local working directory and `nx:run-commands` executor. Every target is non-cached, forwards arguments, and calls only the local `📜️script.ts <mode>`.

| Generator script | Nx project | Registered modes |
| --- | --- | --- |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-mathematical-1-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-sequence-1-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-fem-2d-1-any` | `generate`, `manifests`, `carrier`, `carrier-manifests` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-fem-3d-1-any` | `generate`, `manifests`, `carrier`, `carrier-manifests` |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-draw-1-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-las-1-0-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-gif-87a-any` | `generate`, `manifests`, `aspect`, `aspect-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-gif-89a-any` | `generate`, `manifests`, `build`, `build-manifests`, `extensions`, `extensions-manifests`, `aspect`, `aspect-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-svg-1-1-base` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-bcf-2-1-any` | `generate` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️a/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-4-a` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-4-base` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-4-x` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-a` | `generate`, `manifests`, `encryption`, `encryption-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-base` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-e` | `generate`, `manifests`, `encryption`, `encryption-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-h` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-ua` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-vt` | `generate`, `manifests`, `encryption`, `encryption-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-pdf-1-7-x` | `generate`, `manifests`, `encryption`, `encryption-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-step-ap214-cc6` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-docx-ecma-376-base` | `generate` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-xml-1-0-base` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-jpg-jfif-1-01-document` | `generate`, `manifests`, `libjpeg`, `libjpeg-manifests`, `markers`, `markers-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-png-1-2-any` | `generate`, `manifests`, `chunks`, `chunks-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-avi-1-0-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-json-rfc8259-base` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-dxf-r12-any` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-bmp-v3-any` | `generate`, `manifests`, `header`, `header-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-tiff-6-0-document` | `generate`, `manifests`, `byte-order`, `byte-order-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-gltf-2-0-any` | `generate`, `list` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-obj-3-0-any` | `generate`, `manifests`, `list-recipes`, `document`, `document-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-semio-v1-brep` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-semio-v1-cad` | `generate` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-semio-v1-document` | `generate`, `manifests`, `carrier`, `carrier-manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-semio-v1-drawing` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-stdio-semio-v1-mesh` | `generate`, `manifests` |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` | `@semio-tech/fixture-generator-note-1-any` | `generate`, `manifests` |

## Decisions

- Registered all 38 primary `generate` modes and every additional executable mode, for 108 targets total.
- Added a one-to-one `node-terminal` launch configuration for every target in `3_dev`, ordered `386.701` through `386.808` after the existing scale-fixture entry.
- Standardized `extensions-manifests`, `markers-manifests`, and `chunks-manifests` so each manifest mode matches its plural generator mode.
- Added no `test`, `gate`, dependency, or package-script wiring. Fixture generation remains an explicit developer action.
- Kept `📋️project.json`: native plain-`project.json` discovery reproduces Nx's lossy-Unicode duplicate-root failure, while the repository's emoji-project plugin filters the corrupt path and resolves all 38 projects.
