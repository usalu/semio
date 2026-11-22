# coding: utf-8
import json,sys
G=sys.argv[1]+"/"
res={}
for f in ("raw3a.jsonl","raw3b.jsonl"):
    for l in open(G+f):
        j=json.loads(l); res[j["file"]]=j["rc"]
known=set(x.strip() for n in ("sf-list1.txt","sf-list2.txt","sf-list3.txt","sf-list3b.txt","sf-list4.txt") for x in open(G+n))
new=[f for f,rc in res.items() if rc!=0 and f not in known]
open(G+"sf-list5.txt","w").write("\n".join(new)+"\n"); print(len(new))
