"""🩹 Repairs the half-applied `fixture` → `host_snapshot` codemod in the flow vcs law file.

A name-keyed sweep renamed USES of locals to `host_snapshot` but never their `let` bindings, so
`semio-framework-os-flow --lib` could not build its test binary (21 × E0425). Two classes:
a local holding a `FlowHostSnapshot` takes the new name at its binding; a local holding a parsed
JSON fixture FILE keeps `fixture`, and the stray use is put back.
"""

import pathlib
import sys

TARGET = pathlib.Path("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs")

BIND = [
    ("    let fixture = FlowHostSnapshot::default();", "    let host_snapshot = FlowHostSnapshot::default();"),
    ("    let mut fixture = FlowHostSnapshot::default();", "    let mut host_snapshot = FlowHostSnapshot::default();"),
    (
        "    let fixture = FlowHostSnapshot { widgets: Vec::new(), synapses: Vec::new(), ..FlowHostSnapshot::default() };",
        "    let host_snapshot = FlowHostSnapshot { widgets: Vec::new(), synapses: Vec::new(), ..FlowHostSnapshot::default() };",
    ),
    (
        '    let fixture = <FlowHostSnapshot as crate::os_store::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");',
        '    let host_snapshot = <FlowHostSnapshot as crate::os_store::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");',
    ),
]

RESTORE = [
    ("    match (host_snapshot, path) {", "    match (fixture, path) {"),
    ('&host_snapshot["initial"].clone()', '&fixture["initial"].clone()'),
]

RETAINED_OLD = """fn retained_fixture() -> FlowHostSnapshot {
    let mut host_snapshot = FlowHostSnapshot::default();"""
RETAINED_TAIL_OLD = """    host_snapshot.layout.insert("preview".into(), WidgetLayout { x: 4.0, y: 5.0 });
    fixture
}"""
RETAINED_TAIL_NEW = """    host_snapshot.layout.insert("preview".into(), WidgetLayout { x: 4.0, y: 5.0 });
    host_snapshot
}"""


def main() -> int:
    text = TARGET.read_text(encoding="utf-8")
    changed = 0
    for old, new in BIND + RESTORE:
        if old in text:
            changed += text.count(old)
            text = text.replace(old, new)
    if RETAINED_OLD in text and RETAINED_TAIL_OLD in text:
        text = text.replace(RETAINED_TAIL_OLD, RETAINED_TAIL_NEW, 1)
        changed += 1
    TARGET.write_text(text, encoding="utf-8")
    print(f"repaired {changed} site(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
