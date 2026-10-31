# Final Architectural Audit

Read-only review, 2026-10-29. Snapshot taken while execution and visual agents were still editing; this is a pre-gate audit, not a declaration that tests or final renders pass. Only this Markdown file was written.

## Evidence and Method

Reviewed current model JSON, Python assembly source and README against ticket exploration, architecture sources and primary Domus 1973 images domus-section-full.jpg, domus-5.png and domus-6.png. Also inspected current nakagin-detail-gallery.png and nakagin-orthographic-gallery.png. Generated galleries may precede the last source write; do not treat them as a final render gate.

Authoritative survey count A76/B64 supersedes earlier SFMOMA70/70 allocation. JSON currently has A floors3–13 counts2,7,7,7,8,8,7,8,8,7,7 and B floors3–11 counts2,8,8,7,8,8,7,8,8. Counts and their sum are credible. Historical per-room transforms remain inferred from repository compact slot frames; the 154-unit repository fixture cannot itself certify a140-unit historical arrangement.

## Required Before Final Gate

1. **Core roof and upper doors use incompatible heights.** In inspected source towers() sets core height=storeys×2.75: A35.75/B30.25m. Highest slot bases are34.0833/28.5833; capsule crowns36.5833/31.0833 and door heads36.2333/30.7333. core_assembly clamps wall-opening cuts to t.height and installs a solid roof slab there. Thus top capsule entrances intersect or rise above the core roof unless the in-progress clipping fix changes this contract. Validate full door-volume/roof intersection, not only facade opening rays.
2. **Podium shaft penetrations.** podium_assembly builds complete polygon prisms at z0/4/7.5 without core/lift/stair voids. These solids cross both cores. Validate circulation through podium slabs and the intentionally low two floor3 capsules; do not accept all closed solids as proof of usable access.
3. **Superseded factual prose.** The inspected README says historical70/70, paired facade groups, A upper west addition/B top east omission and3.875m corridor. JSON reconstruction-layout/bridge-clearance notes repeat this earlier geometry. These conflict with the actual A76/B64 compact three-phase layout. Update both languages and related cases/consumer narrative. The architecture-sources.md initial museum allocation also needs a clear superseding note so its chronological source report is not mistaken for current truth.

## Source-Specific Fidelity Limits

- **Circulation:** Domus plan shows the lift in the central core and turning stair/circulation around it. Current generic lift shaft is at x+1.5 near a core side, with paired straight stair flights. Side-entry and rear-entry doors now exist, but internal route topology and levels have not been demonstrated to follow primary plans. Keep this expressly inferred until reconstructed from plans.
- **Basement/foundations:** Domus section explicitly includes basement and substructure below ground; no basement/foundations are present in the inspected assembly. This is a substantial omission if the requested scope is the complete building rather than above-ground teaching exterior.
- **Podium:** The irregular upper outline and recessed ground level improve the silhouette. Current ground perimeter is a uniform .88 scale, fully glazed on every edge. Primary elevation shows open pilotis and opaque core/entrance zones; source-specific ground-plan recesses, cafe/office fitout and main entry are not replicated. The gallery shows two curtain-wall bands rather than the primary open ground level.
- **Capsule interior:** Domus5/6 show the characteristic moulded wet-room with bathtub, basin and toilet, curved/chamfered bath door, integrated cabinet/TV/audio/telephone controls, fold-down desk and storage wall. Current simplified rectangular bathroom partitions/toilet/basin and generic furniture omit the tub and integrated equipment. A–D proposed programs cannot be presented as historical interior variants. Six shell door/window families improve exterior variety but do not authenticate those programs.
- **Roof heads:** The tall diagonal roof forms now match the broad primary silhouette much better than the former low frustum. However capital_solid is an opaque filled wedge, not a hollow enclosure with original structural lattice and equipment layout. Generic added side beams, louvres and water tank are illustrative. Check both roof dimensions and orientation separately against primary section, rather than assume one shared11.36m head is exact for both.
- **Capsule skin and details:** Nominal envelope2.5×4×2.5m remains distinct from complete hardware bbox and M+ restored artifact. Primary details show shaped sheet-metal/lining framing, vents, integrated equipment and specific service connections. New shell/lining/truss/bolts are useful named geometry, but generic truss diagonals, thicknesses, fastening plates and surface finish are not an as-built survey.
- **Exterior materials:** Gallery capsule program colors are pedagogical overlays, not historical facade finishes. Label color coding explicitly. Roof signage, specific weathering, blinds/privacy screens, external service inventory and original facade material seams remain incomplete.

