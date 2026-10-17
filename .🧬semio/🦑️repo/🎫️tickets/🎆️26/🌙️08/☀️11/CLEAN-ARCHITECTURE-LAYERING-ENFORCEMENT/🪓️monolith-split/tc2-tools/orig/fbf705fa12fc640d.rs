//! 🚦️ A captured query observes one exact live geometry publication.
use super::SceneIdentity;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SceneAdmissionStatus{Ready,Pending,Stale,Failed,Unavailable}
/// 🔐️ An old picture can remain visible without granting authority to query it.
pub fn scene_admission(captured:SceneIdentity,live:Option<SceneIdentity>,visual:Option<SceneIdentity>,preparing:bool,failed:bool)->SceneAdmissionStatus{
 if captured.instance==0||live.is_some_and(|id|id.instance==0)||visual.is_some_and(|id|id.instance==0){return SceneAdmissionStatus::Unavailable;}
 if !live.is_some_and(|live|captured.matches(live)){return SceneAdmissionStatus::Stale;}
 if visual.is_some_and(|visual|captured.matches(visual)){return SceneAdmissionStatus::Ready;}
 if failed{SceneAdmissionStatus::Failed}else if preparing{SceneAdmissionStatus::Pending}else{SceneAdmissionStatus::Unavailable}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
