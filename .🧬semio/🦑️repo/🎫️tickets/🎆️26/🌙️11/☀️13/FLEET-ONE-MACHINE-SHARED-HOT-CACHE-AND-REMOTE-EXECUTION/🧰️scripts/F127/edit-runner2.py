import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:80],s.count(old));s=s.replace(old,new)
    tmp=path+".f127tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
R=B+"🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts"
patch(R,[
('export async function readImageEnvelopeV1(directory:string):Promise<number|null>{\n try{const value=JSON.parse(await readFile(join(directory,"image.json"),"utf8"))','export async function readImageEnvelopeV1(directory:string,images?:CacheImagesV1):Promise<number|null>{\n const reader=cacheImageReaderOfV1(images,directory);\n try{const value=JSON.parse(await reader.run(()=>readFile(join(directory,"image.json"),"utf8")))'),
('envelopeBytes:await readImageEnvelopeV1(directory),','envelopeBytes:await readImageEnvelopeV1(directory,ports.images),'),
('  let present=false;for(const path of paths)if(await exists(path,cacheImageReaderOfV1(ports.images,path))){present=true;break;}','  let present=false;for(const path of paths){const reader=cacheImageReaderOfV1(ports.images,path);if(await exists(path,reader)){present=true;break;}}'),
('for(const entry of await undeclaredEntriesV1(workspace,ledger,root)){','for(const entry of await undeclaredEntriesV1(workspace,ledger,root,ports.images)){'),
('names=await undeclaredEntriesV1(workspace,ledger,root);','names=await undeclaredEntriesV1(workspace,ledger,root,ports.images);'),
('export async function undeclaredEntriesV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1):Promise<readonly string[]>{\n const base=cacheRootDirectoryV1(workspace,ledger,root),found:string[]=[],pending=[""];','export async function undeclaredEntriesV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1,images?:CacheImagesV1):Promise<readonly string[]>{\n const base=cacheRootDirectoryV1(workspace,ledger,root),reader=cacheImageReaderOfV1(images,base),found:string[]=[],pending=[""];'),
('try{names=await readdir(prefix?join(base,...prefix.split("/")):base);}catch(error){if(prefix&&','try{names=await reader.run(()=>readdir(prefix?join(base,...prefix.split("/")):base));}catch(error){if(prefix&&'),
('   try{names=await readdir(base);}catch{continue;}\n   resolved.push({target:{id:retirement.root+":"+retirement.directory,','   const reader=cacheImageReaderOfV1(ports.images,base);try{names=await reader.run(()=>readdir(base));}catch{continue;}\n   resolved.push({target:{id:retirement.root+":"+retirement.directory,'),
('    const base=join(workspace,...stray.split("/")),real=await realpath(base).catch(()=>base),mounted=','    const base=join(workspace,...stray.split("/")),reader=cacheImageReaderOfV1(ports.images,base),real=await reader.run(()=>realpath(base).catch(()=>base)),mounted='),
('    if(!(await lstat(base).catch(()=>undefined))?.isDirectory())continue;\n    resolved.push({target:{id:"stray:"+stray,kind:"retired",budgetBytes:null,unusedAgeMs:0,quietMs:ledger.strays.quietMs,units:await observe(base,await readdir(base).catch(()=>[] as string[]),','    if(!(await reader.run(()=>lstat(base).catch(()=>undefined)))?.isDirectory())continue;\n    resolved.push({target:{id:"stray:"+stray,kind:"retired",budgetBytes:null,unusedAgeMs:0,quietMs:ledger.strays.quietMs,units:await observe(base,await reader.run(()=>readdir(base).catch(()=>[] as string[])),'),
])
print("runner2 ok")
