//! 🧪️ C12: standalone proof crate for the window-3 ui-scene `text_splice` twin (no repo dependencies, private target).
#[path = "../../text_splice.rs"]
pub mod text_splice;
#[cfg(test)]
#[path = "../../text_splice_law.rs"]
mod law;
