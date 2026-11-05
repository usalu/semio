//! 🧰️ Generation3d test support: the serial lock, the retirement driver and the document store fixture. A dev-dependency only.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate self as semio_s_artifact_procedural_3d_testkit;

#[path = "🔬️serial/🦀️.rs"]
pub mod test_serial;
#[path = "🔬️retirement-driver/🦀️.rs"]
pub mod retirement_driver;
#[path = "🔬️store-fixture/🦀️.rs"]
pub mod store_fixture;
