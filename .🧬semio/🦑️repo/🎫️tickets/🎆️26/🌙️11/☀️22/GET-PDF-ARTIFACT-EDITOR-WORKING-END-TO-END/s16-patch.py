import sys,re
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs"
s=open(P,encoding='utf8').read()
def rep(old,new,count=1):
    global s
    n=s.count(old)
    if n!=count: sys.exit(f"expected {count} got {n}: {old[:90]}")
    s=s.replace(old,new)

# consts
a=s.index("/// 📚️ New symbols wait"); b=s.index("#[derive(Clone, Copy)]\n#[derive(Debug)]\nstruct Symbol")
s=s[:a]+'''/// 🪣️ Bucket heads one funded row holds: sixteen four-byte slots, the sixty-four bytes of one bounded event.
const BUCKET_ROW: usize = 16;
/// 🪣️ Rows of the fixed chain index; 256 rows of 16 heads make the 4,096 buckets that never grow or rehash.
const BUCKET_ROWS: usize = 256;
/// 🪣️ Buckets of the chain index.
const BUCKETS: usize = BUCKET_ROW * BUCKET_ROWS;
/// 🔗️ End of a symbol chain.
const NONE: u32 = u32::MAX;
/// 🔗️ A lookup whose bucket head is not read yet.
const UNREAD: u32 = u32::MAX - 1;
/// #️⃣ FNV-1a 32-bit offset basis, the resumable text hash of the chain index.
const FNV_OFFSET: u32 = 0x811c_9dc5;
/// #️⃣ FNV-1a 32-bit prime.
const FNV_PRIME: u32 = 0x0100_0193;

'''+s[b:]
rep("key: Option<usize>, rank: usize }\n#[derive(Clone, Copy, Debug)]\nstruct ChunkLocator","key: Option<usize>, rank: usize, hash: u32, next: u32 }\n#[derive(Clone, Copy, Debug)]\nstruct ChunkLocator")
rep("semio_framework_value::artifact_retire_leaf!(Symbol);","""semio_framework_value::artifact_retire_leaf!(Symbol);
#[derive(Clone, Copy)]
struct Bucket([u32; BUCKET_ROW]);
semio_framework_value::artifact_retire_leaf!(Bucket);
/// 🔀️ Bottom-up merge sort cursor: runs of `width` merge from the order into the symbol links as scratch and back, one slot per event.
#[derive(Clone, Copy, Default)]
struct Sort { width: u32, start: u32, mid: u32, end: u32, a: u32, b: u32, k: u32, flip: bool, take: u8 }""")
rep("enum Phase { Initialize, Discover, Find, Reserve, Insert, MergeReserve, MergePush, MergeFind, MergeMove, Select,","enum Phase { Initialize, Buckets, Discover, Hash, Probe, Find, Reserve, Order, Merge, SortBack, Select,")
rep("    tail: [u32; TAIL], tail_len: usize, in_tail: bool, merge_j: usize, merge_hi: usize, merge_base: usize, merge_pos: usize, move_top: usize, merge_final: bool,\n    note: Option<Symbol>, search: usize, compare_offset: usize, compare_left: Option<u8>, insertion: usize,",
"    buckets: PagedList<Bucket, BUCKET_ROWS>, hash: u32, probe: u32, sort: Sort,\n    note: Option<Symbol>, compare_offset: usize, compare_left: Option<u8>,")
rep("tail: [0; TAIL], tail_len: 0, in_tail: false, merge_j: 0, merge_hi: 0, merge_base: 0, merge_pos: 0, move_top: 0, merge_final: false, note: None, search: 0, compare_offset: 0, compare_left: None, insertion: 0,","buckets: PagedList::empty(), hash: 0, probe: NONE, sort: Sort::default(), note: None, compare_offset: 0, compare_left: None,")

