//! 🧠️ The packaged `brep`/`math` extensions as a lib-test binary holds them: CONTRIBUTED under the
//! plugin ids that own them, and LINKED because those two plugins are compiled into this very
//! process.
//!
//! The artifact crate itself never links a geometry kernel in production (see the
//! `[dev-dependencies]` note in `📦️packages/🦀️rust/Cargo.toml`): the two kernels live in their own
//! plugin actors (`flow-extension-brep`, `flow-extension-math`) and reach this app's flow registry as
//! `flow.extension` topic contributions — `sync_host_flow_extension_contributions`, fed by the
//! shell's `buildContributionsJson`.
//!
//! Both halves of that arrangement are installed here, and each carries a different law:
//!
//! - **contributed** (`install_flow_extension_manifest(<plugin id>, …)`) — the registry's
//!   contribution metadata, which is what `flow_extension_invocation_address("brep")` resolves and therefore
//!   the address every `Effect::InvokeExtension` this app emits carries. `preview_tessellate_invocations`
//!   reads exactly that, so the TESSELLATE hop crosses the real wire in every `--lib` test that
//!   drives a tick chain, answered by the in-process host half (`🔬️brep-extension`), which matches on
//!   the plugin id and faults `extension.missing` on any other address.
//! - **linked** (`register_linked_flow_extension_installer(<flow extension id>, …)`) — real
//!   `OperatorImpl`s for the EVALUATE hop. A `--lib` process has both extension crates linked into
//!   it, which is precisely what that registrar means; without it every geometry operator is a
//!   `ContributedExtensionStub` and the synchronous `FlowHost::evaluate` seams the editor and viewer
//!   preview windows call (`with_host(fixture, |host| host.evaluate())`, a FRESH `NeuralCache` per
//!   call) can never produce geometry, because nothing in that scope can answer a
//!   `EvalError::PendingExtension` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! A linked installer SHADOWS the contributed stub for the same operator id
//! (`register_contributed_manifest` skips an operator the registry already carries), so the evaluate
//! hop does not additionally cross the wire in-process. The addressing contract it would otherwise
//! exercise is pinned directly instead, on both sides of the wire:
//! `🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs`'s
//! `contributed_operators_are_addressed_by_their_contributing_plugin_id` (guest half) and
//! `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (host half).
//!
//! @see ../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs — the same linked installation the headless geometry lane uses, which needs no host half.

use semio_framework_os_flow::neural::Registry;
use semio_framework_os_flow::{install_flow_extension_manifest, register_linked_flow_extension_installer};

/// 🪪️ The two contributing plugins' own ids — the ONLY address the host resolves an extension actor
/// by, and therefore the address every `Effect::InvokeExtension` this app emits must carry. Mirrors
/// each extension crate's `ExtensionBundle::new(..)` id.
pub const BREP_EXTENSION_PLUGIN_ID: &str = "flow-extension-brep";
pub const MATH_EXTENSION_PLUGIN_ID: &str = "flow-extension-math";

/// 🌊️ The two extensions' own flow-domain ids — the key the composed registry installs a linked
/// operator pack under, and the id `GENERATION_3D_GEOMETRY_EXTENSION_ID` translates to a plugin id.
pub const BREP_EXTENSION_FLOW_ID: &str = "brep";
pub const MATH_EXTENSION_FLOW_ID: &str = "math";

/// ⏳️ Drives a registration future that is required not to suspend — the packaged extensions'
/// `register`/`extension_manifest_json` entry points are `async` by repository convention but do no
/// external work.
pub fn resolve_ready<T>(future: impl std::future::Future<Output = T>) -> T {
    match std::pin::pin!(future).as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("extension registration must not depend on external work"),
    }
}

/// 📐️ The brep kernel's own operator pack.
fn install_brep(registry: &mut Registry) {
    resolve_ready(semio_s_plugin_flow_extension_brep::register(registry));
}

/// 🧮️ The math kernel's own operator pack.
fn install_math(registry: &mut Registry) {
    semio_s_plugin_flow_extension_math::register(registry);
}

/// 🔌️ The operator sets as the EXTENSION ACTOR itself owns them — the private registry the
/// in-process host half (`🔬️brep-extension`) evaluates a capability against, never this app's own.
pub(crate) fn install(registry: &mut Registry) {
    install_brep(registry);
    install_math(registry);
}

/// 🌿️ Installs both halves once per test binary: the linked operator packs under their flow
/// extension ids, then the contributed manifests under the plugin ids that own them.
pub fn installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        register_linked_flow_extension_installer(BREP_EXTENSION_FLOW_ID, install_brep);
        register_linked_flow_extension_installer(MATH_EXTENSION_FLOW_ID, install_math);
        install_flow_extension_manifest(BREP_EXTENSION_PLUGIN_ID, &resolve_ready(semio_s_plugin_flow_extension_brep::extension_manifest_json())).expect("contributed brep extension admission");
        install_flow_extension_manifest(MATH_EXTENSION_PLUGIN_ID, &semio_s_plugin_flow_extension_math::extension_manifest_json()).expect("contributed math extension admission");
    });
}
