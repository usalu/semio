import pickle,collections,sys,json
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
I=L+'🏗️build/📥️inputs/'
HELP={I+p for p in ["🟨️.mjs","🔏️revision/🟨️.mjs","🔏️revision/🧬️schema/🔣️.json","📋️data/🟨️.mjs","📋️data/📥️load/🟨️.mjs","📋️data/🧬️schema/🔣️.json","🧮️digest/📥️read/🟨️.mjs","🧮️digest/📥️read/🧬️schema/🔣️.json","🔒️lock/🔣️schema.json","💾️storage/🟨️.mjs","💾️storage/📒️ledger/🟨️.mjs","💾️storage/🧬️schema/🔣️.json","🧮️digest/🟨️.mjs","🧮️digest/🔣️schema.json","🔒️lock/📥️read/🟨️.mjs","🔒️lock/🔣️schema.json","🔒️lock/🟨️.mjs"]}|{L+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json'}
ROOT={'bunfig.toml','tsconfig.json','rustfmt.toml','.cargo/config.toml','rust-toolchain.toml','go.work','go.work.sum','Cargo.lock','Cargo.toml','bun.lock','package.json'}
SHIM=L+'🏃️process/🦀️cargo/🏃️execution/📜️script.ts'
def klass(f):
    if f in HELP:return '1 digest helpers (the 17)'
    if f in ROOT or f==SHIM:return '2 root configs and cargo shim'
    if f.endswith('.rs') or f.endswith('Cargo.toml') or '/Cargo.' in f:return '3 Rust sources and manifests'
    if f.startswith(L):return '4 library tooling'
    return '5 other (TS, JSON, assets)'
res={}
for name in sys.argv[1:]:
    rows,=pickle.load(open(name,'rb'))
    c=collections.defaultdict(lambda:[0,0,0])
    for f,n,t in rows:
        k=klass(f);c[k][0]+=1;c[k][1]+=t;c[k][2]+=n*t
    tot=sum(v[2] for v in c.values())
    print('==',name,'total owner-stales',tot)
    for k in sorted(c):print('  ',k.ljust(34),'files',str(c[k][0]).rjust(6),'touches',str(c[k][1]).rjust(6),'owner-stales',str(c[k][2]).rjust(9),'%5.2f%%'%(100*c[k][2]/tot))
    hs=[(n,t,f) for f,n,t in rows if f in HELP]
    print('   helper rows touched:',[(n,t,f.replace(L,'LIB/')[-50:]) for n,t,f in sorted(hs,reverse=True)])
