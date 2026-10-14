//! 🏗️ The defining WidgetInputs constructor produces original owned strings and fixed-page label entries.
use super::{Kind,Localized,Outputs,WidgetInputs};
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneProgress,ordered_map::{BoundedOrdGrant,RetainedOrderedMap,RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapInsertProgress,RetainedOrderedMapInsertStep}},retirement::{RetireOwned,RetirementCursor}};
use std::mem::{ManuallyDrop,size_of};

type Labels=RetainedOrderedMap<String,Localized>;
type Insert=RetainedOrderedMapInsertCursor<String,Localized>;

#[derive(Clone,Copy,PartialEq,Eq)]
struct Source {operation:u64,generation:u64,parent:usize,widget:usize,widget_bytes:usize,kind:usize,ports:usize,port_count:usize}
semio_framework_value::artifact_retire_leaf!(Source);

#[derive(Default,semio_framework_value::RetireOwned)]
struct State {
    source:Option<Source>,
    widget:String,
    labels:Option<Labels>,
    values:Option<Outputs>,
    name:String,
    en:String,
    de:String,
    insertion:Option<Insert>,
    phase:u8,
    port:usize,
    byte:usize,
    reserved:bool,
    text_source:Option<(usize,usize)>,
}

/// 🧵️ Retains every original partial destination through refusal and cancellation.
pub struct WidgetInputsConstruction {state:ManuallyDrop<State>,transferred:bool}
impl Default for WidgetInputsConstruction {fn default()->Self{Self{state:ManuallyDrop::new(State::default()),transferred:false}}}
impl WidgetInputsConstruction {
    /// 🪜️ Performs one original source, UTF8, page insertion, close or header donation event.
    pub fn step(&mut self,widget:&str,kind:&Kind,values:&mut Option<Outputs>,cx:&mut StepContext<'_>)->Result<Option<WidgetInputs>,OriginalTurnFailure>{
        let identity=Source{operation:cx.operation().0,generation:cx.generation().0,parent:cx.original_source_parent()?.source_identity()?,widget:widget.as_ptr() as usize,widget_bytes:widget.len(),kind:kind as *const Kind as usize,ports:kind.inputs.as_ptr() as usize,port_count:kind.inputs.len()};
        let state=&mut *self.state;
        if let Some(source)=state.source {if source!=identity{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"widget input constructor requires its original source and declaration").into())}}
        if state.phase==10{return Ok(None)}
        if !open(cx){return Ok(None)}
        if state.source.is_none(){
            if values.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"widget input construction requires its original resolved value producer").into())}
            post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;
            state.values=values.take();state.labels=Some(Labels::default());state.source=Some(identity);state.phase=1;return Ok(None)
        }
        match state.phase {
            1=>{if text_event(&mut state.widget,widget,&mut state.byte,&mut state.reserved,&mut state.text_source,cx)?{state.phase=2;}}
            2=>{post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=if state.port==kind.inputs.len(){9}else{3};}
            3|4|5=>{
                let port=kind.inputs.get(state.port).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input declaration ordinal changed"))?;
                let done=match state.phase{3=>text_event(&mut state.name,&port.name,&mut state.byte,&mut state.reserved,&mut state.text_source,cx)?,4=>text_event(&mut state.en,&port.label.en,&mut state.byte,&mut state.reserved,&mut state.text_source,cx)?,_=>text_event(&mut state.de,&port.label.de,&mut state.byte,&mut state.reserved,&mut state.text_source,cx)?};
                if done{state.phase+=1;}
            }
            6=>{
                let grant=cx.retained_grant();
                if grant.maximum_copy_bytes<Insert::constructor_copy_bytes()||grant.maximum_capacity_bytes<Insert::constructor_capacity_bytes(){return Ok(None)}
                let map=state.labels.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input labels have no original workspace"))?;
                let key=std::mem::take(&mut state.name);let label=Localized{en:std::mem::take(&mut state.en),de:std::mem::take(&mut state.de)};
                match Insert::admit(map,key,label,grant){
                    Ok((insertion,receipt))=>{state.insertion=Some(insertion);state.phase=7;post(cx,receipt)?;}
                    Err((error,map,key,label))=>{state.labels=Some(map);state.name=key;state.en=label.en;state.de=label.de;return Err(OriginalTurnFailure::debit_original(error,cx));}
                }
            }
            7=>{
                let insertion=state.insertion.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input label insertion disappeared"))?;
                if insertion.output_ready(){
                    post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.labels=insertion.take();insertion.begin_close();state.phase=8;
                }else{
                    let grant=cx.retained_grant();let step=insertion.advance(RetainedOrderedMapInsertGrant{retirement:grant,comparison:BoundedOrdGrant{maximum_items:1,maximum_bytes:1},maximum_moved_items:grant.maximum_items,maximum_moved_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes}).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
                    let receipt=match step{RetainedOrderedMapInsertStep::Progress(receipt)|RetainedOrderedMapInsertStep::Complete{progress:receipt,..}=>receipt};
                    post(cx,insert_receipt(receipt)?)?;
                }
            }
            8=>{
                let insertion=state.insertion.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input insertion close disappeared"))?;
                let receipt=insertion.close_step(cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)?;
                if insertion.terminal_is_empty(){state.insertion.take();state.port+=1;state.phase=2;}
            }
            9=>{
                post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;
                let labels=state.labels.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input labels have no completed original"))?;
                let values=state.values.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input values have no original donor"))?;
                let widget=std::mem::take(&mut state.widget);state.source=None;state.phase=10;return Ok(Some(WidgetInputs{widget,labels,values}))
            }
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"widget input constructor phase is invalid").into()),
        }
        Ok(None)
    }
    pub fn terminal_is_empty(&self)->bool{self.transferred||self.state.source.is_none()&&self.state.values.is_none()&&self.state.labels.is_none()&&self.state.insertion.is_none()&&self.state.widget.capacity()==0&&self.state.name.capacity()==0&&self.state.en.capacity()==0&&self.state.de.capacity()==0}
}

