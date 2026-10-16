import json,sys
a=json.load(open(sys.argv[1]));b=json.load(open(sys.argv[2]))
ka,kb=a['kinds'],b['kinds']
changed=[p for p in ka if p in kb and ka[p]!=kb[p]]
lost=[p for p in ka if p not in kb]
newly=[p for p in kb if p not in ka]
print('before resolved',len(ka),'after resolved',len(kb),'kind-id changed',len(changed),'lost',len(lost),'newly resolved',len(newly),'ambiguous after',len(b['ambig']))
for p in newly: print(' +',kb[p],p)
