const fs=require('fs');
const manifest='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml',source='🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs';
const changes=[
 [manifest,'semio-framework-job-payload = { workspace = true }','semio-framework-job-payload = { workspace = true }\nsemio-framework-job-return-escrow-publication = { workspace = true }'],
 [source,'use semio_framework_job_payload::{','use semio_framework_job_return_escrow_publication::OriginalEscrowPublisher;\nuse semio_framework_job_payload::{'],
 [source,'table:Option<Box<OriginalReturnEscrowTable>>,origin:','table:Option<Box<OriginalReturnEscrowTable>>,publisher:Option<Box<OriginalEscrowPublisher>>,origin:'],
 [source,'table:None,origin:None,spent:','table:None,publisher:None,origin:None,spent:'],
 [source,'budget.table.is_none()&&budget.held_items==0','budget.table.is_none()&&budget.publisher.is_none()&&budget.held_items==0'],
 [source,'let budget=self.budget.get_mut();if let Some(table)=budget.table.as_ref(){','let budget=self.budget.get_mut();if let Some(publisher)=budget.publisher.as_ref(){let(copy,release,work)=OriginalEscrowPublisher::retirement_demand();if !publisher.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}drop(budget.publisher.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})),work}}if let Some(table)=budget.table.as_ref(){']
];
const before=new Map(),after=new Map();for(const [path,old,value] of changes){if(!before.has(path))before.set(path,fs.readFileSync(path,'utf8'));let valueBefore=after.get(path)??before.get(path);const expected=old.startsWith('budget.table.is_none()')?3:2;if(valueBefore.split(old).length!==expected)throw Error('unexpected source match '+path+' '+old);after.set(path,valueBefore.split(old).join(value))}
for(const [path,value] of after){const tmp=path+'.guest-publisher';fs.writeFileSync(tmp,value,{flag:'wx'});if(fs.readFileSync(path,'utf8')!==before.get(path)){fs.unlinkSync(tmp);throw Error('concurrent source change: '+path)}fs.renameSync(tmp,path)}
