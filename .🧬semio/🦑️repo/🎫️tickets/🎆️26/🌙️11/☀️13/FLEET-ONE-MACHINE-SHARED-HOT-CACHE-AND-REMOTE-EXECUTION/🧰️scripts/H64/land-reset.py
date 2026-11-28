import os,sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/"
CAPF="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🛂️root-capability/🟦️.ts"
APPLY="--apply" in sys.argv
f=R+"🟦️.ts";s=open(f,encoding="utf8").read()
if "requireNativeStorageRootCapabilityV1" in s: print("already");sys.exit(0)
a="const workspace=callerRepositoryRootV1(invocation.capabilities)"
assert s.count(a)==7,s.count(a)
s=s.replace(a,'const workspace=requireNativeStorageRootCapabilityV1(callerRepositoryRootV1(invocation.capabilities),"reset-partition").root')
b=' requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);if(!protocol.platforms.includes(process.platform as "darwin"))throw Error("Qualified inactive reset unavailable'
assert s.count(b)==1
s=s.replace(b,' requireNativeStorageRootCapabilityV1(workspace,"reset-partition");'+b[1:])
c='partition:"graph"|"rust"|"products",directory:string,minimumFreeBytes?:number):Promise<boolean|"held">{\n'
assert s.count(c)==1
s=s.replace(c,c+' requireNativeStorageRootCapabilityV1(workspace,"grow-image");\n')
rel=os.path.relpath(CAPF,os.path.dirname(f))
s='import{requireNativeStorageRootCapabilityV1}from"%s";\n'%rel+s
print("ok",rel)
if APPLY:
    open(f+".h64tmp","w",encoding="utf8").write(s);os.replace(f+".h64tmp",f);print("applied")
