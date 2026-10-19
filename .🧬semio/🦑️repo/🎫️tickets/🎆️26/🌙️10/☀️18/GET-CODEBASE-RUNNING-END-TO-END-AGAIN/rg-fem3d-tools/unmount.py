import sys, os, shutil
ROOT="/Users/ueli/Documents/semio"
S=ROOT+"/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
BK=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rg-fem3d/backup"
BLOCK='\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
M3=S+"/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/"
M2=S+"/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/"
files=[M3+"🩹️change-section/🦀️.rs",M3+"🩹️change-solid/🦀️.rs",M2+"🩹️change-section/🦀️.rs",M2+"🩹️change-region/🦀️.rs"]
for f in files:
    t=open(f,encoding="utf8").read()
    assert t.endswith(BLOCK) and t.count(BLOCK)==1, f
    bk=BK+"/"+f.replace(ROOT+"/","").replace("/","__")
    if not os.path.exists(bk): shutil.copy2(f,bk)
    open(f,"w",encoding="utf8").write(t[:-len(BLOCK)]+"\n" if False else t[:-len(BLOCK)])
    print("unmounted",f.replace(ROOT+"/","")[-80:])
