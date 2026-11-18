import os,sys,difflib
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
OB=B+"/👁️observation"
SN=OB+"/📂️source-names"
P={}
P[f"{SN}/🟨️.mjs"]=[
('suffixes=rules.machineLocalFileSuffixes;','suffixes=rules.machineLocalFileSuffixes,local=new Set(rules.machineLocalDirectoryNames);'),
('!(generated.has(name)&&isDirectory(entry))&&!(isHollow','!((generated.has(name)||local.has(name))&&isDirectory(entry))&&!(isHollow'),
('a generated directory (the cache policy list) never where `isDirectory` says the entry is a directory or a link,','a generated directory (the cache policy list) or a machine-local one (`machineLocalDirectoryNames`: git ignores it, a generator fills it on the machines that ran it) never where `isDirectory` says the entry is a directory or a link,'),
]
N=B+"/🕸️graph/📥️inference/📦️project-nodes"
P[f"{N}/🟨️.mjs"]=[
('const sign=()=>Object.values(fleetPort?.producers?.()??{}).reduce((sum,each)=>sum+each,0)+names.filter(name=>taken.has(name)).length;','const sign=()=>Object.keys(fleetPort?.producers?.()??{}).length+names.filter(name=>taken.has(name)).length;'),
]
P[f"{SN}/🧬️schema/🔣️.json"]=[
('''        "machineLocalFileNames": [''','''        "machineLocalDirectoryNames": ["🤖️generated"],
        "machineLocalFileNames": ['''),
]
NEW=[f"{SN}/🧪️tests/🤖️machine-local-dirs/🟦️.ts",f"{N}/🧪️tests/🫥️self-progress/🟦️.ts"]
def patched(live_root,rel):
    t=open(f"{live_root}/{rel}",encoding="utf8").read()
    for old,new in P[rel]:
        assert t.count(old)==1,(rel,old[:70],t.count(old))
        t=t.replace(old,new,1)
    return t
if __name__=="__main__":
    L="/Users/ueli/Documents/semio";S=sys.argv[1]
    for rel in P:
        mine=open(f"{S}/mirror/{rel}",encoding="utf8").read()
        print("EQUAL" if mine==patched(L,rel) else "DIFF",rel[-50:])
