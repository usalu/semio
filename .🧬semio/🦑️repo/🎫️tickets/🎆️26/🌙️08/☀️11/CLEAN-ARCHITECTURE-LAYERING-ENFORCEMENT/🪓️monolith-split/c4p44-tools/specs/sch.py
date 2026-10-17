from specs.lib import *

CH = [
    ch('🧭', 'core-and-mutation-tool-schemas', 'core_and_mutation_tool_schemas', [(32, 192)],
       'Core and mutation tool schemas of the os.mcp registry: the inference job wire limits, the schema dialect, the wire stamp and the context, capability, action and transaction shapes.', glob=False),
    ch('🛡', 'untrusted-content-schemas', 'untrusted_content_schemas', [(198, 325)],
       'Untrusted content of the os.mcp registry: the untrusted content schema, notice, source, authors, revision and provenance records and their shapes.'),
    ch('📄', 'artifact-tool-schemas', 'artifact_tool_schemas', [(331, 488)],
       'Artifact tool schemas of the os.mcp registry: the open, create, validate, snapshot and export input and output shapes.'),
    ch('🖥', 'ui-and-catalog-tool-schemas', 'ui_and_catalog_tool_schemas', [(494, 701)],
       'UI and catalog tool schemas of the os.mcp registry: the focus, reveal, conversation and job shapes and the capability action, dialog and artifact create template shapes.'),
    ch('💡', 'inference-tool-schemas', 'inference_tool_schemas', [(707, 828)],
       'Inference tool schemas of the os.mcp registry: the list, get, submit, job handle, approve, job output and run shapes with the original retained turn schema.'),
    ch('📡', 'artifact-inference-wire', 'artifact_inference_wire', [(834, 927)],
       'Artifact inference wire of the os.mcp registry: the wire version, the budget, cache mode, request and result records and the hub inference approval schema.'),
    ch('📚', 'schema-catalog', 'schema_catalog', [(935, 1165)],
       'Schema catalog of the os.mcp registry: the schemas registry, the committed mirror document and the tool output schema.'),
    ch('🧾', 'scope-exports-and-validation', 'scope_exports_and_validation', [(1171, 1283)],
       'Scope exports and validation of the os.mcp registry: the format leaves, the scope export declaration and the validator compiler.'),
]

LAST = CH[-1]['folder']
OLD = 'rust: include_str!("../🦀️.rs"),'
NEW = 'rust: concat!(include_str!("../🦀️.rs"), ' + ', '.join('include_str!("../%s/🦀️.rs")' % c['folder'] for c in CH[:-1]) + ', include_str!("🦀️.rs")),'
OLD_LINE = 'const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: "", json_schema: SCHEMA_MIRROR_JSON, proto: "" };'
NEW_LINE = OLD_LINE.replace('rust: include_str!("🦀️.rs"),', NEW)

SPEC = dict(
    orig='sch.rs',
    file=SCH + '/🦀️.rs',
    children=CH,
    keep_empty=('🧪️Tests',),
    patch={LAST: [(OLD, NEW)]},
    expect_missing=[OLD_LINE],
    expect_extra=[NEW_LINE],
    external=[],
)
