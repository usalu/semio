//! 📚️ BIM examples: the demo, the house and the office documents with the unit-test checks.

#[cfg(test)]
#[path = "🧰️checks/🦀️.rs"]
pub mod checks;
#[path = "."]
pub mod house {
    #[path = "🏡️house/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod office {
    #[path = "🏢️office/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod demo {
    #[path = "🎬️demo/🦀️.rs"]
    mod component;
    pub use component::*;
}
