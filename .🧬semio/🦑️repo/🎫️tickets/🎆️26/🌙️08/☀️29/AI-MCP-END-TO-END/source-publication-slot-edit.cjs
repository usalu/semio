const fs=require('fs'),path='🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs',before=fs.readFileSync(path,'utf8');let after=before;
const edits=[
 ['use semio_framework_job_return_escrow_publication::OriginalEscrowPublisher;','use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublication};'],
 ['publisher:Option<Box<OriginalEscrowPublisher>>,publisher_placed:','publisher:Option<Box<OriginalEscrowPublisher>>,publication:Option<OriginalEscrowPublication>,publisher_placed:'],
 ['table:None,publisher:None,publisher_placed:false','table:None,publisher:None,publication:None,publisher_placed:false'],
 ['budget.publisher.is_none()&&!budget.publisher_placed','budget.publisher.is_none()&&budget.publication.is_none()&&!budget.publisher_placed'],
 ['|budget|budget.claims!=0||budget.held_items!=0','|budget|budget.publication.is_some()||budget.claims!=0||budget.held_items!=0'],
 ['let budget=self.budget.get_mut();if let Some(publisher)=budget.publisher.as_ref(){','let budget=self.budget.get_mut();if budget.publication.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}if let Some(publisher)=budget.publisher.as_ref(){']
];for(const [old,value] of edits){const expected=old.startsWith('budget.publisher.is_none()')?3:2;if(after.split(old).length!==expected)throw Error('unexpected Source publication slot '+old);after=after.split(old).join(value)}const tmp=path+'.guest-publication-slot';fs.writeFileSync(tmp,after,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before){fs.unlinkSync(tmp);throw Error('concurrent source change')}fs.renameSync(tmp,path);
