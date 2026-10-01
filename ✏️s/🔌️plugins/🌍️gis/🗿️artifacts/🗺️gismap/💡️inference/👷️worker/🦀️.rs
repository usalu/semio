//! 🌍 Finite GIS proposal behavior contributed to the native document service host.
use crate::inference_schema::*;
use semio_framework_os_kernel::os_directory::schema::{DocumentScope,mint_directory_command_request_id};
use semio_framework_os_kernel::os_directory::client::{InstalledServiceContributionV1,InstalledServiceDriverV1,InstalledServiceTurnV1,DocumentHttpPortCodeV1};
use semio_framework_os_kernel::os_dsl::{DslValue,ToValue,FromValue};

/// 💡️ Pure finite driver for one host-owned ephemeral inference port. It performs NO I/O: every
/// turn returns the single bounded action the shell should take next, and every completed action is
/// folded back through the shared `reduce_gis_map_inference_port_v1` reducer the browser worker also
/// runs. It refuses to leave `Idle` at all unless the document's execution-target lease is verified,
/// it never has more than one action in flight, it never invents a phase the server has not
/// reported, and a terminal phase is hard — no later turn or completion can move it.
#[cfg(not(target_arch = "wasm32"))]
pub struct GisMapInferenceDriverV1 {
    scope: DocumentScope,
    status: GisMapInferencePortStatusV1,
    lease_verified: bool,
    intent: Option<GisMapInferenceIntentV1>,
    in_flight: bool,
    turns: u32,
    next_poll_at_ms: u64,
}

/// 🎬 One operator intent the driver may still be holding.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapInferenceIntentV1 {
    Propose,
    Cancel,
    Approve,
}

/// 🎯 The single bounded action one turn asks for.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GisMapInferenceTurnV1 {
    Idle,
    WaitUntil(u64),
    Submit,
    Poll { job_id: String, after: u64 },
    Cancel { job_id: String },
    Approve { job_id: String, proposal_hash: String },
    Terminal,
}

#[cfg(not(target_arch = "wasm32"))]
impl GisMapInferenceDriverV1 {
    /// 🔁 Highest number of poll turns one job may take before it is reported indeterminate.
    pub const MAX_POLL_TURNS: u32 = 240;
    /// ⏱ Bounded poll cadence — a timer-armed reschedule, never a busy loop.
    pub const POLL_INTERVAL_MS: u64 = 750;
    /// ⏳ Lifetime one submitted job asks the hub for.
    pub const JOB_LIFETIME_MS: u64 = 60_000;

    /// 🆕 Builds one driver. `lease_verified` is the caller's own answer to "does this document own a
    /// verified, live execution target right now" — the driver never infers it.
    pub fn new(scope: DocumentScope, lease_verified: bool) -> Self {
        Self { scope, status: GisMapInferencePortStatusV1::default(), lease_verified, intent: None, in_flight: false, turns: 0, next_poll_at_ms: 0 }
    }

    pub fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    pub fn status(&self) -> &GisMapInferencePortStatusV1 {
        &self.status
    }

    /// 🎬 Records one operator intent. A terminal port accepts none.
    pub fn intend(&mut self, intent: GisMapInferenceIntentV1) {
        if self.status.phase.terminal() {
            return;
        }
        if intent == GisMapInferenceIntentV1::Cancel {
            self.apply(&GisMapInferencePortEventV1::Cancel);
        }
        self.intent = Some(intent);
    }

    /// 🧮 Folds one exact answer back through the shared reducer and releases the in-flight slot.
    pub fn complete(&mut self, event: &GisMapInferencePortEventV1) {
        self.in_flight = false;
        self.apply(event);
    }

    fn apply(&mut self, event: &GisMapInferencePortEventV1) {
        self.status = reduce_gis_map_inference_port_v1(&self.status, event);
    }

