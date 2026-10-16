import sys, subprocess; sys.path.insert(0,'.')
from model7 import *
from plan7 import *
PLUGREL='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
def plugin_files():
    a=subprocess.run(['git','-c','core.fsmonitor=false','-c','core.quotepath=false','ls-files','--','%s'%PLUGREL],cwd=ROOT,capture_output=True,text=True).stdout.split('\n')
    b=subprocess.run(['git','-c','core.fsmonitor=false','-c','core.quotepath=false','ls-files','--others','--exclude-standard','--','%s'%PLUGREL],cwd=ROOT,capture_output=True,text=True).stdout.split('\n')
    return sorted({f for f in a+b if f.endswith('.rs')})
if __name__=='__main__':
    files=plugin_files()
    print(len(files))
    names={x['name']:crate_of(x) for x in nodes if crate_of(x)!='app'}
    fam=set(PLUGREL+'/'+r for r in TIER2_FILES)
    pat=re.compile(r'(?:\.|::)\s*('+'|'.join(sorted(map(re.escape,names),key=len,reverse=True))+r')\b\s*(?:::<[^>]*>)?\s*\(')
    hits=collections.defaultdict(collections.Counter)
    byfile=collections.defaultdict(collections.Counter)
    for f in files:
        if f in fam: continue
        t=open(ROOT+f,encoding='utf-8').read()
        m=mask(t)
        for mm in pat.finditer(m):
            hits[mm.group(1)][f]+=1; byfile[f][mm.group(1)]+=1
    print('names used outside family:',len(hits))
    for n,c in sorted(hits.items(), key=lambda kv:-sum(kv[1].values()))[:60]:
        print('%-45s %-12s %3d files %4d uses'%(n,names[n],len(c),sum(c.values())))
    print('files:',len(byfile))
    for f,c in sorted(byfile.items(), key=lambda kv:-sum(kv[1].values()))[:50]:
        print(sum(c.values()), f.replace(PLUGREL+'/',''), list(c)[:6])
