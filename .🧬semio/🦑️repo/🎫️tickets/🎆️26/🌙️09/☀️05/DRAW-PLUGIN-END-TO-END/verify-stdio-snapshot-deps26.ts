import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { dirname, join, resolve } from "node:path";
import law from "./stdio-snapshot-deps26.json" with {type:"json"};

const artifacts = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";
const prefix = "semio-s-artifact-stdio-";
const manifests = new Bun.Glob("**/Cargo.toml");
const sources = new Bun.Glob("**/*.rs");
type Manifest = {
 package?:{name:string};
 dependencies?:Record<string,{workspace?:boolean}>;
 workspace?:{dependencies?:Record<string,{path?:string}>};
};
const toml = async (file:string) => Bun.TOML.parse(await Bun.file(file).text()) as Manifest;

for(const family of law.families)test(`${family.folder} only its actual TXT converter owners depend on the snapshot library`,async()=>{
 const root=resolve(artifacts,family.folder),fixture=family.owners as Record<string,string[]>;
 const expectedPackages=family.packages.map(name=>prefix+name);
 const mountDirectories=new Map<string,string>();
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE actual (owner TEXT, dependency TEXT); CREATE TABLE expected (owner TEXT, dependency TEXT);");
  const expected=db.query("INSERT INTO expected VALUES (?1,?2)");
  for(const owner of Object.keys(fixture))expected.run(prefix+owner,law.snapshotOwner);
  const actual=db.query("INSERT INTO actual VALUES (?1,?2)");
  const observed:string[]=[];
  for await(const file of manifests.scan({cwd:root})){
   const parsed=await toml(join(root,file)),owner=parsed.package?.name;
   if(!owner||!expectedPackages.includes(owner))continue;
   observed.push(owner);
   mountDirectories.set(dirname(file).replaceAll("\\","/"),owner);
   for(const dependency of [law.snapshotOwner,law.excludedFacade]){
    if(parsed.dependencies?.[dependency])actual.run(owner,dependency);
   }
  }
  expect(observed.sort()).toEqual(expectedPackages.sort());
  expect(db.query("SELECT * FROM (SELECT * FROM actual EXCEPT SELECT * FROM expected) UNION ALL SELECT * FROM (SELECT * FROM expected EXCEPT SELECT * FROM actual)").all()).toEqual([]);
 }finally{db.close();}
 const workspace=(await toml(join(root,"Cargo.toml"))).workspace?.dependencies;
 expect(workspace?.[law.excludedFacade]).toBeUndefined();
 const selected=workspace?.[law.snapshotOwner]?.path;
 expect(selected).toBeDefined();
 expect((await toml(resolve(root,selected!,"Cargo.toml"))).package?.name).toBe(law.snapshotOwner);
 const references:string[]=[],mounts:string[]=[];
 for await(const relative of sources.scan({cwd:root})){
  const file=relative.replaceAll("\\","/");
  const content=await Bun.file(join(root,file)).text();
  expect(content).not.toMatch(/\bsemio_s_artifact_stdio_txt\s*::/u);
  if(/\buse semio_s_artifact_stdio_txt_schema::TxtSnapshot;/u.test(content)){
   const direction=/📥️import\/🧩️deserializers/u.test(file)?"import":/📤️export\/🧵️serializers/u.test(file)?"export":file;
   references.push(direction);
  }
  for(const match of content.matchAll(/#\[path\s*=\s*"[^"\n]*\/(?:📥️(import)\/🧩️deserializers|📤️(export)\/🧵️serializers)\/🗿️artifacts\/🔤️txt\/🔖️utf-8\/✳️any\/🦀️.rs"\]/gu)){
   const directory=[...mountDirectories.keys()].find(base=>file.startsWith(base+"/"));
   mounts.push(`${mountDirectories.get(directory??"")}:${match[1]??match[2]}`);
  }
 }
 expect(references.sort()).toEqual(Object.values(fixture).flat().sort());
 expect(mounts.sort()).toEqual(Object.entries(fixture).flatMap(([owner,directions])=>directions.map(direction=>`${prefix}${owner}:${direction}`)).sort());
});

test("the Draw export owner excludes its unused full DEFLATE dependency",async()=>{
 const {workspace,manifest,package:owner,excludedDependency}=law.drawExport;
 const packageManifest=await toml(join(workspace,manifest)),root=await toml(join(workspace,"Cargo.toml"));
 expect(packageManifest.package?.name).toBe(owner);
 expect(packageManifest.dependencies?.[excludedDependency]).toBeUndefined();
 expect(root.workspace?.dependencies?.[excludedDependency]).toBeUndefined();
});
