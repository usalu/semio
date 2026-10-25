import{test,expect}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync}from"node:fs";
import{join}from"node:path";
import{Database}from"bun:sqlite";
const P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/",tools=import.meta.dir;
const lines=(child:Bun.Subprocess<"pipe","pipe","inherit">)=>{const reader=child.stdout.getReader(),decoder=new TextDecoder();let buffer="";return async(pattern:RegExp,limit=120000):Promise<string>=>{const end=Date.now()+limit;for(;;){const at=buffer.indexOf("\n");if(at>=0){const line=buffer.slice(0,at);buffer=buffer.slice(at+1);console.log("  |",line);if(pattern.test(line))return line;continue;}if(Date.now()>end)throw Error("timeout waiting for "+pattern);const{value,done}=await reader.read();if(done)throw Error("stream ended waiting for "+pattern);buffer+=decoder.decode(value);}};};
test("a queued exclusive-ticket recreate swaps a scratch image to a larger size without dropping a held lease",async()=>{
 const{withBuildTestInvocationV1}=await import(P+"🧪️testing/📥️invocation/🟦️.ts"),routing=await import(P+"📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{openNativePlatformStorageV1,retireNativePlatformStorageV1}=await import(P+"📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts"),leases=await import(P+"🔒️leases/🟦️.ts");
 await withBuildTestInvocationV1(async original=>{
  const scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"sw-")),directory=join(scratch,"l"),control_=join(scratch,"c"),MiB=1048576,small={capacity:40*MiB,fs:39*MiB},large={capacity:48*MiB,fs:47*MiB};mkdirSync(control_);
  const env=routing.scriptInvocationEnvironment(original,{...process.env}),control=routing.createIssuedResourceControlV1(original,"swap:driver",1800000);
  const holder=Bun.spawn([process.execPath,tools+"/swap-holder.ts",directory,control_,String(small.fs),String(small.capacity)],{env,stdin:"pipe",stdout:"pipe",stderr:"inherit"}),hold=lines(holder as any);
  let image:any,tool:Bun.Subprocess<"pipe","pipe","inherit">|undefined;
  try{
   const ready=await hold(/^READY /),data=ready.slice(6);
   expect(JSON.parse(readFileSync(join(directory,"image.json"),"utf8")).maximumPhysicalBytes).toBe(small.capacity);
   const heldPath=leases.resourceLeasePathV1(data,"swap:held"),probe=()=>{const db=new Database(heldPath,{readwrite:true});try{db.exec("PRAGMA busy_timeout=0;BEGIN EXCLUSIVE;ROLLBACK");return false;}catch{return true;}finally{db.close();}};
   expect(probe()).toBe(true);
   tool=Bun.spawn(["python3",tools+"/recreate-leases.py","--yes","--directory="+directory,"--control="+control_,"--want="+large.capacity,"--birth-wait=20","--limit=300"],{env,stdout:"pipe",stderr:"inherit"});
   const toolLines=lines(tool as any);
   await toolLines(/ticket queued/);
   await Bun.sleep(4000);
   expect(tool.exitCode).toBeNull();expect(holder.exitCode).toBeNull();expect(probe()).toBe(true);
   expect(JSON.parse(readFileSync(join(directory,"image.json"),"utf8")).maximumPhysicalBytes).toBe(small.capacity);
   holder.stdin.end();await hold(/^CLOSED/);expect(await holder.exited).toBe(0);
   await toolLines(/exclusive lease granted/);await toolLines(/moved aside/);await toolLines(/lease released/);
   image=await openNativePlatformStorageV1({directory,leaseDirectory:control_,filesystemBytes:large.fs,maximumPhysicalBytes:large.capacity,control,maximumLeaseDatabaseBytes:16384});
   await toolLines(/born at the declared size/);expect(await tool.exited).toBe(0);
   const env2=JSON.parse(readFileSync(join(directory,"image.json"),"utf8"));
   expect(env2.maximumPhysicalBytes).toBe(large.capacity);
   const asides=readdirSync(scratch).filter(name=>name.startsWith("l.retired-"));expect(asides.length).toBe(1);expect(existsSync(join(scratch,asides[0]!,"fixed.dmg"))).toBe(true);
   expect(readdirSync(join(image.root)).includes("data")).toBe(false);
   const info=Bun.spawnSync(["hdiutil","info"]).stdout.toString();expect(info.includes(join(scratch,asides[0]!))).toBe(false);expect(info.includes(join(directory,"fixed.dmg"))).toBe(true);
   console.log("  | swap verified: envelope",env2.maximumPhysicalBytes,"aside",asides[0]);
  }finally{
   try{holder.stdin.end();}catch{}try{holder.kill();}catch{}try{tool?.kill();}catch{}
   try{await image?.close();}catch{}
   try{await retireNativePlatformStorageV1({directory,leaseDirectory:control_,filesystemBytes:large.fs,maximumPhysicalBytes:large.capacity,control,maximumLeaseDatabaseBytes:16384} as any);}catch(error){console.log("  | retire failed",String(error).slice(0,200));}
   Bun.spawnSync(["hdiutil","detach",join(directory,"m"),"-force"]);rmSync(scratch,{recursive:true,force:true});
  }
 });
},900000);
