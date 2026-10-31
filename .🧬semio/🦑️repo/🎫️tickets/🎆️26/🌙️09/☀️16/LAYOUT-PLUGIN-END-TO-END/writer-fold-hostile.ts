/** 🧨️ Reject corrupted Writer tapes only after proving the actual native baseline. */
import {readFile} from "node:fs/promises";
import assert from "node:assert/strict";
import {compareWriterNativeRecords} from "./writer-fold-oracle.ts";
const path=process.argv[2];assert(path,"actual native log required");
const text=await readFile(path,"utf8"),native=compareWriterNativeRecords(text);
const marker="[writer-native-fold] ",rows=text.split("\n").filter(line=>line.startsWith(marker)).map(line=>JSON.parse(line.slice(marker.length)));
assert(native>1);
const mutations=[
 ["duplicate-replaces-omitted-case",[...rows.slice(0,-1),rows[0]]],
 ["extra-known-duplicate",[...rows,rows[0]]],
 ["omitted-case",rows.slice(1)],
 ["unknown-case",[{...rows[0],name:"actual-baseline-hostile-unknown"},...rows.slice(1)]],
 ["changed-actual-output",[{...rows[0],result:{kind:"actual-baseline-hostile-output"}},...rows.slice(1)]]
] as const;
for(const [name,changed] of mutations){assert.throws(()=>compareWriterNativeRecords(changed.map(row=>marker+JSON.stringify(row)).join("\n")),name);}
console.log(JSON.stringify({actualNativeRecords:native,nativeCompared:true,hostileRejected:mutations.map(([name])=>name)}));
