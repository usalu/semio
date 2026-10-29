//! 🧷️ Original projections lend exact authored insertion or original baseline content spans.
use crate::{LayoutMutationSource,LayoutMutationView as LayoutMutation};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value::retained_clone::RetainedCloneRef;
use crate::story_text::{OriginalTextSource,OriginalTextSpan};
pub fn story_id<'a,M:LayoutMutationSource>(m:RetainedCloneRef<'a,M>)->RetainedCloneRef<'a,String>{m.project(8810,|m|match m.view(){LayoutMutation::EditStory(p)=>&p.id,_=>unreachable!("original story command")})}
pub fn story_content<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,String>{b.project(5,|b|&b.stories).project(at,move|r|&r[at]).project(1,|s|&s.content)}
fn insertion<'a,M:LayoutMutationSource>(m:RetainedCloneRef<'a,M>)->RetainedCloneRef<'a,String>{m.project(8811,|m|match m.view(){LayoutMutation::EditStory(p)=>&p.insert,_=>unreachable!("original story command")})}
pub struct SpliceTextSource<'a,M:LayoutMutationSource>{pub m:RetainedCloneRef<'a,M>,pub b:RetainedCloneRef<'a,LayoutSnapshot>,pub story:usize,pub range:[usize;2],pub inverse:bool}
impl<M:LayoutMutationSource> OriginalTextSource for SpliceTextSource<'_,M>{type Mutation=M;fn mutation(&self)->RetainedCloneRef<'_,M>{self.m}fn baseline(&self)->RetainedCloneRef<'_,LayoutSnapshot>{self.b}fn identity(&self)->usize{8820+usize::from(self.inverse)}fn piece_count(&self)->usize{if self.inverse{1}else{3}}fn piece(&self,index:usize)->OriginalTextSpan<'_>{let old=story_content(self.b,self.story);if self.inverse{return OriginalTextSpan{source:old,start:self.range[0],end:self.range[1]}}match index{0=>OriginalTextSpan{source:old,start:0,end:self.range[0]},1=>{let source=insertion(self.m);OriginalTextSpan{source,start:0,end:source.get().len()}},2=>OriginalTextSpan{source:old,start:self.range[1],end:old.get().len()},_=>unreachable!("original splice span ordinal")}}}
