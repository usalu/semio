//! 🧳️ Original-bound bitmap application stages bounded fragments before one authorized exchange.
use super::*;
use protocol::{retained_native_child as native_child, retained_routed, RetainedDiff, RetainedDiffApply, RetainedDiffPreflightAdvance, RetainedDone as Done, RetainedFieldPreparation, RetainedFlight as Flight, RetainedReply as Reply, retained_conclude as conclude, retained_settle as settle};
use semio_framework_value::{NativeDecodeControl, OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedCloneClose, retained_clone_grant_admits}};
use std::mem::{size_of, ManuallyDrop};

impl RetainedDiff<BitmapSnapshot> for BitmapDiff { type Apply = BitmapApply; }

pub struct BitmapApply {
    snapshot: RetainedFieldPreparation<BitmapSnapshot>,
    schema: RetainedFieldPreparation<String>,
    candidate: ManuallyDrop<Option<BitmapSnapshot>>,
    new_schema: ManuallyDrop<Option<String>>,
    old_schema: ManuallyDrop<Option<String>>,
    pixels: ManuallyDrop<Option<Vec<u8>>>,
    palette: ManuallyDrop<Option<Vec<BitmapColor>>>,
    slots: ManuallyDrop<Option<Vec<Option<BitmapPinnedPixel>>>>,
    taken: ManuallyDrop<Option<Vec<bool>>>,
    positions: ManuallyDrop<Option<Vec<Option<usize>>>>,
    pins: ManuallyDrop<Option<Vec<BitmapPinnedPixel>>>,
    removed_colors: [bool; 256],
    recolored: [Option<BitmapColor>; 256],
    inserted_colors: [Option<BitmapColor>; 256],
    retirement: RetainedCloneClose,
    flight: Flight,
    phase: u8,
    index: usize,
    survivor: usize,
    write: usize,
    width: u32,
    height: u32,
    output: BitmapOutputSpec,
    palette_len: usize,
    pin_len: usize,
    pixels_dirty: bool,
    palette_dirty: bool,
    pins_dirty: bool,
    valid: bool,
    committed: bool,
    closing: bool,
}

/// 🏁️ The phase at which every field is prepared and the central writes may begin.
const PREPARED: u8 = 26;

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "bitmap retained apply frontier is invalid") }
fn quote(copy_bytes: usize, capacity_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() } }
fn receipt(d: RetirementDemand) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() } }
fn tagged(error: OriginalRetirementFailure, paid: bool) -> OriginalRetirementFailure { if !paid { return error; } let progress = BitmapApply::add(error.retained_progress()); error.with_retained_progress(progress) }
/// 📏️ Quotes a ready field exchange plus the resident slot it publishes into.
fn exchange<T: RetainedClone>(field: &RetainedFieldPreparation<T>) -> Result<RetirementDemand, ValueError> { let mut d = field.exchange_demand()?; d.copy_bytes = d.copy_bytes.checked_add(size_of::<Option<T>>()).ok_or_else(invalid)?; Ok(d) }
fn key_coordinates(key: &str) -> Option<(u32,u32)> {
    if key.len()>7 { return None; }
    let (x,y) = key.split_once(':')?;
    if x.is_empty() || y.is_empty() || x.len()>3 || y.len()>3 || !x.bytes().all(|b|b.is_ascii_digit()) || !y.bytes().all(|b|b.is_ascii_digit()) || (x.len()>1 && x.starts_with('0')) || (y.len()>1 && y.starts_with('0')) { return None; }
    Some((x.parse().ok()?,y.parse().ok()?))
}
fn key_matches(key: &str, pin: BitmapPinnedPixel) -> bool { key_coordinates(key)==Some((pin.x,pin.y)) }

