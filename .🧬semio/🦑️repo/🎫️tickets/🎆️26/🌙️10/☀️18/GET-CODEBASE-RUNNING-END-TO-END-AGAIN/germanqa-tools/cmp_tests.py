import json,sys
b=json.load(open("🗑️generated/germanqa/tests-base.json",encoding="utf-8")); a=json.load(open("🗑️generated/germanqa/tests-after.json",encoding="utf-8"))
new=0;fixed=0;same=0
for d,r in a.items():
    x=b.get(d)
    if x is None: continue
    fa=set(r.get("fails",[])); fb=set(x.get("fails",[]))
    ca=r.get("counts",{}); cb=x.get("counts",{})
    if fa!=fb or ca!=cb or r.get("exit")!=x.get("exit"):
        print("DIFF",d[-70:],"base",cb,x.get("exit"),"after",ca,r.get("exit"));
        for f in sorted(fa-fb): print("   NEW FAIL:",f[:130]); new+=1
        for f in sorted(fb-fa): print("   FIXED:",f[:130]); fixed+=1
    else: same+=1
print("dirs compared",len(a),"identical",same,"new fails",new,"fixed",fixed)
