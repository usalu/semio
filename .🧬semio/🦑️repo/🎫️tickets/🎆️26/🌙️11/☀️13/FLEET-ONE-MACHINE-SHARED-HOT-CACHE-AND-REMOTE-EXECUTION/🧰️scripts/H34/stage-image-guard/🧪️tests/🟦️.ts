/** 🖼️ Contract: a cache image that is nearly full is told before a stage starts and while it runs (floor and need are shares of the image, clamped, never above a quarter of it), the worst image is named first, a failed write names its image from the compiler's own words, the collectors of the image are the ones below its mount, and a full image is looked at again soon. Cross-checked against an independent Python model whose numbers are written from the schema's rule, not read from the code.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_IMAGE_POLICY_V1 as P,readyImageFactsV1,readyImageFloorBytesV1,readyImageGuardV1,readyImageHostLimitedV1,readyImageNeedBytesV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1}from"../🟦️.ts";

const MB=1048576,GB=1024*MB;
const sizes=[62,512,1535,2047,8191,12287,30824,36863,40959,98304].map(megabytes=>megabytes*MB);

test("floor and need of an image equal an independent Python model",()=>{
 const script=`
import json,sys
p=json.loads(sys.argv[1]);rows=json.loads(sys.argv[2]);out=[]
MB=1048576
def clamp(v,lo,hi): return max(lo,min(hi,v))
for total in rows:
    floor=min(total*p["floorShareBasisPoints"]//10000,clamp(total*p["floorBasisPoints"]//10000,p["floorMinimumMegabytes"]*MB,p["floorMaximumMegabytes"]*MB))
    need=max(floor,clamp(total*p["needBasisPoints"]//10000,p["needMinimumMegabytes"]*MB,p["needMaximumMegabytes"]*MB))
    out.append([floor,need])
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(P),JSON.stringify(sizes)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout) as number[][];
 expect<unknown>(sizes.map(total=>[readyImageFloorBytesV1(total),readyImageNeedBytesV1(total)])).toEqual(model);
 for(const[floor,need]of model)expect(need).toBeGreaterThanOrEqual(floor);
});

test("the real images of the fleet: m12 rust image with 47 MB free is short, a healthy one is not",()=>{
 const image=(partition:string,free:number,total:number)=>({partition,path:`/w/.cache/b/${partition}/m`,freeBytes:free*MB,totalBytes:total*MB});
 const short=readyImageShortV1([image("g",2257,12287),image("n",930,1535),image("r",47,40959)])!;
 expect(short.partition).toBe("r");
 expect(short.needed).toBe(4096);
 expect(short.available).toBe(47);
 expect(short.total).toBe(40959);
 expect(readyImageShortV1([image("r",9267,56319),image("g",5083,10239),image("d",16217,36863)])).toBeNull();
 expect(readyImageShortV1([])).toBeNull();
});

test("the worst image is named first and the guard stops below the floor only",()=>{
 const image=(partition:string,free:number,total:number)=>({partition,path:`/w/.cache/b/${partition}/m`,freeBytes:free*MB,totalBytes:total*MB});
 expect(readyImageShortV1([image("g",600,12287),image("r",3000,40959)])!.partition).toBe("g");
 expect(readyImageShortV1([image("g",1000,12287),image("r",1000,40959)])!.partition).toBe("r");
 const hit=readyImageGuardV1([image("g",2000,12287),image("r",1600,40959)])!;
 expect(hit.partition).toBe("r");
 expect(hit.floor).toBe(1639);
 expect(readyImageGuardV1([image("r",1700,40959)])).toBeNull();
 expect(readyImageGuardV1([])).toBeNull();
});

test("a failed write names its image from the compiler's own words, never a control partition",()=>{
 const noSpace=["No space left on device","os error 28","ENOSPC"];
 const cargo=[
  "error: could not parse/generate dep info at: /Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/out/x.d",
  "Caused by:",
  "  failed to write `/Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/fingerprint/dep-lib-x`",
  "Caused by:",
  "  No space left on device (os error 28)",
 ];
 expect(readyImagePartitionOfLinesV1(cargo,noSpace)).toBe("r");
 expect(readyImagePartitionOfLinesV1(["error: error writing dependencies to `/Users/ueli/Documents/semio/.cache/b/g/m/inference/a.d`: No space left on device (os error 28)"],noSpace)).toBe("g");
 expect(readyImagePartitionOfLinesV1(["error writing `C:\\work\\.cache\\b\\d\\m\\j\\a`: There is not enough space on the disk"],["There is not enough space on the disk"])).toBe("d");
 expect(readyImagePartitionOfLinesV1(["No space left on device (os error 28)"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/l/m/data/x: No space left on device"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/r/m/x: permission denied"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/rr/mx/x: No space left on device"],noSpace)).toBeNull();
});

test("the images of a stage are the mounts the policy names for it, own volumes only, unknown volumes skipped",()=>{
 const root=mkdtempSync(join(tmpdir(),"ready-images-"));
 try{
  for(const name of["r","g","n","t","d","l","p"])mkdirSync(join(root,".cache","b",name,"m"),{recursive:true});
  const facts=(path:string)=>path.includes(`${join("b","g")}`)?null:path.includes(`${join("b","n")}`)?{freeBytes:10*MB,totalBytes:100*MB}:{freeBytes:30*GB,totalBytes:40*GB};
  const mounted=(path:string)=>!path.includes(`${join("b","t")}`);
  const workspace=readyImageFactsV1(root,"workspace",facts,P,mounted);
  expect(workspace.map(row=>row.partition)).toEqual(["r"]);
  expect(workspace[0]!.hostFreeBytes).toBe(30*GB);
  expect(readyImageFactsV1(root,"dependencies",facts,P,mounted).map(row=>row.partition)).toEqual(["d"]);
  expect(readyImageFactsV1(root,"tools",facts,P,mounted)).toEqual([]);
  expect(readyImageFactsV1(root,"native",facts,P,mounted)).toEqual([]);
  expect(readyImageFactsV1(root,"workspace",facts,P,()=>false)).toEqual([]);
  expect(readyImageFactsV1(join(root,"missing"),"workspace",facts)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("only the collectors below the mount work for the image, and their relief is the image's",()=>{
 const rows=[{id:"cargo",root:"/w/.cache/b/r/m/cargo",row:{relief:"image"}},{id:"tools",root:"/w/.cache/b/t/m/nx",row:{relief:"image"}},{id:"cas",root:"/h/.cache/semio/cas",row:{relief:"host"}},{id:"mount",root:"/w/.cache/b/r/m",row:{relief:"image"}},{id:"sibling",root:"/w/.cache/b/r/mm",row:{relief:"image"}}];
 const kept=readyImageWritersV1(rows,"/w/.cache/b/r/m");
 expect(kept.map(row=>row.id)).toEqual(["cargo","mount"]);
 expect(kept.every(row=>row.row.relief==="host")).toBe(true);
 expect(rows[0]!.row.relief).toBe("image");
 expect(readyImageWritersV1([{root:"C:\\w\\.cache\\b\\r\\m\\cargo",row:{relief:"image"}}],"C:\\w\\.cache\\b\\r\\m","\\")).toHaveLength(1);
});

test("a full image is looked at again after five minutes or when it gained 256 MB, and the retention frees more than the next write",()=>{
 expect(P.recheckMilliseconds).toBe(300000);
 const said={atMs:1000,availableMegabytes:47};
 expect(readyImageRecheckDueV1(said,47,1000+299999)).toBe(false);
 expect(readyImageRecheckDueV1(said,47,1000+300000)).toBe(true);
 expect(readyImageRecheckDueV1(said,47+255,2000)).toBe(false);
 expect(readyImageRecheckDueV1(said,47+256,2000)).toBe(true);
 expect(readyImageRetainBytesV1({freeBytes:47*MB,totalBytes:40959*MB})).toBe(Math.floor(40959*MB/10)-47*MB);
 expect(readyImageRetainBytesV1({freeBytes:4000*MB,totalBytes:40959*MB})).toBe(512*MB);
});

test("every purpose of the policy is named in English and German",()=>{
 for(const partition of Object.values(P.stages).flat()){
  const name=readyImagePurposeV1(partition);
  expect(name.en.length).toBeGreaterThan(1);
  expect(name.de.length).toBeGreaterThan(1);
 }
 expect(readyImagePurposeV1("zz")).toEqual({en:"zz",de:"zz"});
 expect(P.excluded).toEqual(["c","l"]);
});

test("an image that shows the free bytes of the host is limited by the host, an image with less than the host is full on its own",()=>{
 const image=(free:number,host:number|null)=>({freeBytes:free*MB,hostFreeBytes:host===null?null:host*MB});
 expect(readyImageHostLimitedV1(image(2000,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1985,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1975,2000))).toBe(false);
 expect(readyImageHostLimitedV1(image(47,19741))).toBe(false);
 expect(readyImageHostLimitedV1(image(2000,null))).toBe(false);
});
