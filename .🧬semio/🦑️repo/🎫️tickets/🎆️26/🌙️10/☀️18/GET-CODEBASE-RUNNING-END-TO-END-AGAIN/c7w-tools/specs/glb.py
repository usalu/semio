from specs.lib import *
F = RW + '/🧊️renderer/🦴️glb-decode/🦀️.rs'
c = Cut(F, 'glb.rs').impl('impl GlbSchemaCursor {', [728, 1005]).impl('impl GlbMaterializeCursor {', [2091])
c.child('🔤', 'json-lexer-and-schema-types', 'json_lexer_and_schema_types', [(4, 350)],
        'Json lexer and schema types: the GLB JSON atom, token and lexer cursor and the accessor, view, primitive, material, texture, image, sampler, mesh, node and scene schema records with the numeric array.')
c.child('🗃', 'schema-output-and-cursor-state', 'schema_output_and_cursor_state', [(352, 727)],
        'Schema output and cursor state: the default primitive schema, the schema section and output with its accessors and the schema cursor with its constructor and step.')
c.child('🧮', 'schema-token-application', 'schema_token_application', [(728, 1004)],
        'Schema token application: the token, unsigned, literal, string and value application of the schema cursor.')
c.child('🏁', 'schema-finishing-and-plan-types', 'schema_finishing_and_plan_types', [(1005, 1283)],
        'Schema finishing and plan types: the finish methods of every schema record, the numeric arrays, the exact index conversion and the instance output, node plan frame and plan cursor state.')
c.child('📐', 'instance-plan-math-and-outline', 'instance_plan_math_and_outline', [(1285, 1617)],
        'Instance plan, math and outline: the instance plan cursor, the world frame and matrix math, the transform helpers and the outline edge accumulator.')
c.child('🧵', 'materialize-cursor-step', 'materialize_cursor_step', [(1619, 2090)],
        'Materialize cursor step: the materialize phases and cursor state with its constructor and the long materialize step.')
c.child('📖', 'materialize-readers-and-component-helpers', 'materialize_readers_and_component_helpers', [(2091, 2417)],
        'Materialize readers and component helpers: the bin span validation, the appearance build, the component readers, the asset hand-off, the close step and the texture and component helpers.')
SPEC = c.spec()
