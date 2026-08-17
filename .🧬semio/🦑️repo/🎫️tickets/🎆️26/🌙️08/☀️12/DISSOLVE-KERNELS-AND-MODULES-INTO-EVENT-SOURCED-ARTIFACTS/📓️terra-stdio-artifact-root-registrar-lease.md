# Terra Stdio Artifact-Root Registrar Lease

## Baseline

- Owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️component.json` and direct artifact classification only. Repository, `✏️s`, and `🗄️stdio` instructions were reread; no deeper artifact instruction file exists.
- The canonical artifact-root manifest was absent. The scoped `s.stdio.gltf` census therefore completed with 58 components, 1 error, and 0 warnings: the sole `manifest-child-missing` finding at `🗿️artifacts/🧊️gltf`.
- The 36 direct child definitions and immediate leaves are already present. They are protected concurrent work (`36` added schema definitions and their direct TypeScript leaves are dirty, while glTF's direct Rust leaf is also dirty), so this lease does not modify any child internals.
- Baseline SHA-256 of all direct child definitions and immediate Rust/TypeScript leaves: `d991aefb5392ba7700a89b99bacec2d3aa1c35e0812de6da0f6c64a26a0b983b`.

## Direct Artifact Roster

Every roster entry has exact kind `artifact`, the canonical immediate leaves `🟦️component.ts` and `🦀️component.rs`, and a schema-owned `🧬️schema/📜️artifact-definition.json` matching its directory and ID.

| Directory | Semantic ID | Kind | Responsibility |
| --- | --- | --- | --- |
| ☁️las | s.stdio.las | artifact | LAS Point Cloud artifact definition. |
| ☁️ply | s.stdio.ply | artifact | Polygon File Format artifact definition. |
| 🌐️html | s.stdio.html | artifact | Hypertext Markup Language artifact definition. |
| 🌦️epw | s.stdio.epw | artifact | EnergyPlus Weather artifact definition. |
| 🎒️zip | s.stdio.zip | artifact | ZIP Archive artifact definition. |
| 🎞️gif | s.stdio.gif | artifact | Graphics Interchange Format artifact definition. |
| 🎞️pptx | s.stdio.pptx | artifact | PowerPoint Presentation artifact definition. |
| 🎥️mp4 | s.stdio.mp4 | artifact | MPEG-4 Part 14 artifact definition. |
| 🎨️svg | s.stdio.svg | artifact | Scalable Vector Graphics artifact definition. |
| 🎵️mp3 | s.stdio.mp3 | artifact | MPEG Audio Layer III artifact definition. |
| 🏗️ifc | s.stdio.ifc | artifact | Industry Foundation Classes artifact definition. |
| 💬️bcf | s.stdio.bcf | artifact | BIM Collaboration Format artifact definition. |
| 💾️binary | s.stdio.binary | artifact | Binary artifact definition. |
| 📄️pdf | s.stdio.pdf | artifact | Portable Document Format artifact definition. |
| 📄txt | s.stdio.txt | artifact | Plain Text artifact definition. |
| 📊️csv | s.stdio.csv | artifact | Comma-Separated Values artifact definition. |
| 📐️step | s.stdio.step | artifact | ISO 10303 STEP artifact definition. |
| 📑️tsv | s.stdio.tsv | artifact | Tab-Separated Values artifact definition. |
| 📕️xlsx | s.stdio.xlsx | artifact | Excel Workbook artifact definition. |
| 📜️docx | s.stdio.docx | artifact | Word Document artifact definition. |
| 📝️md | s.stdio.md | artifact | Markdown artifact definition. |
| 📰xml | s.stdio.xml | artifact | Extensible Markup Language artifact definition. |
| 📷️jpg | s.stdio.jpg | artifact | JPEG artifact definition. |
| 📷️png | s.stdio.png | artifact | Portable Network Graphics artifact definition. |
| 📼️avi | s.stdio.avi | artifact | Audio Video Interleave artifact definition. |
| 🔊️wav | s.stdio.wav | artifact | Waveform Audio artifact definition. |
| 🔣️json | s.stdio.json | artifact | JavaScript Object Notation artifact definition. |
| 🖊️dwg | s.stdio.dwg | artifact | Drawing artifact definition. |
| 🖊️dxf | s.stdio.dxf | artifact | Drawing Exchange Format artifact definition. |
| 🖼️bmp | s.stdio.bmp | artifact | Bitmap artifact definition. |
| 🖼️tiff | s.stdio.tiff | artifact | Tagged Image File Format artifact definition. |
| 🗜️deflate | s.stdio.deflate | artifact | Zlib Deflate Stream artifact definition. |
| 🟪️stl | s.stdio.stl | artifact | Stereolithography artifact definition. |
| 🧊️gltf | s.stdio.gltf | artifact | glTF artifact definition. |
| 🧊️obj | s.stdio.obj | artifact | Wavefront OBJ artifact definition. |
| 🧿️semio | s.stdio.semio | artifact | Semio artifact definition. |

## Registrar Decision

The taxonomy declares `🗿️artifacts` as an `artifact` collection and requires `🔣️component.json` with `x-semio.kind: collection`. The root will receive all 36 exact direct-child members, each using the child definition's `id` and `directory`, `kind: artifact`, and its localized English definition responsibility. No root language leaf is added; the collection remains mechanical assembly only.

## Application And Validation

- Added the sole source change, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️component.json`. It is an assembly-only `x-semio` collection manifest with exactly the complete roster above; no direct child definition, leaf, source implementation, glue, taxonomy, or generated file changed in this lease.
- Root manifest SHA-256: `382b3cd93912f6b7f693d62ddb15175f8d61464b0a350fb472153bf3b0260349`.
- JSON/bijection audit passed: `36` manifest members equal `36` direct directories; all `36` IDs and exact directory names match the child artifact definitions; all `72` immediate canonical language leaves are present; the root itself contains only `🔣️component.json`.
- `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` completed with 59 components, 0 errors, and 0 warnings (`No findings`).
- Tracked and untracked whitespace checks passed: `git diff --check` and `git diff --no-index --check` for the root manifest and this report.
- No central registrar request is necessary: the correction is semantic collection classification only and changes no glue/discovery/taxonomy contract.
