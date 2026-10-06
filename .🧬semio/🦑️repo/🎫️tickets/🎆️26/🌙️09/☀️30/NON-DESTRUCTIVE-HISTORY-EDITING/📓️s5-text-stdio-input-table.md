# 📓️ S5-TEXT-STDIO — declared mutation inputs (per crate)

Written by `🗑️generated/s5-text-stdio/table-md.py` from the reviewed table `🧪️s5-text-stdio-input-ui-table.py` (the plan `🧪️s5-text-stdio-input-ui.py` applies). `missing before` compares with HEAD (the tree before session 5): `everything` = the input carried no `x-semio-ui` and read only through type inference and the label glossary. Widget `—` = a structured value or address whose members carry their own declarations; `hidden` = an opaque member beside real parameters (an input without a row). Withdraw-only leaves (design §22.20, descriptor `editable: false`) are listed at the end; none of their inputs is declared.

| crate | leaves with declared inputs | inputs | everything missing | completed | unchanged | withdraw-only leaves |
|---|---:|---:|---:|---:|---:|---:|
| stdio `semio` | 207 | 362 | 328 | 13 | 21 | 25 |
| stdio `pdf` | 107 | 167 | 165 | 1 | 1 | 16 |
| stdio `gltf` | 88 | 166 | 0 | 166 | 0 | 2 |
| stdio `svg` | 27 | 50 | 47 | 0 | 3 | 4 |
| stdio `gif` | 29 | 43 | 0 | 41 | 2 | 2 |
| stdio `step` | 29 | 41 | 37 | 3 | 1 | 7 |
| stdio `xlsx` | 20 | 27 | 26 | 0 | 1 | 5 |
| stdio `ifc` | 19 | 25 | 0 | 23 | 2 | 5 |
| stdio `json` | 13 | 24 | 24 | 0 | 0 | 1 |
| stdio `jpg` | 18 | 23 | 14 | 8 | 1 | 3 |
| stdio `tiff` | 10 | 22 | 6 | 15 | 1 | 4 |
| stdio `obj` | 17 | 21 | 20 | 0 | 1 | 1 |
| stdio `xml` | 10 | 19 | 17 | 0 | 2 | 2 |
| stdio `zip` | 10 | 15 | 14 | 0 | 1 | 4 |
| stdio `pptx` | 14 | 15 | 13 | 1 | 1 | 5 |
| stdio `las` | 9 | 14 | 10 | 3 | 1 | 2 |
| stdio `bmp` | 3 | 14 | 0 | 13 | 1 | 1 |
| stdio `mp4` | 8 | 13 | 8 | 5 | 0 | 1 |
| stdio `avi` | 10 | 13 | 0 | 12 | 1 | 1 |
| stdio `ply` | 8 | 12 | 10 | 1 | 1 | 1 |
| stdio `md` | 4 | 10 | 10 | 0 | 0 | 1 |
| stdio `png` | 3 | 10 | 1 | 9 | 0 | 1 |
| stdio `dxf` | 10 | 10 | 0 | 9 | 1 | 1 |
| stdio `tsv` | 4 | 7 | 4 | 2 | 1 | 1 |
| stdio `stl` | 6 | 7 | 6 | 0 | 1 | 1 |
| stdio `txt` | 4 | 6 | 6 | 0 | 0 | 1 |
| stdio `epw` | 4 | 5 | 0 | 4 | 1 | 1 |
| stdio `csv` | 4 | 5 | 1 | 4 | 0 | 1 |
| stdio `docx` | 5 | 5 | 0 | 4 | 1 | 5 |
| stdio `wav` | 2 | 5 | 3 | 2 | 0 | 2 |
| stdio `binary` | 2 | 3 | 0 | 3 | 0 | 1 |
| stdio `dwg` | 2 | 3 | 0 | 2 | 1 | 1 |
| stdio `deflate` | 2 | 3 | 0 | 3 | 0 | 1 |
| stdio `html` | 2 | 2 | 0 | 2 | 0 | 1 |
| stdio `mp3` | 1 | 1 | 0 | 0 | 1 | 1 |
| stdio `bcf` | 1 | 1 | 0 | 0 | 1 | 2 |
| trinity `rewriting` | 9 | 32 | 32 | 0 | 0 | 1 |
| trinity `jack` | 4 | 9 | 9 | 0 | 0 | 0 |
| vcs `vcs` | 3 | 3 | 3 | 0 | 0 | 0 |
| writer `writer` | 7 | 13 | 13 | 0 | 0 | 0 |
| **total** | | **1226** | **827** | **349** | **50** | **115** |

## stdio · avi

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| hdrl | set-stream-header | `/streamIndex` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | set-stream-format | `/streamIndex` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | remove-stream | `/index` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | insert-stream | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| hdrl | set-chunk-keyframe | `/streamIndex` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | set-chunk-keyframe | `/index` | integer | step | stepper | — | Chunk / Chunk | step=1, precision=0 |
| hdrl | remove-chunk | `/streamIndex` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | remove-chunk | `/index` | integer | step | stepper | — | Chunk / Chunk | step=1, precision=0 |
| hdrl | insert-chunk | `/streamIndex` | integer | step | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| hdrl | insert-chunk | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| hdrl | add-unknown-chunk | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| hdrl | remove-unknown-chunk | `/index` | integer | step | stepper | — | Chunk / Chunk | step=1, precision=0 |
| hdrl | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · bcf

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| markup | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · binary

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | replace-byte-range | `/offset` | integer | step | stepper | — | Byte offset / Byte-Versatz | unit="B", step=1, precision=0 |
| any | replace-byte-range | `/remove_len` | integer | step | stepper | — | Bytes to remove / Zu entfernende Bytes | unit="B", step=1, precision=0 |
| any | truncate-at | `/offset` | integer | step | stepper | — | Byte offset / Byte-Versatz | unit="B", step=1, precision=0 |

## stdio · bmp

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | paint-indexed-region | `/x` | integer | widget, unit, step, precision | stepper | — | Left / Links | unit="px", step=1, precision=0 |
| any | paint-indexed-region | `/y` | integer | widget, unit, step, precision | stepper | — | Top / Oben | unit="px", step=1, precision=0 |
| any | paint-indexed-region | `/width` | integer | widget, unit, step, precision | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| any | paint-indexed-region | `/height` | integer | widget, unit, step, precision | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| any | paint-indexed-region | `/paletteIndex` | integer | widget, step, precision | stepper | — | Palette index / Palettenindex | step=1, precision=0 |
| any | paint-direct-region | `/x` | integer | widget, unit, step, precision | stepper | — | Left / Links | unit="px", step=1, precision=0 |
| any | paint-direct-region | `/y` | integer | widget, unit, step, precision | stepper | — | Top / Oben | unit="px", step=1, precision=0 |
| any | paint-direct-region | `/width` | integer | widget, unit, step, precision | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| any | paint-direct-region | `/height` | integer | widget, unit, step, precision | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| any | paint-direct-region | `/red` | integer | widget, step, precision | slider | — | Red / Rot | step=1, precision=0 |
| any | paint-direct-region | `/green` | integer | widget, step, precision | slider | — | Green / Grün | step=1, precision=0 |
| any | paint-direct-region | `/blue` | integer | widget, step, precision | slider | — | Blue / Blau | step=1, precision=0 |
| any | paint-direct-region | `/alpha` | integer | widget, step, precision | slider | — | Alpha / Alpha | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · csv

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-field | `/recordIndex` | integer | step | stepper | — | Record / Datensatz | step=1, precision=0 |
| any | set-field | `/fieldIndex` | integer | step | stepper | — | Field / Feld | step=1, precision=0 |
| any | remove-record | `/index` | integer | step | stepper | — | Record / Datensatz | step=1, precision=0 |
| any | insert-record | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · deflate

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-preset-dictionary | `/dict_id` | integer | step | stepper | — | Preset dictionary (DICTID) / Vorgabewörterbuch (DICTID) | step=1, precision=0 |
| any | set-compression-params | `/method` | integer | step | stepper | — | Compression method (CM) / Kompressionsverfahren (CM) | step=1, precision=0 |
| any | set-compression-params | `/window_bits` | integer | step | stepper | — | Window size / Fenstergröße | unit="bit", step=1, precision=0, softMin=8, softMax=15 |

## stdio · docx

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| base | insert-table-row | `/index` | integer | step | stepper | — | Row position / Zeilenposition | step=1, precision=0 |
| base | remove-table-row | `/index` | integer | step | stepper | — | Row / Zeile | step=1, precision=0 |
| base | insert-xml-node | `/index` | integer | step | stepper | — | Child position / Kindposition | step=1, precision=0 |
| base | remove-xml-node | `/index` | integer | step | stepper | — | Child position / Kindposition | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · dwg

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-version-info | `/maintenanceVersion` | integer | step | stepper | — | Maintenance version / Wartungsversion | step=1, precision=0 |
| any | set-version-info | `/codepage` | integer | step | stepper | — | Code page / Codepage | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · dxf

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| header | insert-style | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | insert-block | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | set-entity | `/index` | integer | step | stepper | — | Entity / Objekt | step=1, precision=0 |
| header | set-block | `/index` | integer | step | stepper | — | Block / Block | step=1, precision=0 |
| header | remove-entity | `/index` | integer | step | stepper | — | Entity / Objekt | step=1, precision=0 |
| header | insert-entity | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | insert-layer | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | insert-linetype | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| header | remove-block | `/index` | integer | step | stepper | — | Block / Block | step=1, precision=0 |

## stdio · epw

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-record-field | `/recordIndex` | integer | step | stepper | — | Record / Datensatz | step=1, precision=0 |
| any | set-record-field | `/fieldIndex` | integer | step | stepper | — | Field / Feld | step=1, precision=0 |
| any | remove-record | `/index` | integer | step | stepper | — | Record position / Datensatzposition | step=1, precision=0 |
| any | insert-record | `/index` | integer | step | stepper | — | Record position / Datensatzposition | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · gif

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-image-pixels | `/index` | integer | step | stepper | — | Image / Bild | step=1, precision=0 |
| any | set-image-geometry | `/index` | integer | step | stepper | — | Image / Bild | step=1, precision=0 |
| any | set-image-geometry | `/left` | integer | step | stepper | — | Left offset / Abstand links | unit="px", step=1, precision=0 |
| any | set-image-geometry | `/top` | integer | step | stepper | — | Top offset / Abstand oben | unit="px", step=1, precision=0 |
| any | set-image-geometry | `/width` | integer | step | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| any | set-image-geometry | `/height` | integer | step | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| any | set-pixel-aspect-ratio | `/ratio` | integer | step | stepper | — | Pixel aspect ratio / Pixel-Seitenverhältnis | step=1, precision=0 |
| any | set-screen-size | `/width` | integer | step | stepper | — | Logical screen width / Breite des logischen Bildschirms | unit="px", step=1, precision=0 |
| any | set-screen-size | `/height` | integer | step | stepper | — | Logical screen height / Höhe des logischen Bildschirms | unit="px", step=1, precision=0 |
| any | move-image | `/from` | integer | step | stepper | — | From position / Von Position | step=1, precision=0 |
| any | move-image | `/to` | integer | step | stepper | — | To position / Nach Position | step=1, precision=0 |
| any | set-background-color-index | `/index` | integer | step | stepper | — | Background color index / Hintergrundfarbindex | step=1, precision=0 |
| any | insert-image | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | remove-image | `/index` | integer | step | stepper | — | Image / Bild | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| any | set-image-interlace | `/index` | integer | step | stepper | — | Image / Bild | step=1, precision=0 |
| base | set-frame-delay | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | set-frame-delay | `/delayCs` | integer | step | stepper | — | Frame delay / Bildverzögerung | unit="cs", step=1, precision=2 |
| base | set-frame-disposal | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | remove-app-extension | `/index` | integer | step | stepper | — | Application extension / Anwendungserweiterung | step=1, precision=0 |
| base | set-frame-pixels | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | set-frame-transparency | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | set-frame-transparency | `/transparentIndex` | integer | step | stepper | — | Transparent color index / Transparenzfarbindex | step=1, precision=0 |
| base | insert-comment | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | set-frame-geometry | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | set-frame-geometry | `/left` | integer | step | stepper | — | Left offset / Abstand links | unit="px", step=1, precision=0 |
| base | set-frame-geometry | `/top` | integer | step | stepper | — | Top offset / Abstand oben | unit="px", step=1, precision=0 |
| base | set-frame-geometry | `/width` | integer | step | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| base | set-frame-geometry | `/height` | integer | step | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| base | set-pixel-aspect-ratio | `/ratio` | integer | step | stepper | — | Pixel aspect ratio / Pixel-Seitenverhältnis | step=1, precision=0 |
| base | set-screen-size | `/width` | integer | step | stepper | — | Logical screen width / Breite des logischen Bildschirms | unit="px", step=1, precision=0 |
| base | set-screen-size | `/height` | integer | step | stepper | — | Logical screen height / Höhe des logischen Bildschirms | unit="px", step=1, precision=0 |
| base | move-frame | `/from` | integer | step | stepper | — | From position / Von Position | step=1, precision=0 |
| base | move-frame | `/to` | integer | step | stepper | — | To position / Nach Position | step=1, precision=0 |
| base | set-loop-count | `/loopCount` | integer | step | stepper | — | Loop count / Anzahl Wiederholungen | step=1, precision=0 |
| base | set-frame-user-input | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | set-background-color-index | `/index` | integer | step | stepper | — | Background color index / Hintergrundfarbindex | step=1, precision=0 |
| base | insert-frame | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | remove-frame | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| base | remove-comment | `/index` | integer | step | stepper | — | Comment / Kommentar | step=1, precision=0 |
| base | add-app-extension | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| base | set-frame-interlace | `/index` | integer | step | stepper | — | Frame / Einzelbild | step=1, precision=0 |

