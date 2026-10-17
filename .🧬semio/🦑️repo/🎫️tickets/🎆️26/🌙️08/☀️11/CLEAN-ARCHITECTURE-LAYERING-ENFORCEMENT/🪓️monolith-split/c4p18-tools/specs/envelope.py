from specs.common import ch
E = '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/✉️envelope'
SPLIT = '\n\n    fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor)'


def pre(text):
    assert text.count(SPLIT) == 1, text.count(SPLIT)
    return text.replace(SPLIT, '\n}\n\nimpl Generation3dRetainedMutationOwner {\n' + SPLIT[2:])


SPEC = dict(
    orig='envelope.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    pre=pre,
    pre_extra=['}', 'impl Generation3dRetainedMutationOwner {'],
    children=[
        ch('📑', 'owner-vocabulary', 'owner_vocabulary', [(60, 184), (1012, 1016)], 'Vocabulary of the retained mutation owner: the widget, synapse and dictionary owners, the frame and destination enums and the fixed-depth owner state with its drop guard.'),
        ch('🔩', 'owner-assembly', 'owner_assembly', [(186, 606)], 'Assembly half of the retained mutation owner: construction, the string, symbol, JSON and DSL value assembly and the dictionary, widget and record frames.'),
        ch('📨', 'owner-accept', 'owner_accept', [(608, 1010)], 'Token intake half of the retained mutation owner: the token dispatch that fills the typed frames, the handoff and the close steps.'),
        ch('🎬', 'mutation-session', 'mutation_session', [(1018, 1243)], 'Mutation session of the owned envelope catalog: the session phases and the bounded mutation decode session with its drop guard.'),
        ch('📸', 'snapshot-authority', 'snapshot_authority', [(1245, 1507)], 'Snapshot decode authority of the owned envelope catalog: the pack snapshot state machine and its fresh-envelope field authority.'),
        ch('🔑', 'mutation-decode-authority', 'mutation_decode_authority', [(1509, 1832)], 'Mutation decode authority of the owned envelope catalog: the mutation record decode state machine, its field authority and the rejected conflict authority.'),
        ch('📇', 'owned-field-catalog', 'owned_field_catalog', [(1834, 1878)], 'Owned field catalog of the Generation3d envelope: the exact field catalog, the decode owner bundle and the catalog trait impl.'),
    ],
    external=[],
)
