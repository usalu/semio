"""R36 part 3c: a cold whole-graph read engages the project farm at its first miss. `coldInference` was read by the project nodes (`engage`) but set by nobody, so the farm opened only after 24 misses and 25 % of the started projects; the heaviest owners come first (held requests), so the parent inferred the 24 heaviest serially, minutes each, while the farm workers it had started slept."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
D=B+"🕸️graph/🧭️discovery/🟦️.ts"
add(D,'export async function discoverBuildGraphPartV1(','''/** 🏊️ What a part read tells the inference about its cold state: a cold part (no usable generation, the whole graph is inferred) engages the project farm at its first miss, a warm part leaves the farm to the proof that the work is cold. */
export function partInferenceEngagementV1(part:{cold:boolean}):{coldInference?:true}{return part.cold?{coldInference:true}:{};}

export async function discoverBuildGraphPartV1(''')
add(D,'...(part.cold?{}:{ownerRoots:part.roots}),','...(part.cold?{}:{ownerRoots:part.roots}),...partInferenceEngagementV1(part),')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'import{noteSlowV1,','import{partInferenceEngagementV1}from"../../../🕸️graph/🧭️discovery/🟦️.ts";\nimport{openProjectFarmV1,closeProjectFarmsV1}from"../../../🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs";\nimport{noteSlowV1,')
add(TF,'//R36b\n','''//R36b
test("a cold part engages the farm, a warm part does not, and an engaged farm opens at the first miss while a not engaged one waits for 24 misses",async()=>{
 expect(partInferenceEngagementV1({cold:true})).toEqual({coldInference:true});expect(partInferenceEngagementV1({cold:false})).toEqual({});
 const asked:number[]=[];
 const open=(engage:boolean)=>openProjectFarmV1({workspaceRoot:"/w",context:{registry:null} as any,contracts:{},closureStore:undefined,projects:5000,commandInputs:undefined,lateClosures:true,projectsByRoot:new Map(),componentRoots:[],environment:{SEMIO_GRAPH_FARM:"on",SEMIO_LANGUAGE:"en"} as any,launch:null as any,cores:8,engage,request:((input:any)=>{asked.push(input.preferredJobs);return{granted:Promise.resolve(undefined),release:()=>undefined};}) as any,spawnProcess:(() => {throw Error("no process in this proof");}) as any,write:()=>undefined});
 try{
  const cold=open(true);expect(asked.length).toBe(1);await cold?.close();await closeProjectFarmsV1();
  asked.length=0;const warm=open(false);expect(asked.length).toBe(0);
  for(let miss=0;miss<23;miss++)await warm?.prepare({name:"p"+miss,root:"r"+miss,json:{},native:false,pipeline:false,nativeRoot:"r",selectedInputs:[],head:[],cacheKey:"k"});
  expect(asked.length).toBe(0);
  await warm?.close();
 }finally{await closeProjectFarmsV1().catch(()=>undefined);}
});
''')
