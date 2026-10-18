from specs.lib import *
F = STORE + '/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🦀️.rs'
CH = [
    ch('🗂', 'envelope-field-constants-and-authority-traits', 'envelope_field_constants_and_authority_traits', [(17, 237)],
       'Envelope field constants and authority traits: the envelope field names and owned schema, the field decode step and reservation, the field, snapshot, mutation and conflict authority traits and the history entry decoder.'),
    ch('🧰', 'owned-spr-mutation-target-and-array-authority', 'owned_spr_mutation_target_and_array_authority', [(239, 614)],
       'Owned SPR mutation target and array authority: the owned SPR edit fields, the mutation target, the mutation array state and authority and the edit authority struct.'),
    ch('🧵', 'owned-spr-edit-authority-and-history-decoder', 'owned_spr_edit_authority_and_history_decoder', [(616, 1035)],
       'Owned SPR edit authority and history decoder: the edit authority operations, the history entry authority, the edit decoder and the repository history entry decoder.'),
    ch('🎟', 'field-decoder-registry-and-empty-array-authority', 'field_decoder_registry_and_empty_array_authority', [(1037, 1485)],
       'Field decoder registry and empty array authority: the history entry authority operations, the field decoder trait, ticket, registry, lease and returned decoder and the empty array authority.'),
]
SPEC = dict(orig='osf.rs', file=F, children=CH, wiring='block', external=crate_ext(STORE + '/📦️codec/🧬️owned-schema/🧱️fields/🦀️.rs', F, orig='osf.rs', children=CH))
