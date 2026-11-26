//! 📥️ Receives a bitmap solve in the original inference operation.

use super::{BitmapInferenceJob, BitmapSnapshot};
use crate::standards::v1::subsets::any::io::text::snapshot::{BitmapSnapshotDsl,BitmapSnapshotPhysicalFields,color_from_dsl,pin_from_dsl};
use semio_framework_job::{Generation, InteractiveJob, JobOutcomeBorrow, Operation, OperationId, OriginalTurnFailure, RevisionId, StepContext};
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonReadLimits,JsonSourceCursor,Value,Number};
use semio_framework_value::{NativeDecodeControl, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, RetireOwned, ValueError, ValueRefusalKind};
use semio_framework_value::native_decoding::NativeDecodeContinuation;
use semio_plugin::{ArtifactInferenceExecution,ArtifactInferenceExecutionError,ArtifactInferenceExecutionRequest,ArtifactInferenceExecutionStep,RetainedArtifactInference};
use store::os_store::codec_registry::native_snapshot::{OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactHistoryReadLimits};
use store::os_spr::history::HistoryLog;

const PAGE:usize=4096;
type Pair=OriginalArtifactPairReceiving<BitmapSnapshotDsl>;

#[derive(Default,RetireOwned)]
struct Sources {
    pack:Option<Vec<u8>>,
    spr:Option<Vec<u8>>,
    checkpoint:Option<Vec<u8>>,
    snapshot:Option<BitmapSnapshot>,
    fields:Option<BitmapSnapshotPhysicalFields>,
    genesis:Option<(BitmapSnapshotDsl,HistoryLog)>,
    history:Option<HistoryLog>,
    failure:Option<OriginalArtifactPairFailure>,
}

#[derive(Default)]
pub(super) struct BitmapRoute {
    parser:Option<JsonGrammarCursor<Value>>,
    decoder:Option<NativeDecodeContinuation>,
    root:Option<Value>,
    pair:Option<Box<Pair>>,
    pair_close:RetirementDemand,
    pair_refusal:semio_framework_job::OriginalDomainReservation<OriginalArtifactPairFailure>,
    sources:Option<Sources>,
    sources_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    parser_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    root_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    execution_close:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    worker:Option<Box<BitmapInferenceJob>>,
    execution:Option<ArtifactInferenceExecution>,
    stage:u8,
    byte:usize,
    output_page:usize,
    ready:bool,
}

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn moved<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn fits(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}

fn effect<R>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<R,ValueError>)->Result<Option<R>,OriginalTurnFailure>{
    let grant=cx.retained_grant();
    if !fits(grant,d){return Ok(None);}
    let Some((result,work))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(None);};
    cx.consume_work(work);
    result.map(Some).map_err(|error|{
        let progress=error.retained_progress();
        let mut failure=OriginalTurnFailure::from(error);
        failure.receipt_refusal=cx.consume_retained(progress).err();
        failure
    })
}

fn paid(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce()->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
    let done=effect(cx,d,|effect|{
        if !effect.performed(1)?{return Ok(false);}
        run()?;
        if d.capacity_bytes!=0{*effect.actual()=Some(d.capacity_bytes);}
        Ok(true)
    })?.unwrap_or(false);
    if done{cx.consume_retained(receipt(d))?;}
    Ok(done)
}

fn close_demand<T:RetireOwned>(slot:&Option<T>,ticket:&Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,body:usize)->Result<RetirementDemand,ValueError>{
    if let Some(ticket)=ticket{return semio_framework_value::factory_ticket_demands(ticket,body);}
    slot.as_ref().map_or(Ok(Default::default()),semio_framework_value::retirement::controlled::controlled_retirement_birth_demand)
}

fn close_value<T:RetireOwned>(slot:&mut Option<T>,ticket:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
    let body=cx.retained_grant().maximum_copy_bytes.min(PAGE);
    let d=close_demand(slot,ticket,body)?;
    if ticket.is_some(){
        semio_framework_job::close_original_effect_step(cx,d,|effect|semio_framework_value::close_factory_ticket(ticket,body,effect))?;
    }else if slot.is_some(){
        semio_framework_job::close_original_retained_effect_step(cx,d,|effect|{
            let admitted=semio_framework_value::admit_owned_retirement(slot,effect).result?;
            let progress=admitted.map_or_else(Default::default,|(owner,progress)|{*ticket=Some(owner);progress});
            Ok::<_,ValueError>(RetainedCloneStep::Progress(progress))
        })?;
    }
    Ok(())
}

