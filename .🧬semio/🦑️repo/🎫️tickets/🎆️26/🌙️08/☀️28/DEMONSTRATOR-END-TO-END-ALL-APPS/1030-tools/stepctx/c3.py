import glob
from rep import rep
G=glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
rep(G+'/⚙️compute/🎟️original/🦀️.rs',[
("impl WidgetJob for OriginalFixedOutputJob{fn step(&mut self,_fuel:usize)->WidgetStep{WidgetStep::Working{progress:self.progress()}}fn cancel(&mut self){self.state.cancelled=true}fn step_on_original(&mut self,cx:&mut StepContext<'_>)->Result<Option<WidgetStep>,OriginalTurnFailure>{self.step_original(cx)}}",
 "impl WidgetJob for OriginalFixedOutputJob{fn cancel(&mut self){self.state.cancelled=true}fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<WidgetStep>,OriginalTurnFailure>{self.step_original(cx)}}")])
