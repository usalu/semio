from specs.lib import *
PUB = OD + '/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication'
H = 89
B = [177, 309, 465, 598]
CH = [
    ch('🔁', 'stall-witness-and-unit-advance', 'stall_witness_and_unit_advance', [(42, 177), (1010, 1020)],
       'Stall witness and unit advance of the typed operation publication: the stall streak and the interaction write check, the advance of one publication unit and the window config refresh of a published receipt.'),
    ch('🏃', 'mounted-run-and-pump', 'mounted_run_and_pump', [(179, 312)],
       'Mounted run and pump of the typed operation publication: the mounted run with its admitted variant, the publication pump and the mounted interaction unit.'),
    ch('🖱', 'inline-interaction-and-latest-wins', 'inline_interaction_and_latest_wins', [(314, 471)],
       'Inline interaction and latest wins of the typed operation publication: the inline interaction unit and the latest wins command advance.'),
    ch('👶', 'child-operation-unit', 'child_operation_unit', [(473, 607)],
       'Child operation unit of the typed operation publication: the publication of one mounted child operation.'),
    ch('📬', 'typed-operation-unit', 'typed_operation_unit', [(609, 1008)],
       'Typed operation unit of the typed operation publication: the publication of one mounted typed operation with its store, history, receipt and result page steps.'),
]
SPEC = dict(orig='pub.rs', file=PUB + '/🦀️.rs', children=CH, pre=split_impl_ln(H, B), expect_extra=split_extra_ln('pub.rs', H, B),
            external=mounted(PUB + '/🦀️.rs', orig='pub.rs'))
