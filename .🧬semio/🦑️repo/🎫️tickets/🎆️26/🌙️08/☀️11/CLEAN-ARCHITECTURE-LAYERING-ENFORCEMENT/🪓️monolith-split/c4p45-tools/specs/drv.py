from specs.lib import *

A1 = '#[derive(Default, Clone)]\nstruct ContainerAttrs {'
A2 = 'fn to_kebab(name: &str) -> String {'


def pre(t):
    assert t.count(A1) == 1 and t.count(A2) == 1
    t = t.replace(A1, '//#region 🔖️Attrs\n' + A1)
    t = t.replace(A2, '//#region 🔖️VariantHelpers\n' + A2)
    return t


CH = [
    ch('🏷', 'attribute-parsing', 'attribute_parsing', [(80, 204)],
       'Attribute parsing of the record derive: the container and field attribute records and their parsers.'),
    ch('🧩', 'type-shape', 'type_shape', [(207, 331)],
       'Type shape of the record derive: the field kind and the type classifiers for options, lists, maps and byte vectors.'),
    ch('🏗', 'record-field-codegen', 'record_field_codegen', [(335, 734)],
       'Record field codegen of the record derive: the field plans and the shared record and schema field fragments.'),
    ch('🧹', 'lifecycle-codegen', 'lifecycle_codegen', [(737, 866)],
       'Lifecycle codegen of the record derive: the controlled, encoding, retirement and retained projection fragments of a field plan.'),
    ch('🚦', 'emitters', 'emitters', [(868, 1170)],
       'Emitters of the record derive: the record, scalar and tagged variant expansions.'),
    ch('🌿', 'variant-helpers', 'variant_helpers', [(1173, 1296)],
       'Variant helpers of the record derive: the kebab-case conversion and the hygienic field locals and initializers.'),
    ch('🪢', 'receiving-and-projection', 'receiving_and_projection', [(24, 55), (69, 77), (1299, 1340)],
       'Receiving and projection of the record derive: the typed receiving fragments and the ordinary record projection expansion with its input record.'),
    ch('📚', 'borrowed-owner-codegen', 'borrowed_owner_codegen', [(1346, 1423)],
       'Borrowed owner codegen of the record derive: the static field identity tables and the record, scalar and variant owner metadata.'),
]

SPEC = dict(
    orig='drv.rs',
    file=DRV + '/🦀️.rs',
    children=CH,
    external=mounted(DRV + '/🦀️.rs'),
    pre=pre,
    pre_extra=['//#region 🔖️Attrs', '//#region 🔖️VariantHelpers'],
)
