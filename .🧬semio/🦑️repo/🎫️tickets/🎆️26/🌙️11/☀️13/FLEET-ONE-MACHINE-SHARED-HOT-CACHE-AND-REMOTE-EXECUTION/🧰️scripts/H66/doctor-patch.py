"""H66: doctor wiring. usage: doctor-patch.py <doctor storage probe 🟦️.ts> (edits in place, atomic)."""
import os,sys
p=sys.argv[1]
s=open(p).read()
def once(old,new):
    global s
    assert s.count(old)==1,(s.count(old),old[:80])
    s=s.replace(old,new)
PRE="../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/"
once('import {requireMaterializedPathV1} from','import {createNativeImageDetachPortV1} from "'+PRE+'🔁️detach/🟦️.ts";\nimport {NATIVE_IMAGE_GROWTH_V1,growNativeImageShardPlannedV1,nativeImageGrowthTextV1} from "'+PRE+'🪜️growth/🟦️.ts";\nimport {createNativeImageShardDarwinLifecycleV1,measureNativeImageShardFreeV1,nativeImageShardFoldV1,readNativeImageShardLedgerV1,type NativeImageShardLifecycleV1,type NativeImageShardRuleV1} from "'+PRE+'🧩️shard/🟦️.ts";\nimport {requireMaterializedPathV1} from')
once(' return Object.freeze(rows);\n}\n\n/** 📈️ The repair of the images below their need',''' if(!dryRun){
  const rust=(budget.partitions as unknown as Record<string,{directory:string;imageShards?:NativeImageShardRuleV1}>).rust;
  if(rust?.imageShards?.enabled)try{rows.push(...await growShardImagesV1(ports,requireMaterializedPathV1(directoryOf(resolve(ports.root),rust.directory)),rust.imageShards));}
  catch(error){if(ports.signal.aborted)throw error;rows.push({id:"r",state:"failed",note:`r: ${(error instanceof Error?error.message:String(error)).split("\\n")[0]!.slice(0,200)}`});}
 }
 return Object.freeze(rows);
}

/** 🪴️ Grows the shard images of the compiler partition (the Cargo home, the lane images) between stages, never while a build holds them: the exclusive lease of the partition image and of each shard is tried once without waiting, the processes that keep files open on the shard are counted (any is `held`), the plan never shrinks, never passes the cap and leaves the host its keep. A shard that is in use waits for the next quiet moment; the image of a build is never detached. Only macOS has images. @see ../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪜️growth/🟦️.ts */
export async function growShardImagesV1(ports:DoctorPortsV1,directory:string,rule:NativeImageShardRuleV1,world:Readonly<{lifecycle?:NativeImageShardLifecycleV1;measure?:typeof measureNativeImageShardFreeV1}>={}):Promise<readonly ImageGrowRowV1[]>{
 if(ports.platform!=="darwin")return [];
 let live;try{live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory));}catch{return [];}
 if(live.size===0)return [];
 const both=(key:"held"|"unfit",values:Readonly<Record<string,number|string>>)=>nativeImageGrowthTextV1("en",key==="held"?"wait":"hostShort",values)+" / "+nativeImageGrowthTextV1("de",key==="held"?"wait":"hostShort",values);
 const run=(program:string,args:readonly string[])=>ports.command(program,[...args]),lifecycle=world.lifecycle??createNativeImageShardDarwinLifecycleV1({run}),detach=createNativeImageDetachPortV1({cwd:directory,run,sleep:async()=>{}}),measure=world.measure??measureNativeImageShardFreeV1,rows:ImageGrowRowV1[]=[];
 const home=await tryImageLeaseV1(ports,"r",directory);
 if(home==="busy"){const values={image:"r",to:0};return [{id:"r",state:"skipped",kind:"held",values,note:both("held",values)}];}
 try{
  for(const id of [...live.keys()].sort()){
   ports.signal.throwIfAborted();
   const name="r/"+id,before=live.get(id)!.capacityBytes;let free=0;
   try{
    const out=await growNativeImageShardPlannedV1({directory,rule,platform:ports.platform,id,measure:layout=>{const row=measure(layout);free=row?.hostFreeBytes??0;return row&&{usedBytes:row.usedBytes,hostFreeBytes:row.hostFreeBytes};},lease:async layout=>{const lease=await tryImageLeaseV1(ports,id,layout.directory);return lease==="busy"?undefined:()=>lease?.release();},holders:async layout=>(await detach.holders(layout.mountPoint)).length,now:ports.now,lifecycle,lock:async body=>body()});
    if(out.kind==="grown")rows.push({id:name,state:"grown",addedBytes:out.capacityBytes-before});
    else if(out.kind==="wait"){const values={image:name,to:0};rows.push({id:name,state:"skipped",kind:"held",values,note:both("held",values)});}
    else if(out.kind==="refused"&&out.reason==="host-short"){const values={image:name,free:Math.floor(free/1048576),keep:Math.floor(NATIVE_IMAGE_GROWTH_V1.hostKeepBytes/1048576)};rows.push({id:name,state:"skipped",kind:"unfit",values,note:both("unfit",values)});}
   }catch(error){if(ports.signal.aborted)throw error;rows.push({id:name,state:"failed",note:`${name}: ${(error instanceof Error?error.message:String(error)).split("\\n")[0]!.slice(0,200)}`});}
  }
 }finally{if(home!==undefined)home.release();}
 return Object.freeze(rows);
}

/** 📈️ The repair of the images below their need''')
tmp=p+".h66"
open(tmp,"w").write(s);os.replace(tmp,p)
print("patched",p)