fn open(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();!cx.should_yield()&&!cx.is_cancelled()&&grant.maximum_items>0&&grant.maximum_depth>0}
fn post(cx:&mut StepContext<'_>,receipt:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{let result=cx.consume_retained(receipt);cx.consume_fuel(1);result.map_err(|error|error.with_retained_progress(receipt).into())}
fn insert_receipt(receipt:RetainedOrderedMapInsertProgress)->Result<RetainedCloneProgress,ValueError>{receipt.retirement.checked_add(RetainedCloneProgress{copied_items:receipt.moved_items,copied_bytes:receipt.moved_bytes,retained_capacity_bytes:receipt.retained_capacity_bytes,..Default::default()})}

fn text_event(output:&mut String,source:&str,byte:&mut usize,reserved:&mut bool,binding:&mut Option<(usize,usize)>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    let identity=(source.as_ptr() as usize,source.len());
    if binding.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"widget input UTF8 source identity changed").into())}
    let grant=cx.retained_grant();
    if !*reserved {
        if grant.maximum_capacity_bytes<source.len(){return Ok(false)}
        output.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"widget input UTF8 backing reserve failed"))?;
        let receipt=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()};*reserved=true;*binding=Some(identity);post(cx,receipt)?;return Ok(false)
    }
    if *byte<source.len(){
        let character=source.get(*byte..).and_then(|tail|tail.chars().next()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"widget input UTF8 source boundary changed"))?;
        let bytes=character.len_utf8();if grant.maximum_copy_bytes<bytes{return Ok(false)}
        output.push(character);*byte+=bytes;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(false)
    }
    post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;*byte=0;*reserved=false;*binding=None;Ok(true)
}

impl RetireOwned for WidgetInputsConstruction {
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{true}
    fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl Drop for WidgetInputsConstruction{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"widget input constructor reached Drop before original paid destination closure");if self.terminal_is_empty()&&!self.transferred{unsafe{ManuallyDrop::drop(&mut self.state)}}}}
