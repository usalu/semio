/// 🛂️ Original caller policy bounds the complete source, cumulative storage and each declared collection extent.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JsonReadLimits{
    pub maximum_bytes:u64,
    pub maximum_allocation_bytes:usize,
    pub maximum_depth:usize,
    pub maximum_items:u64,
}
