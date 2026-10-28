/** ✂️ Contract: a project that reuses a command walk names it by one derived fact (the question the walk answered and the digest of its answer) instead of by the rows it read. Asking the question again against the live files yields the same digest while the roster, the compiled modules and the generated outputs are the same, whatever the walk's files changed in between (comments, imports of files the walk already reaches, rewritten specifiers), and another digest as soon as one of them moves; the rosters equal esbuild's independent bundle graph. */
import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import fixture from"./🔣️.json";

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};

test("a reused walk is named by one derived fact and asking it again follows the live files",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-"));roots.push(root);write(root,fixture.files);
 const entry=join(root,fixture.entry);
 const first=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.closureOf([entry],[],"test-discovery"),key=scriptInputs.closureHandleOfV1(list).key;
  const resolved=await api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);
  return{resolved,key};
 });
 expect(first.resolved.derived).toHaveLength(1);
 const derived=first.resolved.derived[0];
 expect(derived.row.kind).toBe("derived");expect(JSON.parse(derived.row.path).walk).toBe(1);expect(derived.row.fact).toMatch(/^[a-f0-9]{64}$/u);
 expect(derived.support.length).toBeGreaterThan(64);
 expect(first.resolved.rows.length).toBeLessThan(derived.support.length);
 expect(first.resolved.rows.every((row:any)=>!derived.support.includes(row))).toBe(true);
 const baseline=derived.row.fact as string;
 for(const step of fixture.steps){
  write(root,step.edit);
  const again=await observation.observeInferenceSnapshotV1(async()=>scriptInputs.deriveClosureFactV1(derived.row,createScriptInputCache({workspaceRoot:root,lateClosures:false}))).catch((error:Error)=>error);
  if(step.closureSize===null){expect(again instanceof Error||again.fact!==baseline,step.name).toBe(true);continue;}
  expect(again instanceof Error,step.name+": "+(again as Error).message).toBe(false);
  expect(again.fact===baseline,step.name).toBe(step.same);
  expect(again.rows.length,step.name).toBeGreaterThan(0);
  const fresh=await observation.observeInferenceSnapshotV1(async()=>{const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:false});return(await scriptInputs.relativeScriptInputs([entry],root,scripts)) as string[];});
  const roster=fresh.map((path:string)=>path.replace("{workspaceRoot}/","")).sort();
  expect(roster.length,step.name).toBe(step.closureSize);
  expect(roster,step.name).toEqual(await esbuildClosure(root,fixture.entry));
 }
},120000);

test("a caller that is no project (the test discovery) holds the same derived fact for the same walk, with the same roster, and the project path shares its entry",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-caller-"));roots.push(root);write(root,fixture.files);
 const entry=join(root,fixture.entry);
 const walk=await observation.observeInferenceSnapshotV1(()=>scriptInputs.resolveCommandWalkV1(root,[entry],createScriptInputCache({workspaceRoot:root,lateClosures:true})));
 const project=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.closureOf([entry],[],"test-discovery"),key=scriptInputs.closureHandleOfV1(list).key;
  return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);
 });
 expect(walk.derived).toHaveLength(1);expect(walk.derived[0].row).toEqual(project.derived[0].row);expect(walk.derived[0].support).toBe(project.derived[0].support);
 const direct=await observation.observeInferenceSnapshotV1(async()=>(await scriptInputs.relativeScriptInputs([entry],root,createScriptInputCache({workspaceRoot:root}))) as string[]);
 expect(walk.value).toEqual(direct);
 expect(walk.rows.every((row:any)=>!walk.derived[0].support.includes(row))).toBe(true);expect(walk.rows.length).toBeLessThan(walk.derived[0].support.length);
 expect(()=>scriptInputs.resolveCommandWalkV1(root,[entry],createScriptInputCache({workspaceRoot:root}))).toThrow();
 const baseline=walk.derived[0].row.fact as string;
 for(const step of fixture.steps){
  write(root,step.edit);
  const again=await observation.observeInferenceSnapshotV1(async()=>scriptInputs.deriveClosureFactV1(walk.derived[0].row,createScriptInputCache({workspaceRoot:root}))).catch((error:Error)=>error);
  if(step.closureSize===null){expect(again instanceof Error||again.fact!==baseline,step.name).toBe(true);continue;}
  expect(again instanceof Error,step.name).toBe(false);expect(again.fact===baseline,step.name).toBe(step.same);
 }
},120000);

test("a derived question that no walk answers is refused and a walk of another provider designation is not answered",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-walk-refuse-"));roots.push(root);write(root,fixture.files);
 const cache=createScriptInputCache({workspaceRoot:root});
 await expect(scriptInputs.deriveClosureFactV1({kind:"derived",path:JSON.stringify({walk:2}),fact:"x"},cache)).rejects.toBeDefined();
 await expect(scriptInputs.deriveClosureFactV1({kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:[],requiredSources:false,resources:[],selectors:[],provider:"elsewhere.json"}),fact:"x"},cache)).rejects.toBeDefined();
});

test("a TypeScript program reused by a project is named by one derived fact too, and asking it again follows the files the compiler reads",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"derived-program-"));roots.push(root);write(root,fixture.program.files);
 const target={options:{cwd:"proj",command:"repo ./📜️script.ts check proj"},metadata:{semio:{closure:{typescript:["{projectRoot}/tsconfig.json"]}}}};
 const first=await observation.observeInferenceSnapshotV1(async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),list=await api.genericTargetCommandInputs(target,fixture.program.projectRoot),key=scriptInputs.closureHandleOfV1(list).key;
  return api.resolve([[key,scriptInputs.closureRecipeByKeyV1(key)]]);
 });
 const programs=first.derived.filter((fact:any)=>JSON.parse(fact.row.path).program===1);
 expect(programs).toHaveLength(1);
 const derived=programs[0];
 expect(derived.support.length).toBeGreaterThan(64);expect(derived.row.fact).toMatch(/^[a-f0-9]{64}$/u);
 const nearest=(directory:string):string[]=>directory.startsWith(root+"/")?existsSync(join(directory,"tsconfig.json"))?[join(directory,"tsconfig.json")]:nearest(dirname(directory)):[];
 expect(first.rows.filter((row:any)=>derived.support.includes(row)).map((row:any)=>[row.kind,row.path])).toEqual(nearest(dirname(join(root,fixture.program.projectRoot,"📜️script.ts"))).map(path=>["typescriptText",path]));
 const baseline=derived.row.fact as string,independent=()=>{const ts=createRequire(import.meta.url)("typescript"),config=ts.readConfigFile(join(root,"proj/tsconfig.json"),ts.sys.readFile),parsed=ts.parseJsonConfigFileContent(config.config,ts.sys,join(root,"proj")),program=ts.createProgram(parsed.fileNames,parsed.options);return program.getSourceFiles().map((file:any)=>file.fileName).filter((name:string)=>name.startsWith(root)&&!name.includes("/node_modules/")).sort();};
 const firstFiles=independent();
 for(const step of fixture.program.steps){
  for(const[path,text]of Object.entries(step.edit as Record<string,string|null>)){if(text===null)rmSync(join(root,path),{force:true});else write(root,{[path]:text});}
  const again=await observation.observeInferenceSnapshotV1(async()=>scriptInputs.deriveClosureFactV1(derived.row,createScriptInputCache({workspaceRoot:root})));
  expect(again.fact===baseline,step.name).toBe(step.same);expect(again.rows.length,step.name).toBeGreaterThan(0);
  expect(JSON.stringify(independent())===JSON.stringify(firstFiles),step.name+": the compiler's own program agrees").toBe(step.same);
}
},120000);
