import sys,os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/"
check=len(sys.argv)>1 and sys.argv[1]=="check"
tests_only=len(sys.argv)>1 and sys.argv[1]=="tests"
def write(p,s):
    if check:return
    t=p+".h43.tmp";open(t,"w",encoding="utf-8").write(s);os.replace(t,p)
p=B+"🟨️.mjs";s=open(p,encoding="utf-8").read()
product=[
 ("import { repositoryCachePartitionNativeImageV1, repositoryLeaseDirectoryV1 } from","import { repositoryCachePartitionNativeImageV1, repositoryLeaseDirectoryV1, withRepositoryCommandCustodyV1 } from"),
 (" try{\n  await leases.recheck();const check=()=>{checkResourceLeaseControl(phaseControl);"," try{return await withRepositoryCommandCustodyV1(workspace,leases,async()=>{\n  await leases.recheck();const check=()=>{checkResourceLeaseControl(phaseControl);"),
 ("   return result.value;\n  }\n }catch(error){leaseFailed=true;throw error;}","   return result.value;\n  }\n  });}catch(error){leaseFailed=true;throw error;}"),
] 
for old,new in product:
    if s.count(old)!=1:sys.exit("anchor %d: %s"%(s.count(old),old[:60]))
    s=s.replace(old,new)
if not tests_only:write(p,s)
key="openRepositoryCachePartitionV1:async"
port="withRepositoryCommandCustodyV1:(_workspace:string,_image:unknown,body:()=>Promise<unknown>)=>body(),"
for rel,n in [("🚪️admission",1),("🎛️phase",1),("🔒️lifetime",1),("",3),("🤝️shared-writer",1)]:
    f=B+"🧪️tests/"+(rel+"/" if rel else "")+"🟦️.ts";s=open(f,encoding="utf-8").read()
    if s.count(key)!=n:sys.exit("%s count %d != %d"%(rel,s.count(key),n))
    write(f,s.replace(key,port+key))
print("checked" if check else "applied")
