import{strict as assert}from"node:assert";
import{Database}from"bun:sqlite";
import{portableMsvcCapacityV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📐️capacity/🟦️.ts";
import schema from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📐️capacity/🧬️schema/🔣️.json";
import cases from"../../../../../../../🧰️framework/🔨️modules/🏃️process/🧰️tools/🛠️native-compiler/🪟️portable/📐️capacity/🧫️fixtures/🔣️.json";

const db=new Database(":memory:"),p=schema.definitions.Policy.const;try{for(const row of cases){const expected=db.query("select case when ?1>0 and (?1 & (?1-1))=0 and ?2+?3+?4+?5+(?6+?7+?8+1)*(?1-1)+?9*?1<=?10 then 1 else 0 end accepted").get(row.allocationBytes,p.outputBytes,p.payloadBytes,p.catalogBytes,p.metadataBytes,p.outputFiles,p.payloadFiles,p.metadataFiles,p.entryAllowance,p.maximumBytes) as{accepted:number};assert.equal(Boolean(expected.accepted),row.accepted);assert.equal(portableMsvcCapacityV1(row.allocationBytes)!==null,row.accepted);}console.log(JSON.stringify({cases:cases.length,oracle:"sqlite",authority:false}));}finally{db.close();}
