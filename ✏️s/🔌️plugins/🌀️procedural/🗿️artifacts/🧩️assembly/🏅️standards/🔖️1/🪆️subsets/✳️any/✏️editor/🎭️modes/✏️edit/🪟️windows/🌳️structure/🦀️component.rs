//! 🌳️ Assembly editor — `structure` window: a real, EDITABLE overview tree of the whole WFC problem
//! spec (`AssemblySnapshot`'s seed/slots/edges/modules/weights/rules — never the solved assignment,
//! which is an inference, not persisted state), built from the framework `TreeWindowKit` (contract
//! §2.6). Mirrors `energy.model`'s `🌳️structure` window shape (also authored fresh, no app to migrate)
//! rather than a spatial/mesh render: `AssemblySlot` carries `x`/`y`/`z`, but no module ASSIGNMENT is
//! ever stored on the snapshot (`../../../../../../🧬️schema/💡️inferences/🦀️component.rs`'s own doc
//! comment: "the SOLVE itself is never stored here"), so a mesh view would have nothing solved to
//! place — a rule/slot tree is the honest first-pass representation of the PROBLEM this artifact
//! actually persists. A spatial view over `slots`' raw coordinates is a plausible follow-up, not a
//! purity or completeness requirement for this packet.

use crate::artifacts::assembly::AssemblySnapshot;
use crate::artifacts::assembly::schema::snapshot::{AssemblyRule, AssemblySlot, AssemblySlotEdge};
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TreeWindowKit::KIND_ID;
pub const BODY_KEY: &str = TreeWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::assembly::create_assembly_editor`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Structure", "Struktur"), icon_id: "list-tree".into(), ..TreeWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ Real `AssemblySnapshot -> UiNode`: one branch per collection (slots/edges/modules/weights/
/// rules), each leaf labeled with its real field values — a genuine overview, never a placeholder.
pub fn render(document: &AssemblySnapshot) -> UiNode {
    fn leaf(id: String, label: String) -> TreeNodeView {
        TreeNodeView { id, label, children: Vec::new() }
    }
    fn slot_leaf(slot: &AssemblySlot) -> TreeNodeView {
        let pinned = slot.pinned_module_id.as_deref().map(|module_id| format!(" pinned={module_id}")).unwrap_or_default();
        leaf(format!("slot-{}", slot.id), format!("{} ({:.2}, {:.2}, {:.2}){pinned}", slot.id, slot.x, slot.y, slot.z))
    }
    fn edge_leaf(edge: &AssemblySlotEdge) -> TreeNodeView {
        leaf(format!("edge-{}", edge.id), format!("{}: {} -> {}", edge.id, edge.from_slot_id, edge.to_slot_id))
    }
    fn rule_leaf(rule: &AssemblyRule) -> TreeNodeView {
        leaf(format!("rule-{}", rule.id), format!("{}: {} -> {} allowed={}", rule.id, rule.module_a_id, rule.module_b_id, rule.allowed))
    }

    let slots = TreeNodeView { id: "slots".into(), label: format!("Slots ({})", document.slots.len()), children: document.slots.iter().map(slot_leaf).collect() };
    let edges = TreeNodeView { id: "edges".into(), label: format!("Edges ({})", document.edges.len()), children: document.edges.iter().map(edge_leaf).collect() };
    let modules = TreeNodeView {
        id: "modules".into(),
        label: format!("Modules ({})", document.modules.len()),
        children: document.modules.iter().map(|module| leaf(format!("module-{}", module.child_id), module.child_id.clone())).collect(),
    };
    let weights = TreeNodeView {
        id: "weights".into(),
        label: format!("Weights ({})", document.weights.len()),
        children: document.weights.iter().map(|weight| leaf(format!("weight-{}", weight.module_id), format!("{}: {:.3}", weight.module_id, weight.weight))).collect(),
    };
    let rules = TreeNodeView { id: "rules".into(), label: format!("Rules ({})", document.rules.len()), children: document.rules.iter().map(rule_leaf).collect() };
    let root = TreeNodeView { id: "assembly".into(), label: format!("Assembly (seed {})", document.seed), children: vec![slots, edges, modules, weights, rules] };
    TreeWindowKit::render(&TreeView { roots: vec![root] })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_tree_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_lists_every_collection_branch() {
        let mut document = AssemblySnapshot::default();
        document.slots.push(AssemblySlot { id: "s1".into(), x: 1.0, y: 2.0, z: 0.0, pinned_module_id: None });
        document.rules.push(AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, ..Default::default() });
        let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
        let root = &node.sections[0].items[0];
        let root_children = root.items.as_ref().expect("root has children");
        assert!(root_children.iter().any(|item| item.id == "slots"));
        assert!(root_children.iter().any(|item| item.id == "rules"));
    }
}
//#endregion 🧪️Tests
