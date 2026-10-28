/** 🌌️ Contract and oracle: a plan whose task reads a root-anchored pattern (all schema documents of the checkout, keys5's schema-document rule) is recorded and reused while git would list the same sources; directories that task's negations exclude completely are not part of its universe. Oracles: a complete second planner run (byte digest of the workspace) and `git ls-files -co --exclude-standard`. A root-anchored pattern without such a negation keeps the plan unreusable. Run from the repository root. */
import {expect,test}from"bun:test";
import {createHash}from"node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join,relative}from"node:path";
import corpus from"./🧫️fixtures/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1}from"../../../../../🔭️witness/🟨️.mjs";
import {bazelAdmittedListingV1,bazelIgnoreSourcesDigestV1,bazelTrackedDigestV1}from"../../../📜️script.ts";

process.env.SEMIO_LANGUAGE="en";
const repo=process.cwd(),child=join(import.meta.dirname,"🧫️fixtures/🟦️.ts"),signal=new AbortController().signal;
const git=(root:string,...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","core.fsmonitor=false","-c","commit.gpgsign=false",...args],{cwd:root,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(args.join(" ")+": "+run.stderr.toString());return run.stdout.toString();};
const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const universe=(root:string)=>git(root,"ls-files","-co","--exclude-standard","-z").split("\0").filter(path=>/(^|\/)🧬️schema\/.*\.json$/u.test(path)&&!/(^|\/)(target|node_modules|📦️packages|\.[^/]*)\//u.test(path)).sort().join("|");
function checkout(){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"whole-"))),root=join(base,"checkout");mkdirSync(root);
 for(const[path,text]of Object.entries(corpus.files))write(root,path,text);
 git(root,"init","-q");git(root,"add","-A");git(root,"commit","-q","-m","fixture");
 return{base,root};
}
function plan(base:string,root:string,witnessed:boolean,extra="{}"){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
 const workspace=join(base,"ws"),env={...process.env,SEMIO_LANGUAGE:"en",...(witnessed?worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace,writable:[tmpdir()],racyMilliseconds:0}):{})};
 const run=Bun.spawnSync([process.execPath,...(witnessed?["--preload",join(base,"preload.mjs")]:[]),child,root,workspace,extra,repo],{cwd:repo,env,stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString()+run.stdout.toString());
 return witnessed?readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8")):undefined;
}
const ignored=new Set(["runtime.json","snapshot.json","planner-input.json","planner-output.json"]);
function digest(workspace:string):string{
 const hash=createHash("sha256"),walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1)){
  const path=join(directory,entry.name),name=relative(workspace,path);
  if(!name.includes("/")&&(ignored.has(entry.name)||entry.name.endsWith(".tmp")||entry.name.startsWith("bazel-")))continue;
  if(entry.isDirectory())walk(path);else hash.update(name+"\0"+(lstatSync(path).mode&0o111)+"\0"+readFileSync(path)+"\0");
 }};
 walk(workspace);return hash.digest("hex");
}
const ports=(root:string)=>({git:async(subject:string,expected:string)=>(await bazelTrackedDigestV1(root,JSON.parse(subject) as string[],signal))===expected,ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,signal))===expected});

const relevant:Record<string,(root:string)=>void>={
 "content of a schema document":root=>write(root,"a/🧬️schema/x.json","{\"x\":2}\n"),
 "new document in a schema directory":root=>write(root,"a/🧬️schema/n.json","{}\n"),
 "removed schema document":root=>rmSync(join(root,"b/c/🧬️schema/y.json")),
 "new deep schema directory":root=>write(root,"d/e/f/🧬️schema/n.json","{}\n"),
 "new top-level directory with a schema document":root=>write(root,"z/🧬️schema/u.json","{}\n"),
};
const unrelated:Record<string,(root:string)=>void>={
 "document below an excluded packages directory":root=>write(root,"a/📦️packages/🧬️schema/w.json","{}\n"),
 "document below an excluded dot directory":root=>write(root,".hidden/🧬️schema/w.json","{}\n"),
 "ticket file another agent staged below .🧬semio":root=>{write(root,".🧬semio/🦑️repo/🎫️tickets/t/n.md","n\n");git(root,"add","--",".🧬semio");},
 "content of an unrelated file":root=>write(root,"docs/readme.md","changed\n"),
};
const conservative:Record<string,(root:string)=>void>={
 "non-matching file in a schema directory":root=>write(root,"a/🧬️schema/notes.md","n\n"),
 "new unrelated directory":root=>write(root,"docs/more/x.txt","x\n"),
 "new excluded directory beside observed ones":root=>write(root,"b/target/🧬️schema/t.json","{}\n"),
};

