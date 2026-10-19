import {readFileSync,readdirSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {describe,it,expect} from "vitest";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json";
import {applyPatch} from "fast-json-patch";

const owner=resolve(dirname(fileURLToPath(import.meta.url)),"../..");
const read=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
const kebab=(value:string)=>value.replace(/[A-Z]/g,letter=>`-${letter.toLowerCase()}`);
function schemas(directory:string,found=new Map<string,unknown>()){
  for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isDirectory()){schemas(path,found);continue;}if(entry.name!=="🔣️.json"||!path.includes("🧬️schema"))continue;const schema=read(path);if(schema.$id&&!found.has(schema.$id))found.set(schema.$id,schema);}
  return found;
}

describe("FEM exact mutation grammar vocabulary",()=>{
  for(const dimension of ["◻️2d","🧊️3d"]){
    it(`${dimension} carries every variant and verifies geometry-selection snapshots with RFC6902`,()=>{
      const subsets=resolve(owner,`🗿️artifacts/${dimension}/🏅️standards/🔖️1/🪆️subsets`);const generator=resolve(subsets,"🌐️any/🏭️generator");const routes=readFileSync(resolve(generator,"📜️script.ts"),"utf8").split("const CARRIER_FIXTURES = {")[1]!.split("} as const;")[0]!;const routeKinds=[...routes.matchAll(/^  "([a-z-]+)":/gm)].map(row=>row[1]!);expect(routeKinds).toHaveLength(39);
      const rust=readFileSync(resolve(generator,"🧩️json/🦀️.rs"),"utf8").split("pub const KINDS:")[1]!.split("];",1)[0]!;const rustKinds=[...rust.matchAll(/"([a-z-]+)"/g)].map(row=>row[1]!);expect(new Set(rustKinds)).toEqual(new Set(routeKinds));
      const three=dimension==="🧊️3d";const entity=three?"solid":"region";const scenarios=[[`${three?"🧊️":"🗺️"}create-${entity}`],[`🚫️delete-${entity}`],[`🔄️replace-${entity}`],["🧭️move-selection"]];
      for(const[scenario]of scenarios){const root=resolve(subsets,`🕸️mesh/🧫️fixtures/${scenario}`);const before=read(resolve(root,"⏮️before.json"));const after=read(resolve(root,"⏭️after.json"));const oracle=read(resolve(root,"🩹️oracle.json"));expect(applyPatch(structuredClone(before),oracle,true,false).newDocument).toEqual(after);expect(after).not.toEqual(before);}
      console.log(`[DEBUG] ${dimension} all39 carrier routes and four authored geometry/selection snapshots equal independent RFC6902 output`);
    });

    it(`${dimension} names all39 schema variants and preserves every selected neutral payload`,()=>{
      const subsets=resolve(owner,`🗿️artifacts/${dimension}/🏅️standards/🔖️1/🪆️subsets`);const text=resolve(subsets,"🌐️any/🚪️io/📝️text/🧬️mutations");const grammar=readFileSync(resolve(text,"📖️mutations.grammar.semio"),"utf8");
      const rules=[...grammar.matchAll(/^([a-z-]+) = "([a-z-]+)" ([a-z-]+)-fields$/gm)];expect(rules).toHaveLength(39);for(const rule of rules){expect(rule[1]).toBe(rule[2]);expect(rule[1]).toBe(rule[3]);}expect(grammar).not.toMatch(/artifact-mark|eng-field|"add-node"|"set-load"|"set-support"|"commit-step"/);
      const ajv=new Ajv({strict:false,allErrors:true,validateFormats:false});ajv.addMetaSchema(draft7);const registry=schemas(resolve(owner,"🗿️artifacts"));for(const schema of registry.values())ajv.addSchema(schema as object);const aggregate=read(resolve(subsets,"🌐️any/🧬️schema/🧬️mutations/🔣️.json"));const validate=ajv.getSchema(aggregate.$id)!;expect(validate).toBeDefined();
      const kinds=aggregate.oneOf.map((reference:{$ref:string})=>kebab((registry.get(reference.$ref)as{properties:{mutation:{const:string}}}).properties.mutation.const));expect(new Set(rules.map(rule=>rule[1]))).toEqual(new Set(kinds));
      const test=resolve(text,"🧪️tests/🔬️mutation-grammar-conformance/🦀️.rs");const cases=[...readFileSync(test,"utf8").matchAll(/\("([^\"]+)",include_str!\("([^\"]+)"\)\)/g)];expect(cases).toHaveLength(39);
      for(const row of cases){const mutation=read(resolve(dirname(test),row[2]!));expect(kebab(mutation.mutation)).toBe(row[1]);const reference=aggregate.oneOf.find((reference:{$ref:string})=>kebab((registry.get(reference.$ref)as{properties:{mutation:{const:string}}}).properties.mutation.const)===row[1]);const leaf=ajv.getSchema(reference.$ref)!;expect(leaf(mutation),`${row[1]} ${row[2]}: ${JSON.stringify(leaf.errors)}`).toBe(true);}
      console.log(`[DEBUG] ${dimension} grammar39 keyword rules and neutral payloads verified against independent Ajv union`);
    });
  }
});