fn member<'a>(value:&'a Value,name:&str)->Option<&'a Value>{
    let Value::Object(fields)=value else{return None;};
    fields.iter().find(|(key,_)|*key==name).map(|(_,value)|value)
}

fn document_fields(root:&Value)->Result<(&str,&str,Option<&[Value]>),ValueError>{
    let Value::Object(request)=root else{return Err(invalid("bitmap solve expects an object"));};
    if request.len()>2{return Err(invalid("bitmap solve contains an undeclared member"));}
    if member(root,"snapshot").is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"bitmap cold inference requires a bound artifact document"));}
    if request.iter().any(|(key,_)|!matches!(key,"document"|"checkpoint")){return Err(invalid("bitmap solve contains an undeclared member"));}
    let document=member(root,"document").ok_or_else(||invalid("bitmap solve requires its bound document"))?;
    let Value::Object(fields)=document else{return Err(invalid("bound bitmap document expects an object"));};
    if fields.len()!=2{return Err(invalid("bound bitmap document requires exactly Pack and SPR"));}
    let (Some(Value::String(pack)),Some(Value::String(spr)))=(member(document,"pack"),member(document,"spr")) else{return Err(invalid("bound bitmap document requires textual Pack and SPR"));};
    let checkpoint=match member(root,"checkpoint"){None|Some(Value::Null)=>None,Some(Value::Array(values))=>Some(values.as_slice()),_=>return Err(invalid("bitmap checkpoint expects octets"))};
    Ok((pack,spr,checkpoint))
}

fn reserve<T>(cx:&mut StepContext<'_>,count:usize,destination:&mut Vec<T>)->Result<bool,OriginalTurnFailure>{
    let bytes=count.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"bitmap inference allocation overflow"))?;
    paid(cx,RetirementDemand{capacity_bytes:bytes,..moved::<Vec<T>>()},||{
        destination.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"bitmap inference allocation refused"))
    })
}

fn native_pair_effect<R>(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->Result<R,OriginalArtifactPairFailure>)->Result<Option<Result<R,OriginalArtifactPairFailure>>,OriginalTurnFailure>{
    let (mut scope,mut next,mut committed)=(None,None,0);
    if !cx.with_native_receiving_work(&mut scope,d,|native,_,work|{
        let before=native.actual_work();
        let work=work.saturating_sub(semio_framework_value::ORIGINAL_INVOCATION_WORK);
        if work!=0{next=native.with_allocated_effect(d,work,run)?;}
        committed=native.actual_work().saturating_sub(before);
        Ok(())
    })?{return Ok(None);}
    cx.consume_work(committed);
    cx.settle_receiving_scope()?;
    scope.ok_or_else(||ValueError::invariant("bitmap inference receiving scope vanished"))?.into_result()?;
    Ok(next)
}

fn decode64(text:&str,bytes:&mut Vec<u8>,offset:&mut usize,maximum:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    let source=text.as_bytes();
    if !source.len().is_multiple_of(4){return Err(invalid("bitmap source is not canonical base64").into());}
    let padding=if source.ends_with(b"=="){2}else{usize::from(source.ends_with(b"="))};
    let length=source.len()/4*3-padding;
    if length>maximum{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"bitmap source exceeds its allocation limit").into());}
    if length==0{return Ok(true);}
    if bytes.capacity()==0{reserve(cx,length,bytes)?;return Ok(false);}
    let want=(cx.retained_grant().maximum_copy_bytes/3*4).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)/4*4).min(PAGE).min(source.len()-*offset);
    if want==0{return Ok(*offset==source.len());}
    let produced=(want/4*3).min(length-bytes.len());
    let d=RetirementDemand{copy_bytes:produced,depth:1,..Default::default()};
    let start=*offset;
    let end=start+want;
    if let Some(result)=effect(cx,d,|effect|{
        if !effect.performed(want as u64)?{return Ok(None);}
        let before=bytes.len();
        for (index,quad) in source[start..end].chunks_exact(4).enumerate(){
            let position=start+index*4;
            match semio_framework_value::bytes::decode_standard_quad(quad.try_into().unwrap(),position,position+4==source.len()){
                Ok((decoded,count))=>bytes.extend_from_slice(&decoded[..count]),
                Err(_)=>return Err(invalid("bitmap source is not canonical base64").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes.len()-before,..Default::default()})),
            }
        }
        Ok(Some(()))
    })?{
        if result.is_some(){*offset=end;cx.consume_retained(receipt(d))?;}
    }
    Ok(*offset==source.len())
}

