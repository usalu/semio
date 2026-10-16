import collections,re,os
F=os.environ["F"]; R="/Users/ueli/Documents/semio"
FE="️"
tests="\U0001F9EA"+FE+"tests/"; fx="\U0001F9EB"+FE+"fixtures"
NEUTRAL=[("s.gis.gismap","s.demo.demo"),('"GIS Map"','"Demo Map"'),('"GIS-Karte"','"Demo-Karte"'),("gis.map.operation","demo.map.operation"),("s.cad.cad","s.demo.demo"),('pluginId: "cad"','pluginId: "demo"'),("s.note.note@1","s.demo.demo@1"),("noteShellCommand","demoShellCommand"),('"semio:gis"','"semio:demo"'),('profile: "wasm-release"','profile: "release"')]
def norm(l, neutral):
    l=l.strip()
    if neutral:
        for a,b in NEUTRAL: l=l.replace(a,b)
    # drop fixture tests insertion
    l=l.replace(tests+fx,fx)
    # administration moves
    l=l.replace("\U0001F9EC"+FE+"schema/\U0001F3DB"+FE+"administration/","\U0001F3DB"+FE+"administration/")
    # strip relative path prefixes
    l=re.sub(r'(\.\./)+','',l); l=re.sub(r'(?<=["`])\./','',l)
    return l
orig=[norm(l,True) for l in open(F+"/met-tools/gis-original.ts.txt",encoding="utf8") if l.strip()]
files=[R+"/"+l.strip() for l in open(F+"/met-tools/gis-new-files.txt",encoding="utf8") if l.strip()]
new=[]
for f in files: new+=[norm(l,False) for l in open(f,encoding="utf8") if l.strip()]
co=collections.Counter(orig); cn=collections.Counter(new)
lost=co-cn; added=cn-co
print("lost",sum(lost.values()))
for k,v in sorted(lost.items(), key=lambda kv:-kv[1]): print("  -",v,k[:200])
