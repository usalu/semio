import sys
from pathlib import Path
IFC=Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc")
FW={"value":"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust","pack-json":"🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust","ui-locale":"🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust","staged-form":"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/staged-form/📦️packages/🦀️rust"}
WS=["semio-framework-job","semio-framework-os-kernel","semio-framework-plugin","semio-s-artifact-stdio-contract","semio-s-artifact-stdio-ifc-4-model","semio-s-artifact-stdio-ifc-4-mutations","semio-s-artifact-stdio-ifc-declarations"]
CR={
 "profile":("🔎️details/🧭️profile","semio-s-artifact-stdio-ifc-details-profile","Declaration roles, type metadata and local-rule candidates every IFC details layer shares.",["plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "wire":("🔎️details/📨️wire","semio-s-artifact-stdio-ifc-details-wire","Borrowed pointers, owned receivers, publications and source loans over the retained IFC graph.",["profile","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "type":("🔎️details/🔤️type","semio-s-artifact-stdio-ifc-details-type","Guided type input cursors, type remapping and the constituent roster over declaration metadata.",["wire","domain","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "edit":("🔎️details/✏️edit","semio-s-artifact-stdio-ifc-details-edit","Value, reference, enumeration and domain edits over the retained IFC graph.",["type","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "provider":("🔎️details/👁️provider","semio-s-artifact-stdio-ifc-details-provider","The declaration-aware snapshot details provider.",["edit","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "guidance":("🔎️details/🧶️guidance","semio-s-artifact-stdio-ifc-details-guidance","Staged type-input sources and guided candidates.",["edit","staged-form","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "details":("🔎️details","semio-s-artifact-stdio-ifc-details","Catalog-profile declaration-aware controls and retained edits over the shared lossless IFC graph.",["provider","guidance","os-kernel","pack-json","plugin","ui-locale","value","job","contract","declarations","model","mutations"]),
 "domain":("📚️catalog/⚖️domain","semio-s-artifact-stdio-ifc-domain","Schema-neutral IFC declaration domains, changed-subtree admission and bounded declaration cursors.",["contract","declarations","model"]),
}
NAMES={k:v[1] for k,v in CR.items()}
def dep(key,ups):
    if key in FW:return "semio-framework-%s = { path = \"%s%s\" }"%(key,"../"*ups,FW[key])
    n={"plugin":"semio-framework-plugin","job":"semio-framework-job","os-kernel":"semio-framework-os-kernel","contract":"semio-s-artifact-stdio-contract","declarations":"semio-s-artifact-stdio-ifc-declarations","model":"semio-s-artifact-stdio-ifc-4-model","mutations":"semio-s-artifact-stdio-ifc-4-mutations"}.get(key)
    if n:return "%s = { workspace = true }"%n
    return "%s = { workspace = true }"%NAMES[key]
for key,(d,name,desc,deps) in CR.items():
    base=IFC/d/"📦️packages/🦀️rust"
    depth=len(Path(d).parts)+2
    ups=depth+5
    base.mkdir(parents=True,exist_ok=True)
    lines=["[package]","workspace = \"%s\""%("../"*(depth+0)).rstrip("/"),"name = \"%s\""%name,"version.workspace = true","edition.workspace = true","rust-version.workspace = true","description = \"%s\""%desc,"","[package.metadata.semio]","role = \"library\"","","[lints]","workspace = true","","[lib]","path = \"%s🦀️.rs\""%("../../"),"","[dependencies]"]
    ups_fw=depth+5
    for k in deps:
        if k in FW:lines.append("semio-framework-%s = { path = \"%s%s\" }"%(k,"../"*ups_fw,FW[k]))
        else:lines.append(dep(k,ups_fw))
    (base/"Cargo.toml").write_text("\n".join(lines)+"\n")
    print(d,depth,ups_fw)
