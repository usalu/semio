//! 🧠 Mindmap rendering host bindings; GPU session lives in `puzzle_2d` (`BoardSession::new_normal`).

pub use reasoning_mindmap::*;

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mindmap_rs_reexports_reasoning_mindmap() {
        let id: TopicId = 1;
        assert_eq!(id, 1);
    }
}
// #endregion 🔖Tests
