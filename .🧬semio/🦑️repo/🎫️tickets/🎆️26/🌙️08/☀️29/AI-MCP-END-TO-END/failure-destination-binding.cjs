const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs';const before=fs.readFileSync(file,'utf8');let after=before;
for(const [old,next] of [
 ["clock:fn()->Option<u64>,slot:&mut Option<JobOutcomeDescriptor>)->OriginalActorCall<bool,OriginalActorEntryFailure>","clock:fn()->Option<u64>,binding:semio_framework_job::OriginalSourceBinding,slot:&mut Option<JobOutcomeDescriptor>)->OriginalActorCall<bool,OriginalActorEntryFailure>"],
 ["|cx|->Result<bool,OriginalTurnFailure>{let loan=if let Some(original)=value_fault.as_ref()","|cx|->Result<bool,OriginalTurnFailure>{if !binding.matches(cx){return Err(refusal(\"failure descriptor belongs to another original Source\").into())}let loan=if let Some(original)=value_fault.as_ref()"]
]){if(after.split(old).length!==2)throw Error('failure destination binding marker changed');after=after.replace(old,next)}
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/failure-destination-binding.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared entry changed');fs.renameSync(temp,file);
