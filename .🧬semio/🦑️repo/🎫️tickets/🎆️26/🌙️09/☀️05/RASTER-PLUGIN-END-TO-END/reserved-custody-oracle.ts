import {readFileSync} from "node:fs";
import {Buffer} from "node:buffer";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const files=[
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✂️clipboard-job/🧪️custody/fixture.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/📋️clipboard/🧪️custody/fixture.json",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️media-export-job/🧪️custody/fixture.json",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📋️clipboard/🧵️job/🧪️custody/fixture.json"
];
const validate=new Ajv().compile({type:"object",required:["schema","domain","tools","denialPreservesRequest","denialPreservesInputAddress","requestRemainsUntilPrivateClose","borrows","mediaCloneCount","freshDecoderCount","closeUsesActualChildWork","unknownPreservesRequest"],properties:{schema:{const:"reserved.original-request-custody.v1"},domain:{enum:["puzzle3d","puzzle5d","cad","drawing"]},tools:{type:"array",items:{type:"string"},minItems:1},denialPreservesRequest:{const:true},denialPreservesInputAddress:{const:true},requestRemainsUntilPrivateClose:{const:true},mediaCloneCount:{const:0},freshDecoderCount:{const:0},closeUsesActualChildWork:{const:true},unknownPreservesRequest:{const:true}}});let cases=0;
for(const file of files){const value=JSON.parse(readFileSync(file,"utf8"));if(!validate(value))throw Error(JSON.stringify(validate.errors));cases++;if(value.selectionCases){for(const row of value.selectionCases){const selected=row.domains.vortex?.ids.length?row.domains.vortex.ids:Object.keys(row.domains).sort().flatMap(key=>row.domains[key].ids);const granularity=row.domains.vortex?.granularity||(selected.length?"object":"");if(!new Ajv().compile({const:row.expected})({selected,granularity}))throw Error("selection reference differs from neutral contract");cases++;}}}
const close=JSON.parse(readFileSync("✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🧪️tests/🧫️fixtures/♻️retirement/🔣️.json","utf8")),buffer=Buffer.alloc(close.payloadBytes,19);if(buffer.length!==close.failedReleaseBytes)throw Error("native release oracle differs");const db=new Database(":memory:");try{const value=db.query("SELECT CASE WHEN ?=0 THEN 1 ELSE 0 END AS retained,? AS work").get(close.deniedCapacity,close.childWork)as{retained:number;work:number};if(value.retained!==Number(close.retainsValueOnBirthDenial)||value.work!==close.childWork)throw Error("original close refusal/work fixture differs");}finally{db.close();}cases+=2;
console.log(JSON.stringify({schema:"reserved.original-request-custody.v1",cases,oracle:"Ajv + independent ordered selection + Buffer/SQLite",status:"passed",nativeLawsExecuted:false}));
