const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';const before=fs.readFileSync(file,'utf8');
const marker=' if ORIGINAL_OWNED_INPUT.with(|slot|slot.borrow().as_ref().is_some_and(|owner|!owner.owns_wire(wire)))';
if(before.split(marker).length!==2)throw Error('wire fence changed');
const fence=' if ORIGINAL_OWNED_INPUT.with(|slot|slot.borrow().as_ref().is_some_and(|owner|owner.input_returned())){return return_fixed(OriginalActorFixedCall{input,spent:Default::default(),work_units_spent:0,kind:OriginalActorFixedKind::InputAdmission{ready:false,input_returned:true,refusal:Some(ValueRefusalKind::InvariantViolated)}})}\n';
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/actor-input-returned-fence.tmp';fs.writeFileSync(temp,before.replace(marker,fence+marker));if(fs.readFileSync(file,'utf8')!==before)throw Error('shared exports changed');fs.renameSync(temp,file);
