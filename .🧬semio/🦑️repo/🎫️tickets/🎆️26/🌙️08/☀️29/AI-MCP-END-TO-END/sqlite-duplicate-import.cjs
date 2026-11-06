const fs=require('node:fs');const file='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🦀️.rs';
const before=fs.readFileSync(file,'utf8'),old='use semio_framework_value::retirement::RetireOwned;\n';
if(before.split(old).length!==2||!before.includes('use semio_framework_value::{RetireOwned,ValueError,ValueRefusalKind};'))throw Error('duplicate import changed');
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/sqlite-duplicate-import.tmp';fs.writeFileSync(temp,before.replace(old,''));if(fs.readFileSync(file,'utf8')!==before)throw Error('shared source changed');fs.renameSync(temp,file);
