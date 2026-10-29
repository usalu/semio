# Architectural Source Audit R11

Read-only audit, 2026-10-28. No model or provider source edits. Architectural drawing access is unresolved; this report must not be represented as a measured architectural verification.

## Primary Drawing Access

[University annex record](https://uvadoc.uva.es/handle/10324/66393) identifies separate 1930 cellar, ground, first and roof plans and six sections/elevations, by Fernando Zaparaín Hernández (2024). The search index exposes the first-floor file name SAVOYE PLANTA+1.pdf but no authenticated drawing pixels or exact attachment link was recovered. A direct web open timed out again in this pass. Exact quoted metadata is retained in asbuilt-access-r11.md; avoid treating index descriptions as a visual trace.

[Author publication page](https://www.fernandozaparain.com/le-corbusier) identifies the complete 2024 study and the accessible 2022 structural article. Search found no additional official/author architectural drawing copy. No access restriction was bypassed. The structural article is suitable for supports, slab regions and cellar evidence; its ceiling plans do not establish every partition, furniture contour, door leaf, finish zone or facade detail.

## Acceptance Gate for Remodel

The architectural candidates below remain evidence-informed requirements rather than newly surveyed coordinates. Compare the published model to structural-register-r9.md and JSON before calling the remodelling complete:

- Preserve the 4.75m grid and roughly 19.30m wall envelope rather than scaling it into the old teaching box. Reconcile stair centre near (6.24,9.26) with its openings, landings, guards, cellar stair and adjacent rooms together.
- Replace regular support assumptions by the numbered occurrence register, including first-only transfer supports and special upper16/22 sections. A circular replacement for the aerodynamic columns or an invented analytic ellipse cannot establish their actual shape.
- Keep the hanging garden open to sky and model the small kitchen patio as its own genuine roof void. These are separate spaces; a kitchen door into the main garden does not establish the patio.
- Confirm slab regional soffits/tops independently, then use actual occupied parts to test all walls, fixtures and headroom. Regional slab counts or passing geometry getters alone do not prove architectural correctness.
- Preserve the partial two-bay cellar and connected stairs. Its side projections and garage jib hoist remain unlocated; do not label speculative projections as hoist access.

## Detail Omissions Requiring Architectural Plan Inspection

The R7 completion inventory records outstanding separate stair-side service entrance; explicit guest washing/dressing finish and wardrobe separation; parents bed alcove and its two supports; small kitchen terrace/opening; terrace concrete paving with grass joints; and garage/cellar jib hoist. These must be rechecked against the remodel snapshot, rather than assumed absent forever. Exact Salubra colours and original versus restored frame material also remain undeclared. Source-attested details can be modelled with explicit inferred dimensions; inferred dimensions must remain visibly distinct from measured geometry.

## Snapshot Status

At initial inspection the source remained the pre-remodel teaching JSON with old generic pilotis and 0/3.5/6.7 abstractions. A remodel writer is actively publishing the replacement. No findings below claim the replacement has already been audited. Post-publication observations will be appended after notification.

## Confirmed Pre-Publication Structural Omission

The inspected JSON contains only two IfcBeam entries: RIB-CELLAR and BEAM-CELLAR-STAIR. There is no first-floor/roof primary beam network. The structural annexes explicitly draw dimensioned beams, and register R9 distinguishes beam annotations from slab depths. Regional slabs and column occurrences alone therefore do not constitute the complete archival structural model. Actual beam widths, vertical depths, offsets, transfer paths and joins require source tracing and occupied-solid implementation. Do not infer a complete structure from a plausible slab silhouette.

The initial architectural inventory is stale in specific respects: the inspected source already contains W-G-SERVICE-ENTRY, OP-D-SERVICE and D-SERVICE, and SL-R already contains V-KITCHEN-PATIO. Their presence disproves a categorical absent-service-door or absent-patio-void finding; physical topology still requires the coherent remodel and independent checks.

Pre-publication conflicts remain explicit: aboveground stair centre (6,8.8) differs from cellar centre (6.23842,9.263605); first/roof levels remain3.5/6.7; principal slab outline remains19×21.5m. These are baseline observations only, not findings on the forthcoming rewrite.
