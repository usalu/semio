# Auxiliary Facade Structure R23

## Scope and Source Evidence

Implemented source-bound auxiliary L lintels, metal U inner-leaf supports, 6cm RC shelves, lower parapet stiffeners, corner bearings and distinct upper cavity hanging ties. The main primary beam register remains separate. No native rendering or exact historic profile recovery is claimed.

Read facade-auxiliary-audit-r23.md, facade-r18.md, source-audit-r17.md and primary-drawing-recovery-r21.md. Safely viewed actual local register-r9-section22.png and section23.png: longitudinal Annex5 maps cantilever endpoints through columns30–2 and31–3 with head+5.21 and lower parapet+3.855. Transverse Annex6 maps garden posts16/22 with sill+4.06 and head+5.21. Domestic opposite edge has a separate+3.955 shelf/sill and+3.41 annotation, so its assumed sill was retained rather than indiscriminately corrected. No PDF Worker extraction or shell command was used.

Source dimensions: L outer envelopes20×22cm on cantilevers and16×22cm elsewhere, source29/26cm upper screens, source6cm shelf thickness and14cm upstand height. L toe/web55mm, U65×35×4mm and footprints remain inferred. Nine-centimetre bare post/tie cores are inferred independently of the approximately11cm finished member inference;6cm denotes the apparent reinforcement cage, not a concrete core. Exact covers and local finishes are unresolved.

## Physical Contract

Schema adds native IfcBeamType.sweep_profile with orthogonal L/U/rectangle shapes and independently attributed quantity dimensions. Source section_sweep instances bind a host wall, actual opening datums, inferred horizontal stations and exterior-relative depth. Shared facade module expands profiles into disjoint physical prisms. Primary slabs/beams/columns own common volumes; auxiliaries then use explicit priority and tag order, independent of source ordering. Wall leaves and finishes subtract those occupied members through the existing shared wall ownership interface.

The three-piece metal U remains hollow. Cantilever U placement225mm from exterior is explicitly inferred and selected so its three legs survive adjacent L-toe ownership. L/U/RC sections are never replaced by filled rectangular metal envelopes. Source support graph distinguishes supports from supported_by. U suspension remains expressly probable/unresolved; no measured straps or station layout is invented.

Parapet construction bands now end the lower parapet at sill−20cm, reserve6cm for shelf, then retain14cm clay upstand. The shelf has a separately inferred footprint rather than treating6cm as its width. Ordinary shelves infer20cm width; garden shelf infers12cm. All seven model representations, occupied quantities and drawing beams consume the same shared parts.

## Garden and Kitchen Topology

W-E now encloses only the salon from y0.3 to6.081; OP-E/RW-E end at y5.931 with remaining jamb inferred. W-GARDEN-E spans y6.081–15.5 and has an unfilled native OP-GARDEN-E, preserving the open hanging garden. Its16cm single-leaf finished screen is source-bound, with inferred12cm clay plus existing25/15mm finishes. W-KIOSK-E spans y15.5–21.2 and retains native glazed OP-KIOSK-E/RW-KIOSK-E; jambs and exact module layout remain inferred. Existing aerodynamic posts, garden cornice and roof geometry were retained.

The existing empty OP-KITCHEN-PATIO-FRAME carries its own16×22cm no-curtain-recess auxiliary lintel, U, shelf and hanging tie. It does not duplicate the primary11–17 beam.

S/N/E/garden/kiosk source sill is now+4.06 and head+5.21 (first finish3.17, relative.89/2.04). Domestic W heads use+5.21; W sills remain assumed. New sources include47 auxiliary native beams, two additional walls, two openings, one kiosk filling, native beam types and garden wall material/type; final total currently446 elements is an authored register count, not historic acceptance. Relationships, model memberships, containment, void/fill/type/material relations were updated.

## Validation

Test-first language-neutral JSON fixtures and independent Shapely/GEOS contracts were added before implementation; the first run failed for missing auxiliary module as expected. Four initial contracts passed on the37-member cohort: L/U independent section differences and sweep area, reverse-input deterministic ownership and occupied volume, seven representations/clear apertures, and shelf/upstand dimensions. The final47-member garden cohort adds open-versus-filled garden topology, physical leaf/member material nonoverlap, complete cantilever U leg preservation and explicit source support graph. Final run results will be appended below.

## Files

- Model source 🔣️.json; shared model 🐍️.py (scoped solid dispatch only).
- Model 🧬️schema/🔣️.schema.json.
- New model 🏗️facade/🐍️.py.
- New model 🧪️tests/🏗️facade.py and 🏗️facade.json.
- Clip 🧪️tests/🐍️.py: standard contract registration.

## Remaining Precision Limits

Exact L rebates/toes, U gauge/strap layout, shelf projection, finished-to-core dimensions, true bay station measurements, sash/rebate and pane-module layout remain unmeasured. Domestic section23 shelf/sill registration needs finer interpretation before changing those sill values. Tests certify occupied authored geometry and consumer agreement, not measured as-built exactness. Native IFC export validation is coordinated with the facade exchange author; native Manim remains the parent gate.

## Drawing Discipline Integration

Shared solid/stair geometry now exposes authored metadata.domains. Axonometric, plan and section/elevation consumers use those explicit domains rather than classifying every beam as structure-only; the facade auxiliary members therefore remain visible in both architecture and structure drawing sets. Modified drawing source files are savoye/🧊️axon/🐍️.py, savoye/📋️plan/🐍️.py and savoye/✂️section/🐍️.py.

## Final47-Member Cohort Results

All7 facade contracts passed on the final source in86.157seconds through the portable Python/GEOS harness. The independent material-leaf/auxiliary intersection audit took72.421seconds including fresh geometry preparation; remaining source topology, profile, shelf, reverse-order ownership, all7representations/clear aperture and support-graph checks completed by86.157seconds. This final run excludes broad parent consistency and native rendering. Geometry-only cache keys reduced repeated ownership lookup cost while retaining all geometry mutation dependencies.

## Final Independent Primary Junction Oracle

The eighth contract passed: at every source vertical band, GEOS compares raw auxiliary occupied slices minus the actual primary column/beam/slab union with the authored final auxiliary union. Symmetric differences and material intersections remain below1e-8m². This checks both missing junction volume and primary duplication, beyond object counts and pairwise auxiliary checks. Four modified consumer modules compile; fresh shared solid/stair discipline propagation also passed. Eight facade contracts are registered in the standard BIM test hook.
