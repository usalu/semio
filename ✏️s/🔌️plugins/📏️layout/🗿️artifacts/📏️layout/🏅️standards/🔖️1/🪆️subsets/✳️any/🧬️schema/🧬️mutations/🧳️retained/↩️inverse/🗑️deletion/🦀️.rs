//! 🗑️ Deleted rows retain genuine original-field preparation and inverse command custody.
use crate::LayoutMutation;
use super::super::simple::SimpleKind;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,ImageLink,TextStory,ParagraphStyle,CharacterStyle,Spread,ParentPage};
use semio_os_kernel::RetainedMutationCloseContext;
use super::super::original_field::{OriginalField,close_rows,rows_work_demand};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{size_of,ManuallyDrop};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"deleted Layout row inverse custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutMutation>()*4+512,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn source_link<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,ImageLink>{b.project(6,|b|&b.links).project(at,move|rows|&rows[at])}
fn source_story<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,TextStory>{b.project(5,|b|&b.stories).project(at,move|rows|&rows[at])}
fn source_paragraph<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,ParagraphStyle>{b.project(3,|b|&b.paragraph_styles).project(at,move|rows|&rows[at])}
fn source_character<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,CharacterStyle>{b.project(4,|b|&b.character_styles).project(at,move|rows|&rows[at])}
fn source_spread<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,Spread>{b.project(8,|b|&b.spreads).project(at,move|rows|&rows[at])}
fn source_parent<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,ParentPage>{b.project(7,|b|&b.parent_pages).project(at,move|rows|&rows[at])}
/// 🧳 Only the deleted row and a required duplicate identifier become new owned inverse sources.
pub struct DeletionInverse{parent:OriginalField<ParentPage>,link:OriginalField<ImageLink>,story:OriginalField<TextStory>,paragraph:OriginalField<ParagraphStyle>,character:OriginalField<CharacterStyle>,spread:OriginalField<Spread>,id:OriginalField<String>,rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,retirement:Option<FactoryVectorRetirement<LayoutMutation>>,phase:u8,closing:bool}
impl Default for DeletionInverse{fn default()->Self{Self{parent:Default::default(),link:Default::default(),story:Default::default(),paragraph:Default::default(),character:Default::default(),spread:Default::default(),id:Default::default(),rows:ManuallyDrop::new(None),retirement:None,phase:0,closing:false}}}
impl DeletionInverse{
 pub fn ready(&self)->bool{self.phase==5}
 pub fn demand(&self,kind:SimpleKind,at:[usize;2],b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing||!kind.delete(){return Err(fault())}Ok(match self.phase{0=>RetirementDemand{capacity_bytes:size_of::<LayoutMutation>()*if kind==SimpleKind::DeleteCharacterStyle{2}else{1},..fixed()},1=>match kind{SimpleKind::DeleteParentPage=>if self.parent.value().is_some(){fixed()}else{nested(self.parent.demand(source_parent(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},SimpleKind::DeleteLink=>if self.link.value().is_some(){fixed()}else{nested(self.link.demand(source_link(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},SimpleKind::DeleteStory=>if self.story.value().is_some(){fixed()}else{nested(self.story.demand(source_story(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},SimpleKind::DeleteParagraphStyle=>if self.paragraph.value().is_some(){fixed()}else{nested(self.paragraph.demand(source_paragraph(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},SimpleKind::DeleteCharacterStyle=>if self.character.value().is_some(){fixed()}else{nested(self.character.demand(source_character(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},SimpleKind::DeleteSpread=>if self.spread.value().is_some(){fixed()}else{nested(self.spread.demand(source_spread(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1))?)},_=>return Err(fault())},2=>if kind==SimpleKind::DeleteCharacterStyle&&self.id.value().is_none(){nested(self.id.demand(source_character(b,at[0]).project(0,|row|&row.id),body.saturating_sub(256),maximum.saturating_sub(1))?)}else{fixed()},3=>{let mut d=nested(match kind{SimpleKind::DeleteParentPage=>self.parent.exchange_demand()?,SimpleKind::DeleteLink=>self.link.exchange_demand()?,SimpleKind::DeleteStory=>self.story.exchange_demand()?,SimpleKind::DeleteParagraphStyle=>self.paragraph.exchange_demand()?,SimpleKind::DeleteCharacterStyle=>self.id.exchange_demand()?,SimpleKind::DeleteSpread=>self.spread.exchange_demand()?,_=>return Err(fault())});d.copy_bytes+=size_of::<LayoutMutation>()*2;d},4=>{let mut d=nested(self.character.exchange_demand()?);d.copy_bytes+=size_of::<LayoutMutation>()*2;d},5|7=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
    [OriginalField::<ParentPage>::embedded_birth_work()?,OriginalField::<ImageLink>::embedded_birth_work()?,OriginalField::<TextStory>::embedded_birth_work()?,OriginalField::<ParagraphStyle>::embedded_birth_work()?,OriginalField::<CharacterStyle>::embedded_birth_work()?,OriginalField::<Spread>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?]
        .into_iter().try_fold(1u64,|work,child|work.checked_add(child).ok_or_else(fault))
 }
 pub fn work_demand(&self,kind:SimpleKind,at:[usize;2],b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<u64,ValueError>{
    let limit=maximum.saturating_sub(1);
    let selected=match self.phase{1=>match kind{
   SimpleKind::DeleteParentPage if self.parent.value().is_none()=>self.parent.work_demand(source_parent(b,at[0]),body.saturating_sub(256),limit)?,
SimpleKind::DeleteLink if self.link.value().is_none()=>self.link.work_demand(source_link(b,at[0]),body.saturating_sub(256),limit)?,
SimpleKind::DeleteStory if self.story.value().is_none()=>self.story.work_demand(source_story(b,at[0]),body.saturating_sub(256),limit)?,
SimpleKind::DeleteParagraphStyle if self.paragraph.value().is_none()=>self.paragraph.work_demand(source_paragraph(b,at[0]),body.saturating_sub(256),limit)?,
SimpleKind::DeleteCharacterStyle if self.character.value().is_none()=>self.character.work_demand(source_character(b,at[0]),body.saturating_sub(256),limit)?,
SimpleKind::DeleteSpread if self.spread.value().is_none()=>self.spread.work_demand(source_spread(b,at[0]),body.saturating_sub(256),limit)?,
   _=>0
 },2 if kind==SimpleKind::DeleteCharacterStyle&&self.id.value().is_none()=>self.id.work_demand(source_character(b,at[0]).project(0,|row|&row.id),body.saturating_sub(256),limit)?,3|4=>1,_=>0};
    selected.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,kind:SimpleKind,at:[usize;2],b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{fixed()}else{self.demand(kind,at,b,body,maximum)?};
    if !recover{
        if !retained_clone_grant_admits(g,d)||self.work_demand(kind,at,b,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !effect.performed(1)?{return Err(fault().into())}
    }
 let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{0=>{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();let n=if kind==SimpleKind::DeleteCharacterStyle{2}else{1};let reserved=rows.try_reserve_exact(n);p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();*effect.actual()=Some(p.retained_capacity_bytes);if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1=>match kind{SimpleKind::DeleteParentPage=>if self.parent.value().is_some(){self.phase=2}else{p=receipt(self.parent.advance(source_parent(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},SimpleKind::DeleteLink=>if self.link.value().is_some(){self.phase=2}else{p=receipt(self.link.advance(source_link(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},SimpleKind::DeleteStory=>if self.story.value().is_some(){self.phase=2}else{p=receipt(self.story.advance(source_story(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},SimpleKind::DeleteParagraphStyle=>if self.paragraph.value().is_some(){self.phase=2}else{p=receipt(self.paragraph.advance(source_paragraph(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},SimpleKind::DeleteCharacterStyle=>if self.character.value().is_some(){self.phase=2}else{p=receipt(self.character.advance(source_character(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},SimpleKind::DeleteSpread=>if self.spread.value().is_some(){self.phase=2}else{p=receipt(self.spread.advance(source_spread(b,at[0]),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},_=>return Err(fault().into())},2=>if kind==SimpleKind::DeleteCharacterStyle{if self.id.value().is_some(){self.phase=3}else{p=receipt(self.id.advance(source_character(b,at[0]).project(0,|row|&row.id),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())}}else{self.phase=3},3=>{let inner=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g};match kind{SimpleKind::DeleteParentPage=>{let(value,paid)=self.parent.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateParentPage(crate::create_parent_page::CreateParentPage{page:value.expect("original deleted parent row"),index:Some(at[0])}));self.phase=5},SimpleKind::DeleteLink=>{let(value,paid)=self.link.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateLink(crate::create_link::CreateLink{link:value.expect("original deleted link row"),index:Some(at[0])}));self.phase=5},SimpleKind::DeleteStory=>{let(value,paid)=self.story.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateStory(crate::create_story::CreateStory{story:value.expect("original deleted story row"),index:Some(at[0])}));self.phase=5},SimpleKind::DeleteParagraphStyle=>{let(value,paid)=self.paragraph.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateParagraphStyle(crate::create_paragraph_style::CreateParagraphStyle{style:value.expect("original deleted paragraph row"),index:Some(at[0])}));self.phase=5},SimpleKind::DeleteSpread=>{let(value,paid)=self.spread.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateSpread(crate::create_spread::CreateSpread{spread:value.expect("original deleted spread row"),index:Some(at[0])}));self.phase=5},SimpleKind::DeleteCharacterStyle=>{let(id,paid)=self.id.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);self.rows.as_mut().unwrap().push(LayoutMutation::CreateCharacterStyle(crate::create_character_style::CreateCharacterStyle{id:id.expect("original second character identifier"),name:None,index:Some(at[0])}));self.phase=4},_=>return Err(fault().into())}p.copied_bytes+=size_of::<LayoutMutation>()*2},4=>{let(value,paid)=self.character.take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<LayoutMutation>()*2;let row=value.expect("original deleted character style");self.rows.as_mut().unwrap().push(LayoutMutation::UpdateCharacterStyle(crate::update_character_style::UpdateCharacterStyle{id:row.id,name:row.name,font_family:row.font_family,font_size:row.font_size,font_weight:row.font_weight,italic:row.italic,color:row.color,tracking:row.tracking}));self.phase=7},7=>{self.rows.as_mut().unwrap().swap(0,1);self.phase=5},5=>return Ok(RetainedCloneStep::Complete(p)),_=>return Err(fault().into())}if self.phase==5{Ok(RetainedCloneStep::Complete(p))}else{Ok(RetainedCloneStep::Progress(p))}}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<LayoutMutation>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<LayoutMutation>>,RetainedCloneProgress),ValueError>{if !self.ready()||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=6;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.parent.begin_close();self.link.begin_close();self.story.begin_close();self.paragraph.begin_close();self.character.begin_close();self.spread.begin_close();self.id.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.parent.terminal_is_empty(){return self.parent.close_demand(body)}if !self.link.terminal_is_empty(){return self.link.close_demand(body)}if !self.story.terminal_is_empty(){return self.story.close_demand(body)}if !self.paragraph.terminal_is_empty(){return self.paragraph.close_demand(body)}if !self.character.terminal_is_empty(){return self.character.close_demand(body)}if !self.spread.terminal_is_empty(){return self.spread.close_demand(body)}if !self.id.terminal_is_empty(){return self.id.close_demand(body)}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<LayoutMutation>>>()*2+256,depth:1,..Default::default()})}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
    if !self.parent.terminal_is_empty(){return self.parent.close_work_demand(body)}
    if !self.link.terminal_is_empty(){return self.link.close_work_demand(body)}
    if !self.story.terminal_is_empty(){return self.story.close_work_demand(body)}
    if !self.paragraph.terminal_is_empty(){return self.paragraph.close_work_demand(body)}
    if !self.character.terminal_is_empty(){return self.character.close_work_demand(body)}
    if !self.spread.terminal_is_empty(){return self.spread.close_work_demand(body)}
    if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}

    rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.parent.terminal_is_empty(){return self.parent.requires_original_birth()}
    if !self.link.terminal_is_empty(){return self.link.requires_original_birth()}
    if !self.story.terminal_is_empty(){return self.story.requires_original_birth()}
    if !self.paragraph.terminal_is_empty(){return self.paragraph.requires_original_birth()}
    if !self.character.terminal_is_empty(){return self.character.requires_original_birth()}
    if !self.spread.terminal_is_empty(){return self.spread.requires_original_birth()}
    if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())
 }
 pub fn requires_original_control(&self)->bool{
    if !self.parent.terminal_is_empty(){return self.parent.requires_original_control()}
    if !self.link.terminal_is_empty(){return self.link.requires_original_control()}
    if !self.story.terminal_is_empty(){return self.story.requires_original_control()}
    if !self.paragraph.terminal_is_empty(){return self.paragraph.requires_original_control()}
    if !self.character.terminal_is_empty(){return self.character.requires_original_control()}
    if !self.spread.terminal_is_empty(){return self.spread.requires_original_control()}
    if !self.id.terminal_is_empty(){return self.id.requires_original_control()}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.parent.terminal_is_empty(){return self.parent.close_original_birth_step(original,grant)}
    if !self.link.terminal_is_empty(){return self.link.close_original_birth_step(original,grant)}
    if !self.story.terminal_is_empty(){return self.story.close_original_birth_step(original,grant)}
    if !self.paragraph.terminal_is_empty(){return self.paragraph.close_original_birth_step(original,grant)}
    if !self.character.terminal_is_empty(){return self.character.close_original_birth_step(original,grant)}
    if !self.spread.terminal_is_empty(){return self.spread.close_original_birth_step(original,grant)}
    if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(original,grant)}

    self.retirement.as_mut().expect("selected Layout inverse retirement").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    let grant=effect.grant();
    if !recover&&!retained_clone_grant_admits(grant,self.close_demand(c,body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if !self.parent.terminal_is_empty(){return self.parent.close_step(original,body,effect,recover)}
    if !self.link.terminal_is_empty(){return self.link.close_step(original,body,effect,recover)}
    if !self.story.terminal_is_empty(){return self.story.close_step(original,body,effect,recover)}
    if !self.paragraph.terminal_is_empty(){return self.paragraph.close_step(original,body,effect,recover)}
    if !self.character.terminal_is_empty(){return self.character.close_step(original,body,effect,recover)}
    if !self.spread.terminal_is_empty(){return self.spread.close_step(original,body,effect,recover)}
    if !self.id.terminal_is_empty(){return self.id.close_step(original,body,effect,recover)}

    close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.parent.terminal_is_empty()&&self.link.terminal_is_empty()&&self.story.terminal_is_empty()&&self.paragraph.terminal_is_empty()&&self.character.terminal_is_empty()&&self.spread.terminal_is_empty()&&self.id.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_none()}}
impl Drop for DeletionInverse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"deleted Layout row inverse needs original funded cleanup")}}
