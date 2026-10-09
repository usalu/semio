import {test,expect} from "bun:test";import {readFileSync} from "node:fs";import {resolve} from "node:path";import Ajv from "ajv";
const root=resolve(import.meta.dir,"../../../../../../../..");const board="🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board";const flow="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
test("declared computing facts match independent strict admission",()=>{const base=resolve(root,board,"🧬️schema/🎯️dag-input/📊️progress");const f=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:false}).compile(JSON.parse(readFileSync(resolve(base,"🔣️.json"),"utf8")));for(const row of f.accepted)expect(validate(row)).toBe(true);for(const row of f.refused)expect(validate(row)).toBe(false);console.log("[DEBUG] computing progress independent Ajv accepted2/refused8");});
test("original Flow receives complete retained DAG operation authority",()=>{const source=readFileSync(resolve(root,flow,"🕸️wasm/🦀️.rs"),"utf8"),retained=readFileSync(resolve(root,flow,"🕸️wasm/🧵️dag-input/🦀️.rs"),"utf8");expect(source.includes("fn dag_input_decode<T>(budget")).toBe(false);expect(source.includes("fn dag_input_encode<T>(budget")).toBe(false);for(const operation of [2514,2515,2520,2522,2523,2525,2528])expect(source.includes(`${Math.floor(operation/1000)}_${String(operation%1000).padStart(3,"0")} => Some(Box::new(retained_dag_input::FlowDagAction::new(operation,arguments)))`)).toBe(true);expect(retained).toContain("DagInputApplication::new");expect(retained.includes("serde_json::from_str")).toBe(false);expect(retained.includes("filter_map")).toBe(false);expect(retained).toContain("NativeDecodeControl::resume");expect(retained).toContain("NativeEncodeControl::resume");expect(retained.includes("new(budget.byte_credit")).toBe(false);});

test("canonical JSON writer supports a retained borrowed source",()=>{const source=readFileSync(resolve(root,"🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs"),"utf8");expect(source.includes("pub struct JsonBorrowedWriteCursor")).toBe(true);});

import {decodeDagComputingProgressJson} from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
test("actual computing-progress controlled IO agrees with independent Ajv and preserves refusals",async()=>{
 const base=resolve(root,board,"🧬️schema/🎯️dag-input/📊️progress");const f=JSON.parse(readFileSync(resolve(base,"🧫️fixtures/🔣️.json"),"utf8"));
 const control=(observe:()=>boolean)=>({syntax:{maximumBytes:1048576,maximumNodes:4096,maximumDepth:128,chunk:1,cancelled:()=>false,progress:()=>{},yield:async()=>{}},ownership:new NativeDecodeControl(1048576,observe)});
 let accepted=0,refused=0;
 for(const row of f.accepted){const source=JSON.stringify(row);expect(await decodeDagComputingProgressJson(source,control(()=>true))).toEqual(JSON.parse(source));accepted++;}
 for(const row of f.refused){const source=JSON.stringify(row);await expect(decodeDagComputingProgressJson(source,control(()=>true))).rejects.toThrow();expect(source).toBe(JSON.stringify(row));refused++;}
 for(const text of f.duplicate)await expect(decodeDagComputingProgressJson(text,control(()=>true))).rejects.toThrow();
 let seen=0;const source=JSON.stringify(f.accepted[1]);await decodeDagComputingProgressJson(source,control(()=>{seen++;return true}));
 for(let stop=1;stop<=seen;stop++){let current=0;await expect(decodeDagComputingProgressJson(source,control(()=>++current<stop))).rejects.toThrow();}
 console.log(`[DEBUG] computing-progress actualTS accepted=${accepted} refused=${refused} duplicates=${f.duplicate.length} cancellationBoundaries=${seen} independentAjv=true originalsource=true`);
});

test("typed DAG construction progresses one admitted member without whole collection traversal",()=>{const source=readFileSync(resolve(root,board,"🚪️io/📝️text/🎯️dag-input/🧵️retained/🦀️.rs"),"utf8");expect(source.includes("super::admit_dag_node_statuses_value")).toBe(false);expect(source.includes("admit_ordered_entry")).toBe(false);expect(source.includes("advance_insert_controlled")).toBe(true);expect(source.includes("fn project_one")).toBe(true);});

test("retained feature refusals and revoked source leases cannot resume",()=>{const source=readFileSync(resolve(root,flow,"🕸️wasm/🦀️.rs"),"utf8");expect(source).toContain("terminal_failure:Option<AbiErrorCode>");expect(source).toContain("lease.active.get()==false");expect(source).toContain("self.terminal_failure=Some(failure.code)");});

test("admitted native output transfers its original allocation without a second encoder",()=>{const source=readFileSync(resolve(root,flow,"🕸️wasm/🧵️dag-input/🦀️.rs"),"utf8");expect(source.includes("finish_domain(Ok(output.into_bytes()))")).toBe(false);expect(source).toContain("self.program.output=output.into_bytes()");});

test("original physical argument backing stays borrowed and retires under its admitted owner",()=>{const source=readFileSync(resolve(root,flow,"🕸️wasm/🦀️.rs"),"utf8"),retained=readFileSync(resolve(root,flow,"🕸️wasm/🧵️dag-input/🦀️.rs"),"utf8");expect(source.includes("decoded: [Vec<u8>; 8]")).toBe(false);expect(retained).toContain("std::mem::take(&mut arguments.payload)");expect(retained).toContain("arguments.payload=payload.unwrap_or_default()");});
