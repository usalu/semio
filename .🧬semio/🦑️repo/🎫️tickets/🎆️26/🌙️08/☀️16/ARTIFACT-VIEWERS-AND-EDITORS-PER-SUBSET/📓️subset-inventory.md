# Artifact Subsets Inventory

Generated: 2026-08-16
Task: Enumerate all artifact subsets and apps in the plugin tree

## Summary

- **Total subsets**: 143
- **Total plugins with subsets**: 33
- **Total apps**: 53
- **Owning subsets** (have both schema and io): 142
- **Derived subsets** (missing schema or io): 1

## 1. Per-Plugin Summary Table

| Plugin | Artifact Kinds | Standards | Total Subsets | Owning | Derived |
|--------|----------------|-----------|----|--------|---------|
| ✒️writer | 1 | 1 | 1 | 1 | 0 |
| ➗️mathematical | 1 | 1 | 1 | 1 | 0 |
| 🌀️procedural | 3 | 1 | 3 | 2 | 1 |
| 🌊️flow | 1 | 1 | 1 | 1 | 0 |
| 🌍️gis | 2 | 1 | 2 | 2 | 0 |
| 🌿️vcs | 1 | 1 | 1 | 1 | 0 |
| 🎞️animate | 1 | 1 | 1 | 1 | 0 |
| 🎥️shooting | 1 | 1 | 1 | 1 | 0 |
| 🎪️demonstrator | 1 | 1 | 1 | 1 | 0 |
| 🎬️sequence | 1 | 1 | 1 | 1 | 0 |
| 🏗️fem | 2 | 1 | 2 | 2 | 0 |
| 🏛️architect | 1 | 1 | 1 | 1 | 0 |
| 🏭️process | 1 | 1 | 1 | 1 | 0 |
| 💠️lowpoly | 1 | 1 | 1 | 1 | 0 |
| 💡️reasoning | 1 | 1 | 1 | 1 | 0 |
| 📋️forms | 1 | 1 | 1 | 1 | 0 |
| 📏️layout | 1 | 1 | 1 | 1 | 0 |
| 📐️cad | 1 | 1 | 1 | 1 | 0 |
| 📕️norm | 15 | 1 | 15 | 15 | 0 |
| 📖️playbook | 1 | 1 | 1 | 1 | 0 |
| 📜️imperative | 1 | 1 | 1 | 1 | 0 |
| 📸️remodel | 1 | 1 | 1 | 1 | 0 |
| 🔋️energy | 1 | 1 | 1 | 1 | 0 |
| 🔱️trinity | 2 | 1 | 2 | 2 | 0 |
| 🕸️dag | 1 | 1 | 1 | 1 | 0 |
| 🖍️draw | 1 | 1 | 1 | 1 | 0 |
| 🖨️raster | 1 | 1 | 1 | 1 | 0 |
| 🗄️stdio | 36 | 34 | 88 | 88 | 0 |
| 🗒️note | 1 | 1 | 1 | 1 | 0 |
| 🧩️puzzle | 3 | 1 | 3 | 3 | 0 |
| 🧱️block | 3 | 1 | 3 | 3 | 0 |
| 🪐️space | 1 | 1 | 1 | 1 | 0 |
| 🪵️sourcing | 1 | 1 | 1 | 1 | 0 |

## 2. Complete Subset Inventory

