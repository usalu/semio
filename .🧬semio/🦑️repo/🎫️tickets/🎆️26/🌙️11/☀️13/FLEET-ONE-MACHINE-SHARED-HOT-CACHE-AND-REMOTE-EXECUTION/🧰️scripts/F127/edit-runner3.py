import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:80],s.count(old));s=s.replace(old,new)
    tmp=path+".f127tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
R=B+"🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts"
patch(R,[
('outer=await reader.run(()=>lstat(directory));}catch{return{state:"absent"','outer=await reader.run(()=>lstat(directory));}catch(error){if(cacheLeaseRefusedV1(error))throw error;return{state:"absent"'),
('try{child=await reader.run(()=>lstat(inner));}catch{child=undefined;}','try{child=await reader.run(()=>lstat(inner));}catch(error){if(cacheLeaseRefusedV1(error))throw error;child=undefined;}'),
('try{names=await reader.run(()=>readdir(queue));}catch{continue;}','try{names=await reader.run(()=>readdir(queue));}catch(error){if(cacheLeaseRefusedV1(error))throw error;continue;}'),
('try{return(await reader.run(()=>readdir(root))).filter(','try{return(await reader.run(()=>readdir(root))).filter('),
('.map(name=>join(root,name));}catch{return[];}};','.map(name=>join(root,name));}catch(error){if(cacheLeaseRefusedV1(error))throw error;return[];}};'),
('try{names=await reader.run(()=>readdir(prefix?join(base,...prefix.split("/")):base));}catch(error){if(prefix&&','try{names=await reader.run(()=>readdir(prefix?join(base,...prefix.split("/")):base));}catch(error){if(cacheLeaseRefusedV1(error))throw error;if(prefix&&'),
])
print("runner3 ok")
