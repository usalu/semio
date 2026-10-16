"""🗜️ Step 7: bring the document crate under the file budget by inlining aggregator mounts and folding tiny impl files. Work tree only."""
import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
def rm(rel):
    path=p(rel);guard(os.path.dirname(path));os.remove(path)
    d=os.path.dirname(path)
    while d!=os.path.realpath(WORK).rstrip("/") and not os.listdir(d):os.rmdir(d);d=os.path.dirname(d)
# 1. io aggregators -> one file with inline #[path] modules
write("🚪️io/🦀️.rs",'''//! 🚪️ Artifact representation assembly: the text, binary pack and SQLite representations of the Flow host snapshot.
#[path = "📝️text"]
pub mod text {
    #[path = "📸️snapshot/🧾️host/🦀️.rs"]
    pub mod snapshot;
}
#[path = "💾️binary"]
pub mod binary {
    #[path = "📸️snapshot/🦀️.rs"]
    pub mod snapshot;
}
#[path = "🪶️sqlite"]
pub mod sqlite {
    #[path = "📸️snapshot"]
    pub mod snapshot {
        use crate::vcs::*;
        use crate::io::text::snapshot::*;
        #[path = "🚦️native/🛫️encoding/🦀️.rs"]
        pub(crate) mod flow_native_encoding;
        #[path = "🚦️native/🛬️decoding/🦀️.rs"]
        pub(crate) mod flow_native_decoding;
        #[path = "🚦️native/♻️retirement/🦀️.rs"]
        pub(crate) mod flow_native_retirement;
        #[path = "🚦️native/🛂️carrier/🦀️.rs"]
        pub(crate) mod flow_native_carrier;
        #[path = "🚦️native/📏️bound/🦀️.rs"]
        pub(crate) mod flow_native_bound;
        #[path = "🚦️native/🛂️capability/🦀️.rs"]
        pub(crate) mod flow_snapshot_capability;
        #[path = "🛫️projection/🦀️.rs"]
        pub(crate) mod flow_sql_projection;
        #[path = "🛬️reconstruction/🦀️.rs"]
        pub(crate) mod flow_sql_reconstruction;
    }
}
''')
text_mut=read("🚪️io/📝️text/🧬️mutations/🦀️.rs")
bin_mut=read("🚪️io/💾️binary/🧬️mutations/🦀️.rs")
retire=read("🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs")
for f in ("🚪️io/📝️text/🦀️.rs","🚪️io/💾️binary/🦀️.rs","🚪️io/🪶️sqlite/🦀️.rs","🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs","🚪️io/📝️text/🧬️mutations/🦀️.rs","🚪️io/💾️binary/🧬️mutations/🦀️.rs","🌿️vcs/🧬️schema/🧹️retirement/🦀️.rs"):rm(f)
# 2. mutation codecs + owned retirement into the aggregate file
def body(t,start):
    i=one(t,start);return t[i:]
tm=body(text_mut,"impl crate::os_spr::OpText for FlowMutation {")
bm=body(bin_mut,"impl crate::os_spr::OpBinary for FlowMutation {")
rt=body(retire,"macro_rules! owned_record {")
M="🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs"
t=read(M)
t=replace(t,"use semio_framework_value_derive::{FromValue, ToValue};\n","use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, sequence, sequence_birth_bytes, RetireOwned, RetirementCursor};\nuse semio_framework_value_derive::{FromValue, ToValue};\n")
t=replace(t,"//#region 🔤️Codecs\n\n\n\n//#endregion 🔤️Codecs\n","//#region 🔤️Codecs\n"+tm.rstrip("\n")+"\n\n"+bm.rstrip("\n")+"\n//#endregion 🔤️Codecs\n\n//#region 🧹️OwnedRetirement\n"+rt.rstrip("\n")+"\n//#endregion 🧹️OwnedRetirement\n")
write(M,t)
v=read("🌿️vcs/🦀️.rs")
v=replace(v,'#[path = "🧬️schema/🧹️retirement/🦀️.rs"]\nmod flow_mutation_retirement;\n\n',"")
write("🌿️vcs/🦀️.rs",v)
# 3. diff projection inline
D="🌿️vcs/🧬️schema/🔺️diff/🦀️.rs"
pj=read("🌿️vcs/🧬️schema/🔺️diff/📑️projection/🦀️.rs")
rm("🌿️vcs/🧬️schema/🔺️diff/📑️projection/🦀️.rs")
d=read(D)
d=replace(d,'#[path = "📑️projection/🦀️.rs"]\nmod projection;\nuse projection::FlowProjection;\n',"use projection::FlowProjection;\n")
inline="mod projection {\n"+pj.rstrip("\n")+"\n}\n\n"
d=replace(d,"//#region ▶️Application\n",inline+"//#region ▶️Application\n")
write(D,d)
print("ok")
