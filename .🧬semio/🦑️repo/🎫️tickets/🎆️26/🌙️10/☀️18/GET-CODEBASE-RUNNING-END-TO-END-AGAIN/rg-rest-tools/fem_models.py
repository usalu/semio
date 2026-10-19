import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
F2 = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
F3 = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
SC = "/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema"
ST2 = F2 + SC + "/🏗️structure"
ST3 = F3 + SC + "/🏗️structure"
FD = ST2 + "/🎟️fixed-dofs/🧪️tests/🦀️.rs"
UT = ST2 + "/🧪️tests/🔬️unit/🦀️.rs"
# 2d lib unmount
t = rd(ST2 + "/🦀️.rs")
a = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n\n'
b = '#[cfg(test)]\n#[path="🎟️fixed-dofs/🧪️tests/🦀️.rs"]\nmod fixed_dof_tests;\n'
assert a in t and b in t
t = t.replace(a, "", 1).replace(b, "", 1)
wr(ST2 + "/🦀️.rs", t)
# 3d lib unmount
t = rd(ST3 + "/🦀️.rs")
b3 = '#[cfg(test)]\n#[path="../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🎟️fixed-dofs/🧪️tests/🦀️.rs"]\nmod fixed_dof_tests;\n'
assert b3 in t
wr(ST3 + "/🦀️.rs", t.replace(b3, "", 1))
# test files
S_PATH = "standards::v1::subsets::any::schema::structure"
f = rd(FD)
f = f.replace("use super::*;\n", "use crate::model::%s::{decode_fixed_dofs, retire_fixed_dofs, FemSupport};\nuse semio_s_artifact_fem_2d_model::%s::FemDof;\n" % (S_PATH, S_PATH), 1)
wr(FD, f)
u = rd(UT)
u = u.replace("use super::*;\n", "use crate::model::%s::{FemAnalysisSettings, FemDof};\n" % S_PATH, 1)
wr(UT, u)
# roots
r2 = ST2 + "/🎟️fixed-dofs/🧪️tests/🔬️model-2d/🦀️.rs"
wr(r2, '''//! 🧪️ The fem 2d model structure laws as an integration target: the shared fixed dof boundary law and the dof/settings unit laws, run against the public model API.

extern crate semio_s_artifact_fem_2d_model as model;

#[path = "../🦀️.rs"]
mod fixed_dof_tests;
#[path = "../../../🧪️tests/🔬️unit/🦀️.rs"]
mod structure_unit_tests;
''')
r3 = ST3 + "/🧪️tests/🔬️fixed-dofs/🦀️.rs"
rel = os.path.relpath(ROOT + "/" + FD, ROOT + "/" + os.path.dirname(r3))
wr(r3, '''//! 🧪️ The fem 3d model fixed dof boundary law as an integration target, the same neutral canonical corpus the fem 2d model runs.

extern crate semio_s_artifact_fem_3d_model as model;

#[path = "%s"]
mod fixed_dof_tests;
''' % rel)
# manifests
def add_test(manifest, name, root_file):
    t = rd(manifest)
    rp = os.path.relpath(ROOT + "/" + root_file, ROOT + "/" + os.path.dirname(manifest))
    if '[[test]]' in t and 'name = "%s"' % name in t: return
    wr(manifest, t.rstrip("\n") + '\n\n[[test]]\nname = "%s"\npath = "%s"\n' % (name, rp))
add_test(F2 + SC + "/📦️packages/🦀️rust/Cargo.toml", "structure_laws", r2)
add_test(F3 + SC + "/📦️packages/🦀️rust/Cargo.toml", "fixed_dofs_laws", r3)
# exact-law reader
S = F2 + "/📦️packages/🦀️rust/📜️script.ts"
s = rd(S)
old = 'groups:Object.keys(manifestPaths).map(packageName=>({package:packageName,target:{kind:"lib" as const},cargoArgs:["--locked"],laws:["standards::v1::subsets::any::schema::structure::fixed_dof_tests::fixed_dof_boundary_matches_neutral_canonical_sets"]}))'
assert old in s
new = 'groups:[["semio-s-artifact-fem-2d-model","structure_laws"],["semio-s-artifact-fem-3d-model","fixed_dofs_laws"]].map(([packageName,name])=>({package:packageName,target:{kind:"test" as const,name:name!},cargoArgs:["--locked"],laws:["fixed_dof_tests::fixed_dof_boundary_matches_neutral_canonical_sets"]}))'
wr(S, s.replace(old, new, 1))
