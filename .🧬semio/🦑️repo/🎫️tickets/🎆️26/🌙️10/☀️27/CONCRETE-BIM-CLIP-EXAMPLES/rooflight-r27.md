# Guest Washroom Rooflight R27

ROOFLIGHT-1 is now a physically hollow26-part construction, replacing the1.50×1.40×0.18m solid glass teaching block. Its existing identity and element count are retained. The slab void follows the clear light opening, and every occupied component owns its material. This is an explicitly inferred reconstruction; it is not a claim to measured original shop drawings.

## Source Registration

Inspected existing complete raster register-r9-upper.png (Annex3,2448×3168) and cmn-plans.png (2246×1191). [CMN2021 official visitor document](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), §12 explicitly identifies the guest cabinet de toilette as lit by a zenithal window. The guest room sits beside the shared bathroom and beyond the stair; §17 separately identifies two garage lights masked by concrete planters on the first-floor terrace. Those garage lights are not the family implemented here.

Annex3 is a later author structural reconstruction, not an independently dated original1930 sheet. Its rightmost small crossed opening immediately beyond the stair corresponds topologically to the guest washroom region. Neither the other two small crosses nor the two larger crosses are assigned new rooflight IDs. The authored0.55m clear square is a deliberately rounded trace, with approximately±0.15m registration uncertainty.

For normalized Annex3 pixel coordinates u,v, registration uses the19.00m column-grid span from u≈.100 to.900 and v≈.190 to.808. The model is horizontally reversed relative to the raster: X=23.75(.900−u); Y=1.25+19(v−.190)/.618. The existing grid Y=1.25m, rather than0m, is essential. The chosen clear bounds X=2.95..3.50m,Y=10.90..11.45m map back to u=.752632..775789,v=.503879..521768. These normalized bounds lie inside the previously registered retrieval region .749..780×.501..525. Raster precision does not become metric survey precision. The clear opening is contained in SP-GUEST; source assertions and bilingual historical evidence carry the uncertainty.

## Construction and Datums

[Zaparaín, Savoye as Built, official UVa book](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1), indexed p161–162, supports the bathroom/corridor family:6cm concrete outer leaf,45cm height above structural slab, insulating-brick inner leaf on a6cm upstand, inclined glass with mortar/putty bedding, and front ventilation operated by a rod below. Original FLC H1(13)312/19443/19458 remain retrieval targets; their full measured profiles were not recovered. Applying that family to the source-identified guest washroom light is stated as inference.

The clear opening lies fully in17cm roof region R-PRIMARY-SL-R-51: structural top6.51m, weathering datum6.60m. Outer concrete reaches a maximum6.96m, exactly45cm above structure, rather than45cm above finished roof. Side leaves have inclined upper edges; the front has a real interrupted vent opening, and the inner brick front is also interrupted to preserve its air passage. The inner upstand rises6cm above structure. Glass is a native inclined6mm section extrusion, with bedding, a thin inferred glazed vent flap and separate metal operating rod. The rod continues through the clear slab hole into the room.

Inner leaf5.5cm, cavity2.5cm, slope, glass and bedding thickness, vent/flap form and dimensions, rod section and exact mechanism are inferred. The original vent material is unresolved. MAT-INSULATING-BRICK is the insulating-brick envelope, with assumed8kN/m³ teaching density; MAT-MORTAR-PUTTY is a nominal bedding envelope with assumed16kN/m³ density. Neither is presented as a laboratory measurement or precise historic compound. Outer concrete and metal/glass use existing materials.

An independent slab-intersection test exposed a3mm curb intrusion into the edge of the adjacent23cm structural region R-PRIMARY-SL-R-48. The inferred outer front was moved fromY10.76 to10.77m while preserving the registered clear opening. The final outer footprint is X2.81..3.64,Y10.77..11.59m. No slab material is double-owned.

## Shared Physical Contracts

Native solid-part schema now permits material and role on prism, section and mesh parts. The generic physical representation builder preserves those fields for plan, section, elevation,3D and body primitives. No runtime library was introduced. Existing generic section extrusion provides the actual nonhorizontal rooflight geometry.

Production plan/section/axon providers now choose appearance from each occupied part material. Previously kind=rooflight painted the whole product as glass. The section/elevation provider now routes native section and mesh parts through actual faces rather than rectangular vertical bounds. The source rooflight therefore has opaque leaves, thin glass and metal rod in the actual drawing callbacks.

