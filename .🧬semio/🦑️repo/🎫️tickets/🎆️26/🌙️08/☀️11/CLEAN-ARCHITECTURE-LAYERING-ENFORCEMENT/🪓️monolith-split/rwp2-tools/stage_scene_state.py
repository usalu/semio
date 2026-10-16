import re
from model import *
import imports
def rd(p): return open(p, encoding='utf8').read()
def wr(p, t): open(p, 'w', encoding='utf8').write(t)
ROOT_SC = SC + '/🦀️.rs'
CRATE = T + '/' + FOLDER['scene-state'] + '/🦀️.rs'
PFX = 'semio_framework_os_renderer_wgpu_'
root = rd(ROOT_SC)
a = root.index('//#region SceneRuntime\n'); b = root.index('//#endregion SceneRuntime') + len('//#endregion SceneRuntime')
runtime = root[a:b]
root_wo = root[:a].rstrip('\n') + '\n' + root[b:]
runtime = re.sub(r'\bWorkerCell\b', 'SceneCell', runtime)
runtime = runtime.replace('`crate::element_contracts::test_worker_cell`', '`element_contracts::test_worker_cell`')
runtime = runtime.replace('crate::interpreter::take_world_camera_diagnostic', 'take_world_camera_diagnostic')
runtime = runtime.replace('crate::element_contracts::', 'crate::element_contracts::')
# children files of the crate
ST = SC + '/🗃️scene-state/🦀️.rs'; PR = SC + '/🪣️pending-raster/🦀️.rs'; CD = SC + '/📷️scene-camera-dispatch/🦀️.rs'
st = rd(ST); cd = rd(CD)
cd = re.sub(r'\bWorkerCell\b', 'SceneCell', cd)
cd = cd.replace('crate::interpreter::ui_document_close_pending_for', 'crate::element_contracts::ui_document_close_pending_for').replace('crate::interpreter::component_scene_is_presented', 'crate::element_contracts::component_scene_is_presented')
cd = cd.replace('crate::app_now_ms', 'crate::host_base::app_now_ms')
st = st.replace('crate::app_now_ms', 'crate::host_base::app_now_ms').replace('crate::os_host::', 'crate::')
wr(ST, st); wr(CD, cd)
# imports the crate files need from the old Scenes root
alltext = runtime + st + rd(PR) + cd
need = [u for u in imports.uses(root) if u['alias'] != '_' and re.search(r'(?<![\w.])' + re.escape(u['alias']) + r'\b', alltext) and u['alias'] not in ('engine_canvas',)]
lines = ''.join(imports.render(u) for u in need)
lines = lines.replace('use crate::shell::', 'use crate::element_contracts::')
rel = '../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/'
lib = '''//! 🎭️ Scene state: the per-surface scene registry every scene kind reads and writes, and the shared kit the scene crates build on.
//!
//! The scene cell and its statics (scene state, retirement, canvas gesture and catalogue hover, list transfer, camera dispatch), the admitted surface
//! map, the pending raster registry, the camera dispatch ledger, the component surface close bridge and the world camera ledger lifted out of the
//! host and the chrome ledger, and the list, chrome label, accessibility cell and text editor ui kit the scene leaves share.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
extern crate semio_framework_os_renderer_wgpu_host_base as host_base;
pub(crate) use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_camera as camera_storage;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken;
use crate::element_contracts::WorkerCell;
''' + lines + '''
''' + runtime + '''

#[path = "%s🗃️scene-state/🦀️.rs"]
mod scene_state;
pub use scene_state::*;

#[path = "%s🪣️pending-raster/🦀️.rs"]
mod pending_raster;
pub use pending_raster::*;

#[path = "%s📷️scene-camera-dispatch/🦀️.rs"]
mod scene_camera_dispatch;
pub use scene_camera_dispatch::*;

#[path = "📥️lifted/🏠️os-host/🦀️.rs"]
mod component_surface_close;
pub use component_surface_close::*;

#[path = "📥️lifted/🎯️chrome-ledger/🦀️.rs"]
mod world_camera_ledger;
pub use world_camera_ledger::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
''' % (rel, rel, rel)
wr(CRATE, lib)
# facade: drop the three mounts and the runtime region
for mod, d in [('scene_state', '🗃️scene-state'), ('pending_raster', '🪣️pending-raster'), ('scene_camera_dispatch', '📷️scene-camera-dispatch')]:
    old = '#[path = "%s/🦀️.rs"]\nmod %s;\npub use %s::*;\n\n' % (d, mod, mod)
    assert root_wo.count(old) == 1, old
    root_wo = root_wo.replace(old, '')
wr(ROOT_SC, root_wo)
print('imports in crate root:', [u['alias'] for u in need])
