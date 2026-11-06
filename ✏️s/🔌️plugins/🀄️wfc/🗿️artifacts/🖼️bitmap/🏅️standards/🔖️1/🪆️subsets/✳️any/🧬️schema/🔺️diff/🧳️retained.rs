//! 🧳️ Original-bound bitmap application stages bounded fragments before one authorized exchange.
use super::*;
use protocol::{RetainedDiff, RetainedDiffApply, RetainedDiffPreflightAdvance, RetainedFieldPreparation};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneRef, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedCloneClose}};
use std::mem::{size_of, ManuallyDrop};

impl RetainedDiff<BitmapSnapshot> for BitmapDiff { type Apply = BitmapApply; }

pub struct BitmapApply {
    snapshot: RetainedFieldPreparation<BitmapSnapshot>,
    schema: RetainedFieldPreparation<String>,
    candidate: ManuallyDrop<Option<BitmapSnapshot>>,
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

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "bitmap retained apply frontier is invalid") }
fn quote(copy_bytes: usize, capacity_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() } }
fn receipt(d: RetirementDemand) -> RetainedCloneProgress { RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() } }
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
    fn new() -> Self { Self { snapshot: Default::default(), schema: Default::default(), candidate: ManuallyDrop::new(None), old_schema: ManuallyDrop::new(None), pixels: ManuallyDrop::new(None), palette: ManuallyDrop::new(None), slots: ManuallyDrop::new(None), taken: ManuallyDrop::new(None), positions: ManuallyDrop::new(None), pins: ManuallyDrop::new(None), removed_colors: [false;256], recolored: [None;256], inserted_colors: [None;256], retirement: Default::default(), phase:0,index:0,survivor:0,write:0,width:0,height:0,output:BitmapOutputSpec::default(),palette_len:0,pin_len:0,pixels_dirty:false,palette_dirty:false,pins_dirty:false,valid:true,committed:false,closing:false } }
    fn preflight_demand(&self, d: RetainedCloneRef<'_,BitmapDiff>, _: RetainedCloneRef<'_,BitmapSnapshot>, post: RetainedCloneRef<'_,BitmapSnapshot>, body: usize) -> Result<RetirementDemand,ValueError> {
        if self.closing || self.committed { return Err(invalid()); }
        let mut q=match self.phase {
            0=>if self.snapshot.value().is_some() { self.snapshot.exchange_demand()? } else { self.snapshot.demand(post,body.saturating_sub(Self::fixed()))? },
            1 if d.get().schema.is_some()=>if self.schema.value().is_some() { self.schema.exchange_demand()? } else { self.schema.demand(d.project(0,|d|d.schema.as_ref().unwrap()),body.saturating_sub(Self::fixed()))? },
            3..=8=>quote(0,self.allocation()?),
            25=>quote(2*size_of::<BitmapSnapshot>(),0),
            26=>if !self.snapshot.terminal_is_empty(){self.snapshot.close_demand(body.saturating_sub(Self::fixed()))?}else if !self.schema.terminal_is_empty(){self.schema.close_demand(body.saturating_sub(Self::fixed()))?}else{quote(0,0)},
            _=>quote(0,0),
        };
        q.copy_bytes=q.copy_bytes.checked_add(Self::fixed()).ok_or_else(invalid)?;
        Ok(q)
    }
    fn preflight_work_demand(&self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:RetainedCloneRef<'_,BitmapSnapshot>,body:usize)->Result<u64,ValueError>{self.preflight_demand(d,original,post,body)?;let inner=body.saturating_sub(Self::fixed());match self.phase{0 if self.snapshot.value().is_none()=>self.snapshot.advance_work_demand(post,inner),1 if d.get().schema.is_some()&&self.schema.value().is_none()=>self.schema.advance_work_demand(d.project(0,|d|d.schema.as_ref().unwrap()),inner),26 if !self.snapshot.terminal_is_empty()=>self.snapshot.close_work_demand(inner),26 if !self.schema.terminal_is_empty()=>self.schema.close_work_demand(inner),27.. =>Ok(0),_=>Ok(1)}}
    fn preflight(&mut self, d: RetainedCloneRef<'_,BitmapDiff>, original: RetainedCloneRef<'_,BitmapSnapshot>, post: RetainedCloneRef<'_,BitmapSnapshot>, body:usize, grant:RetainedCloneGrant, _: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> semio_framework_value::RetainedWorkOutcome<RetainedDiffPreflightAdvance,semio_framework_value::OriginalRetirementFailure> {let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
        macro_rules! turn {($value:expr)=>{{let turn=$value;work=turn.work;turn.result.map_err(|error|{let error:semio_framework_value::OriginalRetirementFailure=error.into();if work==0{error}else{let progress=Self::add(error.retained_progress());error.with_retained_progress(progress)}})?}};}
        let q=self.preflight_demand(d,original,post,body)?;
        if !protocol::retained_mutation_demand_fits(q,grant) { return Ok(RetainedDiffPreflightAdvance::Progress(Default::default())); }
        let mut p=receipt(q);work=1;
        let diff=d.get();
        if !self.dirty_phase() {
            if self.phase==22 && diff.output.is_some() && self.index<self.candidate.as_ref().unwrap().pinned.len() {
                self.valid=self.position(self.candidate.as_ref().unwrap().pinned[self.index]).is_some();self.index+=1;
            } else { self.next(); }
            return Ok(if self.valid {RetainedDiffPreflightAdvance::Progress(p)} else {RetainedDiffPreflightAdvance::Ready(p,false)});
        }
        match self.phase {
            0=>{ if self.snapshot.value().is_some() { let (v,r)=self.snapshot.take(Self::child(grant))?; *self.candidate=v; p=Self::add(r); if self.candidate.is_some(){self.next();} } else { p=Self::add(turn!(self.snapshot.advance(post,body.saturating_sub(Self::fixed()),Self::child(grant))).progress()); } },
            1=>{ if diff.schema.is_some() { if self.schema.value().is_some() { let (v,r)=self.schema.take(Self::child(grant))?; p=Self::add(r); if let Some(v)=v { *self.old_schema=Some(std::mem::replace(&mut self.candidate.as_mut().unwrap().schema,v)); self.next(); } } else { p=Self::add(turn!(self.schema.advance(d.project(0,|d|d.schema.as_ref().unwrap()),body.saturating_sub(Self::fixed()),Self::child(grant))).progress()); } } else {self.next();} },
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
                if self.pixels_dirty{std::mem::swap(&mut candidate.input.pixels,self.pixels.as_mut().unwrap());}if self.palette_dirty{std::mem::swap(&mut candidate.input.palette,self.palette.as_mut().unwrap());}if self.pins_dirty{std::mem::swap(&mut candidate.pinned,self.pins.as_mut().unwrap());}self.snapshot.begin_close();self.schema.begin_close();self.next();
            },
            26=>{if !self.snapshot.terminal_is_empty(){p=Self::add(turn!(self.snapshot.close_step(Self::child(grant))).progress());}else if !self.schema.terminal_is_empty(){p=Self::add(turn!(self.schema.close_step(Self::child(grant))).progress());}else{self.next();}},
            _=>{work=0;return Ok(RetainedDiffPreflightAdvance::Ready(Default::default(),self.valid))},
        }
        if !p.fits(grant){return Err(invalid().with_retained_progress(p).into());}
        if !self.valid || self.phase==27 { Ok(RetainedDiffPreflightAdvance::Ready(p,self.valid)) } else {Ok(RetainedDiffPreflightAdvance::Progress(p))}
    })();semio_framework_value::RetainedWorkOutcome{result,work}}
    fn take_messages(&mut self)->Vec<protocol::MutationMessage>{Vec::new()}
    fn commit_demand(&self,_:RetainedCloneRef<'_,BitmapDiff>,_:RetainedCloneRef<'_,BitmapSnapshot>,_:&BitmapSnapshot,_:usize)->Result<RetirementDemand,ValueError>{if self.phase!=27 || !self.valid || self.committed || self.closing{return Err(invalid());}Ok(quote(3*size_of::<BitmapSnapshot>()+size_of::<bool>(),0))}
    fn commit_work_demand(&self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&BitmapSnapshot,body:usize)->Result<u64,ValueError>{self.commit_demand(d,original,post,body)?;Ok(1)}
    fn commit(&mut self,d:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&mut BitmapSnapshot,grant:RetainedCloneGrant,_:protocol::ApplyCapability)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{let q=self.commit_demand(d,original,post,0)?;if !protocol::retained_mutation_demand_fits(q,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}let candidate=self.candidate.as_mut().ok_or_else(invalid)?;work=1;std::mem::swap(post,candidate);self.committed=true;Ok(RetainedCloneStep::Complete(receipt(q)))})();semio_framework_value::RetainedWorkOutcome{result,work}}
    fn begin_close(&mut self){self.closing=true;self.snapshot.begin_close();self.schema.begin_close();}
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.snapshot.terminal_is_empty(){return self.snapshot.close_demand(body);}if !self.schema.terminal_is_empty(){return self.schema.close_demand(body);}
        if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<BitmapSnapshot>(false,&None,body);}
        macro_rules! owner{($field:ident,$t:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$t>(true,&None,body);}};}
        owner!(candidate,BitmapSnapshot);owner!(old_schema,String);owner!(pixels,Vec<u8>);owner!(palette,Vec<BitmapColor>);owner!(slots,Vec<Option<BitmapPinnedPixel>>);owner!(taken,Vec<bool>);owner!(positions,Vec<Option<usize>>);owner!(pins,Vec<BitmapPinnedPixel>);Ok(Default::default())
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demand(body)?;if !self.snapshot.terminal_is_empty(){return self.snapshot.close_work_demand(body)}if !self.schema.terminal_is_empty(){return self.schema.close_work_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<BitmapSnapshot>(false,&None,body)}macro_rules! owner{($field:ident,$t:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$t>(true,&None,body)}};}owner!(candidate,BitmapSnapshot);owner!(old_schema,String);owner!(pixels,Vec<u8>);owner!(palette,Vec<BitmapColor>);owner!(slots,Vec<Option<BitmapPinnedPixel>>);owner!(taken,Vec<bool>);owner!(positions,Vec<Option<usize>>);owner!(pins,Vec<BitmapPinnedPixel>);Ok(0)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
        if !self.closing{return Err(invalid().into());}let q=self.close_demand(grant.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(q,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.snapshot.terminal_is_empty(){let turn=self.snapshot.close_step(grant);work+=turn.work;return turn.result;}if !self.schema.terminal_is_empty(){let turn=self.schema.close_step(grant);work+=turn.work;return turn.result;}
        if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work+=turn.work;return turn.result;}
        macro_rules! close{($field:ident)=>{let turn=self.retirement.begin_granted(&mut self.$field,grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}};}
        close!(candidate);close!(old_schema);close!(pixels);close!(palette);close!(slots);close!(taken);close!(positions);close!(pins);Ok(RetainedCloneStep::Complete(Default::default()))
    })();semio_framework_value::RetainedWorkOutcome{result,work}}
    fn terminal_is_empty(&self)->bool{self.snapshot.terminal_is_empty()&&self.schema.terminal_is_empty()&&self.candidate.is_none()&&self.old_schema.is_none()&&self.pixels.is_none()&&self.palette.is_none()&&self.slots.is_none()&&self.taken.is_none()&&self.positions.is_none()&&self.pins.is_none()&&self.retirement.is_empty()}
}

impl Drop for BitmapApply {
    fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"bitmap retained apply requires paid cancellation closure");if empty{unsafe{ManuallyDrop::drop(&mut self.candidate);ManuallyDrop::drop(&mut self.old_schema);ManuallyDrop::drop(&mut self.pixels);ManuallyDrop::drop(&mut self.palette);ManuallyDrop::drop(&mut self.slots);ManuallyDrop::drop(&mut self.taken);ManuallyDrop::drop(&mut self.positions);ManuallyDrop::drop(&mut self.pins);}}}
}

#[cfg(test)]
#[path = "🧪️tests/retained.rs"]
mod tests;
