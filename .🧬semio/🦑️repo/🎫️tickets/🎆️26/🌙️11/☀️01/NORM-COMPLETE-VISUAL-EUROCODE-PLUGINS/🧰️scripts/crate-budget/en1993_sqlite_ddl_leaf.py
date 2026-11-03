"""🗄️ Moves the SQLite DDL text of en1993-model into the sibling leaf crate `en1993-model-sqlite-ddl` (`LEAF` const; the `.sql` file stays in place, so the entity generator and the contract script are unchanged), like the four facet twin crates. Idempotent. Usage: python -I en1993_sqlite_ddl_leaf.py"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

root = NORM + "/🗿️artifacts/🔩️en1993"
snap = root + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
toml = root + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml"
parent, cname = "semio-s-artifact-norm-en1993-model", "semio-s-artifact-norm-en1993-model-sqlite-ddl"
pkg = snap + "/🗄️sql-ddl-leaf/📦️packages/🦀️rust"
wr(snap + "/🗄️sql-ddl-leaf/🦀️.rs", '//! 🗄️ En1993 model SQLite DDL leaf, kept next to the schema it embeds.\n\n/// 🗄️ The handcrafted SQLite DDL of the `snapshot` schema facet.\npub const LEAF: &str = include_str!("../🗄️.sql");\n')
wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1993 model SQLite DDL leaf"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n' % (("../" * 10).rstrip("/"), cname))
register_workspace(root, cname, pkg)
add_dep(toml, cname)
script_register(root, parent, cname)
lock_register(parent, cname, new_deps=[])
p = snap + "/🦀️.rs"
s = rd(p)
s = s.replace('include_str!("🗄️.sql")', "semio_s_artifact_norm_en1993_model_sqlite_ddl::LEAF", 1)
wr(p, s)
print("ok")
