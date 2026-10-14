//! 🪢️ Generation3d synapse mutation leaves: connect, update and disconnect synapses. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod connect_synapse {
    #[path = "../🔗️connect-synapse/🦀️.rs"]
    mod component;
    #[path = "../🔗️connect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔗️connect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod update_synapse {
    #[path = "../🔄️update-synapse/🦀️.rs"]
    mod component;
    #[path = "../🔄️update-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔄️update-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod disconnect_synapse {
    #[path = "../✂️disconnect-synapse/🦀️.rs"]
    mod component;
    #[path = "../✂️disconnect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../✂️disconnect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
