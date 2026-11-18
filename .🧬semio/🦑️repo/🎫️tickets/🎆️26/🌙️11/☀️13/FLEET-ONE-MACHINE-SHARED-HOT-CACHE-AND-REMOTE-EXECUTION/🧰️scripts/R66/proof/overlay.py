import os,sys,filecmp,shutil
R="/Users/ueli/Documents/semio";S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R66/tree"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
roots=[B+"🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool",B+"🌐️remote-execution/🛰️sidecar/🏊️pool",B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs",B+"🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs",B+"🧠️memory"]
def changed():
    out=[]
    for r in roots:
        p=os.path.join(S,r)
        files=[p] if os.path.isfile(p) else [os.path.join(a,f) for a,_,fs in os.walk(p) for f in fs]
        for f in files:
            rel=os.path.relpath(f,S);live=os.path.join(R,rel)
            if not os.path.exists(live) or not filecmp.cmp(f,live,shallow=False):out.append(rel)
    return sorted(out)
if __name__=="__main__":
    c=changed()
    if len(sys.argv)>1:
        T=sys.argv[1]
        for rel in c:
            d=os.path.join(T,rel);os.makedirs(os.path.dirname(d),exist_ok=True)
            if os.path.exists(d):os.remove(d)
            shutil.copyfile(os.path.join(S,rel),d)
    print(len(c));print("\n".join(x[len(B):] if x.startswith(B) else x for x in c))
