from specs.lib import *
F = STORE + '/🦀️.rs'
CH = [
    ch('🧶', 'config-record-macros', 'config_record_macros', [(334, 503)], 
       'Config record macros: the field set mutations generator, the config record declaration and the config diff declaration.',
       glob=False, cfg_mod='#[macro_use]', reexport=False),
    ch('🎲', 'space-members-macro', 'space_members_macro', [(659, 968)],
       'Space members macro: the generator of a per-plugin space member and member factory enum over several document kinds.',
       glob=False, cfg_mod='#[macro_use]', reexport=False),
]
SPEC = dict(orig='str.rs', file=F, children=CH, wiring='item', no_prune=True, expect_extra=['#[macro_use]', '#[macro_use]'], external=mounted(F, orig='str.rs'))
