//! 📦️ Package glue — wiring only. Domain lives at owner 🦀️.rs.

// 🏢️ Local crate-alias convention every plugin/extension crate in this repo repeats at its own
// root (see cad's own `🦀️.rs`) — `protocol`/`store` name `semio-os-kernel`'s
// `CompositeMutationKind`/`Planner`/`ArtifactPack` etc. so `🦀️.rs` can write
// `protocol::…`/`store::…` exactly like cad's own mutation payloads do.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;
