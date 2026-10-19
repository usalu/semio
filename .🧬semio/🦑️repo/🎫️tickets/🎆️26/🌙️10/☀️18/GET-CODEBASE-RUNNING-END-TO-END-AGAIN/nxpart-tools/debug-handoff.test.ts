import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
const root="/Users/ueli/Documents/semio",S=join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage");
const {createIssuedResourceControlV1,receiveScriptProcessInvocation}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const {openNativePlatformStorageV1}=await import(join(S,"🌐️platform/🟦️.ts"));
const {nativeImageInformationV1}=await import(join(S,"🍎️image/📥️information/🟦️.ts"));
const run=(program:string,args:readonly string[])=>spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
const attached=(scratch:string):string[]=>((nativeImageInformationV1(run("/usr/bin/hdiutil",["info","-plist"]).stdout).images as Record<string,unknown>[])??[]).map(row=>String(row["image-path"])).filter(path=>path.startsWith(scratch));
test("[DEBUG] handoff fault",async()=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"fb-"))),directory=join(scratch,"img"),leaseDirectory=join(scratch,"leases");let born:boolean|undefined,message="",mounted:string[]=[],present:boolean|undefined,entries:string[]=[];mkdirSync(leaseDirectory);
 try{try{await receiveScriptProcessInvocation(process.env,async (original:any)=>{
   const control=createIssuedResourceControlV1(original,"cache:failed-birth",0);
   try{const image=await openNativePlatformStorageV1({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control});born=true;await image.close();}catch(error){born=false;message=String(error);}
   mounted=attached(scratch);present=existsSync(directory);
  });}catch(error){message+=" | outer "+String(error);}
  console.log("[DEBUG] born="+born+" message="+message.slice(0,300)+" mounted="+JSON.stringify(mounted)+" present="+present);
 }finally{for(const path of attached(scratch))run("/usr/bin/hdiutil",["detach","-force",join(path,"..","m")]);rmSync(scratch,{recursive:true,force:true});}
},120000);
