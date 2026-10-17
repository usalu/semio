import json,sys,re
t=open(sys.argv[1],encoding='utf-8').read()
i=t.index('\n{\n')+1
j=t.rindex('\n}\n')+2
d=json.loads(t[i:j])
print({k:d[k] for k in ('base','crates','baseCrates')})
P='✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/'
for mode,v in d['layout'].items():
    print(mode,{k:v[k] for k in v if k!='sampleOnlyCombined'})
    for s in v['sampleOnlyCombined'][:int(sys.argv[2]) if len(sys.argv)>2 else 40]: print('   ',s.replace(P,'')[:260])
print(d.get('admission'),d.get('shared'))
