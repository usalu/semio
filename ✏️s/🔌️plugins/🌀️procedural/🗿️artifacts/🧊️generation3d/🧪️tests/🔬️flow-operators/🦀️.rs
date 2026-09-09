//! 🧠️ The packaged `brep`/`math` operator sets, installed ONCE per lib-test binary.
//!
//! The artifact crate itself never links a geometry kernel (see the `[dev-dependencies]` note in
//! `📦️packages/🦀️rust/Cargo.toml`) — the `[[test]] example-geometry` harness installs them for its
//! own process. Every `--lib` test that evaluates a bundled fixture needs exactly the same
//! installation, and without it `FlowHost::evaluate` answers `unknown kind: brep.curve.polygon` /
//! `unknown kind: math.vector` for every node, so the preview payload comes back empty and every
//! geometry assertion fails on a registry gap rather than on geometry
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! @see ../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs — the same installation for the geometry lane.

use semio_framework_os_flow::neural::Registry;
use semio_framework_os_flow::{install_flow_extension, FlowExtensionSpec};

/// ⏳️ Drives a registration future that is required not to suspend — the packaged extensions'
/// `register` entry points are `async` by repository convention but do no external work.
fn resolve_ready<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("extension registration must not depend on external work"),
    }
}

pub(crate) fn install(registry: &mut Registry) {
    resolve_ready(semio_s_plugin_flow_extension_brep::register(registry));
    semio_s_plugin_flow_extension_math::register(registry);
}

/// 🌿️ Publishes the operator registry once per test binary.
pub fn installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        install_flow_extension(FlowExtensionSpec { id: "generation3d-lib-tests".into(), name: "Generation3d Lib Tests".into(), version: "1".into(), install }).expect("packaged brep/math operator admission");
    });
}
