import os,re,subprocess,sys,shutil
R="/Users/ueli/Documents/semio";D="/private/tmp/r71/t"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
skip={".cache","🧪️tests","🧫️fixtures","node_modules","🧪️fixture","target",".git","🗑️generated","dist"}
def copydir(rel):
    n=0
    for root,dirs,files in os.walk(R+"/"+rel):
        dirs[:]=[d for d in dirs if d not in skip]
        for f in files:
            s=os.path.join(root,f)
            if os.path.islink(s) or os.path.getsize(s)>3_000_000:continue
            d=D+"/"+os.path.relpath(s,R)
            if not os.path.exists(d):os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copyfile(s,d);n+=1
    return n
cwd=D+"/"+sys.argv[1]
for it in range(40):
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
    r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","60000"],cwd=cwd,env=env,capture_output=True,text=True)
    out=r.stdout+r.stderr;m=re.search(r"Cannot find module '([^']+)' from '([^']+)'",out)
    if not m:print("done",it,"rc",r.returncode);break
    spec,frm=m.groups();target=os.path.normpath(os.path.join(os.path.dirname(frm),spec));rel=os.path.relpath(target,D)
    parts=rel.split("/")
    if len(parts)>=3 and parts[0]=="🧰️framework" and parts[1] in("🔨️modules",):top="/".join(parts[:3])
    elif parts[0]=="🧰️framework":top="/".join(parts[:2])
    else:top=parts[0]
    if not os.path.exists(R+"/"+rel) and not os.path.exists(R+"/"+top):print("MISSING live",rel);break
    print("copy",top,copydir(top))
