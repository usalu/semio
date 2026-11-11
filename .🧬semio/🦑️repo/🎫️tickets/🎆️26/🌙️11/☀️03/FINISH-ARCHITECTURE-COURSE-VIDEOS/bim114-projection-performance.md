# BIM 114 large projected-material observation

The restarted source entered an architectural MAT-METAL projected-material region with 7,811 visible facets from a primitive containing 15,687 faces. A read-only py-spy locals sample at 2026-11-07 17:24:54 UTC showed the `_regions` classification loop at atom index 55,418. The process had advanced to Animation 9 by the 17:28:30 UTC sample, so it left the projection within 3 minutes 36 seconds after that sample. Neither the classification start nor the total atom count was exposed by the sampled stack; total elapsed time and completion percentage are unknown.

The stack sample was inside `_regions`' per-atom loop over the rings and their bounding boxes, consistent with that scan consuming work. A single sample does not establish a dominant cost or a complete profile. The primitive's exact tag was not exposed by py-spy, so no independently reproducible named material producer was identified; a stand-alone benchmark would first need its tag or captured actual facets. No process was stopped and no production source was changed for this observation.

The render progressed beyond the formerly failing beam-projection call into Animation 9, but has not yet produced a completed source receipt. Successful passage of that point is not a full source or master validation.

Visual QA: current source 114 partial 8 (extracted by retained ticket input bim114-frames.py) shows a readable English context caption and initial 4.75-unit choices; it contains no staffage. Later plan, axon and vehicle frames remain pending.

Current-attempt partial 9 (mtime 17:42:41 UTC) middle and final frames were decoded with PyAV and inspected. The English caption overlaps only the empty lower plan-grid margin and tops of bottom grid circles; the building outline, A–E labels, 4.75 m dimension and axon remain visible. The transition midpoint briefly overlays the changing number while the view enters. No car or human staffage is visible in this partial; later scenes remain pending.

The two gray circular plan marks in partial 9 correspond to model furnishings FIX-HALL-TABLE-1 and FIX-HALL-TABLE-2 (IfcFurnishingElement, entrance column tables), not vehicle staffage. Their authored bounds center at (8.128, 6.006) m and (10.898, 6.006) m, matching their plan projection. Later car and person frames still require review.

Current-attempt partials 20, 25, 30, 34 and 36 were sampled after verifying their mtimes. Partial 30 shows the sourced handcrafted bim-information-car cast as an isolated 1929 saloon diagram, with 4.60 m × 1.75 m annotated envelope and a visually matching approximately 2.63:1 outline. It is a separate explanatory diagram, so its displayed size is not a shared-plan scale measurement. The later moving plan vehicle and any person staffage remain to be inspected.

Vehicle ownership/spec trace: the explanation, moving plan and pinch vignettes all use source 114 _car and the same bim-information-car → vintage-car-1929 plan cast. The catalog declares overall 4.38 m length, 1.70 m width and 1.70 m height; the prior plan-render bounds were 4.384 × 1.70 m. The separately assumed collision/clearance envelope is 4.60 × 1.75 × 1.65 m and is used for model clearance and diagram dimension lines; the displayed note explicitly identifies it as an assumed clearance envelope. The shared-plan moving vehicle remains to be visually reviewed. Source 114 has a building axon but no separate 3D axon car in its code.

Relevant source callsites in 📊️information/🐍️.py: _car at 210–213, _driver at 224–247, _vignette at 455 onward, and the diagram dimensions/caption at 704 onward. In 🧱️model/🐍️.py, _car_outline at 2995 uses the assumed model envelope. The distinct height numbers have not been judged against a 3D vehicle or collision scene; catalog height is rounded metadata.

2026-11-07 18:45:55 UTC read-only py-spy sample of current 114 native PID 18256: Python deepcopy within _geometry_query → building_geometry → geometry_context → villa_plan → _plan → present_information. This is a single snapshot, with no runtime or dominance inference. No current-attempt partial had yet been written.