## stdio · gltf

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | required/add | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | required/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | scene-root/unbind | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene-root/unbind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | scene-root/reorder | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene-root/bind | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene-root/bind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | scene-root/bind | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | scene-root/move | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene-root/move | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | scene-root/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node/change-weights | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node/reparent | `/parent` | integer | step | stepper | — | Parent node / Elternknoten | step=1, precision=0 |
| any | node/reparent | `/child` | integer | step | stepper | — | Child node / Kindknoten | step=1, precision=0 |
| any | node/reparent | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node/rename | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node/transform | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node/change-extras | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | node/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | node/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node/change-extensions | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-child/unbind | `/parent` | integer | step | stepper | — | Parent node / Elternknoten | step=1, precision=0 |
| any | node-child/unbind | `/child` | integer | step | stepper | — | Child node / Kindknoten | step=1, precision=0 |
| any | node-child/reorder | `/parent` | integer | step | stepper | — | Parent node / Elternknoten | step=1, precision=0 |
| any | node-child/bind | `/parent` | integer | step | stepper | — | Parent node / Elternknoten | step=1, precision=0 |
| any | node-child/bind | `/child` | integer | step | stepper | — | Child node / Kindknoten | step=1, precision=0 |
| any | node-child/bind | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node-child/move | `/parent` | integer | step | stepper | — | Parent node / Elternknoten | step=1, precision=0 |
| any | node-child/move | `/child` | integer | step | stepper | — | Child node / Kindknoten | step=1, precision=0 |
| any | node-child/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | morph/unbind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/unbind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/unbind | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/reorder | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/reorder | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/reorder | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/bind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/bind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/bind | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/bind | `/accessor` | integer | step | stepper | — | Accessor / Accessor | step=1, precision=0 |
| any | morph/move | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/move | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/move | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | sampler/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | sampler/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | sampler/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | sampler/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | animation/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | animation/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | animation/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | animation/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | camera/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | camera/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | camera/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | camera/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | texture/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | texture/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | texture/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | texture/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | scene/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | scene/rename | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene/change-extras | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | scene/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | scene/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | scene/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | scene/change-extensions | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | node-mesh/unbind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-mesh/bind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-mesh/bind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | default-scene/bind | `/scene` | integer | step | stepper | — | Scene / Szene | step=1, precision=0 |
| any | material/change-alpha | `/material` | integer | step | stepper | — | Material / Material | step=1, precision=0 |
| any | material/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | material/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | material/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | material/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | material/change-sides | `/material` | integer | step | stepper | — | Material / Material | step=1, precision=0 |
| any | buffer/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | buffer/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | buffer/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | buffer/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | accessor/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | accessor/create | `/count` | integer | step | stepper | — | Element count / Elementanzahl | step=1, precision=0 |
| any | accessor/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | accessor/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | accessor/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | used/add | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | used/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | node-camera/unbind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-camera/bind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-camera/bind | `/camera` | integer | step | stepper | — | Camera / Kamera | step=1, precision=0 |
| any | primitive/unbind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/unbind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/bind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/accessor` | integer | step | stepper | — | Accessor / Accessor | step=1, precision=0 |
| any | primitive/unbind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/unbind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/reorder | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/reorder | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/bind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/accessor` | integer | step | stepper | — | Accessor / Accessor | step=1, precision=0 |
| any | primitive/move | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/move | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | primitive/create | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | primitive/change-topology | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/change-topology | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/change-topology | `/mode` | integer | step | stepper | — | Topology mode / Topologiemodus | step=1, precision=0 |
| any | primitive/change-extras | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/change-extras | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/reorder | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/delete | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/delete | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/move | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/move | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | primitive/change-extensions | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/change-extensions | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | mesh/change-weights | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | mesh/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | mesh/rename | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | mesh/change-extras | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | mesh/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | mesh/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | mesh/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | mesh/change-extensions | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | image/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | image/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | image/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | image/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | skin/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | skin/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | skin/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | skin/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | morph/create | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/create | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | morph/reorder | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/reorder | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/delete | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/delete | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/delete | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/move | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | morph/move | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | morph/move | `/target` | integer | step | stepper | — | Morph target / Morph-Ziel | step=1, precision=0 |
| any | morph/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | primitive/unbind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/unbind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/mesh` | integer | step | stepper | — | Mesh / Netz | step=1, precision=0 |
| any | primitive/bind | `/primitive` | integer | step | stepper | — | Primitive / Primitiv | step=1, precision=0 |
| any | primitive/bind | `/material` | integer | step | stepper | — | Material / Material | step=1, precision=0 |
| any | node-skin/unbind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-skin/bind | `/node` | integer | step | stepper | — | Node / Knoten | step=1, precision=0 |
| any | node-skin/bind | `/skin` | integer | step | stepper | — | Skin / Skin | step=1, precision=0 |
| any | buffer-view/create | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | buffer-view/create | `/buffer` | integer | step | stepper | — | Buffer / Puffer | step=1, precision=0 |
| any | buffer-view/create | `/byteOffset` | integer | step | stepper | — | Byte offset / Byte-Versatz | unit="B", step=1, precision=0 |
| any | buffer-view/create | `/byteLength` | integer | step | stepper | — | Byte length / Byte-Länge | unit="B", step=1, precision=0 |
| any | buffer-view/delete | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | buffer-view/move | `/index` | integer | step | stepper | — | Index / Index | step=1, precision=0 |
| any | buffer-view/move | `/position` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |

## stdio · html

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | insert-node | `/index` | integer | step | stepper | — | Child position / Kindposition | step=1, precision=0 |
| any | remove-node | `/index` | integer | step | stepper | — | Child position / Kindposition | step=1, precision=0 |

## stdio · ifc

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | insert-entity | `/index` | integer | step | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | remove-entity | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| any | set-entity-arg | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| any | set-entity-arg | `/index` | integer | step | stepper | — | Attribute position / Attributposition | step=1, precision=0 |
| any | set-entity-name | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| any | insert-entity-arg | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| any | insert-entity-arg | `/index` | integer | step | stepper | — | Attribute position / Attributposition | step=1, precision=0 |
| any | remove-entity-arg | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| any | remove-entity-arg | `/index` | integer | step | stepper | — | Attribute position / Attributposition | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| cobie | set-facility-name | `/building` | integer | step | stepper | — | Building / Gebäude | step=1, precision=0 |
| cobie | set-floor-elevation | `/storey` | integer | step | stepper | — | Building storey / Geschoss | step=1, precision=0 |
| cobie | set-floor-elevation | `/elevation` | number | step | stepper | — | Elevation / Höhenkote | step=0.1, precision=3 |
| cobie | set-space | `/id` | integer | step | stepper | — | Space / Raum | step=1, precision=0 |
| cobie | set-type-assignment | `/id` | integer | step | stepper | — | Type relationship / Typbeziehung | step=1, precision=0 |
| cv20 | set-structural-entity | `/id` | integer | step | stepper | — | Entity / Entität | step=1, precision=0 |
| cv20 | set-product-placement | `/product` | integer | step | stepper | — | Product / Produkt | step=1, precision=0 |
| cv20 | set-product-placement | `/placement` | integer | step | stepper | — | Placement / Platzierung | step=1, precision=0 |
| cv20 | set-project-units | `/project` | integer | step | stepper | — | Project / Projekt | step=1, precision=0 |
| cv20 | set-project-units | `/units` | integer | step | stepper | — | Unit assignment / Einheitenzuweisung | step=1, precision=0 |
| sav | set-load-group | `/id` | integer | step | stepper | — | Load group / Lastgruppe | step=1, precision=0 |
| sav | set-group-assignment | `/id` | integer | step | stepper | — | Group relationship / Gruppenbeziehung | step=1, precision=0 |
| sav | set-analysis-model | `/id` | integer | step | stepper | — | Analysis model / Berechnungsmodell | step=1, precision=0 |
| base | remove-instance | `/id` | integer | step | stepper | — | Instance / Instanz | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · jpg

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| baseline | remove-frame-component | `/id` | integer | everything (no x-semio-ui) | reference | target | Component / Komponente | ref={"kind": "component"} |
| baseline | insert-huffman-table | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| baseline | insert-huffman-table | `/table` | object | everything (no x-semio-ui) | — | value | Huffman table / Huffman-Tabelle |  |
| baseline | set-component-sampling | `/id` | integer | everything (no x-semio-ui) | reference | target | Component / Komponente | ref={"kind": "component"} |
| baseline | set-component-sampling | `/hSampling` | integer | step | stepper | — | Horizontal sampling factor / Horizontaler Abtastfaktor | step=1 |
| baseline | set-component-sampling | `/vSampling` | integer | step | stepper | — | Vertical sampling factor / Vertikaler Abtastfaktor | step=1 |
| baseline | set-sample-precision | `/precision` | integer | step, snaps | stepper | — | Sample precision / Abtastgenauigkeit | unit="bit", step=1, snaps=[8, 12] |
| baseline | set-sof-marker | `/marker` | integer | step | stepper | — | Marker / Marker | step=1 |
| baseline | insert-frame-component | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| baseline | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| baseline | remove-huffman-table | `/key` | object | everything (no x-semio-ui) | — | — | Table / Tabelle |  |
| document | replace-huffman | `/table` | object | everything (no x-semio-ui) | — | value | Huffman table / Huffman-Tabelle |  |
| document | change-re | `/quality` | integer | step | stepper | — | Re-encode quality / Qualität der Neukodierung | step=1 |
| document | replace-quant | `/table` | object | everything (no x-semio-ui) | — | value | Quantization table / Quantisierungstabelle |  |
| document | insert-other | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| document | change-restart | `/restartInterval` | integer | step | stepper | — | Restart interval / Restart-Intervall | step=1 |
| document | remove-other | `/index` | integer | everything (no x-semio-ui) | stepper | — | Segment / Segment | step=1, precision=0 |
| document | remove-quant | `/id` | integer | everything (no x-semio-ui) | reference | target | Quantization table / Quantisierungstabelle | ref={"kind": "quantizationTable"} |
| document | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| document | remove-huffman | `/key` | object | everything (no x-semio-ui) | — | — | Table / Tabelle |  |
| document | change-jfif | `/version` | array | everything (no x-semio-ui) | — | value | JFIF version / JFIF-Version |  |
| document | change-jfif | `/xDensity` | integer | step | stepper | — | Horizontal pixel density / Horizontale Pixeldichte | step=1 |
| document | change-jfif | `/yDensity` | integer | step | stepper | — | Vertical pixel density / Vertikale Pixeldichte | step=1 |

## stdio · json

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| i-json | upsert-member | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | upsert-member | `/key` | string | everything (no x-semio-ui) | text | — | Member name / Name der Eigenschaft |  |
| i-json | remove-member | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | remove-member | `/key` | string | everything (no x-semio-ui) | text | — | Member name / Name der Eigenschaft |  |
| i-json | rename-member | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | rename-member | `/from` | string | everything (no x-semio-ui) | text | — | Member name / Name der Eigenschaft |  |
| i-json | rename-member | `/to` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| i-json | remove-array-element | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | remove-array-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Element / Element | step=1, precision=0 |
| i-json | insert-array-element | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | insert-array-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| i-json | set-safe-number | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | set-string | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| i-json | set-string | `/value` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| base | set-member | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | set-member | `/key` | string | everything (no x-semio-ui) | text | — | Member name / Name der Eigenschaft |  |
| base | remove-array-element | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | remove-array-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Element / Element | step=1, precision=0 |
| base | insert-array-element | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | insert-array-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | set-scalar | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | remove-member | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | remove-member | `/key` | string | everything (no x-semio-ui) | text | — | Member name / Name der Eigenschaft |  |
| base | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · las

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| header | set-point | `/index` | integer | everything (no x-semio-ui) | stepper | — | Point / Punkt | step=1, precision=0 |
| header | set-point | `/point` | object | everything (no x-semio-ui) | — | value | Point record / Punktdatensatz |  |
| header | insert-point | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | insert-point | `/point` | object | everything (no x-semio-ui) | — | value | Point record / Punktdatensatz |  |
| header | remove-point | `/index` | integer | everything (no x-semio-ui) | stepper | — | Point / Punkt | step=1, precision=0 |
| header | set-scale-and-offset | `/scale` | array | everything (no x-semio-ui) | — | value | Scale factors / Skalierungsfaktoren |  |
| header | set-scale-and-offset | `/offset` | array | everything (no x-semio-ui) | — | value | Offsets / Versätze |  |
| header | remove-vlr | `/index` | integer | everything (no x-semio-ui) | stepper | — | Variable length record / Datensatz variabler Länge (VLR) | step=1, precision=0 |
| header | insert-vlr | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| header | set-version | `/major` | integer | step | stepper | — | Major version / Hauptversion | step=1 |
| header | set-version | `/minor` | integer | step | stepper | — | Minor version / Nebenversion | step=1 |
| header | set-creation-date | `/dayOfYear` | integer | step | stepper | — | Day of year / Tag im Jahr | step=1 |
| header | set-creation-date | `/year` | integer | everything (no x-semio-ui) | stepper | value | Year / Jahr | step=1, precision=0, softMin=1970, softMax=2100 |
| header | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · md

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-inlines | `/path` | array | everything (no x-semio-ui) | — | — | Container path / Containerpfad |  |
| any | set-inlines | `/index` | integer | everything (no x-semio-ui) | stepper | — | Block / Block | step=1, precision=0 |
| any | insert-block | `/path` | array | everything (no x-semio-ui) | — | — | Container path / Containerpfad |  |
| any | insert-block | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | insert-block | `/block` | union | everything (no x-semio-ui) | — | value | Block / Block |  |
| any | remove-block | `/path` | array | everything (no x-semio-ui) | — | — | Container path / Containerpfad |  |
| any | remove-block | `/index` | integer | everything (no x-semio-ui) | stepper | — | Block / Block | step=1, precision=0 |
| any | replace-block | `/path` | array | everything (no x-semio-ui) | — | — | Container path / Containerpfad |  |
| any | replace-block | `/index` | integer | everything (no x-semio-ui) | stepper | — | Block / Block | step=1, precision=0 |
| any | replace-block | `/block` | union | everything (no x-semio-ui) | — | value | Block / Block |  |

## stdio · mp3

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · mp4

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | insert-track | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | remove-track | `/index` | integer | everything (no x-semio-ui) | stepper | — | Track / Spur | step=1, precision=0 |
| any | set-sample-sync | `/trackIndex` | integer | step | stepper | — | Track / Spur | step=1, precision=0 |
| any | set-sample-sync | `/index` | integer | everything (no x-semio-ui) | stepper | — | Sample / Sample | step=1, precision=0 |
| any | set-track-codec | `/trackIndex` | integer | step | stepper | — | Track / Spur | step=1, precision=0 |
| any | set-track-dimensions | `/trackIndex` | integer | step | stepper | — | Track / Spur | step=1, precision=0 |
| any | set-track-dimensions | `/width` | integer | everything (no x-semio-ui) | stepper | value | Width / Breite | unit="px", step=1, precision=0, softMax=7680, snaps=[640, 1280, 1920, 2560, 3840] |
| any | set-track-dimensions | `/height` | integer | everything (no x-semio-ui) | stepper | value | Height / Höhe | unit="px", step=1, precision=0, softMax=4320, snaps=[360, 480, 720, 1080, 1440, 2160] |
| any | remove-sample | `/trackIndex` | integer | step | stepper | — | Track / Spur | step=1, precision=0 |
| any | remove-sample | `/index` | integer | everything (no x-semio-ui) | stepper | — | Sample / Sample | step=1, precision=0 |
| any | insert-sample | `/trackIndex` | integer | step | stepper | — | Track / Spur | step=1, precision=0 |
| any | insert-sample | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · obj

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| geometry | insert-vertex | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| geometry | remove-vertex | `/index` | integer | everything (no x-semio-ui) | stepper | — | Vertex / Vertex | step=1, precision=0 |
| geometry | set-group | `/name` | string | everything (no x-semio-ui) | text | — | Group / Gruppe |  |
| geometry | set-group | `/faces` | numbers | everything (no x-semio-ui) | — | value | Faces / Flächen | step=1, precision=0 |
| geometry | set-vertex | `/index` | integer | everything (no x-semio-ui) | stepper | — | Vertex / Vertex | step=1, precision=0 |
| geometry | insert-normal | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| geometry | insert-normal | `/normal` | object | everything (no x-semio-ui) | — | value | Normal / Normale |  |
| geometry | set-object | `/name` | string | everything (no x-semio-ui) | text | — | Object / Objekt |  |
| geometry | set-object | `/faces` | numbers | everything (no x-semio-ui) | — | value | Faces / Flächen | step=1, precision=0 |
| geometry | set-face | `/index` | integer | everything (no x-semio-ui) | stepper | — | Face / Fläche | step=1, precision=0 |
| geometry | insert-face | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| geometry | remove-object | `/name` | string | everything (no x-semio-ui) | reference | target | Object / Objekt | ref={"kind": "object"} |
| geometry | remove-face | `/index` | integer | everything (no x-semio-ui) | stepper | — | Face / Fläche | step=1, precision=0 |
| geometry | remove-normal | `/index` | integer | everything (no x-semio-ui) | stepper | — | Normal / Normale | step=1, precision=0 |
| geometry | remove-texcoord | `/index` | integer | everything (no x-semio-ui) | stepper | — | Texture coordinate / Texturkoordinate | step=1, precision=0 |
| geometry | set-texcoord | `/index` | integer | everything (no x-semio-ui) | stepper | — | Texture coordinate / Texturkoordinate | step=1, precision=0 |
| geometry | set-normal | `/index` | integer | everything (no x-semio-ui) | stepper | — | Normal / Normale | step=1, precision=0 |
| geometry | set-normal | `/normal` | object | everything (no x-semio-ui) | — | value | Normal / Normale |  |
| geometry | insert-texcoord | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| geometry | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| geometry | remove-group | `/name` | string | everything (no x-semio-ui) | reference | target | Group / Gruppe | ref={"kind": "group"} |

## stdio · pdf

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| x | set-page-size | `/width` | number | everything (no x-semio-ui) | stepper | value | Page width / Seitenbreite | unit="pt", step=1, precision=2, softMin=72, softMax=3370, snaps=[419.53, 595.28, 612, 841.89, 1190.55] |
| x | set-page-size | `/height` | number | everything (no x-semio-ui) | stepper | value | Page height / Seitenhöhe | unit="pt", step=1, precision=2, softMin=72, softMax=4768, snaps=[595.28, 792, 841.89, 1008, 1190.55, 1683.78] |
| a | set-page-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Page text / Seitentext |  |
| base | replace-page-text | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | replace-page-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Page text / Seitentext |  |
| base | resize-page | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | resize-page | `/width` | number | everything (no x-semio-ui) | stepper | value | Page width / Seitenbreite | unit="pt", step=1, precision=2, softMin=72, softMax=3370, snaps=[419.53, 595.28, 612, 841.89, 1190.55] |
| base | resize-page | `/height` | number | everything (no x-semio-ui) | stepper | value | Page height / Seitenhöhe | unit="pt", step=1, precision=2, softMin=72, softMax=4768, snaps=[595.28, 792, 841.89, 1008, 1190.55, 1683.78] |
| base | insert-page | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | insert-page | `/page` | object | everything (no x-semio-ui) | — | value | Page / Seite |  |
| base | move-page | `/from` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | move-page | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| base | remove-page | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| ua | set-info-title | `/title` | string | everything (no x-semio-ui) | text | value | Title / Titel |  |
| ua | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| ua | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| ua | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| ua | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| h | remove-signature-field | `/name` | string | everything (no x-semio-ui) | reference | target | Signature field / Signaturfeld | ref={"kind": "field"} |
| h | insert-signature-field | `/name` | string | everything (no x-semio-ui) | text | value | Field name / Feldname |  |
| h | set-info-title | `/title` | string | everything (no x-semio-ui) | text | value | Title / Titel |  |
| h | set-info-author | `/author` | string | everything (no x-semio-ui) | text | value | Author / Autor |  |
| h | insert-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | value | JavaScript / JavaScript |  |
| h | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| h | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| h | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| h | insert-launch-action | `/target` | string | everything (no x-semio-ui) | text | value | Launch target / Startziel |  |
| h | remove-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | — | JavaScript / JavaScript |  |
| h | remove-launch-action | `/target` | string | everything (no x-semio-ui) | text | — | Launch target / Startziel |  |
| h | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| e | remove-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | — | Annotation subtype / Anmerkungsuntertyp |  |
| e | remove-media-annotation | `/title` | string | everything (no x-semio-ui) | text | — | Title / Titel |  |
| e | insert-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | value | Annotation subtype / Anmerkungsuntertyp |  |
| e | insert-media-annotation | `/title` | string | everything (no x-semio-ui) | text | value | Title / Titel |  |
| e | set-output-intent | `/identifier` | string | everything (no x-semio-ui) | text | value | Output condition identifier / Kennung der Ausgabebedingung |  |
| e | insert-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | value | JavaScript / JavaScript |  |
| e | insert-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | value | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0, softMin=0, softMax=5 |
| e | insert-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | value | Revision (R) / Revision (R) | step=1, precision=0, softMin=2, softMax=6 |
| e | remove-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | — | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0 |
| e | remove-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | — | Revision (R) / Revision (R) | step=1, precision=0 |
| e | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| e | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| e | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| e | insert-launch-action | `/target` | string | everything (no x-semio-ui) | text | value | Launch target / Startziel |  |
| e | remove-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | — | JavaScript / JavaScript |  |
| e | remove-launch-action | `/target` | string | everything (no x-semio-ui) | text | — | Launch target / Startziel |  |
| e | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| x | remove-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | — | Annotation subtype / Anmerkungsuntertyp |  |
| x | remove-media-annotation | `/title` | string | everything (no x-semio-ui) | text | — | Title / Titel |  |
| x | remove-trim-box | `/pageIndex` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| x | insert-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | value | Annotation subtype / Anmerkungsuntertyp |  |
| x | insert-media-annotation | `/title` | string | everything (no x-semio-ui) | text | value | Title / Titel |  |
| x | set-output-intent | `/identifier` | string | everything (no x-semio-ui) | text | value | Output condition identifier / Kennung der Ausgabebedingung |  |
| x | set-trim-box | `/pageIndex` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| x | insert-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | value | JavaScript / JavaScript |  |
| x | insert-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | value | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0, softMin=0, softMax=5 |
| x | insert-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | value | Revision (R) / Revision (R) | step=1, precision=0, softMin=2, softMax=6 |
| x | remove-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | — | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0 |
| x | remove-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | — | Revision (R) / Revision (R) | step=1, precision=0 |
| x | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| x | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| x | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| x | insert-launch-action | `/target` | string | everything (no x-semio-ui) | text | value | Launch target / Startziel |  |
| x | remove-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | — | JavaScript / JavaScript |  |
| x | remove-launch-action | `/target` | string | everything (no x-semio-ui) | text | — | Launch target / Startziel |  |
| x | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| a | remove-af-relationship | `/fileName` | string | everything (no x-semio-ui) | reference | target | Embedded file / Eingebettete Datei | ref={"kind": "embeddedFile"} |
| a | set-output-intent | `/identifier` | string | everything (no x-semio-ui) | text | value | Output condition identifier / Kennung der Ausgabebedingung |  |
| a | insert-embedded-file | `/fileName` | string | everything (no x-semio-ui) | text | value | File name / Dateiname |  |
| a | insert-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | value | JavaScript / JavaScript |  |
| a | insert-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | value | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0, softMin=0, softMax=5 |
| a | insert-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | value | Revision (R) / Revision (R) | step=1, precision=0, softMin=2, softMax=6 |
| a | remove-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | — | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0 |
| a | remove-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | — | Revision (R) / Revision (R) | step=1, precision=0 |
| a | set-af-relationship | `/fileName` | string | everything (no x-semio-ui) | reference | target | Embedded file / Eingebettete Datei | ref={"kind": "embeddedFile"} |
| a | set-af-relationship | `/relationship` | string | everything (no x-semio-ui) | text | value | Relationship / Beziehung |  |
| a | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| a | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| a | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| a | remove-embedded-file | `/fileName` | string | everything (no x-semio-ui) | reference | target | Embedded file / Eingebettete Datei | ref={"kind": "embeddedFile"} |
| a | insert-launch-action | `/target` | string | everything (no x-semio-ui) | text | value | Launch target / Startziel |  |
| a | remove-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | — | JavaScript / JavaScript |  |
| a | remove-launch-action | `/target` | string | everything (no x-semio-ui) | text | — | Launch target / Startziel |  |
| a | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| base | set-page-crop-box | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-page-content | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-page-content | `/content` | array | everything (no x-semio-ui) | — | value | Content operators / Inhaltsoperatoren |  |
| base | append-page-content | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | append-page-content | `/content` | array | everything (no x-semio-ui) | — | value | Content operators / Inhaltsoperatoren |  |
| base | remove-font | `/id` | string | everything (no x-semio-ui) | reference | target | Font / Schrift | ref={"kind": "font"} |
| base | remove-shading | `/id` | string | everything (no x-semio-ui) | reference | target | Shading / Schattierung | ref={"kind": "shading"} |
| base | remove-image | `/id` | string | everything (no x-semio-ui) | reference | target | Image / Bild | ref={"kind": "image"} |
| base | remove-ext-g-state | `/id` | string | everything (no x-semio-ui) | reference | target | Graphics state / Grafikzustand | ref={"kind": "extGState"} |
| base | remove-color-space | `/name` | string | everything (no x-semio-ui) | reference | target | Color space / Farbraum | ref={"kind": "colorSpace"} |
| base | remove-named-destination | `/name` | string | everything (no x-semio-ui) | reference | target | Named destination / Benanntes Ziel | ref={"kind": "namedDestination"} |
| base | set-image | `/image` | foreign | everything (no x-semio-ui) | — | value | Image / Bild |  |
| base | set-properties | `/properties` | foreign | everything (no x-semio-ui) | — | value | Property list / Eigenschaftsliste |  |
| base | set-optional-content | `/content` | foreign | everything (no x-semio-ui) | — | value | Optional content / Optionaler Inhalt |  |
| base | insert-annotation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | insert-annotation | `/at` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | remove-annotation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | remove-annotation | `/at` | integer | everything (no x-semio-ui) | stepper | — | Annotation / Anmerkung | step=1, precision=0 |
| base | set-page-user-unit | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-page-user-unit | `/userUnit` | number | step, precision, softMin | stepper | — | User unit / Benutzereinheit | step=1, precision=2, softMin=1 |
| base | set-page-media-box | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-annotation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-annotation | `/at` | integer | everything (no x-semio-ui) | stepper | — | Annotation / Anmerkung | step=1, precision=0 |
| base | insert-page | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | insert-page | `/page` | foreign | everything (no x-semio-ui) | — | value | Page / Seite |  |
| base | insert-object | `/id` | foreign | everything (no x-semio-ui) | — | — | Object / Objekt |  |
| base | insert-object | `/value` | foreign | everything (no x-semio-ui) | — | value | Object value / Objektwert |  |
| base | move-page | `/from` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | move-page | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| base | replace-content | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | replace-content | `/at` | integer | everything (no x-semio-ui) | stepper | — | Operator / Operator | step=1, precision=0 |
| base | set-page-rotation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-page-rotation | `/rotation` | integer | everything (no x-semio-ui) | dial | value | Rotation / Drehung | unit="°", step=90, precision=0, softMin=0, softMax=270, snaps=[0, 90, 180, 270] |
| base | set-dict-entry | `/id` | foreign | everything (no x-semio-ui) | — | — | Object / Objekt |  |
| base | set-dict-entry | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | set-dict-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | set-dict-entry | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| base | remove-properties | `/name` | string | everything (no x-semio-ui) | reference | target | Property list / Eigenschaftsliste | ref={"kind": "properties"} |
| base | set-object-value | `/id` | foreign | everything (no x-semio-ui) | — | — | Object / Objekt |  |
| base | set-object-value | `/value` | foreign | everything (no x-semio-ui) | — | value | Object value / Objektwert |  |
| base | insert-content | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | insert-content | `/at` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | insert-content | `/content` | array | everything (no x-semio-ui) | — | value | Content operators / Inhaltsoperatoren |  |
| base | set-page-box | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | set-catalog-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | set-catalog-entry | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| base | remove-embedded-file | `/id` | string | everything (no x-semio-ui) | reference | target | Embedded file / Eingebettete Datei | ref={"kind": "embeddedFile"} |
| base | remove-page | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | remove-form | `/id` | string | everything (no x-semio-ui) | reference | target | Form XObject / Formular-XObject | ref={"kind": "form"} |
| base | set-open-action | `/action` | foreign | everything (no x-semio-ui) | — | value | Open action / Öffnen-Aktion |  |
| base | remove-dict-entry | `/id` | foreign | everything (no x-semio-ui) | — | — | Object / Objekt |  |
| base | remove-dict-entry | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| base | remove-dict-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | set-trailer-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | set-trailer-entry | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| base | remove-object | `/id` | foreign | everything (no x-semio-ui) | — | — | Object / Objekt |  |
| base | remove-catalog-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | remove-content | `/index` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| base | remove-content | `/at` | integer | everything (no x-semio-ui) | stepper | — | First operator / Erster Operator | step=1, precision=0 |
| base | remove-content | `/count` | integer | everything (no x-semio-ui) | stepper | value | Operator count / Anzahl Operatoren | step=1, precision=0, softMin=1 |
| base | remove-trailer-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| base | remove-pattern | `/id` | string | everything (no x-semio-ui) | reference | target | Pattern / Muster | ref={"kind": "pattern"} |
| vt | remove-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | — | Annotation subtype / Anmerkungsuntertyp |  |
| vt | remove-media-annotation | `/title` | string | everything (no x-semio-ui) | text | — | Title / Titel |  |
| vt | remove-trim-box | `/pageIndex` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| vt | insert-media-annotation | `/subtype` | string | everything (no x-semio-ui) | text | value | Annotation subtype / Anmerkungsuntertyp |  |
| vt | insert-media-annotation | `/title` | string | everything (no x-semio-ui) | text | value | Title / Titel |  |
| vt | set-output-intent | `/identifier` | string | everything (no x-semio-ui) | text | value | Output condition identifier / Kennung der Ausgabebedingung |  |
| vt | set-trim-box | `/pageIndex` | integer | everything (no x-semio-ui) | stepper | — | Page / Seite | step=1, precision=0 |
| vt | insert-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | value | JavaScript / JavaScript |  |
| vt | insert-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | value | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0, softMin=0, softMax=5 |
| vt | insert-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | value | Revision (R) / Revision (R) | step=1, precision=0, softMin=2, softMax=6 |
| vt | remove-encryption-dictionary | `/version` | integer | everything (no x-semio-ui) | stepper | — | Algorithm version (V) / Algorithmusversion (V) | step=1, precision=0 |
| vt | remove-encryption-dictionary | `/revision` | integer | everything (no x-semio-ui) | stepper | — | Revision (R) / Revision (R) | step=1, precision=0 |
| vt | embed-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |
| vt | embed-font-file | `/key` | string | everything (no x-semio-ui) | text | — | Font file key / Schlüssel der Schriftdatei |  |
| vt | embed-font-file | `/program` | object | everything (no x-semio-ui) | — | — | Font program / Schriftprogramm |  |
| vt | insert-launch-action | `/target` | string | everything (no x-semio-ui) | text | value | Launch target / Startziel |  |
| vt | remove-javascript-action | `/script` | string | everything (no x-semio-ui) | multiline | — | JavaScript / JavaScript |  |
| vt | remove-launch-action | `/target` | string | everything (no x-semio-ui) | text | — | Launch target / Startziel |  |
| vt | remove-font-file | `/descriptorOrdinal` | integer | everything (no x-semio-ui) | stepper | — | Font descriptor / Schriftdeskriptor | step=1, precision=0 |

## stdio · ply

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-row-property | `/rowIndex` | integer | step | stepper | — | Row index / Zeilenindex | step=1 |
| any | set-row-property | `/value` | union | everything (no x-semio-ui) | — | value | Value / Wert |  |
| any | insert-comment | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | insert-comment | `/comment` | string | everything (no x-semio-ui) | text | value | Comment / Kommentar |  |
| any | remove-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Row / Zeile | step=1, precision=0 |
| any | insert-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | insert-row | `/row` | object | everything (no x-semio-ui) | — | value | Row / Zeile |  |
| any | remove-comment | `/index` | integer | everything (no x-semio-ui) | stepper | — | Comment / Kommentar | step=1, precision=0 |
| any | remove-element | `/name` | string | everything (no x-semio-ui) | reference | target | Element / Element | ref={"kind": "element"} |
| any | add-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | add-element | `/element` | object | everything (no x-semio-ui) | — | value | Element / Element |  |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · png

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | change-gamma | `/gama` | integer | widget, step, precision, snaps | stepper | — | PNG gamma / PNG-Gamma | step=1, precision=0, snaps=[45455, 55556, 100000] |
| any | patch-pixels | `/x` | integer | widget, unit, step, precision | stepper | — | Left / Links | unit="px", step=1, precision=0 |
| any | patch-pixels | `/y` | integer | widget, unit, step, precision | stepper | — | Top / Oben | unit="px", step=1, precision=0 |
| any | patch-pixels | `/width` | integer | widget, unit, step, precision | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| any | patch-pixels | `/height` | integer | widget, unit, step, precision | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| any | patch-pixels | `/red` | integer | widget, step, precision | slider | — | Red / Rot | step=1, precision=0 |
| any | patch-pixels | `/green` | integer | widget, step, precision | slider | — | Green / Grün | step=1, precision=0 |
| any | patch-pixels | `/blue` | integer | widget, step, precision | slider | — | Blue / Blau | step=1, precision=0 |
| any | patch-pixels | `/alpha` | integer | widget, step, precision | slider | — | Alpha / Alpha | step=1, precision=0 |
| any | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · pptx

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| transitional | set-drawing-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Drawing namespace / Zeichnungsnamensraum |  |
| transitional | set-main-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Main namespace / Hauptnamensraum |  |
| transitional | set-conformance-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Conformance class / Konformitätsklasse |  |
| transitional | set-relationship-base | `/base` | string | everything (no x-semio-ui) | text | value | Relationship base / Beziehungsbasis |  |
| strict | set-drawing-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Drawing namespace / Zeichnungsnamensraum |  |
| strict | set-main-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Main namespace / Hauptnamensraum |  |
| strict | insert-alternate-content | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| strict | set-conformance-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Conformance class / Konformitätsklasse |  |
| strict | set-relationship-base | `/base` | string | everything (no x-semio-ui) | text | value | Relationship base / Beziehungsbasis |  |
| strict | insert-vml-part | `/path` | string | everything (no x-semio-ui) | text | — | Part name / Name des Paketteils |  |
| strict | insert-vml-part | `/markup` | string | everything (no x-semio-ui) | multiline | value | VML markup / VML-Markup |  |
| strict | remove-vml-part | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| strict | remove-alternate-content | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| base | move-slide | `/destinationIndex` | integer | widget, step, precision | stepper | — | Destination position / Zielposition | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · semio

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| flow | remove-edge | `/id` | string | everything (no x-semio-ui) | reference | target | Edge / Kante | ref={"kind": "edge"} |
| flow | insert-node | `/node` | foreign | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| flow | insert-edge | `/edge` | foreign | everything (no x-semio-ui) | — | value | Edge / Kante |  |
| flow | set-node-param | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | set-node-param | `/key` | string | everything (no x-semio-ui) | text | — | Parameter / Parameter |  |
| flow | set-node-param | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| flow | set-edge-kind | `/id` | string | everything (no x-semio-ui) | reference | target | Edge / Kante | ref={"kind": "edge"} |
| flow | set-edge-kind | `/kind` | string | everything (no x-semio-ui) | text | value | Kind / Art |  |
| flow | set-node-kind | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | set-node-kind | `/kind` | string | everything (no x-semio-ui) | text | value | Kind / Art |  |
| flow | set-node-position | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | set-node-position | `/position` | foreign | everything (no x-semio-ui) | — | value | Position / Position |  |
| flow | set-edge-endpoints | `/id` | string | everything (no x-semio-ui) | reference | target | Edge / Kante | ref={"kind": "edge"} |
| flow | set-edge-endpoints | `/from` | foreign | everything (no x-semio-ui) | — | value | Source / Quelle |  |
| flow | set-edge-endpoints | `/to` | foreign | everything (no x-semio-ui) | — | value | Target / Ziel |  |
| flow | set-node-label | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | set-node-label | `/label` | string | everything (no x-semio-ui) | text | value | Label / Beschriftung |  |
| flow | remove-node | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | remove-node-param | `/id` | string | everything (no x-semio-ui) | reference | target | Node / Knoten | ref={"kind": "node"} |
| flow | remove-node-param | `/key` | string | everything (no x-semio-ui) | text | — | Parameter / Parameter |  |
| flow | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| animation | insert-timeline | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| animation | set-channel-target | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | set-channel-target | `/index` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | set-channel-target | `/target` | foreign | everything (no x-semio-ui) | — | value | Target / Ziel |  |
| animation | set-timeline-name | `/index` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | set-timeline-name | `/name` | string | everything (no x-semio-ui) | text | value | Name / Name |  |
| animation | set-channel-interpolation | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | set-channel-interpolation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | insert-channel | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | insert-channel | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| animation | insert-keyframe | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | insert-keyframe | `/channelIndex` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | insert-keyframe | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| animation | remove-keyframe | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | remove-keyframe | `/channelIndex` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | remove-keyframe | `/index` | integer | everything (no x-semio-ui) | stepper | — | Keyframe / Schlüsselbild | step=1, precision=0 |
| animation | set-keyframe-value | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | set-keyframe-value | `/channelIndex` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | set-keyframe-value | `/index` | integer | everything (no x-semio-ui) | stepper | — | Keyframe / Schlüsselbild | step=1, precision=0 |
| animation | set-keyframe-value | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| animation | set-keyframe-time | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | set-keyframe-time | `/channelIndex` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | set-keyframe-time | `/index` | integer | everything (no x-semio-ui) | stepper | — | Keyframe / Schlüsselbild | step=1, precision=0 |
| animation | set-keyframe-time | `/t` | number | step, precision | stepper | — | Time / Zeit | unit="s", step=0.01, precision=3 |
| animation | remove-channel | `/timelineIndex` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | remove-channel | `/index` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| animation | remove-timeline | `/index` | integer | everything (no x-semio-ui) | stepper | — | Timeline / Zeitleiste | step=1, precision=0 |
| animation | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| video | insert-sample | `/streamIndex` | integer | everything (no x-semio-ui) | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| video | insert-sample | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| video | insert-stream | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| video | insert-stream | `/stream` | foreign | everything (no x-semio-ui) | — | value | Stream / Datenstrom |  |
| video | set-stream-meta | `/index` | integer | everything (no x-semio-ui) | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| video | set-stream-meta | `/width` | integer | everything (no x-semio-ui) | stepper | value | Width / Breite | unit="px", step=1, precision=0, softMax=7680, snaps=[640, 1280, 1920, 2560, 3840] |
| video | set-stream-meta | `/height` | integer | everything (no x-semio-ui) | stepper | value | Height / Höhe | unit="px", step=1, precision=0, softMax=4320, snaps=[360, 480, 720, 1080, 1440, 2160] |
| video | remove-stream | `/index` | integer | everything (no x-semio-ui) | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| video | set-sample-flags | `/streamIndex` | integer | everything (no x-semio-ui) | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| video | set-sample-flags | `/index` | integer | everything (no x-semio-ui) | stepper | — | Sample / Sample | step=1, precision=0 |
| video | set-sample-flags | `/pts` | integer | step | stepper | — | Presentation time stamp / Präsentationszeitstempel (PTS) | step=1 |
| video | set-sample-flags | `/key` | boolean | everything (no x-semio-ui) | toggle | value | Sync sample / Sync-Sample |  |
| video | remove-sample | `/streamIndex` | integer | everything (no x-semio-ui) | stepper | — | Stream / Datenstrom | step=1, precision=0 |
| video | remove-sample | `/index` | integer | everything (no x-semio-ui) | stepper | — | Sample / Sample | step=1, precision=0 |
| video | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| model | remove-relation | `/id` | string | everything (no x-semio-ui) | reference | target | Relation / Relation | ref={"kind": "relation"} |
| model | set-element | `/id` | string | everything (no x-semio-ui) | reference | target | Element / Element | ref={"kind": "element"} |
| model | set-element | `/placement` | object | everything (no x-semio-ui) | — | value | Placement / Platzierung |  |
| model | insert-spatial-node | `/node` | object | everything (no x-semio-ui) | — | value | Spatial node / Raumknoten |  |
| model | set-relation | `/id` | string | everything (no x-semio-ui) | reference | target | Relation / Relation | ref={"kind": "relation"} |
| model | set-relation | `/kind` | foreign | everything (no x-semio-ui) | — | value | Kind / Art |  |
| model | set-relation | `/from` | string | everything (no x-semio-ui) | reference | value | From / Von | ref={"kind": "element"} |
| model | set-relation | `/to` | string | everything (no x-semio-ui) | reference | value | To / Nach | ref={"kind": "element"} |
| model | remove-element | `/id` | string | everything (no x-semio-ui) | reference | target | Element / Element | ref={"kind": "element"} |
| model | remove-spatial-node | `/id` | string | everything (no x-semio-ui) | reference | target | Spatial node / Raumknoten | ref={"kind": "spatialNode"} |
| model | set-spatial-node | `/id` | string | everything (no x-semio-ui) | reference | target | Spatial node / Raumknoten | ref={"kind": "spatialNode"} |
| model | set-spatial-node | `/name` | string | everything (no x-semio-ui) | text | value | Name / Name |  |
| model | set-spatial-node | `/parentId` | string | everything (no x-semio-ui) | reference | value | Parent / Übergeordneter Knoten | ref={"kind": "spatialNode"} |
| model | set-spatial-node | `/placement` | object | everything (no x-semio-ui) | — | value | Placement / Platzierung |  |
| model | insert-element | `/element` | object | everything (no x-semio-ui) | — | value | Element / Element |  |
| model | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| table | edit-cell | `/row_index` | integer | step | stepper | — | Row / Zeile | step=1 |
| table | remove-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Row / Zeile | step=1, precision=0 |
| table | create-column | `/name` | string | everything (no x-semio-ui) | text | value | Column name / Spaltenname |  |
| table | create-column | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| table | rename-column | `/name` | string | everything (no x-semio-ui) | reference | target | Column / Spalte | ref={"kind": "column"} |
| table | rename-column | `/new_name` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| table | insert-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| table | insert-row | `/row` | object | everything (no x-semio-ui) | — | value | Row / Zeile |  |
| table | reorder-columns | `/name` | string | everything (no x-semio-ui) | reference | target | Column / Spalte | ref={"kind": "column"} |
| table | reorder-columns | `/to_index` | integer | step | stepper | — | Target index / Zielindex | step=1 |
| table | reorder-rows | `/from` | integer | everything (no x-semio-ui) | stepper | — | Row / Zeile | step=1, precision=0 |
| table | reorder-rows | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| table | delete-column | `/name` | string | everything (no x-semio-ui) | reference | target | Column / Spalte | ref={"kind": "column"} |
| table | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| cad | remove-block-entity | `/blockName` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | remove-block-entity | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | set-layer | `/name` | string | everything (no x-semio-ui) | reference | target | Layer / Layer | ref={"kind": "layer"} |
| cad | set-layer | `/colorIndex` | integer | step | stepper | — | Color index / Farbindex | step=1 |
| cad | set-layer | `/visible` | boolean | everything (no x-semio-ui) | toggle | value | Visible / Sichtbar |  |
| cad | set-entity-layer | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | set-entity-layer | `/layer` | string | everything (no x-semio-ui) | reference | value | Layer / Layer | ref={"kind": "layer"} |
| cad | set-block-base-point | `/name` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | set-entity-geometry | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | set-entity-geometry | `/entity` | foreign | everything (no x-semio-ui) | — | value | Geometry / Geometrie |  |
| cad | add-entity | `/entity` | foreign | everything (no x-semio-ui) | — | value | Entity / Entität |  |
| cad | set-block-entity-geometry | `/blockName` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | set-block-entity-geometry | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | set-block-entity-geometry | `/entity` | foreign | everything (no x-semio-ui) | — | value | Geometry / Geometrie |  |
| cad | add-layer | `/layer` | foreign | everything (no x-semio-ui) | — | value | Layer / Layer |  |
| cad | remove-entity | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | remove-block | `/name` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | add-block-entity | `/blockName` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | add-block-entity | `/entity` | foreign | everything (no x-semio-ui) | — | value | Entity / Entität |  |
| cad | add-block | `/block` | foreign | everything (no x-semio-ui) | — | value | Block / Block |  |
| cad | set-block-entity-layer | `/blockName` | string | everything (no x-semio-ui) | reference | target | Block / Block | ref={"kind": "block"} |
| cad | set-block-entity-layer | `/handle` | string | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| cad | set-block-entity-layer | `/layer` | string | everything (no x-semio-ui) | reference | value | Layer / Layer | ref={"kind": "layer"} |
| cad | remove-layer | `/name` | string | everything (no x-semio-ui) | reference | target | Layer / Layer | ref={"kind": "layer"} |
| cad | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| document | set-run-style | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | set-run-style | `/run_index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| document | set-run-style | `/style` | foreign | everything (no x-semio-ui) | — | value | Run style / Zeichenformat |  |
| document | set-style-name | `/id` | string | everything (no x-semio-ui) | reference | target | Style / Formatvorlage | ref={"kind": "style"} |
| document | set-style-name | `/name` | string | everything (no x-semio-ui) | text | value | Style name / Name der Formatvorlage |  |
| document | set-image-bytes | `/id` | string | everything (no x-semio-ui) | reference | target | Image / Bild | ref={"kind": "image"} |
| document | set-heading-level | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | set-heading-level | `/level` | integer | step | stepper | — | Heading level / Überschriftenebene | step=1 |
| document | set-block-content | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | set-block-content | `/block` | foreign | everything (no x-semio-ui) | — | value | Block / Block |  |
| document | set-image-block | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | set-image-block | `/width` | number | everything (no x-semio-ui) | stepper | value | Width / Breite | step=1, precision=2, softMin=0 |
| document | set-image-block | `/height` | number | everything (no x-semio-ui) | stepper | value | Height / Höhe | step=1, precision=2, softMin=0 |
| document | set-list-ordered | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | insert-image | `/image` | object | everything (no x-semio-ui) | — | value | Image / Bild |  |
| document | set-style-based-on | `/id` | string | everything (no x-semio-ui) | reference | target | Style / Formatvorlage | ref={"kind": "style"} |
| document | insert-block | `/path` | object | everything (no x-semio-ui) | — | — | Insert position / Einfügeposition |  |
| document | insert-block | `/block` | foreign | everything (no x-semio-ui) | — | value | Block / Block |  |
| document | set-run-text | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | set-run-text | `/run_index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| document | set-run-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| document | insert-style | `/style` | object | everything (no x-semio-ui) | — | value | Style / Formatvorlage |  |
| document | remove-style | `/id` | string | everything (no x-semio-ui) | reference | target | Style / Formatvorlage | ref={"kind": "style"} |
| document | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| document | remove-block | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| document | remove-image | `/id` | string | everything (no x-semio-ui) | reference | target | Image / Bild | ref={"kind": "image"} |
| document | set-paragraph-style | `/path` | object | everything (no x-semio-ui) | — | — | Block / Block |  |
| object | create-properties | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| object | create-properties | `/target` | foreign | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| object | scale-object | `/scale` | foreign | everything (no x-semio-ui) | — | value | Scale / Skalierung |  |
| object | rotate-object | `/rotation` | foreign | everything (no x-semio-ui) | — | value | Rotation / Drehung |  |
| object | create-mesh | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| object | create-mesh | `/target` | foreign | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| object | create-brep | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| object | create-brep | `/target` | foreign | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| object | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| presentation | set-text-box-blocks | `/slide_index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | set-text-box-blocks | `/shape_index` | integer | everything (no x-semio-ui) | stepper | — | Shape / Form | step=1, precision=0 |
| presentation | insert-slide | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| presentation | remove-slide | `/index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | set-layout-master | `/id` | string | everything (no x-semio-ui) | reference | target | Layout / Layout | ref={"kind": "layout"} |
| presentation | remove-shape | `/slide_index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | remove-shape | `/shape_index` | integer | everything (no x-semio-ui) | stepper | — | Shape / Form | step=1, precision=0 |
| presentation | insert-shape | `/slide_index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | insert-shape | `/shape_index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| presentation | remove-master | `/id` | string | everything (no x-semio-ui) | reference | target | Master / Master | ref={"kind": "master"} |
| presentation | insert-layout | `/layout` | foreign | everything (no x-semio-ui) | — | value | Layout / Layout |  |
| presentation | set-slide-layout | `/index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | remove-layout | `/id` | string | everything (no x-semio-ui) | reference | target | Layout / Layout | ref={"kind": "layout"} |
| presentation | set-slide-notes | `/index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| presentation | set-shape-frame | `/slide_index` | integer | everything (no x-semio-ui) | stepper | — | Slide / Folie | step=1, precision=0 |
| presentation | set-shape-frame | `/shape_index` | integer | everything (no x-semio-ui) | stepper | — | Shape / Form | step=1, precision=0 |
| presentation | set-shape-frame | `/frame` | foreign | everything (no x-semio-ui) | — | value | Frame / Rahmen |  |
| audio | remove-tag | `/index` | integer | everything (no x-semio-ui) | stepper | — | Tag / Tag | step=1, precision=0 |
| audio | set-channel-samples | `/index` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| audio | insert-channel | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| audio | set-sample-rate | `/sampleRate` | integer | step, snaps | stepper | — | Sample rate / Abtastrate | unit="Hz", step=1, snaps=[8000, 16000, 22050, 44100, 48000, 96000] |
| audio | insert-tag | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| audio | insert-tag | `/tag` | object | everything (no x-semio-ui) | — | value | Tag / Tag |  |
| audio | set-tag-value | `/index` | integer | everything (no x-semio-ui) | stepper | — | Tag / Tag | step=1, precision=0 |
| audio | set-tag-value | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| audio | remove-channel | `/index` | integer | everything (no x-semio-ui) | stepper | — | Channel / Kanal | step=1, precision=0 |
| audio | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| value | remove-node | `/id` | foreign | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| value | remove-map-entry | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| value | remove-map-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| value | insert-list-item | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| value | insert-list-item | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| value | insert-list-item | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| value | remove-list-item | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| value | remove-list-item | `/index` | integer | everything (no x-semio-ui) | stepper | — | Item / Element | step=1, precision=0 |
| value | set-value | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| value | set-value | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| value | set-map-entry | `/path` | array | everything (no x-semio-ui) | — | — | Path / Pfad |  |
| value | set-map-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| value | set-map-entry | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| value | set-node | `/id` | foreign | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| value | set-node | `/value` | foreign | everything (no x-semio-ui) | — | value | Value / Wert |  |
| value | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| text | edit-run | `/index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | add-mark | `/run_index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | add-mark | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| text | remove-mark | `/run_index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | remove-mark | `/index` | integer | everything (no x-semio-ui) | stepper | — | Mark / Auszeichnung | step=1, precision=0 |
| text | change-run-language | `/index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | insert-run | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| text | reorder-runs | `/from` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | reorder-runs | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| text | remove-run | `/index` | integer | everything (no x-semio-ui) | stepper | — | Run / Textlauf | step=1, precision=0 |
| text | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| mesh | change-material-metallic | `/id` | string | everything (no x-semio-ui) | reference | target | Material / Material | ref={"kind": "material"} |
| mesh | change-material-metallic | `/new_metallic` | number | widget, step, precision, softMin, softMax | slider | — | Metallic / Metallizität | step=0.01, precision=2, softMin=0, softMax=1 |
| mesh | delete-primitive | `/mesh_id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | delete-primitive | `/primitive_id` | string | everything (no x-semio-ui) | reference | target | Primitive / Primitiv | ref={"kind": "primitive"} |
| mesh | change-material-base | `/id` | string | everything (no x-semio-ui) | reference | target | Material / Material | ref={"kind": "material"} |
| mesh | create-material | `/material` | foreign | everything (no x-semio-ui) | — | value | Material / Material |  |
| mesh | change-texture-mime | `/id` | string | everything (no x-semio-ui) | reference | target | Texture / Textur | ref={"kind": "texture"} |
| mesh | replace-texture-bytes | `/id` | string | everything (no x-semio-ui) | reference | target | Texture / Textur | ref={"kind": "texture"} |
| mesh | move-vertex | `/mesh_id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | move-vertex | `/primitive_id` | string | everything (no x-semio-ui) | reference | target | Primitive / Primitiv | ref={"kind": "primitive"} |
| mesh | move-vertex | `/vertex_index` | integer | step | stepper | — | Vertex index / Eckpunktindex | step=1 |
| mesh | set-primitive-topology | `/mesh_id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | set-primitive-topology | `/primitive_id` | string | everything (no x-semio-ui) | reference | target | Primitive / Primitiv | ref={"kind": "primitive"} |
| mesh | create-primitive | `/mesh_id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | create-primitive | `/primitive` | foreign | everything (no x-semio-ui) | — | value | Primitive / Primitiv |  |
| mesh | delete-texture | `/id` | string | everything (no x-semio-ui) | reference | target | Texture / Textur | ref={"kind": "texture"} |
| mesh | create-mesh | `/mesh` | foreign | everything (no x-semio-ui) | — | value | Mesh / Netz |  |
| mesh | delete-mesh | `/id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | delete-material | `/id` | string | everything (no x-semio-ui) | reference | target | Material / Material | ref={"kind": "material"} |
| mesh | change-material-roughness | `/id` | string | everything (no x-semio-ui) | reference | target | Material / Material | ref={"kind": "material"} |
| mesh | change-material-roughness | `/new_roughness` | number | widget, step, precision, softMin, softMax | slider | — | Roughness / Rauheit | step=0.01, precision=2, softMin=0, softMax=1 |
| mesh | set-primitive-material | `/mesh_id` | string | everything (no x-semio-ui) | reference | target | Mesh / Netz | ref={"kind": "mesh"} |
| mesh | set-primitive-material | `/primitive_id` | string | everything (no x-semio-ui) | reference | target | Primitive / Primitiv | ref={"kind": "primitive"} |
| mesh | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| graph | delete-edge | `/id` | object | everything (no x-semio-ui) | — | — | Edge / Kante |  |
| graph | delete-edge | `/id/value` | string | everything (no x-semio-ui) | reference | target | Edge id / Kanten-ID | ref={"kind": "edge", "domain": "graph", "granularity": "edge"} |
| graph | add-node-property | `/node_id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | add-node-property | `/node_id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | add-node-property | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| graph | remove-node-property | `/node_id` | object | nothing | — | — | Node / Knoten |  |
| graph | remove-node-property | `/node_id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | remove-node-property | `/key` | string | nothing | text | — | Property / Eigenschaft |  |
| graph | create-edge | `/id` | object | everything (no x-semio-ui) | — | — | Edge id / Kanten-ID |  |
| graph | create-edge | `/id/value` | string | everything (no x-semio-ui) | text | value | Id / ID |  |
| graph | create-edge | `/source` | object | everything (no x-semio-ui) | — | — | Source node / Quellknoten |  |
| graph | create-edge | `/source/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | create-edge | `/target` | object | everything (no x-semio-ui) | — | — | Target node / Zielknoten |  |
| graph | create-edge | `/target/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | create-edge | `/kind` | string | everything (no x-semio-ui) | text | value | Kind / Art |  |
| graph | create-edge | `/label` | string | everything (no x-semio-ui) | text | value | Label / Beschriftung |  |
| graph | create-edge | `/properties` | array | everything (no x-semio-ui) | — | value | Properties / Eigenschaften |  |
| graph | create-edge | `/source_port` | string | everything (no x-semio-ui) | text | value | Source port / Quellanschluss |  |
| graph | create-edge | `/target_port` | string | everything (no x-semio-ui) | text | value | Target port / Zielanschluss |  |
| graph | create-node | `/id` | object | everything (no x-semio-ui) | — | — | Node id / Knoten-ID |  |
| graph | create-node | `/id/value` | string | everything (no x-semio-ui) | text | value | Id / ID |  |
| graph | create-node | `/kind` | string | everything (no x-semio-ui) | text | value | Kind / Art |  |
| graph | create-node | `/label` | string | everything (no x-semio-ui) | text | value | Label / Beschriftung |  |
| graph | create-node | `/position` | object | everything (no x-semio-ui) | — | value | Position / Position |  |
| graph | create-node | `/position/x` | number | everything (no x-semio-ui) | stepper | value | X / X | step=1, precision=2, snapSource={"config": "gridFactor"} |
| graph | create-node | `/position/y` | number | everything (no x-semio-ui) | stepper | value | Y / Y | step=1, precision=2, snapSource={"config": "gridFactor"} |
| graph | create-node | `/width` | number | everything (no x-semio-ui) | stepper | value | Width / Breite | step=1, precision=2, softMin=0, snapSource={"config": "gridFactor"} |
| graph | create-node | `/height` | number | everything (no x-semio-ui) | stepper | value | Height / Höhe | step=1, precision=2, softMin=0, snapSource={"config": "gridFactor"} |
| graph | create-node | `/ports` | array | everything (no x-semio-ui) | — | value | Ports / Anschlüsse |  |
| graph | create-node | `/ports/-/kind` | enum | everything (no x-semio-ui) | segmented | — | Direction / Richtung |  |
| graph | create-node | `/ports/-/category` | string | everything (no x-semio-ui) | text | — | Category / Kategorie |  |
| graph | create-node | `/properties` | array | everything (no x-semio-ui) | — | value | Properties / Eigenschaften |  |
| graph | move-node | `/id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | move-node | `/id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | add-node-port | `/node_id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | add-node-port | `/node_id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | add-node-port | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| graph | add-node-port | `/port/category` | string | everything (no x-semio-ui) | text | — | Category / Kategorie |  |
| graph | remove-node-port | `/node_id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | remove-node-port | `/node_id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | remove-node-port | `/index` | integer | everything (no x-semio-ui) | stepper | — | Port / Anschluss | step=1, precision=0 |
| graph | change-node-kind | `/id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | change-node-kind | `/id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | change-node-label | `/id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | change-node-label | `/id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | delete-node | `/id` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| graph | delete-node | `/id/value` | string | everything (no x-semio-ui) | reference | target | Node id / Knoten-ID | ref={"kind": "node", "domain": "graph", "granularity": "node"} |
| graph | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| drawing | create-node | `/parent` | object | everything (no x-semio-ui) | — | — | Parent / Übergeordneter Knoten |  |
| drawing | create-node | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| drawing | create-node | `/node` | foreign | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| drawing | delete-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | create-layer | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| drawing | create-layer | `/layer` | foreign | everything (no x-semio-ui) | — | value | Layer / Ebene |  |
| drawing | unflatten-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | ungroup-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | move-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | scale-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | change-stroke-width | `/style_name` | string | everything (no x-semio-ui) | reference | target | Style / Stil | ref={"kind": "style"} |
| drawing | change-stroke-width | `/new_width` | number | step, precision, softMin | stepper | — | Stroke width / Konturstärke | step=0.25, precision=2, softMin=0 |
| drawing | reorder-nodes | `/parent` | object | everything (no x-semio-ui) | — | — | Parent / Übergeordneter Knoten |  |
| drawing | reorder-nodes | `/from` | integer | everything (no x-semio-ui) | stepper | — | Node / Knoten | step=1, precision=0 |
| drawing | reorder-nodes | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| drawing | rotate-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | change-stroke | `/style_name` | string | everything (no x-semio-ui) | reference | target | Style / Stil | ref={"kind": "style"} |
| drawing | drag-nodes | `/offset` | foreign | everything (no x-semio-ui) | — | value | Offset / Versatz |  |
| drawing | delete-layer | `/id` | string | everything (no x-semio-ui) | reference | target | Layer / Ebene | ref={"kind": "layer"} |
| drawing | replace-path | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| drawing | group-nodes | `/parent` | object | everything (no x-semio-ui) | — | — | Parent / Übergeordneter Knoten |  |
| drawing | group-nodes | `/indices` | numbers | everything (no x-semio-ui) | — | — | Nodes / Knoten | step=1, precision=0 |
| drawing | group-nodes | `/transform` | object | everything (no x-semio-ui) | — | value | Group transform / Gruppentransformation |  |
| drawing | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| drawing | replace-fill | `/style_name` | string | everything (no x-semio-ui) | reference | target | Style / Stil | ref={"kind": "style"} |
| drawing | flatten-node | `/at` | object | everything (no x-semio-ui) | — | — | Node / Knoten |  |
| image | set-frame-delay | `/index` | integer | everything (no x-semio-ui) | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| image | set-frame-delay | `/delay_ms` | integer | step | stepper | — | Frame delay / Bildverzögerung | unit="ms", step=1 |
| image | insert-frame | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| image | insert-frame | `/frame` | object | everything (no x-semio-ui) | — | value | Frame / Einzelbild |  |
| image | set-metadata-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| image | set-metadata-entry | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| image | set-dimensions | `/width` | integer | everything (no x-semio-ui) | stepper | value | Width / Breite | unit="px", step=1, precision=0, softMax=16384 |
| image | set-dimensions | `/height` | integer | everything (no x-semio-ui) | stepper | value | Height / Höhe | unit="px", step=1, precision=0, softMax=16384 |
| image | move-frame | `/from` | integer | everything (no x-semio-ui) | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| image | move-frame | `/to` | integer | everything (no x-semio-ui) | stepper | — | Destination / Zielposition | step=1, precision=0 |
| image | set-bit-depth | `/bit_depth` | integer | step, snaps | stepper | — | Bit depth / Bittiefe | unit="bit", step=1, snaps=[1, 2, 4, 8, 16] |
| image | set-frame-pixels | `/index` | integer | everything (no x-semio-ui) | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| image | remove-metadata-entry | `/key` | string | everything (no x-semio-ui) | text | — | Key / Schlüssel |  |
| image | remove-frame | `/index` | integer | everything (no x-semio-ui) | stepper | — | Frame / Einzelbild | step=1, precision=0 |
| image | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| brep | delete-edge | `/id` | string | everything (no x-semio-ui) | reference | target | Edge / Kante | ref={"kind": "edge"} |
| brep | create-vertex | `/id` | string | everything (no x-semio-ui) | text | value | Vertex id / Vertex-ID |  |
| brep | create-vertex | `/point` | object | everything (no x-semio-ui) | — | value | Point / Punkt |  |
| brep | create-vertex | `/tol` | number | everything (no x-semio-ui) | slider | value | Tolerance / Toleranz | step=1e-09, precision=9, softMin=1e-09, softMax=0.1, snaps=[1e-09, 1e-08, 1e-07, 1e-06, 1e-05, 0.0001, 0.001, 0.01, 0.1], scale="log" |
| brep | create-shell | `/id` | string | everything (no x-semio-ui) | text | value | Shell id / Schalen-ID |  |
| brep | create-shell | `/faces` | array | everything (no x-semio-ui) | — | value | Faces / Flächen |  |
| brep | delete-shell | `/id` | string | everything (no x-semio-ui) | reference | target | Shell / Schale | ref={"kind": "shell"} |
| brep | create-face | `/id` | string | everything (no x-semio-ui) | text | value | Face id / Flächen-ID |  |
| brep | create-face | `/tol` | number | everything (no x-semio-ui) | slider | value | Tolerance / Toleranz | step=1e-09, precision=9, softMin=1e-09, softMax=0.1, snaps=[1e-09, 1e-08, 1e-07, 1e-06, 1e-05, 0.0001, 0.001, 0.01, 0.1], scale="log" |
| brep | delete-solid | `/id` | string | everything (no x-semio-ui) | reference | target | Solid / Körper | ref={"kind": "solid"} |
| brep | create-edge | `/id` | string | everything (no x-semio-ui) | text | value | Edge id / Kanten-ID |  |
| brep | create-edge | `/tol` | number | everything (no x-semio-ui) | slider | value | Tolerance / Toleranz | step=1e-09, precision=9, softMin=1e-09, softMax=0.1, snaps=[1e-09, 1e-08, 1e-07, 1e-06, 1e-05, 0.0001, 0.001, 0.01, 0.1], scale="log" |
| brep | delete-vertex | `/id` | string | everything (no x-semio-ui) | reference | target | Vertex / Vertex | ref={"kind": "vertex"} |
| brep | delete-face | `/id` | string | everything (no x-semio-ui) | reference | target | Face / Fläche | ref={"kind": "face"} |
| brep | create-solid | `/id` | string | everything (no x-semio-ui) | text | value | Solid id / Körper-ID |  |
| brep | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| kit | unbind-representation | `/index` | integer | everything (no x-semio-ui) | stepper | — | Representation / Repräsentation | step=1, precision=0 |
| kit | rename-type | `/id` | string | everything (no x-semio-ui) | reference | target | Type / Typ | ref={"kind": "type"} |
| kit | rename-type | `/new_name` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| kit | add-type | `/id` | string | everything (no x-semio-ui) | text | value | Type id / Typ-ID |  |
| kit | add-type | `/name` | string | everything (no x-semio-ui) | text | value | Name / Name |  |
| kit | remove-type | `/id` | string | everything (no x-semio-ui) | reference | target | Type / Typ | ref={"kind": "type"} |
| kit | add-design | `/id` | string | everything (no x-semio-ui) | text | value | Design id / Entwurfs-ID |  |
| kit | add-design | `/name` | string | everything (no x-semio-ui) | text | value | Name / Name |  |
| kit | create-object | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| kit | create-object | `/target` | any | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| kit | create-model | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| kit | create-model | `/target` | any | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| kit | create-properties | `/child_id` | string | everything (no x-semio-ui) | text | value | Child id / Kind-ID |  |
| kit | create-properties | `/target` | any | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| kit | delete-model | `/child_id` | string | everything (no x-semio-ui) | reference | target | Child / Kind | ref={"kind": "child"} |
| kit | change-representation-pin | `/index` | integer | everything (no x-semio-ui) | stepper | — | Representation / Repräsentation | step=1, precision=0 |
| kit | edit-design | `/id` | string | everything (no x-semio-ui) | reference | target | Design / Entwurf | ref={"kind": "design"} |
| kit | remove-design | `/id` | string | everything (no x-semio-ui) | reference | target | Design / Entwurf | ref={"kind": "design"} |
| kit | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| kit | delete-object | `/child_id` | string | everything (no x-semio-ui) | reference | target | Child / Kind | ref={"kind": "child"} |
| kit | bind-representation | `/target` | foreign | everything (no x-semio-ui) | — | value | Artifact / Artefakt |  |
| kit | bind-representation | `/role` | string | everything (no x-semio-ui) | text | value | Role / Rolle |  |

