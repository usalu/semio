"""✂️ Carves the PDF page tree and page content cursors out of the lift crate and registers both crates in every manifest and lock file."""
import os, re, subprocess, json
os.chdir(subprocess.run(["git","rev-parse","--show-toplevel"],capture_output=True,text=True).stdout.strip())
PDF="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
MOD="🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules"
LIFT=f"{PDF}/{MOD}/⬇️lift"
read=lambda p:open(p,encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True); open(p,"w",encoding="utf8").write(t)
def manifest(name,description,up,deps):
    rows="\n".join(f'{k} = {{ path = "{"../"*up}{v}" }}' for k,v in deps)
    return f'''[package]
workspace = "{"/".join([".."]*(up+5))}"
name = "{name}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{description}"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
{rows}
'''
VALUE="../"*10+"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
DIFF="../../../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📦️packages/🦀️rust"
INF="../🧬️schema/💡️inferences/📦️packages/🦀️rust"
SNAP="../🧬️schema/📸️snapshot/📦️packages/🦀️rust"
XREF="🔗️xref/💧️stream/📦️packages/🦀️rust"
CONTENT="🖋️content/📦️packages/🦀️rust"
def lib(title,modules,mount):
    return f'''//! {title}

#![allow(async_fn_in_trait)]

#[path = "."]
pub mod standards {{
    #[path = "."]
    pub mod v1_7 {{
        #[path = "."]
        pub mod subsets {{
            #[path = "."]
            pub mod base {{
                #[path = "."]
                pub mod modules {{
{modules}
                }}
                #[path = "."]
                pub mod schema {{
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                }}
            }}
        }}
    }}
}}

{mount}
'''
LEXER="                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;"
tree=f"{LIFT}/🌳️page-tree/📦️packages/🦀️rust"; page=f"{LIFT}/🧩️lifter-and-document/📜️page-content/📦️packages/🦀️rust"
write(f"{tree}/Cargo.toml",manifest("semio-s-artifact-stdio-pdf-page-tree","Resumable PDF page-tree traversal",4,[("semio-s-artifact-stdio-pdf-diff",DIFF),("semio-s-artifact-stdio-pdf-inferences",INF),("semio-s-artifact-stdio-pdf-snapshot-2",SNAP),("semio-s-artifact-stdio-pdf-xref-stream",XREF)]))
write(f"{tree}/🦀️.rs",lib("🌳️ Resumable PDF page-tree traversal.",LEXER,'#[path = "../../🦀️.rs"]\nmod page_tree;\npub use page_tree::*;'))
write(f"{page}/Cargo.toml",manifest("semio-s-artifact-stdio-pdf-page-content","Controlled PDF page content cursors",5,[("semio-framework-value",VALUE),("semio-s-artifact-stdio-pdf-diff",DIFF),("semio-s-artifact-stdio-pdf-inferences",INF),("semio-s-artifact-stdio-pdf-mut-family-5",CONTENT),("semio-s-artifact-stdio-pdf-snapshot-2",SNAP)]))
write(f"{page}/🦀️.rs",lib("📜️ Controlled PDF page content cursors.",LEXER+"\n                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;",'#[path = "../../🦀️.rs"]\nmod page_content;\npub use page_content::{project_content_bytes, PageContentBarrier, PageContentCursor, PageContentEvent, PageContentReceipt, PageProgramCursor};'))
def swap(p,old,new,count=1):
    t=read(p); assert t.count(old)==count,(p,old,t.count(old)); write(p,t.replace(old,new))
