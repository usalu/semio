import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';
const original=readFileSync(path,'utf8');
const before='let demand=NativeOwnedDecodeControl::shared_birth_demand::<String>();let turn=control.with_construction_recipient(&mut construction_loan,|control|{frame.quote(control,demand)?;frame.effect(control,|actual,grant|{let turn=SealedShared::admit(input.take().unwrap(),SharedIssuer::owned(),grant,actual);let result=match turn.result{Ok((owner,progress))=>{output=Some(owner);Ok(crate::RetainedCloneStep::Complete(progress))},Err((error,original))=>{input=Some(original);Err(error)}};crate::RetainedWorkOutcome{result,work:turn.work}})?;frame.receipt(control)?;frame.take()}).unwrap();';
const after='let turn=control.with_construction_recipient(&mut construction_loan,|control|Ok::<_,ValueError>(control.receive_shared(&mut input,&mut output,SharedIssuer::owned(),&mut frame))).unwrap();';
if(!original.includes(before))throw new Error('original shared caller law changed');
writeFileSync(path,original.replace(before,after));