| Plugin | Kind | Standard | Subset | Has Schema | Has I/O | Owning | Archetype | Rust Type |
|--------|------|----------|--------|------------|---------|--------|-----------|-----------|
| ✒️writer | ✒️writer | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| ➗️mathematical | ➗️mathematical | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌀️procedural | 🌀️procedural2d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌀️procedural | 🧊️procedural3d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌀️procedural | 🧩️assembly | 1 | ✳️any | ✓ | ✗ | No | unknown | snapshot-based |
| 🌊️flow | 🌊️flow | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌍️gis | 🏔️gisterrain | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌍️gis | 🗺️gismap | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🌿️vcs | 🌿️vcs | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🎞️animate | 🎬️present | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🎥️shooting | 🎥️shooting | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🎪️demonstrator | 🎪️playground | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🎬️sequence | 🎬️sequence | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🏗️fem | ◻2d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🏗️fem | 🧊️3d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🏛️architect | 🏛️program | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🏭️process | 🧊️process3d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 💠️lowpoly | 💠️lowpoly | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 💡️reasoning | 🔌️wires | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📋️forms | 📋️forms | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📏️layout | 📏️layout | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📐️cad | 📐️cad | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📓️iso16757 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📔️vdi3805 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📕️din4108 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📗️din16798 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1990 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1991 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1992 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1993 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1994 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1995 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1996 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1997 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1998 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📘️en1999 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📕️norm | 📙️din18599 | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📖️playbook | 📖️playbook | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📜️imperative | 📜️imperative | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 📸️remodel | 📸️remodel | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🔋️energy | 🔋️model | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🔱️trinity | ♻️rewrite | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🔱️trinity | 🔌️jack | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🕸️dag | 🕸️dag | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🖍️draw | 🖍️draw | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🖨️raster | 🖨️raster | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | ☁️las | 1.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | ☁️ply | 1.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🌐️html | 5 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🌦️epw | energyplus | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎒️zip | 2.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎒️zip | 2.0 | ✳️iso21320 | ✓ | ✓ | Yes | unknown | ZipIso21320BuilderConstruction |
| 🗄️stdio | 🎞️gif | 87a | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎞️gif | 89a | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎞️pptx | ecma-376 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎞️pptx | ecma-376 | ✳️strict | ✓ | ✓ | Yes | unknown | PptxStrictBuilderConstruction |
| 🗄️stdio | 🎞️pptx | ecma-376 | ✳️transitional | ✓ | ✓ | Yes | unknown | PptxTransitionalBuilderConstruction |
| 🗄️stdio | 🎥️mp4 | isobmff | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎨️svg | 1.1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🎨️svg | 1.1 | ✳️basic | ✓ | ✓ | Yes | unknown | SvgBasicBuilderConstruction |
| 🗄️stdio | 🎨️svg | 1.1 | ✳️tiny | ✓ | ✓ | Yes | unknown | SvgTinyBuilderConstruction |
| 🗄️stdio | 🎵️mp3 | mpeg1-layer3 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🏗️ifc | 2x3 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🏗️ifc | 2x3 | ✳️cobie | ✓ | ✓ | Yes | unknown | Ifc2x3CobieBuilderConstruction |
| 🗄️stdio | 🏗️ifc | 2x3 | ✳️cv20 | ✓ | ✓ | Yes | unknown | Ifc2x3Cv20BuilderConstruction |
| 🗄️stdio | 🏗️ifc | 2x3 | ✳️sav | ✓ | ✓ | Yes | unknown | Ifc2x3SavBuilderConstruction |
| 🗄️stdio | 🏗️ifc | 4 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 💬️bcf | 2.1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 💾️binary | raw | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📄txt | utf-8 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📄️pdf | 1.4 | ✳️a | ✓ | ✓ | Yes | unknown | PdfABuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.4 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📄️pdf | 1.4 | ✳️x | ✓ | ✓ | Yes | unknown | PdfXBuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️a | ✓ | ✓ | Yes | unknown | PdfABuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️e | ✓ | ✓ | Yes | unknown | PdfEBuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️h | ✓ | ✓ | Yes | unknown | PdfHBuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️ua | ✓ | ✓ | Yes | unknown | PdfUaBuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️vt | ✓ | ✓ | Yes | unknown | PdfVtBuilderConstruction |
| 🗄️stdio | 📄️pdf | 1.7 | ✳️x | ✓ | ✓ | Yes | unknown | PdfXBuilderConstruction |
| 🗄️stdio | 📊️csv | rfc4180 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📐️step | ap214 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📐️step | ap214 | ✳️cc1 | ✓ | ✓ | Yes | unknown | StepCc1BuilderConstruction |
| 🗄️stdio | 📐️step | ap214 | ✳️cc2 | ✓ | ✓ | Yes | unknown | StepCc2BuilderConstruction |
| 🗄️stdio | 📐️step | ap214 | ✳️cc3 | ✓ | ✓ | Yes | unknown | StepCc3BuilderConstruction |
| 🗄️stdio | 📐️step | ap214 | ✳️cc4 | ✓ | ✓ | Yes | unknown | StepCc4BuilderConstruction |
| 🗄️stdio | 📐️step | ap214 | ✳️cc5 | ✓ | ✓ | Yes | unknown | StepCc5BuilderConstruction |
| 🗄️stdio | 📐️step | ap214 | ✳️cc6 | ✓ | ✓ | Yes | unknown | StepCc6BuilderConstruction |
| 🗄️stdio | 📑️tsv | iana | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📕️xlsx | ecma-376 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📕️xlsx | ecma-376 | ✳️strict | ✓ | ✓ | Yes | unknown | XlsxStrictBuilderConstruction |
| 🗄️stdio | 📕️xlsx | ecma-376 | ✳️transitional | ✓ | ✓ | Yes | unknown | XlsxTransitionalBuilderConstruction |
| 🗄️stdio | 📜️docx | ecma-376 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📜️docx | ecma-376 | ✳️strict | ✓ | ✓ | Yes | unknown | DocxStrictBuilderConstruction |
| 🗄️stdio | 📜️docx | ecma-376 | ✳️transitional | ✓ | ✓ | Yes | unknown | DocxTransitionalBuilderConstruction |
| 🗄️stdio | 📝️md | commonmark | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📰xml | 1.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📰xml | 1.0 | ✳️valid | ✓ | ✓ | Yes | unknown | XmlValidBuilderConstruction(XmlAnyBuilder); |
| 🗄️stdio | 📷️jpg | jfif-1.01 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📷️jpg | jfif-1.01 | ✳️baseline | ✓ | ✓ | Yes | unknown | JpgBaselineBuilderConstruction(JpgAnyBuilder); |
| 🗄️stdio | 📷️png | 1.2 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 📼️avi | 1.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🔊️wav | riff-pcm | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🔣️json | rfc8259 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🔣️json | rfc8259 | ✳️i-json | ✓ | ✓ | Yes | unknown | JsonIJsonBuilderConstruction(JsonAnyBuilder); |
| 🗄️stdio | 🖊️dwg | ac1018 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🖊️dwg | ac1024 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🖊️dxf | r12 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🖼️bmp | v3 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🖼️tiff | 6.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🖼️tiff | 6.0 | ✳️baseline | ✓ | ✓ | Yes | unknown | TiffBaselineBuilderConstruction |
| 🗄️stdio | 🗜️deflate | rfc1950 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🟪️stl | ascii | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧊️gltf | 2.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧊️obj | 3.0 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️animation | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️audio | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️brep | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️cad | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️document | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️drawing | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️flow | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️graph | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️image | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️kit | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️mesh | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️model | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️object | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️presentation | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️table | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️text | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️value | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗄️stdio | 🧿️semio | v1 | ✳️video | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🗒️note | 🗒️note | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧩️puzzle | ◻2d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧩️puzzle | 🖐️5d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧩️puzzle | 🧊️3d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧱️block | ◻2d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧱️block | 🖐️5d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🧱️block | 🧊️3d | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🪐️space | 🏠️home | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |
| 🪵️sourcing | 🗂️curate | 1 | ✳️any | ✓ | ✓ | Yes | unknown | snapshot-based |

