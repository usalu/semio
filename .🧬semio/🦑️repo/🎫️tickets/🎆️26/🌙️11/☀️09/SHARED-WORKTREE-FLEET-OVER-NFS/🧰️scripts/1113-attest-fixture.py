#!/usr/bin/env python3
"""Writes the attest fixture. Digests come from hashlib, verdicts are written by hand below (not computed by the code under test)."""
import hashlib,json,os,sys
OUT=sys.argv[1]
def term(p,o,v): return hashlib.sha256((p+"\0"+o+"\0"+v).encode()).digest()
def fold(entries):
    d=bytearray(32)
    for e in entries:
        if e["operation"]=="delete" or e["path"]==".cache/f/index-base" or e["path"].startswith(".cache/f/index/"): continue
        t=term(e["path"],e["operation"],e["value"])
        for i in range(32): d[i]^=t[i]
    return d.hex()
S=lambda c:hashlib.sha256(c.encode()).hexdigest()
E=[{"path":"a.txt","operation":"put","value":S("a")},{"path":"dir/b.ts","operation":"put","value":S("b")},{"path":"l","operation":"link","value":"dir/b.ts"},{"path":"ü/日本.txt","operation":"put","value":S("u")}]
digests=[
 {"name":"empty","entries":[],"manifest":"0"*64},
 {"name":"four terms","entries":E,"manifest":fold(E)},
 {"name":"order does not matter","entries":list(reversed(E)),"manifest":fold(E)},
 {"name":"a delete contributes nothing","entries":E+[{"path":"gone","operation":"delete","value":""}],"manifest":fold(E)},
 {"name":"realm paths are excluded","entries":E+[{"path":".cache/f/index-base","operation":"link","value":"tree 1"},{"path":".cache/f/index/x.txt","operation":"link","value":"100644 2"}],"manifest":fold(E)},
 {"name":"a changed value changes the digest","entries":E[:3]+[{"path":"ü/日本.txt","operation":"put","value":S("v")}],"manifest":fold(E[:3]+[{"path":"ü/日本.txt","operation":"put","value":S("v")}])},
]
files=[{"path":"📒️journal/🟦️.ts","text":"export const journal=1;\n"},{"path":"🔁️replica/🟦️.ts","text":"export const replica=2;\n"},{"path":"🔁️replica/📜️script.ts","text":"#!/usr/bin/env repo\n"},{"path":"🩺️attest/🟦️.ts","text":"é\n"}]
ignored=[{"path":"🧪️tests/🟦️.ts","text":"x"},{"path":"🔁️replica/🧪️tests/🟦️.ts","text":"y"},{"path":"README.md","text":"z"},{"path":"🔁️replica/🐍️.py","text":"w"}]
def code(fs):
    h=hashlib.sha256()
    for f in sorted(fs,key=lambda f:f["path"].encode()):
        b=f["text"].encode()
        h.update(f["path"].encode()+b"\0"+str(len(b)).encode()+b"\0"+b)
    return h.hexdigest()
codes=[{"name":"sources only, sorted, tests and other files excluded","files":files+ignored,"code":code(files)},{"name":"one byte changes everything","files":files[:3]+[{"path":"🩺️attest/🟦️.ts","text":"e\n"}]+ignored,"code":code(files[:3]+[{"path":"🩺️attest/🟦️.ts","text":"e\n"}])}]
NOW=10_000_000
M1,M2="1"*64,"2"*64
C1,C2="a"*40,"b"*40
T1="c"*40
def att(o,platform="linux",age=10,sync=True,cursors=None,manifest=M1,commit=C1,tree=T1,incoming=0,code="d"*64):
    return {"version":1,"originId":o,"groupId":"g1","platform":platform,"wallMs":NOW-age*1000,"hlc":"%013d-000000-%s"%(NOW-age*1000,o),"sync":sync,"code":code,"cursors":cursors if cursors is not None else {"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":200}},"files":4,"manifest":manifest,"head":{"ref":"refs/heads/main","commit":commit} if commit else None,"indexTree":tree,"queue":{"incoming":incoming}}
H={"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":200}}
def row(o,verdict,platform="linux",age=10,behind=0,incoming=0,sync=True,fresh=True,settled=True,differs=None):
    return {"originId":o,"platform":platform,"sync":sync,"ageSeconds":age,"behindOrigins":behind,"incoming":incoming,"code":"d"*12,"fresh":fresh,"settled":settled,"verdict":verdict,"differs":differs or []}
def ret(o,by="a",retired=True,age=5,reason="left the fleet"):
    return {"version":1,"originId":o,"by":by,"hlc":"%013d-000000-%s"%(NOW-age*1000,by),"wallMs":NOW-age*1000,"reason":reason,"retired":retired}
def case(name,atts,heads,rows,differing,ok,rets=()):
    return {"name":name,"now":NOW,"heads":heads,"attestations":{a["originId"]:a for a in atts},"retirements":{r["originId"]:r for r in rets},"expected":{"version":1,"ok":ok,"freshSeconds":60,"rows":rows,"differing":differing}}
