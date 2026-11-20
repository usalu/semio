//! 🖼️ Export turns resume their admitted frame-address plan.
use semio_s_artifact_layout_layout_engine_export_primitives::frame_planning::FramePlanState;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value::{RetirementDemand,ValueError};

impl super::LayoutExportJob{
 pub(super) fn prepare_frame_plan(&mut self,snapshot:&LayoutSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if cx.is_cancelled()||cx.should_yield(){return Ok(false);}if self.frame_plan.is_none(){let target=if self.request.as_ref().is_some_and(|request|request.kind==super::LayoutExportKind::Pdf){None}else{self.target_page};self.frame_plan=Some(FramePlanState::new(target));}let state=self.frame_plan.as_mut().unwrap();if state.ready(){return Ok(true);}cx.set_stage("layout.export.frame-addresses");let grant=cx.retained_grant();let Some((result,work))=cx.with_original_effect(grant,|_,effect|{let result=state.prepare(snapshot,effect);semio_s_artifact_layout_layout_engine::scene::metadata_index::record_native_step(effect)?;result})?else{return Ok(false)};cx.consume_work(work);cx.consume_retained(state.last_receipt())?;result.map_err(Into::into)}
 pub(super) fn frame_plan_demand(&self,snapshot:&LayoutSnapshot)->Result<RetirementDemand,ValueError>{self.frame_plan.as_ref().map_or(Ok(RetirementDemand{depth:1,..Default::default()}),|state|state.demand(snapshot))}
}
