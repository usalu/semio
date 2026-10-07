import {admitTaxonomyCargoMembershipFacts} from "../../../🟦️.ts";
import {runTaxonomyCliWorkflow} from "../../../🎮️command-contract/🔁️workflow/🟦️.ts";
import {expect,test} from "bun:test";
import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join,dirname} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import ts from "typescript";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {admitCargoMembership,type CargoMembershipFact,type CargoDiscoveryOperation} from "../../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {CargoDiscoveryWorkspace} from "../../../../🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts";
const root=process.env.SEMIO_NORMALIZATION_LAW_SNAPSHOT,output=process.env.SEMIO_NORMALIZATION_LAW_OUTPUT;
if(!root||!output)throw Error("Explicit normalization law owners missing");
const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",taxonomy=JSON.parse(readFileSync(join(root,library,"🔣️taxonomy.json"),"utf8")),catalog=JSON.parse(readFileSync(join(root,taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogPath),"utf8")),row=catalog.packages.find((row:{id:string})=>row.id==="wgpu-renderer");
if(!row)throw Error("Required exact package catalog missing");
const source=readFileSync(join(root,library,"🔍️discovery/🟦️.ts"),"utf8"),tree=ts.createSourceFile("discovery.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="semanticPackageProjectionAuthority");
if(!owner)throw Error("Required pure authority definition missing");
const expressions:ts.Expression[]=[];
const visit=(node:ts.Node)=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.name.text==="declared"&&node.initializer)expressions.push(node.initializer);ts.forEachChild(node,visit);};visit(owner);
if(expressions.length!==1)throw Error("Required exact admitted-fact expression missing");
const expression=expressions[0]!.getText(tree),project=new Function("facts","activeRoot","return "+expression) as (facts:Readonly<{cargoMembership:CargoMembershipFact|null}>,activeRoot:string)=>boolean;

/** 🎛️ Authors one finite fixture control with independently retained progress. */
function operation(){const workspace=new CargoDiscoveryWorkspace(),progress:unknown[]=[],control:CargoDiscoveryOperation={workspace,signal:new AbortController().signal,maximumUnits:2097152,maximumOwnedBytes:16777216,maximumDepth:32,onProgress:event=>{progress.push(event);},yieldContinuation:async()=>{}};return{workspace,progress,control};}

test("same-read membership admission and pure required facts agree with SQLite and Iarna",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const database=new Database(":memory:"),observations:unknown[]=[];
 try{for(const entry of corpus.cases){const directory=entry.layout==="destination"?row.destinationRoot:row.sourceRoot,member=entry.member==="exact"?directory:entry.member==="prefix"?directory+"-imposter":"another/package",members=[member],exclude=entry.excluded?[directory]:[],fixture=join(output,entry.id),body="[workspace]\nmembers="+JSON.stringify(members)+"\nexclude="+JSON.stringify(exclude)+"\n";mkdirSync(fixture,{recursive:true});writeFileSync(join(fixture,"Cargo.toml"),body);const parsed=parse(body),workspace=Object.getOwnPropertyDescriptor(parsed,"workspace")?.value,oracle=database.query("select exists(select 1 from json_each(?1) where value=?3) and not exists(select 1 from json_each(?2) where value=?3) as declared").get(JSON.stringify(workspace.members),JSON.stringify(workspace.exclude),directory) as {declared:number};const state=operation(),fact=await admitCargoMembership(fixture,"Cargo.toml",directory,state.control);expect(fact).toEqual({directory,declared:entry.declared});expect(oracle.declared===1).toBe(entry.declared);expect(project({cargoMembership:fact},directory)).toBe(entry.declared);expect(project({cargoMembership:{...fact,directory:directory+"-imposter"}},directory)).toBe(false);expect(project({cargoMembership:null},directory)).toBe(false);expect(state.workspace.manifestOwners[0]?.pages.join("")).toBe(body);expect(state.progress.length).toBeGreaterThan(0);observations.push({id:entry.id,fact,oracle,source:body,pages:state.workspace.manifestOwners[0]?.pages,units:state.workspace.completed,ownedBytes:state.workspace.ownedBytes,progress:state.progress.length});console.log("[DEBUG] "+JSON.stringify({case:entry.id,fact,oracle,units:state.workspace.completed,ownedBytes:state.workspace.ownedBytes}));}}
 finally{database.close();writeFileSync(join(output,"observation.json"),JSON.stringify({observations,expression,sourceWrites:0,scope:"Actual controlled physical membership and exact pure defining expression; no whole normalization transaction or preview acceptance"}));}
});

test("zero ownership credit refuses physical membership before any admitted fact",async()=>{const state=operation();await expect(admitCargoMembership(join(output,"source"),"Cargo.toml",row.sourceRoot,{...state.control,maximumUnits:0})).rejects.toThrow();expect(state.workspace.partialRecords).toEqual([]);});


test("actual defining async caller admits both exact layouts from one controlled root manifest",async()=>{
 const fixture=join(output,"defining-caller"),taxonomyPath=library+"/🔣️taxonomy.json",catalogPath=taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"].authorityCatalogPath;
 for(const path of [taxonomyPath,catalogPath]){mkdirSync(dirname(join(fixture,path)),{recursive:true});writeFileSync(join(fixture,path),readFileSync(join(root,path),"utf8"));}
 const members=[row.sourceRoot,row.destinationRoot],body="[workspace]\nmembers="+JSON.stringify(members)+"\nexclude=[]\n";writeFileSync(join(fixture,"Cargo.toml"),body);
 const state=operation(),facts=await admitTaxonomyCargoMembershipFacts({repoRoot:fixture,taxonomyPath},state.control),database=new Database(":memory:");
 try{const oracle=database.query<{directory:string;declared:number},[string]>("select value as directory,1 as declared from json_each(?)").all(JSON.stringify(members));expect(facts.map(fact=>({directory:fact.directory,declared:Number(fact.declared)}))).toEqual(oracle);expect(facts).toEqual(members.map(directory=>({directory,declared:true})));expect(state.workspace.manifestOwners).toHaveLength(1);expect(state.workspace.manifestOwners[0]?.pages.join("")).toBe(body);expect(state.workspace.partialRecords.includes(facts)).toBe(true);expect(state.progress.length).toBeGreaterThan(0);console.log("[DEBUG] "+JSON.stringify({definingCaller:{facts,oracle,manifests:state.workspace.manifestOwners.length,units:state.workspace.completed,ownedBytes:state.workspace.ownedBytes}}));}
 finally{database.close();}
});

test("defining caller refuses absent control credit before physical taxonomy reads",async()=>{const state=operation();await expect(admitTaxonomyCargoMembershipFacts({repoRoot:join(output,"absent-taxonomy")},{...state.control,maximumUnits:0})).rejects.toThrow("Cargo operation budget refused");expect(state.workspace.partialRecords).toEqual([]);});

test("actual async taxonomy workflow requires the active owning command without a fallback",async()=>{await expect(runTaxonomyCliWorkflow(join(output,"absent-taxonomy"),["inventory"])).rejects.toThrow("Cargo command requires its explicit admitted operation scope");});
