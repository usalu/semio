/** 🪤️ Contract: a look at a stored owner record (the take pass) never derives a closure. A recipe whose entry is not stored ends the look with `BUILD_TAKE_MISS` and derives nothing; the same recipe outside a take is derived; a take that follows finds the stored entry and answers the same roster without deriving again. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

test("a take never derives a closure: a missing entry is a miss, outside a take it is derived, a later take finds it",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("take-closure-");roots.push(root);
 write(root,{"tools/a.mjs":'import "./b.mjs";\nexport const a=1;\n',"tools/b.mjs":'import "./c.mjs";\nexport const b=2;\n',"tools/c.mjs":"export const c=3;\n"});
 scriptInputs.resetClosureEntriesV1();
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),entry=join(root,"tools/a.mjs");
 const handle=scriptInputs.closureHandleOfV1(await api.closureOf([entry],[],"test-discovery")),recipes=[[handle.key,handle.recipe]];
 const derived=()=>scriptInputs.closureEntryCountsV1().derived as number,start=derived();
 await expect(scriptInputs.runClosureTakeV1(()=>api.resolve(recipes))).rejects.toMatchObject({code:"BUILD_TAKE_MISS"});
 expect(derived()).toBe(start);
 const resolved=await api.resolve(recipes);
 expect(derived()).toBe(start+1);
 const roster=(resolved.values.get(handle.key) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort();
 expect(roster).toEqual(["tools/a.mjs","tools/b.mjs","tools/c.mjs"]);
 const taken=await scriptInputs.runClosureTakeV1(()=>api.resolve(recipes));
 expect((taken.values.get(handle.key) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort()).toEqual(roster);
 expect(derived()).toBe(start+1);
}));
