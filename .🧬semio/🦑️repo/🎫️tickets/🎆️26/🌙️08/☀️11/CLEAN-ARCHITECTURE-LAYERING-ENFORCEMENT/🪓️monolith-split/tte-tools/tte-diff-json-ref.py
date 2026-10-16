import re,os,time,shutil
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
os.makedirs(F+"/🗑️generated/tte-snap",exist_ok=True)
for d,exp in [("◻️2d",3),("🧊️3d",21),("🖐️5d",26)]:
    P=f"/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json"
    assert time.time()-os.stat(P).st_mtime>1800
    shutil.copy(P,F+f"/🗑️generated/tte-snap/puzzle{d[-2:]}-diff.json.pre")
    s=open(P,encoding="utf8").read()
    s2,k=re.subn(r'(\.json)/\$defs/',r'\1#/$defs/',s)
    assert k==exp,(d,k)
    open(P,"w",encoding="utf8").write(s2)
    print(d,k)
