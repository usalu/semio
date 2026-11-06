const fs=require('fs');function edit(f,changes){const before=fs.readFileSync(f,'utf8');let s=before;for(const[a,b]of changes){if(s.split(a).length!==2)throw Error(f+': '+a);s=s.replace(a,b)}if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-failure-retire.tmp';fs.writeFileSync(t,s);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f)}
const failure='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️failure/🦀️.rs';
edit(failure,[
 ['use semio_framework_job_payload::{RetainedPayloadBuilder, JobPayloadStream};','use semio_framework_job_payload::{RetainedPayloadBuilder, JobPayloadStream};\nuse semio_framework_job_fault_publication::OriginalFaultPayloadStep;'],
 ['if self.delivered||cx.should_yield()||!Self::permits(cx.retained_grant(),demand)','if self.delivered||(if cx.original_retirement_active(){cx.retirement_should_yield()}else{cx.should_yield()})||!Self::permits(cx.retained_grant(),demand)'],
 ['let turn=self.builder.advance_initialization(cx);','let turn=if cx.original_retirement_active(){self.builder.advance_initialization(&OriginalFaultPayloadStep::borrow(cx)?)}else{self.builder.advance_initialization(cx)};'],
 ['let turn=self.builder.append_original(cx,&self.buffer[..self.length],&mut self.offset);','let turn=if cx.original_retirement_active(){self.builder.append_original(&mut OriginalFaultPayloadStep::borrow(cx)?,&self.buffer[..self.length],&mut self.offset)}else{self.builder.append_original(cx,&self.buffer[..self.length],&mut self.offset)};'],
 ['let turn=self.builder.seal(cx);','let turn=if cx.original_retirement_active(){self.builder.seal(&OriginalFaultPayloadStep::borrow(cx)?)}else{self.builder.seal(cx)};'],
 ['let outcome=JobOutcomeBorrow::admit_fault_with_cursor(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?;','let outcome=if cx.original_retirement_active(){JobOutcomeBorrow::admit_fault_with_cursor_retiring(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?}else{JobOutcomeBorrow::admit_fault_with_cursor(cx,self.builder.published().ok_or_else(Self::invalid)?,cursor)?};']
]);
const call='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs';
edit(call,[
 ['|cx|->Result<bool,OriginalTurnFailure>{if !binding.matches(cx){return Err(refusal("failure descriptor belongs to another original Source").into())}let loan=','|cx|->Result<bool,OriginalTurnFailure>{cx.with_original_retirement(|cx|{if !binding.matches(cx){return Err(refusal("failure descriptor belongs to another original Source").into())}let loan='],
 ['if let Some(loan)=loan{*slot=Some(loan.into_descriptor());Ok(true)}else{Ok(false)}});','if let Some(loan)=loan{*slot=Some(loan.into_descriptor());Ok(true)}else{Ok(false)}})});'],
 ['let original=context.dispatch_step(input,budget,recipient,clock,|cx|{let view=if value_fault.is_some(){value_publication.borrow_outcome(descriptor)?}else{publication.borrow_outcome(descriptor)?};produce(view,cx)});','let original=context.dispatch_step(input,budget,recipient,clock,|cx|cx.with_original_retirement(|cx|{let view=if value_fault.is_some(){value_publication.borrow_outcome(descriptor)?}else{publication.borrow_outcome(descriptor)?};produce(view,cx)}));']
]);
const exportFile='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';
edit(exportFile,[['frame.prepare(job,cx)}));return OriginalOwnedJobCall','cx.with_original_retirement(|cx|frame.prepare(job,cx))}));return OriginalOwnedJobCall']]);
