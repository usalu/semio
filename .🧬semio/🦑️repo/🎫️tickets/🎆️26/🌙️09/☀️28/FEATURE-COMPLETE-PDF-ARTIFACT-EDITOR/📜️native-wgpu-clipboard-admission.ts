/** ✂️ Joins the native clipboard owner to the worker's retained admission and receipt API. */
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs";
const original = await Bun.file(path).text();
const cfg = '#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]';
function required(body: string, old: string, next: string): string {
    if (body.split(old).length !== 2) throw Error("Native clipboard anchor changed: " + old.slice(0,100));
    return body.replace(old,()=>next);
}
let result = required(original,cfg+"\nstruct MountedClipboardIo {",`${cfg}
type ClipboardJob = semio_ui_wgpu_hosting::host::ClipboardIoJob;
${cfg}
type ClipboardSession = semio_framework_job_worker::WorkerJobSession<ClipboardJob>;

${cfg}
struct MountedClipboardIo {`);
result = required(result,"initializer:semio_ui_wgpu_hosting::host::ClipboardIoJobInitializer,","job:Option<ClipboardJob>,");
result = required(result,"preparation:Option<semio_framework_job_worker::WorkerJobSessionPreparation<semio_ui_wgpu_hosting::host::ClipboardIoJob>>,","admission:semio_framework_job_worker::WorkerJobSessionAdmission<ClipboardJob, ClipboardSession>,");
result = required(result,"session:Option<semio_framework_job_worker::WorkerJobSession<semio_ui_wgpu_hosting::host::ClipboardIoJob>>,","session:Option<ClipboardSession>,");
result = required(result,"pub outcome:Option<semio_framework_job_worker::WorkerJobOutcome<semio_ui_wgpu_hosting::host::ClipboardIoJob>>,","pub outcome:Option<semio_framework_job_worker::WorkerJobOutcome<ClipboardJob>>,");
const admission = result.split("\n").find(line=>line.includes("let initializer=match intent.text"));
if (!admission) throw Error("Native clipboard intent owner changed");
result = required(result,admission,`        let job = match intent.text { Some(text) => ClipboardJob::write(text), None => ClipboardJob::read() };`);
result = required(result,"{invocation,initializer,params:None,parameter_cancel:None,preparation:None,session:None", "{invocation,job:Some(job),params:None,parameter_cancel:None,admission:Default::default(),session:None");
const cleanupStart = result.indexOf("            if let Some(outcome)=owner.outcome.as_mut()");
const cleanupEnd = result.indexOf("            let release=match",cleanupStart);
if(cleanupStart<0||cleanupEnd<=cleanupStart)throw Error("Native clipboard retirement shape changed");
result = required(result,result.slice(cleanupStart,cleanupEnd),`            if let Some(outcome) = owner.outcome.as_mut() {
                if outcome.acknowledge_outcome(cx)? {
                    owner.outcome.take().unwrap().begin_close();
                }
                return Ok(true);
            }
            if let Some(session) = owner.session.as_mut() {
                session.begin_close();
                match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                    Ok(true) => {
                        if let Some((turn, _)) = cx.with_original_effect(cx.retained_grant(), |cx,effect| session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))? {
                            let result = match turn.result {
                                Ok(Some(step)) => Ok(step),
                                Ok(None) | Err(WorkerJobDemandError::Contention(_)) => Ok(RetainedCloneStep::Progress(Default::default())),
                                Err(WorkerJobDemandError::Refused(error)) => Err(error),
                            };
                            receive_original_retained_work(cx,RetainedWorkOutcome { result, work:turn.work })?;
                        }
                        return Ok(true);
                    }
                    Ok(false) => {}
                    Err(WorkerJobDemandError::Contention(_)) => return Ok(true),
                    Err(WorkerJobDemandError::Refused(error)) => return Err(error.into()),
                }
                session.close_step(cx)?;
                if session.terminal_is_empty() { owner.session=None; }
                return Ok(true);
            }
            if !owner.admission.terminal_is_empty() {
                owner.admission.close_step(cx)?;
                return Ok(true);
            }
            if let Some(job) = owner.job.as_mut() {
                job.begin_close();
                if job.terminal_is_empty() {
                    let demand = RetirementDemand { copy_bytes: std::mem::size_of::<Option<ClipboardJob>>(), depth:1, ..Default::default() };
                    close_original_retained_step(cx,demand,|_| {
                        owner.job = None;
                        RetainedWorkOutcome { result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1, copied_bytes:demand.copy_bytes, ..Default::default() })), work:1 }
                    })?;
                } else {
                    job.close_step(cx)?;
                }
                return Ok(true);
            }
            if owner.params.is_some() {
                let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_depth==0 { return Ok(true); }
                let params=owner.params.take().unwrap();
                owner.parameter_cancel=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
                receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;
                return Ok(true);
            }
            if let Some(cancel) = owner.parameter_cancel.as_mut() {
                if let Some((turn,_)) = cx.with_original_effect(cx.retained_grant(), |cx,effect| cancel.return_alias_step(cx.original_cancel_token(),effect))? {
                    let result = match turn.result {
                        Ok(step) => Ok(step),
                        Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => Ok(RetainedCloneStep::Progress(Default::default())),
                        Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(error),
                    };
                    receive_original_retained_work(cx,RetainedWorkOutcome { result, work:turn.work })?;
                }
                if cancel.terminal_is_empty() { owner.parameter_cancel=None; }
                return Ok(true);
            }
`);
const begin = result.indexOf("            if owner.params.is_none(){let Some(cancel)");
const end = result.indexOf("        }\n        let session=owner.session.as_mut()",begin);
if(begin<0||end<=begin)throw Error("Native clipboard session birth shape changed");
result = required(result,result.slice(begin,end),`            if owner.params.is_none() && owner.admission.terminal_is_empty() {
                let mut cancel = None;
                let admitted = cx.admit_original_cancel_alias(&mut cancel);
                owner.params = cancel.map(|cancel| semio_framework_job_worker_authority::BatchJobParams {
                    operation:cx.operation(),
                    generation:cx.generation(),
                    cancel,
                    config:semio_framework_job_worker_authority::BatchDriveConfig {
                        retained:cx.retained_grant(),
                        site:"renderer_original_clipboard",
                        stage:semio_framework_job::InteractiveStage::InteractiveStep,
                        fuel_per_step:cx.fuel_remaining(),
                        step_budget_us:cx.deadline_us().saturating_sub(cx.now_us().unwrap_or(cx.deadline_us())),
                    },
                    now_us:cx.clock_source(),
                });
                admitted?;
                return Ok(true);
            }
            owner.admission.advance(&mut owner.job,&mut owner.params,&mut owner.session,cx)?;
            return Ok(true);
`);
const metered = "let step=outcome.acknowledge_outcome(cx.retained_grant());cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);";
if(result.split(metered).length!==4)throw Error("Native clipboard outcome receipt count changed");
result=result.replaceAll(metered,"let acknowledged=outcome.acknowledge_outcome(cx)?;");
const beforeAck = result.split("\n").filter(line=>line.includes("let acknowledged="));
for(const line of beforeAck){
    const next=line.replace("if outcome.outcome_descriptor().is_some()","if !acknowledged").replace("if outcome.outcome_descriptor().is_none()","if acknowledged");
    if(next===line)throw Error("Native clipboard outcome condition changed");
    result=required(result,line,next);
}
result = required(result,"    use semio_framework_job::{InteractiveJob};\nuse semio_framework_job_worker::{WorkerJobPoll as Poll};use semio_framework_job_worker_authority::{WorkerJobInitializer};",`    use semio_framework_job::{InteractiveJob, close_original_retained_step, receive_original_retained_work};
    use semio_framework_job_worker::{WorkerJobDemandError, WorkerJobPoll as Poll};
    use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError};`);
const helpersStart=result.indexOf(cfg+"\nfn clipboard_demand_failure(");
const helpersEnd=result.indexOf(cfg+"\npub fn validate_clipboard_source",helpersStart);
if(helpersStart<0||helpersEnd<=helpersStart)throw Error("Native clipboard error helpers changed");
result=result.slice(0,helpersStart)+result.slice(helpersEnd);
if((await Bun.file(path).text())!==original)throw Error("Shared native clipboard source changed");
await Bun.write(path,result);
console.log("[DEBUG] Native WGPU clipboard now uses the original WorkerJobSessionAdmission and paid outcome receipts");
