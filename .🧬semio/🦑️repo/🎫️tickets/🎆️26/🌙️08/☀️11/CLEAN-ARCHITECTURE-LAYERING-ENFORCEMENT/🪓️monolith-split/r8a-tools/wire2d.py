import sys,os,re
A,prefix,hub_suffix=sys.argv[1:4]
FWD='["semio-framework-plugin/artifact-app-testing", "semio-framework-os-kernel/protocol-laws", "semio-framework-os-kernel/mutation-testing", "semio-s-plugin-puzzle-jobs/testing"]'
crates={}
for root,ds,fs in os.walk(A):
    ds[:]=[x for x in ds if x not in('node_modules','target','🤖️generated')]
    if 'Cargo.toml' in fs:
        t=open(os.path.join(root,'Cargo.toml'),encoding='utf8').read()
        m=re.search(r'^name\s*=\s*"([^"]+)"',t,re.M)
        if m and m.group(1).startswith(prefix): crates[m.group(1)]=root
byident={n.replace('-','_'):n for n in crates}
pairs={}
for n,root in crates.items():
    for mp in (os.path.join(root,'🪞️mirror','🦀️.rs'),os.path.join(root,'🦀️.rs')):
        if not os.path.exists(mp): continue
        t=open(mp,encoding='utf8').read()
        for m in re.finditer(r'#\[cfg\(any\(test, feature = "testkit"\)\)\]\s*pub use ([A-Za-z0-9_]+)::',t):
            o=byident.get(m.group(1))
            if o and o!=n: pairs.setdefault(o,set()).add(n)
for o,consumers in sorted(pairs.items()):
    cp=crates[o]+'/Cargo.toml'; t=open(cp,encoding='utf8').read()
    if '[features]' not in t:
        fwd=FWD if o.endswith(hub_suffix) else '[]'
        t=t.replace('[dependencies]\n','[features]\ntestkit = %s\n\n[dependencies]\n'%fwd,1)
        open(cp,'w',encoding='utf8').write(t)
    for c in sorted(consumers):
        p=crates[c]+'/Cargo.toml'; x=open(p,encoding='utf8').read()
        m=re.search(r'^'+re.escape(o)+r' = \{ path = "([^"]+)" \}\n',x,re.M)
        assert m,(c,o)
        if re.search(r'^'+re.escape(o)+r' = \{ path = "[^"]+", features = \["testkit"\] \}',x,re.M): continue
        line='%s = { path = "%s", features = ["testkit"] }\n'%(o,m.group(1))
        if '[dev-dependencies]\n' in x: x=x.replace('[dev-dependencies]\n','[dev-dependencies]\n'+line,1)
        else: x=x.rstrip('\n')+'\n\n[dev-dependencies]\n'+line
        open(p,'w',encoding='utf8').write(x)
    print('owner',o[-30:],'consumers',len(consumers))
print('crates',len(crates))