2026-11-07 18:46:01 UTC sample of current 117 native PID 38300: Fraction bound comparisons in _finish_reservations → _slab_geometry_uncached → _slab_geometry_entry → _wall_cutters → building_geometry → _prepare line 164. That call uses the modified scenario model, not the unchanged base-model cache key; the sample alone does not establish cache failure or duration. No process was changed.

2026-11-07 18:48:34 UTC: second 114 snapshot advanced to Fraction operations in plane_chart → material_skin → axon _surface_prims → villa_axon in present_information line 595. At 18:48:38 UTC the 117 scenario geometry worker was again in _finish_reservations candidate selection. These are isolated stacks rather than duration or dominance measurements.

## Vehicle height and collision semantics

The information lesson uses the handcrafted vintage-car-1929 as a plan symbol. Its authored 1:50 elevation cabin and roof reach y=1.70 m and its catalog height is 1.70 m (authored estimate). The teaching model separately declares an assumed 1.65 m car height, alongside its 4.60 × 1.75 m envelope. The actual drawing is a plan view and does not itself depict height, but the selected saloon's authored elevation is 5 cm taller than the assumed collision band.

Native model _vehicle_obstacles calls vehicle_obstacles from ground through ground + car.height; it projects occupied parts of pilotis, walls, fillings, slabs, screens, stairs, details, furnishings, beams, foundations, railings, services, curtain and ramps into that band. path_pinches likewise filters column parts through _vehicle_part, and the lesson's _profile_pair and _vignette use the same ground-to-height band. Thus a building feature confined to the 1.65–1.70 m slice could be excluded by the assumed sweep despite intersecting the depicted asset's upper portion. This is a real limitation of the assumption; it is not proof that the actual path or clearance result changes.

As a bounded source check, the model JSON has 34 ground IfcColumn elements, with no explicitly numeric column-section base/top boundary strictly between 1.65 and 1.70 m above its zero ground datum. That check does not cover unresolved level references, derived native geometry or all other obstacle classes, so it cannot certify the full route. No heavy duplicate model/route calculation or production mutation was performed. The source notes the route and vehicle dimensions are assumed; the lesson should not be described as a collision certification for the 1.70 m saloon without resolving this distinction.

## Height containment correction and invalidation

The shared assumed vehicle height is now 1.70 m, matching the authored saloon elevation/catalog. The native collision algorithm is unchanged. A schema-first permanent fixture first failed both enclosure and synthetic 1.66–1.69 m obstacle checks with the old 1.65 m data; after the data edit, both pass against the actual extracted native height filter and an independent Shapely vertical-segment intersection. Together with the 114 formula and 116 paper-scale tests, four focused tests passed in the production Python 3.12 environment.

The canonical segment-state audit checked all 126 source ordinals; only 114–119 include the changed model JSON. All 19 frozen model/cache code and manifest hashes remain unchanged. The existing full-model digest changes from 5f7d5e6f50b0740028b79fd16df312f6ef49eec4f284890d2298d123abfe689e to e23f4ecb133165ddfed19432a8a7035f63eaeca33cfe4df5942ba5fe1440f00d. The old slab records belong to the old model and must not be reused or relabeled. All ten native slabs require a fresh exact cold/warm gate for the revised full model before BIM output acceptance. The stopped prior BIM and full-suite sessions remain incomplete. Other fixture scenarios with independently declared modern vehicle heights are not automatically the shared historical-car assumption.

Updated/created files under 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip:
- 🧱️model/🔣️.json SHA-256 bc4c150e562c0a3c113dbd9433ade854f28d74d7c493dde8a014ace7803e854c.
- 🧪️tests/🐍️.py SHA-256 9a85008a6f20ac855d6060fd5cdf5b7d203fea3e24eaadc66c2e77d2ddee9f0c.
- 🧪️tests/🚘️height/🐍️.py SHA-256 cb99486cd1a79bee19baed2ced7f9301efe055da9dfb867fbf41d94e2c862bb0.
- 🧪️tests/🚘️height/🔣️.json SHA-256 fae62a2dd27dbfc690797c4e59f5b54bacadc4ab972f923dd150f9caa2fd6517.
- 🧪️tests/🚘️height/🔣️.schema.json SHA-256 2aeb2af7ead65bc4e1c437d86dcebcaf2013e1ef06eeda27d881b9ce2296db98.

