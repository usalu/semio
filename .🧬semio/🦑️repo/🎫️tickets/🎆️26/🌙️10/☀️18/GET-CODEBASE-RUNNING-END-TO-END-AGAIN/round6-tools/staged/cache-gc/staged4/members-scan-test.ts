import{test,expect,beforeEach,afterEach}from"bun:test";
import{appendFileSync,closeSync,fstatSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readSync,readdirSync,rmSync,statSync,writeFileSync,writeSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{createHash}from"node:crypto";
import{MEMBER_INDEX_FILE_V1,encodeMemberHeaderV1,encodeMemberRowV1,lightMemberRowV1,memberHashV1,memberStampV1,parseMemberIndexV1,recordMemberLeafV1}from"../../🟨️.mjs";
import{indexBuildGraphMembersV1,listBuildGraphMemberGroupsV1,verifyBuildGraphMembersDeepV1,type MemberIndexPortsV1,type MemberScanControlV1}from"../../🔎️scan/🟦️.ts";

const sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),selection="a".repeat(64),dataset="b".repeat(64),fs={openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync}as never;
let root="";
beforeEach(()=>{root=mkdtempSync(join(tmpdir(),"ms-"));});
afterEach(()=>{rmSync(root,{recursive:true,force:true});});
const put=(group:string,name:string,text:string)=>{const path=join(root,"inference",group,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const shared=(name:string,text:string)=>{const path=join(root,"inference",name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const generation=(name:string,text:string)=>{const path=join(root,"generations",name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);return path;};
const quiet:MemberScanControlV1={cancelled:()=>false,progress:()=>{}};
const source=(character:string)=>"source:"+character.repeat(64),block=(character:string)=>character.repeat(64);

function fakePorts(state:{calls:number;paths:string[];refuse?:ReadonlySet<string>;leased?:boolean}):MemberIndexPortsV1{
 return{
  lease:async()=>state.leased===false?undefined:{release(){}},
  read:async packet=>{
   state.calls++;const request=JSON.parse(packet);
   const rows=request.files.map((file:{path:string;kind:string;index?:{path:string}})=>{
    if(state.refuse?.has(file.path))throw Error("Original closure read child refused");
    state.paths.push(file.path);const bytes=readFileSync(file.path),value=file.kind==="pack"||file.kind==="observation"?{}:JSON.parse(bytes.toString("utf8")),links=(value.blocks??[]).map((key:string)=>({path:join(dirname(file.path),"ob",key+".br"),missing:"block"})).concat(file.index?[{path:file.index.path,missing:"forbidden"}]:[]);
    return{path:file.path,hash:sha(bytes),expandedBytes:bytes.length*2,publicationGraph:null,references:(value.refs??[]).sort(),links};
   });
   return JSON.stringify({version:1,rows});
  }
 };
}
const build=()=>{
 put("selections/"+selection,"np/"+"1".repeat(24)+".json",JSON.stringify({refs:[source("c")],blocks:[block("d")]}));
 put("selections/"+selection,"ip/"+"2".repeat(24)+".json",JSON.stringify({refs:[]}));
 put("selections/"+selection,"ob/"+block("d")+".br","brotli");
 put("selections/"+selection,"mr/pk-1-"+"e".repeat(16)+".pack","pack-bytes");
 put("selections/"+selection,"mr/pk-1-"+"e".repeat(16)+".idx",'{"packBytes":10}');
 put("contexts/"+dataset,"np/"+"3".repeat(24)+".json",JSON.stringify({refs:[source("f")]}));
 shared("np/"+"6".repeat(24)+".json",JSON.stringify({refs:[source("a")]}));
 shared("inference/emoji-project-json.publication.json",JSON.stringify({refs:[]}));
 generation("o/"+"8".repeat(64)+".json.br",JSON.stringify({refs:[]}));
 generation("current.json",JSON.stringify({generation:0}));
};
const directoryOf=(group:string)=>group==="canonical"?"inference":group==="generations"?"generations":join("inference",group),index=(group:string)=>parseMemberIndexV1(readFileSync(join(root,directoryOf(group),MEMBER_INDEX_FILE_V1),"utf8"),group);

test("groups are listed from the directory names only, the shared namespaces first, and ignore foreign entries",()=>{
 build();mkdirSync(join(root,"inference","selections","not-a-group"),{recursive:true});writeFileSync(join(root,"inference","selections","stray.txt"),"x");
 expect(listBuildGraphMemberGroupsV1(root)).toEqual(["canonical","generations","contexts/"+dataset,"selections/"+selection]);
 expect(listBuildGraphMemberGroupsV1(join(root,"missing"))).toEqual([]);
});

test("one pass gives every leaf a full row with its hash, references, links and size, and the next pass reads nothing",async()=>{
 build();const state={calls:0,paths:[]as string[]},first=await indexBuildGraphMembersV1(root,fakePorts(state),quiet);
 expect(first).toEqual({groups:4,skipped:0,busy:0,indexed:9,refused:0,compacted:0,cancelled:false});
 const picked=index("selections/"+selection),row=picked.rows.get("np/"+"1".repeat(24)+".json")!;
 expect(picked.rows.size).toBe(5);
 expect(row.h).toBe(sha(readFileSync(join(root,"inference","selections",selection,"np","1".repeat(24)+".json"))));
 expect(row.r).toEqual([source("c")]);expect(row.l).toEqual([{p:"np/ob/"+block("d")+".br",m:"block"}]);
 const pack=picked.rows.get("mr/pk-1-"+"e".repeat(16)+".pack")!,idx=picked.rows.get("mr/pk-1-"+"e".repeat(16)+".idx")!;
 expect(pack.l).toEqual([{p:"mr/pk-1-"+"e".repeat(16)+".idx"}]);expect(idx.r).toEqual([]);expect(idx.h).toBe(memberHashV1('{"packBytes":10}'));
 expect(index("contexts/"+dataset).rows.size).toBe(1);
 expect([...index("canonical").rows.keys()]).toEqual(["inference/emoji-project-json.publication.json","np/"+"6".repeat(24)+".json"]);expect([...index("generations").rows.keys()]).toEqual(["o/"+"8".repeat(64)+".json.br"]);expect(index("generations").rows.get("o/"+"8".repeat(64)+".json.br")!.k).toBe("generation-fragment");
 expect(readdirSync(join(root,"inference")).includes("selections")).toBe(true);expect(()=>readFileSync(join(root,"generations","current.json"))).not.toThrow();
 const calls=state.calls,second=await indexBuildGraphMembersV1(root,fakePorts(state),quiet);
 expect(second).toEqual({groups:4,skipped:4,busy:0,indexed:0,refused:0,compacted:0,cancelled:false});expect(state.calls).toBe(calls);
});

test("a light row written at publication time is upgraded to a full row and a changed leaf is indexed again",async()=>{
 build();const group="selections/"+selection,leaf=join(root,"inference",group,"np","1".repeat(24)+".json");
 expect(recordMemberLeafV1(fs,root,leaf)).toBe(true);expect(index(group).rows.get("np/"+"1".repeat(24)+".json")!.r).toBeUndefined();
 const state={calls:0,paths:[]as string[]};await indexBuildGraphMembersV1(root,fakePorts(state),quiet);
 expect(index(group).rows.get("np/"+"1".repeat(24)+".json")!.r).toEqual([source("c")]);
 writeFileSync(leaf,JSON.stringify({refs:[source("9")]}));
 const later=await indexBuildGraphMembersV1(root,fakePorts(state),quiet);expect(later.indexed).toBe(1);
 expect(index(group).rows.get("np/"+"1".repeat(24)+".json")!.r).toEqual([source("9")]);
});

test("a group whose writer lease is held is read without it, never appended to, and counted busy",async()=>{
 build();const state={calls:0,paths:[]as string[],leased:false},report=await indexBuildGraphMembersV1(root,fakePorts(state),quiet);
 expect(report).toEqual({groups:4,skipped:0,busy:4,indexed:0,refused:0,compacted:0,cancelled:false});
 expect(()=>readFileSync(join(root,"inference","selections",selection,MEMBER_INDEX_FILE_V1))).toThrow();
});

test("a leaf the closure read child refuses is counted and skipped, never fatal, and its neighbours are still indexed",async()=>{
 build();const bad=join(root,"inference","selections",selection,"ip","2".repeat(24)+".json"),state={calls:0,paths:[]as string[],refuse:new Set([bad])},report=await indexBuildGraphMembersV1(root,fakePorts(state),quiet);
 expect(report.refused).toBe(1);expect(report.indexed).toBe(8);expect(index("selections/"+selection).rows.has("ip/"+"2".repeat(24)+".json")).toBe(false);
 expect(state.calls).toBeGreaterThan(2);
});

test("an index with more dead rows than live leaves is compacted to the live rows",async()=>{
 build();const group="selections/"+selection,target=join(root,"inference",group,MEMBER_INDEX_FILE_V1),leaf=join(root,"inference",group,"ip","2".repeat(24)+".json");
 await indexBuildGraphMembersV1(root,fakePorts({calls:0,paths:[]}),quiet);
 const base=index(group).rows.get("ip/"+"2".repeat(24)+".json")!;
 for(let at=0;at<200;at++)appendFileSync(target,encodeMemberRowV1(lightMemberRowV1({n:base.n,b:base.b,h:memberHashV1(String(at)),k:"precision",s:"1:"+at+":2"}))+"\n");
 appendFileSync(target,'{"torn');
 const before=statSync(target).size,report=await indexBuildGraphMembersV1(root,fakePorts({calls:0,paths:[]}),quiet);
 expect(report.compacted).toBe(1);expect(statSync(target).size).toBeLessThan(before);
 const after=index(group);expect(after.rejected).toBe(0);expect(after.tornTail).toBe(false);expect(after.count).toBe(after.rows.size);
 expect(after.rows.get(base.n)!.r).toEqual([]);expect(after.rows.size).toBe(5);
 void leaf;
});

test("the writer lease is held only while rows are appended, never while leaves are read",async()=>{
 build();const events:string[]=[],state={calls:0,paths:[]as string[]},ports=fakePorts(state),wrapped:MemberIndexPortsV1={lease:async(kind,identity)=>{events.push("lease:"+kind+":"+identity.slice(0,2));return{release(){events.push("release");}};},read:async packet=>{events.push("read");return ports.read(packet);}};
 await indexBuildGraphMembersV1(root,wrapped,quiet);
 for(let at=0;at<events.length;at++)if(events[at]!.startsWith("lease"))expect(events[at+1],events.join(" ")).toBe("release");
 expect(events.filter(event=>event==="read").length).toBeGreaterThan(0);expect(events.filter(event=>event.startsWith("lease:canonical")).length).toBe(2);
});

test("a compaction temporary a crash left behind is removed before the group is indexed",async()=>{
 build();const temporary=put("selections/"+selection,MEMBER_INDEX_FILE_V1+".tmp","half");
 await indexBuildGraphMembersV1(root,fakePorts({calls:0,paths:[]}),quiet);
 expect(()=>readFileSync(temporary)).toThrow();expect(index("selections/"+selection).rows.size).toBe(5);
});

test("cancellation stops between groups and leaves later groups untouched",async()=>{
 build();let checks=0;const state={calls:0,paths:[]as string[]},control:MemberScanControlV1={cancelled:()=>++checks>1,progress:()=>{}},report=await indexBuildGraphMembersV1(root,fakePorts(state),control);
 expect(report.cancelled).toBe(true);expect(report.groups).toBe(1);
 expect(()=>readFileSync(join(root,"inference","selections",selection,MEMBER_INDEX_FILE_V1))).toThrow();
});

test("deep verification re-hashes every indexed leaf and every closure inventory and names what differs",async()=>{
 build();await indexBuildGraphMembersV1(root,fakePorts({calls:0,paths:[]}),quiet);
 const good=sha("inventory");mkdirSync(join(root,"build-input-closures"),{recursive:true});writeFileSync(join(root,"build-input-closures",good+".json"),"inventory");writeFileSync(join(root,"build-input-closures","f".repeat(64)+".json"),"forged");
 const labels:string[]=[],clean=await verifyBuildGraphMembersDeepV1(root,{cancelled:()=>false,progress:label=>{labels.push(label);}});
 expect(clean.groups).toBe(4);expect(clean.verified).toBe(9);expect(clean.mismatches).toBe(0);expect(clean.closures).toBe(2);expect(clean.closureMismatches).toBe(1);expect(clean.closureMismatched).toEqual(["build-input-closures/"+"f".repeat(64)+".json"]);
 expect(labels).toContain("selections/"+selection);
 const group="selections/"+selection,leaf=join(root,"inference",group,"np","1".repeat(24)+".json"),stamp=memberStampV1(lstatSync(leaf,{bigint:true})),target=join(root,"inference",group,MEMBER_INDEX_FILE_V1);
 appendFileSync(target,encodeMemberRowV1(lightMemberRowV1({n:"np/"+"1".repeat(24)+".json",b:statSync(leaf).size,h:memberHashV1("tampered"),k:"inference",s:stamp}))+"\n");
 const found=await verifyBuildGraphMembersDeepV1(root,quiet);expect(found.mismatches).toBe(1);expect(found.mismatched).toEqual([group+"/np/"+"1".repeat(24)+".json"]);
 writeFileSync(leaf,JSON.stringify({refs:[],extra:1}));
 const stale=await verifyBuildGraphMembersDeepV1(root,quiet);expect(stale.stale).toBe(1);expect(stale.mismatches).toBe(0);
 rmSync(join(root,"inference","contexts"),{recursive:true});
 expect((await verifyBuildGraphMembersDeepV1(root,quiet)).groups).toBe(3);
 expect((await verifyBuildGraphMembersDeepV1(root,{cancelled:()=>true,progress:()=>{}})).cancelled).toBe(true);
});

test("the header and rows of a fresh index are exactly the documented wire form",()=>{
 expect(encodeMemberHeaderV1("contexts/"+dataset)).toBe('{"group":"contexts/'+dataset+'","semio":"semio-member-index","version":1}');
});