impl BitmapApply {
    fn fixed() -> usize { 8*size_of::<usize>() + 2*size_of::<BitmapPinnedPixel>() + 2*size_of::<BitmapColor>() }
    fn position(&self, pin: BitmapPinnedPixel) -> Option<usize> { (pin.x<self.output.width && pin.y<self.output.height).then(|| pin.y as usize*self.output.width as usize+pin.x as usize) }
    fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant { RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes-Self::fixed(), ..grant } }
    fn add(mut p: RetainedCloneProgress) -> RetainedCloneProgress { p.copied_items=p.copied_items.max(1);p.copied_bytes+=Self::fixed(); p }
    fn next(&mut self) { self.phase+=1; self.index=0; }
    fn allocation(&self) -> Result<usize,ValueError> {
        if !self.dirty_phase() { return Ok(0); }
        let count=match self.phase { 3=>self.width as usize*self.height as usize, 4=>self.palette_len, 5|8=>self.pin_len, 6=>self.candidate.as_ref().ok_or_else(invalid)?.pinned.len(), 7=>self.output.width as usize*self.output.height as usize, _=>0 };
        count.checked_mul(match self.phase { 4=>size_of::<BitmapColor>(),5=>size_of::<Option<BitmapPinnedPixel>>(),7=>size_of::<Option<usize>>(),8=>size_of::<BitmapPinnedPixel>(),_=>1 }).ok_or_else(invalid)
    }
    fn dirty_phase(&self) -> bool { match self.phase {3|9|24=>self.pixels_dirty,4|10..=13=>self.palette_dirty,5..=8|14..=23=>self.pins_dirty,_=>true} }
}