impl BitmapRoute {
    fn parse(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let source=request.canonical_payload;
        let policy=usize::try_from(request.budgets.allocation_bytes).unwrap_or(usize::MAX);
        let grant=cx.retained_grant();
        if self.parser.is_none(){
            let cursor=JsonSourceCursor::<_,Value>::new(source,JsonMemberPolicy::Reject,JsonReadLimits{maximum_bytes:source.len() as u64,maximum_allocation_bytes:policy,maximum_depth:(request.budgets.recursion_depth as usize).min(grant.maximum_depth),maximum_items:source.len() as u64}).map_err(|_|invalid("invalid bitmap request wire bounds"))?;
            self.parser=Some(cursor.into_grammar().1);
            cx.consume_retained(receipt(moved::<()>()))?;
            cx.consume_fuel(1);
            return Ok(());
        }
        let mut step_progress=RetainedCloneProgress::default();
        let Some((next,work))=cx.with_original_effect(grant,|same,effect|{
            let mut callback=|_|!same.is_cancelled();
            let mut native=match self.decoder.take(){Some(original)=>match NativeDecodeControl::resume(original,&mut callback){Ok(native)=>native,Err((error,original))=>{self.decoder=Some(original);return Err(error);}},None=>NativeDecodeControl::new_retained(&mut callback)};
            let mut parser=JsonSourceCursor::from_grammar(source,self.parser.take().unwrap());
            let result=parser.step(grant.maximum_items.min(64),&mut native,effect);
            step_progress=parser.normal_step_progress();
            self.parser=Some(parser.into_grammar().1);
            self.decoder=Some(native.pause().map_err(|(error,_)|error)?);
            result.map_err(|error|match error{semio_framework_pack_json::JsonError::Native(error)=>error,_=>invalid("invalid bitmap solve request JSON")})
        })? else{return Ok(());};
        cx.consume_work(work);
        cx.consume_retained(step_progress)?;
        if let Some(root)=next?{self.root=Some(root);self.sources=Some(Sources::default());self.stage=1;}
        Ok(())
    }

    fn bytes(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let (pack,spr,checkpoint)=document_fields(self.root.as_ref().unwrap())?;
        let sources=self.sources.as_mut().unwrap();
        let maximum=usize::try_from(request.budgets.allocation_bytes).unwrap_or(usize::MAX);
        let done=match self.stage {
            1=>decode64(pack,sources.pack.get_or_insert_with(Vec::new),&mut self.byte,maximum,cx)?,
            2=>decode64(spr,sources.spr.get_or_insert_with(Vec::new),&mut self.byte,maximum.saturating_sub(sources.pack.as_ref().unwrap().len()),cx)?,
            3=>if let Some(source)=checkpoint{
                if source.len()>super::engine::job::MAX_CHECKPOINT_BYTES{return Err(invalid("bitmap checkpoint exceeds its declared limit").into());}
                let output=sources.checkpoint.get_or_insert_with(Vec::new);
                if output.len()==source.len(){true}else if output.capacity()==0{reserve(cx,source.len(),output)?;false}else{
                    let count=cx.retained_grant().maximum_copy_bytes.min(source.len()-output.len()).min(PAGE).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                    if count==0{return Ok(());}
                    let d=RetirementDemand{copy_bytes:count,depth:1,..Default::default()};
                    let before=output.len();
                    if let Some(result)=effect(cx,d,|effect|{
                        if !effect.performed(count as u64)?{return Ok(false);}
                        for item in &source[before..before+count]{
                            let Value::Number(Number::UInt(byte))=item else{return Err(invalid("bitmap checkpoint contains a non-octet"));};
                            output.push(u8::try_from(*byte).map_err(|_|invalid("bitmap checkpoint contains a non-octet"))?);
                        }
                        Ok(true)
                    })?{if result{cx.consume_retained(receipt(d))?;}}
                    output.len()==source.len()
                }
            }else{true},
            _=>unreachable!(),
        };
        if done{self.byte=0;self.stage+=1;cx.consume_fuel(1);}
        Ok(())
    }

