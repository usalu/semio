"""⬆️ RI3: `run_native` names `winit_app::{HostUserEvent, WinitApp}` (host-winit, ranked above native-probe), so it is lifted into `host-winit`'s `winit_app` module (dependency-safe lift of a callee with its one caller, the entrypoint, which reaches it through the hub glob of host-winit)."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from common_ri3 import *
from edit import apply

PROBE = f"{T}/🧊️renderer/🔬️native-socket-probe/🦀️.rs"
ROOT = f"{T}/🔬️native-probe/🦀️.rs"
WINIT = f"{T}/🪟️winit-app/🦀️.rs"
HROOT = f"{T}/🎪️host-winit/🦀️.rs"
FN_SIG = 'pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf, services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>) {\n'
OLD_FN = '#[cfg(not(target_arch = "wasm32"))]\n' + FN_SIG + '''    runtime_assets::install_native_runtime_assets();
    semio_framework_os_renderer_wgpu_native_platform::install_native_host_temporal_formatter(std::sync::Arc::new(native_temporal::SystemTemporalFormatter::default()));
    let event_loop = EventLoop::<winit_app::HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let caller=native_caller::NativeFrameEventIssuer::new(semio_framework_job::default_now_us,native_caller::NativeFrameRootPolicyV1::initiating_event()).expect("native initiating event policy");
    let mut app = winit_app::WinitApp::new(proxy, plugin_filter.to_string(), plugin_modules_root,services,caller);
    let _ = event_loop.run_app(&mut app);
}

'''
NEW_FN = '''
/// 🚀️ The native entry: installs the runtime assets and the host temporal formatter, then runs the winit event loop with a [`WinitApp`] until it exits. Lives beside [`WinitApp`], the one item it needs from above the probe crate.
#[cfg(not(target_arch = "wasm32"))]
''' + FN_SIG + '''    semio_framework_os_renderer_wgpu_host_base::runtime_assets::install_native_runtime_assets();
    semio_framework_os_renderer_wgpu_native_platform::install_native_host_temporal_formatter(std::sync::Arc::new(semio_framework_os_renderer_wgpu_native_platform::native_temporal::SystemTemporalFormatter::default()));
    let event_loop = winit::event_loop::EventLoop::<HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let caller = semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameEventIssuer::new(semio_framework_job::default_now_us, semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameRootPolicyV1::initiating_event()).expect("native initiating event policy");
    let mut app = WinitApp::new(proxy, plugin_filter.to_string(), plugin_modules_root, services, caller);
    let _ = event_loop.run_app(&mut app);
}
'''
USE_OLD = '#[cfg(not(target_arch = "wasm32"))]\npub use native::{HostUserEvent, WinitApp};\n'
apply([
    (PROBE, OLD_FN, "", 1),
    (ROOT, '#[cfg(not(target_arch = "wasm32"))]\nuse semio_framework_os_renderer_wgpu_host_plumbing::native_caller;\n', "", 1),
    (ROOT, '#[cfg(not(target_arch = "wasm32"))]\nuse semio_framework_os_renderer_wgpu_native_platform::native_temporal;\n', "", 1),
    (ROOT, '#[cfg(not(target_arch = "wasm32"))]\nuse winit::event_loop::EventLoop;\n', "", 1),
    (WINIT, USE_OLD, USE_OLD + NEW_FN, 1),
    (HROOT, '#[path = "../🪟️winit-app/🦀️.rs"]\npub mod winit_app;\n', '#[path = "../🪟️winit-app/🦀️.rs"]\npub mod winit_app;\n#[cfg(not(target_arch = "wasm32"))]\npub use winit_app::run_native;\n', 1),
])
