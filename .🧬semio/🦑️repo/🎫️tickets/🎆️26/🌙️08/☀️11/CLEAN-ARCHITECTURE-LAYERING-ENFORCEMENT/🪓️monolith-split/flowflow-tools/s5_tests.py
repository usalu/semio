"""🧪 Step 5: relocate the document-level tests into laws crates and the retirement laws into the copy crate. Work tree only."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
LEAVES=[("➕️add-widget","AddWidget",0),("🗑️remove-widget","RemoveWidget",1),("↔️move-widget","MoveWidget",2),("🩹change-widget","ChangeWidget",3),("🔗️add-synapse","AddSynapse",4),("✂️remove-synapse","RemoveSynapse",5),("🔀️move-synapse","MoveSynapse",6),("🔄change-synapse","ChangeSynapse",7),("📐️change-layout","ChangeLayout",8)]
M="🌿️vcs/🧬️schema/🧬️mutations/"
REG='\n//#region 🧪️Tests\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n'
def leaves():
    body="//! 🔬️ Direct-leaf contract of every Flow mutation payload: descriptor, codecs, unknown and missing fields, inverse replay.\n\nuse super::flow_direct_tests;\nuse crate::*;\n"
    for d,name,idx in LEAVES:
        t=read(M+d+"/🦀️.rs")
        assert t.endswith(REG),d
        write(M+d+"/🦀️.rs",t[:-len(REG)]+"")
        os.remove(p(M+d+"/🧪️tests/🔬️unit/🦀️.rs"))
        os.rmdir(os.path.dirname(p(M+d+"/🧪️tests/🔬️unit/🦀️.rs")))
        os.rmdir(os.path.dirname(os.path.dirname(p(M+d+"/🧪️tests/🔬️unit/🦀️.rs"))))
        fn=re.sub(r"^[^a-z]+","",d).replace("-","_")
        body+=f'\n//#region 🧪️Tests\n#[test]\nfn {fn}_direct_leaf_contract() {{\n    flow_direct_tests::assert_leaf_contract::<{name}>({idx}, FlowMutation::{name}, include_str!("../../🧬️schema/🧬️mutations/{d}/🔣️.json"));\n}}\n//#endregion 🧪️Tests\n'
    write("🌿️vcs/🧪️tests/🔬️leaf-contracts/🦀️.rs",body)
def vcs():
    t=read("🌿️vcs/🦀️.rs")
    t=replace(t,'#[cfg(test)]\n#[path = "🧪️tests/🌿️vcs/🦀️.rs"]\nmod flow_direct_tests;\n\n',"")
    write("🌿️vcs/🦀️.rs",t)
    d=read("🌿️vcs/🧬️schema/🔺️diff/🦀️.rs")
    d,_=cut(d,"\n//#region 🧪️Ownership\n","//#endregion 🧪️Ownership\n")
    write("🌿️vcs/🧬️schema/🔺️diff/🦀️.rs",d.rstrip("\n")+"\n")
if __name__=="__main__":
    leaves();vcs();print("ok")
