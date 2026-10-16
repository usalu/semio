import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *
sub1(T + "/🌐️browser-worker/🦀️.rs", '''#[path = "../🏛️host-base/📥️lifted/🌐️browser-worker/🦀️.rs"]
mod host_base_lift;
pub use host_base_lift::*;''', '''pub use semio_framework_os_renderer_wgpu_host_base::declare_boot_subphase;''')
sub1(R + "/🎨️presentation-raster/🦀️.rs", '''#[path = "../../🏛️host-base/📥️lifted/🎨️presentation-raster/🦀️.rs"]
mod host_base_lift;
pub use host_base_lift::*;''', '''pub use semio_framework_os_renderer_wgpu_host_base::RuntimeHostWaker;''')
sub1(EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🎯️chrome-ledger/🦀️.rs", '''#[path = "../../../../../🎯️targets/🧊️wgpu/🎭️scene-state/📥️lifted/🎯️chrome-ledger/🦀️.rs"]
mod scene_state_lift;
pub use scene_state_lift::*;''', '''pub use semio_framework_os_renderer_wgpu_scene_state::{note_world3d_live_camera, take_world_camera_diagnostic, WORLD_CAMERA_LEDGER};''')
sub1(T + "/🪟️winit-app/🦀️.rs", "pub(crate) use crate::dispatch_normalized_event;\n", "")
