import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🦀️.rs';let text=readFileSync(file,'utf8');
const changes:[string,string][]=[
 ['else{let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("sealed birth absent"))?;','else{let mut work=0;let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("sealed birth absent"))?;'],
 ['let (value,mut progress)=turn.result?;*output=Some(value);','work=turn.work;let (value,mut progress)=turn.result?;*output=Some(value);'],
 ['slot.take();Ok(crate::RetainedCloneStep::Complete(progress))})();crate::RetainedWorkOutcome{result,work:1}}})','slot.take();Ok(crate::RetainedCloneStep::Complete(progress))})();crate::RetainedWorkOutcome{result,work}}})'],
 ['else{let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("Arc birth absent"))?;','else{let mut work=0;let result=(||{let birth=slot.as_mut().ok_or_else(||ValueError::invariant("Arc birth absent"))?;'],
 ['target.write(input.take().ok_or_else(||ValueError::invariant("original Arc input absent"))?);','target.write(input.take().ok_or_else(||ValueError::invariant("original Arc input absent"))?);work=1;'],
 ['copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:1}}})','copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work}}})'],
];for(const [before,after] of changes){if(!text.includes(before))throw new Error(`publication changed: ${before}`);text=text.replace(before,after)}writeFileSync(file,text);
