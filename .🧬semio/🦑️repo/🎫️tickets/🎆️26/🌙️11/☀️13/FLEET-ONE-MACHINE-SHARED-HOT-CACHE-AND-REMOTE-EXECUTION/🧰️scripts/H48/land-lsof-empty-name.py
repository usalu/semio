import os,sys
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts"
s=open(P,encoding='utf8').read()
a='if(identifiers.has(id)||!data)throw Error("Unambiguous lsof field required");'
if s.count(a)!=1: sys.exit("anchor")
s=s.replace(a,'if(identifiers.has(id)||!data&&id!=="n")throw Error("Unambiguous lsof field required");')
if os.environ.get("H48_DRY"): print("dry ok"); sys.exit(0)
open(P+".h48tmp","w",encoding='utf8').write(s); os.replace(P+".h48tmp",P); print("ok")
