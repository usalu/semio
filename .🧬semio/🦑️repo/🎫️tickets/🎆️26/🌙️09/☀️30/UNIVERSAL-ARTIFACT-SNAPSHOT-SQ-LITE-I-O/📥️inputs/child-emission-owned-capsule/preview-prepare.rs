    struct ChildEmissionPreviewJob<A:ArtifactApp>{
        emit:Option<Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>>,
        ephemeral:Option<EphemeralEmit<A>>,
        completion:Option<ArtifactToolCompletion<A>>,
        maximum_bytes:usize,
        closing:bool,
    }
    impl<A:ArtifactApp> semio_framework_job::InteractiveJob for ChildEmissionPreviewJob<A>{
        fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->semio_framework_job::StepOutcome{
            use semio_framework_job::{StepOutcome,JobPayloadStream,RetainedJobPayload,CommitCandidate,JobFault};
            if cx.is_cancelled(){return StepOutcome::Cancelled;}
            if cx.should_yield(){return StepOutcome::Yield;}
            let Some(emit)=self.emit.as_mut()else{return StepOutcome::Complete(CommitCandidate{state:RetainedJobPayload::empty(JobPayloadStream::CommitState),output:RetainedJobPayload::empty(JobPayloadStream::CommitOutput)})};
            cx.set_stage("agent-child-emission-preparation");
            let demand=emit.next_child_preparation_byte_demand().max(1);
            let step=if demand>self.maximum_bytes{Err(plugin_sdk_fault("child emission exceeds its captured preview output allocation authority"))}else{emit.prepare_child_one(1,demand)};
            cx.consume_fuel(1);
            match step{
                Ok(ChildEmitPreparationStep::Pending)=>StepOutcome::Yield,
                Ok(ChildEmitPreparationStep::Ready)=>{
                    let Some(completion)=self.completion.as_ref()else{return StepOutcome::Cancelled};
                    let Some(ephemeral)=self.ephemeral.take()else{return StepOutcome::Cancelled};
                    let emit=self.emit.take().expect("ready preview retains its complete emission");
                    if let Err(rejected)=completion.complete(Ok(emit),ephemeral){
                        self.emit=rejected.emit.ok();self.ephemeral=Some(rejected.ephemeral);
                        return child_emission_preview_fault(cx,&rejected.fault);
                    }
                    StepOutcome::Complete(CommitCandidate{state:RetainedJobPayload::empty(JobPayloadStream::CommitState),output:RetainedJobPayload::empty(JobPayloadStream::CommitOutput)})
                },
                Ok(ChildEmitPreparationStep::Refused(fault))|Err(fault)=>child_emission_preview_fault(cx,&fault),
            }
        }
        fn begin_close(&mut self){self.closing=true;}
        fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep{
            use semio_framework_job::InteractiveJobCloseStep;
            if !self.closing{return InteractiveJobCloseStep::Blocked;}
            if maximum_items==0||maximum_bytes==0{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
            if let Some(emit)=self.emit.as_mut(){
                if let Some(step)=emit.close_child_one(maximum_items.min(1),maximum_bytes){
                    return match step{PluginCloseStep::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{released_items,released_bytes},PluginCloseStep::Complete=>InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0},_=>InteractiveJobCloseStep::Blocked};
                }
                self.emit.take();return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
            }
            if self.ephemeral.take().is_some(){return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};}
            if self.completion.take().is_some(){return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};}
            InteractiveJobCloseStep::Complete
        }
        fn terminal_is_empty(&self)->bool{self.emit.is_none()&&self.ephemeral.is_none()&&self.completion.is_none()}
    }
    fn child_emission_preview_fault(cx:&mut semio_framework_job::StepContext<'_>,fault:&Fault)->semio_framework_job::StepOutcome{
        let detail=crate::retained_command::reducer_fault_detail(fault);
        let payload=cx.payload_from_bytes(semio_framework_job::JobPayloadStream::Fault,&detail).unwrap_or_else(|_|semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault{detail:payload})
    }

