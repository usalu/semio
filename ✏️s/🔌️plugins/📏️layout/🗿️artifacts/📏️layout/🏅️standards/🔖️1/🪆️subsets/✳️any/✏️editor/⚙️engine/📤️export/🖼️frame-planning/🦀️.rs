//! 🖼️ Stable export paint addresses are prepared from original identifiers without cloned frames.
use semio_s_artifact_layout_layout_model::{Frame,LayoutBounds,snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_engine::scene::metadata_index::{MetadataIndex,MetadataKey,MetadataSource,MetadataLookup};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,numeric_scratch::NumericIndex,sort::{HeapSortCursor,HeapSortWork}};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FramePlanAddress{pub page_index:usize,pub parent_index:Option<usize>,pub frame_index:usize,pub override_index:Option<usize>,pub link_index:Option<usize>}
impl FramePlanAddress{
 pub fn frame<'a>(&self,snapshot:&'a LayoutSnapshot)->Result<&'a Frame,String>{match self.parent_index{Some(parent)=>snapshot.parent_pages.get(parent).and_then(|owner|owner.frames.get(self.frame_index)),None=>snapshot.pages.get(self.page_index).and_then(|owner|owner.frames.get(self.frame_index))}.ok_or_else(||"layout-export-frame-address".into())}
 fn page_override<'a>(&self,snapshot:&'a LayoutSnapshot)->Result<Option<&'a semio_s_artifact_layout_layout_model::PageOverride>,String>{self.override_index.map(|index|snapshot.pages.get(self.page_index).and_then(|page|page.overrides.get(index)).ok_or_else(||"layout-export-frame-override".into())).transpose()}
 pub fn bounds(&self,snapshot:&LayoutSnapshot)->Result<LayoutBounds,String>{let bounds=self.page_override(snapshot)?.and_then(|over|over.bounds.as_ref()).unwrap_or(self.frame(snapshot)?.bounds());Ok(LayoutBounds{x:bounds.x,y:bounds.y,width:bounds.width,height:bounds.height,rotation:bounds.rotation})}
 pub fn visible(&self,snapshot:&LayoutSnapshot)->Result<bool,String>{Ok(self.page_override(snapshot)?.and_then(|over|over.visible).unwrap_or(self.frame(snapshot)?.visible()))}
}
#[derive(Clone,Copy)]
struct Row{address:FramePlanAddress,rank:usize}
#[derive(Clone,Copy)]
struct SortState(HeapSortCursor);
use super::FramePlanTurn;
struct Source<'a>{doc:&'a LayoutSnapshot}
impl MetadataSource for Source<'_>{
 fn identity(&self)->usize{self.doc as*const LayoutSnapshot as usize}
 fn text(&self,key:MetadataKey)->Result<&str,&'static str>{let doc=self.doc;match key.kind{
  0=>doc.parent_pages.get(key.a).map(|owner|owner.id.as_str()),
  1=>doc.parent_pages.get(key.a).and_then(|owner|owner.layers.get(key.b)).map(|layer|layer.id.as_str()),
  2=>doc.pages.get(key.a).and_then(|owner|owner.layers.get(key.b)).map(|layer|layer.id.as_str()),
  3=>doc.pages.get(key.a).and_then(|owner|owner.overrides.get(key.b)).map(|over|over.object_id.as_str()),
  4=>doc.pages.get(key.a).and_then(|page|page.parent_page_id.as_deref()),
  5=>doc.parent_pages.get(key.a).and_then(|owner|owner.frames.get(key.b)).map(Frame::layer_id),
  6=>doc.pages.get(key.a).and_then(|owner|owner.frames.get(key.b)).map(Frame::layer_id),
  7=>doc.parent_pages.get(key.a).and_then(|owner|owner.frames.get(key.b)).map(Frame::id),8=>doc.links.get(key.a).map(|link|link.id.as_str()),9|10=>{let frame=if key.kind==9{doc.parent_pages.get(key.a).and_then(|owner|owner.frames.get(key.b))}else{doc.pages.get(key.a).and_then(|owner|owner.frames.get(key.b))};match frame{Some(Frame::Image{link_id,..})=>Some(link_id.as_str()),_=>None}},_=>None}.ok_or("layout-export-frame-key")}
}
pub struct FramePlanState{metadata:MetadataIndex,rows:NumericIndex<usize,Row>,sort:Option<SortState>,source:Option<usize>,target:Option<usize>,phase:u8,owner:usize,item:usize,page:usize,parent:Option<usize>,frame:usize,row_phase:u8,rank:usize,over:Option<usize>,link:Option<usize>,output:usize,output_page:Option<usize>,cancelled:bool,last:RetainedCloneProgress}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
impl FramePlanState{
 pub fn new(target:Option<usize>)->Self{Self{metadata:MetadataIndex::new(),rows:NumericIndex::new(),sort:None,source:None,target,phase:0,owner:0,item:0,page:target.unwrap_or(0),parent:None,frame:0,row_phase:0,rank:0,over:None,link:None,output:0,output_page:None,cancelled:false,last:Default::default()}}
 pub fn ready(&self)->bool{self.phase==9&&!self.cancelled}
 pub fn last_receipt(&self)->RetainedCloneProgress{self.last}
 pub fn demand(&self,doc:&LayoutSnapshot)->Result<RetirementDemand,ValueError>{if self.source.is_some_and(|source|source!=doc as*const LayoutSnapshot as usize){return Err(invalid("layout frame plan source changed"));}let mut demand=RetirementDemand{depth:1,..Default::default()};match self.phase{0..=4|10=>return self.metadata.next_demand(),5|6 if self.row_phase<3=>return self.metadata.next_demand(),5|6=>{demand=self.rows.next_reserve_demand()?;demand.depth=demand.depth.max(1);if demand.capacity_bytes==0{demand.copy_bytes=std::mem::size_of::<Row>();}},8=>{if matches!(self.sort.as_ref().unwrap().0.next(),HeapSortWork::Swap{..}){demand.copy_bytes=2*std::mem::size_of::<Row>();}},_=>{}}Ok(demand)}
 fn query(&mut self,doc:&LayoutSnapshot,key:MetadataKey,domain:u8,scope:usize,insert:Option<usize>,grant:RetainedCloneGrant)->Result<Option<Option<usize>>,ValueError>{let result=self.metadata.query(&Source{doc},key,domain,scope,insert,grant);self.last=self.metadata.last_receipt();Ok(match result?{MetadataLookup::Pending=>None,MetadataLookup::Ready(value)=>Some(value)})}
 fn next_page(&mut self){self.frame=0;self.row_phase=0;self.parent=None;if self.target.is_some(){self.phase=7;}else{self.page+=1;self.phase=4;}}
 pub fn prepare(&mut self,doc:&LayoutSnapshot,grant:RetainedCloneGrant)->Result<bool,ValueError>{self.last=Default::default();if self.cancelled{return Err(invalid("layout frame plan cancelled"));}if self.source.is_some_and(|source|source!=doc as*const LayoutSnapshot as usize){return Err(invalid("layout frame plan source changed"));}if self.ready(){return Ok(true);}let demand=self.demand(doc)?;if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(false);}if self.target.is_some_and(|page|page>=doc.pages.len()){return Err(invalid("layout frame target is missing"));}self.source=Some(doc as*const LayoutSnapshot as usize);self.last.copied_items=1;
  match self.phase{
   0=>{if self.owner==doc.parent_pages.len(){self.owner=0;self.phase=1;}else if self.query(doc,MetadataKey{kind:0,a:self.owner,b:0},0,0,Some(self.owner),grant)?.is_some(){self.owner+=1;}},
   1|2|3=>{let owners=if self.phase==1{doc.parent_pages.len()}else{doc.pages.len()};if self.owner==owners{self.owner=0;self.item=0;self.phase=if self.phase==3{10}else{self.phase+1};}else{let length=match self.phase{1=>doc.parent_pages[self.owner].layers.len(),2=>doc.pages[self.owner].layers.len(),_=>doc.pages[self.owner].overrides.len()};if self.item==length{self.owner+=1;self.item=0;}else{let domain=self.phase;let key=MetadataKey{kind:domain,a:self.owner,b:self.item};if self.query(doc,key,domain,self.owner,Some(self.item),grant)?.is_some(){self.item+=1;}}}},
   10=>{if self.owner==doc.links.len(){self.owner=0;self.phase=4;}else if self.query(doc,MetadataKey{kind:8,a:self.owner,b:0},4,0,Some(self.owner),grant)?.is_some(){self.owner+=1;}},
   4=>{if self.page==doc.pages.len(){self.phase=7;}else if doc.pages[self.page].parent_page_id.is_none(){self.phase=6;self.parent=None;self.frame=0;}else if let Some(parent)=self.query(doc,MetadataKey{kind:4,a:self.page,b:0},0,0,None,grant)?{self.parent=Some(parent.ok_or_else(||invalid("layout parent page is missing"))?);self.phase=5;self.frame=0;}},
   5|6=>{let parent=if self.phase==5{self.parent}else{None};let(frame_count,layer_count)=match parent{Some(parent)=>(doc.parent_pages[parent].frames.len(),doc.parent_pages[parent].layers.len()),None=>(doc.pages[self.page].frames.len(),doc.pages[self.page].layers.len())};if self.frame==frame_count{self.frame=0;self.row_phase=0;if self.phase==5{self.phase=6;}else{self.next_page();}return Ok(false);}
    if self.row_phase==0{let(domain,scope,key)=match parent{Some(parent)=>(1,parent,MetadataKey{kind:5,a:parent,b:self.frame}),None=>(2,self.page,MetadataKey{kind:6,a:self.page,b:self.frame})};if let Some(rank)=self.query(doc,key,domain,scope,None,grant)?{self.rank=rank.unwrap_or(layer_count);let visible=match parent{Some(parent)=>doc.parent_pages[parent].layers.get(self.rank),None=>doc.pages[self.page].layers.get(self.rank)}.is_none_or(|layer|layer.visible);if !visible{self.frame+=1;}else{self.row_phase=1;}}}
    else if self.row_phase==1{if let Some(parent)=parent{if let Some(over)=self.query(doc,MetadataKey{kind:7,a:parent,b:self.frame},3,self.page,None,grant)?{self.over=over;self.row_phase=2;}}else{self.over=None;self.row_phase=2;}}
    else if self.row_phase==2{let original=match parent{Some(parent)=>&doc.parent_pages[parent].frames[self.frame],None=>&doc.pages[self.page].frames[self.frame]};if matches!(original,Frame::Image{..}){let key=MetadataKey{kind:if parent.is_some(){9}else{10},a:parent.unwrap_or(self.page),b:self.frame};if let Some(link)=self.query(doc,key,4,0,None,grant)?{self.link=Some(link.ok_or_else(||invalid("layout image link is missing"))?);self.row_phase=3;}}else{self.link=None;self.row_phase=3;}}
    else{let capacity=self.rows.next_allocation_bytes()?;if capacity>0{let progress=self.rows.reserve_one_funded(grant).map_err(|error|{self.last.retained_capacity_bytes=error.allocated_bytes;ValueError::from(error.refusal())})?;self.last.retained_capacity_bytes=progress.retained_capacity_bytes;self.last.copied_bytes=progress.copied_bytes;}else{let bytes=std::mem::size_of::<Row>();if grant.maximum_copy_bytes<bytes{return Ok(false);}self.rows.insert_reserved(self.rows.len(),Row{address:FramePlanAddress{page_index:self.page,parent_index:parent,frame_index:self.frame,override_index:self.over,link_index:self.link},rank:self.rank}).map_err(|_|invalid("layout frame row reservation failed"))?;self.last.copied_bytes=bytes;self.frame+=1;self.row_phase=0;}}},
   7=>{self.sort=Some(SortState(HeapSortCursor::new(self.rows.len())));self.phase=8;},
   8=>{let cursor=&mut self.sort.as_mut().unwrap().0;match cursor.next(){HeapSortWork::Advance=>cursor.advance(None).map_err(|_|invalid("layout frame sort"))?,HeapSortWork::Compare{left,right}=>{let key=|row:&Row|(row.address.page_index,usize::from(row.address.parent_index.is_none()),row.rank,row.address.frame_index);cursor.advance(Some(key(self.rows.get(&left).unwrap()).cmp(&key(self.rows.get(&right).unwrap())))).map_err(|_|invalid("layout frame sort"))?;},HeapSortWork::Swap{left,right}=>{let bytes=2*std::mem::size_of::<Row>();if grant.maximum_copy_bytes<bytes{return Ok(false);}let a=*self.rows.get(&left).unwrap();let b=*self.rows.get(&right).unwrap();self.rows.insert_reserved(left,b).map_err(|_|invalid("layout frame row reservation failed"))?;self.rows.insert_reserved(right,a).map_err(|_|invalid("layout frame row reservation failed"))?;self.last.copied_bytes=bytes;cursor.advance(None).map_err(|_|invalid("layout frame sort"))?;},HeapSortWork::Complete=>self.phase=9}},
   _=>return Err(invalid("layout frame plan phase"))
  }Ok(self.ready())
 }
 pub fn next(&mut self,page:usize)->Result<FramePlanTurn,String>{if !self.ready(){return Err("layout-export-frame-plan-not-ready".into());}if self.target.is_some_and(|target|target!=page)||self.output_page.is_some_and(|previous|page<previous){return Err("layout-export-frame-plan-page-order".into());}self.output_page=Some(page);let Some(row)=self.rows.get(&self.output)else{return Ok(FramePlanTurn::Complete)};if row.address.page_index>page{return Ok(FramePlanTurn::Complete);}self.output+=1;if row.address.page_index<page{return Ok(FramePlanTurn::Pending);}Ok(FramePlanTurn::Frame(row.address))}
 pub fn cancel(&mut self){self.cancelled=true;self.metadata.cancel();self.source=None;}
}
semio_framework_value::artifact_retire_leaf!(FramePlanAddress,Row,SortState);
semio_framework_value::artifact_retire_struct!(FramePlanState{metadata,rows,sort,source,target,phase,owner,item,page,parent,frame,row_phase,rank,over,link,output,output_page,cancelled,last});
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

impl super::LayoutExportJob{
 pub(super) fn prepare_frame_plan(&mut self,snapshot:&LayoutSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if cx.is_cancelled()||cx.should_yield(){return Ok(false);}if self.frame_plan.is_none(){let target=if self.request.as_ref().is_some_and(|request|request.kind==super::LayoutExportKind::Pdf){None}else{self.target_page};self.frame_plan=Some(FramePlanState::new(target));}let state=self.frame_plan.as_mut().unwrap();if state.ready(){return Ok(true);}cx.set_stage("layout.export.frame-addresses");let result=state.prepare(snapshot,cx.retained_grant());cx.consume_retained(state.last_receipt())?;cx.consume_fuel(1);result.map_err(Into::into)}
 pub(super) fn frame_plan_demand(&self,snapshot:&LayoutSnapshot)->Result<RetirementDemand,ValueError>{self.frame_plan.as_ref().map_or(Ok(RetirementDemand{depth:1,..Default::default()}),|state|state.demand(snapshot))}
}