    /// 🔄 One bounded turn. It asks for at most one action, never two, and arms a timer instead of
    /// spinning whenever the only remaining work is the next poll.
    ///
    /// ✅️ The shared reducer owns the whole approve admission (offered, a matching bounded
    /// preview, no pending cancel): the driver applies the intent FIRST and only asks for the
    /// call when that reducer actually entered `Approving`, so this native path is structurally
    /// incapable of approving something the browser port would refuse.
    pub fn turn(&mut self, now_ms: u64) -> GisMapInferenceTurnV1 {
        if self.status.phase.terminal() {
            return GisMapInferenceTurnV1::Terminal;
        }
        if !self.lease_verified {
            self.apply(&GisMapInferencePortEventV1::LeaseUnverified);
            return GisMapInferenceTurnV1::Terminal;
        }
        if self.in_flight {
            return GisMapInferenceTurnV1::WaitUntil(now_ms.saturating_add(Self::POLL_INTERVAL_MS));
        }
        match self.intent.take() {
            Some(GisMapInferenceIntentV1::Propose) if self.status.phase == GisMapInferencePortPhaseV1::Idle => {
                self.apply(&GisMapInferencePortEventV1::Start);
                self.in_flight = true;
                return GisMapInferenceTurnV1::Submit;
            }
            Some(GisMapInferenceIntentV1::Cancel) => {
                if let Some(job_id) = self.status.job_id.clone() {
                    self.in_flight = true;
                    return GisMapInferenceTurnV1::Cancel { job_id };
                }
            }
            Some(GisMapInferenceIntentV1::Approve) => {
                let before = self.status.phase;
                self.apply(&GisMapInferencePortEventV1::Approve);
                if before != self.status.phase && self.status.phase == GisMapInferencePortPhaseV1::Approving {
                    if let (Some(job_id), Some(proposal_hash)) = (self.status.job_id.clone(), self.status.proposal_hash.clone()) {
                        self.in_flight = true;
                        return GisMapInferenceTurnV1::Approve { job_id, proposal_hash };
                    }
                }
            }
            _ => {}
        }
        let Some(job_id) = self.status.job_id.clone() else {
            return GisMapInferenceTurnV1::Idle;
        };
        if self.turns >= Self::MAX_POLL_TURNS {
            self.apply(&GisMapInferencePortEventV1::Failed(GisMapInferencePortCodeV1::Transport));
            return GisMapInferenceTurnV1::Terminal;
        }
        if now_ms < self.next_poll_at_ms {
            return GisMapInferenceTurnV1::WaitUntil(self.next_poll_at_ms);
        }
        self.turns += 1;
        self.in_flight = true;
        self.next_poll_at_ms = now_ms.saturating_add(Self::POLL_INTERVAL_MS);
        GisMapInferenceTurnV1::Poll { job_id, after: self.status.cursor }
    }
}


/// 🧩 Actual native GIS owner inventory for an outward executable composition.
pub fn gis_map_service_contribution_v1() -> InstalledServiceContributionV1 {
    InstalledServiceContributionV1 { owner: "gis", service_id: GIS_MAP_INFERENCE_SERVICE_ID, create: |scope,verified| Box::new(NativeGisMapServiceV1 { driver: GisMapInferenceDriverV1::new(scope,verified), request_id: mint_directory_command_request_id(), reconcile_required:false, recovery_turns:0, next_recovery_at_ms:0, recovery_in_flight:false, exhausted:false }) }
}

struct NativeGisMapServiceV1 {
    driver: GisMapInferenceDriverV1,
    request_id: String,
    reconcile_required: bool,
    recovery_turns: u32,
    next_recovery_at_ms: u64,
    recovery_in_flight: bool,
    exhausted: bool,
}

impl InstalledServiceDriverV1 for NativeGisMapServiceV1 {
    fn intend(&mut self, action: &str, payload: DslValue) -> Result<(),DocumentHttpPortCodeV1> {
        let DslValue::Object(fields)=payload else { return Err(DocumentHttpPortCodeV1::Invalid) };
        match action {
            "propose" if fields.is_empty() => self.driver.intend(GisMapInferenceIntentV1::Propose),
            "cancel"|"close" if fields.is_empty() => self.driver.intend(GisMapInferenceIntentV1::Cancel),
            "approve" if fields.is_empty() => self.driver.intend(GisMapInferenceIntentV1::Approve),
            _ => return Err(DocumentHttpPortCodeV1::Invalid),
        }
        Ok(())
    }