test("a root-anchored pattern with pruning negations leaves a sealed witness that holds on the untouched world",async()=>{
 const{base,root}=checkout();
 try{
  const witness=plan(base,root,true)!;
  expect(witness.tainted,witness.tainted.join(";")).toEqual([]);expect(witness.unstable).toEqual([]);expect(witness.racy).toBeNull();
  expect(witness.observations.some(row=>row[0]==="g"&&row[1]==="[]")).toBe(true);
  expect(witness.observations.some(row=>row[1].includes("📦️packages")),"the excluded directory is outside the universe").toBe(false);
  expect(await verifyWorldWitnessV1(witness,{signal,ports:ports(root)})).toMatchObject({holds:true});
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("the tracked digest covers exactly the entries the planner admits: oracle = git's own listing filtered as text",async()=>{
 const{base,root}=checkout();
 try{
  write(root,".🧬semio/🦑️repo/🎫️tickets/t/n.md","n\n");write(root,"a/🧬️schema/staged.json","{}\n");git(root,"add","-A");
  const text=git(root,"ls-files","-c","-z"),oracle=text.split("\0").filter(path=>path&&!path.startsWith(".🧬semio/")&&!path.startsWith(".git/")).map(path=>path+"\0").join("");
  expect(text).toContain(".🧬semio/");expect(oracle).toContain("a/🧬️schema/staged.json");
  expect(bazelAdmittedListingV1(Buffer.from(text)).toString()).toBe(oracle);
  expect(await bazelTrackedDigestV1(root,[],signal)).toBe(worldWitnessGitDigestV1(oracle));
  expect(await bazelTrackedDigestV1(join(base,"none"),[],signal)).toBe("!failed");
  const before=await bazelTrackedDigestV1(root,[],signal),raw=git(root,"ls-files","-c","-z");
  write(root,".🧬semio/🦑️repo/🎫️tickets/t/later.md","l\n");git(root,"add","-A");
  expect(worldWitnessGitDigestV1(git(root,"ls-files","-c","-z")),"negative control: the digest of the whole listing (the row before this batch) moves").not.toBe(worldWitnessGitDigestV1(raw));
  expect(await bazelTrackedDigestV1(root,[],signal),"the admitted digest does not").toBe(before);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("a root-anchored pattern without a pruning negation keeps the plan unreusable",()=>{
 const{base,root}=checkout();
 try{const witness=plan(base,root,true,JSON.stringify({tasks:["idx:bare"]}))!;expect(witness.tainted.some(reason=>reason.startsWith("the source universe is the whole repository"))).toBe(true);}
 finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("oracle: whenever the witness holds after an edit the complete plan is identical, relevant edits break it, edits below excluded directories do not",async()=>{
 const results:string[]=[];
 for(const[group,edits]of[["relevant",relevant],["unrelated",unrelated],["conservative",conservative]]as const)for(const[name,edit]of Object.entries(edits)){
  const{base,root}=checkout();
  try{
   const witness=plan(base,root,true)!,before=digest(join(base,"ws")),listed=universe(root);
   expect(witness.tainted,name).toEqual([]);
   edit(root);
   const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(root)}),holds=verdict.holds,changed=universe(root)!==listed;
   plan(base,root,false);const after=digest(join(base,"ws"));
   if(holds)expect(after,"unsound reuse after: "+name).toBe(before);
   if(group==="relevant"){expect(holds,"missed: "+name).toBe(false);expect(after,"relevant but no change: "+name).not.toBe(before);}
   else{expect(changed,"oracle git: "+name).toBe(false);if(group==="unrelated")expect(holds,"needless replan after: "+name+" "+JSON.stringify(verdict)).toBe(true);}
   results.push(`${group}: ${name}: holds=${holds} same=${after===before} listed-changed=${changed}`);
  }finally{rmSync(base,{recursive:true,force:true});}
 }
 console.log(results.join("\n"));
},600000);
