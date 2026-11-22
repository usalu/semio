//! 📨️ The original unit measures a compact InstanceOpen and retains its single donor.
use super::{GuestRelayUnitFlow, Phase, OriginalSourceBinding, OriginalTurnFailure, RetainedCloneProgress, StepContext, ValueError, ValueRefusalKind, refused};
use semio_framework::kernel::OwnedCorePollLimits;
use semio_plugin_guest_contract::original_open_frame::{HEADER_BYTES, OpenView};
use std::mem::size_of;

pub(super) struct OpenEncoding {
    identity: (usize,OwnedCorePollLimits),
    segment: usize,
    offset: usize,
    extent: usize,
}

impl GuestRelayUnitFlow {
    /// 🐣️ Produces the canonical open donor without borrowing outside a paid Source callback.
    pub fn encode_open_frame_step(&mut self, source: OpenView<'_>, cx: &mut StepContext<'_>) -> Result<bool,OriginalTurnFailure> {
        self.validate(cx)?;
        if self.phase!=Phase::Encode||!self.encoder.terminal_is_empty(){return Err(refused().into())}
        if self.open_encoding.is_none(){
            if self.input.is_some(){return Err(refused().into())}
            if !charge(cx,false,size_of::<Option<OpenEncoding>>()+size_of::<Option<OriginalSourceBinding>>())?{return Ok(false)}
            self.binding=Some(OriginalSourceBinding::borrow(cx));
            self.open_encoding=Some(OpenEncoding{identity:source.identity(),segment:1,offset:0,extent:HEADER_BYTES});
            return Ok(false)
        }
        let state=self.open_encoding.as_mut().ok_or_else(refused)?;
        if state.identity!=source.identity(){return Err(refused().into())}
        let segments=source.segments()?;
        if self.input.is_none(){
            if state.segment==segments{
                if state.extent>i32::MAX as usize{return Err(ownership_limit().into())}
                let grant=cx.retained_grant();
                let meta=size_of::<Option<Vec<u8>>>()+size_of::<usize>();
                if grant.maximum_capacity_bytes<state.extent||!charge(cx,false,meta)?{return Ok(false)}
                let mut donor=Vec::new();
                donor.try_reserve_exact(state.extent).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"compact InstanceOpen donor allocation failed"))?;
                let capacity=donor.capacity();
                *self.input=Some(donor);
                cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:capacity,..Default::default()})?;
                state.segment=0;
            } else {
                if !charge(cx,false,size_of::<usize>()*2)?{return Ok(false)}
                let length=source.segment(state.segment,state.extent)?.bytes().len();
                state.extent=state.extent.checked_add(length).ok_or_else(ownership_limit)?;
                state.segment+=1;
            }
            return Ok(false)
        }
        let donor=self.input.as_mut().ok_or_else(refused)?;
        if state.segment==segments {
            if donor.len()!=state.extent{return Err(refused().into())}
            if !charge(cx,false,size_of::<Option<OpenEncoding>>()+size_of::<Phase>())?{return Ok(false)}
            self.open_encoding=None;
            self.phase=Phase::Begin;
            return Ok(true)
        }
        let segment=source.segment(state.segment,state.extent)?;
        let bytes=segment.bytes();
        if state.offset==bytes.len(){
            if charge(cx,false,2*size_of::<usize>())?{state.segment+=1;state.offset=0;}
            return Ok(false)
        }
        let grant=cx.retained_grant();
        let meta=size_of::<usize>();
        let count=(bytes.len()-state.offset).min(16_384).min(grant.maximum_copy_bytes.saturating_sub(meta))
            .min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}
        if donor.len().checked_add(count).is_none_or(|end|end>state.extent){return Err(refused().into())}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count+meta,..Default::default()})?;
        cx.consume_fuel(count as u64+1);
        donor.extend_from_slice(&bytes[state.offset..state.offset+count]);
        state.offset+=count;
        Ok(false)
    }

    pub(super) fn close_open_encoding(&mut self,cx:&mut StepContext<'_>) -> Result<bool,OriginalTurnFailure> {
        if self.open_encoding.is_none(){return Ok(true)}
        if !charge(cx,true,size_of::<Option<OpenEncoding>>())?{return Ok(false)}
        self.open_encoding=None;
        Ok(true)
    }
}

fn charge(cx:&mut StepContext<'_>,retiring:bool,bytes:usize)->Result<bool,OriginalTurnFailure>{
    let grant=cx.retained_grant();
    if (if retiring{cx.retirement_should_yield()}else{cx.should_yield()||cx.is_cancelled()})
        ||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
    cx.consume_fuel(1);
    Ok(true)
}

fn ownership_limit()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"compact InstanceOpen exceeds its Core extent")}
