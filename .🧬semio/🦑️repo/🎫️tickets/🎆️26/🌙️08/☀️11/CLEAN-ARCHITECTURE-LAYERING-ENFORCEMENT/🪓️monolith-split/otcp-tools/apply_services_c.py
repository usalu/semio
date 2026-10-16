import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from edit import create
S = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/"
C = S + "🧪️tests/🍎️companion/"
create(C + "📦️packages/🦀️rust/📋️project.json", '{\n  "name": "@semio-tech/os-services-tests-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n')
fw = "../../../../../../../../🔨️modules/"
create(C + "📦️packages/🦀️rust/Cargo.toml", f'''[package]
workspace = "../../../../../../../../.."
name = "semio-framework-os-services-tests"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Companion tests of os-services: host runtime, timer wheel, compute pool, HTTP pool, storage scheduler, event router, file watcher and native IO"
publish = false

[package.metadata.semio]
role = "framework"
id = "os-services-tests"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-os-services = {{ path = "../../../../📦️packages/🦀️rust", features = ["testkit"] }}
semio-framework-actor = {{ workspace = true }}
semio-framework-async = {{ workspace = true, features = ["entrypoint"] }}
semio-framework-async-macros = {{ path = "{fw}⏳️async/✨️macros/📦️packages/🦀️rust" }}
semio-framework-job = {{ workspace = true, features = ["testkit"] }}
semio-framework-job-worker = {{ path = "{fw}🧵️job/👷️worker/📦️packages/🦀️rust", features = ["testkit"] }}
semio-framework-job-worker-authority = {{ path = "{fw}🧵️job/👷️worker/🎟️authority/📦️packages/🦀️rust" }}
semio-framework-trace = {{ workspace = true }}
semio-framework-value = {{ workspace = true }}
serde_json = {{ workspace = true }}

[dev-dependencies]
''')
mods = ["completion_sink", "compute_pool", "event_router", "fixed_file_storage", "host_runtime", "http_contract", "http_pool", "http_socket", "storage_scheduler", "timer_wheel"]
create(C + "🦀️.rs", '''//! 🍎️ Companion tests of os-services: host runtime, timer wheel, compute pool, HTTP pool, storage scheduler, event router, file watcher and native IO.
//!
//! 🧭️ The tests reach the private service modules through the `testkit` hooks of `semio_framework_os_services`, never through the production crate's own module tree.
#![cfg(test)]

#[global_allocator]
static ORIGINAL_SERVICES_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

mod component {
    pub use semio_framework_os_services::testkit::{''' + ", ".join(mods) + '''};

    #[path = "../../🔬️component-unit/🦀️.rs"]
    mod tests;
}

mod file_watcher {
    pub use semio_framework_os_services::testkit::file_watcher::*;
    use semio_framework_async::WorkerPool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::time::{Duration, Instant, UNIX_EPOCH};

    #[path = "../../🔬️file-watcher-unit/🦀️.rs"]
    mod tests;
}

mod native_io {
    pub use semio_framework_os_services::testkit::native_io::*;
    use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
    use semio_framework_value::RetirementDemand;
    use std::path::PathBuf;

    #[path = "../../🔬️native-io-unit/🦀️.rs"]
    mod tests;
}
''')
