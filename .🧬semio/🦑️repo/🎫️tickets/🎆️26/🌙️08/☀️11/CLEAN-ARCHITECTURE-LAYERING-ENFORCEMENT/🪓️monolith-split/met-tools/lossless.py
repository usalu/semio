import collections,sys,re,os
F=os.environ["F"]; R="/Users/ueli/Documents/semio"
orig=[l.strip() for l in open(F+"/met-tools/gis-original.ts.txt",encoding="utf8") if l.strip()]
files=[R+"/"+l.strip() for l in open(F+"/met-tools/gis-new-files.txt",encoding="utf8") if l.strip()]
new=[]
for f in files:
    new+=[l.strip() for l in open(f,encoding="utf8") if l.strip()]
co=collections.Counter(orig); cn=collections.Counter(new)
lost=co-cn; added=cn-co
print("original lines",len(orig),"new lines",len(new))
print("lost (in original, not in new):",sum(lost.values()))
for k,v in sorted(lost.items(), key=lambda kv:-kv[1])[:60]: print("  -",v,k[:170])
print("added (in new, not in original):",sum(added.values()))
for k,v in sorted(added.items(), key=lambda kv:-kv[1])[:5]: print("  +",v,k[:100])
open(F+"/🗑️generated/met-lossless-added.txt","w",encoding="utf8").write("\n".join(f"{v}\t{k}" for k,v in sorted(added.items(), key=lambda kv:-kv[1])))
open(F+"/🗑️generated/met-lossless-lost.txt","w",encoding="utf8").write("\n".join(f"{v}\t{k}" for k,v in sorted(lost.items(), key=lambda kv:-kv[1])))
