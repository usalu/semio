//! 🌱️ A sole InstanceOpen fills the owned Poll slot without building an intermediate value tree.
use super::{ActorInput, OriginalTurnFailure, RetainedCloneProgress, StepContext, ValueError, ValueRefusalKind, charge, invalid};
use semio_framework::kernel::{ActorInstanceOpenRequest, AppInstanceId, BrokerCapabilityGrant, CapabilityId, CapabilityToken, Event, OwnedCorePollLimits, PollRequestLimits, QuotaSchema};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_abi::owned_abi::PollInput;
use semio_plugin_abi::original_open_frame::{self as frame, AssetHeader, CapabilityHeader, OpenHeader};
use std::mem::size_of;

#[derive(Clone, Copy)]
enum Phase {
    Header, Event, App, Actor, Config, Assets, AssetHeader, AssetName, AssetData,
    Capabilities, CapabilityHeader, CapabilityId, CapabilityScope, Complete,
}

pub(super) enum OpenRefusal {
    Wire(ValueError),
    Source(OriginalTurnFailure),
}

impl From<OriginalTurnFailure> for OpenRefusal {
    fn from(error: OriginalTurnFailure) -> Self { Self::Source(error) }
}

pub(super) struct OpenInput {
    phase: Phase,
    header: Option<OpenHeader>,
    cursor: usize,
    first: u32,
    second: u32,
    maximum_bytes: u64,
    maximum_owned_bytes: usize,
    owned_bytes: usize,
}

impl OpenInput {
    pub(super) fn new(maximum_bytes: u64, maximum_owned_bytes: usize) -> Self {
        Self { phase: Phase::Header, header: None, cursor: 0, first: 0, second: 0,
            maximum_bytes, maximum_owned_bytes, owned_bytes: 0 }
    }

