# Services and Historical Identity Audit R21

Read-only source audit, 2026-10-28. Product sources were read only; no shell commands, source edits, ticket lifecycle calls or generated files. Read services-source-followup-r20.md and source-audit-r17.md. This is a handoff for the next implementation wave, not a completeness certificate.

## Declared State and Target

The source identifies a simplified teaching model (1928–1931), selected elements, rounded dimensions, CMN2021 topology and individually sourced structural details. Building.years records completion1931 and restoration1985–1997; those dates do not select an element epoch. Neither the schema's root/element properties nor the API inspected here provides an explicit historical-state field. Therefore the current model has no enforceable declaration that all represented objects belong to the completed original house. Its restored visitor-plan references cannot establish original fixture geometry.

Recommended target: original completed house as reconstructed from contemporary1930–31 evidence, with observations from later surveys explicitly labelled. Book2024 publication date is not the represented epoch. Keep proposed1929 details, executed original details,1932 repairs,1965 observed condition/proposals and modern restoration observations distinct. A hypothetical missing item must remain an uncertainty rather than becoming an invisible exact reconstruction. State identity should be schema-first, bilingual and carried into API/export evidence; do not create simultaneous original/restored alternatives within one physical building state.

## Source Evidence Register

The following compact primary-source findings derive from [Savoye as Built,2024](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1), official indexed passages (no full PDF/drawing inspection this audit): p69 air inlet beside support10; p116 two insulated sheet-metal heating risers beside6/11, with water riser at11; p117 soil stack beside spiral stair and boiler flue by wall; p119 rainwater within2/4 and original kitchen chimney subsequently removed. The book's three-chimney claim and Dubuisson installation references remain prior-audit leads; original drawing pixels were not recovered here. No exact plant footprint or complete routes were verified.

| Component | Attested state and approximate host | Current tag/gap | Measurement status | Next implementation test |
|---|---|---|---|---|
| Historical identity | Original/restored distinction above | No epoch selector; notes identify teaching scope | State attribution, not geometry | Schema fixture must reject unlabelled conflicting states and preserve evidence in exports. |
| Boiler plant | Cellar plant remains source lead, not newly verified location | BL-1 is ground, points(7,19.4,0) to(7,19.4,1.2), size0.4×0.6×1.2m | Authored teaching envelope, not verified equipment | Register plant room against original plan before moving BL-1; test host/level and flue continuity without inventing boiler dimensions. |
| Fuel inlet and cellar ventilation | p69 host noted above; cellar hatch/hopper author owns topology | No separately identifiable air inlet in audited service set | Host attested; slot dimensions not transcribed | Coordinate with cellar author; verify inlet has a real opening to outside and does not collide with footings. |
| Heating riser pair | p116 hosts noted above | PS-1 is one50mm assumed pipe at(6.9,19.4); PS-2/3 branch to RAD-1 | Neither present position nor casing section verified | Replace inferred network only after source registration; verify two distinct nonstructural riser identities, insulation/casing ownership, and source evidence. |
| Water supply | Shared riser at11 noted above | No water supply system among thirteen service-domain entries | No measured pipe profile/route | Add source-bound supply graph after tracing drawings; test cold/hot separation and fixture connectivity. |
| Soil stack | p117 host noted above | No identifiable sanitary stack | Host relationship attested; dimensions unverified | Trace paired WCs to stack and outlet, keeping it separate from structural piloti and boiler flue. |
| Boiler flue | p117 wall relationship noted above | No named flue/chimney product | No complete measured section/roof terminal | Register vertical path with stair and floors; test real penetrations and physical continuity. |
| Roof chimneys | Original kitchen chimney versus later absence is a concrete state conflict | Roof list has SC-1,SC-FRAME, parapets, drains, rooflight and canopy; no named chimney | Roof rectangles cannot establish chimney identity | Identify salon/kitchen/plant terminals from primary drawings/photos before authoring profiles; original target must not copy modern absence. |
| Rainwater | p119 concealed2/4 routes noted above | DR-1–4 and DP-1–4 form four schematic perimeter drops; all DP diameters0.10m assumed | Not an original routing reconstruction | Correlate each inlet, internal/exposed route and outlet; test continuity and hollow-piloti material exclusion without changing structure blindly. |
| Radiators | Original/restored schedule needs per-item verification | RAD-1 plus FIX-SERVANT-RADIATOR-1/2; latter are architectural furnishings | Source-informed fixture forms with inferred geometry | Compare each candidate to dated photos and drawings; keep uncertain candidates explicit; test leg/wall support only where attested. |
| Sanitary fixtures/taps | CMN2021 historical function does not date present replacement fabric | TAP-HALL, laundry/kitchen/guest/shared/parental taps and existing basins present | Functional fitting shapes inferred | Separate fixture shape evidence from plumbing-network evidence; no count-based completeness assertion. |

## Implementation Consequences

First establish state identity and an evidence register. Then prioritize visible, independently identifiable missing systems: riser pair, flue/soil-stack distinction and named roof terminals. They affect architectural silhouette and column reading as well as services. Plant location and distribution routing require another source-registration step, rather than extending the current arbitrary PS chain. Cellar inlet should be coordinated with the active hopper/hoist author.

The API creates its service output by metadata.domains membership. Consequently the thirteen entries are a domain selection, not a full inventory: servant radiators and taps sit in architecture and will not enter that service list automatically. Adding source objects alone will not ensure system exports understand them. Conversely changing domains merely to inflate counts does not establish a network. Implement membership/physical geometry/export checks against real semantic cases.

Tests should distinguish evidence validity, spatial hosting, actual openings/solid ownership, system connectivity and historical-state compatibility. Use a language-agnostic fixture with a third-party reference for computed geometry/network results, as repository instructions require. Passing existing schema, quantity or geometry tests cannot prove historical completeness. No measurements are supplied for unverified pipe positions, casing diameters, boiler form, chimney sections or routes.

## Retrieval Limits

Official indexed passages were used where available; the full52MB book and publisher/annex failures were not retried. The Scribd transcription surfaced during search but is the same authored book, not independent evidence or a measured drawing. Earlier R17/R20 findings remain attributed to those audits. No modern photo was used to infer an original service absence. This report intentionally leaves the complete plant/network schedule unresolved until primary installation drawing registration.
