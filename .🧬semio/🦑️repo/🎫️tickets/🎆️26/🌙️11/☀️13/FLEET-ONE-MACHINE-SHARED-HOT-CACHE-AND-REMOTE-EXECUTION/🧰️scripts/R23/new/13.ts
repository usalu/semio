import Ajv from"ajv";
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{UnportableV1,aboveRootV1,bundleKeysV1,closureKeyV1,dependencyGenerationV1,hostOfRootV1,localBundleV1,localPathV1,localRecordV1,localRowsV1,neutralBundleV1,neutralPathV1,neutralRecordV1,neutralTextV1,localTextV1,portableRowsPolicyV1,portableRowsV1,type ClosureBundleV1,type PortableRecordV1}from"../🟦️.ts";

/** 🧳️ Contract: a record is published without the facts that name the producer's machine and in a neutral form of the host's spelling (root token, forward slashes, provider owner token, neutral closure keys), and is replayed in the spelling of the taker with its own dependency generation, providers and closure keys. The fixture is language-agnostic; a Python oracle with its own Windows path semantics and its own digest derives the same forms from the schema alone. */
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Fixture",definitions:schema.definitions});

test("the fixture and the policy are valid against the schema",()=>{
 expect(validate(fixture)).toBe(true);
 expect(new Ajv({strict:false}).validate({$ref:"#/definitions/Policy",definitions:schema.definitions},portableRowsPolicyV1)).toBe(true);
});

test("every case publishes the portable rows and replays them with the generation of the taker",()=>{
 for(const row of fixture.cases){
  const portable=portableRowsV1(row.rows,row.root);
  expect(portable,row.name).toEqual(row.portable);
  expect(localRowsV1(portable,row.generation??undefined),row.name).toEqual(row.local);
 }
});

test("only directories above the checkout are location rows",()=>{
 expect(aboveRootV1("/Users","/Users/Shared/semio")).toBe(true);
 expect(aboveRootV1("/","/Users/Shared/semio")).toBe(true);
 expect(aboveRootV1("/Users/Shared/semio","/Users/Shared/semio")).toBe(false);
 expect(aboveRootV1("/Users/Shared/semio2","/Users/Shared/semio")).toBe(false);
 expect(aboveRootV1("C:\\work","C:\\work\\semio")).toBe(true);
 expect(aboveRootV1("/Users/Shared/semio/.cache","/Users/Shared/semio")).toBe(false);
});

test("the production shape: a record of one machine holds on a machine of another platform and checkout after the rewrite",()=>{
 const producer=fixture.cases[0]!,rows=portableRowsV1(producer.rows,producer.root),taker=localRowsV1(rows,"8714ffcdb7fdf62c3f57");
 const text=JSON.stringify(taker);
 expect(text).not.toContain("86415d695aa883fffc22");expect(text).not.toContain("\"/Users\"");expect(text).toContain("8714ffcdb7fdf62c3f57/node_modules/typescript/lib/typescript.js");
 expect(taker.filter(row=>row.kind==="file")).toEqual(producer.rows.filter(row=>row.kind==="file"));
});

