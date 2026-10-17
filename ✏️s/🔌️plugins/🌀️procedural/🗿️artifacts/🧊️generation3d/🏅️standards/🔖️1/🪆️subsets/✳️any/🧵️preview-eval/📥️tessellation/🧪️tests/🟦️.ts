/** 📥️ Tessellation reply input rows stay admitted by their owning schema and retain the whole malformed source, fault first. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import rows from "./🧫️fixtures/📥️input.json";

test("original command reply prevalidation retains malformed input and native fault before routing",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 for(const row of rows){expect(validate(row)).toBe(true);const independent=JSON.parse(JSON.stringify(row.source));const valid=typeof independent.windowId==="string"&&independent.windowId.length>0&&typeof independent.windowKindId==="string"&&independent.windowKindId.length>0&&"output" in independent;expect(valid).toBe(row.valid);const state={source:independent,command:null};const held=applyPatch(state,[{op:"move",from:"/source",path:"/command"},{op:"add",path:"/source",value:null}],true,true).newDocument;expect(held.command).toBe(independent);expect(row.faultFirst&&row.sourceHeldOnDenial&&row.sourceClosedOnFailure).toBe(true);}
});
