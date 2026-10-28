"""🧾 [audit7] Appends the laws of batch `contracts` to the captured farm suite (anchored; idempotent). usage: edit_law.py <root>"""
import os, sys
root = sys.argv[1]
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🪆️captured/🟦️.ts"
IMPORT = ('import{serialize}from"node:v8";', 'import{deserialize,serialize}from"node:v8";')
LAWS = r'''
/** 📜️ The generator contracts of a parent that read them before a peer edit: a closed stand-in that no taxonomy holds. */
const parentContracts={"law-generator":{outputs:["law-output.json"],inputs:["law-input.ts"]}};

test("a worker infers with the generator contracts its parent keyed the job with, never with a later read of the taxonomy, and refuses a session without them",()=>{
 const real=realpathSync(root),library=join(real,"🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build");
 const code=`const L=${JSON.stringify(library)},root=${JSON.stringify(real)},parent=${JSON.stringify(parentContracts)};const{libraryBootstrap}=await import(L+"/🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;const{createProjectInferenceEnvironmentV1}=await import(L+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");const{observeInferenceSnapshotV1}=await import(L+"/👁️observation/🟨️.mjs");const open=contracts=>observeInferenceSnapshotV1(()=>createProjectInferenceEnvironmentV1({version:3,workspaceRoot:root,registry:null,contracts,commandInputs:null,lateClosures:false,projectsByRoot:[],componentRoots:[],stagingBase:root}));const environment=await open(parent);let refused=null;try{await open(undefined);}catch(error){refused=String(error?.message??error);}process.stdout.write(JSON.stringify({contracts:environment.contracts,refused}));`;
 const run=Bun.spawnSync([process.execPath,"-e",code],{cwd:real,env:{...process.env},stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString().slice(-2000)).toBe(0);
 const answer=JSON.parse(run.stdout.toString()) as {contracts:unknown;refused:string|null};
 const oracle=Bun.spawnSync([python!,"-I","-c","import hashlib,json,sys\nparent=json.loads(sys.argv[1]);disk=json.load(open(sys.argv[2],encoding='utf8')).get('generatorContracts',{})\nprint(json.dumps({'differs':disk!=parent,'head':hashlib.sha256(json.dumps([parent],separators=(',',':'),ensure_ascii=False).encode()).hexdigest()}))",JSON.stringify(parentContracts),join(real,"🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🔣️taxonomy.json")]);
 const expected=JSON.parse(oracle.stdout.toString()) as {differs:boolean;head:string};
 expect(expected.differs).toBe(true);
 expect(answer.contracts).toEqual(parentContracts);
 expect(sha(JSON.stringify([answer.contracts]))).toBe(expected.head);
 expect(answer.refused).toBe("Generator contracts of the parent inference required");
},120000);

test("the farm hands every worker the generator contracts its parent read once, in the session it starts the worker with",async()=>{
 const plan=JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536}),job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)}),sessions:unknown[]=[];
 class SessionChild extends ScriptedChild{override send(message:{type:string;id?:number;sessionFile?:string}){if(message.type==="init")sessions.push(deserialize(readFileSync(message.sessionFile!)));return super.send(message);}}
 const farm=openProjectFarmV1({workspaceRoot:area(),context:{registry:null},contracts:parentContracts,launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>24,cores:10,write:()=>undefined,
  spawnProcess:(()=>new SessionChild(graphFarmIdentityV1(inferenceRecordIdentityV1(),null))) as never,request:(()=>({granted:Promise.resolve(undefined),release:()=>undefined})) as never} as never)!;
 for(let index=0;index<GRAPH_FARM_POLICY_V1.minimumProjects;index++)await farm.prepare(job("miss"+index));
 await new Promise<number>(done=>farm.onReady(done));
 await farm.close();
 expect(sessions).toHaveLength(1);
 expect((sessions[0] as {contracts:unknown}).contracts).toEqual(parentContracts);
},60000);
'''
path = os.path.join(root, REL)
text = open(path, encoding="utf8").read()
if "a worker infers with the generator contracts its parent keyed the job with" in text: sys.exit(0)
if text.count(IMPORT[0]) != 1: print("[audit7] import anchor lost"); sys.exit(1)
if text.count("class ScriptedChild extends EventEmitter") != 1: print("[audit7] ScriptedChild anchor lost"); sys.exit(1)
text = text.replace(IMPORT[0], IMPORT[1]).rstrip("\n") + "\n" + LAWS
mode = os.stat(path).st_mode & 0o7777
with open(path, "w", encoding="utf8") as handle: handle.write(text)
os.chmod(path, mode)