test("the generation of a machine is the real path of its node_modules; without one nothing is claimed",()=>{
 const base=mkdtempSync(join(tmpdir(),"portable-rows-"));
 try{
  const root=join(base,"root"),generation=join(base,"root",".cache","b","d","m","j","0123456789abcdef0123","node_modules");
  mkdirSync(generation,{recursive:true});symlinkSync(generation,join(root,"node_modules"));
  expect(dependencyGenerationV1(realpathSync(root))).toBe("0123456789abcdef0123");
  expect(dependencyGenerationV1(join(base,"missing"))).toBeUndefined();
  mkdirSync(join(base,"plain","node_modules"),{recursive:true});expect(dependencyGenerationV1(join(base,"plain"))).toBeUndefined();
  expect(dependencyGenerationV1("/x",()=>"/x/.cache/b/d/m/j/aaaaaaaaaaaaaaaaaaaa/node_modules")).toBe("aaaaaaaaaaaaaaaaaaaa");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the Python oracle derives the same rows from the schema",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=(JSON.parse(run.stdout) as {cases:{name:string;portable:unknown;local:unknown}[]}).cases;
 expect(oracle.map(row=>row.portable)).toEqual(fixture.cases.map(row=>portableRowsV1(row.rows,row.root)));
 expect(oracle.map(row=>row.local)).toEqual(fixture.cases.map(row=>localRowsV1(portableRowsV1(row.rows,row.root),row.generation??undefined)));
});

type Bundle=Readonly<{recipeKey:string;bundle:ClosureBundleV1}>;
type Taker={root:string;providers:string[];generation:string|null;local:PortableRecordV1|null;bundles?:Bundle[]};
type RecordCase={name:string;root:string;producer:PortableRecordV1;neutral:PortableRecordV1|null;bundles?:Bundle[];neutralBundles?:Bundle[];takers:Taker[]};
const parsedImage=(record:PortableRecordV1)=>({...record,image:JSON.parse(record.image)});

test("the path table: the root token and forward slashes below the root of every host, the Windows root compared without case, other paths untouched",()=>{
 expect(fixture.paths.length).toBeGreaterThan(12);
 for(const row of fixture.paths){
  const host=hostOfRootV1(row.root),neutral=neutralPathV1(row.path,host);
  expect(neutral,row.name).toBe(row.neutral);
  expect(neutral===row.path?null:localPathV1(neutral,host),row.name).toBe(row.local);
 }
 for(const row of fixture.hosts)expect(localPathV1(row.neutral,hostOfRootV1(row.root)),row.name).toBe(row.local);
 expect(hostOfRootV1("C:\\semio").windows).toBe(true);expect(hostOfRootV1("c:/semio").windows).toBe(true);expect(hostOfRootV1("\\\\server\\share\\x").windows).toBe(true);expect(hostOfRootV1("/Users/x").windows).toBe(false);
 expect(neutralPathV1("/Users/Shared/semio/dir/",hostOfRootV1("/Users/Shared/semio"))).toBe("⟦semio-root⟧/dir/");
 expect(localPathV1("⟦semio-root⟧/dir/",hostOfRootV1("C:\\semio"))).toBe("C:\\semio\\dir\\");
});

test("the above-root rule holds for any drive and without case on Windows",()=>{
 expect(aboveRootV1("c:\\","C:\\semio")).toBe(true);expect(aboveRootV1("C:\\SEMIO","c:/semio")).toBe(false);expect(aboveRootV1("D:\\","C:\\semio")).toBe(false);
 expect(aboveRootV1("/users","/Users/Shared/semio")).toBe(false);
});

test("every record case is published neutral and taken in the spelling of each taker, with the closure bundles, as the oracle derives it",()=>{
 const cases=fixture.records as unknown as RecordCase[];
 expect(cases.length).toBeGreaterThanOrEqual(6);
 for(const row of cases){
  const host=hostOfRootV1(row.root);
  if(row.neutral===null){expect(()=>neutralRecordV1(row.producer,host),row.name).toThrow(UnportableV1);continue;}
  const{record,keys}=neutralRecordV1(row.producer,host);
  expect(parsedImage(record),row.name).toEqual(parsedImage(row.neutral));
  expect(JSON.stringify(record)).not.toContain(row.root.replaceAll("\\","\\\\"));
  const fromBundles=bundleKeysV1(row.bundles??[],host,"neutral"),all=new Map([...keys,...fromBundles]);
  const bundles=(row.bundles??[]).map(bundle=>neutralBundleV1(bundle,host,all)!);
  expect(bundles,row.name).toEqual(row.neutralBundles??[]);
  for(const taker of row.takers){
   const there=hostOfRootV1(taker.root),local=localRecordV1(record,there,{providers:taker.providers,generation:taker.generation??undefined});
   expect(parsedImage(local.record),`${row.name} -> ${taker.root}`).toEqual(parsedImage(taker.local!));
   const keysLocal=new Map(local.keys);
   for(const bundle of row.neutralBundles??[]){const moved=localBundleV1(bundle,there,new Map([...keysLocal,...bundleKeysV1([bundle],there,"local")]),{providers:taker.providers,generation:taker.generation??undefined});keysLocal.set(bundle.recipeKey,moved!.recipeKey);}
   const moved=(row.neutralBundles??[]).map(bundle=>localBundleV1(bundle,there,keysLocal,{providers:taker.providers,generation:taker.generation??undefined})!);
   expect(moved,`${row.name} bundles -> ${taker.root}`).toEqual(taker.bundles??[]);
   for(const bundle of moved)expect(closureKeyV1(bundle.bundle.recipe)).toBe(bundle.recipeKey);
  }
 }
});

test("the keys of a taken record are the keys of its recipes in the spelling of the taker: the table, the markers and the closure rows follow",()=>{
 const row=(fixture.records as unknown as RecordCase[])[0]!,there=hostOfRootV1("D:\\work\\semio"),taken=localRecordV1(neutralRecordV1(row.producer,hostOfRootV1(row.root)).record,there,{providers:[],generation:undefined}).record;
 const image=JSON.parse(taken.image) as {closureRecipes:Record<string,unknown>;targets:{build:{inputs:{closureRef:string}[]}}};
 for(const[key,recipe] of Object.entries(image.closureRecipes))expect(closureKeyV1(recipe)).toBe(key);
 expect(image.targets.build.inputs.map(marker=>marker.closureRef).sort()).toEqual(Object.keys(image.closureRecipes).sort());
 expect(JSON.stringify(image.closureRecipes)).toContain("D:/work/semio/libs/alpha/script.ts");
 expect(taken.observations.filter(item=>item.kind==="closure").some(item=>item.path.includes("D:/work/semio/libs/alpha/script.ts"))).toBe(true);
 for(const closure of taken.observations.filter(item=>item.kind==="closure"))expect(Object.keys(image.closureRecipes)).toContain((JSON.parse(closure.path) as string[])[1]);
 expect(taken.observations.filter(item=>item.kind==="closure").map(item=>(JSON.parse(item.path) as string[])[0])).toEqual(["D:\\work\\semio","D:\\work\\semio","D:\\work\\semio"]);
});

test("a text with a root token of another spelling is refused on taking, a Windows text with backslashes on publishing",()=>{
 expect(()=>localTextV1("x ⟦semio-root-slash⟧/a",hostOfRootV1("/r"))).toThrow(UnportableV1);
 expect(()=>neutralTextV1("C:\\semio\\a",hostOfRootV1("C:\\semio"))).toThrow(UnportableV1);
 expect(()=>neutralTextV1("C:\\\\semio\\\\a",hostOfRootV1("C:\\semio"))).toThrow(UnportableV1);
 expect(neutralTextV1("repo C:/semio/a and c:/SEMIO/b",hostOfRootV1("C:\\semio"))).toBe("repo ⟦semio-root⟧/a and ⟦semio-root⟧/b");
 expect(neutralTextV1("C:/semio2/a",hostOfRootV1("C:\\semio"))).toBe("C:/semio2/a");
});

test("the Python oracle with its own Windows path semantics and digest derives the same paths, hosts and records",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8",maxBuffer:2**28});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {paths:{neutral:string;local:string|null}[];hosts:{local:string}[];records:{neutral:PortableRecordV1|null;takers:{local:PortableRecordV1|null}[]}[]};
 expect(oracle.paths.map(row=>[row.neutral,row.local])).toEqual(fixture.paths.map(row=>[neutralPathV1(row.path,hostOfRootV1(row.root)),row.local]));
 expect(oracle.hosts.map(row=>row.local)).toEqual(fixture.hosts.map(row=>localPathV1(row.neutral,hostOfRootV1(row.root))));
 const cases=fixture.records as unknown as RecordCase[];
 expect(oracle.records.map(row=>row.neutral===null?null:parsedImage(row.neutral))).toEqual(cases.map(row=>{try{return parsedImage(neutralRecordV1(row.producer,hostOfRootV1(row.root)).record);}catch{return null;}}));
 expect(oracle.records.map(row=>row.takers.map(taker=>parsedImage(taker.local!)))).toEqual(cases.map(row=>row.neutral===null?[]:row.takers.map(taker=>parsedImage(localRecordV1(neutralRecordV1(row.producer,hostOfRootV1(row.root)).record,hostOfRootV1(taker.root),{providers:taker.providers,generation:taker.generation??undefined}).record))));
});
