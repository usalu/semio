import json,os,subprocess,sys,shutil
H="/usr/bin/hdiutil"; MiB=1<<20; INITIAL=64*MiB
work=sys.argv[1]; sizes=[int(x) for x in sys.argv[2].split(",")]
seed=os.path.join(work,"seed"); os.makedirs(seed+"/.fseventsd",exist_ok=True)
open(seed+"/.fseventsd/no_log","w").close(); open(seed+"/.metadata_never_index","w").close()
out=[]
for size in sizes:
    img=os.path.join(work,f"m{size}.dmg"); mnt=f"/private/tmp/semio-store-g-mnt/c{size}"; os.makedirs(mnt,exist_ok=True)
    first=min(size,INITIAL)
    subprocess.run([H,"create","-size",f"{first//MiB}m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,img],check=True,capture_output=True)
    if size>first: subprocess.run([H,"resize","-size",f"{size//MiB}m",img],check=True,capture_output=True)
    subprocess.run([H,"attach","-nobrowse","-noautoopen","-owners","on","-mountpoint",mnt,img],check=True,capture_output=True)
    try:
        s=os.statvfs(mnt); out.append({"imageBytes":size,"filesystemBytes":s.f_blocks*s.f_frsize,"usedAtEmptyBytes":(s.f_blocks-s.f_bfree)*s.f_frsize})
    finally:
        subprocess.run([H,"detach",mnt],capture_output=True); os.rmdir(mnt) if os.path.isdir(mnt) else None
    os.remove(img)
print(json.dumps(out))
