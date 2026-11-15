import os,tarfile,filecmp,sys
D=os.environ["R10_SCRATCH"];R="/Users/Shared/semio"
OUT=sys.argv[1];mine=[];new=[]
for d,_,fs in os.walk(D+"/tree"):
    for f in fs:
        p=os.path.join(d,f);rel=os.path.relpath(p,D+"/tree")
        if os.path.exists(D+"/base/"+rel):
            if not filecmp.cmp(p,D+"/base/"+rel,shallow=False):mine.append(rel)
        elif not os.path.exists(R+"/"+rel):new.append(rel)
with tarfile.open(OUT,"w") as t:
    for rel in sorted(mine+new):
        t.add(D+"/tree/"+rel,arcname="mirror/"+rel)
        if rel in mine:t.add(D+"/base/"+rel,arcname="base/"+rel)
print(len(mine),"changed",len(new),"new");print("\n".join(sorted(mine+new)))
