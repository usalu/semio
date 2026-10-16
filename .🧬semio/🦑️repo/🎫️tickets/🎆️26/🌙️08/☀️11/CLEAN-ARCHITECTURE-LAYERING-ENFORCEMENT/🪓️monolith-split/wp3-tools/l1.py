import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lift import *
E = os.environ['E']; T = os.environ['T']; R = os.environ['R']
I = E + f'/🧱{V}elements/🗣{V}Interpreter/🎯{V}targets/🧊{V}wgpu'
print(lift(I + f'/🖼{V}ui-image-loading/🦀{V}.rs', ['fn framework_widget_context'], f'🤝{V}contracts', f'🖼{V}ui-image-loading', 'contracts_lift',
           f'//! 🖼{V} Framework widget context: the one place a retained UI image render step builds the widget context its scenes draw with, lifted out of the Interpreter for the `contracts` crate of the renderer partition (plan section 5.2).', region=f'UiImageLoading'))
print(lift(I + f'/🎯{V}chrome-ledger/🦀{V}.rs', ['static WORLD_CAMERA_LEDGER', 'fn note_world3d_live_camera', 'fn take_world_camera_diagnostic'], f'🎭{V}scene-state', f'🎯{V}chrome-ledger', 'scene_state_lift',
           f'//! 🎯{V} World camera ledger: the live world camera rows and their diagnostic, lifted out of the chrome ledger for the `scene-state` crate of the renderer partition (plan section 5.2).', region=f'🎯{V}ChromeLedger'))
print(lift(T + f'/🏠{V}os-host/🦀{V}.rs', ['struct ComponentSurfaceCloseToken', 'struct ComponentSurfaceCloseRequest', 'enum ComponentSurfaceCloseBridgeState', 'struct ComponentSurfaceCloseBridge', 'impl Default for ComponentSurfaceCloseBridge',
           'thread_local COMPONENT_SURFACE_CLOSE_BRIDGE', 'static COMPONENT_SURFACE_CLOSE_BRIDGE', 'fn with_component_surface_close_bridge', 'fn request_component_surface_close', 'fn component_surface_close_terminal',
           'fn acknowledge_component_surface_close', 'fn take_component_surface_close_request', 'fn publish_component_surface_close_terminal', 'fn component_surface_close_pending', 'fn component_surface_close_occupied'],
           f'🎭{V}scene-state', f'🏠{V}os-host', 'scene_state_lift', f'//! 🏠{V} Component surface close bridge: the request, terminal and acknowledgement cells between a component scene and the host that closes its surface, lifted out of the OS host for the `scene-state` crate of the renderer partition (plan section 5.2).'))
print(lift(T + f'/🌐{V}browser-worker/🦀{V}.rs', ['fn declare_boot_subphase'], f'🏛{V}host-base', f'🌐{V}browser-worker', 'host_base_lift',
           f'//! 🌐{V} Boot subphase declaration: the one console line a boot subphase declares, lifted out of the browser worker for the `host-base` crate of the renderer partition (plan section 5.2).'))
print(lift(R + f'/🎨{V}presentation-raster/🦀{V}.rs', ['type RuntimeHostWaker'], f'🏛{V}host-base', f'🎨{V}presentation-raster', 'host_base_lift',
           f'//! 🎨{V} Runtime host waker: the platform wake callback type of the runtime mailbox, lifted for the `host-base` crate of the renderer partition (plan section 5.2).'))