## stdio · step

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| 1️⃣cc1 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 1️⃣cc1 | remove-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 1️⃣cc1 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| 2️⃣cc2 | demote-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 2️⃣cc2 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 2️⃣cc2 | set-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 2️⃣cc2 | set-shape-representation | `/representation` | object | everything (no x-semio-ui) | — | value | Representation / Repräsentation |  |
| 2️⃣cc2 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| 3️⃣cc3 | demote-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 3️⃣cc3 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 3️⃣cc3 | set-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 3️⃣cc3 | set-shape-representation | `/representation` | object | everything (no x-semio-ui) | — | value | Representation / Repräsentation |  |
| 3️⃣cc3 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| 4️⃣cc4 | demote-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 4️⃣cc4 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 4️⃣cc4 | set-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 4️⃣cc4 | set-shape-representation | `/representation` | object | everything (no x-semio-ui) | — | value | Representation / Repräsentation |  |
| 4️⃣cc4 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| 5️⃣cc5 | demote-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 5️⃣cc5 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 5️⃣cc5 | set-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 5️⃣cc5 | set-shape-representation | `/representation` | object | everything (no x-semio-ui) | — | value | Representation / Repräsentation |  |
| 5️⃣cc5 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| 6️⃣cc6 | set-file-schema | `/schemas` | ids | everything (no x-semio-ui) | — | value | Schema identifiers / Schemakennungen |  |
| 6️⃣cc6 | set-shape-representation | `/id` | integer | everything (no x-semio-ui) | reference | target | Shape representation / Formrepräsentation | ref={"kind": "entity"} |
| 6️⃣cc6 | set-shape-representation | `/representation` | object | everything (no x-semio-ui) | — | value | Representation / Repräsentation |  |
| 6️⃣cc6 | set-product-identity | `/identity` | object | everything (no x-semio-ui) | — | value | Product identity / Produktidentität |  |
| base | set-entity-name | `/id` | integer | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| base | set-entity-name | `/name` | string | everything (no x-semio-ui) | text | value | Entity type / Entitätstyp |  |
| base | insert-entity-arg | `/id` | integer | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| base | insert-entity-arg | `/argIndex` | integer | step | stepper | — | Argument position / Argumentposition | step=1, precision=0 |
| base | insert-entity-arg | `/value` | union | everything (no x-semio-ui) | — | value | Argument value / Argumentwert |  |
| base | remove-entity-arg | `/id` | integer | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| base | remove-entity-arg | `/argIndex` | integer | step | stepper | — | Argument position / Argumentposition | step=1, precision=0 |
| base | set-entity-arg | `/id` | integer | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| base | set-entity-arg | `/argIndex` | integer | step | stepper | — | Argument position / Argumentposition | step=1, precision=0 |
| base | set-entity-arg | `/value` | union | everything (no x-semio-ui) | — | value | Argument value / Argumentwert |  |
| base | remove-entity | `/id` | integer | everything (no x-semio-ui) | reference | target | Entity / Entität | ref={"kind": "entity"} |
| base | insert-entity | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| base | insert-entity | `/entity` | object | everything (no x-semio-ui) | — | value | Entity / Entität |  |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · stl

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | insert-triangle | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | remove-triangle | `/index` | integer | everything (no x-semio-ui) | stepper | — | Triangle / Dreieck | step=1, precision=0 |
| any | set-solid-name | `/name` | string | everything (no x-semio-ui) | text | value | Solid name / Körpername |  |
| any | set-triangle-vertices | `/index` | integer | everything (no x-semio-ui) | stepper | — | Triangle / Dreieck | step=1, precision=0 |
| any | set-triangle-normal | `/index` | integer | everything (no x-semio-ui) | stepper | — | Triangle / Dreieck | step=1, precision=0 |
| any | set-triangle-normal | `/normal` | vector | everything (no x-semio-ui) | vector | value | Normal / Normale | step=0.01, precision=6 |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · svg

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| tiny | set-text | `/path` | numbers | everything (no x-semio-ui) | — | — | Text path / Textpfad | step=1, precision=0 |
| tiny | set-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| tiny | insert-tiny-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| tiny | insert-tiny-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child position / Kindposition | step=1, precision=0 |
| tiny | insert-tiny-element | `/node` | union | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| tiny | remove-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| tiny | remove-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child / Kindknoten | step=1, precision=0 |
| tiny | set-tiny-attribute | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| tiny | set-tiny-attribute | `/name` | string | everything (no x-semio-ui) | text | — | Attribute / Attribut |  |
| tiny | set-tiny-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| tiny | set-transform | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| tiny | set-transform | `/transform` | array | everything (no x-semio-ui) | — | value | Transform / Transformation |  |
| tiny | set-view-box | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| tiny | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| tiny | stamp-base-profile | `/version` | string | everything (no x-semio-ui) | text | value | SVG version / SVG-Version |  |
| basic | set-clip-path-reference | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| basic | set-text | `/path` | numbers | everything (no x-semio-ui) | — | — | Text path / Textpfad | step=1, precision=0 |
| basic | set-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| basic | insert-basic-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| basic | insert-basic-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child position / Kindposition | step=1, precision=0 |
| basic | insert-basic-element | `/node` | union | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| basic | remove-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| basic | remove-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child / Kindknoten | step=1, precision=0 |
| basic | set-basic-attribute | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| basic | set-basic-attribute | `/name` | string | everything (no x-semio-ui) | text | — | Attribute / Attribut |  |
| basic | set-basic-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| basic | insert-clip-path-shape | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| basic | insert-clip-path-shape | `/node` | union | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| basic | set-transform | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| basic | set-transform | `/transform` | array | everything (no x-semio-ui) | — | value | Transform / Transformation |  |
| basic | set-view-box | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| basic | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| basic | stamp-base-profile | `/version` | string | everything (no x-semio-ui) | text | value | SVG version / SVG-Version |  |
| base | set-text | `/path` | numbers | everything (no x-semio-ui) | — | — | Text path / Textpfad | step=1, precision=0 |
| base | set-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| base | set-attribute | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| base | set-attribute | `/name` | string | everything (no x-semio-ui) | text | — | Attribute / Attribut |  |
| base | set-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| base | set-doctype | `/doctype` | object | everything (no x-semio-ui) | — | value | Document type / Dokumenttyp |  |
| base | insert-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| base | insert-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child position / Kindposition | step=1, precision=0 |
| base | insert-element | `/node` | union | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| base | set-transform | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| base | set-transform | `/transform` | array | everything (no x-semio-ui) | — | value | Transform / Transformation |  |
| base | set-element-name | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| base | set-element-name | `/name` | string | everything (no x-semio-ui) | text | value | Element name / Elementname |  |
| base | set-view-box | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| base | remove-element | `/parent` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| base | remove-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child / Kindknoten | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · tiff

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| baseline | set-photometric-interpretation | `/photometric` | integer | step | stepper | — | Photometric interpretation / Photometrische Interpretation | step=1 |
| baseline | set-compression | `/compression` | integer | step | stepper | — | Compression / Kompression | step=1 |
| baseline | insert-tile-tags | `/tileWidth` | integer | step | stepper | — | Tile width / Kachelbreite | unit="px", step=16 |
| baseline | insert-tile-tags | `/tileLength` | integer | step | stepper | — | Tile length / Kachelhöhe | unit="px", step=16 |
| baseline | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| document | paint-region | `/ifdIndex` | integer | widget, step, precision | stepper | — | Image page / Bildseite | step=1, precision=0 |
| document | paint-region | `/x` | integer | widget, unit, step, precision | stepper | — | Left / Links | unit="px", step=1, precision=0 |
| document | paint-region | `/y` | integer | widget, unit, step, precision | stepper | — | Top / Oben | unit="px", step=1, precision=0 |
| document | paint-region | `/width` | integer | widget, unit, step, precision | stepper | — | Width / Breite | unit="px", step=1, precision=0 |
| document | paint-region | `/height` | integer | widget, unit, step, precision | stepper | — | Height / Höhe | unit="px", step=1, precision=0 |
| document | paint-region | `/red` | integer | widget, step, precision | slider | — | Red / Rot | step=1, precision=0 |
| document | paint-region | `/green` | integer | widget, step, precision | slider | — | Green / Grün | step=1, precision=0 |
| document | paint-region | `/blue` | integer | widget, step, precision | slider | — | Blue / Blau | step=1, precision=0 |
| document | paint-region | `/alpha` | integer | widget, step, precision | slider | — | Alpha / Alpha | step=1, precision=0 |
| document | replace-tag | `/ifdIndex` | integer | step | stepper | — | IFD index / IFD-Index | step=1 |
| document | replace-tag | `/tag` | integer | everything (no x-semio-ui) | reference | target | Tag / Tag | ref={"kind": "tag"} |
| document | replace-tag | `/values` | foreign | everything (no x-semio-ui) | — | value | Values / Werte |  |
| document | remove-ifd | `/index` | integer | everything (no x-semio-ui) | stepper | — | Image file directory / Bildverzeichnis (IFD) | step=1, precision=0 |
| document | insert-ifd | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| document | remove-tag | `/ifdIndex` | integer | step | stepper | — | IFD index / IFD-Index | step=1 |
| document | remove-tag | `/tag` | integer | everything (no x-semio-ui) | reference | target | Tag / Tag | ref={"kind": "tag"} |
| document | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · tsv

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | insert-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | insert-row | `/row` | ids | everything (no x-semio-ui) | — | value | Fields / Felder |  |
| any | remove-row | `/index` | integer | everything (no x-semio-ui) | stepper | — | Record / Datensatz | step=1, precision=0 |
| any | set-cell | `/rowIndex` | integer | step | stepper | — | Record index / Datensatzindex | step=1 |
| any | set-cell | `/fieldIndex` | integer | step | stepper | — | Field index / Feldindex | step=1 |
| any | set-cell | `/value` | string | everything (no x-semio-ui) | text | value | Field value / Feldwert |  |
| any | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · txt

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-trailing-newline | `/value` | boolean | everything (no x-semio-ui) | toggle | value | Trailing newline / Abschließender Zeilenumbruch |  |
| any | set-line | `/index` | integer | everything (no x-semio-ui) | stepper | — | Line / Zeile | step=1, precision=0 |
| any | set-line | `/text` | string | everything (no x-semio-ui) | text | value | Line text / Zeilentext |  |
| any | insert-line | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| any | insert-line | `/text` | string | everything (no x-semio-ui) | text | value | Line text / Zeilentext |  |
| any | remove-line | `/index` | integer | everything (no x-semio-ui) | stepper | — | Line / Zeile | step=1, precision=0 |

