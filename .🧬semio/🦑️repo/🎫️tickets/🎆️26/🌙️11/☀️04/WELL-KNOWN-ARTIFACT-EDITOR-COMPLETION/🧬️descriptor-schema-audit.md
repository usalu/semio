# Current Descriptor Schema Audit

Inspected all 38 exact acceptance owners in current committed family component descriptors. 37 exist with nonempty AppIo.artifactSchema; IDML is absent. Of those 37, 36 match an authored DOCUMENT_SCHEMA constant in the owning artifact tree. Semio text is mismatched: descriptor s.stdio.semio.text versus editor/viewer/snapshot stdio.semio.text. This is a source/descriptor audit, not a claim of runtime archive decoding.

| Artifact | Current descriptor schema | Authored match |
| --- | --- | --- |
| html | stdio.html | yes |
| json | stdio.json | yes |
| md | stdio.md | yes |
| txt | stdio.txt | yes |
| xml | stdio.xml | yes |
| csv | stdio.csv | yes |
| docx | stdio.docx | yes |
| idml | MISSING | stdio.idml |
| pptx | stdio.pptx | yes |
| tsv | stdio.tsv | yes |
| xlsx | stdio.xlsx | yes |
| bmp | stdio.bmp | yes |
| gif | stdio.gif.89a | yes |
| jpg | stdio.jpg | yes |
| png | stdio.png | yes |
| tiff | stdio.tiff | yes |
| dwg | stdio.dwg | yes |
| dxf | stdio.dxf | yes |
| pdf | stdio.pdf.1.7 | yes |
| svg | stdio.svg | yes |
| gltf | stdio.gltf | yes |
| las | stdio.las | yes |
| obj | stdio.obj | yes |
| ply | stdio.ply | yes |
| step | stdio.step | yes |
| stl | stdio.stl | yes |
| bcf | stdio.bcf | yes |
| epw | stdio.epw | yes |
| idf | energy.model | yes |
| ifc | stdio.ifc | yes |
| avi | stdio.avi | yes |
| mp3 | stdio.mp3 | yes |
| mp4 | stdio.mp4 | yes |
| wav | stdio.wav | yes |
| binary | stdio.binary | yes |
| deflate | stdio.deflate | yes |
| semio | s.stdio.semio.text | NO — stdio.semio.text |
| zip | stdio.zip | yes |

AppDefinition v2 and AppIo v1 authoritative schema projections require io and artifactSchema. The stale comments saying not populated do not reflect current data. Direct access in new scope is typed correctly; do not add breadcrumb fallbacks.
