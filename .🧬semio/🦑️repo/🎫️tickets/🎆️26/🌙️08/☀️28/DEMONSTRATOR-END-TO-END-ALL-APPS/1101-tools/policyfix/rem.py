import json,re,subprocess,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
subprocess.run(["python3",C+"/1101-tools/policyfix/classify.py"],stdout=subprocess.DEVNULL)
o=json.load(open(C+"/🗑️generated/1101/policyfix-classes.json"))
seen=set();cache={};out=open(C+"/🗑️generated/1101/policyfix-rem.txt","w");last=None
TESTISH=re.compile(r'(testkit|🧪|cold-laws|/tests?/)')
cnt=0
for f,n,k,nm,r in sorted(o,key=lambda x:(x[0],x[1])):
    if (f,n,k) in seen:continue
    seen.add((f,n,k))
    if TESTISH.search(f):continue
    if f!=last:out.write("## "+f.replace("🧰️framework/","FW/")+"\n");last=f
    L=cache.setdefault(f,open(f).read().split("\n"))
    l=L[n-1]
    for p in [m.start() for m in re.finditer(re.escape(k),l)][:4]:
        out.write(f"{n} [{nm}:{(r or '')[:34]}] ...{l[max(0,p-80):p+60]}\n");cnt+=1
print(cnt)