    fn turn(&mut self, now_ms:u64) -> InstalledServiceTurnV1 {
        if self.exhausted { return InstalledServiceTurnV1::Terminal; }
        if self.reconcile_required {
            if self.recovery_in_flight || now_ms < self.next_recovery_at_ms { return InstalledServiceTurnV1::WaitUntil(self.next_recovery_at_ms.max(now_ms.saturating_add(GisMapInferenceDriverV1::POLL_INTERVAL_MS))); }
            if self.recovery_turns >= GisMapInferenceDriverV1::MAX_POLL_TURNS { self.exhausted=true; return InstalledServiceTurnV1::Terminal; }
            self.recovery_turns+=1;
            self.recovery_in_flight=true;
            self.next_recovery_at_ms=now_ms.saturating_add(GisMapInferenceDriverV1::POLL_INTERVAL_MS);
            return InstalledServiceTurnV1::Call {action:"reconcile".into(),payload:DslValue::Object(vec![("schema".into(),"semio.framework.job-reconcile/v1".to_value()),("version".into(),1u32.to_value()),("requestId".into(),self.request_id.to_value())])};
        }
        let (action,payload)=match self.driver.turn(now_ms) {
            GisMapInferenceTurnV1::Idle => return InstalledServiceTurnV1::Idle,
            GisMapInferenceTurnV1::Terminal => return InstalledServiceTurnV1::Terminal,
            GisMapInferenceTurnV1::WaitUntil(time) => return InstalledServiceTurnV1::WaitUntil(time),
            GisMapInferenceTurnV1::Submit => ("submit", GisMapInferenceJobRequestV1 { schema:"semio.hub.inference-request/v1".into(),version:1,request_id:self.request_id.clone(),service_id:GIS_MAP_INFERENCE_SERVICE_ID.into(),policy_version:1,lifetime_ms:GisMapInferenceDriverV1::JOB_LIFETIME_MS }.to_value()),
            GisMapInferenceTurnV1::Poll {job_id,after} => ("events",DslValue::Object(vec![("jobId".into(),job_id.to_value()),("after".into(),after.to_value())])),
            GisMapInferenceTurnV1::Cancel {job_id} => ("cancel",DslValue::Object(vec![("jobId".into(),job_id.to_value())])),
            GisMapInferenceTurnV1::Approve {job_id,proposal_hash} => ("approve",GisMapInferenceApprovalRequestV1 {schema:"semio.hub.inference-approval/v1".into(),version:1,job_id,proposal_hash}.to_value()),
        };
        InstalledServiceTurnV1::Call {action:action.into(),payload}
    }

