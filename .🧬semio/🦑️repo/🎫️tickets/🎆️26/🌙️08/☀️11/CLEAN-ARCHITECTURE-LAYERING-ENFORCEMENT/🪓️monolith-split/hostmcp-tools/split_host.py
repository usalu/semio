import re,sys,hashlib,os
from rlex import lex
HOST="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/"
SRC=HOST+"🦀️.rs"
ORIG=os.path.join(os.path.dirname(os.path.abspath(__file__)),"orig-host.rs.txt")
FOLDERS={"host":"🔌️plugin-host","backbone":"🗃️document-backbone","instance":"🧱️instance","media_export_raster":"🖼️media-export-raster","media_export_simple":"🧾️media-export-simple","workflow":"🔀️workflow","codec_abi":"📡️codec-abi","registry":"🗂️registry"}
RENAMES={"registry_export_media_legacy":"registry_export_media_via_composer_registry","registry_import_media_legacy":"registry_import_media_via_composer_registry"}
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
data=open(ORIG,encoding="utf-8").read()
if apply:
    cur=open(SRC,encoding="utf-8").read()
    assert hashlib.sha1(cur.encode()).hexdigest()==hashlib.sha1(data.encode()).hexdigest(),"source changed since snapshot"
L=data.split("\n")
assert L[-1]==""
L.pop()
L=[l+"\n" for l in L]
I=lex(L)
mods=[]
for i,l in enumerate(L):
    m=re.match(r"pub mod (\w+) \{$",l.rstrip("\n"))
    if m and I[i]["depth"]==0:
        j=next(k for k in range(i+1,len(L)) if I[k]["end_depth"]==0)
        assert L[j]=="}\n"
        mods.append((m.group(1),i,j))
assert [m[0] for m in mods]==list(FOLDERS),mods
def dedent(lo,hi):
    out=[]
    for k in range(lo,hi):
        l=L[k]
        if I[k]["state"] in("code","blk") and l.startswith("    "):
            l=l[4:]
        out.append(l)
    return out
children={}
for name,i,j in mods:
    body=dedent(i+1,j)
    text="".join(body)
    n_inc=len(re.findall(r'include!\("🧪️tests/',text))
    n_str=len(re.findall(r'include_str!\("🧬️schema/',text))
    text=text.replace('include!("🧪️tests/','include!("../🧪️tests/').replace('include_str!("🧬️schema/','include_str!("../🧬️schema/')
    for a,b in RENAMES.items():
        text=text.replace(a,b)
    bl=text.split("\n")
    if bl[-1]=="":bl.pop()
    bl=[x+"\n" for x in bl]
    if bl[0].strip().startswith("// #region") and bl[1].startswith("//!"):
        k=1
        while bl[k].startswith("//!"):k+=1
        bl=bl[1:k]+[bl[0]]+bl[k:]
    elif name=="codec_abi":
        bl=["//! 📡️ Host codec ABI: operation schema, error codes, codec session, retained and direct codec services.\n"]+bl
    if name=="media_export_simple":
        assert bl[0].startswith("//! 🖼️")
        bl[0]=bl[0].replace("🖼️","🧾️",1)
    children[name]=(bl,n_inc,n_str)
parent=[]
parent+=L[0:3]
parent.append("\n")
for name,i,j in mods:
    parent.append(f'#[path = "{FOLDERS[name]}/🦀️.rs"]\n')
    parent.append(f"pub mod {name};\n\n")
parent+=L[mods[-1][2]+1:]
while parent[parent.index("pub use crate::space::*;\n")-1]=="\n" and parent[parent.index("pub use crate::space::*;\n")-2]=="\n":
    del parent[parent.index("pub use crate::space::*;\n")-1]
if not apply:
    for name,(bl,a,b) in children.items():
        print(name,len(bl),a,b,bl[0].strip()[:50])
    print("parent",len(parent))
    sys.exit(0)
for name,(bl,a,b) in children.items():
    d=HOST+FOLDERS[name]
    os.makedirs(d,exist_ok=True)
    open(d+"/🦀️.rs","w",encoding="utf-8").write("".join(bl))
open(SRC,"w",encoding="utf-8").write("".join(parent))
print("applied")
