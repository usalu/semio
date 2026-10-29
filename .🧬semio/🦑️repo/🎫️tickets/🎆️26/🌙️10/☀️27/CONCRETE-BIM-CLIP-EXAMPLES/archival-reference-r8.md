# Archival Reference Investigation R8

2026-10-28. Read-only archival evidence audit following detail-completeness.md. No source edits.

## Stronger Accessible Evidence and Saved Images

The author-hosted [2022 structural study PDF](https://www.fernandozaparain.com/_files/ugd/89d2e0_ff4aadc4c8a3464d97e8c5f36e0b93d8.pdf) was downloaded as generated/author-r8.pdf. It contains 24 PDF pages: cover/issue pages precede the twelve-page article, followed by seven detailed annexes. The underlying research is [Zaparaín Hernández et al., DOI10.3989/ic.88625](https://doi.org/10.3989/ic.88625), based on in-situ measurement and original FLC contractor drawings. The publisher identifies the article as CC BY4.0; cite authors and DOI when reproducing research-derived geometry.

Saved inspected renders under the ticket's generated directory:

| File | PDF page (one-based) | Content |
|---|---|---|
| archive-r8-page18.png |18| Annex1 basement/foundation plan |
| archive-r8-page19.png |19| Annex2 ground-storey ceiling/first-floor structure |
| archive-r8-page20.png |20| Annex3 first-storey ceiling structure |
| archive-r8-page22.png |22| Annex5 longitudinal sections |
| archive-r8-page23.png |23| Annex6 transverse sections |
| archive-r8-page24.png |24| Annex7 structural exploded axon |

Some raster photographs in PDFJS rendering failed JPEG2000 decoding; vector annex18,22,23,24 remained legible and were inspected. Do not treat blank photograph areas on a rendered sheet as original absent content. Original downloaded PDF remains available for an independent renderer.

The stronger full architectural book [Savoye As Built, university copy](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1) was identified by coordinator but could not be downloaded in this audit because the university host repeatedly timed out. This is a retrieval limitation, not evidence that drawings do not exist.

## Cellar: It Is Documented, but Not a Full-Footprint Basement

The study identifies FLC-H1-13-298 setting-out/excavation plan (28February1929), definitive-stair setting-out FLC-H1-13-300 (15March1929), basement ceiling FLC-H1-13-301 (19March1929), and foundation plan302. The annex1 reconstruction gives an actual perimeter and access topology. This resolves the prior uncertainty: a cellar plan DOES exist, independently of CMN's hoist sentence.

Inspected annex1 has a narrow central masonry rectangle with two longitudinal bays, partial central dividing wall, internal column14, and spiral stair in the upper-right portion of the sheet. Small side projections are shown but their functions should not be named without the corresponding architectural drawing. It is substantially smaller than the upper square box. The stair lies within the right bay. The lower central partition ends short of the upper transverse bay, permitting connection between sides. Do not produce a whole-building underground slab or arbitrary cellar room sequence.

The annex orientation is180degrees rotated relative to the inspected CMN visitor diagram: spiral stair appears upper-right in annex1 versus lower-left in CMN. Rotate the basement plan before registering it to the source model. Fit the stair centre and column/ramp geometry, not the outer page edges. Existing source coordinates are schematic, so a source-informed basement can preserve archival topology while explicitly approximating registration.

Article §2.1 gives basement height240cm, exposed stone/lime masonry retaining walls, and30cm lime-concrete strip footing; basement ceiling has7cm slab and16cm-wide/27cm-deep ribs, deepening to35cm locally. Annex5/6 section labels show basement floor approximately-2.42m and footing-2.72m relative to original datum. Keep structural thickness separate from finished levels; do not simply subtract240cm from the model's assumed ground slab.

CMN visitor leaflet §9 attests a rotating metal jib connecting garage and cellar. The structural annex does not label the garage hoist hatch. Its small side projections and stair void must not automatically be called the hoist aperture. A schematic hoist needs an explicitly inferred hatch position with a real opening, or further architectural/photo evidence before claiming an exact historical installation.

## Original FLC Ground Drawing: Useful but Earlier Design

[World Heritage archive ground drawing FLC19414](https://lecorbusier-worldheritage.org/wp-content/uploads/2019/07/10-villa-savoye-et-loge-du-jardinier-plan-soubassement.jpg) saved as generated/flc19414-ground.jpg. Its title is soubassement, which here means the actual ground/service storey, NOT proof of underground cellar level. It shows5m grid dimensions and an earlier chauffeur kitchen/service arrangement; therefore do not use it as definitive as-built geometry.

It nevertheless distinguishes main entrance at diagram-bottom from entree de service on diagram-left beside the stair/service corridor. The main entrance has an asymmetrical column/table arrangement, confirming that a uniform structural grid is an insufficient hall reconstruction. Two column tables are attested in the CMN leaflet; a specific four-column hall claim needs definitive measured-plan registration. Do not add two random posts merely to meet a count.

## Other Concrete Constraints Recovered From the Structural Study

- Article §4 explicitly describes the kitchen patio opening under beam11–17, continuing the northeast facade beam16×30cm. This is evidence for a real side patio/opening distinct from the main hanging garden. Relate it to the northeast facade after rotating annex coordinates. Do not enlarge the kitchen by relabelling an existing room or an arbitrary main-terrace rectangle.
- Article §6 gives typical22×22cm square columns, with34/35 instead18×22cm, and explains upper supports that begin on transfer beams. The grid is not a stack of identical uninterrupted posts. Use these data to audit existing33 column occurrences, not retrofit them silently as exact archival counts.
- Article §9 gives6cm solid concrete stair parapets,61cm parapet height,35mm upper handrail and50mm central metal tube. The basement flight differs from upper flights and is surrounded by full-height partitions; it starts at an8cm floor slab with11×11cm support Pa on its own30×30cm footing. An exact copy of the21-step upper stair down to basement would be unsupported.
- Article §4 and annex3 distinguish occupied-floor23cm structure (18+5) from terrace17cm (12+5). The current uniform25cm slab abstraction is not a faithful construction description. These are structural build-ups, not complete finish/waterproofing layers.
- Annex5/6 provide vertical coordination stronger than the existing assumed0/3.5/6.7 datum. They show first-level approximately+3.08/+3.14 and upper roof+6.77 with soffit/structural offsets. Adopt a coherent measured datum only as a deliberate whole-model update.
- The study identifies original salon and kitchen chimney forms and notes the northeast kitchen chimney has disappeared. Declare original/restored state before adding caps from historic images.

Parents bed alcove: CMN proves two supporting columns frame the sleeping position, but the present study is structural rather than a labelled furnishing plan. Annex3/7 can register actual support candidates only after180degree orientation mapping; they do not independently identify which two should be labelled the bed alcove. Exact alcove position therefore remains pending the measured architectural book or an inspected photograph. Do not create new posts merely from the sentence.

## Execution Recommendation

The full basement can now be included as an explicitly source-informed reconstruction using annex1 and sections, with actual stair access and masonry bounds. Hoist location, hall four-post interpretation, exact bed-alcove posts and kitchen patio shape still require careful registration or further architectural evidence. Feature existence is not sufficient to assert an as-built model. Preserve unresolved distinctions in metadata and reports; complete generic and physical propagation/tests after any datum or structural change.

All downloads and temporary images are under generated and should be removed at coordinator cleanup; retain this report. Shell attempted only through repo; its tools subcommand remained unavailable. Rendering and file work used direct Node filesystem/network/library APIs.
