from specs.lib import *
FR = PL + '/🪟️window/🎚️config/📥️retained'
CH = [
    ch('🧩', 'pack-load-contract-and-value-frames', 'pack_load_contract_and_value_frames', [(10, 232)],
       'Pack load contract and value frames: the mounted token, the controlled demands, the pack load diagnostic, phase, progress and step, the erased pack load, the expected and built values, the value frame, the retained string and the retained typed state.'),
    ch('🗃', 'retained-typed-state', 'retained_typed_state', [(234, 666)],
       'Retained typed state: the stepping of the retained window config typed state with its retirement and drop.'),
    ch('📖', 'retained-state-decode', 'retained_state_decode', [(668, 1087)],
       'Retained state decode: the retained state phase, the retained window config state decode, its stepping, retirement and drop.'),
    ch('📦', 'typed-pack-load', 'typed_pack_load', [(1089, 1596)],
       'Typed pack load: the load original, the typed window config pack load with its stepping, its erased pack load implementation and drop, and the begin and commit entry points.'),
]
SPEC = dict(orig='frr.rs', file=FR + '/🦀️.rs', children=CH, wiring='block', nest=True, rebase_super=True, external=mounted(FR + '/🦀️.rs', orig='frr.rs'), external_prod=family_prod(FR + '/🦀️.rs'),
            patch_parent=[('use typed_pack_load::TypedWindowConfigPackLoad;', '#[cfg(test)]\nuse typed_pack_load::TypedWindowConfigPackLoad;')])
