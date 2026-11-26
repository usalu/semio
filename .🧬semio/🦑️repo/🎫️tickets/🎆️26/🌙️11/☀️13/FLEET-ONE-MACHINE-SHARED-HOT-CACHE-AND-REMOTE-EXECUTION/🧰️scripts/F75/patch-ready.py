import sys,os
R=sys.argv[1]
F=R+"/🧬️schema/🔣️.json";TF=R+"/🧪️tests/🧱️source-transient/🟦️.ts"
t=open(F,encoding="utf8").read()
tail='"unexpected failure \\\\(reference [0-9a-f]+\\\\): Error: Slice worker exited with code null"]'
assert t.count(tail)==1
new=tail[:-1]+', "unexpected failure \\\\(reference [0-9a-f]+\\\\): (?:\\\\w+: )?(?:Closure index descriptor changed|Closure proof descriptor changed|Original retiring closure (?:descriptor|identity) changed|Original group retirement descriptor changed|Original terminal retirement intent changed)", "unexpected failure \\\\(reference [0-9a-f]+\\\\): (?:\\\\w+: )?(?:Closure index|Retiring closure leaf|Group retirement leaf|Terminal retirement intent) (?:append-only )?file (?:changed while it was open|was replaced)", "Closure proof (?:leaf|authority|pack index) descriptor stamp mismatch"]'
t=t.replace(tail,new)
tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F)
s=open(TF,encoding="utf8").read()
anchor=' {lines:["The resident service met an unexpected failure (reference 1): Error: Slice worker exited with code 1: out of memory"],root,want:false},\n'
assert s.count(anchor)==1
add=''.join(' {lines:["%s"],root,want:%s},\n'%(l,w) for l,w in [
 ("The resident service met an unexpected failure (reference 92b15f5b): Error: Closure index descriptor changed","true"),
 ("The resident service met an unexpected failure (reference 92b15f5c): Error: Closure proof descriptor changed","true"),
 ("The resident service met an unexpected failure (reference 92b15f5d): Error: Original group retirement descriptor changed","true"),
 ("The resident service met an unexpected failure (reference 92b15f5e): ConcurrentEditErrorV1: Closure index file changed while it was open / Abschlussindex Datei hat sich geändert, während sie offen war: /x/y.idx","true"),
 ("The resident service met an unexpected failure (reference 92b15f5f): ConcurrentEditErrorV1: Closure index append-only file was replaced, linked again or shortened while it was open / Abschlussindex anhängbare Datei wurde ersetzt, neu verlinkt oder gekürzt, während sie offen war: /x/y.idx","true"),
 ("Slice worker failed: Closure proof leaf descriptor stamp mismatch (ino)","true"),
 ("The resident service met an unexpected failure (reference 92b15f60): Error: Closure index is required","false"),
])
s=s.replace(anchor,anchor+add)
tmp=TF+".f75tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,TF);print("ok")
