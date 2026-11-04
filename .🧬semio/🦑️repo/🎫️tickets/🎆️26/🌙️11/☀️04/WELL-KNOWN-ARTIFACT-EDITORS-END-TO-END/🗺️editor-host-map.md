# 🗺️ Editor → Window Kit Map (T18h, static)

| Kit | Editors |
| --- | --- |
| Text | html, txt, md, binary (splices), deflate |
| Table | epw, bcf, csv, tsv, xlsx base, wav |
| Tree | xml, json, zip (editable kind) |
| Image | gif, svg, jpg, png, bmp, tiff, dxf, dwg |
| Document | docx base, pptx, idml (new) |
| Media | mp3, avi, mp4 |
| Mesh render | step base and cc1–cc6, gltf, all 19 semio subsets |
| Mesh window_kind only, no render call in main | obj, stl, ply, las, ifc 4, ifc 2x3 |
| Delegated, not traced | pdf (page kit), xlsx/docx strict and transitional |

The top-right Details panel is hidden by default in the shell (`🖼️Panel/🟦️.tsx:372`, `visible=false`). Whether the app-level `snapshot_details_split_layout` shows Details regardless of the shell panel is unverified (T18i).