    fn complete(&mut self,action:&str,request:&DslValue,result:Result<DslValue,DocumentHttpPortCodeV1>) {
        self.recovery_in_flight=false;
        if action=="reconcile" {
            match result.and_then(|value|crate::inference_client::decode_reconcile_v1(value,self.driver.scope(),&self.request_id)) {
                Ok(recovered) => {
                    let semio_framework_job::reconcile::JobReconcileResultV1::Found {job,..}=recovered else { return; };
                    if self.driver.status.job_id.as_ref().is_some_and(|id|id!=&job.receipt.job_id) { self.driver.complete(&GisMapInferencePortEventV1::Failed(GisMapInferencePortCodeV1::Conflict)); return; }
                    if self.driver.status.job_id.is_none() { self.driver.complete(&GisMapInferencePortEventV1::Receipt(job.receipt.clone())); }
                    if let Some(approval)=job.approval {
                        if approval.state==GisMapReconciledApprovalStateV1::UndoPrepared { return; }
                        if let Some(receipt)=approval.receipt {
                            self.driver.status.job_id=Some(job.receipt.job_id);
                            self.driver.status.proposal_hash=job.receipt.proposal_hash;
                            self.driver.complete(&GisMapInferencePortEventV1::Indeterminate(GisMapInferencePortCodeV1::Transport));
                            self.driver.complete(&GisMapInferencePortEventV1::Approval(receipt));
                            self.reconcile_required=false;
                            return;
                        }
                    }
                    let cancel_requested=job.page.cancel_requested;
                    let page=GisMapInferenceEventPageV1 {schema:"semio.hub.inference-job-events/v1".into(),job_id:job.page.job_id,state:job.page.state,proposal_state:job.page.proposal_state,cancel_requested,stale:job.page.proposal_state==GisMapInferenceProposalStateV1::Stale,proposal_hash:job.page.proposal_hash,preview:None,events:job.page.events,progress:job.page.progress,next_cursor:job.page.next_cursor};
                    self.driver.complete(&GisMapInferencePortEventV1::Page(page));
                    self.reconcile_required=false;
                    if self.driver.status.cancel_requested && !cancel_requested { self.driver.intend(GisMapInferenceIntentV1::Cancel); }
                }
                Err(DocumentHttpPortCodeV1::Transport|DocumentHttpPortCodeV1::Cancelled|DocumentHttpPortCodeV1::Unavailable) => {}
                Err(code) => self.driver.complete(&GisMapInferencePortEventV1::Failed(document_http_port_code(code))),
            }
            return;
        }
        if matches!(result,Err(DocumentHttpPortCodeV1::Transport|DocumentHttpPortCodeV1::Cancelled|DocumentHttpPortCodeV1::Unavailable)) {
            self.reconcile_required=true;
            self.driver.complete(&GisMapInferencePortEventV1::Indeterminate(GisMapInferencePortCodeV1::Transport));
            return;
        }
        let event=result.and_then(|value|match action {
            "submit" => GisMapInferenceJobReceiptV1::from_value(value).map_err(|_|DocumentHttpPortCodeV1::Invalid).and_then(|receipt| (receipt.validate()).then_some(GisMapInferencePortEventV1::Receipt(receipt)).ok_or(DocumentHttpPortCodeV1::Invalid)),
            "events"|"cancel" => {
                let DslValue::Object(fields)=request else {return Err(DocumentHttpPortCodeV1::Invalid)};
                let job=fields.iter().find(|(key,_)|key=="jobId").and_then(|(_,value)|String::from_value(value.clone()).ok()).ok_or(DocumentHttpPortCodeV1::Invalid)?;
                GisMapInferenceEventPageV1::from_value(value).map_err(|_|DocumentHttpPortCodeV1::Invalid).and_then(|page|page.validate(&job).then_some(GisMapInferencePortEventV1::Page(page)).ok_or(DocumentHttpPortCodeV1::Invalid))
            }
            "approve" => {
                let request=GisMapInferenceApprovalRequestV1::from_value(request.clone()).map_err(|_|DocumentHttpPortCodeV1::Invalid)?;
                GisMapInferenceApprovalReceiptV1::from_value(value).map_err(|_|DocumentHttpPortCodeV1::Invalid).and_then(|receipt|receipt.validate(&request.job_id,&request.proposal_hash).then_some(GisMapInferencePortEventV1::Approval(receipt)).ok_or(DocumentHttpPortCodeV1::Invalid))
            }
            _=>Err(DocumentHttpPortCodeV1::Invalid),
        }).unwrap_or_else(|code|GisMapInferencePortEventV1::Failed(document_http_port_code(code)));
        self.driver.complete(&event);
    }

    fn status(&self)->DslValue { self.driver.status().to_value() }
    fn terminal(&self)->bool { self.exhausted || self.driver.status().phase.terminal() }
}

fn document_http_port_code(code:DocumentHttpPortCodeV1)->GisMapInferencePortCodeV1 {
    match code {
        DocumentHttpPortCodeV1::Invalid=>GisMapInferencePortCodeV1::Invalid,
        DocumentHttpPortCodeV1::Bounds=>GisMapInferencePortCodeV1::Bounds,
        DocumentHttpPortCodeV1::Cancelled=>GisMapInferencePortCodeV1::Cancelled,
        DocumentHttpPortCodeV1::Denied=>GisMapInferencePortCodeV1::Denied,
        DocumentHttpPortCodeV1::Transport=>GisMapInferencePortCodeV1::Transport,
        DocumentHttpPortCodeV1::NotFound=>GisMapInferencePortCodeV1::NotFound,
        DocumentHttpPortCodeV1::Conflict=>GisMapInferencePortCodeV1::Conflict,
        DocumentHttpPortCodeV1::Gone=>GisMapInferencePortCodeV1::Expired,
        DocumentHttpPortCodeV1::Capacity=>GisMapInferencePortCodeV1::Capacity,
        DocumentHttpPortCodeV1::Unavailable=>GisMapInferencePortCodeV1::Unavailable,
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
