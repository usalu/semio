import {expect,test} from "bun:test";
import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {dirname,join,resolve,relative} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import schema from "../../🧬️schema/📦️artifacts/🔣️.json";
import corpus from "../../🧫️fixtures/📦️artifacts/🔣️.json";
import {discoverMutationInventoryProviders,selectMutationInventoryProvider} from "../../🟦️.ts";
import {CargoDiscoveryWorkspace,type CargoDiscoveryOperation} from "../../../../../📚️library/🗂️workspaces/🦀️cargo/📁️physical/🟦️.ts";

type Pair=Readonly<{path:string;after:string|null}>;
type Selection=Readonly<{dimension:string;owner:string;crate:string;provider:Readonly<{script:string;roots:readonly string[]}>}>;
type Authority=Readonly<{pairs:readonly Pair[];selected:readonly Selection[];workspace:Readonly<{path:string;source:string}>}>;
const input=process.env.SEMIO_PROVIDER_ARTIFACT_INPUT,output=process.env.SEMIO_PROVIDER_ARTIFACT_OUTPUT;
if(!input||!output)throw Error("Required artifact authority and fixture owner missing");
const authority:Authority=JSON.parse(readFileSync(input,"utf8"));

/** 📇️ Reads an oracle field only after its own table admission. */
function field(value:unknown,key:string):unknown{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Oracle table missing");return Object.getOwnPropertyDescriptor(value,key)?.value;}

/** 📦️ Selects the retained exact manifest by its independent package identity. */
function manifestFor(selected:Selection):Pair&{after:string}{const pair=authority.pairs.find(pair=>pair.path.endsWith("Cargo.toml")&&pair.after!==null&&field(field(parse(pair.after),"package"),"name")===selected.crate);if(!pair||pair.after===null)throw Error("Exact declaring manifest missing");return{...pair,after:pair.after};}

/** 🎛️ Authors finite fixture discovery control with retained progress. */
function operation(){const workspace=new CargoDiscoveryWorkspace(),events:unknown[]=[],control:CargoDiscoveryOperation={workspace,signal:new AbortController().signal,maximumUnits:16777216,maximumOwnedBytes:134217728,maximumDepth:64,onProgress:event=>{events.push(event);},yieldContinuation:async()=>{}};return{workspace,events,control};}

test("exact three proposed artifact declarations and removed-first survivors select their own producers",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);expect(authority.selected.map(row=>row.dimension)).toEqual(["2d","3d","5d"]);
 const db=new Database(":memory:"),observations:unknown[]=[];
 try{for(const row of corpus.cases){const root=join(output,row.id);mkdirSync(root,{recursive:true});const rootManifest='[package]\nname="semio-provider-selection-fixture"\nversion="0.1.0"\nedition="2021"\n[workspace]\nmembers=["."]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=["✏️s/Cargo.toml"]\nmember-manifests=["Cargo.toml"]\nexclude-patterns=[]\n';writeFileSync(join(root,"Cargo.toml"),rootManifest);mkdirSync(dirname(join(root,authority.workspace.path)),{recursive:true});writeFileSync(join(root,authority.workspace.path),authority.workspace.source);
  const retained=authority.selected.filter(selected=>!row.removed.includes(selected.dimension));for(const selected of retained){const manifest=manifestFor(selected),metadata=field(field(field(field(parse(manifest.after),"package"),"metadata"),"semio"),"mutation-inventory");expect(metadata).toEqual(selected.provider);const script=resolve(dirname(manifest.path),selected.provider.script),producer=authority.pairs.find(pair=>resolve(pair.path)===script);if(!producer||producer.after===null)throw Error("Exact artifact producer body missing");for(const pair of [manifest,producer]){if(pair.after===null)throw Error("Retained endpoint body missing");mkdirSync(dirname(join(root,pair.path)),{recursive:true});writeFileSync(join(root,pair.path),pair.after);expect(readFileSync(join(root,pair.path),"utf8")).toBe(pair.after);}}
  const state=operation(),providers=await discoverMutationInventoryProviders(root,state.control);expect(providers.length).toBe(retained.length);const outcomes:unknown[]=[];for(const selected of authority.selected){const owner=join(root,selected.owner),oracle=db.query("select json_extract(value,'$.script') as script from json_each(?) p where exists(select 1 from json_each(json_extract(p.value,'$.roots')) r where ?=r.value or substr(?,1,length(r.value)+1)=r.value||'/')").all(JSON.stringify(providers),owner,owner),actual=await selectMutationInventoryProvider(providers,owner,state.control),expected=retained.includes(selected)?resolve(root,dirname(manifestFor(selected).path),selected.provider.script):null;expect(oracle.length).toBe(expected===null?0:1);expect(actual?.script??null).toBe(expected);if(expected!==null){expect(oracle).toEqual([{script:expected}]);expect(existsSync(expected)).toBe(true);}outcomes.push({dimension:selected.dimension,expected,actual:actual?.script??null,oracle});}
  expect(state.events.length).toBeGreaterThan(0);expect(state.workspace.manifestOwners.every(owner=>owner.state==="complete")).toBe(true);observations.push({id:row.id,root,providers,outcomes,progress:state.events.length,units:state.workspace.completed,ownedBytes:state.workspace.ownedBytes,manifests:state.workspace.manifestOwners.map(owner=>({path:owner.path,pages:owner.pages,state:owner.state}))});console.log("[DEBUG] "+JSON.stringify({case:row.id,providers:providers.length,selected:outcomes,units:state.workspace.completed,ownedBytes:state.workspace.ownedBytes}));
 }}finally{db.close();writeFileSync(join(output,"observation.json"),JSON.stringify({observations,sourceWrites:0,productionNativeReady:false,declaringReady:false}));}
});
