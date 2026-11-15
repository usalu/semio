import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{createHash}from"node:crypto";
import{CLOSURE_SETTLE_V1,createClosureSettlerV1,stampOfV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createManifestV1,forgetEntryV1,indexPartV1,loadManifestV1,planSaveV1,readManifestV1,recordEntryV1,writeManifestV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../🟨️.mjs";
import{createFactIndexV1}from"../../../🟨️.mjs";
import type{GenerationRowV1}from"../../../🟨️.mjs";

const COUNT=40,names=Array.from({length:COUNT},(_,at)=>"q"+String(at).padStart(3,"0")),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1;
const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),{kind:"file",path:"own/"+name,fact:"v"}]] as [string,GenerationRowV1[]])]));
const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))));
const manifestIdentity=(subject:string)=>"closure-manifest\0"+subject+"\0";

function fixture(){
 const store=mkdtempSync(join(tmpdir(),"vouch-store-")),workspace=mkdtempSync(join(tmpdir(),"vouch-ws-"));
 mkdirSync(join(workspace,"own"));mkdirSync(join(workspace,"shared"));writeFileSync(join(workspace,"shared/s.txt"),"s0");
 for(const name of names)writeFileSync(join(workspace,"own",name),"v0-"+name);
 const text=(path:string)=>readFileSync(join(workspace,path),"utf8");
 const run=async(settler:ReturnType<typeof createClosureSettlerV1>,dirty:{paths:string[]}|undefined,options:{stamp?:boolean;during?:(name:string)=>void}={}) => {
  const derived:string[]=[];let rows=0;
  const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive:async(row:{path:string})=>{derived.push(row.path);options.during?.(row.path);return{fact:"old",parts:[[{kind:"file",path:"shared/s.txt",fact:text("shared/s.txt")}],[{kind:"file",path:"own/"+row.path,fact:text("own/"+row.path)}]]};},hold:async(list:{path:string;fact:string}[])=>{rows+=list.length;return list.every(row=>text(row.path)===row.fact);},dirty,...options.stamp===false?{}:{vouchStamp:(name:string)=>stampOfV1(join(workspace,name))}} as never);
  return{settled,derived,rows};
 };
 const fresh=()=>createClosureSettlerV1({store:createNodeStoreV1({root:store})});
 const edit=(name:string,content:string)=>writeFileSync(join(workspace,"own",name),content);
 return{store,workspace,run,fresh,edit,text,drop:()=>{rmSync(store,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}};
}

test("a second start over an unchanged tree verifies no closure, a start with k changed files verifies exactly the closures that name them, and a file edited again, reverted or shared by all is never taken from a stale vouch",async()=>{
 const world=fixture(),edited=["q003","q010","q020"],dirty={paths:edited.map(name=>"own/"+name)};
 try{
  const first=world.fresh(),cold=await world.run(first,undefined);expect(cold.derived.length).toBe(COUNT);await first.markIndexed({held:cold.settled.held});
  for(const name of edited)world.edit(name,"v1-"+name);
  const one=world.fresh(),restart=await world.run(one,dirty);
  expect(restart.derived.sort(),"only the closures that name an edited file").toEqual(edited);
  expect(restart.settled.vouched).toBe(0);await one.markIndexed({held:restart.settled.held});
  const two=world.fresh(),again=await world.run(two,dirty);
  expect(again.derived,"unchanged tree, same journal: nothing is derived").toEqual([]);expect(again.rows,"and no row is checked").toBe(0);
  expect(again.settled.vouched).toBe(edited.length);
  expect(again.settled.held.size+again.settled.kept.size).toBe(COUNT);
  const three=world.fresh(),third=await world.run(three,dirty);expect(third.derived).toEqual([]);expect(third.rows).toBe(0);
  world.edit("q003","v2-q003");
  const edit=await world.run(world.fresh(),dirty);expect(edit.derived,"the edited file again").toEqual(["q003"]);expect(edit.rows).toBeGreaterThan(0);
  world.edit("q003","v1-q003");
  const reverted=await world.run(world.fresh(),dirty);expect(reverted.derived,"back to an older content is not the content that was vouched").toEqual(["q003"]);
  const settled=await world.run(world.fresh(),dirty);expect(settled.derived).toEqual([]);expect(settled.rows).toBe(0);
  writeFileSync(join(world.workspace,"shared/s.txt"),"s1");
  const wide=await world.run(world.fresh(),{paths:[...dirty.paths,"shared/s.txt"]});expect(wide.derived.length,"a shared file reaches every closure").toBe(COUNT);
  const wideAgain=await world.run(world.fresh(),{paths:[...dirty.paths,"shared/s.txt"]});expect(wideAgain.derived).toEqual([]);expect(wideAgain.rows).toBe(0);
 }finally{world.drop();}
});

