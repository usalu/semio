from specs.lib import *
GB = '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot'
H = 'impl Generation2dMountedTypedSnapshotOwner {'
B = [387, 593]
CH = [
    ch('🏷', 'mounted-owner-state', 'mounted_owner_state', [(34, 163)],
       'Mounted owner state of the generation2d binary snapshot: the canonical pack prefix, depth and required field constants and the owner records and enums the typed snapshot owner is built from.'),
    ch('🔤', 'owner-string-targets', 'owner_string_targets', [(165, 387)],
       'String targets of the typed snapshot owner: the owner constructor, the container stack, the root field cursor and the string and symbol assignment.'),
    ch('🧮', 'owner-values-and-dictionaries', 'owner_values_and_dictionaries', [(389, 596)],
       'Values and dictionaries of the typed snapshot owner: the JSON and DSL value assignment, the number and boolean scalars, the dictionary containers and the record start.'),
    ch('🗃', 'owner-containers-and-widgets', 'owner_containers_and_widgets', [(598, 801)],
       'Containers and widgets of the typed snapshot owner: the container start, the widget assembly and the container end.'),
    ch('🔌', 'owner-pack-protocol', 'owner_pack_protocol', [(803, 1017)],
       'Pack protocol of the typed snapshot owner: the retained typed pack owner implementation, the drop guard and the mounted pack session entry.'),
]
SPEC = dict(orig='gb.rs', file=GB + '/🦀️.rs', children=CH, pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(GB + '/🦀️.rs', orig='gb.rs'), external_cfg_test=True, oracle_root=GB + '/../🦀️.rs')
SPEC['oracle_root'] = os.path.normpath(GB + '/../🦀️.rs')
