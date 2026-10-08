//! 📐️ Flow edit rules: every editing gesture that changes the graph (remove, delete selection, disconnect, connect, rename, auto-layout, insert port) is resolved to the concrete intent
//! leaves of the composed Flow content child (`remove-edge`, `remove-node`, `insert-node`, `insert-edge`, `set-edge-endpoints`, `set-node-position`, `set-node-param`) while it is
//! applied to a working copy of that content the next row of the same gesture reads. The `FlowHost` stays the oracle of port compatibility, id minting and layout; the leaves are
//! built here from the payload and the content, never by differencing two contents.

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, remove_edge, remove_node, set_edge_endpoints, set_node_param, set_node_position, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};

/// ✏️ One gesture in progress: the content child as the gesture has left it and the concrete leaves it has emitted so far.
#[derive(Clone, Debug)]
pub struct ContentEdit {
    pub content: SemioFlowSnapshot,
    pub leaves: Vec<SemioFlowMutation>,
}

impl ContentEdit {
    /// 🏗️ Starts a gesture over `content`.
    pub fn new(content: SemioFlowSnapshot) -> Self {
        Self { content, leaves: Vec::new() }
    }

    /// ➖️ Removes the edge `id`: one `remove-edge`. `false` when absent.
    pub fn remove_edge(&mut self, id: &str) -> bool {
        let Some(index) = self.content.edges.iter().position(|edge| edge.id == id) else { return false };
        self.content.edges.remove(index);
        self.leaves.push(SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: id.into() }));
        true
    }

    /// ➖️ Removes the edges `edge_ids` and every edge touching `node_ids` (child order, so no edge ever dangles), then the nodes `node_ids`. `false` when nothing existed to remove.
    pub fn remove(&mut self, node_ids: &[String], edge_ids: &[String]) -> bool {
        let severed: Vec<String> = self.content.edges.iter().filter(|edge| edge_ids.contains(&edge.id) || node_ids.contains(&edge.from.node) || node_ids.contains(&edge.to.node)).map(|edge| edge.id.clone()).collect();
        let doomed: Vec<String> = self.content.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| node.id.clone()).collect();
        for id in &severed {
            self.remove_edge(id);
        }
        for id in &doomed {
            self.content.nodes.retain(|node| &node.id != id);
            self.leaves.push(SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: id.clone() }));
        }
        !severed.is_empty() || !doomed.is_empty()
    }

    /// 🧭️ Moves the node `id` to `(x, y)`: one `set-node-position`. `false` when absent or already there.
    pub fn set_position(&mut self, id: &str, x: f64, y: f64) -> bool {
        let Some(node) = self.content.nodes.iter_mut().find(|node| node.id == id) else { return false };
        let position = SemioPoint2 { x, y };
        if node.position == position {
            return false;
        }
        node.position = position.clone();
        self.leaves.push(SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: id.into(), position }));
        true
    }

    /// 🎛️ Sets the param `key` of the node `id`: one `set-node-param`. `false` when the node is absent or already holds the value.
    pub fn set_param(&mut self, id: &str, key: &str, value: &str) -> bool {
        let Some(node) = self.content.nodes.iter_mut().find(|node| node.id == id) else { return false };
        match node.params.iter_mut().find(|param| param.key == key) {
            Some(param) if param.value == value => return false,
            Some(param) => param.value = value.into(),
            None => node.params.push(semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowParam { key: key.into(), value: value.into() }),
        }
        self.leaves.push(SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: id.into(), key: key.into(), value: value.into(), at: None }));
        true
    }

    /// 🔗️ Connects `edge`: `remove-edge` for every edge already entering its target port, then one `insert-edge`.
    pub fn connect(&mut self, edge: FlowEdge) {
        let entering: Vec<String> = self.content.edges.iter().filter(|known| known.to == edge.to).map(|known| known.id.clone()).collect();
        for id in &entering {
            self.remove_edge(id);
        }
        self.leaves.push(SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(edge.clone())));
        self.content.edges.push(edge);
    }

    /// 🏷️ Renames the node `old` to `new`: `insert-node` of the renamed copy at the same position in the child, `set-edge-endpoints` for every edge touching `old`, then `remove-node` of `old`.
    /// `false` when `old` is absent, `new` is empty or equals `old`, or `new` is taken.
    pub fn rename_node(&mut self, old: &str, new: &str) -> bool {
        let Some(index) = self.content.nodes.iter().position(|node| node.id == old) else { return false };
        if new.is_empty() || new == old || self.content.nodes.iter().any(|node| node.id == new) {
            return false;
        }
        let renamed = FlowNode { id: new.into(), ..self.content.nodes[index].clone() };
        self.leaves.push(SemioFlowMutation::InsertNode(insert_node::InsertNode { node: renamed.clone(), at: Some(index) }));
        self.content.nodes.insert(index, renamed);
        let repoint = |port: &PortRef| if port.node == old { PortRef { node: new.into(), port: port.port.clone() } } else { port.clone() };
        for edge in self.content.edges.iter_mut().filter(|edge| edge.from.node == old || edge.to.node == old) {
            edge.from = repoint(&edge.from);
            edge.to = repoint(&edge.to);
            self.leaves.push(SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }));
        }
        self.content.nodes.retain(|node| node.id != old);
        self.leaves.push(SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: old.into() }));
        true
    }

    /// 🔌️ Inserts a port at `index` on the input (`input` true) or output side of the node `id`, whose side now lists `ports`: `set-node-param` of the side's port list and one
    /// `set-edge-endpoints` for every edge on that side whose numeric port is at or past the insertion point.
    pub fn insert_port(&mut self, id: &str, input: bool, index: usize, ports: &[String]) {
        let insert_at = index.min(ports.len().saturating_sub(1));
        let key = if input { "inputPorts" } else { "outputPorts" };
        self.set_param(id, key, &serde_json::to_string(ports).unwrap_or_default());
        for edge in self.content.edges.iter_mut() {
            let side = if input { &mut edge.to } else { &mut edge.from };
            if side.node != id {
                continue;
            }
            if let Ok(old_index) = side.port.parse::<usize>() {
                if old_index >= insert_at {
                    side.port = (old_index + 1).to_string();
                    self.leaves.push(SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }));
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
