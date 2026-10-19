import os, shutil
ROOT="/Users/ueli/Documents/semio"
S=ROOT+"/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
BK=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rg-fem3d/backup"
EXT="extern crate semio_framework_os_kernel as protocol;\n"
VCS="extern crate semio_framework_os_kernel as vcs;\n"
def go(sub, root, leaf, mod):
    f=S+"/"+sub+"/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/"+root+"/🦀️.rs"
    t=open(f,encoding="utf8").read()
    assert t.count(EXT)==1 and VCS not in t, f
    bk=BK+"/"+f.replace(ROOT+"/","").replace("/","__")
    if not os.path.exists(bk): shutil.copy2(f,bk)
    t=t.replace(EXT,EXT+VCS)
    if not t.endswith("\n"): t+="\n"
    t+='#[path = "../../🧬️schema/🧬️mutations/%s/🧪️tests/🔬️unit/🦀️.rs"]\nmod %s;\n'%(leaf,mod)
    open(f,"w",encoding="utf8").write(t); print("mounted",mod)
go("🧊️3d","🔬️section-kinds","🩹️change-section","change_section_field_intent")
go("🧊️3d","🔬️solid-kinds","🩹️change-solid","change_solid_field_intent")
go("◻️2d","🔬️section-kinds","🩹️change-section","change_section_field_intent")
go("◻️2d","🔬️region-kinds","🩹️change-region","change_region_field_intent")
