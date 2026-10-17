import json,sys
def load(p):
    cur=None; out={}
    for l in open(p,encoding='utf-8'):
        l=l.strip()
        if l.startswith('##'): cur=l[3:]; continue
        if not l: continue
        d=json.loads(l); out[(cur,d['name'])]=d
    return out
b=load(sys.argv[1]); a=load(sys.argv[2])
rows=[]
over_a=0
for k in sorted(b):
    x=b[k]; y=a.get(k)
    if not y: print('MISSING after',k); continue
    ws,tr,te=k[0].split()
    if tr!='triple=native' : continue
    n=k[1].replace('semio-s-artifact-','')
    print('%-9s %-9s %-34s files %3d>%3d bytes %7d>%7d lines %5d>%5d ratio %.3f>%.3f %s' % (ws.replace('ws=','')[:9], te, n, x['files'],y['files'],x['bytes'],y['bytes'],x['activeLines'],y['activeLines'],x['worstRatio'],y['worstRatio'], 'OVER' if y['overLimit'] else 'in-gate'))
# all triples identical?
diff=[(k,b[k]['files'],a[k]['files']) for k in b if k in a and not k[0].split()[1]=='triple=native' and (b[k]['files']!=a[k]['files'] or b[k]['bytes']!=a[k]['bytes'])]
same_native={}
for k in b:
    if k in a:
        ws,tr,te=k[0].split()
        if tr!='triple=native':
            nk=(ws+' triple=native '+te,k[1])
            if nk in a and (a[k]['files'],a[k]['bytes'],a[k]['activeLines'])!=(a[nk]['files'],a[nk]['bytes'],a[nk]['activeLines']): same_native[k]=1
print('wasm32 triples differing from native after:',len(same_native))
print('worsened to over-limit:',[k for k in a if a[k]['overLimit'] and not b[k]['overLimit']])