swap(f"{LIFT}/🦀️.rs",'#[path = "🌳️page-tree/🦀️.rs"]\nmod page_tree;\npub use page_tree::*;','pub use semio_s_artifact_stdio_pdf_page_tree::*;' if False else 'pub use semio_s_artifact_stdio_pdf_page_tree::{PageAttribute, PageInheritance, PageTreeCursor, PageTreeEvent, PageTreeLeaf, PageTreeReceipt};')
p=f"{LIFT}/🧩️lifter-and-document/🦀️.rs"
swap(p,'#[path = "📜️page-content/🦀️.rs"]\nmod page_content;\npub use page_content::{','pub use semio_s_artifact_stdio_pdf_page_content::{')
swap(p,'page_content::project_content_bytes(','project_content_bytes(')
p=f"{LIFT}/📦️packages/🦀️rust/Cargo.toml"
swap(p,'semio-s-artifact-stdio-pdf-xref-stream = { path = "../../../🔗️xref/💧️stream/📦️packages/🦀️rust" }\n','semio-s-artifact-stdio-pdf-page-content = { path = "../../🧩️lifter-and-document/📜️page-content/📦️packages/🦀️rust" }\nsemio-s-artifact-stdio-pdf-page-tree = { path = "../../🌳️page-tree/📦️packages/🦀️rust" }\n')
t=read(p); t="".join(l for l in t.splitlines(True) if "lift::page_tree" not in l and "document::page_content" not in l); write(p,t)
p=f"{PDF}/Cargo.toml"
swap(p,f'    "{MOD}/⬇️lift/📦️packages/🦀️rust",\n',f'    "{MOD}/⬇️lift/🌳️page-tree/📦️packages/🦀️rust",\n    "{MOD}/⬇️lift/📦️packages/🦀️rust",\n    "{MOD}/⬇️lift/🧩️lifter-and-document/📜️page-content/📦️packages/🦀️rust",\n')
p=f"{PDF}/📦️packages/🦀️rust/📜️script.ts"
swap(p,'"semio-s-artifact-stdio-pdf-page-editor", ','"semio-s-artifact-stdio-pdf-page-content", "semio-s-artifact-stdio-pdf-page-editor", ')
swap(p,'"semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests", ','"semio-s-artifact-stdio-pdf-page-tree", "semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests", ')
p="📋️build-owners.json"
t=read(p); anchor='    {\n      "name": "semio-s-artifact-stdio-pdf-xref-stream",'; assert t.count(anchor)==1
row=lambda n,r:f'    {{\n      "name": "{n}",\n      "root": "{r}",\n      "manifest": "{r}/Cargo.toml",\n      "packages": []\n    }},\n'
write(os.path.join(".",p),t.replace(anchor,row("semio-s-artifact-stdio-pdf-page-content",page)+row("semio-s-artifact-stdio-pdf-page-tree",tree)+anchor))
p="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📏️paths/🧪️tests/🧫️fixtures/🔣️.json"
t=read(p); json.loads(t)
new=lambda n:f'    {{\n      "packageName": "{n}",\n      "crateName": "{n.replace("-","_")}",\n      "platform": "wasm32-wasip2",\n      "profile": "prototype"\n    }},\n'
names=re.findall(r'"packageName": "([^"]+)"',t)
for n in ("semio-s-artifact-stdio-pdf-page-content","semio-s-artifact-stdio-pdf-page-tree"):
    nxt=min(x for x in names if x>n and x.startswith("semio-s-artifact-stdio-pdf")); a='    {\n      "packageName": "%s",'%nxt
    assert t.count(a)>=1,nxt; t=t.replace(a,new(n)+a,1)
json.loads(t); write(p,t)
TREE_DEPS=["semio-s-artifact-stdio-pdf-diff","semio-s-artifact-stdio-pdf-inferences","semio-s-artifact-stdio-pdf-snapshot-2","semio-s-artifact-stdio-pdf-xref-stream"]
PAGE_DEPS=["semio-framework-value","semio-s-artifact-stdio-pdf-diff","semio-s-artifact-stdio-pdf-inferences","semio-s-artifact-stdio-pdf-mut-family-5","semio-s-artifact-stdio-pdf-snapshot-2"]
changed=[]; skipped=[]
for f in subprocess.run(["git","-c","core.quotepath=off","ls-files","*Cargo.lock"],capture_output=True,text=True).stdout.split("\n"):
    if not f or "/🎫️tickets/" in f or not os.path.exists(f): continue
    s=read(f)
    if 'name = "semio-s-artifact-stdio-pdf-lift"\n' not in s: continue
    blocks=s.split("\n[[package]]\n"); head,pk=blocks[0],blocks[1:]
    name=lambda b:re.search(r'name = "([^"]+)"',b).group(1)
    names=[name(b) for b in pk]; i=names.index("semio-s-artifact-stdio-pdf-lift")
    m=re.search(r'dependencies = \[\n(.*?)\n\]',pk[i],re.S); deps=set(d.strip().strip(',"') for d in m.group(1).split("\n"))
    if not set(TREE_DEPS+PAGE_DEPS)<=deps: skipped.append((f,sorted(set(TREE_DEPS+PAGE_DEPS)-deps))); continue
    deps=sorted((deps-{"semio-s-artifact-stdio-pdf-xref-stream"})|{"semio-s-artifact-stdio-pdf-page-content","semio-s-artifact-stdio-pdf-page-tree"})
    pk[i]=pk[i][:m.start(1)]+"\n".join(f' "{d}",' for d in deps)+pk[i][m.end(1):]
    for n,d in (("semio-s-artifact-stdio-pdf-page-content",PAGE_DEPS),("semio-s-artifact-stdio-pdf-page-tree",TREE_DEPS)):
        names=[name(b) for b in pk]; pos=next(k for k,x in enumerate(names) if x>n)
        pk.insert(pos,f'name = "{n}"\nversion = "0.1.0"\ndependencies = [\n'+"\n".join(f' "{x}",' for x in d)+'\n]\n')
    write(os.path.join(".",f),head+"".join("\n[[package]]\n"+b for b in pk)); changed.append(f)
print(len(changed),"locks changed",skipped)