    pub(super) fn advance(
        &mut self, wire: &[u8], out: &mut Option<ActorInput>, cx: &mut StepContext<'_>,
    ) -> Result<bool, OpenRefusal> {
        if cx.should_yield() || cx.is_cancelled() || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
        if matches!(self.phase, Phase::Header) {
            if !charge(cx, false, frame::HEADER_BYTES + size_of::<Option<OpenHeader>>() + size_of::<Phase>())? { return Ok(false); }
            if wire.len() as u64 > self.maximum_bytes {
                return Err(OpenRefusal::Wire(ValueError::literal(ValueRefusalKind::OwnershipLimit, "compact InstanceOpen exceeds its admitted extent")));
            }
            let bytes=frame::field(wire,0,frame::HEADER_BYTES).map_err(malformed)?;
            self.header=Some(OpenHeader::read(bytes,wire.len()).map_err(malformed)?);
            self.cursor=frame::HEADER_BYTES;
            self.phase=Phase::Event;
            return Ok(false)
        }
        let header=self.header.ok_or_else(invalid)?;
        if matches!(self.phase, Phase::Event) {
            let request=ActorInstanceOpenRequest{activation_generation:header.activation_generation,instance_id:header.instance_id,request_sequence:header.request_sequence};
            if !request.is_valid(){return Err(malformed(frame::InvalidFrame))}
            let [items,copy,capacity,release,depth,work_fuel,fuel,deadline_us,max_effects,max_patch_bytes,max_frames]=header.limits;
            let limits=OwnedCorePollLimits::try_from(PollRequestLimits{
                retained_policy:RetainedCloneGrant{maximum_items:items as usize,maximum_copy_bytes:copy as usize,maximum_capacity_bytes:capacity as usize,maximum_release_bytes:release as usize,maximum_depth:depth as usize},
                work_fuel,fuel,deadline_us,max_effects:max_effects as u32,max_patch_bytes:max_patch_bytes as u32,max_frames:max_frames as u32,
            }).map_err(OpenRefusal::Wire)?;
            let mut events=Vec::new();
            if !self.reserve(&mut events,1,size_of::<Option<ActorInput>>()+size_of::<Event>()+size_of::<Phase>(),cx)?{return Ok(false)}
            events.push(Event::InstanceOpen { request, app_id:AppInstanceId(String::new()), actor:String::new(),
                config:Vec::new(), assets:Vec::new(), capabilities:Vec::new(), quotas:quotas(header.quotas) });
            *out=Some(ActorInput::Poll(PollInput { events, command_page:None, cold_pair_page:None, limits }));
            self.phase=Phase::App;
            return Ok(false)
        }
        let Some(ActorInput::Poll(poll))=out.as_mut() else {return Err(invalid().into())};
        let Some(Event::InstanceOpen{app_id,actor,config,assets,capabilities,..})=poll.events.first_mut() else {return Err(invalid().into())};
        match self.phase {
            Phase::App => self.copy(wire,header.app_bytes,Target::Text(&mut app_id.0),Phase::Actor,cx)?,
            Phase::Actor => self.copy(wire,header.actor_bytes,Target::Text(actor),Phase::Config,cx)?,
            Phase::Config => self.copy(wire,header.config_bytes,Target::Bytes(config),Phase::Assets,cx)?,
            Phase::Assets => {
                if self.reserve(assets,header.assets as usize,size_of::<Vec<(String,Vec<u8>)>>()+size_of::<Phase>(),cx)?{self.phase=Phase::AssetHeader;}
            }
            Phase::AssetHeader if assets.len()==header.assets as usize => self.transition(Phase::Capabilities,cx)?,
            Phase::AssetHeader => {
                if !charge(cx,false,frame::ASSET_HEADER_BYTES+size_of::<(String,Vec<u8>)>()+size_of::<(usize,u32,u32,Phase)>())?{return Ok(false)}
                let row=AssetHeader::read(frame::field(wire,self.cursor,frame::ASSET_HEADER_BYTES).map_err(malformed)?).map_err(malformed)?;
                self.begin_row(wire,frame::ASSET_HEADER_BYTES,row.name_bytes,row.data_bytes,Phase::AssetName)?;
                assets.push((String::new(),Vec::new()));
            }
            Phase::AssetName => self.copy(wire,self.first,Target::Text(&mut assets.last_mut().ok_or_else(invalid)?.0),Phase::AssetData,cx)?,
            Phase::AssetData => self.copy(wire,self.second,Target::Bytes(&mut assets.last_mut().ok_or_else(invalid)?.1),Phase::AssetHeader,cx)?,
            Phase::Capabilities => {
                if self.reserve(capabilities,header.capabilities as usize,size_of::<Vec<BrokerCapabilityGrant>>()+size_of::<Phase>(),cx)?{self.phase=Phase::CapabilityHeader;}
            }
            Phase::CapabilityHeader if capabilities.len()==header.capabilities as usize => {
                if self.cursor!=wire.len(){return Err(malformed(frame::InvalidFrame))}
                self.transition(Phase::Complete,cx)?;
            }
            Phase::CapabilityHeader => {
                if !charge(cx,false,frame::CAPABILITY_HEADER_BYTES+size_of::<BrokerCapabilityGrant>()+size_of::<(usize,u32,u32,Phase)>())?{return Ok(false)}
                let row=CapabilityHeader::read(frame::field(wire,self.cursor,frame::CAPABILITY_HEADER_BYTES).map_err(malformed)?).map_err(malformed)?;
                self.begin_row(wire,frame::CAPABILITY_HEADER_BYTES,row.id_bytes,row.scope_bytes,Phase::CapabilityId)?;
                capabilities.push(BrokerCapabilityGrant{token:CapabilityToken(row.token),id:CapabilityId(String::new()),scope:String::new(),expires_ms:row.expires_ms});
            }
            Phase::CapabilityId => self.copy(wire,self.first,Target::Text(&mut capabilities.last_mut().ok_or_else(invalid)?.id.0),Phase::CapabilityScope,cx)?,
            Phase::CapabilityScope => self.copy(wire,self.second,Target::Text(&mut capabilities.last_mut().ok_or_else(invalid)?.scope),Phase::CapabilityHeader,cx)?,
            Phase::Complete => {},
            Phase::Header|Phase::Event => return Err(invalid().into()),
        }
        Ok(matches!(self.phase, Phase::Complete))
    }

    fn begin_row(&mut self, wire: &[u8], header: usize, first: u32, second: u32, phase: Phase) -> Result<(), OpenRefusal> {
        let base=self.cursor.checked_add(header).ok_or_else(||malformed(frame::InvalidFrame))?;
        let extent=(first as usize).checked_add(second as usize).ok_or_else(||malformed(frame::InvalidFrame))?;
        frame::field(wire,base,extent).map_err(malformed)?;
        self.cursor=base;
        self.first=first;
        self.second=second;
        self.phase=phase;
        Ok(())
    }

    fn transition(&mut self, phase: Phase, cx: &mut StepContext<'_>) -> Result<(), OpenRefusal> {
        if charge(cx,false,size_of::<Phase>())?{self.phase=phase;}
        Ok(())
    }

    fn allocation(&mut self, extent: usize, metadata: usize, cx: &mut StepContext<'_>) -> Result<bool, OpenRefusal> {
        if self.owned_bytes.checked_add(extent).is_none_or(|total|total>self.maximum_owned_bytes){
            return Err(OpenRefusal::Wire(ValueError::literal(ValueRefusalKind::OwnershipLimit, "compact InstanceOpen exceeds its retained capacity")));
        }
        if cx.retained_grant().maximum_capacity_bytes<extent{return Ok(false)}
        Ok(charge(cx,false,metadata)?)
    }

