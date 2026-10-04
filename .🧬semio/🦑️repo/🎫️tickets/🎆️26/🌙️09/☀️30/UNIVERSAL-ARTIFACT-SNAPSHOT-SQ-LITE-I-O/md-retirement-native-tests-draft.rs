#[cfg(test)]
std::thread_local!{
 static RETIRE_ALLOCATION_TRACKING:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static RETIRE_ALLOCATION_REFUSAL:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
 static RETIRE_ALLOCATION_ATTEMPTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static RETIRE_LIVE_BYTES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static RETIRE_VISITS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
}
#[cfg(test)]
fn retirement_test_visit(){RETIRE_VISITS.with(|count|count.set(count.get()+1));}
#[cfg(test)]
struct RetireObservedAllocator;
#[cfg(test)]
fn refuse_retirement_allocation()->bool{if RETIRE_ALLOCATION_REFUSAL.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_ALLOCATION_ATTEMPTS.try_with(|count|count.set(count.get()+1));true}else{false}}
#[cfg(test)]
fn retirement_live_add(bytes:usize){if RETIRE_ALLOCATION_TRACKING.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_LIVE_BYTES.try_with(|live|live.set(live.get()+bytes));}}
#[cfg(test)]
fn retirement_live_remove(bytes:usize){if RETIRE_ALLOCATION_TRACKING.try_with(std::cell::Cell::get).unwrap_or(false){let _=RETIRE_LIVE_BYTES.try_with(|live|live.set(live.get()-bytes));}}
#[cfg(test)]
unsafe impl std::alloc::GlobalAlloc for RetireObservedAllocator{
 unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};if !result.is_null(){retirement_live_add(layout.size());}result}
 unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System,layout)};if !result.is_null(){retirement_live_add(layout.size());}result}
 unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{if refuse_retirement_allocation(){return std::ptr::null_mut()}let result=unsafe{std::alloc::GlobalAlloc::realloc(&std::alloc::System,pointer,layout,size)};if !result.is_null(){retirement_live_remove(layout.size());retirement_live_add(size);}result}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){retirement_live_remove(layout.size());unsafe{std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)}}
}
#[cfg(test)]
#[global_allocator]
static RETIRE_OBSERVED_ALLOCATOR:RetireObservedAllocator=RetireObservedAllocator;

#[test]
fn sqlite_snapshot_md_retirement_releases_deep_and_wide_literals_when_all_allocations_refuse(){
 for case in 0..3{let(owned_bytes,remaining_bytes,attempts,visits,maximum_visits)=std::thread::Builder::new().stack_size(64*1024).spawn(move||{
  RETIRE_ALLOCATION_TRACKING.with(|flag|flag.set(true));let(blocks,inlines,maximum_visits)=match case{
   0=>{let mut inline=MdInline::Text{text:"retained text 世界".into()};for _ in 0..8192{inline=MdInline::Link{text:vec![inline],url:"literal URL 世界".into(),title:Some("literal title\0".into())};}(Vec::new(),vec![inline],2*(8192+1))},
   1=>{let mut block=MdBlock::CodeBlock{info:Some("literal info\0".into()),literal:"literal code 世界".into()};for depth in 0..8192{block=if depth%2==0{MdBlock::BlockQuote{blocks:vec![block,MdBlock::HtmlBlock{raw:"retained sibling 世界".into()}]}}else{MdBlock::List{ordered:true,start:Some(u32::MAX),tight:false,items:vec![Vec::new(),vec![block],Vec::new()]}};}(vec![block],Vec::new(),8*(8192+1)+4*(3*4096))},
   _=>{let items=(0..4096).map(|index|if index%2==0{Vec::new()}else{vec![MdBlock::Paragraph{inlines:vec![MdInline::Image{alt:"literal alt\0".into(),url:"literal image URL".into(),title:Some(String::new())},MdInline::Code{literal:"literal code 世界".into()},MdInline::HtmlInline{raw:"literal HTML\0".into()}]}]}).collect();(vec![MdBlock::List{ordered:false,start:None,tight:true,items}],Vec::new(),8*(1+4*2048)+4*4096)}
  };let mut blocks=blocks;if !inlines.is_empty(){blocks.push(MdBlock::Paragraph{inlines});}let snapshot=MdSnapshot{schema:"retained literal schema 世界\0".into(),blocks};let owned=RETIRE_LIVE_BYTES.with(std::cell::Cell::get);RETIRE_ALLOCATION_ATTEMPTS.with(|count|count.set(0));RETIRE_VISITS.with(|count|count.set(0));RETIRE_ALLOCATION_REFUSAL.with(|flag|flag.set(true));<MdSnapshot as store::ArtifactSqliteSnapshot>::retire_sqlite_snapshot(snapshot);RETIRE_ALLOCATION_REFUSAL.with(|flag|flag.set(false));let remaining=RETIRE_LIVE_BYTES.with(std::cell::Cell::get);RETIRE_ALLOCATION_TRACKING.with(|flag|flag.set(false));(owned,remaining,RETIRE_ALLOCATION_ATTEMPTS.with(std::cell::Cell::get),RETIRE_VISITS.with(std::cell::Cell::get),maximum_visits)
 }).unwrap().join().unwrap();assert!(owned_bytes>0);assert_eq!(remaining_bytes,0,"every owned string/vector must deallocate in case {case}");assert_eq!(attempts,0,"retirement must not request any allocation in case {case}");assert!(visits<=maximum_visits,"case {case} exceeds linear work frontier: {visits}>{maximum_visits}");}
}
