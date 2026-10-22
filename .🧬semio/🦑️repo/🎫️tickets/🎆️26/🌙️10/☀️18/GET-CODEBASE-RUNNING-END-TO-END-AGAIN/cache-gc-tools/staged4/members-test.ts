import{Database}from"bun:sqlite";
import{spawnSync}from"node:child_process";
import{closeSync,fstatSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readSync,rmSync,utimesSync,writeFileSync,writeSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import ts from"typescript";
import{MEMBER_INDEX_FILE_V1,appendMemberRowsV1,compactMemberIndexV1,encodeMemberHeaderV1,fullMemberRowV1,lightMemberRowV1,lookupMemberV1,memberGroupOfPathV1,memberGroupProtectedV1,memberHashV1,memberKindOfV1,memberStampOfRowStampV1,parseMemberIndexV1,proveGroupV1,readMemberRowV1,recordMemberLeafV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import cases from"./🧫️cases.json"with{type:"json"};

const fs={openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync}as never,Protocol=schema.definitions.Protocol.const;

test("kinds and groups of image-relative paths agree with the corpus and with the closure proof's own role table",()=>{
 for(const row of cases.kinds){
  expect(memberKindOfV1(row.name)??null,row.name).toBe(row.kind as never);
  const group=memberGroupOfPathV1(row.name);expect(group?.group??null,row.name).toBe(row.group);expect(group?.directory??null,row.name).toBe(row.directory);expect(group?.member??null,row.name).toBe(row.member);
 }
 const text=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("closures.ts",text,ts.ScriptTarget.Latest,true),node=tree.statements.find((value):value is ts.FunctionDeclaration=>ts.isFunctionDeclaration(value)&&value.name?.text==="role");
 expect(node).toBeDefined();
 const role=new Function(ts.transpileModule(node!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return role;")()as(name:string)=>string;
 for(const row of cases.kinds)if(row.kind!==null)expect(role(row.name),row.name).toBe(row.kind);
});

test("rows are sealed and closed: Ajv accepts the shape, the engine additionally verifies the seal, and an independent Python oracle recomputes every digest",()=>{
 const ajv=new Ajv({strict:false,allowUnionTypes:true});ajv.addSchema(schema);const light=ajv.getSchema(schema.$id+"#/definitions/LightRow")!,full=ajv.getSchema(schema.$id+"#/definitions/FullRow")!;
 for(const row of cases.rows){
  const parsed=(()=>{try{return JSON.parse(row.line);}catch{return undefined;}})(),admitted=readMemberRowV1(parsed);
  expect(admitted!==undefined,row.id).toBe(row.valid);
  if(row.valid)expect(light(parsed)||full(parsed),row.id).toBe(true);
 }
 const python=spawnSync(["python3","python","py"].find(name=>spawnSync(name,["-I","-c","pass"]).status===0)??"python3",["-I","-c","import sys,json,hashlib\nfor line in sys.stdin.read().splitlines():\n r=json.loads(line);c=r.pop('c');print(hashlib.sha256(json.dumps(r,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()[:16]==c)"],{input:cases.rows.filter(row=>row.valid).map(row=>row.line).join("\n"),encoding:"utf8"});
 if(python.status===0)expect(python.stdout.trim().split("\n")).toEqual(cases.rows.filter(row=>row.valid).map(()=>"True"));
});

test("an index keeps the last row of a name, ignores a torn tail, counts damaged lines and refuses a foreign group",()=>{
 for(const row of cases.indexes){
  const index=parseMemberIndexV1(row.text,row.group);
  expect(index.valid,row.id).toBe(row.valid);expect([...index.rows.keys()].sort(),row.id).toEqual(row.names);expect(index.rejected,row.id).toBe(row.rejected);expect(index.tornTail,row.id).toBe(row.tornTail);
  for(const[name,digest]of Object.entries((row as{winner?:Record<string,string>}).winner??{}))expect(index.rows.get(name)!.h,row.id).toBe(digest);
 }
});

test("a group is proven from its index and the physical stamps alone, equal to an independent SQLite join",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of cases.proofs){
   const index=parseMemberIndexV1(row.text,row.group),proof=proveGroupV1(index,row.physical);
   expect({full:proof.full,light:proof.light,stale:proof.stale,unindexed:proof.unindexed,complete:proof.complete,referencesComplete:proof.referencesComplete,references:proof.references},row.id).toEqual(row.expect);
   database.exec("DROP TABLE IF EXISTS member;DROP TABLE IF EXISTS leaf;CREATE TABLE member(n TEXT,k TEXT,b INTEGER,s TEXT,full INTEGER);CREATE TABLE leaf(n TEXT,k TEXT,b INTEGER,s TEXT)");
   const insert=database.prepare("INSERT INTO member VALUES(?,?,?,?,?)");if(index.valid)for(const member of index.rows.values())insert.run(member.n,member.k,member.b,member.s,member.r===undefined?0:1);
   const leaves=database.prepare("INSERT INTO leaf VALUES(?,?,?,?)");for(const leaf of row.physical)if(leaf.kind!=="member-index")leaves.run(leaf.name,leaf.kind,leaf.bytes,leaf.stamp);
   const oracle=database.query("SELECT COALESCE(SUM(m.n IS NOT NULL AND m.full=1),0) AS full,COALESCE(SUM(m.n IS NOT NULL AND m.full=0),0) AS light,COALESCE(SUM(m.n IS NULL AND EXISTS(SELECT 1 FROM member x WHERE x.n=l.n)),0) AS stale,COALESCE(SUM(m.n IS NULL AND NOT EXISTS(SELECT 1 FROM member x WHERE x.n=l.n)),0) AS unindexed FROM leaf l LEFT JOIN member m ON m.n=l.n AND m.k=l.k AND m.b=l.b AND m.s=l.s").get()as{full:number;light:number;stale:number;unindexed:number};
   expect(oracle,row.id).toEqual({full:row.expect.full,light:row.expect.light,stale:row.expect.stale,unindexed:row.expect.unindexed});
  }
 }finally{database.close();}
});

test("a proof from indexes reads no leaf: it needs only index text, so its cost is the index size",()=>{
 const group="selections/"+"c".repeat(64),rows=Array.from({length:100000},(_,i)=>lightMemberRowV1({n:"np/"+i.toString(16).padStart(24,"0")+".json",b:1000+i,h:memberHashV1(String(i)),k:"inference",s:i+":"+(1000+i)+":"+i*7})),text=encodeMemberHeaderV1(group)+"\n"+rows.map(row=>JSON.stringify(row)).join("\n")+"\n",physical=rows.map(row=>({name:row.n,kind:row.k,bytes:row.b,stamp:row.s}));
 const started=performance.now(),index=parseMemberIndexV1(text,group),proof=proveGroupV1(index,physical),elapsed=performance.now()-started;
 console.log("[members] proof of "+physical.length+" leaves from "+(text.length/1048576).toFixed(1)+" MiB of index in "+elapsed.toFixed(0)+" ms");
 expect(proof.light).toBe(100000);expect(proof.complete).toBe(true);expect(index.rejected).toBe(0);
 expect(elapsed).toBeLessThan(10000);
},60000);

test("write-time recording hashes a published leaf once, repairs a torn index, follows replacements and ignores what it cannot vouch for",()=>{
 const root=mkdtempSync(join(tmpdir(),"mi-")),id="d".repeat(64),group="selections/"+id,groupDirectory=join(root,"inference","selections",id),leaf=join(groupDirectory,"np","1".repeat(24)+".json");
 try{
  mkdirSync(join(groupDirectory,"np"),{recursive:true});writeFileSync(leaf,"first");
  expect(recordMemberLeafV1(fs,root,leaf)).toBe(true);
  const read=()=>parseMemberIndexV1(readFileSync(join(groupDirectory,MEMBER_INDEX_FILE_V1),"utf8"),group),stampOf=()=>memberStampOfRowStampV1(["0",...[lstatSync(leaf,{bigint:true})].map(row=>[row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":"))].join(":"));
  expect(read().rows.get("np/"+"1".repeat(24)+".json")!.h).toBe(memberHashV1("first"));
  expect(lookupMemberV1(read(),"np/"+"1".repeat(24)+".json","inference",5,stampOf())).toBeDefined();
  writeFileSync(join(groupDirectory,MEMBER_INDEX_FILE_V1),readFileSync(join(groupDirectory,MEMBER_INDEX_FILE_V1),"utf8")+'{"b":1,"c":"torn');
  const replaced=join(groupDirectory,"np","2".repeat(24)+".json");writeFileSync(replaced,"second");
  expect(recordMemberLeafV1(fs,root,replaced)).toBe(true);
  expect(readFileSync(join(groupDirectory,MEMBER_INDEX_FILE_V1),"utf8").split("\n").filter(Boolean).length).toBe(4);
  const healed=read();expect(healed.rejected).toBe(1);expect(healed.rows.size).toBe(2);
  writeFileSync(leaf,"third!");utimesSync(leaf,new Date(Date.now()+5000),new Date(Date.now()+5000));
  expect(recordMemberLeafV1(fs,root,leaf)).toBe(true);
  const newest=read(),row=newest.rows.get("np/"+"1".repeat(24)+".json")!;expect(row.h).toBe(memberHashV1("third!"));expect(row.b).toBe(6);
  expect(proveGroupV1(newest,[{name:row.n,kind:"inference",bytes:6,stamp:row.s},{name:"np/"+"2".repeat(24)+".json",kind:"inference",bytes:6,stamp:newest.rows.get("np/"+"2".repeat(24)+".json")!.s}]).complete).toBe(true);
  const outside=join(root,"elsewhere","np","3".repeat(24)+".json");mkdirSync(join(root,"elsewhere","np"),{recursive:true});writeFileSync(outside,"outside");
  expect(recordMemberLeafV1(fs,root,outside)).toBe(false);
  const foreign=join(groupDirectory,"notes.txt");writeFileSync(foreign,"x");expect(recordMemberLeafV1(fs,root,foreign)).toBe(false);
  const compact=compactMemberIndexV1(read(),group,new Map([[row.n,row.s]]));expect(parseMemberIndexV1(compact,group).rows.size).toBe(1);expect(compact.endsWith("\n")).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("rows can be appended under any file system port and never rewrite what is there",()=>{
 const root=mkdtempSync(join(tmpdir(),"mi-")),path=join(root,"members.jsonl"),group="contexts/"+"e".repeat(64),row=lightMemberRowV1({n:"np/"+"4".repeat(24)+".json",b:3,h:memberHashV1("abc"),k:"inference",s:"1:3:4"}),full=fullMemberRowV1({n:"ob/"+"5".repeat(64)+".br",b:4,h:memberHashV1("abcd"),k:"observation",s:"2:4:5",x:16,g:null,r:["source:"+"6".repeat(64)],l:[{p:"ob/"+"7".repeat(64)+".br",m:"block"}]});
 try{
  appendMemberRowsV1(fs,path,group,[row]);const first=readFileSync(path,"utf8");appendMemberRowsV1(fs,path,group,[full]);
  const second=readFileSync(path,"utf8");expect(second.startsWith(first)).toBe(true);expect(second.split("\n").filter(line=>line.includes("semio-member-index")).length).toBe(1);
  const index=parseMemberIndexV1(second,group);expect([...index.rows.keys()].sort()).toEqual([full.n,row.n].sort());expect(index.rows.get(full.n)!.r).toEqual(["source:"+"6".repeat(64)]);
  expect(()=>appendMemberRowsV1(fs,path,group,[{...row,n:"x".repeat(Protocol.maximumAppendBytes)}])).toThrow("Bounded member index append required");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a group touched within the protected idle time is left alone",()=>{
 expect(memberGroupProtectedV1(1000,1000+Protocol.groupProtectedIdleMilliseconds-1,Protocol.groupProtectedIdleMilliseconds)).toBe(true);
 expect(memberGroupProtectedV1(1000,1000+Protocol.groupProtectedIdleMilliseconds,Protocol.groupProtectedIdleMilliseconds)).toBe(false);
});

test("the shared namespace and the generation documents keep indexes of their own, written where their leaves live",()=>{
 const root=mkdtempSync(join(tmpdir(),"mi-")),canonical=join(root,"inference","np","6".repeat(24)+".json"),fragment=join(root,"generations","o","8".repeat(64)+".json.br"),pointer=join(root,"generations","current.json");
 try{
  mkdirSync(join(root,"inference","np"),{recursive:true});mkdirSync(join(root,"generations","o"),{recursive:true});writeFileSync(canonical,"{}");writeFileSync(fragment,"fragment");writeFileSync(pointer,"{}");
  expect(recordMemberLeafV1(fs,root,canonical)).toBe(true);expect(recordMemberLeafV1(fs,root,fragment)).toBe(true);expect(recordMemberLeafV1(fs,root,pointer)).toBe(false);
  expect(parseMemberIndexV1(readFileSync(join(root,"inference",MEMBER_INDEX_FILE_V1),"utf8"),"canonical").rows.get("np/"+"6".repeat(24)+".json")!.h).toBe(memberHashV1("{}"));
  expect(parseMemberIndexV1(readFileSync(join(root,"generations",MEMBER_INDEX_FILE_V1),"utf8"),"generations").rows.get("o/"+"8".repeat(64)+".json.br")!.k).toBe("generation-fragment");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the storage publication seals each published leaf into its group index after the commit and before it returns, and never fails the publication",()=>{
 const url=new URL("../../../../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs",import.meta.url),source=readFileSync(url,"utf8"),tree=ts.createSourceFile("storage.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 expect(source).toContain("reservation.commit();recordBuildGraphOptionalMemberV1(scope.store.root,resolve(path));return true;");
 const node=tree.statements.find((value):value is ts.FunctionDeclaration=>ts.isFunctionDeclaration(value)&&value.name?.text==="recordBuildGraphOptionalMemberV1");expect(node).toBeDefined();
 const record=new Function("recordMemberLeafV1","openSync","fstatSync","readSync","writeSync","closeSync","lstatSync","readFileSync",ts.transpileModule(node!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return recordBuildGraphOptionalMemberV1;")(recordMemberLeafV1,openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync)as(root:string,path:string)=>void;
 const root=mkdtempSync(join(tmpdir(),"mi-")),id="f".repeat(64),leaf=join(root,"inference","selections",id,"ip","7".repeat(24)+".json");
 try{
  mkdirSync(join(root,"inference","selections",id,"ip"),{recursive:true});writeFileSync(leaf,"{}");
  record(root,leaf);expect(parseMemberIndexV1(readFileSync(join(root,"inference","selections",id,MEMBER_INDEX_FILE_V1),"utf8"),"selections/"+id).rows.get("ip/"+"7".repeat(24)+".json")!.h).toBe(memberHashV1("{}"));
  expect(()=>record(root,join(root,"inference","selections",id,"ip","8".repeat(24)+".json"))).not.toThrow();
  expect(()=>record(root,join(root,"outside.json"))).not.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});