test("without a stamp of the files the same journal makes every start check the rows of the same closures again (the behaviour before the vouch)",async()=>{
 const world=fixture(),dirty={paths:["own/q001","own/q002"]};
 try{
  const first=world.fresh(),cold=await world.run(first,undefined,{stamp:false});await first.markIndexed({held:cold.settled.held});
  world.edit("q001","v1");world.edit("q002","v1");
  const rounds=[];for(let round=0;round<3;round++)rounds.push((await world.run(world.fresh(),dirty,{stamp:false})).rows);
  expect(rounds.every(rows=>rows>0),"each start checks rows: "+rounds.join(",")).toBe(true);
  const stamped=await world.run(world.fresh(),dirty);expect(stamped.rows).toBeGreaterThan(0);expect((await world.run(world.fresh(),dirty)).rows,"with the stamp the third start checks none").toBe(0);
 }finally{world.drop();}
});

test("a file that changes while the run derives is not vouched for, and the next start derives it once more",async()=>{
 const world=fixture(),dirty={paths:["own/q005"]};
 try{
  const first=world.fresh(),cold=await world.run(first,undefined);await first.markIndexed({held:cold.settled.held});
  world.edit("q005","v1");
  const racing=await world.run(world.fresh(),dirty,{during:name=>{if(name==="q005")world.edit("q005","v2");}});expect(racing.derived).toEqual(["q005"]);
  const next=await world.run(world.fresh(),dirty);expect(next.rows,"the vouch was never recorded: the rows are checked").toBeGreaterThan(0);
  const stable=await world.run(world.fresh(),dirty);expect(stable.derived).toEqual([]);expect(stable.rows).toBe(0);
 }finally{world.drop();}
});

test("a damaged manifest shard in the store is a miss: the start is correct without the manifest and an edit is never answered from it",async()=>{
 const world=fixture(),dirty={paths:["own/q009"]};
 try{
  const first=world.fresh(),cold=await world.run(first,undefined);await first.markIndexed({held:cold.settled.held});
  world.edit("q009","v1");
  const store=createNodeStoreV1({root:world.store}),head=await store.get(manifestIdentity("all"));
  expect(head?.value?.version).toBe(3);
  const row=head.value.shards.find((shard:string[])=>shard[0][0]==="e"),shard=await store.get(manifestIdentity(row[1]));
  shard.value.digests[0]="0".repeat(64);store.put(shard,"");await store.flush?.();
  const broken=await world.run(world.fresh(),dirty);
  expect(broken.derived,"the edited closure is derived, the others are not").toEqual(["q009"]);
  expect(broken.settled.held.size+broken.settled.kept.size).toBe(COUNT);
  expect(broken.settled.vouched).toBe(0);
 }finally{world.drop();}
});