# terminal / capacity
rep("self.order.terminal_is_empty() && self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty()&&self.expression.terminal_is_empty() }","self.order.terminal_is_empty() && self.buckets.terminal_is_empty() && self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty()&&self.expression.terminal_is_empty() }")
rep("else if self.phase==Phase::MergeReserve&&self.order.len()+self.tail_len>self.order.capacity(){self.order.next_allocation_bytes().map_err(page_error)} else { Ok(0) } }",
"else if self.phase==Phase::Buckets&&self.buckets.len()<BUCKET_ROWS&&!self.buckets.has_reserved_slot(){self.buckets.next_allocation_bytes().map_err(page_error)}else if self.phase==Phase::Order&&self.order.len()<self.symbols.len()&&!self.order.has_reserved_slot(){self.order.next_allocation_bytes().map_err(page_error)} else { Ok(0) } }")
rep("            Phase::MergeReserve if self.order.len()+self.tail_len>self.order.capacity() => self.order.reserve_one_work_demand(),",
"            Phase::Buckets if self.buckets.len()<BUCKET_ROWS&&!self.buckets.has_reserved_slot() => self.buckets.reserve_one_work_demand(),\n            Phase::Order if self.order.len()<self.symbols.len()&&!self.order.has_reserved_slot() => self.order.reserve_one_work_demand(),")
# depth
rep("            let backing=self.symbols.next_pop_depth_demand().map_err(page_error)?.max(self.order.next_pop_depth_demand().map_err(page_error)?).checked_add(1)","            let backing=self.symbols.next_pop_depth_demand().map_err(page_error)?.max(self.order.next_pop_depth_demand().map_err(page_error)?).max(self.buckets.next_pop_depth_demand().map_err(page_error)?).checked_add(1)",2)
rep("            Phase::Find if self.search<self.insertion=>slot_depth(self.slot_at((self.search+self.insertion)/2))?.max(self.note.ok_or_else(||invalid(\"Pack depth lost pending text locator\"))?.depth+1),\n",
"""            Phase::Buckets=>if self.buckets.len()==BUCKET_ROWS{1}else if self.buckets.has_reserved_slot(){self.buckets.next_pop_depth_demand().map_err(page_error)?+1}else{self.buckets.next_reserve_depth_demand().map_err(page_error)?+1},
            Phase::Hash=>self.note.ok_or_else(||invalid("Pack depth lost pending text locator"))?.depth.checked_add(1).ok_or_else(||invalid("Pack symbol source depth overflow"))?,
            Phase::Probe=>if self.probe==NONE||self.probe==UNREAD{self.buckets.next_pop_depth_demand().map_err(page_error)?+1}else{slot_depth(self.probe)?},
            Phase::Find=>slot_depth(self.probe)?.max(self.note.ok_or_else(||invalid("Pack depth lost pending text locator"))?.depth+1),
""")
rep("            Phase::Insert=>1,\n            Phase::MergeReserve=>if self.order.len()+self.tail_len>self.order.capacity(){self.order.next_reserve_depth_demand().map_err(page_error)?+1}else{1},\n            Phase::MergePush|Phase::MergeMove=>self.order.next_pop_depth_demand().map_err(page_error)?+1,\n            Phase::MergeFind if self.search<self.insertion=>slot_depth(self.tail[self.merge_j])?.max(slot_depth(self.order[(self.search+self.insertion)/2])?),\n",
"""            Phase::Order=>if self.order.len()==self.symbols.len(){1}else if self.order.has_reserved_slot(){self.order.next_pop_depth_demand().map_err(page_error)?+1}else{self.order.next_reserve_depth_demand().map_err(page_error)?+1},
            Phase::SortBack=>self.symbols.next_pop_depth_demand().map_err(page_error)?.max(self.order.next_pop_depth_demand().map_err(page_error)?)+1,
            Phase::Merge=>{let sort=self.sort;let(a,b)=(sort.a as usize,sort.b as usize);let(mid,end)=(sort.mid as usize,sort.end as usize);if sort.take!=0||a==mid||b==end{self.symbols.next_pop_depth_demand().map_err(page_error)?.max(self.order.next_pop_depth_demand().map_err(page_error)?)+1}else{slot_depth(self.sort_get(a))?.max(slot_depth(self.sort_get(b))?)}},
""")
rep("Phase::Reserve=>if !self.symbols.has_reserved_slot(){self.symbols.next_reserve_depth_demand()}else{self.symbols.next_pop_depth_demand()}.map_err(page_error)?.checked_add(1).ok_or_else(||invalid(\"Pack symbol reservation depth overflow\"))?,",
"Phase::Reserve=>if !self.symbols.has_reserved_slot(){self.symbols.next_reserve_depth_demand()}else{self.symbols.next_pop_depth_demand()}.map_err(page_error)?.max(self.buckets.next_pop_depth_demand().map_err(page_error)?).checked_add(1).ok_or_else(||invalid(\"Pack symbol reservation depth overflow\"))?,")
# min copy
rep("Phase::Reserve=>size_of::<Symbol>(),Phase::Insert=>(TAIL+1)*size_of::<u32>(),Phase::MergeReserve if self.order.len()+self.tail_len>self.order.capacity()=>self.order.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::MergePush=>size_of::<u32>(),Phase::MergeMove=>MOVE_BLOCK*size_of::<u32>(),",
"""Phase::Reserve=>size_of::<Symbol>()+2*size_of::<u32>(),Phase::Buckets if self.buckets.len()==BUCKET_ROWS=>0,Phase::Buckets if !self.buckets.has_reserved_slot()=>self.buckets.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::Buckets=>size_of::<Bucket>(),Phase::Probe=>2*size_of::<u32>(),Phase::Order if self.order.len()==self.symbols.len()=>0,Phase::Order if !self.order.has_reserved_slot()=>self.order.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::Order=>size_of::<u32>(),Phase::SortBack=>if self.symbol_index==self.symbols.len(){0}else{size_of::<u32>()},Phase::Merge=>{let sort=self.sort;let(a,b)=(sort.a,sort.b);if a==sort.mid&&b==sort.end{0}else if sort.take!=0||a==sort.mid||b==sort.end{size_of::<u32>()}else{1}},""")
rep("Phase::Find | Phase::MergeFind | Phase::SymbolBytes | Phase::Body => 1, _ => 0 }","Phase::Hash | Phase::Find | Phase::SymbolBytes | Phase::Body => 1, _ => 0 }")
# close ladder
rep("else if !self.order.terminal_is_empty(){if !self.order.is_empty(){Ok(0)}else{self.order.next_release_copy_byte_demand().map_err(page_error)}}else if !self.frames.is_empty()","else if !self.order.terminal_is_empty(){if !self.order.is_empty(){Ok(0)}else{self.order.next_release_copy_byte_demand().map_err(page_error)}}else if !self.buckets.terminal_is_empty(){if !self.buckets.is_empty(){Ok(0)}else{self.buckets.next_release_copy_byte_demand().map_err(page_error)}}else if !self.frames.is_empty()")
rep("else if !self.order.terminal_is_empty(){if !self.order.is_empty(){Ok(0)}else{self.order.next_release_allocation_bytes().map_err(page_error)}}else if !self.frames.is_empty()","else if !self.order.terminal_is_empty(){if !self.order.is_empty(){Ok(0)}else{self.order.next_release_allocation_bytes().map_err(page_error)}}else if !self.buckets.terminal_is_empty(){if !self.buckets.is_empty(){Ok(0)}else{self.buckets.next_release_allocation_bytes().map_err(page_error)}}else if !self.frames.is_empty()")
rep("else if !self.order.terminal_is_empty(){if !self.order.is_empty(){self.order.next_pop_depth_demand()}else{self.order.next_release_depth_demand()}}else if !self.frames.is_empty()","else if !self.order.terminal_is_empty(){if !self.order.is_empty(){self.order.next_pop_depth_demand()}else{self.order.next_release_depth_demand()}}else if !self.buckets.terminal_is_empty(){if !self.buckets.is_empty(){self.buckets.next_pop_depth_demand()}else{self.buckets.next_release_depth_demand()}}else if !self.frames.is_empty()")
rep("        else if !self.frames.is_empty(){self.frames.pop();}\n","        else if !self.buckets.is_empty() { self.buckets.pop(); }\n        else if !self.buckets.terminal_is_empty() { let required = self.buckets.next_release_allocation_bytes().map_err(page_error)?; if required > grant.maximum_release_bytes { return Ok(Default::default()); } progress.released_bytes = release_list(&mut self.buckets,effect)?; }\n        else if !self.frames.is_empty(){self.frames.pop();}\n")
rep("self.order.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty(), \"borrowed Pack symbol/frame","self.order.terminal_is_empty()&&self.buckets.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty(), \"borrowed Pack symbol/frame")
rep("order:PagedList<u32,SYMBOL_LIMIT>,frames:PagedList<Frame,64>,chunks","order:PagedList<u32,SYMBOL_LIMIT>,buckets:PagedList<Bucket,BUCKET_ROWS>,frames:PagedList<Frame,64>,chunks")
# pop
rep("self.phase = if self.phase == Phase::Discover { if self.tail_len > 0 { self.merge_final = true; self.merge_base = self.order.len(); Phase::MergeReserve } else { Phase::Select } } else { Phase::Complete };","self.phase = if self.phase == Phase::Discover { Phase::Order } else { Phase::Complete };")
# helpers replacing slot_at
rep("    fn slot_at(&self, index: usize) -> u32 { if self.in_tail { self.tail[index] } else { self.order[index] } }\n",
'''    fn lookup(&mut self) { self.phase = Phase::Hash; self.hash = FNV_OFFSET; self.symbol_offset = 0; self.compare_offset = 0; self.compare_left = None; self.compare_readers = Default::default(); }
    fn head(&self, hash: u32) -> Result<u32, ValueError> { let at = hash as usize & (BUCKETS - 1); self.buckets.get(at / BUCKET_ROW).map(|row| row.0[at % BUCKET_ROW]).ok_or_else(|| invalid("Pack bucket index lost its row")) }
    fn sort_get(&self, index: usize) -> u32 { if self.sort.flip { self.symbols[index].next } else { self.order[index] } }
    fn sort_run(&mut self, start: usize) { let width = self.sort.width as usize; let n = self.symbols.len(); let (mid, end) = ((start + width).min(n), (start + 2 * width).min(n)); self.sort = Sort { start: start as u32, mid: mid as u32, end: end as u32, a: start as u32, b: mid as u32, k: start as u32, take: 0, ..self.sort }; }
    fn sort_take(&mut self, right: bool) { let from = if right { self.sort.b } else { self.sort.a } as usize; let value = self.sort_get(from); let to = self.sort.k as usize; if self.sort.flip { self.order[to] = value; } else { self.symbols[to].next = value; } self.sort.k += 1; if right { self.sort.b += 1; } else { self.sort.a += 1; } self.sort.take = 0; }
''')
# discover sites
rep("self.note = Some(Symbol { path: self.path, depth: self.depth, occurrences: 1, selected: false, forced: frame.force_text,key:None,rank:0 }); self.frames[self.depth].phase=1; self.pop(); self.phase = Phase::Find; self.search = 0; self.insertion = self.order.len(); self.in_tail = false; self.compare_offset = 0; self.compare_left = None; }",
"self.note = Some(Symbol { path: self.path, depth: self.depth, occurrences: 1, selected: false, forced: frame.force_text,key:None,rank:0,hash:0,next:NONE }); self.frames[self.depth].phase=1; self.pop(); self.lookup(); }")
rep("self.note=Some(Symbol{path:self.path,depth:self.depth,occurrences:1,selected:false,forced,key:Some(frame.index),rank:0});self.frames[self.depth].phase=31;self.frames[self.depth].count=0;self.phase=Phase::Find;self.search=0;self.insertion=self.order.len();self.in_tail=false;self.compare_offset=0;self.compare_left=None;}}",
"self.note=Some(Symbol{path:self.path,depth:self.depth,occurrences:1,selected:false,forced,key:Some(frame.index),rank:0,hash:0,next:NONE});self.frames[self.depth].phase=31;self.frames[self.depth].count=0;self.lookup();}}")
rep("if self.frames.len()==64{self.phase=Phase::Discover;}}},","if self.frames.len()==64{self.phase=Phase::Buckets;}}},")
# advance phases: replace from Phase::Find => { ... up to Phase::Select =>
a=s.index("            Phase::Find => {\n                let note"); b=s.index("            Phase::Select => {")
new='''            Phase::Buckets => {
                if self.buckets.len() == BUCKET_ROWS { self.phase = Phase::Discover; }
                else if !self.buckets.has_reserved_slot() {
                    if self.buckets.next_allocation_bytes().map_err(page_error)? > grant.maximum_capacity_bytes { return Ok(Default::default()); }
                    progress = reserve_list(&mut self.buckets, effect)?;
                } else { self.buckets.push_reserved(Bucket([NONE; BUCKET_ROW])).map_err(|_| invalid("Pack bucket row rejected"))?; progress.copied_bytes = size_of::<Bucket>(); }
            }
            Phase::Hash => {
                let note = self.note.ok_or_else(|| invalid("borrowed Pack lost its pending text locator"))?;
                let text = note.text(source)?;
                if self.symbol_offset == text.len() { self.note = Some(Symbol { hash: self.hash, ..note }); self.compare_readers = Default::default(); self.probe = UNREAD; self.phase = Phase::Probe; }
                else {
                    let mut block = [0u8; 64]; let count = (text.len() - self.symbol_offset).min(maximum);
                    let Poll::Ready(copied) = self.compare_readers[0].copy_step(text, self.symbol_offset, &mut block[..count])? else { return Ok(BorrowedProjectedPackProgress { progress, ..Default::default() }); };
                    for byte in &block[..copied] { self.hash = (self.hash ^ u32::from(*byte)).wrapping_mul(FNV_PRIME); }
                    self.symbol_offset += copied; progress.copied_bytes = copied;
                }
            }
            Phase::Probe => {
                let note = self.note.ok_or_else(|| invalid("borrowed Pack lost its pending text locator"))?;
                if self.probe == UNREAD { self.probe = self.head(note.hash)?; progress.copied_bytes = size_of::<u32>(); }
                else if self.probe == NONE { self.phase = Phase::Reserve; }
                else {
                    let candidate = self.symbols[self.probe as usize]; progress.copied_bytes = 2 * size_of::<u32>();
                    if candidate.hash == note.hash && candidate.text(source)?.len() == note.text(source)?.len() { self.compare_offset = 0; self.compare_left = None; self.compare_readers = Default::default(); self.phase = Phase::Find; } else { self.probe = candidate.next; }
                }
            }
            Phase::Find => {
                let note = self.note.ok_or_else(|| invalid("borrowed Pack lost its pending text locator"))?;
                let slot = self.probe as usize;
                let left = note.text(source)?; let right = self.symbols[slot].text(source)?;
                let Some((order, bytes)) = self.compare(left, right, maximum)? else { return Ok(Default::default()); };
                progress.copied_bytes = bytes;
                if let Some(order) = order { if order == Ordering::Equal { self.symbols[slot].occurrences = self.symbols[slot].occurrences.checked_add(1).ok_or_else(|| invalid("Pack symbol occurrence overflow"))?; self.symbols[slot].forced |= note.forced; self.note = None; self.phase = Phase::Discover; } else { self.probe = self.symbols[slot].next; self.phase = Phase::Probe; } }
            }
            Phase::Reserve => {
                if !self.symbols.has_reserved_slot() {
                    let required = self.symbols.next_allocation_bytes().map_err(page_error)?;
                    if required > grant.maximum_capacity_bytes { return Ok(Default::default()); }
                    progress=reserve_list(&mut self.symbols,effect)?;
                } else {
                    let note = self.note.ok_or_else(|| invalid("Pack insertion lost text locator"))?;
                    let slot = u32::try_from(self.symbols.len()).ok().filter(|slot| *slot < UNREAD).ok_or_else(|| invalid("Pack symbol table exceeds its chain index"))?;
                    let at = note.hash as usize & (BUCKETS - 1);
                    let next = self.head(note.hash)?;
                    self.symbols.push_reserved(Symbol { next, ..note }).map_err(|_| invalid("Pack symbol reserved slot rejected"))?;
                    self.buckets[at / BUCKET_ROW].0[at % BUCKET_ROW] = slot;
                    progress.copied_bytes = size_of::<Symbol>() + 2 * size_of::<u32>(); self.note = None; self.phase = Phase::Discover;
                }
            }
            Phase::Order => {
                let n = self.symbols.len();
                if self.order.len() == n { if n < 2 { self.phase = Phase::Select; } else { self.sort = Sort { width: 1, ..Default::default() }; self.sort_run(0); self.phase = Phase::Merge; } }
                else if !self.order.has_reserved_slot() {
                    if self.order.next_allocation_bytes().map_err(page_error)? > grant.maximum_capacity_bytes { return Ok(Default::default()); }
                    progress = reserve_list(&mut self.order, effect)?;
                } else {
                    let first = self.order.len(); let count = (n - first).min(BUCKET_ROW).min(maximum / size_of::<u32>());
                    let mut block = [0u32; BUCKET_ROW]; for (offset, slot) in block[..count].iter_mut().enumerate() { *slot = (first + offset) as u32; }
                    progress.copied_bytes = self.order.extend_reserved_from_slice(&block[..count]) * size_of::<u32>();
                }
            }
            Phase::Merge => {
                let sort = self.sort; let (a, b, mid, end) = (sort.a as usize, sort.b as usize, sort.mid as usize, sort.end as usize);
                if sort.take != 0 { self.sort_take(sort.take == 2); progress.copied_bytes = size_of::<u32>(); }
                else if a == mid && b == end {
                    if end >= self.symbols.len() {
                        let width = sort.width as usize * 2; self.sort.flip = !sort.flip;
                        if width >= self.symbols.len() { self.symbol_index = 0; self.phase = if self.sort.flip { Phase::SortBack } else { Phase::Select }; } else { self.sort.width = width as u32; self.sort_run(0); }
                    } else { self.sort_run(end); }
                }
                else if a == mid || b == end { self.sort_take(a == mid); progress.copied_bytes = size_of::<u32>(); }
                else {
                    let left = self.symbols[self.sort_get(a) as usize].text(source)?; let right = self.symbols[self.sort_get(b) as usize].text(source)?;
                    let Some((order, bytes)) = self.compare(left, right, maximum)? else { return Ok(Default::default()); };
                    progress.copied_bytes = bytes;
                    match order { Some(Ordering::Less) => self.sort.take = 1, Some(Ordering::Greater) => self.sort.take = 2, Some(Ordering::Equal) => return Err(invalid("Pack symbol table holds a duplicate text")), None => {} }
                    if self.sort.take != 0 && bytes + size_of::<u32>() <= maximum { self.sort_take(self.sort.take == 2); progress.copied_bytes += size_of::<u32>(); }
                }
            }
            Phase::SortBack => {
                let n = self.symbols.len();
                if self.symbol_index == n { self.symbol_index = 0; self.phase = Phase::Select; }
                else { let count = (n - self.symbol_index).min(BUCKET_ROW).min(maximum / size_of::<u32>()); for _ in 0..count { let index = self.symbol_index; self.order[index] = self.symbols[index].next; self.symbol_index += 1; } progress.copied_bytes = count * size_of::<u32>(); }
            }
'''
s=s[:a]+new+s[b:]
open(P,"w",encoding='utf8').write(s)
print("ok")
