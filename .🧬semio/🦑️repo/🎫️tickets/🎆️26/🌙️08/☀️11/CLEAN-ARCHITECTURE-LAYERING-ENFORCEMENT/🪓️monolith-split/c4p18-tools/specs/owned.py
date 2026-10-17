from specs.common import ch, V
E = '✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned'
SPLIT = '\n\n\n    fn applied_id(&self'


def pre(text):
    assert text.count(SPLIT) == 1, text.count(SPLIT)
    return text.replace(SPLIT, '\n}\n\nimpl GisMapStoreInitializationAuthority {\n' + SPLIT[3:])


SPEC = dict(
    orig='owned.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    pre=pre,
    pre_extra=['}', 'impl GisMapStoreInitializationAuthority {'],
    children=[
        ch('♻', 'retirement-and-clone', 'retirement_and_clone', [(93, 288)], 'Retirement and retained clone of the GIS map owned values: the snapshot and mutation retirement factories and the bounded snapshot clone authority.'),
        ch('🏁', 'initialization-authority', 'initialization_authority', [(9, 91), (290, 664)], 'Store initialization authority of the GIS map document: the authority state, its phases and the stepped initialization that applies the genesis envelope.'),
        ch('🔚', 'initialization-close', 'initialization_close', [(666, 874)], 'Store initialization close of the GIS map document: the bounded close and retirement of the initialization authority and the initialization job constructor.'),
        ch('🔡', 'owned-field-decode', 'owned_field_decode', [(925, 1126)], 'Owned field decode of the GIS map envelope: the pack decoders and the macro-generated snapshot and mutation field decode authorities.'),
        ch('📇', 'owned-field-catalog', 'owned_field_catalog', [(876, 917), (1128, 1165)], 'Owned field catalog of the GIS map envelope: the exact field catalog, the decode owner bundle and the rejected conflict authority.'),
    ],
    patch={
        '🔡' + V + 'owned-field-decode': [
            ('        struct $authority {', '        pub(super) struct $authority {'),
            ('            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {', '            pub(super) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {'),
        ],
    },
    external=[E + '/🧪️tests/🔬️unit/🦀️.rs'],
)
