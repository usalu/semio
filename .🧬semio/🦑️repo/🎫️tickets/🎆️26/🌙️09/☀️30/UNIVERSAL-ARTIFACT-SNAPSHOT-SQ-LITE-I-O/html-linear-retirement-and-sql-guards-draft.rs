//! 🧹️ Literal HTML cold retirement reuses owned vector slots without allocating.
/// 🪢️ Each continuation owns the vector containing its popped-node spare slot.
struct HtmlContinuation{work:Vec<HtmlNode>,previous:*mut HtmlContinuation}
const _:()=assert!(std::mem::size_of::<HtmlContinuation>()==std::mem::size_of::<Vec<HtmlNode>>()+std::mem::size_of::<*mut HtmlContinuation>());
const _:()=assert!(std::mem::size_of::<HtmlContinuation>()<=std::mem::size_of::<(String,Vec<HtmlAttr>,Vec<HtmlNode>)>());
const _:()=assert!(std::mem::size_of::<HtmlContinuation>()<=std::mem::size_of::<HtmlNode>()&&std::mem::align_of::<HtmlContinuation>()<=std::mem::align_of::<HtmlNode>());
pub(super) fn retire_node(root:HtmlNode){
 #[cfg(test)]super::sqlite_lifecycle_tests::retirement_visit();
 let HtmlNode::Element{children,..}=root else{return};let mut work=children;let mut previous=std::ptr::null_mut::<HtmlContinuation>();
 loop{
  if let Some(node)=work.pop(){
   #[cfg(test)]super::sqlite_lifecycle_tests::retirement_visit();
   if let HtmlNode::Element{children,..}=node{let slot=unsafe{work.as_mut_ptr().add(work.len())}.cast::<HtmlContinuation>();let frame=HtmlContinuation{work:std::mem::take(&mut work),previous};unsafe{std::ptr::write(slot,frame)}previous=slot;work=children;}
  }else if previous.is_null(){return}else{let frame=unsafe{std::ptr::read(previous)};previous=frame.previous;work=frame.work;}
 }
}
struct OwnedNodeMap(BTreeMap<i64,HtmlNode>);
impl std::ops::Deref for OwnedNodeMap{type Target=BTreeMap<i64,HtmlNode>;fn deref(&self)->&Self::Target{&self.0}}
impl std::ops::DerefMut for OwnedNodeMap{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}
impl Drop for OwnedNodeMap{fn drop(&mut self){for node in std::mem::take(&mut self.0).into_values(){super::controlled_native::retire_node(node)}}}
struct OwnedChildren(Vec<HtmlNode>);
impl std::ops::Deref for OwnedChildren{type Target=Vec<HtmlNode>;fn deref(&self)->&Self::Target{&self.0}}
impl std::ops::DerefMut for OwnedChildren{fn deref_mut(&mut self)->&mut Self::Target{&mut self.0}}
impl Drop for OwnedChildren{fn drop(&mut self){while let Some(node)=self.0.pop(){super::controlled_native::retire_node(node)}}}
