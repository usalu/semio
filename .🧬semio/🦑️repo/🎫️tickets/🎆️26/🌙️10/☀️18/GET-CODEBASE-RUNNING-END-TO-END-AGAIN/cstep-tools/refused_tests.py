"""cstep: drop the removed `Refused` variant from consumer arms in test drivers.
 `X::Refused { .. } => ..,` arms are deleted; `A | X::Refused { .. }` alternatives lose the alternative."""
import re,sys,os
write='--write' in sys.argv
ALIAS=r'(?:[\w]+::)*(?:InteractiveJobCloseStep|Close|Step|ICS|CloseStep)'
alt=re.compile(r'\s*\|\s*'+ALIAS+r'::Refused\s*(?:\{[^}]*\}|\([^)]*\))')
alt2=re.compile(ALIAS+r'::Refused\s*(?:\{[^}]*\}|\([^)]*\))\s*\|\s*')
arm=re.compile(r'^[ \t]*'+ALIAS+r'::Refused\s*\{[^}]*\}\s*=>\s*(?:panic!\((?:[^()]|\([^()]*\))*\)|[^,\n]*?),?[ \t]*\n',re.M)
n=0
for p in [l for l in open([a for a in sys.argv[1:] if not a.startswith('--')][0]).read().split('\n') if l]:
    s=open(p,encoding='utf-8').read();ns=s
    ns,k1=arm.subn('',ns)
    ns,k2=alt.subn('',ns)
    ns,k3=alt2.subn('',ns)
    if k1+k2+k3:
        n+=k1+k2+k3;print(p[-80:],k1,k2,k3)
        if write: open(p,'w',encoding='utf-8').write(ns)
print('edits',n)
