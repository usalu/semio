# Labrador face

The installed shoulder and lumbar revision `dbe3b1e913` was the starting point. Its seated 1:50 model exposed about 121 mm² of the near eye and 124 mm² of the far eye. The three coarser digital bodies had no physical eye. At 1:200 and 1:500 the nasal heat region also had no equivalent external body part.

The revised skull-to-muzzle transition has a forehead rise, a defined stop, a broad bridge and a blunt muzzle. The flews follow the sides and underside of the upper muzzle. The mandibular skin and bone articulate under them. The tongue follows the floor of the lower jaw and the oral outlet stays with its tip. The design retains the broad skull, moderate stop, close-fitting lids and wide nares described in the [United Kennel Club Labrador standard](https://www.ukcdogs.com/labrador-retriever/).

Each aperture is 26.4 mm long with separate upper and lower lids, a moist surface and a round pupil. The modelled distance from the corneal apex to the posterior globe is 21.55 mm. A canine ultrasound study reports approximately 21.7 mm for its Labrador group. The aperture and nasal dimensions are authoring estimates. [Working-canine ocular biometry](https://pmc.ncbi.nlm.nih.gov/articles/PMC13426095/)

| Measurement | Production geometry |
| --- | ---: |
| Exposed seated eye surface per side, excluding pupil pigment | 294 mm² |
| Front-to-back depth at either nostril centre | 8.24 mm |
| Exposed seated nasal planum, excluding vestibule surfaces | 3,322 mm² |
| Standing exchange area across both wet eyes, including pupils | 701 mm² |
| Standing exchange area of nasal skin and vestibule surfaces | 3,439 mm² |
| Standing exchange area of head coat | 0.07537 m² |
| Exposed tongue in trot phase 0 at 1:50 | 5,629 mm² |

The [facial mesh test](lab-face-check.ts) measures the physical eyes and pupils, ray-casts into both nostrils and checks the wet-surface bindings over all 19 named poses at each scale. In the seated mesh it checks every vertex of the skull, mandible and paired zygomatic arches against the skin. The [thermal measurement](lab-face-heat.ts) measures the fine standing exchange surfaces. The [source fixture](lab-face-case.json) contains the contour stations and geometric thresholds.

The [author](lab-face-fit.ts) also retraces the elevation, front and plan views from each scale’s face. The 1:500 paper outline groups the smallest facial regions into the head. The [close-up sheet](lab-face-sheet.ts) renders the front, oblique, profile and bony orbit with production normals. It accepts a document, an output filename and an optional pose.

Installed catalog SHA-256 `7fe1518b5d723750adb136ab93283e623d9fd1121d815fec801b592a7b1faaed`. The independent 12-scene GLB passed with zero findings at 4,841,792 bytes. All 52 volume-rule evaluations and the full schema and semantic checks passed.
