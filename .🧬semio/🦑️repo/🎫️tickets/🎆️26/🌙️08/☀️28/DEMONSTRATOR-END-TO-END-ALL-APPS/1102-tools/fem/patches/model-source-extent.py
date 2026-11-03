import sys
M="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any"
D=M+"/🧬️schema/🔺️diff/🦀️.rs"
T=M+"/🧪️tests/diff-tests.rs"
t=open(D).read()
a='#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
if t.count(a)!=1: sys.exit("anchor count != 1")
import os
if not os.path.exists(T): sys.exit("diff-tests.rs must exist in the dedicated test crate first")
open(D,"w").write(t.replace(a,""))
print("diff unit tests now compile in semio-s-artifact-fem-3d-snapshot-tests (diff-tests.rs); the model crate drops ~16 KB of source; leave the old 🧪️tests/🔬️unit/🦀️.rs unreferenced and delete it by hand")
