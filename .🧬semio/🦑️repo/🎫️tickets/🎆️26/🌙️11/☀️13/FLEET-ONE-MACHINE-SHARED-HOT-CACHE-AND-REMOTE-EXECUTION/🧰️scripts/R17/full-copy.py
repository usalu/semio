import os,re,sys,shutil
LIVE="/Users/Shared/semio"
SCR="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R17"
OV=SCR+"/overlay";FULL=SCR+"/full"
if os.path.exists(FULL):
    assert os.path.realpath(FULL).startswith("/private/tmp/claude-501/"),FULL
    shutil.rmtree(FULL)
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B=L+"📚️library/🏗️build/"
SC=B+"🌐️remote-execution/🛰️sidecar/"
entries=[SC+"🟦️.ts",L+"🤝️shared-worktree/🔁️replica/🧷️sidecars/🟦️.ts",SC+"🧪️tests/🔁️restart/🟦️.ts",SC+"🧪️tests/🧮️closure/🟦️.ts",SC+"🧪️tests/🟦️.ts",SC+"🧪️tests/🕸️graph/🟦️.ts",B+"🌐️remote-execution/🧪️tests/💽️disk/🟦️.ts"]+[SC+r+"/🟦️.ts" for r in ["👷️agent","🗓️scheduler","🗄️cache","🕸️graph"]]
def where(rel):
    for base in (OV,LIVE):
        if os.path.isfile(base+"/"+rel):return base+"/"+rel
rx=re.compile(r'''(?:from|import)\s*\(?\s*["'](\.{1,2}/[^"']+)["']''')
seen={};stack=list(entries)
while stack:
    rel=stack.pop()
    if rel in seen:continue
    p=where(rel)
    if p is None:continue
    seen[rel]=p
    if re.search(r'\.(ts|mjs|js|tsx)$',rel):
        for m in rx.findall(open(p,encoding="utf8").read()):
            stack.append(os.path.normpath(os.path.join(os.path.dirname(rel),m)))
for rel,p in seen.items():
    d=FULL+"/"+rel;os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(p,d)
print("copied",len(seen),"files")
