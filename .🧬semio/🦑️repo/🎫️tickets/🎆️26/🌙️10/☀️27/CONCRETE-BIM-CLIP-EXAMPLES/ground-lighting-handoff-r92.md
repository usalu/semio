# Ground Services and Original Lighting R92

Ground programme, hollow sanitary construction, native classes, original lighting witnesses and construction qualifications are recorded in ground-services-plan-r92.md, sanitary-programme-register-r92.md and kitchen-exterior-light-anchors-r92.md.

## Authored source

Two servant rooms and laundry now follow a coherent partition with service WC; chauffeur bathroom has a rounded WC niche, hollow bidet, four-foot freestanding bath and relocated basin. Hall basin has an elliptical hollow pedestal beside column19. Four basins preserve original GUIDs. Garage boundaries/support axes are preserved. Radiators are proper IfcSpaceHeater products and servants have tiled basin niches.

Twelve original photo/text-attested lights have physical composite IfcLightFixture bodies: two Zeiss hall bowls, column19 basin light, two Albalite ceiling bodies, salon inverted-V trough and two hangers, two kitchen wall reflectors and four exterior spots. Source-presence evidence is distinct from inferred fabrication and actual mount-point selection. Original wattage, photometry and wiring remain unknown; budget-only/private and 1952 fixtures were not added as original.

## Changed files

Model root: 🔣️.json (targeted programme and fixtures); 🧬️schema/🔣️.schema.json (light class, lighting system, closed mounting and composite envelope); 🐍️.py (light in service/quantity/building dispatch tuples); 🔌️exchange/🐍️.py (native light service export dispatch). Existing parent sanitary/end-plane/L/chimney changes preserved by fresh reads.

Model tests: 🚿️ground.json, 🚿️ground.schema.json, 🚿️ground.py; 💡️lights.json and 💡️lights.py. Language-neutral ground/lighting inventory drives native acceptance. GEOS Simpson integration independently verifies sanitary hollow lofts; SciPy hulls independently verify light occupied volume and declared actual host/fixture contact.

## Validation

Ground programme/disjoint spaces, all hollow quantities, and all-fixed-material 0.6m diameter by2.1m body routes passed independently; fresh combined rerun in progress. Lighting identity RED correctly failed missing native class before publication. Protocol passed; all12 EN/DE native identities passed146.161s. Independent attachment test caught a salon point declared at centre rather than physical hanger; corrected toX10.25. Kitchen wall contact refinement is in progress. No producer guard weakened.

Generated lighting-physical-r92.png was visually inspected: bowl, Albalite steps, short wall arms, salon trough/hangers, directional exterior heads are physically present. It is a geometry projection diagnostic with inferred dimensions labelled. Independent strict SDK audit for representative taper/section/mesh lights requested from audit owner.

Retained ticket inputs/runners: ground-services-tests-r92.py, lighting-tests-r92.py, lighting-hosts-r92.py, light-wall-contact-r92.py, lighting-atlas-r92.py. Generated pass/failure JSON, manifests and PNG remain only under 🗑️generated for final parent review/cleanup. No runtime dependency added.

Native contact probe confirmed kitchen W-W inner surface X0.13; kitchen1 shifted−0.02m as a whole, retaining exact mounting contact. W-KITCHEN faceX4.725 confirmed. This corrects the earlier nominal face assumption without altering the source witness.

Fresh combined ground suite GREEN on501 products: GEOS hollow quantities4.642s; actual all-fixed-material WC body routes118.564s; programme/disjoint-space0.550s. Evidence generated/ground-services-green-r92.json. All12 native EN/DE light records GREEN146.161s; protocol GREEN.

Independent SciPy light material hull quantities and actual both-sided host attachment are GREEN for all12 lights203.422s after salon/kitchen contact corrections. A final expanded independent host-penetration guard rerun is active; no source writes pending.

FINAL light protocol and independent SciPy quantities/contact/positive host-penetration guards GREEN: physical contract233.750s, protocol0.001s. Both casing-mounted diagonal heads and all walls/soffits were checked against actual host solids; no positive material penetration. Evidence generated/lighting-final-green-r92.json. Ground/light delegated work complete; source501.