    fn retained(&mut self, bytes: usize, cx: &mut StepContext<'_>) -> Result<(), OpenRefusal> {
        self.owned_bytes=self.owned_bytes.checked_add(bytes).ok_or_else(invalid)?;
        cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:bytes,..Default::default()}).map_err(OriginalTurnFailure::from)?;
        Ok(())
    }

    fn reserve<T>(&mut self, target: &mut Vec<T>, count: usize, meta: usize, cx: &mut StepContext<'_>) -> Result<bool, OpenRefusal> {
        let extent=count.checked_mul(size_of::<T>()).ok_or_else(||malformed(frame::InvalidFrame))?;
        if !self.allocation(extent,meta,cx)?{return Ok(false)}
        target.try_reserve_exact(count).map_err(|_|allocation_failed())?;
        self.retained(target.capacity()*size_of::<T>(),cx)?;
        Ok(true)
    }

    fn copy(&mut self, wire: &[u8], extent: u32, mut target: Target<'_>, next: Phase, cx: &mut StepContext<'_>) -> Result<(), OpenRefusal> {
        let extent=extent as usize;
        let bytes=frame::field(wire,self.cursor,extent).map_err(malformed)?;
        let offset=target.len();
        if offset==extent {
            if charge(cx,false,size_of::<(usize,Phase)>())?{self.cursor+=extent;self.phase=next;}
            return Ok(())
        }
        if target.capacity()==0 {
            if self.allocation(extent,size_of::<Vec<u8>>(),cx)? {
                target.reserve(extent).map_err(|_|allocation_failed())?;
                self.retained(target.capacity(),cx)?;
            }
            return Ok(())
        }
        let grant=cx.retained_grant();
        let cost=if matches!(target,Target::Text(_)){2}else{1};
        let meta=size_of::<usize>();
        let count=(extent-offset).min(16_384).min(grant.maximum_copy_bytes.saturating_sub(meta))
            .min(usize::try_from(cx.fuel_remaining().saturating_sub(1)/cost).unwrap_or(usize::MAX));
        if grant.maximum_items==0||count==0{return Ok(())}
        let page=&bytes[offset..offset+count];
        cx.consume_fuel(count as u64*cost+1);
        let actual=match &mut target {
            Target::Text(text)=>{
                let value=match std::str::from_utf8(page){
                    Ok(value)=>value,
                    Err(error) if error.error_len().is_none()&&offset+count<extent=>
                        std::str::from_utf8(&page[..error.valid_up_to()]).map_err(|_|malformed(frame::InvalidFrame))?,
                    Err(_)=>return Err(malformed(frame::InvalidFrame)),
                };
                text.push_str(value);
                value.len()
            }
            Target::Bytes(bytes)=>{bytes.extend_from_slice(page);count}
        };
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:actual+meta,..Default::default()}).map_err(OriginalTurnFailure::from)?;
        Ok(())
    }
}

enum Target<'a>{Text(&'a mut String),Bytes(&'a mut Vec<u8>)}
impl Target<'_>{
    fn len(&self)->usize{match self{Self::Text(value)=>value.len(),Self::Bytes(value)=>value.len()}}
    fn capacity(&self)->usize{match self{Self::Text(value)=>value.capacity(),Self::Bytes(value)=>value.capacity()}}
    fn reserve(&mut self,extent:usize)->Result<(),std::collections::TryReserveError>{
        match self{Self::Text(value)=>value.try_reserve_exact(extent),Self::Bytes(value)=>value.try_reserve_exact(extent)}
    }
}

fn malformed(_:frame::InvalidFrame)->OpenRefusal{
    OpenRefusal::Wire(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid compact InstanceOpen input"))
}
fn allocation_failed()->OpenRefusal{
    OpenRefusal::Wire(ValueError::literal(ValueRefusalKind::AllocationFailed,"compact InstanceOpen allocation failed"))
}

fn quotas(slots:[Option<u64>;frame::QUOTA_COUNT])->QuotaSchema{
    let [memory_bytes,fuel_per_turn,turn_deadline_ms,tables,mailbox_len,message_bytes,outstanding_requests,timers,storage_bytes,network_bytes_per_min,ui_nodes,patch_bytes,patch_hz,blob_resident_bytes,gpu_ms_per_frame,background_ms_per_min,log_bytes_per_min]=slots;
    QuotaSchema{memory_bytes,fuel_per_turn,turn_deadline_ms,tables,mailbox_len,message_bytes,outstanding_requests,timers,storage_bytes,network_bytes_per_min,ui_nodes,patch_bytes,patch_hz,blob_resident_bytes,gpu_ms_per_frame,background_ms_per_min,log_bytes_per_min}
}