impl RetainedDiffApply<BitmapSnapshot, BitmapDiff> for BitmapApply {
    fn new() -> Self { Self { snapshot: Default::default(), schema: Default::default(), candidate: ManuallyDrop::new(None), new_schema: ManuallyDrop::new(None), old_schema: ManuallyDrop::new(None), pixels: ManuallyDrop::new(None), palette: ManuallyDrop::new(None), slots: ManuallyDrop::new(None), taken: ManuallyDrop::new(None), positions: ManuallyDrop::new(None), pins: ManuallyDrop::new(None), removed_colors: [false;256], recolored: [None;256], inserted_colors: [None;256], retirement: Default::default(), flight: Default::default(), phase:0,index:0,survivor:0,write:0,width:0,height:0,output:BitmapOutputSpec::default(),palette_len:0,pin_len:0,pixels_dirty:false,palette_dirty:false,pins_dirty:false,valid:true,committed:false,closing:false } }
    fn preflight_demand(&self, d: RetainedCloneRef<'_,BitmapDiff>, _: RetainedCloneRef<'_,BitmapSnapshot>, post: RetainedCloneRef<'_,BitmapSnapshot>, body: usize, maximum_work_units: u64) -> Result<RetirementDemand,ValueError> {
        if self.closing || self.committed || self.phase>PREPARED { return Err(invalid()); }
        if maximum_work_units==0 || self.phase==PREPARED { return Ok(Default::default()); }
        let inner=body.saturating_sub(Self::fixed());
        let mut q=match self.phase {
            0=>if self.snapshot.value().is_some() { exchange(&self.snapshot)? } else { self.snapshot.demand(post,inner,maximum_work_units)? },
            1 if d.get().schema.is_some()=>if self.schema.value().is_some() { exchange(&self.schema)? } else { self.schema.demand(d.project(0,|d|d.schema.as_ref().unwrap()),inner,maximum_work_units)? },
            3..=8=>quote(0,self.allocation()?),
            25=>quote(2*size_of::<BitmapSnapshot>(),0),
            _=>quote(0,0),
        };
        q.copy_bytes=q.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;
        Ok(q)
    }
    fn preflight_work_demand(&self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:RetainedCloneRef<'_,BitmapSnapshot>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
        self.preflight_demand(d,original,post,body,maximum_work_units)?;
        let inner=body.saturating_sub(Self::fixed());
        match self.phase{
            _ if maximum_work_units==0 || self.phase==PREPARED=>Ok(0),
            0 if self.snapshot.value().is_none()=>self.snapshot.advance_work_demand(post,inner,maximum_work_units),
            1 if d.get().schema.is_some()&&self.schema.value().is_none()=>self.schema.advance_work_demand(d.project(0,|d|d.schema.as_ref().unwrap()),inner,maximum_work_units),
            _=>Ok(1),
        }
    }
    fn preflight_reply<'r>(&mut self, d: RetainedCloneRef<'_,BitmapDiff>, original: RetainedCloneRef<'_,BitmapSnapshot>, post: RetainedCloneRef<'_,BitmapSnapshot>, body:usize, maximum_work_units:u64, effect:&mut RetainedAllocationEffect<'_>, _: &mut NativeDecodeControl<'_>, reply: Reply<'r,RetainedDiffPreflightAdvance>, recover: bool) -> Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError> {
        let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
        let result:Result<RetainedDiffPreflightAdvance,RetirementPollExit>=(||{
        let diff=d.get();
        let inner=body.saturating_sub(Self::fixed());
        let route:u8=if recover{self.flight.route().ok_or_else(invalid)?}else{
            if !self.flight.idle(){return Err(invalid().into());}
            if self.phase==PREPARED{return Ok(RetainedDiffPreflightAdvance::Ready(Default::default(),self.valid));}
            let q=self.preflight_demand(d,original,post,body,maximum_work_units)?;
            if self.preflight_work_demand(d,original,post,body,maximum_work_units)?==0 || !protocol::retained_mutation_demand_fits(q,grant) { return Ok(RetainedDiffPreflightAdvance::Progress(Default::default())); }
            match self.phase { 0 if self.snapshot.value().is_none()=>1, 1 if diff.schema.is_some()&&self.schema.value().is_none()=>2, _=>0 }
        };
        let mut p=RetainedCloneProgress::default();
        if route!=0 {
            let before=work;
            let step=if route==1 {
                retained_routed!(self.flight,effect,recover,1,body,maximum_work_units,None,work,|r| self.snapshot.advance_into(post,inner,maximum_work_units,&mut effect.nested(Self::child(grant)),r),self.snapshot.recover_advance_into(post,inner,maximum_work_units,&mut effect.nested(Self::child(grant)),r))
            } else {
                retained_routed!(self.flight,effect,recover,2,body,maximum_work_units,None,work,|r| self.schema.advance_into(d.project(0,|d|d.schema.as_ref().unwrap()),inner,maximum_work_units,&mut effect.nested(Self::child(grant)),r),self.schema.recover_advance_into(d.project(0,|d|d.schema.as_ref().unwrap()),inner,maximum_work_units,&mut effect.nested(Self::child(grant)),r))
            };
            p=Self::add(step.map_err(|error|tagged(error,work!=before))?.progress());
        } else {
        let q=self.preflight_demand(d,original,post,body,maximum_work_units)?;
        p=receipt(q);work=1;
        if !self.dirty_phase() {
            if self.phase==22 && diff.output.is_some() && self.index<self.candidate.as_ref().unwrap().pinned.len() {
                self.valid=self.position(self.candidate.as_ref().unwrap().pinned[self.index]).is_some();self.index+=1;
            } else { self.next(); }
            return Ok(if self.valid {RetainedDiffPreflightAdvance::Progress(p)} else {RetainedDiffPreflightAdvance::Ready(p,false)});
        }
        match self.phase {
            0=>{ let slot=RetainedCloneGrant{maximum_copy_bytes:Self::child(grant).maximum_copy_bytes.saturating_sub(size_of::<Option<BitmapSnapshot>>()),..grant}; work=0; let step=native_child(effect,&mut work,|e,r| self.snapshot.take_into(&mut self.candidate,&mut e.nested(slot),r))??; p=Self::add(step.progress()); p.copied_bytes+=size_of::<Option<BitmapSnapshot>>(); if self.candidate.is_some(){self.next();} },
            1=>{ if diff.schema.is_some() { let slot=RetainedCloneGrant{maximum_copy_bytes:Self::child(grant).maximum_copy_bytes.saturating_sub(size_of::<Option<String>>()),..grant}; work=0; let step=native_child(effect,&mut work,|e,r| self.schema.take_into(&mut self.new_schema,&mut e.nested(slot),r))??; p=Self::add(step.progress()); p.copied_bytes+=size_of::<Option<String>>(); if let Some(v)=self.new_schema.take() { *self.old_schema=Some(std::mem::replace(&mut self.candidate.as_mut().unwrap().schema,v)); self.next(); } } else {self.next();} },
            2=>{
                let base=self.candidate.as_ref().unwrap();
                (self.width,self.height)=diff.input.size.map_or((base.input.width,base.input.height),|s|(s.width,s.height));
                self.output=diff.output.unwrap_or(base.output);
                self.pixels_dirty=diff.input.size.is_some() || !diff.input.writes.is_empty();self.palette_dirty=!diff.palette.is_empty();self.pins_dirty=!diff.pinned.is_empty();
                self.palette_len=base.input.palette.len().checked_add(diff.palette.inserted.len()).and_then(|n|n.checked_sub(diff.palette.removed.len())).unwrap_or(0);
                self.pin_len=base.pinned.len().checked_add(diff.pinned.inserted.len()).and_then(|n|n.checked_sub(diff.pinned.removed.len())).unwrap_or(usize::MAX);
                self.valid=self.width>0 && self.height>0 && self.width<=512 && self.height<=512 && self.output.width>0 && self.output.height>0 && self.output.width<=512 && self.output.height<=512 && base.input.width>0 && base.input.height>0 && base.input.width<=512 && base.input.height<=512 && base.input.pixels.len()==base.input.width as usize*base.input.height as usize && self.palette_len>0 && self.palette_len<=256 && base.input.palette.len()<=256 && self.pin_len<=self.output.width as usize*self.output.height as usize;
                self.next();
            },
            3=>{let v=Vec::with_capacity(self.width as usize*self.height as usize);p.retained_capacity_bytes=v.capacity();*self.pixels=Some(v);self.next();},
            4=>{let v=Vec::with_capacity(self.palette_len);p.retained_capacity_bytes=v.capacity()*size_of::<BitmapColor>();*self.palette=Some(v);self.next();},
            5=>{let v=Vec::with_capacity(self.pin_len);p.retained_capacity_bytes=v.capacity()*size_of::<Option<BitmapPinnedPixel>>();*self.slots=Some(v);self.next();},
            6=>{let v=Vec::with_capacity(self.candidate.as_ref().unwrap().pinned.len());p.retained_capacity_bytes=v.capacity();*self.taken=Some(v);self.next();},
            7=>{let v=Vec::with_capacity(self.output.width as usize*self.output.height as usize);p.retained_capacity_bytes=v.capacity()*size_of::<Option<usize>>();*self.positions=Some(v);self.next();},
            8=>{let v=Vec::with_capacity(self.pin_len);p.retained_capacity_bytes=v.capacity()*size_of::<BitmapPinnedPixel>();*self.pins=Some(v);self.next();},
            9=>{
                if self.index<self.width as usize*self.height as usize { let b=&self.candidate.as_ref().unwrap().input;let x=self.index%self.width as usize;let y=self.index/self.width as usize;let value=if x<b.width as usize && y<b.height as usize {b.pixels[y*b.width as usize+x]}else{0};self.pixels.as_mut().unwrap().push(value);self.index+=1; } else {self.next();}
            },
            10=>{if let Some(i)=diff.palette.removed.get(self.index){let i=*i as usize;if i>=self.candidate.as_ref().unwrap().input.palette.len() || self.removed_colors[i]{self.valid=false;}else{self.removed_colors[i]=true;}self.index+=1;}else{self.next();}},
            11=>{if let Some(row)=diff.palette.recolored.get(self.index){let i=row.index as usize;if i>=self.candidate.as_ref().unwrap().input.palette.len() || self.removed_colors[i] || self.recolored[i].is_some(){self.valid=false;}else{self.recolored[i]=Some(row.color);}self.index+=1;}else{self.next();}},
            12=>{if let Some(row)=diff.palette.inserted.get(self.index){let i=row.index as usize;if i>=self.palette_len || self.inserted_colors[i].is_some(){self.valid=false;}else{self.inserted_colors[i]=Some(row.color);}self.index+=1;}else{self.next();}},
            13=>{if self.index==self.palette_len{self.next();self.survivor=0;}else if let Some(color)=self.inserted_colors[self.index]{self.palette.as_mut().unwrap().push(color);self.index+=1;}else if self.survivor<self.candidate.as_ref().unwrap().input.palette.len(){let i=self.survivor;self.survivor+=1;if !self.removed_colors[i]{let c=self.recolored[i].unwrap_or(self.candidate.as_ref().unwrap().input.palette[i]);self.palette.as_mut().unwrap().push(c);self.index+=1;}}else{self.valid=false;}},
            14=>{if self.index<self.pin_len{self.slots.as_mut().unwrap().push(None);self.index+=1;}else{self.next();}},
            15=>{if self.index<self.candidate.as_ref().unwrap().pinned.len(){self.taken.as_mut().unwrap().push(false);self.index+=1;}else{self.next();}},
            16=>{if self.index<self.output.width as usize*self.output.height as usize{self.positions.as_mut().unwrap().push(None);self.index+=1;}else{self.next();}},
            17=>{if let Some(row)=diff.pinned.removed.get(self.index){if self.candidate.as_ref().unwrap().pinned.get(row.index).is_none_or(|pin|!key_matches(&row.id,*pin)) || self.taken.as_ref().unwrap().get(row.index).copied().unwrap_or(true){self.valid=false;}else{self.taken.as_mut().unwrap()[row.index]=true;}self.index+=1;}else{self.next();}},
            18=>{if let Some(row)=diff.pinned.moved.get(self.index){if self.candidate.as_ref().unwrap().pinned.get(row.from).is_none_or(|pin|!key_matches(&row.id,*pin)) || self.taken.as_ref().unwrap().get(row.from).copied().unwrap_or(true){self.valid=false;}else{self.taken.as_mut().unwrap()[row.from]=true;}self.index+=1;}else{self.next();}},
            19=>{if let Some(row)=diff.pinned.inserted.get(self.index){if let Some(slot)=self.slots.as_mut().unwrap().get_mut(row.index){if slot.is_some(){self.valid=false;}else{*slot=Some(row.row);}}else{self.valid=false;}self.index+=1;}else{self.next();}},
            20=>{if let Some(row)=diff.pinned.moved.get(self.index){if let Some(slot)=self.slots.as_mut().unwrap().get_mut(row.to){if slot.is_some(){self.valid=false;}else{*slot=Some(self.candidate.as_ref().unwrap().pinned[row.from]);}}else{self.valid=false;}self.index+=1;}else{self.next();}},
            21=>{if self.index==self.pin_len{self.next();}else if self.slots.as_ref().unwrap()[self.index].is_some(){self.index+=1;}else if self.survivor<self.candidate.as_ref().unwrap().pinned.len(){let i=self.survivor;self.survivor+=1;if !self.taken.as_ref().unwrap()[i]{self.slots.as_mut().unwrap()[self.index]=Some(self.candidate.as_ref().unwrap().pinned[i]);self.index+=1;}}else{self.valid=false;}},
            22=>{if self.index==self.pin_len{self.next();}else if let Some(pin)=self.slots.as_ref().unwrap()[self.index]{if let Some(at)=self.position(pin){if self.positions.as_ref().unwrap()[at].is_some(){self.valid=false;}else{self.positions.as_mut().unwrap()[at]=Some(self.index);self.pins.as_mut().unwrap().push(pin);}}else{self.valid=false;}self.index+=1;}else{self.valid=false;}},
            23=>{if let Some(row)=diff.pinned.modified.get(self.index){let at=key_coordinates(&row.id).and_then(|(x,y)|self.position(BitmapPinnedPixel{x,y,color:0})).and_then(|i|self.positions.as_ref().unwrap()[i]);if let Some(at)=at{if let Some(color)=row.patch.color{self.pins.as_mut().unwrap()[at].color=color;}}else{self.valid=false;}self.index+=1;}else{self.next();}},
            24=>{
                if let Some(write)=diff.input.writes.get(self.write){match write{
                    BitmapInputWrite::Region{region}=>{if region.width==0 || region.height==0 || region.x.checked_add(region.width).is_none_or(|n|n>self.width) || region.y.checked_add(region.height).is_none_or(|n|n>self.height) || region.pixels.len()!=region.width as usize*region.height as usize{self.valid=false;}else if self.index<region.pixels.len(){let x=self.index%region.width as usize+region.x as usize;let y=self.index/region.width as usize+region.y as usize;self.pixels.as_mut().unwrap()[y*self.width as usize+x]=region.pixels[self.index];self.index+=1;}else{self.index=0;self.write+=1;}},
                    BitmapInputWrite::Cells{cells}=>{if let Some(cell)=cells.get(self.index){if cell.x>=self.width || cell.y>=self.height || cell.value>255{self.valid=false;}else{self.pixels.as_mut().unwrap()[cell.y as usize*self.width as usize+cell.x as usize]=cell.value as u8;}self.index+=1;}else{self.index=0;self.write+=1;}},
                }}else{self.next();}
            },
            25=>{
                let candidate=self.candidate.as_mut().unwrap();
                candidate.seed=diff.seed.unwrap_or(candidate.seed);candidate.output=self.output;candidate.model=diff.model.unwrap_or(candidate.model);candidate.input.width=self.width;candidate.input.height=self.height;
                if self.pixels_dirty{std::mem::swap(&mut candidate.input.pixels,self.pixels.as_mut().unwrap());}if self.palette_dirty{std::mem::swap(&mut candidate.input.palette,self.palette.as_mut().unwrap());}if self.pins_dirty{std::mem::swap(&mut candidate.pinned,self.pins.as_mut().unwrap());}self.next();
            },
            _=>return Err(invalid().into()),
        }
        }
        if !p.fits(grant){return Err(invalid().with_retained_progress(p).into());}
        Ok(if !self.valid || self.phase==PREPARED { RetainedDiffPreflightAdvance::Ready(p,self.valid) } else { RetainedDiffPreflightAdvance::Progress(p) })
        })();
        conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
    }
    fn take_messages(&mut self)->Vec<protocol::MutationMessage>{Vec::new()}
    fn commit_demand(&self,_:RetainedCloneRef<'_,BitmapDiff>,_:RetainedCloneRef<'_,BitmapSnapshot>,_:&BitmapSnapshot,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=PREPARED || !self.valid || self.committed || self.closing{return Err(invalid());}Ok(quote(3*size_of::<BitmapSnapshot>()+size_of::<bool>(),0))}
    fn commit_work_demand(&self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&BitmapSnapshot,body:usize)->Result<u64,ValueError>{self.commit_demand(d,original,post,body)?;Ok(1)}
    fn commit(&mut self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&mut BitmapSnapshot,effect:&mut RetainedAllocationEffect<'_>,_:protocol::ApplyCapability)->OriginalRetirementOutcome{
        let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
        let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{let q=self.commit_demand(d,original,post,0)?;if !retained_clone_grant_admits(grant,q){return Ok(RetainedCloneStep::Progress(Default::default()));}let candidate=self.candidate.as_mut().ok_or_else(invalid)?;work=1;std::mem::swap(post,candidate);self.committed=true;Ok(RetainedCloneStep::Complete(receipt(q)))})();
        settle(effect,base,result,work)
    }
    fn begin_close(&mut self){self.closing=true;}
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.snapshot.terminal_is_empty(){return if self.snapshot.close_started(){self.snapshot.close_demand(body)}else{Ok(self.snapshot.begin_close_demand())};}
        if !self.schema.terminal_is_empty(){return if self.schema.close_started(){self.schema.close_demand(body)}else{Ok(self.schema.begin_close_demand())};}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<BitmapSnapshot>(false,&None,body);}
        macro_rules! owner{($field:ident,$t:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$t>(true,&None,body);}};}
        owner!(candidate,BitmapSnapshot);owner!(new_schema,String);owner!(old_schema,String);owner!(pixels,Vec<u8>);owner!(palette,Vec<BitmapColor>);owner!(slots,Vec<Option<BitmapPinnedPixel>>);owner!(taken,Vec<bool>);owner!(positions,Vec<Option<usize>>);owner!(pins,Vec<BitmapPinnedPixel>);Ok(Default::default())
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        self.close_demand(body)?;
        if !self.snapshot.terminal_is_empty(){return if self.snapshot.close_started(){self.snapshot.close_work_demand(body)}else{Ok(self.snapshot.begin_close_work_demand())};}
        if !self.schema.terminal_is_empty(){return if self.schema.close_started(){self.schema.close_work_demand(body)}else{Ok(self.schema.begin_close_work_demand())};}
        if !self.retirement.is_empty(){return self.retirement.next_owner_work::<BitmapSnapshot>(false,&None,body);}
        macro_rules! owner{($field:ident,$t:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$t>(true,&None,body);}};}
        owner!(candidate,BitmapSnapshot);owner!(new_schema,String);owner!(old_schema,String);owner!(pixels,Vec<u8>);owner!(palette,Vec<BitmapColor>);owner!(slots,Vec<Option<BitmapPinnedPixel>>);owner!(taken,Vec<bool>);owner!(positions,Vec<Option<usize>>);owner!(pins,Vec<BitmapPinnedPixel>);Ok(0)
    }
    fn requires_original_birth(&self)->bool{
        if !self.flight.idle(){return false;}
        if !self.snapshot.terminal_is_empty(){return self.snapshot.close_started()&&self.snapshot.requires_original_birth();}
        if !self.schema.terminal_is_empty(){return self.schema.close_started()&&self.schema.requires_original_birth();}
        !self.retirement.is_empty()&&self.retirement.requires_original_birth()
    }
    fn requires_original_control(&self)->bool{
        if let Some(loaned)=self.flight.loaned(){return loaned;}
        if !self.snapshot.terminal_is_empty(){return self.snapshot.close_started()&&self.snapshot.requires_original_control();}
        if !self.schema.terminal_is_empty(){return self.schema.close_started()&&self.schema.requires_original_control();}
        !self.retirement.is_empty()&&self.retirement.requires_original_control()
    }
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        if !self.snapshot.terminal_is_empty(){return self.snapshot.close_original_birth_step(original,grant);}
        if !self.schema.terminal_is_empty(){return self.schema.close_original_birth_step(original,grant);}
        self.retirement.step_original_birth(original,grant)
    }
    fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
        let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            let route:u8;let loaned:bool;
            if recover{route=self.flight.route().ok_or_else(invalid)?;loaned=self.flight.loaned().unwrap_or(false);}else{
                if !self.flight.idle()||!self.closing{return Err(invalid().into());}
                if !protocol::retained_mutation_demand_fits(self.close_demand(body)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
                if self.requires_original_birth(){return Err(invalid().into());}
                loaned=self.requires_original_control();
                if loaned&&original.is_none(){return Err(invalid().into());}
                if !self.snapshot.terminal_is_empty(){
                    if !self.snapshot.close_started(){let turn=self.snapshot.begin_close_paid(effect);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}
                    route=1;
                }else if !self.schema.terminal_is_empty(){
                    if !self.schema.close_started(){let turn=self.schema.begin_close_paid(effect);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?));}
                    route=2;
                }else if !self.retirement.is_empty(){route=3;}else{
                    macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.map(|step|step.progress()).unwrap_or_default()));}};}
                    close!(candidate);close!(new_schema);close!(old_schema);close!(pixels);close!(palette);close!(slots);close!(taken);close!(positions);close!(pins);
                    return Ok(RetainedCloneStep::Complete(Default::default()));
                }
            }
            let loan=if loaned{original.as_deref_mut()}else{None};
            let missing=||ValueError::invariant("bitmap close lost its original controller");
            let step=retained_routed!(self.flight,effect,recover,route,body,0,loan,work,
                |r| match(route,loaned){
                    (1,false)=>self.snapshot.close_into(body,effect,r),(1,true)=>self.snapshot.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    (2,false)=>self.schema.close_into(body,effect,r),(2,true)=>self.schema.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    (3,false)=>self.retirement.step_into(body,effect,r),(3,true)=>self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    _=>Err(invalid()),
                },
                match(route,loaned){
                    (1,false)=>self.snapshot.recover_close_into(body,effect,r),(1,true)=>self.snapshot.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    (2,false)=>self.schema.recover_close_into(body,effect,r),(2,true)=>self.schema.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    (3,false)=>self.retirement.recover_step_into(body,effect,r),(3,true)=>self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r),
                    _=>Err(invalid()),
                });
            Ok(RetainedCloneStep::Progress(step?.progress()))
        })();
        conclude(effect,base,result,work,|step|step.progress(),reply)
    }
    fn terminal_is_empty(&self)->bool{self.snapshot.terminal_is_empty()&&self.schema.terminal_is_empty()&&self.candidate.is_none()&&self.new_schema.is_none()&&self.old_schema.is_none()&&self.pixels.is_none()&&self.palette.is_none()&&self.slots.is_none()&&self.taken.is_none()&&self.positions.is_none()&&self.pins.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}

impl Drop for BitmapApply {
    fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"bitmap retained apply requires paid cancellation closure");if empty{unsafe{ManuallyDrop::drop(&mut self.candidate);ManuallyDrop::drop(&mut self.new_schema);ManuallyDrop::drop(&mut self.old_schema);ManuallyDrop::drop(&mut self.pixels);ManuallyDrop::drop(&mut self.palette);ManuallyDrop::drop(&mut self.slots);ManuallyDrop::drop(&mut self.taken);ManuallyDrop::drop(&mut self.positions);ManuallyDrop::drop(&mut self.pins);}}}
}

#[cfg(test)]
#[path = "🧪️tests/retained.rs"]
mod tests;
