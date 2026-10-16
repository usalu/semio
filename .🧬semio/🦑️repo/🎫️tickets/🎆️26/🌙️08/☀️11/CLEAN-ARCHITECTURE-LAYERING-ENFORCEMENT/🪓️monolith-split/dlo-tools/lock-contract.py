# -*- coding: utf-8 -*-
"""Rewrites Cargo.lock files for the procedural core re-point: the three core crates depend on the flow and playbook contract facades instead of the artifact crates; the two facade packages are added in sorted position.
Usage: lock-contract.py <lock>... (idempotent; prints the edited package names per lock)"""
import re,sys
CORE={"semio-s-procedural-core-generation-sqlite","semio-s-procedural-core-generation-sqlite-test-support","semio-s-procedural-core-semantic-ui"}
SWAP={"semio-framework-artifact-flow-flow":"semio-framework-os-flow-contract","semio-framework-artifact-playbook-playbook":"semio-framework-os-playbook-contract"}
NEW={"semio-framework-os-flow-contract":"semio-framework-artifact-flow-flow","semio-framework-os-playbook-contract":"semio-framework-artifact-playbook-playbook"}
for path in sys.argv[1:]:
    text=open(path,encoding="utf8").read()
    head,*blocks=text.split("\n[[package]]\n")
    out=[]; edited=[]
    names=[]
    for b in blocks:
        m=re.match(r'name = "([^"]+)"\n',b); name=m.group(1)
        if name in CORE:
            lines=b.split("\n"); new=[]; changed=False
            for l in lines:
                mm=re.match(r'^ "([^"]+)",$',l)
                if mm and mm.group(1) in SWAP: l=' "%s",'%SWAP[mm.group(1)]; changed=True
                new.append(l)
            if changed:
                i=next(k for k,l in enumerate(new) if l=="dependencies = [")
                j=next(k for k in range(i,len(new)) if new[k]=="]")
                deps=sorted(set(new[i+1:j]),key=lambda l:re.match(r'^ "([^"]+)"',l).group(1))
                new[i+1:j]=deps
                b="\n".join(new); edited.append(name)
        out.append((name,b))
    have={n for n,_ in out}
    anything=bool(edited)
    if anything:
        for n,dep in NEW.items():
            if n in have: continue
            if not any(x==dep for x in have): raise SystemExit(f"{path}: missing {dep}")
            blk=f'name = "{n}"\nversion = "0.1.0"\ndependencies = [\n "{dep}",\n]\n'
            k=next((i for i,(x,_) in enumerate(out) if x>n),len(out))
            out.insert(k,(n,blk)); edited.append(n)
    if edited:
        open(path,"w",encoding="utf8").write("\n[[package]]\n".join([head]+[b for _,b in out]))
    print(path[-60:],edited)
