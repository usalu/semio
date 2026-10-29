# Roof Source Gaps R19

## Current Physical Omission

SC-1 is one IfcWall SOLIDWALL with a single assumed0.15m band, a two-arc/polyline path, and a uniform6.60–9.40m envelope. There is no native opening occurrence hosted by SC-1. The production building geometry exposes a screen path/base/top/thickness only; no occupied screen parts or opening list. The plan provider draws the uninterrupted band. The axon provider can read screen opening intervals but receives none from the source. Thus the solarium's most recognizable rectangular landscape aperture is absent from the actual model.

The independent four-storey geometry sheet at generated/physical-storeys-r19.png draws the current source and makes this omission visible. Its screen band is derived with the existing core offset helper only for the review; this does not modify or repair the production screen contract. This diagnostic is not a native Manim frame. Space labels and below-cut furniture are schematic annotations; the occupied wall cuts themselves come from the physical model.

## Primary Evidence

[CMN2021 visitor document](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), §18, identifies the rectangular landscape opening in the ramp axis. The [CMN garden article](https://www.villa-savoye.fr/decouvrir/un-jardin-suspendu) independently describes and photographs that opening, as does its [history article](https://www.villa-savoye.fr/decouvrir/histoire-de-la-villa-savoye). These are primary owner documents. Their existence claim is strong; exact aperture width, sill/head and the full wind-wall contour require registered plan/elevation or measured detail inspection.

The CMN roof plan also shows circulation from both the stair and ramp, multiple curved wall limbs, stair enclosure/landings and the irregular wind-wall arrangement. The current assumed two-half-circle path and uniform top must be audited against it. Do not repair only the hole and call the complete roof faithful. Structural Annex6 shows a9.325m top leader, which differs from the current assumed building.height9.40m; the corresponding wall family and finished datum must be mapped before changing the entire building's maximum.

## Required Clean Contract

Introduce occupied curved/piecewise wall solids with native hosted apertures measured by cumulative path station. Opening cuts must traverse the actual wall thickness at their sill/head and remain physically empty in plan, section, elevation, axon and native representations. Curved centerline length alone is not solid volume; corners and caps must share one physical band rather than overlapping segment boxes. Use independent GEOS offset/difference/volume checks and actual projection recorder checks. Preserve uncertainty/provenance per authored metric.

A separate read-only roof audit should register the actual contour, aperture, stair arrival and wind-wall height changes before source publication. Source JSON is frozen for the chapter baseline at the time of this report.


## R20 Source Correction

Both high-resolution Annex5 and Annex6 independently show the exact highest leader +9.325m. The earlier9.335 reading was incorrect. Annex5 lower section additionally gives the aperture sill/shelf +7.25m, opening head +8.475m and cap inner edge +9.105m. The CMN photograph generated/roof-cmn-aperture-r19.webp shows a continuous wall above and below the opening. The first R20 vector audit incorrectly interpreted the cut-height gap as circulation; [the corrective audit](roof-aperture-topology-correction-r20.md) withdraws that claim. A plan subpath gap is not by itself proof of a full-height physical break.
