"""cstep: allocation-oracle sites must not allocate a cancellation root inside the observation: hoist a SelfFundedCloseCaller.
usage: observed.py [--write] listfile"""
import re,sys,os
write='--write' in sys.argv
pat=re.compile(r'semio_framework_job::close_with_self_funded_context\(')
n=0
for p in [l for l in open([a for a in sys.argv[1:] if not a.startswith('--')][0]).read().split('\n') if l]:
    s=open(p,encoding='utf-8').read();lines=s.split('\n');changed=False
    for i,l in enumerate(lines):
        if 'close_with_self_funded_context' not in l: continue
        m=pat.search(l)
        if not m: continue
        pre=l[:m.start()]
        if not re.search(r'(?:observe\w*|measured\w*)\s*\(\s*\|\|',pre): continue
        # rewrite every occurrence on this line to use `self_funded`
        nl=pat.sub('self_funded.turn(',l)
        ind=re.match(r'\s*',l).group(0)
        lines[i]=ind+'let mut self_funded = semio_framework_job::SelfFundedCloseCaller::new();'+('\n' if False else '')+nl.lstrip() if False else None
        lines[i]=ind+'let mut self_funded = semio_framework_job::SelfFundedCloseCaller::new();\n'+nl
        changed=True;n+=1
    if changed:
        print(p[-80:])
        if write: open(p,'w',encoding='utf-8').write('\n'.join(lines))
print('sites',n)
