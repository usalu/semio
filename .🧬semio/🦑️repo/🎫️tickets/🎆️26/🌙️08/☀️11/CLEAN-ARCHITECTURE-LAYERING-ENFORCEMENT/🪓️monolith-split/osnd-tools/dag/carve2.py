"""✂️ DAG carve step 2: the 14 per-leaf stub tests become one consolidated leaf-contract test mounted by the parent."""
import os,re
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/"
M=D+"🌿️vcs/🧬️schema/🧬️mutations/"
STUB='\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
rows=[]
for leaf in sorted(os.listdir(M)):
    lp=M+leaf
    if not os.path.isdir(lp):continue
    rs=lp+"/🦀️.rs";ut=lp+"/🧪️tests/🔬️unit/🦀️.rs"
    s=open(rs,encoding="utf-8").read()
    assert s.count(STUB)==1 and s.endswith(STUB),leaf
    u=open(ut,encoding="utf-8").read()
    m=re.search(r'assert_leaf_contract::<(\w+)>\((\d+), DagMutation::(\w+), include_str!\("\.\./\.\./🔣️\.json"\)\)',u)
    assert m and m.group(1)==m.group(3),(leaf,u)
    rows.append((int(m.group(2)),m.group(1),leaf))
rows.sort()
assert [r[0] for r in rows]==list(range(14)),rows
def snake(n):return re.sub(r'(?<!^)(?=[A-Z])','_',n).lower()
body="//! 🧾️ Direct leaf contract of every DAG mutation: the committed descriptor, wire witness, codecs and exact inverse of each leaf.\nuse super::dag_direct_tests::assert_leaf_contract;\nuse super::{"+", ".join(sorted([r[1] for r in rows]+["DagMutation"]))+"};\n"
for idx,ty,leaf in rows:
    body+=f'\n#[test]\nfn direct_leaf_contract_{snake(ty)}() {{\n    assert_leaf_contract::<{ty}>({idx}, DagMutation::{ty}, include_str!("../../🧬️schema/🧬️mutations/{leaf}/🔣️.json"));\n}}\n'
os.makedirs(D+"🌿️vcs/🧪️tests/🔬️leaf-contracts",exist_ok=True)
open(D+"🌿️vcs/🧪️tests/🔬️leaf-contracts/🦀️.rs","w",encoding="utf-8").write(body)
for idx,ty,leaf in rows:
    lp=M+leaf
    rs=lp+"/🦀️.rs";s=open(rs,encoding="utf-8").read()
    open(rs,"w",encoding="utf-8").write(s[:-len(STUB)]+"\n" if not s[:-len(STUB)].endswith("\n") else s[:-len(STUB)])
    os.remove(lp+"/🧪️tests/🔬️unit/🦀️.rs");os.rmdir(lp+"/🧪️tests/🔬️unit");os.rmdir(lp+"/🧪️tests")
print("leaves",len(rows))
