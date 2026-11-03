"""📐️ Registers the en1996-masonry-formulas crate (modules already moved) in workspace, locks, script.ts and the masonry facade manifest, and rewires the facade root."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

art = NORM + "/🗿️artifacts/🪨️en1996"
S = art + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
pkg = S + "/📐️masonry-formulas/📦️packages/🦀️rust"
facade = S + "/⚖️masonry/📦️packages/🦀️rust/Cargo.toml"
old, new = "semio-s-artifact-norm-en1996-masonry", "semio-s-artifact-norm-en1996-masonry-formulas"
root = S + "/⚖️masonry/🦀️.rs"
s = rd(root)
a = '#[path = "📏️factors-and-geometry/🦀️.rs"]\nmod factors_and_geometry;\npub use factors_and_geometry::*;\n\n#[path = "🔥️fire-and-design-effects/🦀️.rs"]\nmod fire_and_design_effects;\npub use fire_and_design_effects::*;\n'
if a in s:
    wr(root, s.replace(a, "pub use semio_s_artifact_norm_en1996_masonry_formulas::*;\n"))
register_workspace(art, new, pkg)
add_dep(facade, new)
script_register(art, old, new)
lock_register(old, new, new_deps=toml_deps(rd(pkg + "/Cargo.toml")))
