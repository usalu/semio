#!/usr/bin/env python3
"""Fail-closed patch: drive_worker_step ignored a checked-out worker CONTINUATION (a step that returned Err is carried as OriginalJobReturn::Continuation, so outcome()==Ok(None)). It took the 'no outcome -> resume' path, resume() was refused forever (is_resumable() is false while payload.continuation() is Some), and the op parked in AwaitingInput with zero progress -> fixture WorkLimit / host action never settles. Fix: surface the continuation's cause as the operation's terminal fault, mark it terminal so the resume branch begins the paid close."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs"
s=open(p).read()
a="""                Ok(None)=>{self.worker_semantic_pending=false;self.worker_resume_pending=true;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));},"""
b="""                Ok(None)=>{
                    if let Some(view)=session.checked_out_continuation(){
                        let cause=view.callback.or(view.qualification).or(view.receipt_qualification).map_or_else(||"typed operation worker step returned a continuation without a cause".to_owned(),|failure|format!("{failure:?}"));
                        self.terminal_fault.get_or_insert_with(||ArtifactBoundedToolFault::from_fault(&Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.worker-continuation"),cause)));
                        self.terminal_seen=true;
                        if let Some(lease)=self.cancellation_lease.as_ref(){lease.cancel();}
                    }
                    self.worker_semantic_pending=false;self.worker_resume_pending=true;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
                },"""
assert s.count(a)==1
open(p,"w").write(s.replace(a,b));print("applied")
