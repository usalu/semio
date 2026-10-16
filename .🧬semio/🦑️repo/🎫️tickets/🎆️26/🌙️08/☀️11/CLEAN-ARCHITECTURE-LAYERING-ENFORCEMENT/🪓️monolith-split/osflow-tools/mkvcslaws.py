"""⚖️ Moves the flow-vcs suite into the test-only crate vcs-laws and reads its fixtures at runtime. Idempotent per step."""
import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from families import FL
V=FL+"🌿️vcs/"
T=V+"🧪️tests/🔬️flow-vcs/🦀️.rs"
def rd(p):return open(p,encoding="utf-8").read()
def wr(p,t):open(p,"w",encoding="utf-8").write(t)
t=rd(T)
MAP=[('include_str!("../🧫️fixtures/','read_node("🧪️tests/🧫️fixtures/'),
 ('include_str!("🦀️.rs")','read_node("🧪️tests/🔬️flow-vcs/🦀️.rs")'),
 ('include_str!("../../🦀️.rs")','read_node("🦀️.rs")'),
 ('include_str!("../../../📚️examples/','read_node("../📚️examples/'),
 ('include_str!("../../♻️retirement/','read_node("♻️retirement/')]
for a,b in MAP:t=t.replace(a,b)
t=t.replace("use crate::history_identity;\n","")
assert "include_str" not in t
wr(T,t)
r=rd(V+"🦀️.rs")
m='#[cfg(test)]\n#[path = "🧪️tests/🔬️flow-vcs/🦀️.rs"]\nmod flow_vcs_tests;\n\n'
if m in r:wr(V+"🦀️.rs",r.replace(m,"",1))
