"""🏛️ Writes the facade root `📦️packages/🦀️rust/🦀️.rs` with explicit re-exports of the pre-split public API."""
import os,re,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade import *
ORDER=["geometry","mesh","drawing","operators","vcs","extensions","host-source","host-core","host-read","host-session","host","wasm-domain","wasm-actions-a","wasm-actions-b","wasm"]
def locate(n,newn):
    for cid in ORDER:
        if cid in newn and n in newn[cid]:return cid
    return None
def build():
    on=old_names()
    newn={cid:new_names(cid) for cid in CR}
    ext_schema_old=names(ORIG+"🧩️extensions/🧬️schema/🦀️.rs")
    on["extensions::wasm"].update({k:v for k,v in ext_schema_old.items() if not k.startswith("EXTGLOB")})
    on["extensions::wasm"].pop("__extglobs",None)
    for m in on:on[m].pop("__extglobs",None)
    on["host"].pop("history_identity",None)
    groups={}
    for m,ns in on.items():
        for n in ns:
            if n=="schema":continue
            c=locate(n,newn)
            if c is None:
                if m=="extensions::wasm" and n in ext_schema_old:c="extensions"
                else:print("UNLOCATED",m,n,file=sys.stderr);continue
            groups.setdefault(c,set()).add(n)
    return groups
def render(groups):
    L=[]
    L.append("//! 🌊️ OS flow family facade — re-exports the pre-split public API of the document, catalogue, registry, bridge, host, drawing, vcs, geometry, mesh and wasm session producers.")
    L.append("//! Light/draw/brep operator packs are packaged extensions under ✏️s/🔌️plugins/🌊️flow.")
    L.append("")
    L.append("extern crate self as flow_extension_sdk;")
    L.append("")
    L.append("//#region 🔖️KernelModuleAliases")
    L+=["pub use semio_framework_os_kernel::os_dsl;","pub use semio_framework_os_kernel::os_pack;","pub use semio_framework_os_kernel::os_spr;","/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).","pub use semio_framework_os_kernel::os_store;","pub use semio_framework_os_kernel::os_vcs;","pub use protocol::value::ordered::{OrderedMap, OrderedSet};","//#endregion 🔖️KernelModuleAliases","","//#region 🔖️InfiniteAlias","/// ♾️ Flow components use `crate::infinite::{board,canvas}` paths.","pub use semio_framework_os_infinite as infinite;","//#endregion 🔖️InfiniteAlias","","//#region 🔖️DagCanvasNeural","pub use crate::infinite::board::ports::directed_dag as dag;","pub use semio_framework_canvas as canvas;","pub use neural_engine as neural;","//#endregion 🔖️DagCanvasNeural",""]
    L+=["pub use semio_framework_os_flow_vcs::forms;","pub use semio_framework_os_flow_operators::{bridge, catalogue, registry};","pub use semio_framework_os_flow_host as host;","pub use semio_framework_os_flow_drawing as drawing;","pub use semio_framework_os_flow_vcs as vcs;","pub use semio_framework_os_flow_mesh as mesh;","pub use semio_framework_os_flow_geometry as geometry;","","// 🌉️ `wasm_session`'s `flow_bridge_*` linear-memory ABI is the browser wasm-pack SDK entry point","// (its own `ui_webgpu::{CanvasMetrics, SurfaceId, SurfaceGeneration}` imports are canvas-surface","// types); nothing in `semio-s-plugin-flow`'s own `plugin_exports!` guest dispatch calls it","// (confirmed by repo-wide grep). `target_arch = \"wasm32\"` is TRUE for `wasm32-wasip2` too, so a","// bare arch gate would not exclude the shipped WASI component target — narrowed to keep native","// (the producers' own `#[cfg(test)]` suites over the bridge protocol) and browser wasm, drop only wasip2,","// matching `os-kernel-host-crates-split.md`'s target-table shape. RUNTIME-DEPENDENCY-ELIMINATION ticket","// 26/09/01.","#[cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))]","pub use semio_framework_os_flow_wasm as wasm_session;","","pub mod extensions {","    pub use semio_framework_os_flow_operators::extensions::schema;","    pub use semio_framework_os_flow_extensions as wasm;","}",""]
    for cid in ORDER:
        if cid in groups:
            ns=sorted(groups[cid],key=lambda s:(s.lower(),s))
            crate="semio_framework_os_flow_"+cid.replace("-","_")
            L.append(f"pub use {crate}::{{")
            for i in range(0,len(ns),6):L.append("    "+", ".join(ns[i:i+6])+",")
            L.append("};")
            L.append("")
    return "\n".join(L).rstrip("\n")+"\n"
if __name__=="__main__":
    g=build()
    txt=render(g)
    if "--write" in sys.argv:
        open(FL+"📦️packages/🦀️rust/🦀️.rs","w",encoding="utf-8").write(txt)
    else:print(txt)
