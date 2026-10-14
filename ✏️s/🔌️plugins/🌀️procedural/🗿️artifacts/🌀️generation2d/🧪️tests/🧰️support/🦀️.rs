//! 🧰️ Generation2d test support: the publication door, the retirement driver and the document store fixture. A dev-dependency only.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "🔬️store-fixture/🦀️.rs"]
pub mod store_fixture;
#[path = "🔬️retirement-driver/🦀️.rs"]
pub mod retirement_driver;
#[path = "🔬️publication-authority/🦀️.rs"]
pub mod publication_authority;
