import sys
p=sys.argv[1];s=open(p).read()
a="proofs=new WeakMap<object,Proof>(),"
b="protectedGroupsV1=new WeakMap<object,Set<string>>(),proofs=new WeakMap<object,Proof>(),"
c="protectedSources=new Set<string>(),started=Date.now()"
d="protectedSources=protectedGroupsV1.get(custody)??new Set<string>(),started=Date.now()"
assert s.count(a)==1 and s.count(c)==1
s=s.replace(a,b).replace(c,d)
e="let retiredBytes=0,retiredFiles=0,iterations=0;const free="
assert s.count(e)==1
s=s.replace(e,"protectedGroupsV1.set(custody,protectedSources);"+e)
open(p,"w").write(s)
