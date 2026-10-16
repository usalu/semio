import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *
P = T + "/🏠️os-host/🦀️.rs"
cut = cut_between(P, "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub(crate) struct ComponentSurfaceCloseToken(u64);", "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nenum ComponentSurfaceClosePhase {", '''pub(crate) use semio_framework_os_renderer_wgpu_scene_state::{
    acknowledge_component_surface_close, component_surface_close_occupied, component_surface_close_pending, component_surface_close_terminal, publish_component_surface_close_terminal, request_component_surface_close, take_component_surface_close_request, ComponentSurfaceCloseRequest,
    ComponentSurfaceCloseToken,
};

''')
assert "COMPONENT_SURFACE_CLOSE_BRIDGE" in cut and cut.count("with_component_surface_close_bridge") >= 8
