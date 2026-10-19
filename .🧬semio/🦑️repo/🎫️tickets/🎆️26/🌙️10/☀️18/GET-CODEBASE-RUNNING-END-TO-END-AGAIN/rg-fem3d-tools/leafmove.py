import os,re,shutil,sys
ROOT="/Users/ueli/Documents/semio"
S=ROOT+"/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
BK=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rg-fem3d/backup"
BLOCK='\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
EXT="extern crate semio_framework_os_kernel as protocol;\n"
VCS="extern crate semio_framework_os_kernel as vcs;\n"
DRY=len(sys.argv)>1 and sys.argv[1]=="dry"
def bk(f):
    b=BK+"/"+f.replace(ROOT+"/","").replace("/","__")
    if not os.path.exists(b): shutil.copy2(f,b)
PLAN=[("🏋️load","🩹️change-combination-term","🔬️combination-kinds","change_combination_term_field_intent"),
("🏋️load","🩹️change-combination-name","🔬️combination-kinds","change_combination_name_field_intent"),
("🏋️load","🩹️change-load","🔬️load-kinds","change_load_field_intent"),
("🛡️boundary","🩹️change-support","🔬️support-kinds","change_support_field_intent"),
("🧱️material","🩹️change-material","🔬️material-kinds","change_material_field_intent")]
for a,dim in [("🧊️3d","3d"),("◻️2d","2d")]:
    for sub,leaf,root,mod in PLAN:
        D=S+"/"+a+"/🏅️standards/🔖️1/🪆️subsets/"+sub
        lf=D+"/🧬️schema/🧬️mutations/"+leaf+"/🦀️.rs"
        tf=D+"/🧬️schema/🧬️mutations/"+leaf+"/🧪️tests/🔬️unit/🦀️.rs"
        t=open(lf,encoding="utf8").read(); tt=open(tf,encoding="utf8").read()
        assert t.endswith(BLOCK) and t.count(BLOCK)==1,lf
        bad=[m for m in ("super::","crate::") if m in tt]
        print(dim,leaf,"refs",bad,"serde_json" in tt)
        if bad: sys.exit("abort "+tf)
        rf=D+"/🧪️tests/"+root+"/🦀️.rs"
        r=open(rf,encoding="utf8").read()
        if not DRY:
            bk(lf); open(lf,"w",encoding="utf8").write(t[:-len(BLOCK)])
            assert r.count(EXT)==1
            if VCS not in r: r=r.replace(EXT,EXT+VCS)
            if not r.endswith("\n"): r+="\n"
            r+='#[path = "../../🧬️schema/🧬️mutations/%s/🧪️tests/🔬️unit/🦀️.rs"]\nmod %s;\n'%(leaf,mod)
            bk(rf); open(rf,"w",encoding="utf8").write(r)
