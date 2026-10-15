import os, collections, re, sys
TM = '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map'
ORIG = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/tiled-map.orig.rs'
def norm(l): return l.strip()
orig = collections.Counter(norm(l) for l in open(ORIG,encoding='utf-8').read().split('\n') if norm(l))
new = collections.Counter(); files=[]
for root,_,fs in os.walk(TM):
    if '🧪️tests' in root or '🔮️oracles' in root or '🧬️schema' in root: continue
    for f in fs:
        if f=='🦀️.rs':
            p=os.path.join(root,f); files.append(p)
            for l in open(p,encoding='utf-8').read().split('\n'):
                if norm(l): new[norm(l)]+=1
print('files',len(files),'orig nonblank lines',sum(orig.values()),'new nonblank lines',sum(new.values()))
missing=orig-new; extra=new-orig
print('--- in original but not in new (count %d):'%sum(missing.values()))
for k,v in sorted(missing.items()): print('  -%d %s'%(v,k[:140]))
print('--- in new but not in original (count %d):'%sum(extra.values()))
for k,v in sorted(extra.items()): print('  +%d %s'%(v,k[:160]))
