const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';const before=fs.readFileSync(file,'utf8');let after=before;
for(const [old,next] of [
 ['let mut ready=false;let mut returned=false;','let mut ready=false;let mut returned=false;let mut failed=false;'],
 ['|cx| -> Result<(bool,bool),OriginalTurnFailure> {ORIGINAL_OWNED_INPUT.with','|cx| -> Result<(bool,bool,bool),OriginalTurnFailure> {ORIGINAL_OWNED_INPUT.with'],
 ['if budget.cancelled{return Ok((false,true))}','if budget.cancelled{return Ok((false,true,false))}'],
 ['return Ok((false,false))}cx.consume_retained','return Ok((false,false,false))}cx.consume_retained'],
 ['return Ok((false,false))}let state=owner.advance(Some(wire),cx)?;Ok((matches!(state,crate::original_input::InputAdmission::Ready),owner.input_returned()))','return Ok((false,false,false))}let state=owner.advance(Some(wire),cx)?;Ok((matches!(state,crate::original_input::InputAdmission::Ready),owner.input_returned(),matches!(state,crate::original_input::InputAdmission::Failed)))'],
 ['Ok((r,h))=>{ready=r;returned=h;Ok(None)}','Ok((r,h,f))=>{ready=r;returned=h;failed=f;Ok(None)}'],
 ['.or_else(||budget.cancelled.then_some(ValueRefusalKind::Canceled));','.or_else(||if failed{ORIGINAL_OWNED_INPUT.with(|slot|slot.borrow().as_ref().and_then(|owner|owner.failure_kind()))}else{None}).or_else(||budget.cancelled.then_some(ValueRefusalKind::Canceled));']
]){if(after.split(old).length!==2)throw Error(`expected one marker ${old}`);after=after.replace(old,next)}
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/actor-input-failure-kind.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared exports changed');fs.renameSync(temp,file);
