# Villa Savoye Architectural Reference Audit

Date: 2026-10-28. Read-only architectural audit; source model editing belongs to the modelling agent. Existing `research-villa-savoye.md` was inspected and contains unresolved secondary-source orientation conflicts. The primary plan below supersedes those orientation claims.

## Authoritative Floor Plans

[CMN visitor leaflet, 2021](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), PDF page index 1 (second page), contains ground, first and second-level plans with room numbers and north arrows. A temporary inspected rendering is `🗑️generated/cmn-plans.png`.

Plan-top is southeast, bottom northwest, left northeast and right southwest. Ground: central ramp; stair lower-left; curved entrance hall below; garage right; service rooms left; laundry top-left; chauffeur top-right. First: salon across bottom, kitchen bottom-left; guest middle-left, shared bathroom above it, son top-left; parents top-middle, boudoir top-right; terrace right of ramp. Roof: stair left of ramp, curved solarium screens below; first-level terrace remains open to sky. These placements are visual readings of the published schematic, not surveyed coordinates.

The leaflet identifies three-car echelon parking, two servants rooms, chauffeur studio, laundry, pantry access to kitchen, shared bath with two doors, parents suite and boudoir. It gives salon 86 m² and parents suite 60 m². Roof garden includes covered kiosk, open garden, fixed table and two concrete planters concealing garage skylights. The solarium view opening aligns with the ramp.

## Coordinate Interpretation

For a plan-aligned model, define positive x toward image-right and positive y toward image-top; true north is approximately (-1,-1). Do not label positive y geographic north. The facade normals are then y-positive southeast, y-negative northwest, x-negative northeast and x-positive southwest. This is an orientation interpretation rather than a measured georeferencing result.

Prioritize these topology constraints in implementation: the long salon occupies the entrance-facade side; terrace lies beside the central ramp; bedrooms occupy the opposite and left portions; garage lies below the terrace; the stair and ramp connect all three levels. Preserve actual slab voids above the terrace and circulation. Do not fill the roof garden with a continuous upper roof slab.

## Dimensions and Construction Evidence

[CMN teaching dossier hosted by Fondation Le Corbusier](https://www.fondationlecorbusier.fr/wp-content/uploads/2022/05/2049_3993.pdf), page index 3, explicitly states that the final column spacing reduced from 5 to 4.75 m. Page index 7 describes a 60 m² entrance hall and northwest entry with full-height glazing. Page index 9 describes the salon as 6 by 14 m, with glazing to three sides, full-height sliding glazing to the terrace, kitchen opening to a small terrace, and alternating curved/straight solarium windbreak walls. Treat 6 by 14 m and the visitor leaflet's 86 m² as rounded accounts rather than forcing false numerical agreement.

Exact storey elevations, outer box size, column diameters, slab thicknesses, ramp gradients, curve radii, sill heights and opening dimensions remain unverified here. The existing research's approximately 21.5 by 19 m envelope is a teaching assumption, not a newly verified survey measurement. Normalizing plan proportions to that envelope is acceptable only with an explicit approximation label.

## Materials and Promenade

[CMN promenade](https://www.villa-savoye.fr/decouvrir/une-promenade-architecturale) confirms central ramp and helical stair as distinct circulation routes; ramp changes from interior to exterior and comprises parallel flights. Avoid disconnected decorative ramp pieces.

[CMN garden](https://www.villa-savoye.fr/decouvrir/un-jardin-suspendu) confirms the first-level garden, fixed table, semi-open shelter, concrete paving with grass joints, and upper curved windbreak with rectangular framed landscape opening.

[CMN colours](https://www.villa-savoye.fr/decouvrir/la-villa-savoye-en-couleurs) confirms dark English green ground enclosure beneath white upper facades and eight interior Salubra shades. Avoid assigning invented exact colour codes.

[Fondation Le Corbusier building record](https://www.fondationlecorbusier.fr/oeuvre-architecture/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-1931/) confirms post-and-slab structure, free facades, ribbon windows, central ramp plus stair, curved automotive ground circulation and construction completion in summer 1931. Its restoration account distinguishes original timber window frames from later painted aluminium replacements; frame material must match the chosen historical state.

## Modelling Priorities and Limits

Implement full room topology, curved enclosure and windbreaks, connected circulation and credible openings before loose decorative objects. Distinguish glazing from empty framed terrace openings. Architectural detail should derive from an inspected reference rather than generic villa furniture or guessed construction assemblies. The CMN leaflet is sufficient for schematic spatial relationships; it is insufficient for millimetre coordinates or construction detailing. Describe the result as a source-informed teaching reconstruction, with explicit assumptions for all unverified dimensions.

## Audit Execution Note

All shell commands used the repo interface. Near report completion concurrent launcher changes caused `repo tools` to fail and `repo --help` to report a malformed runtime-registry restart field. The report was saved through the file-patch tool. Temporary downloaded PDF, rendered plan and isolated renderer dependencies are under the ticket's generated directory and should be removed by the coordinator during final cleanup.
