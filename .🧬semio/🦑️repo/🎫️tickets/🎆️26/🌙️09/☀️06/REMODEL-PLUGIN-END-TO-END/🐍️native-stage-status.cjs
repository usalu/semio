const fs=require("node:fs"),path=require("node:path");
const requested=process.argv.slice(2),ids=new Set((requested.length?requested:[41432,53092,40120,36356,24588,43512]).map(value=>{const id=Number(value);if(!Number.isSafeInteger(id)||id<=0)throw Error("Positive process identity required");return id;}));
if(ids.size>8)throw Error("At most eight observed process identities required");
const root=path.resolve(".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/timing"),rows=[];
for(const name of["cargo-phases.jsonl.1","cargo-phases.jsonl"]){const file=path.join(root,name);if(!fs.existsSync(file))continue;const size=fs.statSync(file).size;if(size>16777216)throw Error("Observed phase ledger exceeds read bound");for(const line of fs.readFileSync(file,"utf8").split(/\r?\n/)){if(!line.trim())continue;let row;try{row=JSON.parse(line);}catch{continue;}if(row.version===1&&ids.has(row.pid))rows.push(row);}}
console.log(JSON.stringify({atMs:Date.now(),ledgerRoot:root,processes:[...ids].map(pid=>({pid,marks:rows.filter(row=>row.pid===pid).sort((a,b)=>a.atMs-b.atMs)}))},null,2));
