import os,sys
LIVE="/Users/ueli/Documents/semio"
FARM=sys.argv[1]
chain=["🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🏛️sealed","🧰️tool-snapshot"]
lp=LIVE;fp=FARM
for i,comp in enumerate(chain):
    nxt=chain[i+1] if i+1<len(chain) else None
    base=os.path.join(lp,"") if i==0 else lp
    parent_live=lp;parent_farm=fp
    if i==0:
        pass
    # populate siblings at this level (children of parent_live) except comp, for levels > 0; at i==0 populate nothing at farm root except node_modules
    if i>0:
        for e in os.listdir(parent_live):
            if e==comp: continue
            t=os.path.join(parent_farm,e)
            if not os.path.lexists(t): os.symlink(os.path.join(parent_live,e),t)
    lp=os.path.join(lp,comp);fp=os.path.join(fp,comp);os.makedirs(fp,exist_ok=True)
# root files
for e in ["node_modules","package.json","bunfig.toml","tsconfig.json","bun.lock"]:
    t=os.path.join(FARM,e)
    if not os.path.lexists(t) and os.path.exists(os.path.join(LIVE,e)): os.symlink(os.path.join(LIVE,e),t)
# framework-level siblings (children of 🧰️framework other than 🛍️products): handled at i==1 for 🛍️products siblings; need framework children too
fw=os.path.join(LIVE,"🧰️framework");ffw=os.path.join(FARM,"🧰️framework")
for e in os.listdir(fw):
    t=os.path.join(ffw,e)
    if e!="🛍️products" and not os.path.lexists(t): os.symlink(os.path.join(fw,e),t)
