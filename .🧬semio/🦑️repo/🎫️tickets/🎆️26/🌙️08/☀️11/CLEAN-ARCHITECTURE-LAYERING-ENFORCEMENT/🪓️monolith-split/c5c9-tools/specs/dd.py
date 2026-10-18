from specs.lib import *
DD = OD + '/🗣️dsl/✨️derive'
CH = [
    ch('🔖', 'mutation-source-authority', 'source_authority', [(9, 319)],
       'Mutation source authority of the dsl derive expansion: the source authority records, the authority locators and schemas, the common and aggregate authority readers, the domain owners and the path normalisation, no-follow reads and workspace root lookup.'),
    ch('🧾', 'mutation-leaf-descriptor', 'mutation_leaf_descriptor', [(324, 472)],
       'Mutation leaf descriptor of the dsl derive expansion: the leaf descriptor enums and JSON record, the descriptor parser with its key, string, number, kebab and surface readers and the descriptor emitter.'),
    ch('🪪', 'mutation-leaf-attrs-and-schema-index', 'mutation_leaf_attrs_and_schema_index', [(477, 818)],
       'Mutation leaf attrs and schema index of the dsl derive expansion: the leaf attribute parser, the portable and workspace paths, the withdraw only check, the leaf expansion with its inverse rows, the payload schema path and the schema document index.', glob=False, uses=['use quote::quote;', 'use super::{BTreeMap, Data, DeriveInput, Fields, HashSet, MUTATION_SCHEMA_INDEX_LOCATOR, MUTATION_SCHEMA_INDEX_SCHEMA, MutationLeafJson, MutationSourceAuthority, Path, PathBuf, TokenStream, emit_mutation_leaf_descriptor, fs, mutation_authority_locator, mutation_authority_no_follow, mutation_authority_relative, mutation_authority_workspace_root, mutation_source_authority, parse_macro_input, parse_mutation_leaf_descriptor};']),
    ch('🏗', 'document-diff-and-composite-expansion', 'document_diff_and_composite_expansion', [(821, 899), (1284, 1403)],
       'Document, diff and composite expansion of the dsl derive expansion: the dsl document, diff text and diff binary expansions, the composite attribute parser, the composite mutation expansion and the kebab helper.'),
    ch('🔁', 'mutations-expansion', 'mutations_expansion', [(902, 1280)],
       'Mutations expansion of the dsl derive expansion: the mutations attribute parser, the derive mutations expansion, the mutations expansion with its payload law test and the relative path helper.', glob=False, uses=['use quote::quote;', 'use super::{Data, DeriveInput, Fields, MutationAggregateSourceAuthority, Path, TokenStream, Type, mutation_aggregate_source_authority, mutation_authority_canonical, mutation_authority_relative, mutation_authority_workspace_token, mutation_leaf_include_path, parse_macro_input, to_kebab};']),
]
SPEC = dict(orig='dd.rs', file=DD + '/🦀️.rs', children=CH, expect_extra=[u for c in CH for u in c.get('uses', [])], external=mounted(DD + '/🦀️.rs', orig='dd.rs'))
