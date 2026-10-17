//! 📌️ BIM editor panels: every panel of the editor, each mounted from its own folder.

#[path = "."]
pub mod options {
    #[path = "🧭️options/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod outliner {
    #[path = "🌳️outliner/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod properties {
    #[path = "🔍️properties/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod library {
    #[path = "🛍️library/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod families {
    #[path = "🪑️families/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod classification {
    #[path = "🗂️classification/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod diagnostics {
    #[path = "🚨️diagnostics/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod coordination {
    #[path = "🤝️coordination/🦀️.rs"]
    mod component;
    pub use component::*;
}
