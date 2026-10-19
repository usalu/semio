import sys, re
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
G = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/'
roots = ('app', 'instance_a', 'instance_b', 'sender', 'receiver', 'reopened')
# ---- terrain
T = G + '🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
p = T + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
s, ty = alias_to_tuple(s, 'Gis3dApp', vis='pub ')
old = '        let mut app = new_app_with_registry::<EditorApp<Gis3dPlayApp>>('
assert s.count(old) == 1
s = s.replace(old, f'        let scope = {SCOPE_T}::admit();\n' + old)
old = '        app.bind_instance_id(meta("local").instance_id).await;\n        app\n    }'
assert s.count(old) == 1
s = s.replace(old, '        app.bind_instance_id(meta("local").instance_id).await;\n        Gis3dApp(app, scope)\n    }')
write(p, s)
for rel in ['🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/🧪️tests/🔬️unit/🦀️.rs']:
    s, counts, rts = user_port(T + rel, 'Gis3dApp', 'Gis3dApp', rename_raw=False, extra_roots=roots)
    print('terrain', rel[-30:], counts)

# ---- map
M = G + '🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/'
p = M + '✏️editor/🧪️tests/🔬️unit/🦀️.rs'
s, counts, rts = wrapper_port(p, 'Gis2dTestApp', 'Gis2dApp', extra_roots=roots)
s = read(p); s, n = std_drop(s, 'Gis2dTestApp'); s, m = ctor_fix(s); write(p, s)
print('map unit', counts, rts, n, m)
for rel in ['✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs', '✏️editor/🎮️commands/🗺️features/🧪️tests/🔬️unit/🦀️.rs', '👁️viewer/🧪️tests/🔬️unit/🦀️.rs']:
    s, counts, rts = user_port(M + rel, 'Gis2dTestApp', 'Gis2dApp', extra_roots=roots)
    print('map', rel[-34:], counts, rts)
