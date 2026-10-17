#!/usr/bin/env python3
"""🧰 bd10: splits the raster layer mutations crate into structure (create, delete, reorder, rename, move, resize) and layer properties (visible, locked, opacity, blend mode, adjustment kind and parameter, transform).
usage: split_layers.py [--write]"""
import os,re,sys
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster'
A=R+'/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any'
M=A+'/🧬️schema/🧬️mutations'
write='--write' in sys.argv
OLD='semio_s_artifact_raster_raster_mutations_layers'
NEW='semio_s_artifact_raster_raster_mutations_layer_properties'
PROPS=['change_layer_adjustment_kind','change_layer_adjustment_parameter','change_layer_blend_mode','change_layer_locked','change_layer_opacity','change_layer_transform','change_layer_visible']
root=open(M+'/🗂️layers/🦀️.rs',encoding='utf-8').read()
mounts=re.findall(r'#\[path = "([^"]+)"\]\npub mod (\w+);\n',root)
prop_mounts=[(p,n) for p,n in mounts if n in PROPS]
assert len(prop_mounts)==7,prop_mounts
head=root[:root.index('//#region 🪢️TaxonomyMounts')]
new_head=head.replace('//! 🗂️ Raster layer mutations: create, delete, reorder, move, rename, resize and restyle layers.','//! 🏷️ Raster layer property mutations: visibility, lock, opacity, blend mode, adjustment kind and parameter, and transform.')
new_root=new_head+'//#region 🪢️TaxonomyMounts\n'+''.join(f'#[path = "{p}"]\npub mod {n};\n' for p,n in sorted(prop_mounts,key=lambda x:x[1]))+'//#endregion 🪢️TaxonomyMounts\n'
slim=root
for p,n in prop_mounts:
    blk=f'#[path = "{p}"]\npub mod {n};\n'
    assert slim.count(blk)==1
    slim=slim.replace(blk,'')
slim=slim.replace('//! 🗂️ Raster layer mutations: create, delete, reorder, move, rename, resize and restyle layers.','//! 🗂️ Raster layer structure mutations: create, delete, reorder, move, rename and resize layers.')
print(new_root)
edits={}
for dp,dn,fn in os.walk(R):
    for f in fn:
        if not f.endswith('.rs'): continue
        p=os.path.join(dp,f)
        s=open(p,encoding='utf-8').read()
        if OLD not in s: continue
        o=s
        s=re.sub(OLD+r'::('+'|'.join(PROPS)+r')\b',NEW+r'::\1',s)
        def grp(m):
            names=[x.strip() for x in m.group(1).split(',') if x.strip()]
            prop=[x for x in names if x.split(' as ')[0].strip() in PROPS]
            rest=[x for x in names if x not in prop]
            out=[]
            if rest: out.append(f'use {OLD}::{{{", ".join(rest)}}};')
            if prop: out.append(f'use {NEW}::{{{", ".join(prop)}}};')
            return '\n'.join(out)
        s=re.sub(r'use '+OLD+r'::\{([^}]*)\};',grp,s)
        if s!=o: edits[p]=s
print(len(edits),'files rewritten')
for p in edits: print(' ',p.split('any/')[-1])
if write:
    os.makedirs(M+'/🏷️layer-properties',exist_ok=True)
    open(M+'/🏷️layer-properties/🦀️.rs','w',encoding='utf-8').write(new_root)
    open(M+'/🗂️layers/🦀️.rs','w',encoding='utf-8').write(slim)
    for p,s in edits.items(): open(p,'w',encoding='utf-8').write(s)
