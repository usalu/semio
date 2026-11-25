import os,re
X="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml"
B=X+"/🏅️standards/🔖️1.0/🪆️subsets/🧱️base"
FAC=B+"/🧬️schema/📸️snapshot/📦️packages/🦀️rust"
orig=open(FAC+"/Cargo.toml").read()
dep_lines={}
for l in orig.split("[dev-dependencies]")[0].split("[dependencies]")[1].strip().split("\n"):
    dep_lines[l.split(" =")[0]]=l
dev_lines={}
for l in orig.split("[dev-dependencies]")[1].strip().split("\n"):
    dev_lines[l.split(" =")[0]]=l
def up(l): return l.replace('path = "../','path = "../../',1) if 'path = "../' in l else l
def manifest(name,desc,deps,devs,extra=()):
    s='[package]\nworkspace = "../../../../../../../../.."\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "🦀️.rs"\n\n[dependencies]\n'%(name,desc)
    s+="\n".join(up(dep_lines[d]) for d in deps)+"\n"
    s+="".join(e+"\n" for e in extra)
    if devs: s+="\n[dev-dependencies]\n"+"\n".join(up(dev_lines[d]) for d in devs)+"\n"
    return s
DOC="semio-s-artifact-stdio-xml-snapshot-document"
docdep='semio-s-artifact-stdio-xml-snapshot-document = { path = "../../../../../🧬️schema/📸️snapshot/📃️document/📦️packages/🦀️rust" }'
EXT='extern crate semio_os_kernel as store;\nextern crate semio_framework_schema as framework_schema;\nextern crate semio_framework_value_derive as value_derive;\n'
ALLDEPS=list(dep_lines)
crates={
 "doc":(B+"/🧬️schema/📸️snapshot/📃️document/📦️packages/🦀️rust",DOC,"Component crate of semio-s-artifact-stdio-xml: snapshot document model",ALLDEPS,list(dev_lines),[]),
 "text":(B+"/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust","semio-s-artifact-stdio-xml-snapshot-text","Component crate of semio-s-artifact-stdio-xml: text snapshot",ALLDEPS,list(dev_lines),[docdep]),
 "binary":(B+"/🚪️io/💾️binary/📸️snapshot/📦️packages/🦀️rust","semio-s-artifact-stdio-xml-snapshot-binary","Component crate of semio-s-artifact-stdio-xml: binary snapshot",ALLDEPS,[],[docdep]),
 "sqlite":(B+"/🚪️io/🪶️sqlite/📸️snapshot/📦️packages/🦀️rust","semio-s-artifact-stdio-xml-snapshot-sqlite","Component crate of semio-s-artifact-stdio-xml: sqlite snapshot",ALLDEPS,list(dev_lines),[docdep]),
}
for k,(d,n,desc,deps,devs,extra) in crates.items():
    os.makedirs(d,exist_ok=True)
    open(d+"/Cargo.toml","w").write(manifest(n,desc,deps,devs,extra))
D="semio_s_artifact_stdio_xml_snapshot_document"
core='''//! 📃️ snapshot document model of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

'''+EXT+'''
pub use schema::snapshot::XmlSnapshot;
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}
pub use crate::standards::v1_0::subsets::base::schema::mutation_support::artifact_items::{STDIO_XML_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../🔨️modules/🧬️mutation-support/🦀️.rs"]
                    pub mod mutation_support;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod valid {
                #[path = "."]
                pub mod schema {
                    #[path="../../../../../../✅️valid/🧬️schema/🏅️conformance/🦀️.rs"]
                    pub mod conformance;
                }
            }
        }
    }
}
'''
open(crates["doc"][0]+"/🦀️.rs","w").write(core)
def io(kind,emoji,title,extra_mods=""):
    return '''//! %s %s snapshot of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

%s
pub use %s::{schema, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use %s::standards::v1_0::subsets::base::schema;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod %s {
                        #[path = "../../🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
%s        }
    }
}
'''%(emoji,title,EXT,D,D,kind,extra_mods)
open(crates["text"][0]+"/🦀️.rs","w").write(io("text","📝️","text"))
open(crates["binary"][0]+"/🦀️.rs","w").write(io("binary","💾️","binary").replace("                pub use %s::standards::v1_0::subsets::base::schema;\n"%D,"                pub use %s::standards::v1_0::subsets::base::schema;\n                pub use %s::standards::v1_0::subsets::base::schema::mutation_support;\n"%(D,D)) if False else io("binary","💾️","binary"))
val='''            #[path = "."]
            pub mod valid {
                #[path = "."]
                pub mod schema {
                    pub use %s::standards::v1_0::subsets::valid::schema::conformance;
                }
            }
'''%D
open(crates["sqlite"][0]+"/🦀️.rs","w").write(io("sqlite","🪶️","sqlite",val))
