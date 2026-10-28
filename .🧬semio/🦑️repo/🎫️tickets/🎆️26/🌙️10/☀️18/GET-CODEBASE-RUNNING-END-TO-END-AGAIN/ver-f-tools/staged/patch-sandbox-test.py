"""usage: patch-sandbox-test.py <file> — fixtures of the three managed-parent/short-base sandbox tests live outside every managed native parent even when TMPDIR is inside one (the official runner's TMPDIR is .cache/b/p/m/tmp)."""
import sys
p=sys.argv[1];s=open(p).read()
n=0
for old,new in [('mkdtempSync(join(tmpdir(), "d-"))','mkdtempSync(join(fixtureBaseV1(), "d-"))'),('mkdtempSync(join(tmpdir(), "mp-"))','mkdtempSync(join(fixtureBaseV1(), "mp-"))')]:
    n+=s.count(old);s=s.replace(old,new)
assert n==3,n
anchor='process.env.SEMIO_AGENT_SESSION_DIRECTORY'
assert s.count(anchor)==1
s=s.replace(anchor,'const fixtureBaseV1=():string=>safeTemporaryDirectoryV1()??tmpdir();\n'+anchor,1)
open(p,'w').write(s)
