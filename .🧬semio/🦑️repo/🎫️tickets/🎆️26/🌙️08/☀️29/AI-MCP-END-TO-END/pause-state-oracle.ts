import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const root=import.meta.dirname,value=JSON.parse(readFileSync(`${root}/pause-state-case.json`,"utf8"));
const valid=new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(`${root}/pause-state-case.schema.json`,"utf8")));if(!valid(value))throw Error(JSON.stringify(valid.errors));
const db=new Database(":memory:");const row=db.query("select ? as maximum,? as owned,? as completed,? as total,? as stage,? as depth,? as started,? as receiving,? as sourceDepth,? as allocationLocal,? as retirement,? as allocation,? as work,? as refusal").get(value.maximum,value.owned,value.completed,value.total,value.stage,value.depth,Number(value.started),Number(value.receiving),value.sourceDepth,Number(value.allocationLocal),Number(value.retirement),Number(value.allocation),Number(value.work),value.refusal) as Record<string,unknown>;for(const [key,item] of Object.entries(value)){const expected=typeof item==="boolean"?Number(item):item;if(row[key]!==expected)throw Error(key)}console.log("SQLite pause-state custody oracle passed");
