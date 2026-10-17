#!/usr/bin/env python3
# 🔧️ Verify-1 findings: retirement module under the schema wrapper, FlowDirectStoreWork reachable from its hoisted tests, sqlite snapshot test imports the type from the crate.
ROOT="/Users/ueli/Documents/semio"
A=ROOT+"/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow"
S=A+"/🏅️standards/🔖️1/🪆️subsets/✳️any"
def edit(p,pairs):
    t=open(p,encoding="utf8").read()
    for a,b,n in pairs:
        assert t.count(a)==n,(p,a,t.count(a)); t=t.replace(a,b)
    open(p,"w",encoding="utf8").write(t)
edit(A+"/🦀️.rs",[
 ('#[path = "♻️retirement/🦀️.rs"]\npub mod retirement;\n\n','',1),
 ('''                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
''','''                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "♻️retirement/🦀️.rs"]
                    pub mod retirement;
''',1)])
edit(S+"/✏️editor/🦀️.rs",[("crate::retirement::store_owners_source_demands()","crate::schema::retirement::store_owners_source_demands()",1),("crate::retirement::store_owners(grant)","crate::schema::retirement::store_owners(grant)",1),
 ("struct FlowDirectStoreWork {\n    tool_id","pub struct FlowDirectStoreWork {\n    tool_id",1),
 ("    preview_off: Option<Vec<String>>,\n    preview_next","    pub preview_off: Option<Vec<String>>,\n    preview_next",1),
 ("impl FlowDirectStoreWork {\n    fn new(tool_id: &'static str) -> Self {","impl FlowDirectStoreWork {\n    pub fn new(tool_id: &'static str) -> Self {",1)])
edit(S+"/👁️viewer/🦀️.rs",[("crate::retirement::store_owners_source_demands()","crate::schema::retirement::store_owners_source_demands()",1),("crate::retirement::store_owners(grant)","crate::schema::retirement::store_owners(grant)",1)])
edit(S+"/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs",[("use crate::standards::v1::subsets::any::io::sqlite::snapshot::FlowSnapshot;","use crate::FlowSnapshot;",1)])
