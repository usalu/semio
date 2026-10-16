"""📦 OS host carve step 3: manifests and project files of the five producers, parent facade mounts (run once)."""
import os,re,toml
R="/Users/ueli/Documents/semio/"
H=R+"🧰️framework/🛍️products/💻️os/🖥️host/"
PP=H+"📦️packages/🦀️rust/"
par=toml.load(PP+"Cargo.toml")
def emit(key,spec,childpkg):
    if isinstance(spec,str):return f'{key} = "{spec}"'
    parts=[]
    for k,v in spec.items():
        if k=="path":
            v=os.path.relpath(os.path.normpath(os.path.join(PP,v)),childpkg)
        if isinstance(v,bool):parts.append(f"{k} = {str(v).lower()}")
        elif isinstance(v,list):parts.append(f"{k} = ["+", ".join(f'"{x}"' for x in v)+"]")
        else:parts.append(f'{k} = "{v}"')
    return f"{key} = {{ {', '.join(parts)} }}"
def pdep(key,table=None):
    t=par["dependencies"] if table is None else table
    return t[key]
NP="cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))"
C={
 "codec-abi":dict(folder="📡️codec-abi",crate="semio-framework-os-host-codec-abi",id="os-host-codec-abi",desc="OS host codec ABI: operation schema, error codes, codec session, retained and direct codec services",
   deps=["semio-framework","semio-framework-os-kernel"],ndeps=[],dev=[],local=[]),
 "instance":dict(folder="🧱️instance",crate="semio-framework-os-host-instance",id="os-host-instance",desc="OS host app instance schemas, parameters and studio bindings",
   deps=["semio-framework","semio-framework-os-kernel","semio-framework-value","semio-framework-dsl-record-derive","serde","serde_json","semio-framework-artifact-workflow-workflow"],ndeps=[],dev=["semio-framework-ui-locale"],local=[]),
 "registry":dict(folder="🗂️registry",crate="semio-framework-os-host-registry",id="os-host-registry",desc="OS host plugin manifest registry, artifact catalog and the immediate kernel-future bridge",
   deps=["semio-framework","semio-framework-artifact-reference","semio-framework-ui-locale","serde","ui_wgpu","semio-framework-artifact-space-space","semio-framework-artifact-space-collection","semio-framework-artifact-workflow-workflow"],ndeps=[],dev=["semio-framework-pack-json","semio-framework-value"],local=["instance"]),
 "workflow":dict(folder="🔀️workflow",crate="semio-framework-os-host-workflow",id="os-host-workflow",desc="OS host workflow vocabulary, media negotiation and the media export and import handler registries",
   deps=["semio-framework","semio-framework-artifact-reference","semio-framework-async","semio-framework-os-kernel","semio-framework-value","semio-framework-value-derive","serde","serde_json","base64_codec","semio-framework-artifact-workflow-workflow"],ndeps=[],dev=["semio-framework-pack-json","semio-framework-ui-locale"],local=["instance","registry"]),
 "media-raster":dict(folder="🖼️media-export-raster",crate="semio-framework-os-host-media-raster",id="os-host-media-raster",desc="OS host native SVG rasterization, planar geometry extraction and the mesh export and import registrars",
   deps=["semio-framework","semio-framework-os-kernel","serde_json","base64_codec"],ndeps=["semio-framework-pixels","resvg","tiny-skia","usvg"],dev=["png","semio-framework-plugin"],local=["workflow"]),
}
for name,c in C.items():
    cp=H+c["folder"]+"/📦️packages/🦀️rust/"
    os.makedirs(cp,exist_ok=True)
    ws=os.path.relpath(R.rstrip("/"),cp.rstrip("/"))
    lines=["[package]",f'workspace = "{ws}"',f'name = "{c["crate"]}"','version = "0.1.0"','edition = "2021"','rust-version.workspace = true',f'description = "{c["desc"]}"','',"[package.metadata.semio]",'role = "product"',f'id = "{c["id"]}"','',"[package.metadata.semio.rust-source-boundary]","maxSourceBytes = 262144","maxSourceFiles = 32","maxActiveLines = 4000",'',"[lints]","workspace = true",'',"[lib]",'path = "../../🦀️.rs"','',"[dependencies]"]
    for k in c["deps"]:lines.append(emit(k,pdep(k),cp.rstrip("/")))
    for l in c["local"]:
        lc=C[l];lp=H+lc["folder"]+"/📦️packages/🦀️rust"
        lines.append(f'{lc["crate"]} = {{ path = "{os.path.relpath(lp,cp.rstrip("/"))}" }}')
    if c["ndeps"]:
        lines+=["",f"[target.'{NP}'.dependencies]"]
        for k in c["ndeps"]:lines.append(emit(k,par["target"][NP]["dependencies"][k],cp.rstrip("/")))
    dev=[]
    for k in c["dev"]:dev.append(emit(k,pdep(k) if k in par["dependencies"] else par["dev-dependencies"][k],cp.rstrip("/")))
    if name in("workflow","registry","media-raster"):
        d=par["dev-dependencies"]["semio-framework-os-kernel"];dev.append(emit("semio-framework-os-kernel",d,cp.rstrip("/")))
    if dev:lines+=["","[dev-dependencies]"]+dev
    open(cp+"Cargo.toml","w",encoding="utf-8").write("\n".join(lines)+"\n")
    open(cp+"📋️project.json","w",encoding="utf-8").write('{\n  "name": "@semio-tech/framework-%s-rs",\n  "tags": [\n    "lang:rust",\n    "role:product",\n    "family:os-host"\n  ]\n}\n'%c["id"])
    print(name,"written")
