import os,sys
root="/Users/ueli/Documents/semio"
patches=[]
for rel,old,new in patches:
    p=os.path.join(root,rel)
    s=open(p).read()
    if new in s and old not in s: continue
    assert s.count(old)==1,rel
    open(p,"w").write(s.replace(old,new))
print("assembly precheck: %d patch(es), nothing to apply"%len(patches))
