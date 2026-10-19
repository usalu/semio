import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from tmpl_mounted import *
W = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/'
I = '/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
port(W + '🧊️3d' + I + '🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs', 'Wfc3dApp', 'wfc3d')
port(W + '◻️2d' + I + '🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs', 'Wfc2dApp', 'wfc2d')
p = W + '🖼️bitmap' + I + '🧪️tests/🧪️brush-tool/🦀️.rs'
s = port(p, 'BitmapApp', 'bitmap')
old = 'block_on(app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&blur), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()))'
new = 'block_on(semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.app, semio_framework::HOST_EVENT_ACTION_ID, Some(blur), &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.scope.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close))'
assert s.count(old) == 1
write(p, s.replace(old, new))
