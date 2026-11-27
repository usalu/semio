import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:70],s.count(old));s=s.replace(old,new)
    tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
C=B+"🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
s=open(C,encoding="utf8").read()
a=s.index("  const partition=budgets[target.partition],base=partition?join(context.workspace");b=s.index("  rows.push({id:target.id,directory:base,groups:outcome.plan.groups")
b2=s.index("\n",b)+1
block=s[a:b2]
new='''  const partition=budgets[target.partition],base=partition?join(context.workspace,partition.directory,"m",...(target.directory?.split("/")??[])):undefined;
  if(partition===undefined||base===undefined){if(!parsed.json)context.write(cacheTextV1(language,"dedupe.absent",{id:target.id,directory:target.directory??target.partition}));continue;}
  if(context.leaseDirectory===undefined)throw Error(cacheTextV1(language,"dedupe.leaseRequired"));
  const reader=createNativeImageSharedReaderV1({leaseDirectory:context.leaseDirectory,imageDirectory:join(context.workspace,partition.directory),signal:ports.signal,timeoutMs:CACHE_GC_LIMITS_V1.imageReadLeaseWaitMilliseconds});
  try{
   await ports.progress("dedupe:lease "+target.id);
   const present=await reader.run(()=>stat(base).then(row=>row.isDirectory(),()=>false));
   if(!present){if(!parsed.json)context.write(cacheTextV1(language,"dedupe.absent",{id:target.id,directory:target.directory??target.partition}));continue;}
   if(!parsed.json)context.write(cacheTextV1(language,"dedupe.target",{id:target.id,directory:base}));
   await ports.progress("dedupe:"+target.id);const before=parsed.dryRun?undefined:free(base);
   let outcome:Awaited<ReturnType<typeof dedupeTreeV1>>;
   try{outcome=await dedupeTreeV1({base,reader,minimumBytes:target.minimumBytes,quietMs:target.quietMs,maximumFiles:target.maximumFiles,dryRun:parsed.dryRun,language,write:parsed.json?()=>{}:context.write},ports,context.clone);}
   catch(error){
    if(ports.signal.aborted)throw error;
    const reason=String((error as Error).message??error);failed++;
    if(!parsed.json)context.write(cacheTextV1(language,"dedupe.failedTarget",{id:target.id,reason}));
    rows.push({id:target.id,directory:base,groups:0,files:0,clones:false,plannedReplacements:0,reclaimableBytes:0,replaced:0,freedBytes:0,volumeGainBytes:null,skipped:0,failed:[reason],partial:false});continue;
   }
   if(outcome.partial&&!parsed.json)context.write(cacheTextV1(language,"dedupe.partial",{id:target.id,count:target.maximumFiles}));
   const after=parsed.dryRun?undefined:free(base),gain=before!==undefined&&after!==undefined?Math.max(0,after-before):null;
   if(gain!==null)gained+=gain;else if(!parsed.dryRun&&outcome.clones){unmeasured++;if(!parsed.json)context.write(cacheTextV1(language,"dedupe.freeUnknown",{directory:base}));}
   replaced+=outcome.replaced;freed+=outcome.freedBytes;skipped+=outcome.skipped;failed+=outcome.failed.length;
   for(const message of outcome.failed)if(!parsed.json)context.write(target.id+": "+message);
   rows.push({id:target.id,directory:base,groups:outcome.plan.groups,files:outcome.plan.files,clones:outcome.clones,plannedReplacements:outcome.plan.replacements.length,reclaimableBytes:outcome.plan.reclaimableBytes,replaced:outcome.replaced,freedBytes:outcome.freedBytes,volumeGainBytes:parsed.dryRun?0:gain,skipped:outcome.skipped,failed:outcome.failed,partial:outcome.partial});
  }finally{await reader.close();}
'''
print(block[-700:])
s=s[:a]+new+s[b2:]
tmp=C+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,C)
patch(C,[
 ('import {withNativeImageSharedReadV1} from','import {createNativeImageSharedReaderV1,withNativeImageSharedReadV1} from'),
 ('machinePolicy?:MachinePolicyV1;freeBytes?(path:string):number|undefined;clone?(source:string,destination:string):Promise<boolean>}>;','machinePolicy?:MachinePolicyV1;freeBytes?(path:string):number|undefined;clone?(source:string,destination:string):Promise<boolean>;leaseDirectory?:string}>;'),
 ('{workspace:root,environment:process.env,write:writeResultLineV1,verify},ports);','{workspace:root,environment:process.env,write:writeResultLineV1,verify,leaseDirectory:directory},ports);'),
])
T=B+"⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json"
patch(T,[('    "dedupe.partial":','    "dedupe.leaseRequired": { "en": "The deduplication needs the lease directory of the repository to hold the shared lease of an image while it works", "de": "Die Deduplizierung braucht das Lease-Verzeichnis des Repositorys, um die geteilte Lease eines Images zu halten, solange sie arbeitet" },\n    "dedupe.failedTarget": { "en": "Skipping {id}: not deduplicated ({reason})", "de": "Überspringe {id}: nicht dedupliziert ({reason})" },\n    "dedupe.partial":')])
print("command ok")
