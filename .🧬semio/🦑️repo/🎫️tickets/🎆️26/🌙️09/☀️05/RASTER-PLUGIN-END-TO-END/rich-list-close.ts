import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📋️list/🦀️.rs';let body=readFileSync(path,'utf8');
const start=body.indexOf(' fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{');const end=body.indexOf('\n fn terminal_is_empty',start);if(start<0||end<0)throw new Error('list close missing');let part=body.slice(start,end);
part=part.replace('->RetainedWorkOutcome{let mut work=0;let result=(||{','->crate::OriginalRetirementOutcome{let mut work=0;let result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>=(||{');
part=part.replace('turn.result.map(|step|RetainedCloneStep::Progress(step.progress()))','turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into)');
part=part.replaceAll('turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))','turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into)');
part=part.replace('turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default())))','turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)');
part=part.replace('work=turn.work;turn.result})();','work=turn.work;turn.result.map_err(Into::into)})();');
body=body.slice(0,start)+part+body.slice(end);writeFileSync(path,body);