    fn pair_step(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let sources=self.sources.as_mut().unwrap();
        let (pack,spr)=(sources.pack.as_deref().unwrap(),sources.spr.as_deref().unwrap());
        let before=cx.retained_progress();
        let admission=match self.stage {
            4=>{
                if pack.is_empty()||spr.is_empty(){return Err(invalid("bitmap solve document requires nonempty Pack and SPR").into());}
                let size=u32::try_from(spr.len()).map_err(|_|invalid("bitmap SPR extent exceeds its file policy"))?;
                let max=request.budgets.allocation_bytes;
                let depth=u16::try_from(request.budgets.recursion_depth).unwrap_or(u16::MAX);
                let limits=protocol::PackLimits{max_file_len:max,max_total_alloc:max,max_depth:depth,..Default::default()};
                let history_limits=OriginalArtifactHistoryReadLimits{file_bytes:size,frame_body_bytes:size,records:size,maximum_fields:128,maximum_text_bytes:size,maximum_binary_bytes:size,maximum_json_bytes:size,maximum_json_allocation_bytes:u32::try_from(max).unwrap_or(u32::MAX),maximum_json_items:size,maximum_json_depth:depth};
                let d=Pair::boxed_birth_demands();
                native_pair_effect(cx,d,|native,effect|{
                    if !effect.performed(1)?{return Ok(Default::default());}
                    let candidate=Pair::admit_boxed(pack,spr,limits,history_limits,native,effect.grant())?;
                    let Some((candidate,progress))=candidate else{return Ok(Default::default());};
                    *effect.actual()=Some(progress.retained_capacity_bytes);
                    self.pair=Some(candidate);
                    self.stage=5;
                    Ok(progress)
                })?
            }
            5=>{
                let body=cx.retained_grant().maximum_copy_bytes.min(PAGE);
                let pair=self.pair.as_deref_mut().unwrap();
                let d=match cx.with_native_retirement(|native|Ok(pair.demands(pack,spr,native,body)))?{
                    Ok(d)=>d,Err(error)=>{sources.failure=Some(error);return Ok(());}
                };
                native_pair_effect(cx,d,|native,effect|{
                    let progress=pair.step(pack,spr,native,body,effect).result?;
                    if pair.is_ready(){self.stage=6;}
                    Ok(progress)
                })?
            }
            6=>{
                let pair=self.pair.as_deref_mut().unwrap();
                let d=pair.take_demand();
                native_pair_effect(cx,d,|_,effect|{
                    if !effect.performed(1)?{return Ok(Default::default());}
                    let progress=pair.take_into(&mut sources.genesis,effect.grant())?;
                    if sources.genesis.is_some(){self.stage=7;}
                    Ok(progress)
                })?
            }
            _=>unreachable!(),
        };
        if let Some(result)=admission{
            let progress=match result{Ok(progress)=>progress,Err(error)=>{let progress=error.progress;sources.failure=Some(error);progress}};
            post_remaining(progress,between(before,cx.retained_progress()),cx)?;
        }
        if let Some(pair)=self.pair.as_deref().filter(|_|cx.fuel_remaining()>0){
            let body=cx.retained_grant().maximum_copy_bytes.min(PAGE);
            match cx.with_native_retirement(|native|Ok(pair.close_demands(pack,spr,native,body)))?{
                Ok(quote)=>self.pair_close=quote,
                Err(error)=>if sources.failure.is_none(){sources.failure=Some(error);},
            }
        }
        Ok(())
    }

    fn close_pair(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let Some(pair)=self.pair.as_mut()else{return Ok(());};
        if pair.terminal_is_empty(){
            paid(cx,RetirementDemand{release_bytes:size_of::<Pair>(),..moved::<Option<Box<Pair>>>()},||{self.pair=None;Ok(())})?;
            return Ok(());
        }
        let sources=self.sources.as_mut().unwrap();
        let (pack,spr)=(sources.pack.as_deref().unwrap(),sources.spr.as_deref().unwrap());
        let grant=cx.retained_grant();
        let Some((result,performed))=cx.with_original_effect(grant,|same,effect|same.with_native_retirement(|native|{
            pair.begin_close();
            Ok(pair.close_step(pack,spr,native,effect))
        }))?else{return Ok(());};
        let result=result?;
        let turn=match result{
            semio_framework_value::RetainedWorkPoll::Pending{work}=>{cx.consume_work(work);return Ok(());}
            semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,
        };
        cx.consume_work(performed.max(turn.work));
        match turn.result{
            Ok(step)=>cx.consume_retained(step.progress())?,
            Err(error)=>{let progress=error.progress;if sources.failure.is_none(){sources.failure=Some(error);}cx.consume_retained(progress)?;return Err(ValueError::invariant("bitmap original pair close refused with cause retained").into());}
        }
        let body=cx.retained_grant().maximum_copy_bytes.min(PAGE);
        if cx.fuel_remaining()>0{
            if let Ok(quote)=cx.with_native_retirement(|native|Ok(pair.close_demands(pack,spr,native,body)))?{self.pair_close=quote;}
        }
        Ok(())
    }

