//! 🪢️ Generation2d synapse mutation leaves: connect, replace and disconnect synapses. Every leaf is generic over the dispatch enum.
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
pub mod replace_synapse {
    #[path = "../🔄️replace/🦀️.rs"]
    mod component;
    #[path = "../🔄️replace/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔄️replace/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod disconnect_synapse {
    #[path = "../✂️disconnect/🦀️.rs"]
    mod component;
    #[path = "../✂️disconnect/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../✂️disconnect/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
