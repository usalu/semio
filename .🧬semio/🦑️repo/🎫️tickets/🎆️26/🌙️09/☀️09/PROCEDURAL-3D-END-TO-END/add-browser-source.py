#!/usr/bin/env python3
"""🧩️ Adds one source path to every WGPU browser source list (project inputs and taxonomy) next to the kernel plugin-catalog row, in codepoint order."""
import sys,re
new=sys.argv[1]
files=["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"]
anchor="🧰️framework/🔨️modules/🎠️kernel/📚️plugin-catalog/🟦️.ts"
for f in files:
    lines=open(f,encoding='utf8').read().split("\n")
    out=[];i=0;added=0
    # find list blocks that contain the anchor
    idx=[k for k,l in enumerate(lines) if anchor in l and l.strip().startswith(('"{workspaceRoot}','"🧰️'))]
    for a in idx:
        s=a
        while lines[s-1].strip().startswith('"') and lines[s-1].strip().endswith(('",','"')) and not lines[s-1].strip().endswith('[') : s-=1
        e=a
        while lines[e+1].strip().startswith('"') and lines[e+1].strip().endswith(('",','"')): e+=1
        block=lines[s:e+1]
        prefix='{workspaceRoot}/' if '{workspaceRoot}' in lines[a] else ''
        if any(new in l for l in block): continue
        ind=re.match(r'\s*',lines[a]).group(0)
        entry=f'{ind}"{prefix}{new}"'
        pos=None
        for k,l in enumerate(block):
            v=l.strip().rstrip(',').strip('"').replace('{workspaceRoot}/','')
            if v.startswith("🧰️framework/🔨️modules/🎠️kernel/") and v>new: pos=k;break
        if pos is None: print("skip block",f,s); continue
        block.insert(pos,entry+",")
        lines[s:e+1]=block
        # shift later indexes
        idx=[x+1 if x>=s else x for x in idx]
        added+=1
    open(f,'w',encoding='utf8').write("\n".join(lines))
    print(f,added)