## stdio · wav

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | patch-data | `/index` | integer | everything (no x-semio-ui) | stepper | — | Start sample / Start-Sample | step=1, precision=0 |
| any | patch-data | `/removeCount` | integer | step | stepper | — | Remove count / Anzahl zu entfernender Elemente | step=1 |
| any | patch-data | `/data` | foreign | everything (no x-semio-ui) | hidden | value | Inserted samples / Eingefügte Samples |  |
| any | patch-data | `/moveTo` | integer | step | stepper | — | Move to index / Verschieben nach Index | step=1 |
| any | patch-snapshot | `/patch` | foreign | everything (no x-semio-ui) | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · xlsx

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| transitional | set-conformance-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Conformance class / Konformitätsklasse |  |
| transitional | set-main-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Main namespace / Hauptnamensraum |  |
| transitional | set-worksheet-content-type | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| transitional | set-worksheet-content-type | `/content_type` | string | everything (no x-semio-ui) | text | value | Content type / Inhaltstyp |  |
| transitional | set-relationships-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Relationships namespace / Beziehungsnamensraum |  |
| strict | set-conformance-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Conformance class / Konformitätsklasse |  |
| strict | insert-vml-part | `/path` | string | everything (no x-semio-ui) | text | — | Part name / Name des Paketteils |  |
| strict | insert-vml-part | `/markup` | string | everything (no x-semio-ui) | multiline | value | VML markup / VML-Markup |  |
| strict | set-main-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Main namespace / Hauptnamensraum |  |
| strict | set-worksheet-content-type | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| strict | set-worksheet-content-type | `/content_type` | string | everything (no x-semio-ui) | text | value | Content type / Inhaltstyp |  |
| strict | set-relationships-namespace | `/namespace` | string | everything (no x-semio-ui) | text | value | Relationships namespace / Beziehungsnamensraum |  |
| strict | remove-vml-part | `/path` | string | everything (no x-semio-ui) | reference | target | Part / Paketteil | ref={"kind": "part"} |
| base | set-cell | `/address` | foreign | everything (no x-semio-ui) | — | — | Cell / Zelle |  |
| base | set-cell | `/value` | union | everything (no x-semio-ui) | — | value | Cell value / Zellwert |  |
| base | insert-sheet | `/sheet` | object | everything (no x-semio-ui) | — | value | Sheet / Arbeitsblatt |  |
| base | insert-cell | `/address` | foreign | everything (no x-semio-ui) | — | — | Cell / Zelle |  |
| base | insert-cell | `/value` | union | everything (no x-semio-ui) | — | value | Cell value / Zellwert |  |
| base | remove-sheet | `/name` | string | everything (no x-semio-ui) | reference | target | Sheet / Arbeitsblatt | ref={"kind": "sheet"} |
| base | rename-sheet | `/name` | string | everything (no x-semio-ui) | reference | target | Sheet / Arbeitsblatt | ref={"kind": "sheet"} |
| base | rename-sheet | `/newName` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| base | remove-shared-string | `/index` | integer | everything (no x-semio-ui) | stepper | — | Shared string / Gemeinsame Zeichenfolge | step=1, precision=0 |
| base | insert-shared-string | `/value` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| base | set-shared-string | `/index` | integer | everything (no x-semio-ui) | stepper | — | Shared string / Gemeinsame Zeichenfolge | step=1, precision=0 |
| base | set-shared-string | `/value` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| base | remove-cell | `/address` | foreign | everything (no x-semio-ui) | — | — | Cell / Zelle |  |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · xml

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| valid | set-text | `/path` | numbers | everything (no x-semio-ui) | — | — | Text path / Textpfad | step=1, precision=0 |
| valid | set-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| valid | rename-document-element | `/name` | string | everything (no x-semio-ui) | text | value | Element name / Elementname |  |
| valid | declare-entity | `/index` | integer | everything (no x-semio-ui) | stepper | — | Insert position / Einfügeposition | step=1, precision=0 |
| valid | declare-entity | `/name` | string | everything (no x-semio-ui) | text | value | Entity name / Entitätsname |  |
| valid | declare-entity | `/value` | string | everything (no x-semio-ui) | text | value | Replacement text / Ersetzungstext |  |
| valid | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |
| base | set-text | `/path` | numbers | everything (no x-semio-ui) | — | — | Text path / Textpfad | step=1, precision=0 |
| base | set-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Text / Text |  |
| base | set-attribute | `/path` | numbers | everything (no x-semio-ui) | — | — | Element path / Elementpfad | step=1, precision=0 |
| base | set-attribute | `/name` | string | everything (no x-semio-ui) | text | — | Attribute / Attribut |  |
| base | set-attribute | `/value` | string | everything (no x-semio-ui) | text | value | Value / Wert |  |
| base | set-doctype | `/doctype` | object | everything (no x-semio-ui) | — | value | Document type / Dokumenttyp |  |
| base | insert-element | `/path` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| base | insert-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child position / Kindposition | step=1, precision=0 |
| base | insert-element | `/node` | union | everything (no x-semio-ui) | — | value | Node / Knoten |  |
| base | remove-element | `/path` | numbers | everything (no x-semio-ui) | — | — | Parent path / Elternpfad | step=1, precision=0 |
| base | remove-element | `/index` | integer | everything (no x-semio-ui) | stepper | — | Child / Kindknoten | step=1, precision=0 |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## stdio · zip

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| iso21320 | remove-entry | `/name` | string | everything (no x-semio-ui) | reference | target | Entry / Eintrag | ref={"kind": "entry"} |
| iso21320 | rename-entry | `/name` | string | everything (no x-semio-ui) | reference | target | Entry / Eintrag | ref={"kind": "entry"} |
| iso21320 | rename-entry | `/newName` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| iso21320 | set-archive-comment | `/comment` | string | everything (no x-semio-ui) | multiline | value | Archive comment / Archivkommentar |  |
| iso21320 | add-stored-entry | `/entry` | foreign | everything (no x-semio-ui) | — | value | Entry / Eintrag |  |
| iso21320 | add-stored-entry | `/before` | string | everything (no x-semio-ui) | reference | target | Insert before / Einfügen vor | ref={"kind": "entry"} |
| iso21320 | add-deflated-entry | `/entry` | foreign | everything (no x-semio-ui) | — | value | Entry / Eintrag |  |
| iso21320 | add-deflated-entry | `/before` | string | everything (no x-semio-ui) | reference | target | Insert before / Einfügen vor | ref={"kind": "entry"} |
| base | add-entry | `/entry` | foreign | everything (no x-semio-ui) | — | value | Entry / Eintrag |  |
| base | add-entry | `/before` | string | everything (no x-semio-ui) | reference | target | Insert before / Einfügen vor | ref={"kind": "entry"} |
| base | remove-entry | `/name` | string | everything (no x-semio-ui) | reference | target | Entry / Eintrag | ref={"kind": "entry"} |
| base | rename-entry | `/name` | string | everything (no x-semio-ui) | reference | target | Entry / Eintrag | ref={"kind": "entry"} |
| base | rename-entry | `/newName` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |
| base | set-archive-comment | `/comment` | string | everything (no x-semio-ui) | multiline | value | Archive comment / Archivkommentar |  |
| base | patch-snapshot | `/patch` | foreign | nothing | — | — | Snapshot edit / Änderung der Momentaufnahme |  |

