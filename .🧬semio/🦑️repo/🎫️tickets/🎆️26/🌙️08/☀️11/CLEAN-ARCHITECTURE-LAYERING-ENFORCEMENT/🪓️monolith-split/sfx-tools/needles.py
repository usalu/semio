import re,sys,os
R="/Users/ueli/Documents/semio/"
RT=R+"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"
D=R+"🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/"
T=open(RT+"🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs").read()
def cat(paths): return "\n".join(open(D+p).read() for p in paths)
DRAW=["🏷️types/🦀️.rs","🕸️mesh/🦀️.rs","🔧️pipeline/🏭️factory/🦀️.rs","🔧️pipeline/🎬️execution/🦀️.rs","🖼️raster/🦀️.rs"]
LAWS=["🕸️mesh/🧪️tests/🔬️unit/🦀️.rs","🔧️pipeline/🏭️factory/🧪️tests/🦀️.rs","🔧️pipeline/🎬️execution/🧪️tests/🔬️unit/🦀️.rs","🖼️raster/🧪️tests/🔬️unit/🦀️.rs"]
GPU=["🎮️execution/🦀️.rs"]
extra=sys.argv[1:]
srcs={"DRAW":cat(DRAW+[e for e in extra if e.startswith("D:")] and DRAW),"LAWS":cat(LAWS),"GPU":cat(GPU)}
lit=r'"((?:[^"\\]|\\.)*)"'
def un(s): return s.replace('\\"','"').replace("\\\\","\\").replace("\\n","\n")
pat={"DRAW":r'(?:DRAW_SOURCE|\bdraw)\.(?:contains|replace|replacen|find|matches|split)\(\s*'+lit,
     "LAWS":r'(?:DRAW_LAWS_SOURCE|\blaws)\.(?:contains|replace|find|matches)\(\s*'+lit,
     "GPU":r'(?:GPU_SOURCE|\bgpu)\.(?:contains|replace|replacen|find|matches|split)\(\s*'+lit}
for k,p in pat.items():
    miss=[]; tot=0
    for m in re.finditer(p,T):
        n=un(m.group(1)); tot+=1
        if n not in srcs[k]: miss.append(n)
    print(k,"needles",tot,"missing",len(miss),"bytes",len(srcs[k]))
    for n in dict.fromkeys(miss): print("   MISSING:",n[:140].replace("\n","\\n"))
