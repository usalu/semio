//! 🌀️ The propagation queue shared by every propagation engine. The engines themselves (AC-3 in
//! this phase, AC-4/watched-support in a later phase) live in sibling `prop_*.rs` modules.

use crate::wfc_engine::ids::NodeId;

// #region 🔖️Queue
/// 🌀️ A FIFO node queue with membership-bit dedup — pushing an already-queued node is a no-op, so
/// a node dirtied twice before being processed is still visited exactly once per drain.
#[derive(Clone, Debug)]
pub(crate) struct PropQueue {
    queue: std::collections::VecDeque<u32>,
    in_queue: Vec<bool>,
}

impl PropQueue {
    pub fn new(node_count: usize) -> Self {
        Self { queue: std::collections::VecDeque::new(), in_queue: vec![false; node_count] }
    }

    pub fn push(&mut self, n: NodeId) {
        let idx = n.index();
        if !self.in_queue[idx] {
            self.in_queue[idx] = true;
            self.queue.push_back(n.get());
        }
    }

    pub fn pop(&mut self) -> Option<NodeId> {
        let raw = self.queue.pop_front()?;
        self.in_queue[raw as usize] = false;
        Some(NodeId(raw))
    }

    #[allow(dead_code)] // queue-introspection API exercised by the step/resume API added in a later phase
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.in_queue.iter_mut().for_each(|b| *b = false);
    }

    pub fn push_all(&mut self, node_count: usize) {
        self.clear();
        for i in 0..node_count {
            self.push(NodeId::from_index(i));
        }
    }
}
// #endregion 🔖️Queue

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
