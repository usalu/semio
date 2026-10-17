from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'reg.rs'), encoding='utf-8').read()
CUTS = [(765, 765, [973, 1068, 1272])]

CH = [
    ch('🪪', 'command-identity', 'command_identity', [(27, 198)],
       'Command identity of the registry: the verbs, the command id grammar, invocation parsing and formatting, taxonomy segments and the lexical predicates of declarations.', glob=False),
    ch('🧾', 'declarations', 'declarations', [(201, 302)],
       'Declarations of the registry: the parameter kind, the wire-shaped target, tool, compound, group, project and ticket declarations and the problem record.'),
    ch('📋', 'facts', 'facts', [(304, 387)],
       'Facts of the registry: the serialisable input of the registry build with its project, target, playground and ticket facts.'),
    ch('🧬', 'model', 'model', [(389, 504)],
       'Model of the registry: the command kind, origin, effect, destination, parameter, action, entry and request.'),
    ch('⚖', 'run-policy', 'run_policy', [(506, 572)],
       'Run policy of the registry: the graph reuse variable, the run policy, the file fingerprint and the graph basis.'),
    ch('🛂', 'tokens-and-validation', 'tokens_and_validation', [(575, 600), (602, 731)],
       'Tokens and validation of the registry: the name tokens of a text and the validation scope that reports problems of a declaration.'),
    ch('📚', 'registry-record', 'registry_record', [(733, 763)],
       'Registry record: the registry with its entries, index and axes, the playground constants and the playground target.'),
    ch('🧶', 'build-and-derive', 'build_and_derive', [(765, 972)],
       'Build and derive of the registry: the build from facts, the tool and compound derivation and the settling of parameters and ownership.'),
    ch('🔍', 'lookup-and-search', 'lookup_and_search', [(974, 1069)],
       'Lookup and search of the registry: the accessors, the find and search operations, the parameters of an entry and the graph basis.'),
    ch('🚀', 'resolve-and-launch', 'resolve_and_launch', [(1071, 1275)],
       'Resolve and launch of the registry: the resolution of an invocation into a launch and the process plan of an entry.'),
    ch('🩺', 'check-and-json', 'check_and_json', [(1277, 1344)],
       'Check and json of the registry: the consistency check and the launch plan and entry json projections.'),
    ch('🪶', 'entry-draft', 'entry_draft', [(1346, 1390)],
       'Entry draft of the registry: the draft a source states, the entry construction and the argument helpers.'),
]

SPEC = dict(
    orig='reg.rs',
    file=REG + '/🦀️.rs',
    children=CH,
    pre=split_impls_h(CUTS),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[],
)
