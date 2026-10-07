//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! The crate's `[lib] name` is `protocol`: every replica, authority and plugin crate speaks the
//! replication contract through that one canonical name.

// 🔕 async_fn_in_trait warns that callers can't assume Send on the returned future; R3 answers this
// structurally — every former dyn seam becomes a concrete enum so Send falls out at the spawn site.
// Never resolve this by adding `+ Send` to a trait method or by making it sync (R7).
#![allow(async_fn_in_trait)]
#![allow(ambiguous_glob_reexports, unused_imports)]

#[path = ""]
pub mod codec {
    #[path = "../../⚙️codec/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../⚙️codec/🆔️ids/🦀️.rs"]
    pub mod ids;

    pub use self::ids::*;
}

#[path = "../../🚰️source/🦀️.rs"]
pub mod source;

use semio_framework_diagnostic::*;

pub use semio_framework_value as value;
pub use semio_framework_value::dsl_value;

#[path = "../../🆔️ids/🦀️.rs"]
pub mod ids;

#[path = "../../🔢️scalar/🦀️.rs"]
pub mod scalar;

#[path = "../../📖️dictionary/🦀️.rs"]
pub mod dictionary;

#[path = "../../🔐️crypto/🦀️.rs"]
pub mod crypto;

#[path = "../../🎮️mutation/🦀️.rs"]
pub mod mutation;

#[path = "../../🔗️causal/🦀️.rs"]
pub mod causal;

#[path = "../../⚔️conflict/🦀️.rs"]
pub mod conflict;

#[path = ""]
pub mod wire {
    #[path = "../../🧾️wire/🦀️.rs"]
    mod codec;
    pub use codec::*;

    #[path = "../../📡️wire/🦀️.rs"]
    mod frames;
    pub use frames::*;

    #[path = "../../📡️wire/🏠️local-interaction/🦀️.rs"]
    pub mod local_interaction;
    pub use local_interaction::*;

    // 🧬️ The facade has always surfaced ids/crypto/dictionary through `wire::`.
    pub use super::crypto::*;
    pub use super::dictionary::*;
    pub use super::ids::*;
}

#[path = "../../📐️format/🦀️.rs"]
pub mod format;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;


#[path = "../../👕️peer-overlay/🦀️.rs"]
pub mod peer_overlay;
pub use peer_overlay::*;

pub use crate::causal::*;
pub use crate::causal::{FrontierComparison as RuntimeFrontierComparison, FrontierSummary as RuntimeFrontierSummary};
pub use crate::codec::*;
pub use crate::conflict::*;
pub use crate::format::*;
pub use crate::mutation::*;
pub use crate::scalar::*;
pub use crate::source::*;
pub use crate::value::*;
pub use crate::wire::*;

#[path = "../../🧬️retirement/🦀️.rs"]
mod retirement_integration;