test("every way to damage a persisted manifest reads as an empty one, and a sound one round-trips with its vouches",()=>{
 const book=createManifestV1();
 for(const[at,name]of["a","b","c"].entries()){indexPartV1(book,"part"+at,{paths:["f"+at],dirs:["d"],bare:[],config:false,volatile:false,outside:[],size:1});recordEntryV1(book,name,{digest:"d"+at,fact:"fact"+at,length:2,ids:["part"+at],vouch:at===1?undefined:"v"+at});}
 const sound=JSON.parse(JSON.stringify(writeManifestV1(book))),back=readManifestV1(sound);
 expect([...back.entries.keys()].sort()).toEqual(["a","b","c"]);expect(back.entries.get("a")!.vouch).toBe("v0");expect(back.entries.get("b")!.vouch).toBeUndefined();expect(back.entries.get("c")!.digest).toBe("d2");
 expect([...back.paths.keys()].sort()).toEqual(["f0","f1","f2"]);expect(back.dirs.get("d")!.length).toBe(3);
 const check=createHash("sha256").update(JSON.stringify([sound.head.at,sound.head.parts,sound.head.indexed,sound.head.marks,sound.head.volatile,sound.head.config,sound.head.bareAny,sound.head.outside,sound.head.shards])).digest("hex");
 expect(sound.head.check,"the check equals an independent sha256 of the head").toBe(check);
 const external=Bun.spawnSync(["shasum","-a","256"],{stdin:Buffer.from(JSON.stringify(Object.values(sound.shards)[0]))});
 if(external.exitCode===0)expect(sound.head.shards.some((row:string[])=>row[2]===external.stdout.toString().split(" ")[0]),"a shard digest equals an independent sha256 of its text").toBe(true);
 const subjectOf=(letter:string)=>Object.keys(sound.shards).find(subject=>subject.startsWith(letter))!;
 const damage:[string,(value:any)=>void][]=[
  ["head check",value=>{value.head.check="0".repeat(64);}],["head part",value=>{value.head.parts.pop();}],["head version",value=>{value.head.version=2;}],["head shard list",value=>{value.head.shards.pop();}],["head time",value=>{value.head.at=1;}],
  ["entry digest",value=>{value.shards[subjectOf("e")].digests[0]="x";}],["entry vouch",value=>{value.shards[subjectOf("e")].vouches[0]="x";}],["entry name",value=>{value.shards[subjectOf("e")].names[0]="z";}],
  ["entry bits",value=>{value.shards[subjectOf("e")].bits=value.shards[subjectOf("e")].bits.slice(4);}],["index path",value=>{value.shards[subjectOf("i")].paths.push(["x",[0]]);}],
  ["missing shard",value=>{delete value.shards[subjectOf("e")];}],["swapped shard",value=>{value.shards[subjectOf("e")]=value.shards[subjectOf("i")];}],["no shards",value=>{delete value.shards;}],
 ];
 for(const[label,hurt]of damage){const copy=JSON.parse(JSON.stringify(sound));hurt(copy);expect(readManifestV1(copy).entries.size,label).toBe(0);}
 for(const junk of[undefined,null,0,"x",[],{},{head:{},shards:{}}])expect(readManifestV1(junk as never).entries.size).toBe(0);
});

test("only the shards that changed are written again, and a manifest saved after every change loads to the same entries",async()=>{
 const book=createManifestV1();
 for(let at=0;at<300;at++){indexPartV1(book,"part"+at,{paths:["f"+at],dirs:[],bare:[],config:false,volatile:false,outside:[],size:1});recordEntryV1(book,"q"+at,{digest:"d"+at,fact:"fact"+at,length:2,ids:["part"+at,"part"+((at+1)%300)]});}
 const first=planSaveV1(book);expect(first.writes.length).toBeGreaterThan(2);first.commit();
 const idle=planSaveV1(book);expect(idle.writes,"nothing changed: nothing is written").toEqual([]);
 recordEntryV1(book,"q7",{digest:"changed",fact:"fact7",length:2,ids:["part7"],vouch:"v"});
 const one=planSaveV1(book);expect(one.writes.length,"one entry shard").toBe(1);one.commit();
 forgetEntryV1(book,"q8");const gone=planSaveV1(book);expect(gone.writes.length).toBe(1);gone.commit();
 const stored=new Map<string,unknown>([["all",first.head]]);
 const disk=new Map<string,unknown>();for(const plan of[first,one,gone])for(const[subject,value]of plan.writes)disk.set(subject,JSON.parse(JSON.stringify(value)));
 const back=await loadManifestV1(async subject=>subject==="all"?JSON.parse(JSON.stringify(gone.head)):disk.get(subject) as never);
 expect(back.entries.size).toBe(299);expect(back.entries.get("q7")!.digest).toBe("changed");expect(back.entries.get("q7")!.vouch).toBe("v");expect(back.entries.has("q8")).toBe(false);
 expect(planSaveV1(back).writes,"a loaded manifest writes nothing until it changes").toEqual([]);
 const stale=planSaveV1(back,false,Date.now()+2*86400000);
 expect(stale.writes.length,"a day later every shard is written again, so none outlives the retention of the store").toBe(stale.head.shards.length);
 expect(stale.head.at).toBeGreaterThan(back.fullAt);
 stored.clear();
});

