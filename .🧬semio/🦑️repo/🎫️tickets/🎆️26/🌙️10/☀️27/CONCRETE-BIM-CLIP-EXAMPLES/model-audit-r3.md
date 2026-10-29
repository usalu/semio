# Villa Savoye Model Audit R3

## Sources

Shared authored source: clip/🧱️model/🔣️.json. Schema: 🧬️schema/🔣️.schema.json. API: 🧱️model/🐍️.py. All drawing modules consume building_geometry. This is repository evidence, not a measured historic survey.

## Omissions

Ground has only hall, garage, boiler spaces; no explicit ground walls, entrance/garage doors, staff rooms, laundry, WC, stairs or stair void. First has salon and terrace only, four exterior walls/windows; bedrooms, kitchen, bathrooms, partitions, terrace glazing and doors are absent. Roof has one solarium space and curved screen; no access stair, parapets or screen opening. All 25 pilotis stop at first soffit. Dimensions 21.5 by 19 m, levels 0/3.5/6.7 and overall 9.4 m are inferred/assumed, not surveyed.

## Integration Constraints

- _wall_geometry requires an IfcWindow for every opening; door fillings currently fail. building_geometry omits wall storey/role and opening filling class. It hardcodes CW-1 and assumes ramps exist.
- Core catalogue has no doors/stairs. Plan draws ordinary walls only at first floor and derives ground walls from spaces. Its cut height depends on first wall/opening globally.
- sheet.ground_walls derives synthetic enclosure edges by exact space-ring matching. Replace when explicit walls are authored.
- Axon _build creates opaque ground wall tiles from non-hall room rings and treats hall as floor only. Actual walls are full boxes with overlaid windows, so ribbon openings are not cut out of the main wall solids. Floor/roof selection takes first slab of each kind.
- Section/elevation derive ground enclosure separately and assume first slab/wall for label bands.
- wall_frame points thickness toward building centre, unsafe for arbitrary interior partitions; adopt explicit centred footprints consistently.
- represent supports columns, standard walls, slabs, windows/openings. STEP supports limited classes; mesh is column-specific.

## Consumers and Tests

Consumers: visual core/sheet/plan/section/axon, visual analysis/panels, chapters objects, representations, information, models, exchange, consistency, and Savoye aggregate mixin. Tests: clip/🧪️tests/🐍️.py plus portable cases and presentation cases. Tests hardcode 25 pilotis and class counts; structural analysis assumes grid supports and floor shell. Adding upper columns requires level-aware support selection. Update relations, discipline memberships, type assignments, fixtures and analysis loss accounting together.

## Storey Work Boundaries

One integrator owns central JSON/schema/API/renderers. Storey contributors should own separate definition modules or source fragments: ground enclosures/services/entry/garage/stair; first domestic plan/terrace glazing/upper supports; roof parapets/screen opening/access arrivals. Each contributes bilingual names, stable tags, explicit geometry/provenance and portable test cases. Integrator serializes fragments and handles whole-model invariants. Do not concurrently edit the single source JSON from all storey tasks.
