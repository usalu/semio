import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import {readFileSync,readdirSync} from "node:fs";
import {join,resolve,dirname,relative,sep} from "node:path";

type Mapping={module:string;owner:string;namespace:string};
const workspace=process.cwd(),prefix="semio-framework-3d-brep-",retired=prefix+"operations",corpus=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")) as {version:1;cases:Mapping[];sourceRoots:string[]};
const sources:string[]=[],manifests:string[]=[];
function collect(directory:string):void{for(const row of readdirSync(directory,{withFileTypes:true})){if(row.isSymbolicLink()||["node_modules",".git",".cache",".🧬semio","target","dist"].includes(row.name))continue;const path=join(directory,row.name);if(row.isDirectory())collect(path);else if(row.isFile()){if(row.name==="Cargo.toml")manifests.push(path);else if(row.name.endsWith(".rs")&&corpus.sourceRoots.some(root=>path.startsWith(join(workspace,root)+sep)))sources.push(path);}}}
for(const directory of ["🧰️framework","✏️s","🌎️hub"])collect(join(workspace,directory));
const parsed=new Map(manifests.map(path=>{const source=readFileSync(path,"utf8"),value=Bun.TOML.parse(source) as any;expect(value).toEqual(TOML.parse(source));return[path,value] as const;}));

test("neutral BREP namespace ownership agrees with SQLite and physical canonical packages",()=>{
 expect(new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8")))(corpus)).toBe(true);
 const database=new Database(":memory:");try{database.exec("CREATE TABLE ownership(module TEXT PRIMARY KEY,owner TEXT NOT NULL,namespace TEXT NOT NULL)");const insert=database.prepare("INSERT INTO ownership VALUES (?,?,?)");for(const row of corpus.cases)insert.run(row.module,row.owner,row.namespace);
 for(const row of corpus.cases){const actual=database.query("SELECT owner,namespace FROM ownership WHERE module=?").get(row.module);expect(actual).toEqual({owner:row.owner,namespace:row.namespace});const owners=[...parsed].filter(([,value])=>value.package?.name===row.owner);expect(owners.length,row.owner).toBe(1);const [manifest,value]=owners[0]!,source=readFileSync(resolve(dirname(manifest),value.lib.path),"utf8");if(row.namespace)expect(source,manifest).toContain("pub mod "+row.namespace+";");else expect(source).toMatch(/pub (?:fn|struct|enum|use) /u);}
 }finally{database.close();}
});

test("known authored BREP consumers use actual package identities and direct namespace owners",()=>{
 const failures:string[]=[];let edges=0;
 for(const[path,value]of parsed){for(const section of [value.dependencies,value["dev-dependencies"],value["build-dependencies"],value.workspace?.dependencies]){for(const[key,row]of Object.entries(section??{}) as [string,any][]){if(key===retired||row?.package===retired)failures.push(relative(workspace,path)+": retired package "+key);if(typeof row?.path==="string"&&key.startsWith(prefix)){const destination=resolve(dirname(path),row.path,"Cargo.toml"),physical=parsed.get(destination);if(!physical||physical.package?.name!==(row.package??key))failures.push(relative(workspace,path)+": physical package disagrees with "+key);edges++;}}}}
 for(const path of sources){const source=readFileSync(path,"utf8");if(source.includes("semio_framework_3d_brep_operations"))failures.push(relative(workspace,path)+": retired Rust namespace");}
 const registry=JSON.parse(readFileSync(join(workspace,"📋️build-owners.json"),"utf8"));if(JSON.stringify(registry).includes('"'+retired+'"'))failures.push("build owner registry retains retired package");
 expect(failures).toEqual([]);expect(edges).toBeGreaterThan(20);console.log("[DEBUG] Canonical BREP package/TOML and Source ownership "+JSON.stringify({manifests:parsed.size,sources:sources.length,physicalEdges:edges}));
});