Native STEP now accepts IfcBuildingElementProxy multipart solids through the existing native section/sweep path. Material constituent sets and shape aspects partition all26solids by7roles and5materials, and bilingual historical evidence is emitted from the source historical_evidence field. Existing window/door/curtain constituent naming is retained.

## Verification

The language-neutral ☀️rooflights.json contract was written first; its first run failed because the prior source had only an unowned glass prism. Final ☀️rooflights.py has2passing tests (1.885s latest run): GEOS exact clear aperture and room containment; clear ventilation passage; pairwise occupied component slices; no occupied slab intersection; SciPy convex-hull volume for every part; actual production plan callback polygons with distinct opaque/glass appearance; exact inclined production section and face-area checks; all5physical representations. JSON Schema2020-12 full-source validation using pinned jsonschema4.23.0 passed with0errors after latest publication.

Independent pinned web-ifc0.0.78 decoded both English and German native excerpts:26placedsolids,312triangles, native volume0.13133235404220736m³ versus public API0.131332357m³. Native placed bounds agree within1e−5m. Independent decoding verified7named material constituent roles,5localized material names and an exact26-solid shape-aspect ownership partition, with historical target/source/uncertainty properties present. These checks validate physical authored geometry, not historical metric fidelity.

The source overlay and actualY-Z material section are generated/rooflight-register-r27.svg (under 🗑️generated). Native samples and diagram remain generated output for parent final cleanup. Verification inputs rooflight-r27-author.mjs and rooflight-r27-native.py/.mjs are retained. The author script uses fresh source reads and narrow element/void/material changes, and is repeatable without duplicating materials.

Native Manim remains blocked by the existing launcher/registry issue. No native rendered frame or complete roof-terminal identity register is claimed. Other crossed Annex3 openings, exact shop profiles and full original vent operation remain unresolved.

## Changed Files

- CLIP/🧱️model/🔣️.json
- CLIP/🧱️model/🐍️.py (physical representation material/role preservation only)
- CLIP/🧱️model/🧬️schema/🔣️.schema.json
- CLIP/🧱️model/🔌️exchange/🐍️.py
- CLIP/🧱️model/🧪️tests/☀️rooflights.json and ☀️rooflights.py
- CLIP/🎨️visuals/🏛️savoye/🧱️core/🐍️.py
- CLIP/🎨️visuals/🏛️savoye/📋️plan/🐍️.py
- CLIP/🎨️visuals/🏛️savoye/✂️section/🐍️.py
- CLIP/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py
- ticket/rooflight-r27-author.mjs, rooflight-r27-native.py, rooflight-r27-native.mjs, rooflight-r27.md

Main test discovery registration was handed to clips_r11, which owns the shared main test file. Parent owns lifecycle and generated cleanup. No shell, Git mutation, worktree, IDE or AGENTS edit occurred.

## Material-Owned Weight and Mass Inventory

At parent request the generic authored multipart quantities now also return material_quantities, weight in kN and mass in kg. Each material_quantities entry includes the source material tag, occupied volume in m³, a complete unit_weight quantity in kN/m³, weight and mass. Unit-weight assumptions remain explicit. Each part uses its own material when present and the product material otherwise. MAT-AIR is excluded from material inventory and weight; existing geometric volume behavior is preserved. Occupied unions are integrated separately per owned material, avoiding repeated same-material internal parts. Mixed-material cross-ownership remains a geometry error and is tested separately. Typed filling quantities are preserved.

A second language-neutral fixture was written before this implementation:1m³ fallback-RC prism,0.2m³ explicit insulating-brick inclined sweep, and an air part with nonzero nominal unit weight. Its first run failed with missing material_quantities. Final3-test rooflight suite passes (2.047s), including the fixture and SciPy convex-hull density-weighted source oracle. Standard gravity9.80665m/s² converts weight to mass: mass_kg=weight_kN×1000/9.80665.

ROOFLIGHT-1 inventory: RC0.080725427m³; insulating brick0.044618180m³; nominal bedding0.001625250m³; glass0.004272600m³; metal0.000090900m³. Their distinct teaching unit weights give2.515035760kN and256.462274069kg. Applying the product fallback concrete unit weight to the entire assembly would incorrectly treat brick, bedding and metal as concrete; that shortcut is no longer used by the generic inventory.

The analytical tributary API currently estimates slab regional field self-weight, finish and imposed area loads. It has no accessory extras/mass path. This work does not silently add rooflight inventory weight to slab-field reactions or claim a whole-frame accessory-load model. Native exchange currently has no Qto quantity output dialect; material constituent geometry and evidence are exported, but no new ad hoc native mass property is invented. All6modified Python providers parse successfully.
