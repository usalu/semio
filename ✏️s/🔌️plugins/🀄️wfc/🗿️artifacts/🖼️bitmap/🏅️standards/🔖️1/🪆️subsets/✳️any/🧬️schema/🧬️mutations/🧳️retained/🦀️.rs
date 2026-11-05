//! 🧳️ Bitmap decisions and exact inverses advance one authored row at a time.
use crate::{BitmapMutation as M, diff::*};
use crate::schema::snapshot::*;
use crate::standards::v1::subsets::any::schema::mutations as mutations;
use protocol::{RetainedMutation, RetainedMutationPreparation, RetainedMutationAdvance, RetainedMutationDecision, RetainedMutationCloseContext};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{ManuallyDrop,size_of};

fn demand(copy_bytes:usize,capacity_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,capacity_bytes,depth:1,..Default::default()}}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{protocol::retained_mutation_demand_fits(d,g)}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Invalid bitmap mutation source or payload")}
fn cells(p:&BitmapSnapshot)->usize{p.input.width as usize*p.input.height as usize}
fn pins(m:&M)->Option<&[BitmapPinnedPixel]>{match m{M::PinSolution(p)=>Some(&p.pins),M::RestorePins(p)=>Some(&p.pins),_=>None}}
fn palette_boundary(m:&M)->Option<(u32,bool)>{match m{M::AddPaletteColor(p)=>Some((p.index as u32,true)),M::RemovePaletteColor(p)=>Some((p.index as u32,false)),_=>None}}
fn region(m:&M)->Option<(u32,u32,u32,u32)>{match m{M::SetInputPixels(p)=>Some((p.x,p.y,p.width,p.height)),_=>None}}
fn pin_target(m:&M)->Option<(u32,u32)>{match m{M::PinPixel(p)=>Some((p.x,p.y)),M::UnpinPixel(p)=>Some((p.x,p.y)),_=>None}}

