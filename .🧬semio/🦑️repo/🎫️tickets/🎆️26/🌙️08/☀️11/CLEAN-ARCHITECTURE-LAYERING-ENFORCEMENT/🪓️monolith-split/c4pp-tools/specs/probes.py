from specs.lib import *

CH = [
    ch('🧾', 'json-writer', 'json_writer', [(24, 116)],
       'JSON writer: the smallest JSON value type that can express a probe report, its escaping and rendering and the object, string and number constructors.'),
    ch('📋', 'report-and-readings', 'report_and_readings', [(120, 221)],
       'Report and readings: the probe version, the report record with its rendering, the engine names and the entity and reading records.'),
    ch('🖊', 'dxf-and-step-readers', 'dxf_and_step_readers', [(225, 405)],
       'DXF and STEP readers: the DXF entity reader and discriminant, the Part-21 parameter accessors and the STEP reader with its first reference helper.'),
    ch('⚖', 'compare-readings', 'compare_readings', [(409, 478)],
       'Reading comparison: the near-exact tolerance and the comparison of two readings into a difference report.'),
    ch('🧱', 'fixture-entity-builders', 'fixture_entity_builders', [(482, 565)],
       'Fixture entity builders: the point, layer, line, circle, arc, polyline, text, insert and solid builders and the pinned wall clock.'),
    ch('🖼', 'base-and-after-drawings', 'base_and_after_drawings', [(567, 713)],
       'Base and after drawings: the base drawing and the drawing after each recorded mutation.'),
    ch('♻', 'rebuilds-and-step-writer', 'rebuilds_and_step_writer', [(715, 839)],
       'Rebuilds and the STEP writer: the drawing rebuilds with replaced entities, layers and blocks, the STEP entity list and the deterministic Part-21 writer.'),
    ch('🏭', 'recipes-generate-and-verify', 'recipes_generate_and_verify', [(841, 945)],
       'Recipes, generation and verification: the DXF and STEP recipes, the fixture directories, fixture generation and the STEP round trip check.'),
]

SPEC = dict(
    orig='probes.rs',
    file=PROBES + '/🦀️.rs',
    children=CH,
    wiring='each',
    external=[],
)
