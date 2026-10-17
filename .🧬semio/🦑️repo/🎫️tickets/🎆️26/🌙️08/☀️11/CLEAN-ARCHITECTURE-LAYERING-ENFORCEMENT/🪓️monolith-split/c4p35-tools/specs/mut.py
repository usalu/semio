from specs.lib import *

SPEC = dict(
    orig='mut.rs',
    file=MUT + '/🦀️.rs',
    nest=False,
    external=[
        MUT + '/🗂️map/🦀️.rs',
        MUT + '/🌐️foreign/🦀️.rs',
        MUT + '/🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs',
        MUT + '/🧪️tests/🧪️outcome-code/🦀️.rs',
    ],
    children=[
        ch('📋', 'mutation-metadata-enums', 'mutation_metadata_enums', [(138, 268)],
           'Mutation metadata enums: the invertibility, diff participation, outcome class, composition and language surface enums with their value conversions.', glob=False),
        ch('🪪', 'leaf-descriptor-validation', 'leaf_descriptor_validation', [(270, 628)],
           'Mutation leaf descriptor: the descriptor record, its validation errors, the descriptor, roster and uniqueness validators and the owner, path and name helpers.'),
        ch('🔎', 'leaf-source-scope', 'leaf_source_scope', [(632, 871)],
           'Mutation leaf source scope: the provenance, domain operation and owner layout records, the source validator, the validated scope and the leaf trait.'),
        ch('🚦', 'outcome-and-message', 'outcome_and_message', [(879, 1219)],
           'Mutation outcomes and messages: the outcome codes, the message record and the mutation outcome with their value conversions.'),
    ],
)
