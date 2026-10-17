from specs.lib import *

CH = [
    ch('🪪', 'capability-identity', 'capability_identity', [(23, 251)],
       'Capability identity of the catalog: the capability reference, owner, kind and audience types with their conversions and audience constants.'),
    ch('🧰', 'presentation-and-definition', 'presentation_and_definition', [(257, 402)],
       'Presentation and definition of the catalog: the tool exposure, the argument summary and presentation, the example, the source, the capability definition and the action input schema builder.'),
    ch('♻', 'framework-dedup', 'framework_dedup', [(408, 513)],
       'Framework dedup of the catalog: the framework-injected shell actions, the app action verbs and the framework capability definitions.'),
    ch('🏭', 'capability-builders', 'capability_builders', [(519, 802)],
       'Capability builders of the catalog: capabilities from actions, commands, contributions and OS commands, the contribution rows, the humanizer and the dialog and artifact creation capabilities.'),
    ch('📚', 'catalog-compile', 'catalog_compile', [(808, 1009)],
       'Catalog compile: the catalog source, its error, the compiled catalog and the compile entry points.'),
    ch('🔍', 'catalog-audit', 'catalog_audit', [(1015, 1236)],
       'Catalog audit: the gesture and destructive word lexicons, the audit finding and the declaration and source audits.'),
    ch('💬', 'description-law', 'description_law', [(1242, 1425)],
       'Description law of the catalog: the manifest schema, the description problems, findings and verbs and the description validator.'),
]

SPEC = dict(
    orig='cat.rs',
    file=CAT + '/🦀️.rs',
    children=CH,
    keep_empty=('🧪️Tests',),
    external=[],
)