    fn project(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        let sources=self.sources.as_mut().unwrap();
        if let Some((_,history))=sources.genesis.as_ref(){
            if history.schema!="s.wfc.bitmap"||history.doc_id.is_empty(){return Err(invalid("bitmap SPR identity differs from its declared artifact schema").into());}
            if !history.edits.is_empty()||!history.transitions.is_empty()||history.composition.is_some()||!history.conflicts.is_empty()||history.viewer_line.is_some()||history.viewer_checkpoint.is_some(){
                return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"bitmap cold inference requires retained replay for this edited document").into());
            }
            paid(cx,moved::<(BitmapSnapshotDsl,BitmapSnapshot,BitmapSnapshotPhysicalFields,HistoryLog)>(),||{
                let (genesis,history)=sources.genesis.take().unwrap();
                let (snapshot,fields)=genesis.into_snapshot_fields();
                sources.history=Some(history);
                sources.snapshot=Some(snapshot);
                sources.fields=Some(fields);
                Ok(())
            })?;
            return Ok(());
        }
        let snapshot=sources.snapshot.as_mut().unwrap();
        let fields=sources.fields.as_ref().unwrap();
        let maximum=usize::try_from(request.budgets.allocation_bytes).unwrap_or(usize::MAX);
        if self.stage==8 && decode64(&fields.pixels,&mut snapshot.input.pixels,&mut self.byte,maximum,cx)?{self.byte=0;self.stage=9;}
        if self.stage==9 && project_rows(cx,&fields.palette,&mut snapshot.input.palette,color_from_dsl)?{self.stage=10;}
        if self.stage==10 && project_rows(cx,&fields.pinned,&mut snapshot.pinned,pin_from_dsl)?{self.stage=11;}
        Ok(())
    }

    fn worker_step(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,revision:RevisionId,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
        match self.stage{
            11=>{
                let sources=self.sources.as_mut().unwrap();
                BitmapInferenceJob::validate_snapshot(sources.snapshot.as_ref().unwrap(),sources.checkpoint.as_deref()).map_err(invalid)?;
                let d=RetirementDemand{capacity_bytes:size_of::<BitmapInferenceJob>(),..moved::<(BitmapSnapshot,Option<Vec<u8>>,Option<Box<BitmapInferenceJob>>)>()};
                if paid(cx,d,||{
                    let operation=Operation::new(OperationId(request.operation),revision,Generation(request.generation),0);
                    let mut worker=BitmapInferenceJob::from_admitted_snapshot(operation,sources.snapshot.take().unwrap(),sources.checkpoint.take());
                    worker.publish_intermediate=false;
                    self.worker=Some(Box::new(worker));
                    Ok(())
                })?{self.stage=12;}
            }
            12=>match self.worker.as_deref_mut().unwrap().step_value(cx)?{
                Some(JobOutcomeBorrow::Complete { .. })=>self.stage=13,
                Some(JobOutcomeBorrow::Cancelled { .. })=>return Err(ValueError::literal(ValueRefusalKind::Canceled,"bitmap inference cancelled").into()),
                Some(JobOutcomeBorrow::Fault { .. })=>return Err(invalid("bitmap inference worker refused the solve").into()),
                _=>{},
            },
            13=>{
                let payload=self.worker.as_ref().unwrap().output_payload.as_ref().ok_or_else(||ValueError::invariant("completed bitmap solve lost its published result"))?;
                let maximum=usize::try_from(request.budgets.allocation_bytes).unwrap_or(usize::MAX);
                if payload.len()>maximum{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"bitmap result exceeds its allocation limit").into());}
                if self.execution.is_none(){
                    let d=RetirementDemand{capacity_bytes:"valid".len()+"unreported".len(),..moved::<ArtifactInferenceExecution>()};
                    paid(cx,d,||{self.execution=Some(ArtifactInferenceExecution{retirement_progress:Default::default(),canonical_payload:Some(Vec::new()),diagnostics:Vec::new(),validity:"valid".into(),quality:"unreported".into(),complete:true,actual_cache_mode:request.requested_cache_mode.clone()});Ok(())})?;
                    return Ok(());
                }
                let output=self.execution.as_mut().unwrap().canonical_payload.as_mut().unwrap();
                if reserve(cx,payload.len(),output)?{self.stage=14;}
            }
            14=>{
                let payload=self.worker.as_ref().unwrap().output_payload.as_ref().unwrap();
                let Some(page)=payload.page(self.output_page)else{self.stage=15;return Ok(());};
                if page.is_empty(){paid(cx,moved::<usize>(),||{self.output_page+=1;Ok(())})?;return Ok(());}
                let output=self.execution.as_mut().unwrap().canonical_payload.as_mut().unwrap();
                let count=(page.len()-self.byte).min(cx.retained_grant().maximum_copy_bytes).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                if count==0{return Ok(());}
                let d=RetirementDemand{copy_bytes:count,depth:1,..Default::default()};
                if effect(cx,d,|effect|{
                    if !effect.performed(count as u64)?{return Ok(false);}
                    output.extend_from_slice(&page[self.byte..self.byte+count]);
                    Ok(true)
                })?.unwrap_or(false){
                    cx.consume_retained(receipt(d))?;
                    self.byte+=count;
                    if self.byte==page.len(){self.output_page+=1;self.byte=0;}
                }
            }
            15=>self.close_owners(cx,false)?,
            _=>unreachable!(),
        }
        Ok(())
    }

    fn owners_are_empty(&self)->bool{
        self.worker.is_none()&&self.pair.is_none()&&self.pair_refusal.terminal_is_empty()&&self.decoder.is_none()
            &&self.sources.is_none()&&self.sources_close.is_none()&&self.parser.is_none()&&self.parser_close.is_none()&&self.root.is_none()&&self.root_close.is_none()
    }

    fn close_owners(&mut self,cx:&mut StepContext<'_>,drop_result:bool)->Result<(),OriginalTurnFailure>{
        if let Some(worker)=self.worker.as_deref_mut(){
            worker.begin_close();
            if worker.terminal_is_empty(){
                paid(cx,RetirementDemand{release_bytes:size_of::<BitmapInferenceJob>(),..moved::<Option<Box<BitmapInferenceJob>>>()},||{self.worker=None;Ok(())})?;
            }else{worker.close_step(cx)?;}
        }else if self.pair.is_some(){self.close_pair(cx)?;
        }else if !self.pair_refusal.terminal_is_empty(){
            let body=cx.retained_grant().maximum_copy_bytes.min(PAGE);
            let demand=self.pair_refusal.retirement_demands(body)?;
            semio_framework_job::close_original_effect_step(cx,demand,|effect|self.pair_refusal.close_step(body,effect))?;
        }else if self.sources.is_some()||self.sources_close.is_some(){close_value(&mut self.sources,&mut self.sources_close,cx)?;
        }else if self.parser.is_some()||self.parser_close.is_some(){close_value(&mut self.parser,&mut self.parser_close,cx)?;
        }else if self.root.is_some()||self.root_close.is_some(){close_value(&mut self.root,&mut self.root_close,cx)?;
        }else if self.decoder.is_some(){paid(cx,moved::<Option<NativeDecodeContinuation>>(),||{self.decoder=None;Ok(())})?;
        }else if drop_result{close_value(&mut self.execution,&mut self.execution_close,cx)?;}
        Ok(())
    }
}

