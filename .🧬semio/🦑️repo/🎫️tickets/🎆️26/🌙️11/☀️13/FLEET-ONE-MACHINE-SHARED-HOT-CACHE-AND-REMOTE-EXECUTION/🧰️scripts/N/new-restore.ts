/** 📥️ Restores the roots of an install state from the group store: verifies the manifest's signature, lockfile binding and index, extracts every unit blob (its SHA-256 is checked while it streams into `tar`, so a corrupt blob is never trusted), recreates links and empty directories, then proves the tree equals the publisher's per-file fingerprint and the installer's own shape. Any disagreement removes what was placed and answers `false`, which costs one real installation and never a wrong tree. */
export async function fleetInstallRestoreV1(request:FleetInstallRequestV1,manifest?:FleetInstallManifestV1):Promise<boolean>{
 const {context}=request,directory=directoryOf(context),key=fleetInstallKeyV1(request.projectionSha256,request.runtime),found=manifest??await fleetInstallManifestV1(context,key);
 if(directory===undefined||!found||found.projectionSha256!==request.projectionSha256||JSON.stringify(found.runtime)!==JSON.stringify(request.runtime))return false;
 for(const root of found.roots)if(await lstat(join(request.directory,...root.split("/"))).then(()=>true,()=>false))return false;
 const machine=context.root===undefined?undefined:readFleetMachineV1(context.root),currentLock=await lockOf(request.directory);
 if(machine===undefined||machine.storeRoot===undefined||currentLock===undefined)return false;
 if(found.lock.sha256!==currentLock.sha256||found.lock.packages!==currentLock.packages){request.event?.({stage:"skip",detail:"the pack was installed from other packages than this lockfile pins"});return false;}
 const registry=createFleetRegistryV1({storeRoot:machine.storeRoot,group:machine.group,pinsPath:join(context.root!,".cache","f","identity","pins-install.json")});
 const {publisher,...bare}=found;
 if(!await verifyFleetDocumentV1(registry,publisher.origin,"cache-write","install-pack",signedText(bare,publisher.origin),publisher.signature)){request.event?.({stage:"skip",detail:"the pack is not signed by a registered fleet origin"});return false;}
 const index=await fleetInstallIndexV1(directory,found);
 if(index===undefined){request.event?.({stage:"skip",detail:"the pack index is missing or corrupt"});return false;}
 const placed=found.roots.map(root=>join(request.directory,...root.split("/")));
 const undo=async()=>{for(const path of [...placed].reverse())await request.remove(path).catch(()=>{});};
 try{
  request.event?.({stage:"restore",detail:"extract",bytes:found.index.packedBytes});
  for(const path of index.directories)await mkdir(join(request.directory,...path.split("/")),{recursive:true});
  await poolV1(index.units,policy.installPackConcurrency,async unit=>{
   request.check();
   const file=blobPathOf(directory,unit.sha256),facts=await stat(file);
   if(!facts.isFile()||facts.size!==unit.bytes)throw Error("A pack blob is missing or has the wrong size");
   const child=runTar(["-xf","-","--no-same-owner","-C",request.directory],request.directory,request.check,["pipe","ignore","pipe"]),hash=createHash("sha256");
   child.stderr?.resume();
   const exit=closed(child);
   const verified=async function*(source:AsyncIterable<Buffer>){for await(const chunk of source){hash.update(chunk);yield chunk;}};
   await pipeline(createReadStream(file,{highWaterMark:1<<20}),verified,createGunzip(),child.stdin!);
   if(await exit!==0)throw Error("tar refused a pack blob");
   if(hash.digest("hex")!==unit.sha256)throw Error("A pack blob does not match its digest");
  });
  request.check();
  const platform=process.platform,source=fleetInstallLinkTextV1(found.storeRoot,platform).replace(/[\\/]+$/u,""),target=request.storeRoot.replace(/[\\/]+$/u,"");
  for(const link of index.links){
   request.check();
   const path=join(request.directory,...link.path.split("/")),text=fleetInstallLinkTextV1(link.target,platform);
   await mkdir(dirname(path),{recursive:true});
   const next=source!==target&&text.startsWith(source+"/")?target+text.slice(source.length):link.target;
   await symlink(platform==="win32"?next.replaceAll("/","\\"):next,path,platform==="win32"?"junction":undefined);
  }
  await rewriteFleetInstallLinksV1(request.directory,found.roots,found.storeRoot,request.storeRoot,request.check);
  if(await fleetInstallFingerprintV1(request.directory,found.roots,request.storeRoot,request.check)!==found.fingerprint)throw Error("Restored tree differs from its publisher");
  const shape=await request.scan(),expected=found.shape;
  if(found.storeRoot===request.storeRoot&&(shape.entries!==expected.entries||shape.directories!==expected.directories||shape.files!==expected.files||shape.links!==expected.links||shape.sha256!==expected.sha256))throw Error("Restored tree differs from its publisher");
  if(found.storeRoot!==request.storeRoot&&(shape.entries!==expected.entries||shape.directories!==expected.directories||shape.files!==expected.files||shape.links!==expected.links))throw Error("Restored tree differs from its publisher");
  request.event?.({stage:"restore",detail:key});
  return true;
 }catch(error){
  await undo();request.check();
  request.event?.({stage:"skip",detail:`restore failed: ${error instanceof Error?error.message:String(error)}`});
  return false;
 }
}
