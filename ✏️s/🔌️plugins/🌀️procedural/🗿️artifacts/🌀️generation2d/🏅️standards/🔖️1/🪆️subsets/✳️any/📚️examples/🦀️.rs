//! 📚️ Generation2d bundled examples: one module per authored example document.
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod demo {
    #[path = "🎬️demo/🦀️.rs"]
    mod component;
    pub use component::*;
    #[cfg(test)]
    #[path = "🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
    mod tests;
}