fn project_rows<I,O>(cx:&mut StepContext<'_>,src:&[I],out:&mut Vec<O>,project:fn(&I)->O)->Result<bool,OriginalTurnFailure>{
    if out.len()==src.len(){return Ok(true);}
    if out.capacity()==0{reserve(cx,src.len(),out)?;return Ok(false);}
    let count=(cx.retained_grant().maximum_copy_bytes/size_of::<O>()).min(src.len()-out.len()).min(64).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
    if count==0{return Ok(false);}
    let d=RetirementDemand{copy_bytes:count*size_of::<O>(),depth:1,..Default::default()};
    let rows=&src[out.len()..out.len()+count];
    if effect(cx,d,|effect|{
        if !effect.performed(count as u64)?{return Ok(false);}
        out.extend(rows.iter().map(project));
        Ok(true)
    })?.unwrap_or(false){cx.consume_retained(receipt(d))?;}
    Ok(out.len()==src.len())
}

impl RetainedArtifactInference for BitmapRoute {
    fn step(&mut self,request:&ArtifactInferenceExecutionRequest<'_>,revision:RevisionId,cx:&mut StepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
        let before=cx.retained_progress();
        let result=(||->Result<(),OriginalTurnFailure>{
            if self.sources.as_ref().is_some_and(|sources|sources.failure.is_some())||self.pair_refusal.pending(){
                if !self.pair_refusal.prepare(cx)?{return Ok(());}
                if self.sources.as_ref().is_some_and(|sources|sources.failure.is_some()){
                    self.pair_refusal.capture_original(cx,||self.sources.as_mut().unwrap().failure.take().unwrap())?;
                }
                if let Some(error)=semio_framework_job::admit_original_domain_failure(&mut self.pair_refusal,cx)?{return Err(error);}
                return Ok(());
            }
            if request.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"bitmap inference cancelled").into());}
            match self.stage{
                0=>self.parse(request,cx)?,
                1..=3=>self.bytes(request,cx)?,
                4..=6=>self.pair_step(request,cx)?,
                7=>{self.close_pair(cx)?;if self.pair.is_none(){self.stage=8;}},
                8..=10=>self.project(request,cx)?,
                _=>self.worker_step(request,revision,cx)?,
            }
            if self.stage==15&&self.owners_are_empty()&&paid(cx,moved::<Option<ArtifactInferenceExecution>>(),||Ok(()))?{self.ready=true;}
            Ok(())
        })();
        let progress=between(before,cx.retained_progress());
        if let Err(failure)=result{
            let mut error=ArtifactInferenceExecutionError::original(failure,self.terminal_is_empty());
            error.retained_progress=progress;
            return Err(error);
        }
        let mut execution=if self.ready{self.execution.take()}else{None};
        if let Some(execution)=execution.as_mut(){execution.retirement_progress=progress;}
        Ok(ArtifactInferenceExecutionStep{execution,retained_progress:progress,terminal:self.ready})
    }

    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(worker)=self.worker.as_ref(){
            return if worker.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<BitmapInferenceJob>(),..moved::<Option<Box<BitmapInferenceJob>>>()})}
            else{Ok(RetirementDemand{copy_bytes:worker.next_close_copy_byte_demand()?,capacity_bytes:worker.next_close_capacity_byte_demand(body)?,release_bytes:worker.next_close_release_byte_demand()?,depth:worker.next_close_depth_demand()?})};
        }
        if let Some(pair)=self.pair.as_ref(){
            return if pair.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<Pair>(),..moved::<Option<Box<Pair>>>()})}else{Ok(self.pair_close)};
        }
        if !self.pair_refusal.terminal_is_empty(){return self.pair_refusal.retirement_demands(body);}
        if self.sources.is_some()||self.sources_close.is_some(){return close_demand(&self.sources,&self.sources_close,body);}
        if self.parser.is_some()||self.parser_close.is_some(){return close_demand(&self.parser,&self.parser_close,body);}
        if self.root.is_some()||self.root_close.is_some(){return close_demand(&self.root,&self.root_close,body);}
        if self.decoder.is_some(){return Ok(moved::<Option<NativeDecodeContinuation>>());}
        close_demand(&self.execution,&self.execution_close,body)
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
        let before=cx.retained_progress();
        self.close_owners(cx,true)?;
        Ok(between(before,cx.retained_progress()))
    }

    fn terminal_is_empty(&self)->bool{self.owners_are_empty()&&self.execution.is_none()&&self.execution_close.is_none()}
}

fn between(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{
    RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}
}

fn post_remaining(total:RetainedCloneProgress,posted:RetainedCloneProgress,cx:&mut StepContext<'_>)->Result<(),ValueError>{
    cx.consume_retained(RetainedCloneProgress{
        copied_items:total.copied_items.saturating_sub(posted.copied_items),
        copied_bytes:total.copied_bytes.saturating_sub(posted.copied_bytes),
        retained_capacity_bytes:total.retained_capacity_bytes.saturating_sub(posted.retained_capacity_bytes),
        released_bytes:total.released_bytes.saturating_sub(posted.released_bytes),
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
