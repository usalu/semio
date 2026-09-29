#!/usr/bin/env python3
"""🧪️ U6 set A5 — wav edit-window laws after LB2 p3 (row actions are `TableRow.row_actions` props, not child buttons), test-only
(rule 22). A verb's surface control is a node binding OR a row action of a `TableRow` (the host paints each row action as a native,
keyboard-operable button — the p3/WG11 TableRow contract), so the helpers read both and the control law admits a `TableRow`.

Usage: u6-test-drift-5.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-5")
TEST = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"
SETS = {
    TEST: [
        (
            """fn binding_named<'a>(node: &'a BuiltNode, action: &str) -> Option<&'a ActionBinding> {
    node.bindings.iter().find(|binding| binding.action.name.as_str() == action).or_else(|| node.children.iter().find_map(|child| binding_named(child, action)))
}

fn node_with_binding<'a>(node: &'a BuiltNode, action: &str) -> Option<&'a BuiltNode> {
    node.bindings.iter().any(|binding| binding.action.name.as_str() == action).then_some(node).or_else(|| node.children.iter().find_map(|child| node_with_binding(child, action)))
}
""",
            """fn own_bindings(node: &BuiltNode) -> impl Iterator<Item = &ActionBinding> {
    let row_actions = match &node.component {
        Component::TableRow(props) => Some(props.row_actions.iter().map(|row_action| &row_action.action)),
        _ => None,
    };
    node.bindings.iter().chain(row_actions.into_iter().flatten())
}

fn binding_named<'a>(node: &'a BuiltNode, action: &str) -> Option<&'a ActionBinding> {
    own_bindings(node).find(|binding| binding.action.name.as_str() == action).or_else(|| node.children.iter().find_map(|child| binding_named(child, action)))
}

fn node_with_binding<'a>(node: &'a BuiltNode, action: &str) -> Option<&'a BuiltNode> {
    own_bindings(node).any(|binding| binding.action.name.as_str() == action).then_some(node).or_else(|| node.children.iter().find_map(|child| node_with_binding(child, action)))
}
""",
            1,
        ),
        (
            '        assert!(matches!(&control.component, Component::Button(_) | Component::Input(_)), "{action_id} must remain a natively keyboard-operable control");\n',
            '        assert!(matches!(&control.component, Component::Button(_) | Component::Input(_) | Component::TableRow(_)), "{action_id} must remain a natively keyboard-operable control (a row action is painted as a native button)");\n',
            1,
        ),
    ]
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel[-60:]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
