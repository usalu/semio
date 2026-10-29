import{test,expect}from"bun:test";
import{constants}from"node:fs";
import{mkdtemp,open,lstat,writeFile,rm}from"node:fs/promises";
import{join,dirname}from"node:path";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";
import{withArtifactDescriptorV1}from"../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import{assertNativeSelectedDescriptorCustodyV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts";

declareBuildTestEnvelopeV1({owner:"test:selected-native-descriptor-census",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
test("owned readonly guards remain live through complete selected descriptor commands",async()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:selected-native-census",30000),directory=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"sel-")),rows:any[]=[];
 try{await withArtifactDescriptorV1(()=>open(directory,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK),async handle=>{
  const before=await handle.stat({bigint:true}),guard={selection:"filesystem"as const,pid:process.pid,files:[{fd:handle.fd,path:directory,type:"DIR"as const}]};
  for(const [name,args]of[["empty-recursive",["+D",directory]],["empty-exact",["-f","--",directory]],["populated-recursive",["+D",directory]],["populated-search-misses",["-Q","+D",directory]],["missing-positive-guard",["-Q","-f","--",join(directory,"closed-file")]],["invalid-option",["-Q","--invalid-semio-option"]]]as const){
   if(name==="populated-recursive")await writeFile(join(directory,"closed-file"),"original bounded fixture");
   const command=["-nP","-F0pcfatn",...args],result=await captureOwnedCommandResultV1("lsof",command,dirname(directory),{control,output:{maximumBytes:32768,maximumLines:256}}),opened=await handle.stat({bigint:true}),named=await lstat(directory,{bigint:true});
   const stable=before.dev===opened.dev&&before.ino===opened.ino&&before.dev===named.dev&&before.ino===named.ino&&opened.isDirectory()&&named.isDirectory()&&!named.isSymbolicLink();
   let accepted=true,refusal="";try{assertNativeSelectedDescriptorCustodyV1({...result,reason:"exit"},guard);}catch(error){accepted=false;refusal=String((error as Error).message);}
   const row={name,command:["lsof",...command],pid:process.pid,fd:handle.fd,stable,status:result.status,signal:result.signal,stdoutBytes:Buffer.byteLength(result.stdout),stdout:result.stdout,stdoutHex:Buffer.from(result.stdout).toString("hex"),stderr:result.stderr,accepted,refusal};rows.push(row);process.stdout.write("[DEBUG] "+JSON.stringify(row)+"\n");
  }
 });expect(rows.every(row=>row.stable)).toBe(true);expect(rows.map(row=>[row.name,row.status,row.accepted])).toEqual([["empty-recursive",0,true],["empty-exact",0,true],["populated-recursive",1,false],["populated-search-misses",0,true],["missing-positive-guard",0,false],["invalid-option",1,false]]);
 }finally{await rm(directory,{recursive:true,force:true});}
}),45000);
