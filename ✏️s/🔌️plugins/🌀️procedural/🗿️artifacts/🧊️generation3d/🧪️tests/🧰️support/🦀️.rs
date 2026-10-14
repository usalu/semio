//! 🧰️ Generation3d test support: the serial lease lock, the publication door, the retirement driver and the document store fixture. A dev-dependency only.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate self as semio_s_artifact_procedural_3d_testkit;

#[path = "🔬️serial/🦀️.rs"]
pub mod test_serial;
#[path = "🔬️publication-authority/🦀️.rs"]
pub mod publication_authority;
#[path = "🔬️retirement-driver/🦀️.rs"]
pub mod retirement_driver;
#[path = "🔬️store-fixture/🦀️.rs"]
pub mod store_fixture;
