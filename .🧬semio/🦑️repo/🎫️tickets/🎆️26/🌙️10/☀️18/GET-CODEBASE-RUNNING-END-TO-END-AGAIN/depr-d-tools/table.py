import json,sys,re
G=sys.argv[1]
d=json.load(open(G+'/new.json'))['newDeps']
u=json.load(open(G+'/usage.json'))
u2=json.load(open(G+'/usage2.json'))
removed={'color':'(a) UNUSED: no importer in any tracked/untracked source; cui-d added it for the ui-styling color oracle, which uses `color-string` (`🛡️verification/🧪️tests/⚖️color-oracle`). REMOVED from root package.json + root lock',
 'npm-run-path':'(a) UNUSED root pin: no importer; its only consumers (execa, nx) resolve their own copy. REMOVED from root package.json + root lock',
 'terser':'(a) UNUSED: no importer; vite is only run with `--minify esbuild|false` (`semioViteBuild`, `⚡️caching/🌐️vite`). REMOVED from course + ui-react manifests, 6 locks'}
kinds={'@types/':'type declarations for the TS check of the library/tests below (no runtime code)','@webgpu/types':'type declarations (6 test/probe files)'}
def short(f):
    parts=f.split('/')
    return '/'.join(parts[-3:-1]) if len(parts)>3 else f
rows=[]
for e in d:
    n=e['name'];eco=e['ecosystem']
    decl=[x['user'] for x in e['declarations']]
    where='root' if decl==['package.json'] else ('root+%d'%(len(decl)-1) if 'package.json' in decl else '%d member(s)'%len(decl))
    kind=','.join(e['kinds'])
    if n in removed: dec=removed[n]; ev='0 importers'
    elif eco=='rust': dec='(b) test oracle: `bitvec` Hamming distance recomputes every traced match verdict (`🧪️tests/🔬️unit/🦀️.rs`), `[dev-dependencies]` only, standalone remodeling workspace lock carries it. KEEP, baseline'; ev='1 test file'
    elif n.startswith('@types/') or n=='@webgpu/types': dec='(b) type-only devDependency; no runtime. baseline'; ev='type packages'
    else:
        fs=u.get(n,[]) or u2.get(n,[])
        tf=[f for f in fs if '/🧪️' in '/'+f or '/🔬️probes/' in f or '/🔮️oracles/' in f]
        ev='%d test importer(s), e.g. %s'%(len(tf),short(tf[0])) if tf else 'script/ambient only'
        if n in('@bytecodealliance/jco-transpile','@bytecodealliance/preview2-shim'): dec='(b) devDependency of the os plugin browser-bundle build script (`browser-bundle/📜️script.ts`, vendoring); build-time, not course/print/dashboard. FOR os owner: vendored shim ships in the browser bundle, check the boundary. baseline'
        elif n=='scheduler': dec='(b) oracle inside the draw plugin law script (`📜️script.ts` `bootstrapYieldOracle`, `scheduler/unstable_mock`); devDependency. baseline'
        elif n=='ws': dec='(b) hub forwarding-proxy test + browser-serving test oracle; devDependency (hub). baseline'
        else: dec='(b) test oracle / test infrastructure, devDependency only. baseline'
    rows.append(f"| {eco} | `{n}` | {e['version']} | {where} | {kind} | {ev} | {dec} |")
open(G+'/table.md','w').write('| eco | name | version | declared | kind | evidence | decision |\n|---|---|---|---|---|---|---|\n'+'\n'.join(rows)+'\n')
print(len(rows))
