import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./🧬️publication-receipt.json" with {type:"json"};
import cases from "./🔣️publication-receipt.json" with {type:"json"};
if(!new Ajv().validate(schema,cases))throw Error("invalid publication receipt corpus");
const db=new Database(":memory:");
const oracle=db.query("select ?1 + ?2 as historical, ?3 as published, ?1 + ?2 + ?3 as total, ?4 as refusal");
for(const row of cases){const expected=oracle.get(row.producer,row.capture,row.move,Number(row.refused)) as {historical:number;published:number;total:number;refusal:number};const historical=row.producer+row.capture,current=row.move,outer=0;if(historical!==expected.historical||current!==expected.published||historical+current+outer!==expected.total||Number(row.refused)!==expected.refusal)throw Error("publication receipt oracle disagreed");}
console.log(`publication receipt oracle: ${cases.length} cases`);
