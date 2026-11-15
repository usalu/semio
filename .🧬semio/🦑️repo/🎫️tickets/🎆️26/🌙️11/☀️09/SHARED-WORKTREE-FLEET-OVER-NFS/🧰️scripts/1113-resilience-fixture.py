#!/usr/bin/env python3
"""Generates the language-agnostic fixture of the resilience member with hashlib only; expected values are computed here, not by the TypeScript code.
usage: 1113-resilience-fixture.py <fixture.json>"""
import hashlib,json,sys

def jitter(origin,digest,span):
    if span<=0: return 0
    return int.from_bytes(hashlib.sha256((origin+"\0"+digest).encode()).digest()[:4],"big")%span

D=[hashlib.sha256(str(i).encode()).hexdigest() for i in range(6)]
cases=[("g1-m01",D[0],60000),("g1-m02",D[0],60000),("g1-m03",D[0],60000),("g1-m01",D[1],60000),("g2-server",D[2],60000),("a",D[3],1),("a",D[3],0),("zz",D[4],1000),("g1-m10",D[5],59999),("g1-m11",D[5],60000)]
jit=[{"originId":o,"digest":d,"span":s,"expected":jitter(o,d,s)} for o,d,s in cases]

closure=[
 {"name":"module sources only, no imports","files":[{"path":"mod/a/🟦️.ts","text":"export const a=1;\n"},{"path":"mod/b/📜️script.ts","text":"#!/usr/bin/env repo\n"},{"path":"mod/🧪️tests/🟦️.ts","text":"import \"../a/🟦️.ts\";"}],"entries":["a/🟦️.ts","b/📜️script.ts"],"expected":[]},
 {"name":"outside imports, json and transitive files are followed once","files":[
   {"path":"mod/a/🟦️.ts","text":"import {x} from \"../../shared/x.ts\";\nimport data from \"./data.json\" with {type:\"json\"};\nimport {b} from \"../b/🟦️.ts\";\nexport const a=x+b;\n"},
   {"path":"mod/b/🟦️.ts","text":"export {y} from \"../../shared/y.ts\";\nexport const b=1;\n"},
   {"path":"mod/a/data.json","text":"{\"from\":\"./ignored.ts\"}"},
   {"path":"shared/x.ts","text":"import \"./side.ts\";\nexport const x=await import(\"./dyn.ts\");\n"},
   {"path":"shared/y.ts","text":"import {x} from \"./x.ts\";\nexport const y=x;\n"},
   {"path":"shared/side.ts","text":"export {};\n"},
   {"path":"shared/dyn.ts","text":"import \"node:fs\";\nimport \"bun:test\";\nimport {z} from \"left-pad\";\n"}],
  "entries":["a/🟦️.ts","b/🟦️.ts"],"expected":["../shared/dyn.ts","../shared/side.ts","../shared/x.ts","../shared/y.ts","a/data.json"]},
 {"name":"a specifier that does not exist and text inside strings are ignored","files":[
   {"path":"mod/a/🟦️.ts","text":"import {q} from \"./missing.ts\";\n/** says from \"./never.ts\" in prose */\nconst t=\"import './string.ts'\";\n"},
   {"path":"mod/a/string.ts","text":"export {};\n"}],
  "entries":["a/🟦️.ts"],"expected":["a/string.ts"]}
]

def problem(c):
    if not isinstance(c,dict): return "catalogue must be an object"
    langs=c.get("languages"); msgs=c.get("messages")
    if not isinstance(langs,list) or not langs or any((not isinstance(x,str)) or x=="" for x in langs): return "languages must be a non-empty list of names"
    if len(set(langs))!=len(langs): return "languages must be unique"
    if not isinstance(msgs,dict): return "messages must be an object"
    import re
    for key,m in msgs.items():
        if not isinstance(m,dict): return "message %s must be an object"%key
        if len(m)!=len(langs) or any(l not in m for l in langs): return "message %s must have exactly the languages %s"%(key,",".join(langs))
        ref=None
        for l in langs:
            t=m[l]
            if not isinstance(t,str) or t=="": return "message %s has no text in %s"%(key,l)
            names=",".join(sorted(set(re.findall(r"\{([a-z0-9]+)\}",t))))
            if ref is None: ref=names
            elif ref!=names: return "message %s differs in its placeholders between the languages"%key
    return None

texts=[
 ("complete catalogue",{"version":1,"languages":["en","de"],"messages":{"a":{"en":"Hello {name}","de":"Hallo {name}"},"b":{"en":"Plain","de":"Schlicht"}}}),
 ("a missing language",{"version":1,"languages":["en","de"],"messages":{"a":{"en":"Hello"}}}),
 ("an empty text",{"version":1,"languages":["en","de"],"messages":{"a":{"en":"Hello","de":""}}}),
 ("placeholders that differ",{"version":1,"languages":["en","de"],"messages":{"a":{"en":"Hello {name}","de":"Hallo {wer}"}}}),
 ("an extra language",{"version":1,"languages":["en","de"],"messages":{"a":{"en":"Hello","de":"Hallo","fr":"Salut"}}}),
 ("duplicate languages",{"version":1,"languages":["en","en"],"messages":{}}),
 ("no messages object",{"version":1,"languages":["en"],"messages":[]}),
]
tx=[{"name":n,"catalogue":c,"error":problem(c)} for n,c in texts]

def prune(created,keep):
    order=sorted(created.items(),key=lambda kv:(-kv[1],kv[0]))
    return [k for k,_ in order[:keep]]
pr=[]
for name,created,keep in [("keeps the newest two",{"a":5,"b":9,"c":7},2),("ties break by digest",{"b":4,"a":4,"c":3},2),("keep zero keeps nothing",{"a":1},0),("fewer than keep",{"a":1},2)]:
    pr.append({"name":name,"keep":keep,"created":created,"kept":prune(created,keep)})

def rotate(text,mx,kb):
    b=text.encode()
    if len(b)<=mx: return False,""
    return True,b[-kb:].decode("utf8","replace")
rot=[]
for name,text,mx,kb in [("small stays",  "abc\n",10,4),("large rotates and keeps the tail","0123456789\nabcdefghij\n",10,6),("exactly at the limit stays","0123456789",10,4)]:
    r,t=rotate(text,mx,kb); rot.append({"name":name,"text":text,"maxBytes":mx,"keepBytes":kb,"rotated":r,"tail":t})

json.dump({"version":1,"jitter":jit,"closure":closure,"texts":tx,"prune":pr,"rotate":rot},open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)
