/** 🧩️ One-time operator tool of the image shard switchover (see the section "Sharded images" of 📓️cache-image-space-return-2026-11-23.md). Run only in the idle window, with the bun of the repository tool store:
 *   bun run shard-switchover.ts status  <workspace> <partition>
 *   bun run shard-switchover.ts birth   <workspace> <partition> <logical path below the mount, e.g. u/l/pcab777>...
 *   bun run shard-switchover.ts repair  <workspace> <partition>
 *   bun run shard-switchover.ts retire  <workspace> <partition> <shard id>...
 * The rule is read from the partition policy and forced to `enabled`, because the policy flag is flipped only after the shards exist. */
import{spawnSync}from"node:child_process";
import{join,resolve}from"node:path";
import policy from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"with{type:"json"};
import{createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,measureNativeImageShardV1,nativeImageShardAggregateV1,nativeImageShardFoldV1,nativeImageShardLayoutsV1,readNativeImageShardLedgerV1,repairNativeImageShardsV1,requireNativeImageShardRuleV1,retireNativeImageShardV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
const[,,command,workspace,partition,...rest]=process.argv;
const declared=(policy.partitions as Record<string,{directory:string;imageShards?:unknown}>)[partition??""];
if(!command||!workspace||!declared?.imageShards)throw Error("usage: <status|birth|repair|retire> <workspace> <partition with imageShards> [arguments]");
const directory=resolve(workspace,declared.directory),rule=requireNativeImageShardRuleV1({...(declared.imageShards as object),enabled:true});
const env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const lifecycle=createNativeImageShardDarwinLifecycleV1({run:async(program,args)=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr+row.stdout);return row.stdout;}});
let tail=Promise.resolve();const lock=<T>(body:()=>Promise<T>):Promise<T>=>{const next=tail.then(body);tail=next.then(()=>undefined,()=>undefined);return next;};
const hostFree=()=>{const row=spawnSync("/bin/df",["-k",directory],{encoding:"utf8",env}).stdout.trim().split("\n").at(-1)!.split(/\s+/u);return Number(row[3])*1024;};
if(command==="status"){
 const layouts=nativeImageShardLayoutsV1(directory,nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory))),rows=layouts.map(measureNativeImageShardV1);
 console.log(JSON.stringify({directory,layouts:layouts.map(layout=>layout.mountPoint),aggregate:rows.every(Boolean)?nativeImageShardAggregateV1(rows as never):null},null,1));
}else if(command==="birth")for(const path of rest)console.log(path,JSON.stringify(await ensureNativeImageShardV1({directory,rule,platform:process.platform,path,hostFreeBytes:hostFree,now:Date.now,lifecycle,lock})));
else if(command==="repair")console.log(JSON.stringify(await repairNativeImageShardsV1({directory,now:Date.now,lifecycle,lock})));
else if(command==="retire")for(const id of rest)console.log(id,await retireNativeImageShardV1({directory,id,now:Date.now,lifecycle,lock}));
else throw Error("Unknown command "+command);
