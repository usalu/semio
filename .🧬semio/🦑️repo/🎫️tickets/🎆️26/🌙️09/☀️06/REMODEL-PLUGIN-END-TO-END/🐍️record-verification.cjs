const fs=require("node:fs"),path=require("node:path");
const bytes=fs.readFileSync(0);if(bytes.length>33554432)throw Error("Bounded verification record required");
const records=JSON.parse(bytes.toString("utf8"));if(!Array.isArray(records)||records.length>8)throw Error("Finite verification record batch required");
const root=path.resolve(__dirname,"🗑️generated");fs.mkdirSync(root,{recursive:true});
for(const record of records){if(!Number.isSafeInteger(record.session)||record.session<1||typeof record.result?.output!=="string")throw Error("Actual tool result required");const target=path.resolve(root,"verification-"+record.session+".jsonl");if(path.dirname(target)!==root||target.length>256)throw Error("Ticket output path required");fs.appendFileSync(target,JSON.stringify(record)+"\n");}
console.log(JSON.stringify({records:records.length,bytes:bytes.length}));
