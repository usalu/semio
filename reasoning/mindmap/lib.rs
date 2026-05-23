//! 🧠 Mindmap graph extension: topics and relationships on a property graph.

pub use infinite_cavas as cavas;
pub use mathematical_graph_normal_directed as graph;

// #region 🔖MindmapExtension
/// 🧠 Mindmap semantics over a property graph canvas.
pub trait MindmapExtension: graph::GraphExtension {
    fn topic_label(&self, node_id: graph::NodeId) -> Option<&str>;
}

/// 🧩 Topic is a graph node; relationship is a graph edge.
pub type TopicId = graph::NodeId;
pub type RelationshipId = graph::EdgeId;

/// 🧭 Default mindmap extension stub.
#[derive(Clone, Debug, Default)]
pub struct DefaultMindmapExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
}

impl cavas::CanvasExtension for DefaultMindmapExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/default"
    }
}

impl graph::GraphExtension for DefaultMindmapExtension {}

impl MindmapExtension for DefaultMindmapExtension {
    fn topic_label(&self, node_id: TopicId) -> Option<&str> {
        self.topics.get(&node_id).map(String::as_str)
    }
}
// #endregion 🔖MindmapExtension

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_is_node_id() {
        let id: TopicId = 42;
        let mut ext = DefaultMindmapExtension::default();
        ext.topics.insert(id, "Semantics".into());
        assert_eq!(ext.topic_label(id), Some("Semantics"));
    }
}
// #endregion 🔖Tests
