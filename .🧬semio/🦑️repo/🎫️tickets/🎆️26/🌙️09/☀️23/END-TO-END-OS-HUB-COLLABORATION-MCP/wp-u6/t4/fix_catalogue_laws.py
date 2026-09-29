"""🎯️ Catalogue laws: a catalogue row's own add verb is its target's ACTIVATION (a' — one target per row), not a binding."""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins"

def edit(rel, pairs):
    p = ROOT / rel
    s = p.read_text()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:90], s.count(old))
        s = s.replace(old, new)
    p.write_text(s)

ACT = '''fn activation_of(row: &BuiltNode) -> Option<String> {
    match &row.component {
        ui::Component::TreeItem(props) => props.target.as_ref().and_then(|target| target.activation.as_ref()).map(|activation| activation.as_str().to_string()),
        _ => None,
    }
}
'''
P2 = "🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"
edit(P2, [
    ('''/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`; every catalogue row keeps
/// its own `addNode` binding, because a catalogue row is not a pick target.
#[test]
fn a_catalogue_window_materialises_its_slice_with_row_bindings_intact() {''', '''/// 🎯️ The verb a row activates — its ONE target's activation.
''' + ACT + '''
/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`; every catalogue row keeps
/// its own `addNode` activation, because a catalogue row is not a pick target.
#[test]
fn a_catalogue_window_materialises_its_slice_with_row_activations_intact() {'''),
    ('''        assert_eq!(row.bindings.len(), 1, "catalogue row {} keeps its own addNode binding", row.key.as_str());''',
     '''        assert_eq!(activation_of(row).as_deref(), Some("addNode"), "catalogue row {} keeps its own addNode activation", row.key.as_str());'''),
])
P5 = "🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"
s5 = (ROOT / P5).read_text()
i = s5.index("fn a_catalogue_window_materialises_its_slice_with_row_bindings_intact() {")
doc_start = s5.rfind("\n\n", 0, i) + 2
(ROOT / P5).write_text(s5[:doc_start] + "/// 🎯️ The verb a row activates — its ONE target's activation.\n" + ACT + "\n" + s5[doc_start:])
edit(P5, [
    ('''`addPartKind` binding, because a catalogue row is not a pick target.''', '''`addPartKind` activation, because a catalogue row is not a pick target.'''),
    ('''fn a_catalogue_window_materialises_its_slice_with_row_bindings_intact() {''', '''fn a_catalogue_window_materialises_its_slice_with_row_activations_intact() {'''),
    ('''        assert_eq!(row.bindings.len(), 1, "catalogue row {} keeps its own addPartKind binding", row.key.as_str());''',
     '''        assert_eq!(activation_of(row).as_deref(), Some("addPartKind"), "catalogue row {} keeps its own addPartKind activation", row.key.as_str());'''),
    ('''        assert_eq!(row.bindings.len(), 1, "an inferred part row keeps its own addPartKind binding: {}", row.key.as_str());''',
     '''        assert_eq!(activation_of(row).as_deref(), Some("addPartKind"), "an inferred part row keeps its own addPartKind activation: {}", row.key.as_str());'''),
])
PP = "🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs"
edit(PP, [
    ('''/// targets, so the tree declares no `interactionDomain` and every materialised row keeps its own
/// `addStep` binding (the domain-pick form of law (d) is pinned on `📌️panels/🗿️artifact`).''', '''/// targets, so the tree declares no `interactionDomain` and every materialised row keeps its own
/// `addStep` activation (the domain-pick form of law (d) is pinned on `📌️panels/🗿️artifact`).'''),
    ('''            assert!(!row["bindings"].as_array().cloned().unwrap_or_default().is_empty(), "an action row keeps its own addStep binding: {row}");''',
     '''            assert_eq!(row["component"]["target"]["activation"].as_str(), Some("addStep"), "an action row keeps its own addStep activation: {row}");'''),
])
print("ok")
