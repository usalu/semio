    pub fn prepare_reserved_wire_admission(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::OriginalConstructionStatus;
        use semio_plugin_instance_jobs::reserved_preparation::ReservedWirePhase;
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError};
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||ValueError::invariant("reserved wire requires its preparation"))?;
        owner.validate(self.stores.store.content_revision(),cx)?;
        let Some(length)=owner.raw_wire.as_ref().map(Vec::len)else{return Ok(false)};
        match owner.wire_phase{
            ReservedWirePhase::Cursor=>{
                if owner.wire_cursor.is_none()&&owner.wire_key.is_none()&&owner.wire_schema.is_none(){
                    if ![5,6,7].into_iter().all(|field|owner.header_ready(field)){return Ok(false)}
                    let bytes=size_of::<semio_framework::ToolFactoryKey>()+size_of::<String>();let grant=cx.retained_grant();
                    if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
                    let headers=owner.headers.as_mut().ok_or_else(||ValueError::invariant("reserved wire metadata was donated"))?;
                    *owner.wire_key=Some(semio_framework::ToolFactoryKey::new(std::mem::take(&mut headers[5]),std::mem::take(&mut headers[6])));
                    *owner.wire_schema=Some(std::mem::take(&mut headers[7]));
                    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
                    return Ok(false)
                }
                match semio_framework::action_bus::OriginalWireAdmission::admit(&mut owner.wire_key,&mut owner.wire_schema,length,cx,&mut owner.wire_construction,&mut owner.wire_cursor)?{
                    OriginalConstructionStatus::Failed=>return Err(ValueError::invariant("reserved wire admission construction failed; its causes remain in the preparation").into()),
                    OriginalConstructionStatus::Ready=>owner.advance_wire_phase(ReservedWirePhase::ResetCursor,cx)?,
                    OriginalConstructionStatus::Pending=>{},
                }
            }
            ReservedWirePhase::ResetCursor=>{
                if owner.wire_construction.reset_step(cx)?{owner.advance_wire_phase(ReservedWirePhase::Search,cx)?;}
            }
            ReservedWirePhase::Search=>{
                let cursor=owner.wire_cursor.as_mut().ok_or_else(||ValueError::invariant("reserved wire search lost its original cursor"))?;
                if cursor.advance(&*self.tools.tool_jobs,cx,&mut owner.wire_construction,&mut owner.admission)?{
                    owner.advance_wire_phase(ReservedWirePhase::Close,cx)?;
                }
            }
            ReservedWirePhase::Close=>{
                if !owner.wire_construction.terminal_is_empty(){
                    cx.with_original_retirement(|cx|owner.wire_construction.retire_step(cx))?;
                    return Ok(false)
                }
                let Some(cursor)=owner.wire_cursor.as_mut()else{return Ok(owner.admission.is_some())};
                if !cursor.terminal_is_empty(){
                    cx.with_original_retirement(|cx|cursor.close_step(&*self.tools.tool_jobs,cx))?;
                    return Ok(false)
                }
                let bytes=size_of::<Option<semio_framework::action_bus::OriginalWireAdmission>>();
                let demand=RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};
                cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_turn(cx,demand,|_|{
                    owner.wire_cursor.take();
                    RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}
                }))?;
            }
        }
        Ok(false)
    }
