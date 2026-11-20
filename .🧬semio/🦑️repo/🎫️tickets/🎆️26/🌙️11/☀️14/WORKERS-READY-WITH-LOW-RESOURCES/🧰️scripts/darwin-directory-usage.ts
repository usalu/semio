import {closeSync,linkSync,lstatSync,mkdirSync,openSync,symlinkSync,truncateSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";

const [modulePath,tree]=process.argv.slice(2);
if(process.platform!=="darwin"||!modulePath||!tree)throw Error("Darwin metering module and private fixture directory required");
const root=resolve(tree),{measureNativeDirectoryUsageAsyncV1,measureNativeDirectoryUsageV1}=await import(pathToFileURL(resolve(modulePath)).href);
mkdirSync(root);
const data=Buffer.alloc(1024,47),large=Buffer.alloc(8192,23);
for(let shard=0;shard<8;shard++){
 const group=join(root,"g"+shard);mkdirSync(group);
 for(let partition=0;partition<24;partition++){
  const directory=join(group,"p"+partition);mkdirSync(directory);
  for(let index=0;index<128;index++)writeFileSync(join(directory,"f"+index),index%3===0?large:data);
  const origin=join(directory,"f1"),alias=join(directory,"alias");
  linkSync(origin,alias);
  symlinkSync("missing",join(directory,"dead"));
  if(partition%6===0){
   const sparse=join(directory,"sparse");
   closeSync(openSync(sparse,"wx"));truncateSync(sparse,8*1024*1024);
   symlinkSync("..",join(directory,"up"));
  }
 }
}
const external=join(root,"g0","external");symlinkSync(process.execPath,external);
const file=join(root,"g0","p0","f1"),linked=lstatSync(file,{bigint:true}),sparse=lstatSync(join(root,"g0","p0","sparse"),{bigint:true}),link=lstatSync(external,{bigint:true});
const oracle=Bun.spawn(["python3","-I","-c",`
import json, os, stat, sys
stack=[sys.argv[1]]; entries=allocated=logical=0; seen=set()
while stack:
 p=stack.pop(); r=os.lstat(p); entries+=1
 if stat.S_ISREG(r.st_mode):
  if r.st_nlink>1:
   key=(r.st_dev,r.st_ino)
   if key in seen: continue
   seen.add(key)
  logical+=r.st_size
 allocated+=r.st_blocks*512
 if stat.S_ISDIR(r.st_mode): stack.extend(os.path.join(p,n) for n in os.listdir(p))
print(json.dumps(dict(allocatedBytes=allocated,logicalBytes=logical,entries=entries)))
`,root],{stdout:"pipe",stderr:"inherit"});
const expected=JSON.parse(await new Response(oracle.stdout).text());
if(await oracle.exited!==0)throw Error("Independent filesystem lstat oracle failed");
const status=[];
for(let round=0;round<3;round++){
 for(const [method,measure] of [["async",()=>measureNativeDirectoryUsageAsyncV1(root)],["sync",()=>measureNativeDirectoryUsageV1(root)]] as const){
  const before=performance.now(),cpu=process.cpuUsage();
  const value=await measure(),spent=process.cpuUsage(cpu),milliseconds=performance.now()-before;
  if(JSON.stringify(value)!==JSON.stringify(expected))throw Error("Native reading differs from the independent oracle");
  status.push({round:round+1,method,milliseconds,processCpuMilliseconds:(spent.user+spent.system)/1000});
 }
}
const stop=new AbortController(),marker=Error("fixture cancellation"),before=performance.now();
const timer=setTimeout(()=>stop.abort(marker),8);
let canceled=false;
try{await measureNativeDirectoryUsageAsyncV1(root,{signal:stop.signal});}catch(error){if(error!==marker)throw error;canceled=true;}
finally{clearTimeout(timer);}
if(!canceled)throw Error("Expected an actual canceled measurement");
console.log(JSON.stringify({platform:process.platform,architecture:process.arch,bun:Bun.version,root,pathDevice:String(linked.dev),expected,examples:{linkedNames:Number(linked.nlink),sparseLogicalBytes:Number(sparse.size),sparseBlocks:Number(sparse.blocks),externalLinkBlocks:Number(link.blocks)},status,cancellationMilliseconds:performance.now()-before},null,2));
