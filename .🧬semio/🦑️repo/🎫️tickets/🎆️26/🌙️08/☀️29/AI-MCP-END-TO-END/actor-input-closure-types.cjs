const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';
const before=fs.readFileSync(file,'utf8');let after=before;
for(const [old,next] of [
 ['original_job_clock,|cx|ORIGINAL_OWNED_INPUT.with(|slot|{','original_job_clock,|cx| -> Result<(bool,bool),OriginalTurnFailure> {ORIGINAL_OWNED_INPUT.with(|slot|{'],
 ['\n })))?;spent=call.receipt_spent;work=call.work_units_spent;','\n })}))?;spent=call.receipt_spent;work=call.work_units_spent;']
]){if(after.split(old).length!==2)throw Error('input closure marker changed');after=after.replace(old,next)}
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/actor-input-closure-types.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared exports changed');fs.renameSync(temp,file);