test("the stamp of a path follows its bytes, not its times, and equals an independent sha256 of the file",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"stamp-"));
 try{
  const file=join(dir,"f.txt");writeFileSync(file,"hello");
  const stamp=await stampOfV1(file),external=Bun.spawnSync(["shasum","-a","256",file]);
  expect(stamp).toBe("f:"+createHash("sha256").update("hello").digest("hex"));
  if(external.exitCode===0)expect(stamp).toBe("f:"+external.stdout.toString().split(" ")[0]);
  utimesSync(file,new Date(1e12),new Date(1e12));expect(await stampOfV1(file),"a touch does not change the stamp").toBe(stamp);
  writeFileSync(file,"hellp");expect(await stampOfV1(file)).not.toBe(stamp);
  expect(await stampOfV1(join(dir,"absent"))).toBe("-");
  const listed=await stampOfV1(dir);writeFileSync(join(dir,"g.txt"),"x");expect(await stampOfV1(dir),"a new entry changes the listing").not.toBe(listed);
  const before=await stampOfV1(dir);writeFileSync(join(dir,"g.txt"),"y");expect(await stampOfV1(dir),"content of an entry is not part of the listing").toBe(before);
  symlinkSync("f.txt",join(dir,"link"));expect(await stampOfV1(join(dir,"link"))).toBe("l:f.txt");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a settle killed in the middle leaves its manifest durable: the resumed start answers the finished chunks from it and derives only what was unfinished",async()=>{
 const count=60,chunk=10,killAt=35,world=fixture(),root=mkdtempSync(join(tmpdir(),"vouch-killed-")),ws=mkdtempSync(join(tmpdir(),"vouch-killed-ws-")),ids=Array.from({length:count},(_,at)=>"q"+String(at).padStart(4,"0")),row=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1;
 try{
  mkdirSync(join(ws,"own"));for(const name of ids)writeFileSync(join(ws,"own",name),"v-"+name);
  const child=Bun.spawnSync([process.execPath,...process.env.R5_PRELOAD?["--preload",process.env.R5_PRELOAD]:[],join(dirname(fileURLToPath(import.meta.url)),"🪓️killed/🟨️.mjs"),root,ws,String(count),String(chunk),String(killAt)]);
  expect(child.signalCode ?? child.exitCode,"the child was killed").not.toBe(0);
  const durable=createNodeStoreV1({root}),loaded=await loadManifestV1(async subject=>(await durable.get(manifestIdentity(subject)))?.value),flushed=Math.floor((killAt-1)/chunk)*chunk;
  expect(loaded.entries.size,"the manifest of the finished chunks is durable").toBeGreaterThanOrEqual(flushed);
  const full=createFactIndexV1(new Map([["holder",ids.map(row)],...ids.map(name=>[derivedOwnerNameV1(row(name)),[row(name),{kind:"file",path:"own/"+name,fact:"v"}]] as [string,GenerationRowV1[]])]));
  const derived:string[]=[],settled=await createClosureSettlerV1({store:createNodeStoreV1({root}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:chunk}}).settle({pseudoRows:suspectRowsV1(full,new Set(ids.map(name=>derivedOwnerNameV1(row(name))))),derive:async(r:{path:string})=>{derived.push(r.path);return{fact:"old",parts:[[{kind:"file",path:"own/"+r.path,fact:readFileSync(join(ws,"own",r.path),"utf8")}]]};},hold:async()=>true,dirty:{paths:[]},vouchStamp:(name:string)=>stampOfV1(join(ws,name))} as never);
  expect(settled.held.size+settled.kept.size).toBe(count);
  expect(derived.length,"only the unfinished closures were derived").toBeLessThanOrEqual(count-flushed);
 }finally{world.drop();rmSync(root,{recursive:true,force:true});rmSync(ws,{recursive:true,force:true});}
});
