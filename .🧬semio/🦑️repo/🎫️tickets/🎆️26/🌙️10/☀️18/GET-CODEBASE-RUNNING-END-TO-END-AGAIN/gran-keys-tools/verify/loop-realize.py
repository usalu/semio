import os,re,shutil,subprocess,sys
SB="/private/tmp/gk/sb"; ROOT="/Users/ueli/Documents/semio"
test=sys.argv[1]
for i in range(60):
    r=subprocess.run(["bun","test","./🟦️.ts"],cwd=test,capture_output=True,text=True)
    m=re.search(r"(?m)^error: Build inference requires real authority (?:ancestry|file): (\S.*)",r.stdout+r.stderr)
    if not m: print("done after",i); print((r.stdout+r.stderr)[-1500:]); break
    path=m.group(1).strip()
    if not path.startswith(SB+"/"): print("outside",path); break
    rel=path[len(SB)+1:]
    # find the symlinked ancestor
    parts=rel.split("/"); cur=SB
    for j,p in enumerate(parts):
        cur=os.path.join(cur,p)
        if os.path.islink(cur):
            target=os.path.join(ROOT,*parts[:j+1]); os.unlink(cur); subprocess.run(["cp","-Rc",target,cur],check=True); print("realized","/".join(parts[:j+1])); break
    else:
        print("no symlink ancestor for",rel); break