pub struct Preparation{
 diff:ManuallyDrop<Option<BitmapDiff>>,discarded:ManuallyDrop<Option<BitmapDiff>>,inverse:ManuallyDrop<Option<Vec<M>>>,
 bytes:ManuallyDrop<Option<Vec<u8>>>,prior:ManuallyDrop<Option<Vec<u8>>>,mask:ManuallyDrop<Option<Vec<u8>>>,
 saved:ManuallyDrop<Option<Vec<BitmapPinnedPixel>>>,cell_rows:ManuallyDrop<Option<Vec<BitmapPixelCell>>>,
 messages:ManuallyDrop<Option<Vec<protocol::MutationMessage>>>,ledger:Option<protocol::MutationMessageLedgerRetirement>,inverse_close:Option<FactoryVectorRetirement<M>>,retirement:RetainedCloneClose,
 phase:u8,index:usize,found:Option<usize>,insert_at:usize,same:bool,partial:bool,
 extent:(u32,u32,u32,u32),segment:usize,line:Option<(i64,i64,i64,i64,i64,i64,i64,i64,i64)>,
 inverse_phase:u8,inverse_index:usize,strip:u8,closing:bool,refused:bool,noop:bool,carried_capacity:usize,key_capacity:usize,
}
impl RetainedMutation<BitmapSnapshot> for M{type Preparation=Preparation;}
impl Preparation{
 fn fixed()->usize{size_of::<BitmapDiff>()+size_of::<M>()*2+size_of::<Self>()}
 fn counts(&self,m:&M,p:&BitmapSnapshot)->[usize;9]{
  let batch=pins(m).map_or(0,|v|v.len());
  let remove=if batch>0||matches!(m,M::PinSolution(_)|M::RestorePins(_)|M::ResizeOutput(_)){p.pinned.len()}else{usize::from(matches!(m,M::UnpinPixel(_)))};
  let insert=if pins(m).is_some(){batch}else{usize::from(matches!(m,M::PinPixel(_)))};
  let modified=if palette_boundary(m).is_some(){p.pinned.len()}else{usize::from(matches!(m,M::PinPixel(_)))};
  let region=region(m).map_or_else(||if matches!(m,M::PaintInputStroke(_)){cells(p)}else{0},|(_,_,w,h)|w as usize*h as usize);
  [remove,insert,modified,if palette_boundary(m).is_some(){cells(p)}else{0},region,if matches!(m,M::RestorePins(v)if v.pins.len()>1){p.output.width as usize*p.output.height as usize}else if matches!(m,M::PaintInputStroke(_)){cells(p)}else{0},usize::from(region>0),usize::from(palette_boundary(m).is_some()||matches!(m,M::ChangePaletteColor(_))),usize::from(region>0||palette_boundary(m).is_some())]
 }
 fn reject(&mut self,message:protocol::MutationMessage)->ValueError{let mut rows=Vec::with_capacity(1);rows.push(message);*self.messages=Some(rows);refusal()}
 fn validate(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  use protocol::MutationMessage as Message;
  let color=match m{M::AddPaletteColor(v)=>Some(v.color),M::ChangePaletteColor(v)=>Some(v.color),_=>None};
  if color.is_some_and(|c|[c.r,c.g,c.b,c.a].into_iter().any(|v|v>255)){return Err(self.reject(Message::fatal("mutation.invariant","Palette colour channels must be between 0 and 255.").at(["palette"])));}
  match m{
   M::ResizeInput(v)=>{
    if v.width==0||v.height==0{return Err(self.reject(Message::fatal("mutation.invariant","An input bitmap may not have a zero edge.").at(["input"])));}
    if v.width>BITMAP_MAX_EDGE||v.height>BITMAP_MAX_EDGE{return Err(self.reject(Message::fatal("mutation.invariant",format!("An input edge may not exceed {BITMAP_MAX_EDGE}.")).at(["input"])));}
   },
   M::ResizeOutput(v)=>{
    if v.width==0||v.height==0{return Err(self.reject(Message::fatal("mutation.invariant","An output bitmap may not have a zero edge.").at(["output"])));}
    if v.width>BITMAP_MAX_EDGE||v.height>BITMAP_MAX_EDGE{return Err(self.reject(Message::fatal("mutation.invariant",format!("An output edge may not exceed {BITMAP_MAX_EDGE}.")).at(["output"])));}
   },
   M::SetInputPixels(v)=>{
    if v.width==0||v.height==0||(v.width as usize).checked_mul(v.height as usize)!=Some(v.pixels.len()){return Err(self.reject(Message::fatal("mutation.invariant",format!("The region payload holds {} bytes, not {}×{}.",v.pixels.len(),v.width,v.height)).at(["pixels"])));}
    if v.x.saturating_add(v.width)>p.input.width||v.y.saturating_add(v.height)>p.input.height{return Err(self.reject(Message::fatal("mutation.invariant",format!("The region falls outside the {}×{} input bitmap.",p.input.width,p.input.height)).at(["pixels"])));}
   },
   M::AddPaletteColor(v)=>{
    if v.index>p.input.palette.len(){return Err(self.reject(Message::fatal("mutation.invariant",format!("Palette index {} is past the end of a {}-entry palette.",v.index,p.input.palette.len())).at(["palette"])));}
    if p.input.palette.len()>=BITMAP_MAX_PALETTE{return Err(self.reject(Message::fatal("mutation.invariant",format!("A palette may hold at most {BITMAP_MAX_PALETTE} colours.")).at(["palette"])));}
   },
   M::RemovePaletteColor(v)=>{
    if v.index>=p.input.palette.len(){return Err(self.reject(Message::error("mutation.target-missing",format!("The palette has no colour at index {}.",v.index)).at([v.index.to_string()])));}
    if p.input.palette.len()==1{return Err(self.reject(Message::fatal("mutation.invariant","A bitmap keeps at least one palette colour.").at([v.index.to_string()])));}
   },
   M::ChangePaletteColor(v)=>if v.index>=p.input.palette.len(){return Err(self.reject(Message::error("mutation.target-missing",format!("The palette has no colour at index {}.",v.index)).at(["palette"])));},
   M::ChangeModel(v)=>{
    if v.pattern_size<BITMAP_PATTERN_SIZE_RANGE.0||v.pattern_size>BITMAP_PATTERN_SIZE_RANGE.1{return Err(self.reject(Message::fatal("mutation.invariant",format!("The pattern size must be between {} and {}.",BITMAP_PATTERN_SIZE_RANGE.0,BITMAP_PATTERN_SIZE_RANGE.1)).at(["model"])));}
    if v.symmetry<BITMAP_SYMMETRY_RANGE.0||v.symmetry>BITMAP_SYMMETRY_RANGE.1{return Err(self.reject(Message::fatal("mutation.invariant",format!("The symmetry count must be between {} and {}.",BITMAP_SYMMETRY_RANGE.0,BITMAP_SYMMETRY_RANGE.1)).at(["model"])));}
    if let Some(ground)=v.ground{if ground as usize>=p.input.palette.len(){return Err(self.reject(Message::error("mutation.target-missing",format!("Ground colour {ground} is not in this document's palette.")).at(["model"])));}}
   },
   M::PinPixel(v)=>{
    if v.x>=p.output.width||v.y>=p.output.height{return Err(self.reject(Message::fatal("mutation.invariant",format!("({}, {}) falls outside the {}×{} output.",v.x,v.y,p.output.width,p.output.height)).at([pin_key(v.x,v.y)])));}
    if v.color as usize>=p.input.palette.len(){return Err(self.reject(Message::error("mutation.target-missing",format!("Palette index {} is not in this document's palette.",v.color)).at([pin_key(v.x,v.y)])));}
   },
   M::PaintInputStroke(v)=>{
    if v.points.is_empty()||v.points.len()>mutations::paint_input_stroke::BITMAP_STROKE_MAXIMUM_POINTS{return Err(self.reject(Message::fatal("mutation.invariant",format!("A stroke carries 1 to {} points, not {}.",mutations::paint_input_stroke::BITMAP_STROKE_MAXIMUM_POINTS,v.points.len())).at(["points"])));}
    if v.color as usize>=p.input.palette.len(){return Err(self.reject(Message::error("mutation.target-missing",format!("Palette index {} is not in this document's palette.",v.color)).at(["color"])));}
   },
   M::PinSolution(v)=>if v.pins.len()!=p.output.width as usize*p.output.height as usize{return Err(self.reject(Message::fatal("mutation.invariant","A pinned solution must cover every output cell.").at(["pinned"])));},
   _=>{}
  }
  if (matches!(m,M::AddPaletteColor(_)|M::PaintInputStroke(_))||matches!(m,M::ResizeInput(v)if(v.width,v.height)!=(p.input.width,p.input.height)))&&p.input.pixels.len()!=cells(p){return Err(self.reject(Message::fatal("mutation.apply.invalid-base","The base input pixel buffer does not decode.").at(["input"])));}
  self.same=true;self.insert_at=p.pinned.len();self.extent=region(m).unwrap_or((p.input.width,p.input.height,0,0));
  let d=self.diff.as_mut().unwrap();match m{
   M::ChangeSeed(v)=>if v.seed!=p.seed{d.seed=Some(v.seed)},
   M::ResizeInput(v)=>if(v.width,v.height)!=(p.input.width,p.input.height){d.input.size=Some(BitmapSize{width:v.width,height:v.height})},
   M::ResizeOutput(v)=>{let output=BitmapOutputSpec{width:v.width,height:v.height,periodic:v.periodic};if output!=p.output{d.output=Some(output)}},
   M::ChangeModel(v)=>{let model=BitmapOverlappingModel{pattern_size:v.pattern_size,symmetry:v.symmetry,periodic_input:v.periodic_input,ground:v.ground};if model!=p.model{d.model=Some(model)}},
   _=>{}
  }Ok(())
 }
 fn reserve_demand(&self,m:&M,p:&BitmapSnapshot)->RetirementDemand{
  let n=self.counts(m,p);let bytes=match self.phase{1=>n[0]*size_of::<BitmapPinnedRemoval>(),2=>n[1]*size_of::<BitmapPinnedInsertion>(),3=>n[2]*size_of::<BitmapPinnedModification>(),4=>n[3]*size_of::<BitmapPixelCell>(),5|6=>n[4],7=>n[5],8=>0,9=>n[7]*size_of::<BitmapPaletteInsertion>()+n[7]*size_of::<BitmapPaletteRecolor>()+n[7]*size_of::<u32>(),10=>n[8]*size_of::<BitmapInputWrite>(),_=>0};demand(Self::fixed(),bytes)
 }
 fn reserve(&mut self,m:&M,p:&BitmapSnapshot){
  let n=self.counts(m,p);let d=self.diff.as_mut().unwrap();match self.phase{
   1=>d.pinned.removed=Vec::with_capacity(n[0]),2=>d.pinned.inserted=Vec::with_capacity(n[1]),3=>d.pinned.modified=Vec::with_capacity(n[2]),
   4=>*self.cell_rows=Some(Vec::with_capacity(n[3])),5=>*self.bytes=Some(Vec::with_capacity(n[4])),6=>*self.prior=Some(Vec::with_capacity(n[4])),
   7=>*self.mask=Some(Vec::with_capacity(n[5])),8=>{},9=>{d.palette.removed=Vec::with_capacity(n[7]);d.palette.inserted=Vec::with_capacity(n[7]);d.palette.recolored=Vec::with_capacity(n[7]);match m{
    M::AddPaletteColor(v)=>d.palette.inserted.push(BitmapPaletteInsertion{index:v.index as u32,color:v.color}),
    M::RemovePaletteColor(v)=>d.palette.removed.push(v.index as u32),
    M::ChangePaletteColor(v)=>if p.input.palette[v.index]!=v.color{d.palette.recolored.push(BitmapPaletteRecolor{index:v.index as u32,color:v.color})},_=>{}}},
   10=>d.input.writes=Vec::with_capacity(n[8]),_=>{}
  }self.phase+=1;
 }
 fn scan_pin(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  if pins(m).is_none()&&palette_boundary(m).is_none()&&pin_target(m).is_none()&&!matches!(m,M::ResizeOutput(_)){self.index=0;self.phase=14;return Ok(());}
  if let Some(row)=p.pinned.get(self.index){
   let d=self.diff.as_mut().unwrap();
   let remove=match m{M::PinSolution(_)|M::RestorePins(_)=>true,M::ResizeOutput(v)=>d.output.is_some()&&(row.x>=v.width||row.y>=v.height),M::UnpinPixel(v)=>row.x==v.x&&row.y==v.y,_=>false};
   if remove{let id=pin_key(row.x,row.y);self.key_capacity+=id.capacity();d.pinned.removed.push(BitmapPinnedRemoval{id,index:self.index});}
   if let Some((x,y))=pin_target(m){if(row.x,row.y)==(x,y)&&self.found.is_none(){self.found=Some(self.index);}if(row.y,row.x)>(y,x)&&self.insert_at==p.pinned.len(){self.insert_at=self.index;}}
   if let Some((boundary,add))=palette_boundary(m){if !add&&row.color==boundary{return Err(self.reject(protocol::MutationMessage::error("mutation.target-referenced",format!("Palette colour {boundary} is still painted or pinned.")).at([boundary.to_string()])));}if(add&&row.color>=boundary)||(!add&&row.color>boundary){let id=pin_key(row.x,row.y);self.key_capacity+=id.capacity();d.pinned.modified.push(BitmapPinnedModification{id,patch:BitmapPinnedPatch{color:Some(if add{row.color+1}else{row.color-1})}});}}
   self.index+=1;
  }else{self.index=0;self.phase=14;}Ok(())
 }
 fn scan_pixel(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  let i=self.index;
  if matches!(m,M::RemovePaletteColor(_))&&p.input.pixels.len()!=cells(p){return Err(self.reject(protocol::MutationMessage::fatal("mutation.apply.invalid-base","The base input pixel buffer does not decode.").at(["input"])));}
  if let Some((boundary,add))=palette_boundary(m){if let Some(&value)=p.input.pixels.get(i){if !add&&u32::from(value)==boundary{return Err(self.reject(protocol::MutationMessage::error("mutation.target-referenced",format!("Palette colour {boundary} is still painted or pinned.")).at([boundary.to_string()])));}if(add&&u32::from(value)>=boundary)||(!add&&u32::from(value)>boundary){self.cell_rows.as_mut().unwrap().push(BitmapPixelCell{x:(i%p.input.width as usize)as u32,y:(i/p.input.width as usize)as u32,value:if add{u32::from(value)+1}else{u32::from(value)-1}});}self.index+=1;return Ok(());}}
  if let M::SetInputPixels(v)=m{if let Some(&value)=v.pixels.get(i){if value as usize>=p.input.palette.len(){return Err(self.reject(protocol::MutationMessage::error("mutation.target-missing",format!("Palette index {value} is not in this document's palette.")).at(["pixels"])));}let x=v.x+i as u32%v.width;let y=v.y+i as u32/v.width;let old=p.input.pixels.get((y*p.input.width+x)as usize).copied().unwrap_or(0);self.same&=value==old;self.bytes.as_mut().unwrap().push(value);self.prior.as_mut().unwrap().push(old);self.index+=1;return Ok(());}}
  if matches!(m,M::SetInputPixels(_))&&p.input.pixels.len()!=cells(p){return Err(self.reject(protocol::MutationMessage::fatal("mutation.apply.invalid-base","The base input pixel buffer does not decode.").at(["input"])));}self.index=0;self.phase=15;Ok(())
 }
 fn scan_assignment(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  let Some(rows)=pins(m)else{self.phase=16;return Ok(())};
  if let Some(&row)=rows.get(self.index){
   if row.x>=p.output.width||row.y>=p.output.height||row.color as usize>=p.input.palette.len(){return Err(self.reject(protocol::MutationMessage::fatal("mutation.invariant","A pin assignment must name unique valid output cells and palette colours.").at([pin_key(row.x,row.y)])));}
   let slot=(row.y*p.output.width+row.x)as usize;
   let duplicate=if matches!(m,M::PinSolution(_)){slot<self.index}else if rows.len()>1{let mask=self.mask.as_mut().unwrap();let duplicate=mask[slot]!=0;mask[slot]=1;duplicate}else{false};
   if duplicate{return Err(self.reject(protocol::MutationMessage::fatal("mutation.invariant","A pin assignment must name unique valid output cells and palette colours.").at([pin_key(row.x,row.y)])));}
   if matches!(m,M::PinSolution(_))&&slot!=self.index{return Err(self.reject(protocol::MutationMessage::fatal("mutation.invariant","A pinned solution must use row-major order.").at(["pinned"])));}
   self.same&=p.pinned.get(self.index)==Some(&row);self.diff.as_mut().unwrap().pinned.inserted.push(BitmapPinnedInsertion{index:self.index,row});self.index+=1;
  }else{self.same&=rows.len()==p.pinned.len();self.index=0;self.phase=16;}Ok(())
 }
 fn stroke(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  let M::PaintInputStroke(v)=m else{self.phase=18;return Ok(())};
  if self.line.is_none(){
   if self.segment>=v.points.len(){if self.extent.2==0{return Err(self.reject(protocol::MutationMessage::error("mutation.target-missing",format!("No stroke cell lies inside the {}×{} input sample.",p.input.width,p.input.height)).at(["points"])));}self.index=0;self.phase=17;return Ok(());}
   let first=v.points[self.segment.saturating_sub(1)];let last=v.points[self.segment];let(x,y,ex,ey)=(i64::from(first.x),i64::from(first.y),i64::from(last.x),i64::from(last.y));let(dx,dy)=((ex-x).abs(),-(ey-y).abs());self.line=Some((x,y,ex,ey,dx,dy,if x<ex{1}else{-1},if y<ey{1}else{-1},dx+dy));
  }
  let(x,y,ex,ey,dx,dy,sx,sy,e)=self.line.unwrap();
  if x<i64::from(p.input.width)&&y<i64::from(p.input.height){let at=(y*i64::from(p.input.width)+x)as usize;self.mask.as_mut().unwrap()[at]=1;let(minx,miny,w,h)=self.extent;let maxx=if w==0{x as u32}else{(minx+w-1).max(x as u32)};let maxy=if h==0{y as u32}else{(miny+h-1).max(y as u32)};let minx=minx.min(x as u32);let miny=miny.min(y as u32);self.extent=(minx,miny,maxx-minx+1,maxy-miny+1);}else{self.partial=true;}
  if(x,y)==(ex,ey){self.line=None;self.segment+=1;}else{let doubled=2*e;let(mut x,mut y,mut e)=(x,y,e);if doubled>=dy{e+=dy;x+=sx;}if doubled<=dx{e+=dx;y+=sy;}self.line=Some((x,y,ex,ey,dx,dy,sx,sy,e));}Ok(())
 }
 fn stroke_region(&mut self,m:&M,p:&BitmapSnapshot){
  let M::PaintInputStroke(v)=m else{self.phase=18;return};
  let(x,y,w,h)=self.extent;if self.index<w as usize*h as usize{let px=x+self.index as u32%w;let py=y+self.index as u32/w;let at=(py*p.input.width+px)as usize;let old=p.input.pixels[at];let next=if self.mask.as_ref().unwrap()[at]!=0{v.color as u8}else{old};self.same&=next==old;self.bytes.as_mut().unwrap().push(next);self.prior.as_mut().unwrap().push(old);self.index+=1;}else{self.index=0;self.phase=18;}
 }
}
impl RetainedMutationPreparation<BitmapSnapshot,M> for Preparation{
 fn new()->Self{Self{diff:ManuallyDrop::new(None),discarded:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),bytes:ManuallyDrop::new(None),prior:ManuallyDrop::new(None),mask:ManuallyDrop::new(None),saved:ManuallyDrop::new(None),cell_rows:ManuallyDrop::new(None),messages:ManuallyDrop::new(None),ledger:None,inverse_close:None,retirement:Default::default(),phase:0,index:0,found:None,insert_at:0,same:true,partial:false,extent:(0,0,0,0),segment:0,line:None,inverse_phase:0,inverse_index:0,strip:0,closing:false,refused:false,noop:false,carried_capacity:0,key_capacity:0}}
 fn decision_demand(&self,m:RetainedCloneRef<'_,M>,p:RetainedCloneRef<'_,BitmapSnapshot>,_:usize)->Result<RetirementDemand,ValueError>{
  Ok(if(1..=10).contains(&self.phase){self.reserve_demand(m.get(),p.get())}else{demand(Self::fixed()+512,1024)})
 }
 fn decide(&mut self,m:RetainedCloneRef<'_,M>,p:RetainedCloneRef<'_,BitmapSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedMutationAdvance,ValueError>{
  let q=self.decision_demand(m,p,body)?;if !fits(q,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}
  let before=self.capacity();let result=(||->Result<(),ValueError>{let(m,p)=(m.get(),p.get());match self.phase{
   0=>{*self.diff=Some(BitmapDiff::default());*self.messages=Some(Vec::new());self.validate(m,p)?;self.phase=1;},
   1..=10=>self.reserve(m,p),
   11=>{let mask=self.mask.as_mut().unwrap();if mask.len()<mask.capacity(){mask.push(0);}else{self.phase=12;}},
   12=>{self.phase=13;self.index=0;},
   13=>self.scan_pin(m,p)?,
   14=>self.scan_pixel(m,p)?,
   15=>self.scan_assignment(m,p)?,
   16=>self.stroke(m,p)?,
   17=>self.stroke_region(m,p),
   18=>{self.finalize(m,p)?;self.phase=19;},
   _=>{}
  }Ok(())})();
  if result.is_err(){self.refused=true;self.phase=19;assert!(self.messages.as_ref().is_some_and(|rows|!rows.is_empty()),"Every semantic refusal carries its original diagnostic");}
  let mut r=receipt(q);r.retained_capacity_bytes=self.capacity().saturating_sub(before);
  if !r.fits(g){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Bitmap preparation exceeded its admitted quote").with_retained_progress(r));}
  Ok(if self.phase==19{RetainedMutationAdvance::Ready(r)}else{RetainedMutationAdvance::Progress(r)})
 }
 fn take_decision(&mut self)->RetainedMutationDecision<BitmapDiff>{assert_eq!(self.phase,19);self.phase=20;if self.refused{RetainedMutationDecision::Refused(self.messages.take().unwrap())}else{RetainedMutationDecision::Admitted(self.diff.take().unwrap(),self.messages.take().unwrap())}}
 fn inverse_demand(&self,m:RetainedCloneRef<'_,M>,p:RetainedCloneRef<'_,BitmapSnapshot>,_:RetainedCloneRef<'_,BitmapDiff>,_:usize)->Result<RetirementDemand,ValueError>{
  let(m,p)=(m.get(),p.get());let cap=match self.inverse_phase{
   0=>self.inverse_count(m,p)*size_of::<M>(),
   1=>if pins(m).is_some(){p.pinned.len()*size_of::<BitmapPinnedPixel>()}else{0},
   4=>self.extent.2 as usize*self.extent.3 as usize,
   _=>0
  };Ok(demand(Self::fixed()+512,cap))
 }
 fn inverse(&mut self,m:RetainedCloneRef<'_,M>,p:RetainedCloneRef<'_,BitmapSnapshot>,d:RetainedCloneRef<'_,BitmapDiff>,body:usize,g:RetainedCloneGrant)->Result<RetainedMutationAdvance,ValueError>{
  let q=self.inverse_demand(m,p,d,body)?;if !fits(q,g){return Ok(RetainedMutationAdvance::Progress(Default::default()));}let before=self.capacity();let(m,p)=(m.get(),p.get());match self.inverse_phase{
   0=>{*self.inverse=Some(Vec::with_capacity(self.inverse_count(m,p)));self.inverse_phase=if self.inverse_count(m,p)==0{9}else{1};},
   1=>{if pins(m).is_some(){*self.saved=Some(Vec::with_capacity(p.pinned.len()));}self.inverse_phase=2;self.inverse_index=0;},
   2=>match m{
    M::PinSolution(_)|M::RestorePins(_)=>if let Some(&row)=p.pinned.get(self.inverse_index){self.saved.as_mut().unwrap().push(row);self.inverse_index+=1;}else{let rows=self.saved.take().unwrap();self.carried_capacity+=rows.capacity()*size_of::<BitmapPinnedPixel>();self.inverse.as_mut().unwrap().push(mutations::restore_pins(rows));self.inverse_phase=9;},
    M::ResizeOutput(v)=>if self.inverse_index<p.pinned.len(){let row=p.pinned[p.pinned.len()-1-self.inverse_index];if row.x>=v.width||row.y>=v.height{self.inverse.as_mut().unwrap().push(mutations::pin_pixel(row.x,row.y,row.color));}self.inverse_index+=1;}else{self.inverse.as_mut().unwrap().push(mutations::resize_output(p.output.width,p.output.height,p.output.periodic));self.inverse_phase=9;},
    M::ResizeInput(v)=>{self.strip=0;self.extent=(0,v.height,v.width.min(p.input.width),p.input.height.saturating_sub(v.height));self.inverse_phase=4;},
    _=>{self.inverse_scalar(m,p);self.inverse_phase=9;}
   },
   4=>{if self.extent.2==0||self.extent.3==0{self.inverse_phase=6;}else{*self.prior=Some(Vec::with_capacity(self.extent.2 as usize*self.extent.3 as usize));self.inverse_index=0;self.inverse_phase=5;}},
   5=>{let(x,y,w,h)=self.extent;if self.inverse_index<w as usize*h as usize{let at=(y+self.inverse_index as u32/w)*p.input.width+x+self.inverse_index as u32%w;self.prior.as_mut().unwrap().push(p.input.pixels[at as usize]);self.inverse_index+=1;}else{let pixels=self.prior.take().unwrap();self.carried_capacity+=pixels.capacity();self.inverse.as_mut().unwrap().push(mutations::set_input_pixels(x,y,w,h,pixels));self.inverse_phase=6;}},
   6=>{let M::ResizeInput(v)=m else{unreachable!()};if self.strip==0{self.strip=1;self.extent=(v.width,0,p.input.width.saturating_sub(v.width),p.input.height);self.inverse_phase=4;}else{self.inverse.as_mut().unwrap().push(mutations::resize_input(p.input.width,p.input.height));self.inverse_phase=9;}},
   _=>{}
  }
  let mut r=receipt(q);r.retained_capacity_bytes=self.capacity().saturating_sub(before);if !r.fits(g){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Bitmap inverse exceeded its admitted quote").with_retained_progress(r));}Ok(if self.inverse_phase==9{RetainedMutationAdvance::Ready(r)}else{RetainedMutationAdvance::Progress(r)})
 }
 fn take_inverse(&mut self)->Vec<M>{assert_eq!(self.inverse_phase,9);self.inverse_phase=10;self.inverse.take().unwrap()}
 fn begin_close(&mut self){self.closing=true;}
 fn close_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<BitmapDiff>(false,&None,body);}
  if self.diff.is_some()||self.discarded.is_some(){return self.retirement.next_owner_demands::<BitmapDiff>(true,&None,body);}
  if self.bytes.is_some()||self.prior.is_some()||self.mask.is_some(){return self.retirement.next_owner_demands::<Vec<u8>>(true,&None,body);}
  if self.saved.is_some(){return self.retirement.next_owner_demands::<Vec<BitmapPinnedPixel>>(true,&None,body);}
  if self.cell_rows.is_some(){return self.retirement.next_owner_demands::<Vec<BitmapPixelCell>>(true,&None,body);}
  if let Some(o)=self.inverse_close.as_ref(){return o.demands(c.mutations,body);}
  if self.inverse.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand());}
  if let Some(o)=self.ledger.as_ref(){return Ok(RetirementDemand{copy_bytes:o.next_copy_byte_demand(),capacity_bytes:o.next_capacity_byte_demand(body)?,release_bytes:o.next_release_byte_demand()?,depth:o.next_depth_demand()?});}
  if self.messages.is_some(){return Ok(demand(size_of::<protocol::MutationMessageLedgerRetirement>(),0));}
  Ok(Default::default())
 }
 fn close_step(&mut self,c:RetainedMutationCloseContext<'_,M>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  assert!(self.closing);let q=self.close_demand(c,g.maximum_copy_bytes)?;if !fits(q,g){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(g).result.map(|s|RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.diff,g).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.discarded,g).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  for v in [&mut *self.bytes,&mut *self.prior,&mut *self.mask]{if let Some(s)=self.retirement.begin_granted(v,g).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}}
  if let Some(s)=self.retirement.begin_granted(&mut self.saved,g).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(s)=self.retirement.begin_granted(&mut self.cell_rows,g).result?{return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(o)=self.inverse_close.as_mut(){let s=o.close_step(c.mutations,g).result?;if o.terminal_is_empty(){self.inverse_close=None;}return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.inverse.take(){return match FactoryVectorRetirement::admit(rows,g){Ok((o,r))=>{self.inverse_close=Some(o);Ok(RetainedCloneStep::Progress(r))},Err((e,rows))=>{*self.inverse=Some(rows);Err(e)}};}
  if let Some(o)=self.ledger.as_mut(){let s=o.close_step(g)?;if o.terminal_is_empty(){self.ledger=None;}return Ok(RetainedCloneStep::Progress(s.progress()));}
  if let Some(rows)=self.messages.take(){self.ledger=Some(protocol::MutationMessageLedgerRetirement::new(String::new(),rows));return Ok(RetainedCloneStep::Progress(receipt(q)));}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.diff.is_none()&&self.discarded.is_none()&&self.inverse.is_none()&&self.bytes.is_none()&&self.prior.is_none()&&self.mask.is_none()&&self.saved.is_none()&&self.cell_rows.is_none()&&self.messages.is_none()&&self.ledger.is_none()&&self.inverse_close.is_none()&&self.retirement.is_empty()}
}
impl Preparation{
 fn capacity(&self)->usize{
  let mut n=self.key_capacity+self.carried_capacity;for d in [self.diff.as_ref(),self.discarded.as_ref()].into_iter().flatten(){n+=d.pinned.removed.capacity()*size_of::<BitmapPinnedRemoval>()+d.pinned.inserted.capacity()*size_of::<BitmapPinnedInsertion>()+d.pinned.modified.capacity()*size_of::<BitmapPinnedModification>()+d.input.writes.capacity()*size_of::<BitmapInputWrite>()+d.palette.removed.capacity()*size_of::<u32>()+d.palette.inserted.capacity()*size_of::<BitmapPaletteInsertion>()+d.palette.recolored.capacity()*size_of::<BitmapPaletteRecolor>();}
  n+=self.inverse.as_ref().map_or(0,|v|v.capacity()*size_of::<M>());
  n+=self.bytes.as_ref().map_or(0,Vec::capacity)+self.prior.as_ref().map_or(0,Vec::capacity)+self.mask.as_ref().map_or(0,Vec::capacity);
  n+=self.saved.as_ref().map_or(0,|v|v.capacity()*size_of::<BitmapPinnedPixel>())+self.cell_rows.as_ref().map_or(0,|v|v.capacity()*size_of::<BitmapPixelCell>());
  if let Some(rows)=self.messages.as_ref(){n+=rows.capacity()*size_of::<protocol::MutationMessage>()+rows.iter().map(|r|r.code.0.capacity()+r.message.capacity()+r.target.capacity()*size_of::<String>()+r.target.iter().map(String::capacity).sum::<usize>()).sum::<usize>();}n
 }
 fn finalize(&mut self,m:&M,p:&BitmapSnapshot)->Result<(),ValueError>{
  let d=self.diff.as_mut().unwrap();
  match m{
   M::PinPixel(v)=>match self.found{
    Some(i)=>if p.pinned[i].color!=v.color{let id=pin_key(v.x,v.y);self.key_capacity+=id.capacity();d.pinned.modified.push(BitmapPinnedModification{id,patch:BitmapPinnedPatch{color:Some(v.color)}})},
    None=>d.pinned.inserted.push(BitmapPinnedInsertion{index:self.insert_at,row:BitmapPinnedPixel{x:v.x,y:v.y,color:v.color}})
   },
   M::UnpinPixel(v)=>if self.found.is_none(){return Err(self.reject(protocol::MutationMessage::error("mutation.target-missing",format!("({}, {}) carries no pin.",v.x,v.y)).at([pin_key(v.x,v.y)])));},
   M::SetInputPixels(_)|M::PaintInputStroke(_)=>if !self.same{let(x,y,width,height)=self.extent;let pixels=self.bytes.take().unwrap();self.carried_capacity+=pixels.capacity();d.input.writes.push(BitmapInputWrite::Region{region:BitmapPixelRegion{x,y,width,height,pixels}});},
   M::AddPaletteColor(_)|M::RemovePaletteColor(_)=>if !self.cell_rows.as_ref().unwrap().is_empty(){let cells=self.cell_rows.take().unwrap();self.carried_capacity+=cells.capacity()*size_of::<BitmapPixelCell>();d.input.writes.push(BitmapInputWrite::Cells{cells});},
   M::PinSolution(_)|M::RestorePins(_)=>if self.same{*self.discarded=self.diff.take();*self.diff=Some(BitmapDiff::default());},
   _=>{}
  }
  let warning=match m{
   M::ChangeSeed(v)if self.diff.as_ref().unwrap().seed.is_none()=>Some(format!("Seed is already {}.",v.seed)),
   M::ResizeInput(v)if self.diff.as_ref().unwrap().input.size.is_none()=>Some(format!("The input is already {}×{}.",v.width,v.height)),
   M::ResizeOutput(v)if self.diff.as_ref().unwrap().output.is_none()=>Some(format!("The output is already {}×{}.",v.width,v.height)),
   M::ChangeModel(_)if self.diff.as_ref().unwrap().model.is_none()=>Some("The model already holds those parameters.".into()),
   M::ChangePaletteColor(v)if self.diff.as_ref().unwrap().palette.recolored.is_empty()=>Some(format!("Palette colour {} already holds that value.",v.index)),
   M::PinPixel(v)if self.found.is_some_and(|i|p.pinned[i].color==v.color)=>Some(format!("({}, {}) is already pinned to colour {}.",v.x,v.y,v.color)),
   M::SetInputPixels(_)if self.same=>Some("The region already holds these pixels.".into()),
   M::PaintInputStroke(_)if self.same=>Some("Every stroke cell already holds this colour.".into()),
   M::PinSolution(_)|M::RestorePins(_)if self.same=>Some("The output already carries this pin assignment.".into()),
   _=>None
  };
  let message=if let Some(message)=warning{Some(protocol::MutationMessage::warning("mutation.no-op",message))}else if matches!(m,M::ResizeOutput(_))&&!self.diff.as_ref().unwrap().pinned.removed.is_empty(){Some(protocol::MutationMessage::info("mutation.cascade",format!("{} pinned pixel(s) fell outside the new output and were removed.",self.diff.as_ref().unwrap().pinned.removed.len())))}else if matches!(m,M::PaintInputStroke(_))&&self.partial{Some(protocol::MutationMessage::warning("mutation.partial",format!("Stroke cells outside the {}×{} input sample are skipped.",p.input.width,p.input.height)))}else{None};
  if let Some(message)=message{let mut rows=Vec::with_capacity(1);rows.push(message);*self.messages=Some(rows);}self.noop=protocol::DiffAlgebra::<BitmapSnapshot>::is_empty(self.diff.as_ref().unwrap());Ok(())
 }
 fn inverse_count(&self,m:&M,p:&BitmapSnapshot)->usize{if self.noop{return 0;}match m{
  M::PinSolution(_)|M::RestorePins(_)=>usize::from(!self.same),
  M::ResizeInput(v)=>if(v.width,v.height)==(p.input.width,p.input.height){0}else{3},
  M::ResizeOutput(v)=>if(v.width,v.height,v.periodic)==(p.output.width,p.output.height,p.output.periodic){0}else{p.pinned.len()+1},
  M::PinPixel(v)=>usize::from(self.found.is_none_or(|i|p.pinned[i].color!=v.color)),
  _=>1
 }}
 fn inverse_scalar(&mut self,m:&M,p:&BitmapSnapshot){
  let rows=self.inverse.as_mut().unwrap();match m{
   M::ChangeSeed(_)=>rows.push(mutations::change_seed(p.seed)),
   M::ChangeModel(_)=>rows.push(mutations::change_model(p.model.pattern_size,p.model.symmetry,p.model.periodic_input,p.model.ground)),
   M::AddPaletteColor(v)=>rows.push(mutations::remove_palette_color(v.index)),
   M::RemovePaletteColor(v)=>rows.push(mutations::add_palette_color(v.index,p.input.palette[v.index])),
   M::ChangePaletteColor(v)=>rows.push(mutations::change_palette_color(v.index,p.input.palette[v.index])),
   M::PinPixel(v)=>rows.push(match self.found{Some(i)=>mutations::pin_pixel(v.x,v.y,p.pinned[i].color),None=>mutations::unpin_pixel(v.x,v.y)}),
   M::UnpinPixel(_)=>{let row=p.pinned[self.found.unwrap()];rows.push(mutations::pin_pixel(row.x,row.y,row.color));},
   M::SetInputPixels(_)|M::PaintInputStroke(_)=>{let(x,y,w,h)=self.extent;let pixels=self.prior.take().unwrap();self.carried_capacity+=pixels.capacity();rows.push(mutations::set_input_pixels(x,y,w,h,pixels));},
   _=>unreachable!()
  }
 }
}
impl Drop for Preparation{
 fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"Bitmap preparation requires admitted closure");if empty{unsafe{ManuallyDrop::drop(&mut self.diff);ManuallyDrop::drop(&mut self.discarded);ManuallyDrop::drop(&mut self.inverse);ManuallyDrop::drop(&mut self.bytes);ManuallyDrop::drop(&mut self.prior);ManuallyDrop::drop(&mut self.mask);ManuallyDrop::drop(&mut self.saved);ManuallyDrop::drop(&mut self.cell_rows);ManuallyDrop::drop(&mut self.messages);}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
