import sys,os
R=sys.argv[1];F=R+"/🧬️schema/🔣️.json";TF=R+"/🧪️tests/🧱️source-transient/🟦️.ts"
t=open(F,encoding="utf8").read()
old='"unexpected failure \\\\(reference [0-9a-f]+\\\\): (?:\\\\w+: )?(?:Closure index|Retiring closure leaf|Group retirement leaf|Terminal retirement intent) (?:append-only )?file (?:changed while it was open|was replaced)"'
assert t.count(old)==1
new='"file changed while it was open / [^\\\\n]*Datei hat sich geändert, während sie offen war", "append-only file was replaced, linked again or shortened while it was open / [^\\\\n]*anhängbare Datei wurde ersetzt", "file was replaced again while it was opened by name / [^\\\\n]*Datei wurde beim Öffnen über den Namen erneut ersetzt"'
t=t.replace(old,new)
tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F)
s=open(TF,encoding="utf8").read()
anchor=' {lines:["The resident service met an unexpected failure (reference 92b15f60): Error: Closure index is required"],root,want:false},\n'
assert s.count(anchor)==1
add=''.join(' {lines:["%s"],root,want:%s},\n'%(l,w) for l,w in [
 ("The resident service met an unexpected failure (reference 92b15f61): ConcurrentEditErrorV1: Cache charge receipt file changed while it was open / Cache-Belastungsbeleg Datei hat sich geändert, während sie offen war: /x/y.json","true"),
 ("The resident service met an unexpected failure (reference 92b15f62): ConcurrentEditErrorV1: Retiring closure leaf file was replaced again while it was opened by name / Abzuräumendes Abschlussblatt Datei wurde beim Öffnen über den Namen erneut ersetzt: /x/y.json","true"),
 ("The file changed while it was open in another editor","false"),
])
s=s.replace(anchor,anchor+add)
tmp=TF+".f75tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,TF);print("ok")
