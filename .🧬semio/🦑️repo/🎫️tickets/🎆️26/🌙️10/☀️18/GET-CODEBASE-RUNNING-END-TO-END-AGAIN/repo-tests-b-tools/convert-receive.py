import os,re,sys
H="🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"
for p in sys.argv[1:]:
  s=open(p).read(); n=0
  for pat in (r'issuer\.receiveScriptProcessInvocation\(process\.env,\s*', r'(?<![\w.])receiveScriptProcessInvocation\(process\.env,\s*'):
    s,k=re.subn(pat,'withBuildTestInvocationV1(',s); n+=k
  if not n: print("skip",p); continue
  rel=os.path.relpath(H,os.path.dirname(p))
  if not rel.startswith("."): rel="./"+rel
  if "withBuildTestInvocationV1" not in s.split("withBuildTestInvocationV1(")[0]:
    imp='import {withBuildTestInvocationV1} from "%s";\n'%rel
    i=s.index("\n")+1 if s.startswith("#!") else 0
    s=s[:i]+imp+s[i:]
  open(p,"w").write(s); print("converted",n,p[-60:])
