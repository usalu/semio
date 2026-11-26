extern crate semio_os_kernel as store;
use semio_framework::kernel::Event;

#[derive(Debug)]
struct Fault { code: String, message: String }
struct PluginArtifactChannel;
impl PluginArtifactChannel {
    fn not_wired(what: &str, detail: &str) -> Fault { Fault { code: what.into(), message: detail.into() } }
}
#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/⚠️actor-failure.rs"]
mod actor_failure;
