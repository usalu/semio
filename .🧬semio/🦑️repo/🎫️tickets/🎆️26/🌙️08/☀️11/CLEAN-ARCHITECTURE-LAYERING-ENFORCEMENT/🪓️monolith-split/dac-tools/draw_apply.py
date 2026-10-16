"""✂️ Re-point the crate-private module aliases of the wgpu render crate to their real crates and delete the alias lines."""
import re,os,json,sys
os.chdir("/Users/ueli/Documents/semio")
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
reach=json.load(open(F+"/🗑️generated/dac-reach.json"))
APPLY="--apply" in sys.argv
MAP={"draw":"semio_framework_ui_wgpu_draw_execution","draw_types":"semio_framework_ui_draw_contract","prepared":"semio_framework_ui_prepared","shaders":"semio_framework_ui_render_prepared_shaders","theme":"semio_framework_ui_theme","raster_ownership":"semio_framework_ui_raster_owner"}
files=[f for f,v in reach.items() if any(o['lib']=='semio_framework_ui_wgpu_render' for o in v)]
root="🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs"
for f in sorted(files):
    t=open(f,encoding="utf8").read(); o=t
    for n,c in MAP.items():
        t=re.sub(r"(?<![A-Za-z0-9_])(?:crate|super)::"+n+r"(?![A-Za-z0-9_])",c,t)
        t=re.sub(r"^[ \t]*pub\(crate\) use "+c+r" as "+n+r";[ \t]*\n","",t,flags=re.M)
    if f==root:
        assert t.count("pub use draw::{")==1
        t=t.replace("pub use draw::{","pub use semio_framework_ui_wgpu_draw_execution::{")
    if t!=o:
        print("CHANGE",f[-60:],len(re.findall("semio_framework_ui_",t))-len(re.findall("semio_framework_ui_",o)))
        if APPLY:
            assert open(f,encoding="utf8").read()==o
            open(f,"w",encoding="utf8").write(t)