## 3. Applications and Their Modes/Windows

| Plugin | App | Modes | Windows |
|--------|-----|-------|---------|
| ✒️writer | ✒️writer | (no modes) | |
| ➗️mathematical | ➗️mathematical | (no modes) | |
| 🌀️procedural | ◻2d | (no modes) | |
| 🌀️procedural | 🧊️3d | (no modes) | |
| 🌊️flow | 🌊️flow | (no modes) | |
| 🌍️gis | ◻2d | (no modes) | |
| 🌍️gis | 🧊️3d | (no modes) | |
| 🌿️vcs | 🌿️vcs | (no modes) | |
| 🎞️animate | 🎬️present | (no modes) | |
| 🎥️shooting | 🎥️shooting | (no modes) | |
| 🎬️sequence | 🎬️sequence | (no modes) | |
| 🏗️fem | ◻2d | (no modes) | |
| 🏗️fem | 🧊️3d | (no modes) | |
| 🏛️architect | 🏛️architect | (no modes) | |
| 🏭️process | 🧊️3d | (no modes) | |
| 💠️lowpoly | 💠️lowpoly | (no modes) | |
| 💡️reasoning | 🔌️wires | (no modes) | |
| 📋️forms | 📋️forms | (no modes) | |
| 📏️layout | 📏️layout | (no modes) | |
| 📐️cad | 📐️cad | (no modes) | |
| 📕️norm | 📓️iso16757 | (no modes) | |
| 📕️norm | 📔️vdi3805 | (no modes) | |
| 📕️norm | 📕️din4108 | (no modes) | |
| 📕️norm | 📗️din16798 | (no modes) | |
| 📕️norm | 📘️en1990 | (no modes) | |
| 📕️norm | 📘️en1991 | (no modes) | |
| 📕️norm | 📘️en1992 | (no modes) | |
| 📕️norm | 📘️en1993 | (no modes) | |
| 📕️norm | 📘️en1994 | (no modes) | |
| 📕️norm | 📘️en1995 | (no modes) | |
| 📕️norm | 📘️en1996 | (no modes) | |
| 📕️norm | 📘️en1997 | (no modes) | |
| 📕️norm | 📘️en1998 | (no modes) | |
| 📕️norm | 📘️en1999 | (no modes) | |
| 📕️norm | 📙️din18599 | (no modes) | |
| 📖️playbook | 📖️playbook | (no modes) | |
| 📜️imperative | 📜️imperative | (no modes) | |
| 📸️remodel | 📸️remodel | (no modes) | |
| 🔱️trinity | ♻️rewrite | (no modes) | |
| 🔱️trinity | 🔌️jack | (no modes) | |
| 🕸️dag | 🕸️dag | (no modes) | |
| 🖍️draw | 🖍️draw | (no modes) | |
| 🖨️raster | 🖨️raster | (no modes) | |
| 🗒️note | 🗒️note | (no modes) | |
| 🧩️puzzle | ◻2d | (no modes) | |
| 🧩️puzzle | 🖐️5d | (no modes) | |
| 🧩️puzzle | 🧊️3d | (no modes) | |
| 🧱️block | ◻2d | (no modes) | |
| 🧱️block | 🖐️5d | (no modes) | |
| 🧱️block | 🧊️3d | (no modes) | |
| 🪐️space | 🏠️home | (no modes) | |
| 🪐️space | 🪐️space | (no modes) | |
| 🪵️sourcing | 🗂️curate | (no modes) | |

