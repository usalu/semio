"""🪚 Crate-budget H: the EN 1995 timber check formulas become the crate `-schema-timber` (root = the module file) and the spec-trace table its own crate; the schema re-exports both under the old paths."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🪵️en1995"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
SC = ANY + "/🧬️schema"
P = "semio-s-artifact-norm-en1995-schema"
N = P + "-timber"
TR = "semio-s-artifact-norm-en1995-spec-trace"
root = SC + "/🦀️.rs"
r = rd(root)
if N.replace("-", "_") in r:
    sys.exit("done")
toml = SC + "/📦️packages/🦀️rust/Cargo.toml"
new_crate(ART, SC + "/⚖️timber", N, "EN 1995 timber compliance formulas and check builders", ["semio-s-artifact-norm-en1995-kernel", "semio-s-artifact-norm-en1995-model", "semio-s-plugin-norm-compliance"], P, toml)
add_dep(toml, N)
pkg = ANY + "/🧭️spec-trace/📦️packages/🦀️rust"
wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1995 specification traceability table: every spec id with its clause, entity and realisation"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % relp(ART, pkg), TR) if False else wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1995 specification traceability table: every spec id with its clause, entity and realisation"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\nsemio-s-plugin-norm-compliance = { workspace = true }\n' % (relp(ART, pkg), TR))
register_workspace(ART, TR, pkg)
add_dep(toml, TR)
script_register(ART, P, TR)
lock_register(P, TR, extra_old_deps=[TR], new_deps=["semio-s-plugin-norm-compliance"])
old = '#[path = "."]\npub mod checks {\n    #[path = "⚖️timber/🦀️.rs"]\n    pub mod timber;\n}\n\n#[path = "../🧭️spec-trace/🦀️.rs"]\npub mod spec_trace;\n'
assert old in r
r = r.replace(old, "pub mod checks {\n    pub use %s as timber;\n}\n\npub use %s as spec_trace;\n" % (N.replace("-", "_"), TR.replace("-", "_")))
wr(root, r)
