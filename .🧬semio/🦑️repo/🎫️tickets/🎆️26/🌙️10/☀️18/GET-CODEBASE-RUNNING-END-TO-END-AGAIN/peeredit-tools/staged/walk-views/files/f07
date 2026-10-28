/** 👓 Contract: a project holds a command walk through the view of the closure recipe it reads it with. A target's own closure (target-sources, generic-target) drops every scheduling file, a Rust command's closure (native-command, native-target) keeps the native-build modules; so the derived fact such a project holds stays equal after an edit that only moves files its view leaves out (a peer edit of scheduling code leaves product owners verified), and changes as soon as its view's roster or compiled modules move (an edit that changes a command's inputs still re-infers). A recipe the policy does not name (closure-of, the test discovery) keeps holding the whole walk. The real walk, the real `resolve` and the real `deriveClosureFactV1` run under the real observation layer on a tiny workspace; an independent Python model (reading the scheduling table itself) states every verdict and roster, and esbuild's bundle graph states the whole roster. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️.json" with {type:"json"};
import policySchema from "../🧬️schema/🔣️.json" with {type:"json"};
import table from "../../../../../📥️source/🔏️closure/🗓️scheduling/🔣️.json" with {type:"json"};

setDefaultTimeout(120000);
type Verdict={whole:boolean;scheduling:boolean;native:boolean};
const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🟨️.mjs") as any,views=await import("../🟨️.mjs") as any;
const python=requireTestToolV1("python"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const oracle=():{rosters:string[][];verdicts:Verdict[]}=>{const run=spawnSync(python,["-I",join(import.meta.dir,"🐍️oracle","🐍️.py"),join(import.meta.dir,"🔣️.json"),join(import.meta.dir,"../../../../../📥️source/🔏️closure/🗓️scheduling/🔣️.json")],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const esbuildRoster=async(root:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,corpus.entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};
const target={options:{cwd:"proj",command:"repo ./📜️script.ts build"}};
const resolveRecipes=(root:string)=>observation.observeInferenceSnapshotV1(async()=>{
 const scripts=scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
 const own=scriptInputs.closureHandleOfV1(await api.genericTargetCommandInputs(target,"proj")),whole=scriptInputs.closureHandleOfV1(await api.closureOf([join(root,corpus.entry)],[]));
 return{own:await api.resolve([[own.key,own.recipe]]),whole:await api.resolve([[whole.key,whole.recipe]])};
});
const walks=(resolved:any)=>resolved.derived.filter((fact:any)=>JSON.parse(fact.row.path).walk===1);
const ask=(row:any,root:string)=>observation.observeInferenceSnapshotV1(()=>scriptInputs.deriveClosureFactV1(row,scriptInputs.createScriptInputCache({workspaceRoot:root})));
const viewed=(row:any,view:string)=>({kind:"derived",path:JSON.stringify({...JSON.parse(row.path),view}),fact:""});

test("the view policy is closed under its schema, the native view leaves out exactly the scheduling prefixes no native-keyed module lies in, and an unknown view is refused",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(policySchema,policy)).toBe(true);expect(validateJsonSchemaSubset(policySchema,policy)).toEqual([]);
 expect(validateJsonSchemaSubset(policySchema,{...policy,recipes:{"target-sources":"everything"}}).length).toBeGreaterThan(0);
 const scheduling=[...new Set(table.domains.flatMap(domain=>domain.prefixes))].sort(),keyed=table.domains.flatMap(domain=>(domain as {nativeKeyed?:string[]}).nativeKeyed??[]);
 expect([...views.walkViewPrefixesV1("scheduling")]).toEqual(scheduling);
 expect([...views.walkViewPrefixesV1("native")]).toEqual(scheduling.filter(prefix=>!keyed.some(row=>row.startsWith(prefix)||prefix.startsWith(row))));
 expect(views.walkViewOfRecipeV1("target-sources")).toBe("scheduling");expect(views.walkViewOfRecipeV1("native-command")).toBe("native");expect(views.walkViewOfRecipeV1("closure-of")).toBeUndefined();
 expect(()=>views.walkViewPrefixesV1("everything")).toThrow();
 expect(views.walkViewAnswerV1(["{workspaceRoot}/a.ts"],["{workspaceRoot}/a.ts"],[],undefined)).not.toBe(views.walkViewAnswerV1(["{workspaceRoot}/a.ts"],["{workspaceRoot}/a.ts"],[],"scheduling"));
});

test("the corpus is closed under its schema and every authored verdict is the independent model's",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(corpusSchema,corpus)).toBe(true);
 expect(oracle().verdicts).toEqual(corpus.mutations.map(mutation=>mutation.changed));
});

test("a target's own closure holds the walk through the scheduling view and the whole walk stays with closure-of; each held fact follows exactly its view's roster across the edits",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"walk-views-")));roots.push(root);write(root,corpus.files);
 const first=await resolveRecipes(root),own=walks(first.own),whole=walks(first.whole);
 expect(own).toHaveLength(1);expect(whole).toHaveLength(1);
 expect(JSON.parse(own[0].row.path).view).toBe("scheduling");expect(JSON.parse(whole[0].row.path).view).toBeUndefined();
 const held={whole:whole[0].row,scheduling:own[0].row,native:viewed(own[0].row,"native")} as Record<keyof Verdict,any>;
 const facts=Object.fromEntries(await Promise.all(Object.entries(held).map(async([name,row])=>[name,(await ask(row,root)).fact]))) as Record<keyof Verdict,string>;
 expect(facts.whole).toBe(held.whole.fact);expect(facts.scheduling).toBe(held.scheduling.fact);
 const model=oracle();
 for(const[index,mutation]of corpus.mutations.entries()){
  write(root,mutation.edit);
  const verdict={} as Verdict;
  for(const name of ["whole","scheduling","native"] as const){const again=(await ask(held[name],root)).fact;verdict[name]=again!==facts[name];facts[name]=again;}
  expect(verdict,mutation.name).toEqual(mutation.changed);
  const fresh=await resolveRecipes(root);
  expect(walks(fresh.own)[0].row.fact,mutation.name+": the held fact is what a fresh inference holds").toBe(facts.scheduling);
  expect(walks(fresh.whole)[0].row.fact,mutation.name).toBe(facts.whole);
  const roster=(fresh.whole.values.values().next().value as string[]).filter(path=>typeof path==="string").map(path=>path.replace("{workspaceRoot}/","")).sort();
  expect(roster,mutation.name).toEqual(model.rosters[index]!);expect(roster,mutation.name).toEqual(await esbuildRoster(root));
 }
});

test("a walk question naming a view the policy does not carry is refused",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"walk-views-refuse-")));roots.push(root);write(root,corpus.files);
 const whole=walks((await resolveRecipes(root)).whole)[0];
 await expect(scriptInputs.deriveClosureFactV1(viewed(whole.row,"everything"),scriptInputs.createScriptInputCache({workspaceRoot:root}))).rejects.toBeDefined();
});