Retained ticket inputs: bim114-height-test.py, bim114-height-cases.json, bim114-height-cases.schema.json and bim-height-closure.py. The closure log is temporary generated output.

2026-11-07: The corrected full-model v3 native slab gate was admitted at 8 GiB in session 55491. Its dedicated cache and log are under 🗑️generated/full-slab-v3 and full-slab-v3.log; all 10 native slabs are listed by the retained full-slab-validation.py input. All 19 frozen producer/cache inputs plus corrected model JSON SHA bc4c150e562c0a3c113dbd9433ade854f28d74d7c493dde8a014ace7803e854c were verified before execution and recorded in cache-freeze-v3.json. The gate is running, not yet a pass; all 20 inputs must be rechecked after exit. Old v2 results and records remain unchanged.

## Isolated plan vehicle transform

The source helper named _driver creates the moving car and swept path; it does not create a person. Source 114 calls only the bim-information-car staffage cast. A text search and read of the shared Savoye plan/axon visual modules found no human/person/staffage cast insertion there, so a shared-plan human comparison is not applicable to this source; this is not a claim about other lessons.

The existing permanent 🚘️car-scale test compares actual _car bounds at authored scales and right-angle headings with Shapely. A separate bounded native source-helper probe, bim114-driver-scale.py, constructed _driver using synthetic poses and no building geometry. With SCALE 0.22 scene units per metre, world displacement (2,1) m maps to (0.44,0.22) units; the actual car centre is (-2.56,0.22) after the authored origin. At headings 0° and 90°, actual bounds are 0.96448 × 0.374 and 0.374 × 0.96448 scene units, respectively, matching independent Shapely rotation/scaling of the native authored plan bounds. Both isolated Cairo frames were captured under 🗑️generated. This establishes local transform consistency only; the eventual full moving-scene frames still need visual review.

V3 was deliberately stopped and reaped exit 1 after the stale bilingual model notes were discovered; its single SL-1 result remains historical and the full gate is incomplete. Guarded capacity root 64504 and its exact descendant tree were stopped; unrelated producers continued. The English and German physical-datums-r12 notes now describe the assumed vehicle-height envelope without duplicating its structured numeric value. The permanent height test failed on the stale note before the edit and passes after it.

Bounded searches covered BIM and architecture authored Python, Markdown and JSON for 1.65/1,65 near car, vehicle, Auto, Fahrzeug, Wagen or height language, plus explicit model and fixture references. The independent 4.2 × 1.65 × 1.65 m profile-oracle car and modern parking teaching fixtures remain unchanged; no other concrete historical-car note match was found. Large generated-looking parking/overlay fixture geometries were not exhaustively searched as prose.

Corrected model SHA-256 bd43e5585fce0eec30332c4eb8b953451e89d84af8ee09f385139bf10aa926e6; permanent height test SHA-256 a283cd133ebccbe64f754f908cdefb9ce1286a4cbb9de18019679d95ff420ea2. Four focused tests passed; all 19 producer/cache inputs matched. The new 20-input snapshot is cache-freeze-v4.json, and independent v4 gate session 60843 is tracked in slab-cache-v4-validation.md.

The height regression was refined to compare any explicit English/German numeric height claim with the structured model value, while accepting a numeric-free note. Its schema-first examples cover stale 1.65/1,65, matching 1.70/1,70 and numeric-free formulations; the focused test passed. Current test SHA-256 cd6235e08309d6ff64eda700025daae234c186cf913c5b50e25b7071c6f4ed7f; fixture 497bf63d8a2c60ac173e41e14bf09d96dbf909e9948103ef5b20ae955e4e978f; schema 631ddba56d9c5318fd3d2facd2c2f5def0146cdac72940db3d3be16ed87dd279. All 20 v4 inputs remained unchanged.