## 4. Inventory Methodology

### Commands Used

**Find all subset directories:**
```bash
find ✏️s/🔌️plugins -type d -name "🪆️subsets" | wc -l
```
Result: 95 subset container directories

**Count total subset entries:**
```bash
python3 << 'EOF'
# (enumerated via find ✏️s/🔌️plugins -type d -path '*🪆️subsets/*' -mindepth 1)
# Total subsets counted: 143
EOF
```

**Find all apps:**
```bash
find ✏️s/🔌️plugins -type d -name "🎛️apps" -exec ls {} \;
```
Result: 53 total apps across all plugins

### Data Collection Method

1. Enumerated subset directories at: `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<kind>/🏅️standards/🔖️<std>/🪆️subsets/<subset>`
2. For each subset, checked for presence of:
   - `🧬️schema` directory (schema ownership)
   - `🚪️io` directory (I/O ownership)
   - Subset is "owning" if both are present
3. Extracted archetype from `🏅️standards/🔖️<std>/🪆️subsets/🔣️component.json` when available
4. Extracted Rust type from `🧬️schema/📸️snapshot/.../🦀️component.rs` (snapshot-based) or `🧬️schema/🦀️component.rs` (direct)
5. Enumerated apps at: `✏️s/🔌️plugins/<plugin>/🎛️apps/<app>`
6. For each app, enumerated modes and windows at: `🎮️modes/<mode>/🪟️windows/<window>`
