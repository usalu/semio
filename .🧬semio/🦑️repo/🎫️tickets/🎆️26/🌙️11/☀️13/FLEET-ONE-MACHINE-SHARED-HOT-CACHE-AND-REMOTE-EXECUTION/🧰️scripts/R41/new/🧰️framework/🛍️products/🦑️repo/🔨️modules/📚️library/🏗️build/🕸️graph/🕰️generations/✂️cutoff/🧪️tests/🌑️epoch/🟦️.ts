import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,unlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createClosureSettlerV1,stampOfV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{staleSummaryV1,rowPathsV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/🧭️row-paths/🟨️.mjs";
import{affectedPartsV1,createManifestV1,indexPartV1}from"../../../../🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🟨️.mjs";
import{createNodeStoreV1}from"../../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{derivedOwnerNameV1,suspectRowsV1}from"../../🟨️.mjs";
import{createFactIndexV1}from"../../../🟨️.mjs";
import type{GenerationRowV1}from"../../../🟨️.mjs";

const COUNT=40,names=Array.from({length:COUNT},(_,at)=>"q"+String(at).padStart(3,"0")),rowOf=(name:string,fact="old")=>({kind:"derived",path:name,fact}) as GenerationRowV1;
const indexOf=(facts:Record<string,string>={})=>createFactIndexV1(new Map([["holder",names.map(name=>rowOf(name,facts[name]))],...names.map(name=>[derivedOwnerNameV1(rowOf(name,facts[name])),[rowOf(name,facts[name]),{kind:"file",path:"own/"+name,fact:"v"}]] as [string,GenerationRowV1[]])]));
const suspectsOf=(facts:Record<string,string>={})=>new Set(names.map(name=>derivedOwnerNameV1(rowOf(name,facts[name]))));
const manifestIdentity=(subject:string)=>"closure-manifest\0"+subject+"\0";

function fixture(){
 const store=mkdtempSync(join(tmpdir(),"epoch-store-")),workspace=mkdtempSync(join(tmpdir(),"epoch-ws-"));
 mkdirSync(join(workspace,"own"));mkdirSync(join(workspace,"shared"));writeFileSync(join(workspace,"shared/s.txt"),"s0");
 for(const name of names)writeFileSync(join(workspace,"own",name),"v0-"+name);
 const text=(path:string)=>{try{return readFileSync(join(workspace,path),"utf8");}catch{return"gone";}};
 const stale=(...paths:string[])=>paths.map(path=>({kind:"file",path,fact:"recorded-before"}));
 const run=async(settler:ReturnType<typeof createClosureSettlerV1>,staleFacts:unknown[]|undefined,options:{facts?:Record<string,string>;during?:(name:string)=>void}={}) => {
  const derived:string[]=[];let rows=0;
  const settled=await settler.settle({pseudoRows:suspectRowsV1(indexOf(options.facts),suspectsOf(options.facts)),derive:async(row:{path:string})=>{derived.push(row.path);options.during?.(row.path);return{fact:"old",parts:[[{kind:"file",path:"shared/s.txt",fact:text("shared/s.txt")}],[{kind:"file",path:"own/"+row.path,fact:text("own/"+row.path)}]]};},hold:async(list:{path:string;fact:string}[])=>{rows+=list.length;return list.every(row=>text(row.path)===row.fact);},staleFacts,vouchStamp:(name:string)=>stampOfV1(join(workspace,name))} as never);
  return{settled,derived,rows};
 };
 const fresh=()=>createClosureSettlerV1({store:createNodeStoreV1({root:store})});
 const edit=(name:string,content:string)=>writeFileSync(join(workspace,"own",name),content);
 const cold=async()=>{const first=fresh(),result=await run(first,undefined);await first.markIndexed({held:result.settled.held});return result;};
 return{store,workspace,run,fresh,edit,text,stale,cold,drop:()=>{rmSync(store,{recursive:true,force:true});rmSync(workspace,{recursive:true,force:true});}};
}

test("a read without a journal resumes: the rows that no longer hold are the dirty set, a restart over the same edits derives nothing and checks no row, and a changed, reverted or shared file is never taken from a stale stamp",async()=>{
 const world=fixture(),edited=["q003","q010","q020"],stale=world.stale(...edited.map(name=>"own/"+name));
 try{
  const cold=await world.cold();expect(cold.derived.length).toBe(COUNT);
  for(const name of edited)world.edit(name,"v1-"+name);
  const control=world.fresh(),before=await world.run(control,undefined);
  expect(before.derived.sort(),"control: no journal and no stale facts asks every closure its rows").toEqual(edited);expect(before.rows).toBeGreaterThan(0);await control.markIndexed({held:before.settled.held});
  for(const name of edited)world.edit(name,"v2-"+name);
  const one=world.fresh(),restart=await world.run(one,world.stale(...edited.map(name=>"own/"+name)));
  expect(restart.derived.sort(),"only the closures that name a stale row").toEqual(edited);await one.markIndexed({held:restart.settled.held});
  const two=await world.run(world.fresh(),stale);
  expect(two.derived,"the same stale rows again: nothing is derived").toEqual([]);expect(two.rows,"and no row is checked").toBe(0);expect(two.settled.vouched).toBe(edited.length);
  expect(two.settled.held.size+two.settled.kept.size).toBe(COUNT);
  world.edit("q003","v3-q003");
  expect((await world.run(world.fresh(),stale)).derived,"a changed leaf").toEqual(["q003"]);
  world.edit("q003","v2-q003");
  expect((await world.run(world.fresh(),stale)).derived,"a leaf back at an older content is not the content that was vouched").toEqual(["q003"]);
  expect((await world.run(world.fresh(),stale)).derived).toEqual([]);
  writeFileSync(join(world.workspace,"shared/s.txt"),"s1");
  const wide=await world.run(world.fresh(),world.stale("shared/s.txt",...edited.map(name=>"own/"+name)));expect(wide.derived.length,"a shared file reaches every closure").toBe(COUNT);
  const wideAgain=await world.run(world.fresh(),world.stale("shared/s.txt",...edited.map(name=>"own/"+name)));expect(wideAgain.derived).toEqual([]);expect(wideAgain.rows).toBe(0);
 }finally{world.drop();}
});

test("a removed leaf is derived once, vouched while it stays removed, and derived again when it comes back with other bytes",async()=>{
 const world=fixture(),stale=world.stale("own/q004");
 try{
  await world.cold();
  unlinkSync(join(world.workspace,"own/q004"));
  const gone=await world.run(world.fresh(),stale);expect(gone.derived).toEqual(["q004"]);
  const kept=await world.run(world.fresh(),stale);expect(kept.derived).toEqual([]);expect(kept.rows).toBe(0);expect(kept.settled.vouched).toBe(1);
  world.edit("q004","v9-q004");
  const back=await world.run(world.fresh(),stale);expect(back.derived,"present again with other bytes").toEqual(["q004"]);
 }finally{world.drop();}
});

test("a changed record is never answered from the manifest: the generation holds another fact than the recorded answer",async()=>{
 const world=fixture();
 try{
  await world.cold();
  const changed=await world.run(world.fresh(),world.stale("own/q007"),{facts:{q007:"other"}});
  expect([...changed.settled.changed.values()].map((row:any)=>row.path)).toEqual(["q007"]);
 }finally{world.drop();}
});

test("a torn or forged stamp is a miss: a damaged vouch derives the closure again and a damaged shard reads as no manifest, the answer stays correct",async()=>{
 const world=fixture(),stale=world.stale("own/q009");
 try{
  await world.cold();world.edit("q009","v1");
  const sealed=world.fresh();await world.run(sealed,stale);
  const store=createNodeStoreV1({root:world.store}),head=await store.get(manifestIdentity("all"));
  const row=head.value.shards.find((shard:string[])=>shard[0][0]==="e"),shard=await store.get(manifestIdentity(row[1]));
  shard.value.vouches=shard.value.vouches.map(()=>"0".repeat(64));store.put(shard,"");await store.flush?.();
  const forged=await world.run(world.fresh(),stale);
  expect(forged.settled.vouched,"the vouch does not match the bytes: not trusted").toBe(0);expect(forged.rows,"the rows are checked instead").toBeGreaterThan(0);
  expect(forged.settled.held.size+forged.settled.kept.size).toBe(COUNT);
  world.edit("q009","v2");const edited=await world.run(world.fresh(),stale);expect(edited.derived,"and an edit under a forged vouch is derived").toEqual(["q009"]);
  const bytes=JSON.stringify(shard.value);shard.value={torn:bytes.slice(0,bytes.length>>1)};store.put(shard,"");await store.flush?.();
  world.edit("q009","v3");
  const torn=await world.run(world.fresh(),stale);
  expect(torn.derived).toContain("q009");expect(torn.settled.held.size+torn.settled.kept.size).toBe(COUNT);
 }finally{world.drop();}
});

test("two reads of one machine that settle the same stale rows at the same time agree, and the next start derives nothing",async()=>{
 const world=fixture(),edited=["q001","q002","q030"],stale=world.stale(...edited.map(name=>"own/"+name));
 try{
  await world.cold();for(const name of edited)world.edit(name,"v1-"+name);
  const[left,right]=await Promise.all([world.run(world.fresh(),stale),world.run(world.fresh(),stale)]);
  for(const side of[left,right]){expect(side.derived.sort()).toEqual(edited);expect(side.settled.held.size+side.settled.kept.size).toBe(COUNT);}
  const next=await world.run(world.fresh(),stale);expect(next.derived).toEqual([]);expect(next.rows).toBe(0);
 }finally{world.drop();}
});

test("an empty list of stale rows is no dirty set: the old behaviour stays",async()=>{
 const world=fixture();
 try{
  await world.cold();
  const none=await world.run(world.fresh(),[]);expect(none.settled.vouched).toBe(0);expect(none.rows,"every closure is checked as before").toBeGreaterThan(0);
 }finally{world.drop();}
});

function rng(seed:number){let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}

test("law: every part that holds a stale row is a part the stale summary reaches (random books, brute force as the oracle)",async()=>{
 const identity=(path:string)=>path.startsWith("/ws/")?path.slice(4):"\0volatile";
 for(let seed=1;seed<=200;seed++){
  const next=rng(seed),pick=<T>(list:T[])=>list[Math.floor(next()*list.length)]!;
  const files=Array.from({length:12},(_,at)=>"/ws/d"+(at%3)+"/f"+at+".ts"),dirs=["/ws/d0","/ws/d1","/ws/d2"];
  const makeRow=():any=>{
   const roll=next();
   if(roll<0.18)return{kind:"file",path:pick(files),fact:{size:1}};
   if(roll<0.3)return{kind:"analysis",path:pick(files),fact:{}};
   if(roll<0.4)return{kind:"readdir",path:pick(dirs),fact:"x"};
   if(roll<0.5)return{kind:"fileExists",path:pick(files)+"x",fact:false};
   if(roll<0.6)return{kind:"lstat",path:pick(files),fact:"file"};
   if(roll<0.7)return{kind:"resolve",path:JSON.stringify([pick(files),"./"+pick(["a","b","c"])]),fact:next()<0.5?pick(files):"!MODULE_NOT_FOUND"};
   if(roll<0.76)return{kind:"resolve",path:JSON.stringify([pick(files),"react"]),fact:pick(files)};
   if(roll<0.82)return{kind:"field",path:pick(files),fact:{paths:[pick(files)]}};
   if(roll<0.86)return{kind:"registry",path:"r",fact:"x"};
   if(roll<0.9)return{kind:"file",path:"/outside/o.ts",fact:{size:1}};
   return{kind:"digest",path:pick(files),fact:{sha256:"x"}};
  };
  const book=createManifestV1(),parts:any[][]=Array.from({length:10},()=>Array.from({length:1+Math.floor(next()*6)},makeRow));
  parts.forEach((rows,at)=>{
   const meta:any={paths:[],dirs:[],bare:[],config:false,volatile:false,outside:[],size:rows.length};
   for(const row of rows){const one=rowPathsV1(row,identity,"\0volatile");meta.paths.push(...one.paths);meta.dirs.push(...one.dirs);meta.bare.push(...one.bare);meta.config||=one.config;meta.volatile||=one.volatile;if(one.outside)meta.outside.push(row);}
   indexPartV1(book,"p"+at,meta);
  });
  const stale=parts.flat().filter(()=>next()<0.15);
  if(stale.length===0)continue;
  const bits=await affectedPartsV1(book,staleSummaryV1(stale,identity,"\0volatile") as never,async()=>stale.filter(row=>row.path==="/outside/o.ts"));
  parts.forEach((rows,at)=>{
   const holdsStale=rows.some(row=>stale.includes(row)),number=book.partNo.get("p"+at)!,hit=((bits[number>>5]??0)>>>(number&31)&1)===1;
   if(holdsStale)expect(hit,"seed "+seed+" part "+at).toBe(true);
  });
 }
});

test("the stale summary of rows above the checkout and of unknown kinds adds nothing, a bare resolution asks for every configuration and install part",()=>{
 const identity=(path:string)=>path.startsWith("/ws/")?path.slice(4):"\0volatile";
 const quiet=staleSummaryV1([{kind:"file",path:"/elsewhere/x",fact:{}},{kind:"digest",path:"/ws/a",fact:{}}],identity,"\0volatile");
 expect([...quiet.paths]).toEqual([]);expect(quiet.configs).toBe(false);expect(quiet.modules).toBe(false);
 const bare=staleSummaryV1([{kind:"resolve",path:JSON.stringify(["/ws/a/b.ts","react"]),fact:"!MODULE_NOT_FOUND"}],identity,"\0volatile");
 expect(bare.configs).toBe(true);expect(bare.modules).toBe(true);expect(bare.directories.has("a")).toBe(true);
 const config=staleSummaryV1([{kind:"file",path:"/ws/p/package.json",fact:{}}],identity,"\0volatile");expect(config.configs).toBe(true);expect(config.paths.has("p/package.json")).toBe(true);
});
