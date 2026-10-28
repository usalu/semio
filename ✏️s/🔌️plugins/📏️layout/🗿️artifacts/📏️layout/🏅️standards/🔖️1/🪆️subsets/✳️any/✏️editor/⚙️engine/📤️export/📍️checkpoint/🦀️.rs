//! 📍️ Replay milestones identify committed export effects independently of cooperative preparation turns.
#[derive(Clone,Copy,PartialEq,Eq)]pub(super) struct ExportMilestone{pub stage:u8,pub raw_bytes:usize,pub encoded_bytes:usize,pub published_chunks:usize}
pub(super) fn committed_milestone(before:ExportMilestone,after:ExportMilestone)->bool{before!=after}
pub(super) fn replay_position(current:u64,target:u64,same_prefix:bool,finished:bool)->Result<bool,&'static str>{if current<target&&!finished{return Ok(false);}if current!=target||!same_prefix{return Err("layout-export-checkpoint-replay-mismatch");}Ok(true)}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
