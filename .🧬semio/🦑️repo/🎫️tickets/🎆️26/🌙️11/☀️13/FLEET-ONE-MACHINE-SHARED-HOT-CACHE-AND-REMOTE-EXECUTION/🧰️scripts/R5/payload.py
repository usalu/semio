#!/usr/bin/env python3
"""R5 payload: derives the new scheduling domains (ports from the real import graph) and writes edits.json (anchored whole-region replacements) and baseline.json."""
import json,os,re,sys,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import ports as P,domains as D
R=P.R;LIB=P.LIB
SCHED=LIB+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
POLICY=LIB+"🏗️build/🔣️policy.json"
NX="nx.json"
u16=lambda s:s.encode("utf-16-be")
def build():
    files=P.tree();g=P.graph(files)
    raw=open(R+SCHED,encoding="utf8").read();table=json.loads(raw)
    have={d["id"] for d in table["domains"]}
    new=[]
    for did,prefixes,en,de in D.DOMAINS:
        if did in have:continue
        pr=sorted(prefixes,key=u16)
        ports=sorted(P.LIB+p for p in P.ports([x[len(LIB):] for x in pr],g).keys()) if False else sorted((LIB+p for p in P.ports([x[len(LIB):] for x in pr],g)),key=u16)
        new.append({"id":did,"contract":{"en":en,"de":de},"prefixes":pr,"ports":ports})
    return table,raw,new
def render(table):return json.dumps(table,indent=2,ensure_ascii=False)+"\n"
def region(text,name):
    m=re.search(r'"'+name+r'": \[\n.*?\n\s*\]',text,re.S);return m.group(0)
def main():
    table,raw,new=build()
    after=json.loads(raw);after["domains"]=after["domains"]+new
    out=render(after)
    assert render(table)==raw
    old_tail=json.dumps({"domains":table["domains"][-1:]},indent=2,ensure_ascii=False)
    edits=[];base={}
    edits.append([SCHED,raw,out,1]);base[SCHED]=hashlib.sha256(raw.encode()).hexdigest()
    prefixes=sorted({p for d in after["domains"] for p in d["prefixes"]},key=u16)
    pats=["!{workspaceRoot}/"+p+"**/*" for p in prefixes]
    for rel in (NX,POLICY):
        text=open(R+rel,encoding="utf8").read();old=region(text,"schedulingSourceExclusions")
        indent=re.search(r'\n(\s*)\]$',old).group(1)
        body=",\n".join(indent+"  "+json.dumps(p,ensure_ascii=False) for p in pats)
        newr='"schedulingSourceExclusions": [\n'+body+"\n"+indent+"]"
        edits.append([rel,old,newr,1]);base[rel]=hashlib.sha256(text.encode()).hexdigest()
    sys.path.insert(0,os.path.dirname(HERE)+"/R5c")
    import edits_si,edits_policy
    for mod in (edits_si,edits_policy):
        for rel,old,nw,count in mod.EDITS:
            edits.append([rel,old,nw,count])
            if rel not in base:base[rel]=hashlib.sha256(open(R+rel,'rb').read()).hexdigest()
    json.dump(edits,open(HERE+"/edits.json","w",encoding="utf8"),ensure_ascii=False)
    json.dump(base,open(HERE+"/baseline.json","w"),indent=1)
    print("domains",len(table["domains"]),"+",len(new),"prefixes",len(prefixes),"ports",[len(d["ports"]) for d in new])
main()