a,b,c=att("a"),att("b","darwin"),att("c","win32")
three=[row("a","ok"),row("b","ok",platform="darwin"),row("c","ok",platform="win32")]
cases=[case("three origins agree",[a,b,c],H,three,[],True)]
c2=att("c","win32",manifest=M2)
cases.append(case("one manifest differs and is named",[a,b,c2],H,three[:2]+[row("c","diverged",platform="win32",differs=["manifest"])],[{"originId":"c","field":"manifest","value":M2,"expected":M1}],False))
c3=att("c","win32",commit=C2)
cases.append(case("one commit differs and is named",[a,b,c3],H,three[:2]+[row("c","diverged",platform="win32",differs=["head.commit"])],[{"originId":"c","field":"head.commit","value":C2,"expected":C1}],False))
c4=att("c","win32",tree="e"*40)
cases.append(case("one index tree differs and is named",[a,b,c4],H,three[:2]+[row("c","diverged",platform="win32",differs=["indexTree"])],[{"originId":"c","field":"indexTree","value":"e"*40,"expected":T1}],False))
b5=att("b","darwin",age=61,manifest=M2)
cases.append(case("an attestation older than 60 s is stale and not compared",[a,b5,c],H,[row("a","ok"),row("b","stale",platform="darwin",age=61,fresh=False),row("c","ok",platform="win32")],[],False))
b6=att("b","darwin",age=60)
cases.append(case("exactly 60 s is still fresh",[a,b6,c],H,[row("a","ok"),row("b","ok",platform="darwin",age=60),row("c","ok",platform="win32")],[],True))
c7=att("c","win32",cursors={"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":150}},manifest=M2)
cases.append(case("a lagging origin is not settled and not compared",[a,b,c7],H,three[:2]+[row("c","lagging",platform="win32",behind=1,settled=False)],[],False))
c8=att("c","win32",cursors={"a":{"segment":0,"bytes":100}})
cases.append(case("a missing cursor counts as behind",[a,b,c8],H,three[:2]+[row("c","lagging",platform="win32",behind=1,settled=False)],[],False))
c9=att("c","win32",incoming=3)
cases.append(case("a queued incoming file is not settled",[a,b,c9],H,three[:2]+[row("c","lagging",platform="win32",incoming=3,settled=False)],[],False))
c10=att("c","win32",sync=False,manifest=M2)
cases.append(case("an origin with sync off is excluded",[a,b,c10],H,three[:2]+[row("c","off",platform="win32",sync=False)],[],True))
H2=dict(H);H2["d"]={"segment":0,"bytes":50}
a11,b11,c11=[att(o,p,cursors={"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":200},"d":{"segment":0,"bytes":50}}) for o,p in (("a","linux"),("b","darwin"),("c","win32"))]
cases.append(case("an origin with a head and no attestation is missing",[a11,b11,c11],H2,three+[{"originId":"d","platform":None,"sync":None,"ageSeconds":None,"behindOrigins":None,"incoming":None,"code":None,"fresh":False,"settled":False,"verdict":"missing","differs":[]}],[],False))
a12,b12=att("a",manifest=M2),att("b","darwin",manifest=M1)
cases.append(case("a tie expects the smaller value",[a12,b12],H,[row("a","diverged",differs=["manifest"]),row("b","ok",platform="darwin")],[{"originId":"a","field":"manifest","value":M2,"expected":M1}],False))
a13=att("a",commit=None,tree=None)
b13=att("b","darwin",commit=None,tree=None)
cases.append(case("no git anywhere agrees",[a13,b13],H,[row("a","ok"),row("b","ok",platform="darwin")],[],True))
cases.append(case("an empty store is not ok",[],{},[],[],False))
Q={"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":200}}
d14=att("d","darwin",age=9000,cursors=Q,manifest=M2)
d14row=row("d","retired",platform="darwin",age=9000,fresh=False)
cases.append(case("a retired silent origin is excluded and the fleet is ok",[a,b,c,d14],H2,three+[d14row],[],True,[ret("d")]))
cases.append(case("a retired origin without attestation is retired and not missing",[a,b,c],H2,three+[{"originId":"d","platform":None,"sync":None,"ageSeconds":None,"behindOrigins":None,"incoming":None,"code":None,"fresh":False,"settled":False,"verdict":"retired","differs":[]}],[],True,[ret("d")]))
Qd={"a":{"segment":0,"bytes":100},"b":{"segment":0,"bytes":200},"d":{"segment":0,"bytes":50}}
d15=att("d","darwin",age=9000,cursors=Qd)
cases.append(case("an undone retirement counts the origin again",[att("a",cursors=Qd),att("b","darwin",cursors=Qd),att("c","win32",cursors=Qd),d15],H2,three+[row("d","stale",platform="darwin",age=9000,fresh=False)],[],False,[ret("d",retired=False)]))
d17=att("d","darwin",cursors=Q,manifest=M2)
cases.append(case("a retired origin that still attests is not compared",[a,b,c,d17],H2,three+[row("d","retired",platform="darwin")],[],True,[ret("d")]))
cases.append(case("a retirement record alone makes a retired row",[a,b,c],H,three+[{"originId":"e","platform":None,"sync":None,"ageSeconds":None,"behindOrigins":None,"incoming":None,"code":None,"fresh":False,"settled":False,"verdict":"retired","differs":[]}],[],True,[ret("e","a")]))
cases.append(case("only retired origins is not ok",[d14],{"d":{"segment":0,"bytes":50}},[row("d","retired",platform="darwin",age=9000,fresh=False)],[],False,[ret("d")]))
json.dump({"version":1,"digests":digests,"code":codes,"verdicts":cases},open(OUT,"w",encoding="utf8"),ensure_ascii=False,indent=1)