## What the Current Evidence Supports

The model supports a detailed, shared, evidence-informed teaching reconstruction with corrected140-unit inventory, distinct tower floor counts, three-phase capsule levels, six door/window families, long-wall and end-wall portholes, ragged attachment groups, tall diagonal roof heads, irregular podium and explicit bridges. It does not support 'perfect replica', 'surveyed model', 'all room positions verified', or 'all interior details reproduced'.

Tests of count, manifold solids and collision avoidance establish consistency of the authored model. They do not establish historical identity. Final acceptance still needs fresh four-direction exterior renders and representative full-detail capsule/core/podium sections after the last writes, with the geometric blockers above resolved or explicitly scoped by the coordinator.

## Primary References

- https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html
- https://readyfor.jp/projects/time_capsule_2072/announcements/241581
- https://readyfor.jp/projects/time_capsule_2072/announcements/265901
- https://gluon.tokyo/nakagin/index_en.html
- https://gluon.tokyo/nakagin/models/NakaginCapsuleTower.glb (merged normalized scan; no semantic room identities or documented metre scale)
- https://annualreview.bdp.com/stories/second-life-for-an-icon/
- https://www.mplus.org.hk/en/collection/objects/unit-a806-nakagin-capsule-tower-19701972-tokyo-japan-2024104/


## Consumer Claims And Test Audit — In Progress

This later read-only pass covers shared model, all Grasshopper/Excel clips, bilingual transcripts and related README/JSON. Structural corrections are being implemented by another agent; none is marked resolved until re-read after completion.

### Newly identified regressions

- Grasshopper capsule source lines103–104 and transcript EN/DE line91 say A–D alter window offset and diameter. Current canonical types all retain1.3m diameter and zero window_u. Either the teaching demonstration must explicitly override these parameters, or the narration must describe actual program/fitting changes. The tower swap clause already correctly preserves1.3m.
- Grasshopper tower narration and on-screen labels call inferred masks/world door ports/capsule levels measured. Exact source clauses: items, orient, count, mistake, towers; source lines54–77,177,205–206,228. Bilingual transcript lines125–147 repeat these. Counts A76/B64 are measured, but JSON's placement_basis remains assumed/derived and every exact slot comes from the154-unit repository reference. Prefer authored/inferred masks and door-port placement, with measured counts as the constraint.
- Grasshopper trees branch clause/source61–62 and transcript213 describes a measured floor layout. Only lengths/count constraints are known; exact branch placement remains inferred. Its measured totals/per-storey counts language is supportable separately.
- Grasshopper README EN5/13 and DE67/75 repeats measured capsules/masks language. Qualify these as inventory totals and inferred placement constrained by documented floor counts. Current four-program Excel narration already says teaching model; do not replace these program categories with shell-family names.
- Shared tree() docstring says two slots per branch, although branches are ragged; shared capital drawing docstring still says frustum. These misleading implementation descriptions should match the final API.

### Existing checks and missing independent coverage

- Existing entry-wall test checks one point at localz1.0 and only core_wall roles. It does not test door-head/roof clearance or a complete pedestrian route, and therefore cannot detect all roof, slab, lift-wall or stair obstructions.
- Existing structure/capsule check includes bridge, bridge_rail, capital and roof roles, but excludes floor, lift_shaft, stair_tread, core_wall, landing and podium_glazing. This is not a general interior-access check. Bounding-box checks are conservative and may reject valid hole geometry; pair solid checks with explicit opening volumes.
- Missing tests should cover full width/height entrance clearance, capsule threshold to a reachable core landing, central lift access at each three-phase landing, stairs connected to landings, podium shaft voids, below-ground route, and final highest-door/roof relation. New native structural/fitting parts need portable case roles/dimensions plus third-party geometry validation, not only required-role presence.
- Existing independent case inventory and Python tests cover140 placements, nominal portholes, six shell families, frame transformations, counts and closed positive-volume parts. They remain useful but cannot prove historical identity.

