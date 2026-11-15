import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const fixture=await Bun.file(B+"🕸️graph/📥️inference/🧪️tests/✂️derived-walks/🔣️.json").json();
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs") as any;const {createScriptInputCache,libraryBootstrap}=inference;await libraryBootstrap;
const observation=await import(B+"👁️observation/🟨️.mjs") as any,scriptInputs=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];afterAll(()=>{for(const r of roots)rmSync(r,{recursive:true,force:true});});
const write=(root:string,files:any)=>{for(const[p,t]of Object.entries(files)){mkdirSync(dirname(join(root,p)),{recursive:true});writeFileSync(join(root,p),t as string);}};
test("overlap of project rows and derived support per recipe kind",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"n6-exp-"));roots.push(root);write(root,{...fixture.files,"tools/📜️script.ts":"import \"./entry.mjs\";\nexport {};\n"});
 const target={options:{cwd:".",command:"repo ./tools/📜️script.ts run"}};
 for(const kind of ["closure-of","generic-target"]){
  const r=await observation.observeInferenceSnapshotV1(async()=>{
   const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
   const list=kind==="closure-of"?await api.closureOf([join(root,fixture.entry)],[],"test-discovery"):await api.genericTargetCommandInputs(target,".");
   const key=scriptInputs.closureHandleOfV1(list).key;return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);});
  const sup=new Set(r.derived.flatMap((d:any)=>[...d.support]));
  if(kind==="generic-target"){const g=new Map<string,number>();for(const x of r.rows){const k=x.kind+(x.kind==="analysis"?" "+x.fact.provider.split(":")[1]:"")+(sup.has(x)?" IN":" out");g.set(k,(g.get(k)??0)+1);}console.log("N6 groups",JSON.stringify([...g]));}
  console.log("N6",kind,"rows",r.rows.length,"derived",r.derived.length,"support",sup.size,"rows in support",r.rows.filter((x:any)=>sup.has(x)).length,"kinds",JSON.stringify(Object.entries(r.rows.reduce((a:any,x:any)=>(a[x.kind]=(a[x.kind]??0)+1,a),{}))));
 }
},120000);
