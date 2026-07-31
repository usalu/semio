"""Patch read-after-writes query in expand_comprehensive_fixture.py."""
from pathlib import Path

p = Path(__file__).parent / "expand_comprehensive_fixture.py"
t = p.read_text(encoding="utf-8")
old = (
    '                "query($designId: ID!) { store { wip { theKit { kit { name tags { edges { node { name } } } }"\n'
    '                " concepts { edges { node { name } } } qualities { edges { node { key } } }"\n'
    '                " design(id: $designId) { pieces { edges { node { id } } } } } }"\n'
    '                " alternatives { edges { node { id name } } } } } }"'
)
new = (
    '                "query($designId: ID!, $tagId: ID!) { store { wip { theKit { kit { name tag(id: $tagId) { name }"\n'
    '                " concepts { edges { node { name } } } qualities { edges { node { key } } }"\n'
    '                " design(id: $designId) { pieces { edges { node { id } } } } } } }"\n'
    '                " alternatives { edges { node { id name } } } } } }"'
)
if old not in t:
    raise SystemExit("old block not found")
p.write_text(t.replace(old, new), encoding="utf-8")
print("patched expand script")