### Resolution status

| Finding | Current status |
| --- | --- |
| A76/B64 inventory and floor-count schedule | Confirmed in JSON |
| Core top/roof/upper-door conflict | Reported; awaiting post-fix re-read |
| Podium shaft openings | Reported; awaiting post-fix re-read |
| Basement/pilotis/central lift/wetroom/equipment | Implementation reported in progress; not verified |
|70/70 and paired-facade documentary claims | Still present at this read |
| Unsupported measured layout/ports claims | Reported above; awaiting edit |
| Capsule window-parameter narration | Reported above; awaiting edit |
| Final renders/tests | Awaiting final writes; no pass asserted |


### Additional dimensional attribution caution

M+ accession lists only overall255×270×420cm, without explicit axis labels in accessible text. Current detail.measured_a806 assigns width2.55,height2.7,length4.2. That axis mapping is not established by the page. Museum catalog dimensions commonly use height×width×depth; repository capsule bounds also suggest approximately2.63m width/2.53m height including attachments. Retain the raw museum dimension sequence or verify its convention before assigning sourced width/height fields. Do not silently assert either axis mapping.

Domus section repeated vertical dimension appears approximately2.90m; current2.75m pitch is an assumed repository-kit parameter. Keep its provenance explicit until the primary dimension chain is read and calibrated; count agreement does not authenticate vertical scaling.


## Post-Fix Source Re-Read

Re-read after coordinator's structural-save message. Confirmed source improvements: core height now follows highest capsule crown/bridge head; lift is central; stairs rotate around it; podium slabs have actual clipped core holes; ground glazing is omitted for open pilotis; foundation/basement parts exist; default reference interiors now include curved wetroom, bathtub, TV/audio/telephone, floor bed and storage. Explicit interior=study preserves proposed program geometry separately. These resolve the prior absence findings at source level; they do not by themselves validate usable circulation or final rendered appearance.

### New functional blockers

1. core_assembly constructs four complete perimeter floor strips at every access elevation. Access includes all capsule sill phases, typically0.9167m apart. Consequently slabs occupy the headroom of other facade doors/circulation: a full ring every phase yields less than0.74m clearance, and slabs cut through full-height doorway approaches. Local staggered landings/floor sectors are required, with full doorway-volume and route-headroom tests against floor, landing, stairs, lift and wall solids. Wall-point tests do not cover this.
2. foundation_assembly's two basement flights repeat the same x/y direction at successive elevations. The first ends at y+1.2 and the next starts at y-1.2 without a connecting landing. Core/lift wall construction still begins atz0, while basement floor removes the complete core footprint. Below-ground circulation remains disconnected rather than a complete accessible route.

The diagonal roof envelope remains the intended high oblique wedge. Its interior structure/equipment is illustrative and already scoped in this report; no additional historical-fidelity claim is made. Latest README/narration corrections are expected separately and remain pending re-read.


### Podium-relative height correction

Coordinator identified current floor3 capsule base4.75/crown7.25 beneath podium roof7.5. Re-inspected Domus section/elevation and primary photo domus-4.png: isolated lowest capsules stand ABOVE the office/podium roof; fuller rows begin roughly one capsule pitch higher. Therefore current low capsules are geometrically/source-specifically wrong. Raising the entire capsule/bridge schedule by one assumed2.75m pitch, so lowest capsule bases are7.5m, is consistent with the primary visible hierarchy. It does not independently authenticate the assumed pitch or total building height. Ground office, capsule floor3 and bridge datum changes must stay synchronized with core access levels and tests.


## Handoff Status Before Fleet Reassignment

Coordinator reports all findings assigned. Execution applied+2.75m capsule/bridge world shift and an explicitly assumed2.8m service overrun, yielding A core42.133m and top53.493m. This numerical alignment is not presented as measured. Structural circulation/headroom fixes, fresh render coverage, consumer narration/README synchronization and final tests remain in progress. No final pass is asserted by this audit. The next targeted gate should re-read final source/tests, inspect corrected podium-height and core/basement cutaways, and confirm unsupported measured-mask/window-change prose is gone in both languages and transcripts.
