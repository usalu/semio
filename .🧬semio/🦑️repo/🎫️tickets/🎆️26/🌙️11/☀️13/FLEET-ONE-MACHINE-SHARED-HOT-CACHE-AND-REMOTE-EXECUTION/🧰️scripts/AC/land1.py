import os,sys,hashlib,subprocess,tempfile,importlib.util
W=os.path.dirname(os.path.abspath(__file__))
LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
dry=len(sys.argv)>1 and sys.argv[1]=="dry"
def sha(b): return hashlib.sha256(b).hexdigest()[:12]
plan=[]
def read(p): return open(p,encoding="utf8").read()
def anchored(text,pairs):
    for old,new in pairs:
        assert text.count(old)==1,(text.count(old),old[:80])
        text=text.replace(old,new)
    return text
# cursor.ts
ns={}
src=read(W+"/edit_cursor.py")
def run_edit(script,live_rel):
    t=read(LIB+live_rel)
    tmp=tempfile.NamedTemporaryFile("w",delete=False,suffix=".ts",encoding="utf8");tmp.write(t);tmp.close()
    out=tmp.name+".out"
    subprocess.run([sys.executable,W+"/"+script,tmp.name,out],check=True)
    return read(out)
files={}
files["🪪️graph-state/📍️cursor/🟦️.ts"]=run_edit("edit_cursor.py","🪪️graph-state/📍️cursor/🟦️.ts")
schema=read(LIB+"🪪️graph-state/📍️cursor/🧬️schema/🔣️.json")
schema=anchored(schema,[
('    { "$ref": "#/definitions/TextCatalogue" }\n  ],','    { "$ref": "#/definitions/VerdictsHeader" },\n    { "$ref": "#/definitions/TextCatalogue" }\n  ],'),
('    "TextCatalogue": {','''    "VerdictPolicy": {
      "const": {
        "version": 1,
        "header": "semio-graph-verdicts\\t1",
        "extension": ".verdicts.br",
        "keyLength": 32,
        "kept": 2,
        "saveMinimum": 256,
        "maximumEntries": 4194304,
        "maximumBytes": 268435456,
        "brotliQuality": 5,
        "brotliWindow": 22
      }
    },
    "VerdictsHeader": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "key"],
      "properties": {
        "version": { "const": 1 },
        "key": { "type": "string", "pattern": "^[0-9a-f]{32}$", "description": "The first 128 bits of the SHA-256 over the header, the text of the ignore rules and the digest of the code that judges a path." }
      },
      "description": "The stored document is the brotli compressed text: the header line `semio-graph-verdicts<TAB>1`, the line `key<TAB><key>`, then one line per judged path: `1` (the shared snapshot cannot vouch for it) or `0` (it can) directly followed by the repository relative path. A document of another key, a damaged one or one past its limits is an empty memory."
    },
    "TextCatalogue": {''')])
files["🪪️graph-state/📍️cursor/🧬️schema/🔣️.json"]=schema
test=read(LIB+"🪪️graph-state/📍️cursor/🧪️tests/🟦️.ts")
add=read(W+"/verdict.test.add.ts")
test=anchored(test,[
('import schema from "../🧬️schema/🔣️.json" with {type:"json"};','import {createVerdictMemoV1,GRAPH_VERDICT_POLICY_V1,verdictCodeDigestV1,verdictKeyV1} from "../🧠️verdicts/🟦️.ts";\nimport schema from "../🧬️schema/🔣️.json" with {type:"json"};'),
('import {tmpdir} from "node:os";','import {brotliCompressSync,brotliDecompressSync} from "node:zlib";\nimport {tmpdir} from "node:os";')])
test=test.rstrip("\n")+"\n"+add
files["🪪️graph-state/📍️cursor/🧪️tests/🟦️.ts"]=test
files["🪪️graph-state/📍️cursor/🧠️verdicts/🟦️.ts"]=read(W+"/verdicts/memo.ts")
# spans via edit_spans pairs
sp={"__file__":W+"/edit_spans.py"}
exec(read(W+"/edit_spans.py"),sp)
files["🕸️graph/🕰️generations/🟨️.mjs"]=anchored(read(LIB+"🕸️graph/🕰️generations/🟨️.mjs"),sp["gens"])
files["🕸️graph/🕰️host/🟦️.ts"]=anchored(read(LIB+"🕸️graph/🕰️host/🟦️.ts"),sp["host"])
files["🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts"]=anchored(read(LIB+"🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts"),sp["exch"])
# stamps
st=read(LIB+"🕸️graph/🕰️generations/🗂️directory-stamps/🟨️.mjs")
new=read(W+"/stamps.new.mjs");base=read(W+"/stamps.mjs")
assert st==base,"stamps base moved"
files["🕸️graph/🕰️generations/🗂️directory-stamps/🟨️.mjs"]=new
dt=read(LIB+"🕸️graph/🕰️generations/🧪️tests/🔎️doubt/🟦️.ts");assert dt==read(W+"/doubt.test.ts"),"doubt test base moved"
files["🕸️graph/🕰️generations/🧪️tests/🔎️doubt/🟦️.ts"]=read(W+"/doubt.test.new.ts")
ed={};exec(read(W+"/edit_doubt.py"),{"__name__":"x"} if False else ed)
files["🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs"]=ed["apply"](read(LIB+"🕸️graph/🕰️generations/🔎️doubt/🟨️.mjs"))
files["🕸️graph/🕰️generations/🌊️stream-patch/🟨️.mjs"]=anchored(read(LIB+"🕸️graph/🕰️generations/🌊️stream-patch/🟨️.mjs"),[(" const rankOf=text=>{let low=0,high=D.stringCount-1;while(low<=high){const mid=(low+high)>>>1,order=compareCodePointsV1(D.textAt(mid,false),text);"," const decoded=new Map(),textOf=rank=>{let text=decoded.get(rank);if(text===undefined){text=D.textAt(rank,false);if(decoded.size<65536)decoded.set(rank,text);}return text;};\n const rankOf=text=>{let low=0,high=D.stringCount-1;while(low<=high){const mid=(low+high)>>>1,order=compareCodePointsV1(textOf(mid),text);")])
pl=LIB+"🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";assert read(pl)==read(W+"/portable.ts"),"portable base moved"
files["🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts"]=read(W+"/portable.new.ts")
tl=LIB+"🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts";assert read(tl)==read(W+"/exch.test.ts"),"exchange test base moved"
files["🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts"]=read(W+"/exch.test.new.ts")
for rel,text in files.items():
    path=LIB+rel
    old=open(path,"rb").read() if os.path.exists(path) else b""
    print(("NEW " if not old else "EDIT"),rel,sha(old) if old else "-","->",sha(text.encode()))
    if dry: continue
    os.makedirs(os.path.dirname(path),exist_ok=True)
    tmp=path+".ac.tmp";open(tmp,"w",encoding="utf8").write(text);os.replace(tmp,path)
