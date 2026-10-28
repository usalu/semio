"""[lane diskbud] read-only: bytes of files whose mtime lies in the last 3 h / 24 h / 72 h, grouped by the first DEPTH path parts below a root (one streaming walk, low RAM). usage: written.py ROOT DEPTH"""
import os,sys,time
root,depth=sys.argv[1],int(sys.argv[2]);now=time.time();cuts=(3*3600,24*3600,72*3600);acc={}
def walk(path,parts):
    try:
        with os.scandir(path) as it:
            for e in it:
                try:
                    if e.is_dir(follow_symlinks=False):
                        if e.name==".fseventsd":continue
                        walk(e.path,parts if len(parts)>=depth else parts+[e.name])
                    elif e.is_file(follow_symlinks=False):
                        st=e.stat(follow_symlinks=False);age=now-st.st_mtime
                        key="/".join(parts[:depth]) or "."
                        row=acc.setdefault(key,[0,0,0,0])
                        row[3]+=st.st_blocks*512
                        for i,c in enumerate(cuts):
                            if age<c:row[i]+=st.st_blocks*512
                except OSError:pass
    except OSError:pass
walk(root,[])
print("area\tMiB<3h\tMiB<24h\tMiB<72h\tMiB total(blocks)")
for k,v in sorted(acc.items(),key=lambda kv:-kv[1][1]):
    if v[2] or v[3]>50<<20:print(k,*[round(x/2**20) for x in v],sep="\t")
