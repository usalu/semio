export async function writeJavascriptDependencyGenerationFilesV1(storeRoot:string,files:readonly Readonly<{name:string;bytes:Uint8Array}>[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<void>{
 const state:{unfinished?:string}={};
 try{
  await withArtifactRelativeDirectoryV1(storeRoot,checkpoint,async base=>{
   if(await base.entry(protocol.directory)===null){await base.withNewDirectory(protocol.directory,async child=>{await child.sync();});await base.sync();}
   await base.withDirectory(protocol.directory,async directory=>{
    for(const file of files){
     const existing=await directory.entry(file.name);
     if(existing){if(existing.kind!=="file"||existing.size!==BigInt(file.bytes.length))throw Error("Existing immutable generation file changed");continue;}
     state.unfinished=file.name;
     await directory.withNewFile(file.name,async handle=>{let offset=0;while(offset<file.bytes.length){checkpoint();offset+=handle.write(file.bytes,offset,Math.min(relativeWriteCeilingBytes,file.bytes.length-offset),offset);await yieldContinuation();}handle.sync();});
     state.unfinished=undefined;
     await directory.sync();
    }
   });
  });
 }catch(error){
  if(state.unfinished!==undefined)await removeUnfinishedGenerationLeafV1(storeRoot,state.unfinished).catch(()=>{});
  throw error;
 }
}
/** 🧽️ Removes the one leaf a failed write created through a fresh descriptor scope, because the failed scope retains its created leaf and refuses further children. */
async function removeUnfinishedGenerationLeafV1(storeRoot:string,name:string):Promise<void>{
 await withArtifactRelativeDirectoryV1(storeRoot,()=>{},base=>base.withDirectory(protocol.directory,async directory=>{const leaf=await directory.entry(name);if(leaf&&leaf.kind==="file")await directory.remove(name,leaf);}));
}
