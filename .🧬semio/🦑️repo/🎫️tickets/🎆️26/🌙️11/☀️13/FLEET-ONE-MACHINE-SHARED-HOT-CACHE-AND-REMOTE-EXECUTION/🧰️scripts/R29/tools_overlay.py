import json,os,re,sys,posixpath
LIVE="/Users/Shared/semio/"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PC=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/"
PROVDIR=LIB+"🏃️process/🔌️providers"
SKIP={"typescript","packages","process","modules","library","framework","products","repo","tests","testing-framework"}
def res_json(items,indent):
    if not items:return "[]"
    pad=" "*indent
    return "[\n"+",\n".join(pad+"  "+json.dumps(i,ensure_ascii=False) for i in items)+"\n"+pad+"]"
def slug(path,taken):
    parts=[re.sub(r'[^a-z0-9-]','',p.lower()) for p in path.split('/')[:-1]]
    parts=[p for p in parts if p and p not in ("typescript","packages","process","modules","library","framework","products","repo")]
    for k in range(min(2,len(parts)),len(parts)+1):
        s="-".join(parts[-k:])
        if s and s not in taken:return s
    raw=[re.sub(r'[^a-z0-9-]','',p.lower()) for p in path.split('/')[:-1]];raw=[p for p in raw if p]
    for k in range(2,len(raw)+1):
        s="-".join(raw[-k:])
        if s not in taken:return s
    raise SystemExit("no slug "+path)
def entries_json(entry,indent):
    return "[]" if entry is None else '[\n'+" "*(indent+2)+json.dumps(entry,ensure_ascii=False)+'\n'+" "*indent+']'
def block(name,entry,en,de,view="scheduling",resources=()):
    return '''      "%s": {
        "entries": %s,
        "resources": %s,
        "view": "%s",
        "selection": "included",
        "contract": {
          "en": "%s",
          "de": "%s"
        }
      },
'''%(name,entries_json(entry,8),res_json(resources,8),view,en,de)
def corpus_block(name,entry,resources=()):
    return '''    "%s": {
      "entries": %s,
      "resources": %s
    },
'''%(name,entries_json(None if entry is None else posixpath.normpath(posixpath.join("lib/providers",entry)),6),res_json([posixpath.normpath(posixpath.join("lib/providers",r)) for r in resources],6))
def make(tools,out):
    """tools: list of dict(name,path,en,de[,view])"""
    pol=open(LIVE+PC+"🔣️.json",encoding="utf8").read()
    cor=open(LIVE+PC+"🧪️tests/🔣️.json",encoding="utf8").read()
    pb="";cb=""
    for t in sorted(tools,key=lambda t:t["name"]):
        entry=None if t.get("path") is None else os.path.relpath(LIVE+t["path"],LIVE+PROVDIR)
        res=[os.path.relpath(LIVE+r,LIVE+PROVDIR) for r in t.get("resources",[])]
        pb+=block(t["name"],entry,t["en"],t["de"],t.get("view","scheduling"),res);cb+=corpus_block(t["name"],entry,res)
    auto={}
    for t in tools:
        for rel in ([] if t.get("path") is None else [os.path.relpath(LIVE+t["path"],LIVE+PROVDIR)])+[os.path.relpath(LIVE+r,LIVE+PROVDIR) for r in t.get("resources",[])]:
            inner=posixpath.normpath(posixpath.join("lib/providers",rel))
            if not inner.startswith(".."):auto.setdefault(inner,"export const fixture = 1;\n")
    for t in tools:
        auto.update(t.get("fixtureFiles",{}))
    fx="".join('    %s: %s,\n'%(json.dumps(k,ensure_ascii=False),json.dumps(v,ensure_ascii=False)) for k,v in sorted(auto.items()))
    a='      "cargo-lock-owner": {\n'
    assert pol.count(a)==1
    pol=pol.replace(a,pb+a)
    b='    "cargo-lock-owner": {\n      "entries": ['
    assert cor.count(b)==1
    cor=cor.replace(b,cb+b)
    if fx:
        anchor='  "files": {\n'
        assert cor.count(anchor)==1
        cor=cor.replace(anchor,anchor+fx)
    sch=open(LIVE+PC+"🧬️schema/🔣️.json",encoding="utf8").read()
    assert sch.count('"maxProperties": 16')==1
    sch=sch.replace('"maxProperties": 16','"maxProperties": 48')
    old='        "entries": {\n          "type": "array",\n          "minItems": 1,'
    assert sch.count(old)==1
    sch=sch.replace(old,'        "entries": {\n          "type": "array",\n          "minItems": 0,')
    for rel,text in ((PC+"🔣️.json",pol),(PC+"🧪️tests/🔣️.json",cor),(PC+"🧬️schema/🔣️.json",sch)):
        d=out+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);open(d,"w",encoding="utf8").write(text)
    json.loads(pol);json.loads(cor)
if __name__=="__main__":
    make(json.load(open(sys.argv[1])),sys.argv[2]);print("overlay tools",len(json.load(open(sys.argv[1]))))
