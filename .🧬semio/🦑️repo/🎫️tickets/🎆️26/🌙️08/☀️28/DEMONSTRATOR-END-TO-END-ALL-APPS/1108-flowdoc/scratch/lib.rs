#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs"]
pub mod registry;
pub use registry::*;
#[path = "."]
pub mod extensions {
    #[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[cfg(test)]
mod laws_host{
    use crate::registry::catalogue_work::*;
    use semio_framework_value::{RetirementDemand,retained_clone::RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
    use neural_engine::{SharedRegistry,OperatorInfo};
    type Infos=protocol::causal::transition::HistoryFoldIndex<String,OperatorInfo>;
    #[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/📇️catalogue-work/🧪️laws/🦀️.rs"]
    mod laws;
}
