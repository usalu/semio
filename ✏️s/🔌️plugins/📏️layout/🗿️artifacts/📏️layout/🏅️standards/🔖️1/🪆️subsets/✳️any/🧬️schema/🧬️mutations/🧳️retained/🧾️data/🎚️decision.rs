//! 🎚️ Optional dictionary equality borrows exact authored and baseline projections.
use crate::{LayoutMutationSource,LayoutMutationView as LayoutMutation};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value_dictionary::{QuestionDictionary,QuestionDictionaryEqualityCursor};
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep}};
use crate::simple::SimpleAdvance;
pub fn authored<M:LayoutMutationSource>(m:RetainedCloneRef<'_,M>)->RetainedCloneRef<'_,Option<QuestionDictionary>>{m.project(0xda7a,|m|match m.view(){LayoutMutation::ChangeDataFields(p)=>&p.new_fields,_=>unreachable!("typed data-field command")})}
pub fn baseline(b:RetainedCloneRef<'_,LayoutSnapshot>)->RetainedCloneRef<'_,Option<QuestionDictionary>>{b.project(11,|b|&b.data_fields)}
pub struct DataFieldsDecision{equality:QuestionDictionaryEqualityCursor}
impl Default for DataFieldsDecision{fn default()->Self{Self{equality:QuestionDictionaryEqualityCursor::new()}}}
impl DataFieldsDecision{
 pub fn demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{self.equality.demand(authored(m),baseline(b),body)}
 pub fn advance<M:LayoutMutationSource>(&mut self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,g:RetainedCloneGrant)->Result<SimpleAdvance,ValueError>{let step=self.equality.advance(authored(m),baseline(b),g)?;Ok(SimpleAdvance{progress:step.progress,done:step.equal.is_some(),unchanged:step.equal==Some(true),error:None})}
 pub fn begin_close(&mut self){self.equality.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.equality.close_demand(body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.equality.close_step(g)}
 pub fn terminal_is_empty(&self)->bool{self.equality.terminal_is_empty()}
}
