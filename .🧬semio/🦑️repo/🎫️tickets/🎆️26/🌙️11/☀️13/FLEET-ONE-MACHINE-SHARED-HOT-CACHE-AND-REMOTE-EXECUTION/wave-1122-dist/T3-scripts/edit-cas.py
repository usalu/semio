import re,hashlib,os,sys
p=sys.argv[1];s=open(p,encoding="utf8").read();o=s
blocks=re.split(r'\n(?=test\()',s);seen=set();kept=[]
for b in blocks:
    m=re.match(r'test\("([^"]+)"',b)
    if m and b in seen:continue
    if m:seen.add(b)
    kept.append(b)
s="\n".join(kept)
def rep(a,b):
    global s
    assert s.count(a)==1,a;s=s.replace(a,b)
rep('let collects=0,mode:"relieve"|"keep"|"hang"="relieve";let used=150*MIB;','let collects=0,mode:"relieve"|"keep"|"hang"="relieve",hung=false;let used=150*MIB;')
rep('if(mode==="hang")await new Promise(done=>setTimeout(done,500));','if(mode==="hang"){await new Promise(done=>setTimeout(done,500));hung=true;}')
rep(' const began=Date.now(),hanging=createCasGuardV1(ports(true)),timed=await refusal(hanging.admit(MIB));\n expect(timed).toBeInstanceOf(CasCapRefusalV1);expect(Date.now()-began).toBeLessThan(400);',
    ' const hanging=createCasGuardV1(ports(true)),timed=await refusal(hanging.admit(MIB));\n expect(timed).toBeInstanceOf(CasCapRefusalV1);expect(hung,"refused at the admission budget, before the hanging collection ended").toBe(false);')
open(p+".t3tmp","w",encoding="utf8").write(s);os.replace(p+".t3tmp",p)
print(hashlib.sha256(o.encode()).hexdigest()[:12],"->",hashlib.sha256(s.encode()).hexdigest()[:12],len(o),len(s))