## trinity · jack

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-camera | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-camera | `/camera` | object | everything (no x-semio-ui) | — | value | Camera / Kamera |  |
| any | set-camera | `/camera/x` | number | everything (no x-semio-ui) | stepper | value | Pan X / Verschiebung X | step=1, precision=1 |
| any | set-camera | `/camera/y` | number | everything (no x-semio-ui) | stepper | value | Pan Y / Verschiebung Y | step=1, precision=1 |
| any | set-camera | `/camera/zoom` | number | everything (no x-semio-ui) | slider | value | Zoom / Zoom | step=0.05, precision=2, softMin=0.1, softMax=8, snaps=[0.25, 0.5, 1, 2, 4], scale="log" |
| any | set-lod-mode | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-lod-mode | `/value` | string | everything (no x-semio-ui) | text | value | Detail level / Detailstufe |  |
| any | replace-query-result | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-query | `/value` | string | everything (no x-semio-ui) | multiline | value | Query / Abfrage |  |

## trinity · rewriting

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-camera | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-camera | `/camera` | object | everything (no x-semio-ui) | — | value | Camera / Kamera |  |
| any | set-camera | `/camera/x` | number | everything (no x-semio-ui) | stepper | value | Pan X / Verschiebung X | step=1, precision=1 |
| any | set-camera | `/camera/y` | number | everything (no x-semio-ui) | stepper | value | Pan Y / Verschiebung Y | step=1, precision=1 |
| any | set-camera | `/camera/zoom` | number | everything (no x-semio-ui) | slider | value | Zoom / Zoom | step=0.05, precision=2, softMin=0.1, softMax=8, snaps=[0.25, 0.5, 1, 2, 4], scale="log" |
| any | set-lod-mode | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-lod-mode | `/value` | string | everything (no x-semio-ui) | text | value | Detail level / Detailstufe |  |
| any | edit-lhs | `/newLhs` | foreign | everything (no x-semio-ui) | — | value | Left-hand side / Linke Regelseite |  |
| any | edit-rhs | `/newRhs` | foreign | everything (no x-semio-ui) | — | value | Right-hand side / Rechte Regelseite |  |
| any | change-rule-layout | `/key` | string | everything (no x-semio-ui) | text | — | Layout point / Layoutpunkt |  |
| any | change-rule-layout | `/newPoint/x` | number | everything (no x-semio-ui) | stepper | value | X / X | step=1, precision=2 |
| any | change-rule-layout | `/newPoint/y` | number | everything (no x-semio-ui) | stepper | value | Y / Y | step=1, precision=2 |
| any | change-parameter | `/key` | string | everything (no x-semio-ui) | text | — | Parameter / Parameter |  |
| any | remove-rule-layout | `/key` | string | everything (no x-semio-ui) | text | — | Layout point / Layoutpunkt |  |
| any | remove-parameter-binding | `/key` | string | everything (no x-semio-ui) | text | — | Parameter / Parameter |  |
| (document) | (shared schema) | `/properties/lhs/properties/pattern` | object | everything (no x-semio-ui) | — | — | Pattern / Muster |  |
| (document) | (shared schema) | `/properties/lhs/properties/whereClause` | string | everything (no x-semio-ui) | multiline | — | Where clause / Where-Klausel |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/leftVar` | string | everything (no x-semio-ui) | text | — | Left variable / Linke Variable |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/leftKind` | string | everything (no x-semio-ui) | text | — | Left node kind / Linke Knotenart |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/edgeVar` | string | everything (no x-semio-ui) | text | — | Edge variable / Kantenvariable |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/edgeKind` | string | everything (no x-semio-ui) | text | — | Edge kind / Kantenart |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/rightVar` | string | everything (no x-semio-ui) | text | — | Right variable / Rechte Variable |  |
| (document) | (shared schema) | `/$defs/Pattern/properties/rightKind` | string | everything (no x-semio-ui) | text | — | Right node kind / Rechte Knotenart |  |
| (document) | (shared schema) | `/properties/rhs/properties/create` | array | everything (no x-semio-ui) | — | — | Create / Erzeugen |  |
| (document) | (shared schema) | `/properties/rhs/properties/delete` | ids | everything (no x-semio-ui) | — | — | Delete / Löschen |  |
| (document) | (shared schema) | `/properties/rhs/properties/set` | array | everything (no x-semio-ui) | — | — | Set / Setzen |  |
| (document) | (shared schema) | `/properties/rhs/properties/set/items/properties/var` | string | everything (no x-semio-ui) | text | — | Variable / Variable |  |
| (document) | (shared schema) | `/properties/rhs/properties/set/items/properties/prop` | string | everything (no x-semio-ui) | text | — | Property / Eigenschaft |  |
| (document) | (shared schema) | `/properties/rhs/properties/merge` | array | everything (no x-semio-ui) | — | — | Merge / Zusammenführen |  |
| (document) | (shared schema) | `/properties/rhs/properties/parameters` | array | everything (no x-semio-ui) | — | — | Parameters / Parameter |  |
| (document) | (shared schema) | `/properties/rhs/properties/parameters/items/properties/name` | string | everything (no x-semio-ui) | text | — | Name / Name |  |
| (document) | (shared schema) | `/properties/rhs/properties/parameters/items/properties/kind` | enum | everything (no x-semio-ui) | segmented | — | Type / Typ |  |

## vcs · vcs

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | rename-vcs | `/newTitle` | string | everything (no x-semio-ui) | text | value | New title / Neuer Titel |  |
| any | add-tag | `/tag` | string | everything (no x-semio-ui) | text | value | Tag / Tag |  |
| any | remove-tag | `/tag` | string | everything (no x-semio-ui) | reference | target | Tag / Tag | ref={"kind": "tag"} |

## writer · writer

| subset | leaf | input | JSON type | missing before | widget | role | label en / de | facets |
|---|---|---|---|---|---|---|---|---|
| any | set-editor-settings | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-camera | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-camera | `/camera` | object | everything (no x-semio-ui) | — | value | Camera / Kamera |  |
| any | set-camera | `/camera/x` | number | everything (no x-semio-ui) | stepper | value | Pan X / Verschiebung X | step=1, precision=1 |
| any | set-camera | `/camera/y` | number | everything (no x-semio-ui) | stepper | value | Pan Y / Verschiebung Y | step=1, precision=1 |
| any | set-camera | `/camera/zoom` | number | everything (no x-semio-ui) | slider | value | Zoom / Zoom | step=0.05, precision=2, softMin=0.1, softMax=8, snaps=[0.25, 0.5, 1, 2, 4], scale="log" |
| any | set-engagement-input | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-engagement-input | `/value` | string | everything (no x-semio-ui) | text | value | Input / Eingabe |  |
| any | set-editor-selection | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-lint-generation | `/kind` | const | everything (no x-semio-ui) | — | discriminator | Change / Änderung |  |
| any | set-lint-generation | `/value` | integer | everything (no x-semio-ui) | stepper | value | Lint generation / Prüfstand | step=1, precision=0 |
| any | edit-text | `/text` | string | everything (no x-semio-ui) | multiline | value | Document text / Dokumenttext |  |
| any | rename-writer | `/newId` | string | everything (no x-semio-ui) | text | value | New name / Neuer Name |  |

## Withdraw-only leaves

| crate | subset | leaf | its value is | descriptor |
|---|---|---|---|---|
| stdio `avi` | hdrl | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `bcf` | markup | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `bcf` | markup | set-viewpoint-snapshot | the PNG bytes of one viewpoint's snapshot image | pending |
| stdio `binary` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `bmp` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `csv` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `deflate` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `docx` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `docx` | strict | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `docx` | strict | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `docx` | transitional | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `docx` | transitional | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `dwg` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `dxf` | header | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `epw` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `gif` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `gif` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | `editable: false` |
| stdio `gltf` | any | default-scene/unbind | nothing: the leaf takes no parameter | blocked |
| stdio `gltf` | any | snapshot/set | the whole glTF asset that replaces the current one (an import or a revert to file) | pending |
| stdio `html` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `ifc` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `ifc` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `ifc` | cobie | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `ifc` | cv20 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `ifc` | sav | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `jpg` | baseline | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `jpg` | document | replace-pixels | the decoded samples of the whole image | pending |
| stdio `jpg` | document | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `json` | i-json | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `las` | header | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `las` | header | set-vlr-data | the raw payload bytes of one variable length record | pending |
| stdio `md` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `mp3` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `mp4` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `obj` | geometry | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pdf` | a | clear-page-text | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | a | remove-output-intent | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | base | set-document-id | the two byte strings of the trailer's ID entry | pending |
| stdio `pdf` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pdf` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pdf` | e | remove-output-intent | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | ua | remove-display-doc-title | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | ua | remove-lang | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | ua | remove-mark-info | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | ua | remove-struct-tree-root | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | ua | set-struct-tree-root | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | vt | remove-dpart-metadata | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | vt | remove-dpart-root | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | vt | remove-output-intent | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | x | collapse-page-size | nothing: the leaf takes no parameter | pending |
| stdio `pdf` | x | remove-output-intent | nothing: the leaf takes no parameter | pending |
| stdio `ply` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `png` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pptx` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pptx` | strict | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `pptx` | strict | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `pptx` | transitional | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `pptx` | transitional | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | animation | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | audio | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | brep | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | cad | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | document | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | drawing | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | flow | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | graph | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | image | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | kit | delete-properties | nothing: the leaf takes no parameter | pending |
| stdio `semio` | kit | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | mesh | replace-primitive-geometry | the bulk vertex and index buffers of one primitive | pending |
| stdio `semio` | mesh | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | model | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | object | delete-brep | nothing: the leaf takes no parameter | pending |
| stdio `semio` | object | delete-mesh | nothing: the leaf takes no parameter | pending |
| stdio `semio` | object | delete-properties | nothing: the leaf takes no parameter | pending |
| stdio `semio` | object | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | presentation | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | table | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | text | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | value | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `semio` | video | set-sample-data | the encoded payload bytes of one sample | pending |
| stdio `semio` | video | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 1️⃣cc1 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 2️⃣cc2 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 3️⃣cc3 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 4️⃣cc4 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 5️⃣cc5 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | 6️⃣cc6 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `step` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `stl` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `svg` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `svg` | basic | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `svg` | tiny | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `svg` | tiny | strip-non-tiny | nothing: the leaf takes no parameter | pending |
| stdio `tiff` | baseline | remove-strip-offsets | nothing: the leaf takes no parameter | pending |
| stdio `tiff` | baseline | remove-tile-tags | nothing: the leaf takes no parameter | pending |
| stdio `tiff` | baseline | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `tiff` | document | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `tsv` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `txt` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `wav` | any | set-data | the complete sample data of the data chunk | pending |
| stdio `wav` | any | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `xlsx` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `xlsx` | strict | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `xlsx` | strict | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `xlsx` | transitional | remove-conformance-attribute | nothing: the leaf takes no parameter | pending |
| stdio `xlsx` | transitional | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `xml` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `xml` | valid | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `zip` | base | set-entry-data | the uncompressed bytes of one archive member | pending |
| stdio `zip` | base | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| stdio `zip` | iso21320 | set-entry-data | the uncompressed bytes of one archive member | pending |
| stdio `zip` | iso21320 | set-snapshot | the whole document that replaces the current one (an import or a revert to file) | pending |
| trinity `rewriting` | any | edit-before-fixture | the whole working graph the rule is tried on (nodes, edges and manifest); working-canvas edits are child-lane leaves | `editable: false` |
