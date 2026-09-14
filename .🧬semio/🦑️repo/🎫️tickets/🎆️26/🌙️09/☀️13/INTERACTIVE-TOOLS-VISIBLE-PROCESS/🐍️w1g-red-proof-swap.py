"""🔴️ Lane W1-G red proof: `revert` swaps the fill planner's two placed-body resolutions back to kind-only, `restore` swaps them back. Exact-string, so foreign edits elsewhere survive."""
import sys
p = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs"
pairs = [
    ("resolve_placed_object_mesh_url(object, &self.catalogs, &fixture)", 'resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), &self.catalogs, &fixture)'),
    ("resolve_placed_object_mesh_url(object, catalogs, &self.scene.fixture)", 'resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), catalogs, &self.scene.fixture)'),
]
s = open(p).read()
for fixed, old in pairs:
    a, b = (fixed, old) if sys.argv[1] == "revert" else (old, fixed)
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(p, "w").write(s)
print(sys.argv[1], "ok")
