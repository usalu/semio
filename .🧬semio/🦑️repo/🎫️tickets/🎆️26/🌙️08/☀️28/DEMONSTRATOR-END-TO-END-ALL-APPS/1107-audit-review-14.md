# 1107 Audit review-14: callers of the piecewise ControlledRetirement demands

Read-only audit. No edits outside this report, no sub-agents, no cargo, no modifying git commands.
Paths below are relative to the repo root `/Users/ueli/Documents/semio/`. Line numbers are file:line.

## Change under audit

From `C/1107-result-rootfix.md`: `ControlledRetirement::next_copy_byte_demand`, `next_capacity_byte_demand(body)` and `next_release_byte_demand` now equal `next_demand(0|body).{copy_bytes,capacity_bytes,release_bytes}`.

Verified in `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs`:
- lines 144, 147, 150: inherent methods delegate to `next_demand`.
- lines 67-69 and 278-280: `ErasedSnapshotRetirement` and `ErasedControlledRetirement` forward to the inherent methods.
- lines 38, 112, 120-124: `next_demand` already includes the frame birth (`size_of::<Option<T>>()+size_of::<ControlledRetirement<T>>()`) and the cursor reserve quote. A caller that adds a frame or allocation size on top of the piecewise copy counts it twice.
- Copy and release are quoted at body 0; capacity at the given body. A caller that mixes copy(0) with capacity(body) only matches `next_demand(body)` when copy does not depend on body. Not checked per site (see class d).

## Scope and method

- Grep: `/usr/bin/grep -rn --include="*.rs"` over `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand`, excluding `node_modules`, `dist`, `target`, `.git`. Test code and wasm-gated code are included.
- Non-.rs files were not searched. A full-tree grep of `♻️mit-bestand` and `🌎️hub` for all file types was started and stopped before finishing.
- Results: 439 call sites and 683 forwarder definitions (`fn next_copy/capacity/release_byte_demand`).
- `♻️mit-bestand`: 0 .rs hits. `🌎️hub`: 4 hits (section Findings C).
- Scope note: the grep matches the method names only. Receiver types were not traced for each call site, so class (a) for call sites means "plain read or pass-through, no arithmetic, no workaround marker". It is not a per-site proof that the receiver is ControlledRetirement-backed.

## Classes

- (a) correct now: reads the new quote with no added bytes, or a forwarder that delegates unchanged.
- (b) double count: adds a header, frame or allocation quote that `next_demand` already includes. None confirmed.
- (c) per-site workaround that can be removed: a rebuild from `selected_demand` or from the pieces, a `.max()` floor or inflated grant, or a body hint used to widen a quote.
- (d) unsure: needs a trace or a run.

## Findings

### A. Forwarders and trait impls

- Pure delegations (e.g. `ControlledRetirement` erased impls, `ErasedSnapshotRetirement`, `RetirementCursor` forwarders): class a.
- `next_demand`-based forwarders (`self.next_demand(0|copy|b|body)`): class a, for example `🛫️encode/🫴️recipient/🦀️.rs:37-39`, `🛬️decode/🫴️recipient/🦀️.rs:86-88`, `🗑️discard/🦀️.rs:91-93` (`time-travel`), `♻️retirement/🔗️arc-custody/🦀️.rs:22-23`, `📝️text/📦️paged/🦀️.rs:332`.
- Time-travel discard `⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:113-115`: the piecewise copy comes from the residue owner, and the header copy is a separate phase, so there is no double count. Class a.
- Hard-coded body 4096: `🏪️store/🔗️read/📸️snapshot-alias/🦀️.rs:52-54` and `🏪️store/🔁️replay/🎮️operation/🌱️derived/🦀️.rs:231-233` quote copy and release at body 4096 through `next_demand(4096)`. Not body 0. Class d; a candidate workaround to check.
- Remaining forwarders (593): own-quote bodies that do not reference `next_demand` or `ControlledRetirement`. Probably unaffected by the change, but not traced to their owner. Class d.
- Forwarders with a self-computed header or allocation quote (`size_of::<...>()` in the body): `🖼️canvas/♻️retirement/➰️path/🦀️.rs:67`, `🔲️pixels/📸️jpeg/📥️decode/🦀️.rs:35`, `🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs:71`, `🌱️value/🛬️decode/🛂️allocation/🧪️tests/🦀️.rs:39`, `🔌️plugin/🧠️interpreter/🏃️runtime/🎛️operation/📥️input/🦀️.rs:15`. Class d: double count only if the owner is also wrapped by `ControlledRetirement`, which was not traced.

### B. Call sites: arithmetic that may double count

- `🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs:86`: `deflate.next_copy_byte_demand()? + compressor_bookkeeping_copy()`. Class d. Check whether the deflate copy or the bookkeeping already includes the header.
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🪜️base64/🦀️.rs:59`: `size_of::<Base64DecodeCursor>() + cursor.next_copy_byte_demand(available)`. Class d. The base64 cursor's own quote (`🚪️io/🔤️base64/🎮️cursor/🦀️.rs:107`) is not a ControlledRetirement; if it is not wrapped, this is correct.
- `🎒️pack/.../🎒️document/⏳️cursor/🦀️.rs:67`: `schema.next_depth_demand()? + 1`. Depth, not bytes. Class a.
- No other site adds a size_of or header to a piecewise quote.

### C. Call sites: workarounds that can be removed (class c, 59 sites)

- `selected_demand` rebuilds (5): `🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs:21`; `🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🔬️unit/🦀️.rs:442, 469, 494`; `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/⏱️tessellation/🦀️.rs:99`. Next: `next_demand(body)` directly.
- Floors and inflated copies (`.max(...)`, `4096.max(...)`, `7.max(...)`, `.max(256)`, `.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)`): 19 sites. Listed in the appendix with the `max` marker. Notable production sites: `🔌️plugin/♻️lifecycle/🦀️.rs:94` (`.max(grant.maximum_copy_bytes)`, inflates grant), `🏪️store/🧰️test-support/🦀️.rs:617`, `🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:211`, `🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs:463`, `📖️playbook/.../🧱️learning-program-and-geometry`, `🧊️3d/📐️brep/⚙️engine/🧪️tests/♻️retention/🦀️.rs:13`.
- `RetirementDemand { copy: next_copy(), capacity: next_capacity(body), release: next_release() }` rebuilds (35 sites): these equal `next_demand` fields except depth, and they can drop depth. Replace with `next_demand(body)`. Caveat: the copy here is quoted at body 0, which may differ from `next_demand(body)`'s copy. Examples: `✏️s/.../📖️playbook/.../learning-program-and-geometry/🦀️.rs:235, 266`; `🌱️value/♻️retirement/🧬️contract/🦀️.rs:140`; `🏪️store/🧩️composition/🚪️open/🦀️.rs:149`; `📡️spr/📜️history/🦀️.rs:208`.

Not located in grep: the history2, reviewfix9 and reviewfix3 workaround sites named in `C/1028-coordination.md` (lines 388-390) and `C/1105b-claims.md` (lines 1408 fold admission copy, 1446 fold birth `admission_copy`, 1399 file fixture copyBytes 8192, 1501 3d mesh `narrow()` helper, 1523 replication codec copy >= 1). Those names are claim notes, not code symbols. No `selected_demand` hits in `🏪️store`. Class d until located.

### D. Call sites: unsure (class d, 8 sites)

- `pack/.../🎒️document/⏳️cursor/🦀️.rs:86` (see B).
- `raster/.../🪜️base64/🦀️.rs:59` (see B).
- `🖼️canvas/♻️retirement/➰️path/🧪️tests/🦀️.rs:63`: asserts `next_copy_byte_demand() == size_of::<PathEl>()`. Will fail if the slot is a ControlledRetirement (copy now includes the frame).
- `🖼️canvas/♻️retirement/🖊️stroke/🧪️tests/🦀️.rs:50`: same kind of assert.
- `🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:114` and `:179`: `if copy == 0 { release } else { ... }` body hint. The `copy == 0` branch no longer runs for ControlledRetirement births.
- `✏️s/.../📐️cad/.../📎️references/🦀️.rs:86`: `next_capacity` hint.
- `🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🦀️.rs:103-105`: `next_capacity(if copy == 0 { release } else { copy })`. Body hint widened by the old zero copy. Candidate workaround.

### E. Hub and mit-bestand

- `🌎️hub/🧩️compositions/🀄️wfc/🧪️tests/🎟️mounted-owner-policy/🦀️.rs:55`: release quote check, class a.
- `🌎️hub/.../📏️layout/🖊️drawing/🦀️.rs:103-105`: class d (see D).
- `♻️mit-bestand`: no .rs hits.

### F. Tests that may now fail (not run)

Asserts of the form `piecewise == 0` or `== size_of` are at risk. The rootfix note already flags `🧪️tests/🦀️.rs` line 32 (rewritten). Others are listed in D. Not run, because this audit is read-only.

## Appendix A. Call sites (439), class per row

| class | file:line | snippet |
|---|---|---|
| a | 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛫️encode/🧪️tests/🦀️.rs:16 | `    let copied=cursor.next_copy_byte_demand(&writer,None).unwrap();if copied>0{let before=(cursor.total,cursor.depth,cur` |
| a | 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🦀️.rs:18 | ` let observed_copy=frame.next_copy_byte_demand().unwrap();let copy=f["terminalChild"]["copyBytes"].as_u64().unwrap()as u` |
| a | 🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/🔬️tessellation/📦️ownership/🦀️.rs:34 | `    while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=semio_framework_value::re` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🔗️merge-and-remap/🦀️.rs:77 | `                let copy=job.next_copy_byte_demand().expect("cold original reachability copy");let grant=RetainedCloneGr` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🔗️merge-and-remap/🦀️.rs:91 | `        let mut stall=0;while !job.terminal_is_empty(){let copy=job.next_copy_byte_demand(self).expect("cold original co` |
| c | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs:21 | `    while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let demand={let first=owner.selecte` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🧱️primitives/🧪️tests/🔬️topology-compaction/🦀️.rs:18 | `            let copy=job.next_copy_byte_demand(&body).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_cop` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🧱️primitives/🧪️tests/🔬️topology-compaction/🦀️.rs:25 | `        while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {m` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🧱️primitives/🧪️tests/🔬️topology-reachability/🦀️.rs:120 | `            let copy=job.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_byt` |
| c | 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/♻️retention/🦀️.rs:13 | `    let copy=grant.maximum_copy_bytes.max(job.next_copy_byte_demand(kernel).unwrap());` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/♻️retention/🦀️.rs:30 | `            let copy=job.next_copy_byte_demand(&kernel).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_co` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:333 | `    let copy=owner.next_copy_byte_demand().unwrap();semio_framework_value::retained_clone::RetainedCloneGrant {maximum_i` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:411 | `            let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_by` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:438 | `        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:459 | `        let copy=owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:498 | `        let copy = owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:508 | `                assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);` |
| a | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🦀️.rs:591 | `        let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes` |
| c | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🔬️unit/🦀️.rs:442 | `        while !closing.terminal_is_empty(){let copy=closing.next_copy_byte_demand().unwrap();let demand=closing.selected` |
| c | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🔬️unit/🦀️.rs:469 | `        while !close.terminal_is_empty(){let copy=close.next_copy_byte_demand().unwrap();let demand=close.selected_deman` |
| c | 🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🔬️unit/🦀️.rs:494 | `  while !close.terminal_is_empty(){let d=close.selected_demand(close.next_copy_byte_demand().unwrap()).unwrap();let g=Re` |
| a | 🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🧯️retirement-laws/🦀️.rs:25 | `assert_eq!(owner.next_capacity_byte_demand(3).unwrap(), grant.maximum_capacity_bytes);` |
| a | 🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🧯️retirement-laws/🦀️.rs:26 | `assert_eq!(owner.next_release_byte_demand().unwrap(), grant.maximum_release_bytes);` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🏭️schema/🧪️tests/🦀️.rs:48 | `   let capacity=cursor.next_capacity_byte_demand().unwrap();let copy=cursor.next_copy_byte_demand().unwrap();let depth=c` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/📦️operation-pages/🦀️.rs:439 | ` let demand=ErasedSnapshotRetirement::next_copy_byte_demand(&owner).unwrap();assert_eq!(demand,1);assert_eq!(ErasedSnaps` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs:43 | `  for _ in 0..20000{let capacity=cursor.next_capacity_byte_demand().unwrap();let(step,birth,free)=semio_framework_trace_` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs:57 | ` for _ in 0..100000{let capacity=cursor.next_capacity_byte_demand().unwrap();let(step,birth,free)=semio_framework_trace_` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs:119 | `                let demand=cursor.next_capacity_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs:137 | `            for _ in 0..stop { let demand=cursor.next_capacity_byte_demand().unwrap();let step=cursor.advance(&source,sp` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🦀️.rs:168 | `                    let demand=cursor.next_capacity_byte_demand().unwrap();let minimum=cursor.next_minimum_copy_bytes();` |
| a | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs:67 | `  Phase::Schema=>(self.schema.next_capacity_byte_demand()?,self.schema.next_copy_byte_demand()?,self.schema.next_depth_d` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs:86 | `  Phase::Segment=>(self.deflate.next_capacity_byte_demand()?,self.deflate.next_copy_byte_demand()?+self.compressor_bookk` |
| c | 🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs:37 | `        let copy = owner.next_copy_byte_demand().unwrap_or_else(\|error\| panic!("{label}: copy demand: {error}")).max(sto` |
| a | 🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs:41 | `            maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap_or_else(\|error\| panic!("{label}: capaci` |
| a | 🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs:42 | `            maximum_release_bytes: owner.next_release_byte_demand().unwrap_or_else(\|error\| panic!("{label}: release dema` |
| a | 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🦀️.rs:59 | ` fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🦀️.rs:60 | ` fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}` |
| a | 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🦀️.rs:61 | ` fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🧪️tests/🦀️.rs:11 | `    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: cursor.next_copy_byte_demand().unwrap(), maximum_capacity` |
| a | 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🪟️resolved-host-context/🦀️.rs:147 | `   let copy=owner.next_copy_byte_demand().unwrap().min(fixture["copyGrant"].as_u64().unwrap()as usize);let release=owner` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🧪️tests/🦀️.rs:63 | `                assert_eq!(slot.as_ref().unwrap().next_copy_byte_demand().unwrap(), size_of::<PathEl>());` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🧪️tests/🦀️.rs:86 | `                    let extent = slot.as_ref().unwrap().next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🦀️.rs:56 | `        let release = self.next_release_byte_demand()?;` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🦀️.rs:57 | `        let copy = self.next_copy_byte_demand()?;` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🧪️tests/🦀️.rs:32 | `            let first_copy = slot.as_ref().unwrap().next_copy_byte_demand().unwrap();` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🧪️tests/🦀️.rs:50 | `                if slot.as_ref().unwrap().next_copy_byte_demand().unwrap()==size_of::<f64>() {` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🧪️tests/🦀️.rs:61 | `                let release=slot.as_ref().unwrap().next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🦀️.rs:50 | `        let copy = self.next_copy_byte_demand()?;` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🦀️.rs:11 | `  let(demand,born,freed)=observe(\|\|owner.next_release_byte_demand().unwrap());assert_eq!((born,freed),(0,0));assert!(dem` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🦀️.rs:23 | `   for _ in 0..cut{let(demand,born,freed)=observe(\|\|cursor.next_capacity_byte_demand(source).unwrap());assert_eq!((born,` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🧪️tests/🦀️.rs:25 | `    let copy=cursor.next_copy_byte_demand(source).unwrap();if copy>0{let(step,born,freed)=observe(\|\|cursor.advance(sourc` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:160 | `  let capacity=self.next_capacity_byte_demand(source)?;` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:161 | `  if capacity>grant.maximum_capacity_bytes\|\|self.next_copy_byte_demand(source)?>grant.maximum_copy_bytes{return Ok(Retai` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:44 | `        let demand=cursor.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:49 | `            assert_eq!(cursor.next_release_byte_demand().unwrap(),demand);` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:68 | `                let capacity=cursor.next_capacity_byte_demand(&source).unwrap();` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:74 | `                    assert_eq!(cursor.next_capacity_byte_demand(&source).unwrap(),capacity);` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:115 | `            let capacity=cursor.next_capacity_byte_demand(&source).unwrap();` |
| c | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🧪️tests/🦀️.rs:116 | `            let copied=7.max(cursor.next_copy_byte_demand(&source).unwrap());` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:201 | `        let demand = self.next_capacity_byte_demand(source)?;` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:304 | `        let demand = self.next_release_byte_demand()?;` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:94 | `        let copied = self.next_copy_byte_demand();` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🧪️tests/🦀️.rs:145 | `            let copied=cursor.next_copy_byte_demand();let freed=cursor.next_release_byte_demand().unwrap();let depth=cur` |
| c | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/👁️borrow/🦀️.rs:28 | `            return Ok(RetirementDemand { copy_bytes: cursor.next_copy_byte_demand(&source[self.index])?, capacity_bytes:` |
| c | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:45 | `        Ok(RetirementDemand{copy_bytes:cursor.next_copy_byte_demand(source),capacity_bytes:cursor.next_capacity_byte_dem` |
| c | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:157 | `        Ok(RetirementDemand { copy_bytes: self.original.next_copy_byte_demand(), release_bytes: self.original.next_relea` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🦀️.rs:87 | `        let capacity = self.next_capacity_byte_demand(); let copy = self.next_copy_byte_demand(source); let release = se` |
| a | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🦀️.rs:99 | `            let release=(retirement.next_copy_byte_demand().unwrap()==0).then(\|\|grant.maximum_release_bytes.checked_sub(` |
| a | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🦀️.rs:100 | `            let zero_copy=(retirement.next_copy_byte_demand().unwrap()!=0).then_some(RetainedCloneGrant{maximum_copy_byt` |
| a | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🥣️companion/📏️retirement/🦀️.rs:45 | `        assert_eq!(job.next_capacity_byte_demand(7).unwrap(), birth);` |
| a | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🥣️companion/📏️retirement/🦀️.rs:52 | `            assert_eq!(job.next_capacity_byte_demand(7).unwrap(), birth);` |
| a | 🧰️framework/🔨️modules/🔲️pixels/♻️retirement/🧪️tests/🦀️.rs:16 | `  let bodies=[owner.next_copy_byte_demand().unwrap(),1<<16,owner.next_demand(1<<30).map_or(1<<16,\|demand\|demand.copy_byt` |
| a | 🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🗂️layers/🦀️.rs:225 | `            let copy=retired.next_copy_byte_demand().map_err(denied)?;let release=retired.next_release_byte_demand().map` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs:213 | `    while retirement.next_release_byte_demand().unwrap() <= release_grant.maximum_release_bytes {` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs:221 | `    let demand = retirement.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🧪️tests/🔬️unit/🦀️.rs:227 | `        assert_eq!(retirement.next_release_byte_demand().unwrap(), demand);` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🧪️tests/🦀️.rs:37 | `            let copy=close.next_copy_byte_demand().unwrap();let capacity=close.next_capacity_byte_demand(copy).unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🧪️tests/🦀️.rs:38 | `            let release=close.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:32 | `        if !self.is_empty(){return self.next_capacity_byte_demand(body);}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:73 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:74 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:75 | `    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:74 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:75 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:76 | `    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧪️tests/🗂️ordered/🦀️.rs:81 | `        turns+=1;assert!(turns<10000);let copy=owner.next_copy_byte_demand().unwrap();let capacity=owner.next_capacity_b` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧪️tests/🗂️ordered/🦀️.rs:142 | `                let demand=cursor.next_copy_byte_demand();` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:348 | `        if state.closing\|\|state.is_complete()\|\|copy<self.next_copy_byte_demand(){return Ok(0)}` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:503 | `            Some(Owner::Key(key))=>key.next_release_byte_demand(),` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:504 | `            Some(Owner::Value(value))=>value.next_release_byte_demand(),` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🎟️work/♻️retirement/🧪️tests/🦀️.rs:10 | `  let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand(),maximum_capacity_bytes:0,max` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🦀️.rs:29 | `    fn copy_demand(&self)->usize{self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🦀️.rs:84 | `    fn release_demand(&self)->Result<usize,ValueError>{Ok(self.owner.next_release_byte_demand())}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:90 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:92 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:93 | `    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| c | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:140 | `        Ok(RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🦀️.rs:14 | `    let quote=owner.next_capacity_byte_demand(4096).unwrap();let reserve_copy=owner.next_demand(4096).unwrap().copy_byte` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🦀️.rs:32 | `  assert_eq!(owner.next_copy_byte_demand().unwrap(),owner.next_demand(0).unwrap().copy_bytes);let root_copy=crate::retir` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:305 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:308 | `    fn next_close_byte_demand(&self) -> Option<usize> { self.next_release_byte_demand().ok() }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🧪️tests/🎮️ownership/🦀️.rs:40 | `            if g.maximum_release_bytes>0 && queue.next_copy_byte_demand().unwrap()==0 {let ((small,_),(a,r))=observe_ret` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🧪️tests/🎮️ownership/🦀️.rs:116 | `            if g.maximum_release_bytes>0&&owner.next_copy_byte_demand().unwrap()==0{let ((small,_),heap)=observe_retirem` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:66 | `            Some(owner)=>owner.next_release_byte_demand(),` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:138 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:140 | `    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🧪️tests/🦀️.rs:124 | `                let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_cop` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:106 | `    fn next_birth_bytes(&self,body:usize)->Option<usize> { self.0.next_capacity_byte_demand(body).ok() }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:107 | `    fn next_close_byte_demand(&self)->Option<usize> { self.0.next_release_byte_demand().ok() }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️aliases/🦀️.rs:31 | `            let copy=owner.next_copy_byte_demand()?;` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:112 | `            let copy = owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:113 | `            let release = owner.next_release_byte_demand().unwrap();` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:114 | `            (copy, owner.next_capacity_byte_demand(if copy == 0 { release } else { law["maximumCopyBytes"].as_u64().unwr` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:177 | `                let copy = inner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:178 | `                let release = inner.next_release_byte_demand().unwrap();` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:179 | `                let capacity = inner.next_capacity_byte_demand(if copy == 0 { release } else { frontier_law["maximumCopy` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:206 | `                let copy = owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:207 | `                (copy, owner.next_capacity_byte_demand(copy).unwrap(), owner.next_release_byte_demand().unwrap())` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:222 | `                assert_eq!(owner.next_copy_byte_demand().unwrap(), copy);` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:234 | `        assert_eq!(owner.next_copy_byte_demand().unwrap(), 0);` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:363 | `            let capacity = owner.next_capacity_byte_demand(grant.maximum_release_bytes).unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:364 | `            let release = owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:368 | `                assert_eq!(owner.next_release_byte_demand().unwrap(), release);` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:421 | `            assert!(retirement.next_capacity_byte_demand(grant.maximum_release_bytes).unwrap() <= budget);` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:422 | `            assert!(retirement.next_release_byte_demand().unwrap() <= budget);` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:779 | `            let capacity=owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:780 | `            let release=owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:879 | `                let capacity = owner.next_capacity_byte_demand(work).unwrap();` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:644 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.0.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:645 | `    fn next_close_byte_demand(&self)->Option<usize> {self.0.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔐️mutex/🦀️.rs:20 | ` fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔐️mutex/🦀️.rs:21 | ` fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔐️mutex/🦀️.rs:22 | ` fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:346 | `    fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}` |
| c | 🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/📦️blob/🧬️schema/🧪️tests/🦀️.rs:39 | `        let copy=owner.next_copy_byte_demand().unwrap().max(1);` |
| a | 🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️unit/🦀️.rs:353 | `        let release = owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️unit/🦀️.rs:361 | `                assert_eq!(owner.next_release_byte_demand().unwrap(), release);` |
| a | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️testing/♻️retirement/🦀️.rs:6 | `    let copy=owner.next_copy_byte_demand()?;assert!(copy<=maximum_copy_bytes);let release=owner.next_release_byte_demand` |
| a | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/♻️retirement/🦀️.rs:28 | `    let demands=\|owner:&ControlledRetirement<T>\|{let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_` |
| a | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🧮️expression/🦀️.rs:40 | `  let capacity=self.next_capacity_byte_demand()?;let copy=self.next_copy_byte_demand()?;if grant.maximum_items==0\|\|grant` |
| c | 🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🕸️mesh/🧱️mesh-owner/🦀️.rs:40 | `    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(\|_\|Mesh3dFault::Closing)?,capacity_bytes:owner.` |
| a | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/🧯️candidate-baseline-close-and-layout-worker/🦀️.rs:174 | `            if original.next_copy_byte_demand()? > grant.maximum_copy_bytes` |
| a | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/🧯️candidate-baseline-close-and-layout-worker/🦀️.rs:175 | `                \|\| original.next_capacity_byte_demand(grant.maximum_copy_bytes)? > grant.maximum_capacity_bytes` |
| a | 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/🧯️candidate-baseline-close-and-layout-worker/🦀️.rs:176 | `                \|\| original.next_release_byte_demand()? > grant.maximum_release_bytes` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧾️eval-session-state/🦀️.rs:505 | `        Ok(RetirementDemand{copy_bytes:self.owners.next_copy_byte_demand()?,capacity_bytes:self.owners.next_capacity_byt` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/♻️ownership/🦀️.rs:59 | `            let release=(retirement.next_copy_byte_demand().unwrap()==0).then(\|\|grant.maximum_release_bytes.checked_sub(` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:116 | `            let copy=self.next_copy_byte_demand().expect("cold geometry copy demand");` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/♻️retirement/🦀️.rs:22 | ` Ok(FlowVcsCloseDemands{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:92 | `                let copy=cursor.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🧬️fields/🦀️.rs:47 | `            if grant.maximum_release_bytes!=0 && owner.next_copy_byte_demand().unwrap()==0 {let (denied,a,r)=observe(\|\|o` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:62 | `        while !self.terminal_is_empty(){let copy=self.next_copy_byte_demand().expect("cold Flow copy demand");let grant=` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:213 | `   let grant=paid(&cursor,copy);let(stage,heap)=observe(\|\|(cursor.next_capacity_byte_demand(copy).unwrap(),cursor.next_r` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:253 | `  let released_bytes=self.next_release_byte_demand()?;if grant.maximum_release_bytes<released_bytes{return Ok(RetainedCl` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:489 | `        Ok(semio_framework_value::RetirementDemand{copy_bytes:self.next_copy_byte_demand()?,capacity_bytes:self.next_cap` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌉️domain-impl/🦀️.rs:41 | `            return Ok(FlowVcsCloseDemands{copy_bytes:frame.next_copy_byte_demand().map_err(flow_close_value_failure)?,ca` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧵️dag-input/🦀️.rs:83 | `            let copy=owner.next_copy_byte_demand().map_err(dag_input_failure)?.min(admitted.maximum_copy_bytes);let capa` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/⏱️evaluation-jobs-and-cancellation/🦀️.rs:23 | `        let copy=owner.next_copy_byte_demand().expect("cold evaluation copy demand");let demand=evaluation_controlled_de` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/⏱️evaluation-jobs-and-cancellation/🦀️.rs:62 | `    Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧯️reply-failure-and-child-demands/🦀️.rs:34 | `    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧩️extension-resources-topic-and-command/🦀️.rs:72 | `        Ok(RetirementDemand {copy_bytes:retirement.next_copy_byte_demand()?,capacity_bytes:retirement.next_capacity_byte` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:51 | `            let capacity = issuer.next_capacity_byte_demand(source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:57 | `                    assert_eq!(issuer.next_capacity_byte_demand(source), Some(capacity));` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:93 | `        for _ in 0..stop { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, gran` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:97 | `    while !issuer.ready() { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, gra` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:127 | `            let capacity = issuer.next_capacity_byte_demand(source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:133 | `                    assert_eq!(issuer.next_capacity_byte_demand(source), Some(capacity));` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:181 | `        for _ in 0..stop { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, gran` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:213 | `                    let capacity = parent.next_capacity_byte_demand(parent_source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:218 | `                    let capacity = child.next_capacity_byte_demand(child_source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs:17 | `        Ok(self.encoder.next_capacity_byte_demand(source).unwrap_or(usize::MAX))` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:95 | `        self.input.next_capacity_byte_demand(parent_id, dialect, self.existing(registry), actor, transaction, Some(group` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:435 | `            RetirementDemand { copy_bytes: open.next_copy_byte_demand()?, capacity_bytes: open.next_capacity_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs:446 | `        row.lane.as_ref().map_or(Ok(0), \|lane\| lane.next_capacity_byte_demand(row.member(registry)?).map_err(plugin_sdk_` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🧪️tests/🦀️.rs:10 | `            if owner.next_capacity_byte_demand() == 70 { break; }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🧪️tests/🦀️.rs:15 | `        assert_eq!(owner.next_capacity_byte_demand(), 70);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🧪️tests/🦀️.rs:20 | `            assert_eq!(owner.next_capacity_byte_demand(), 70);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🧪️tests/🦀️.rs:36 | `            for _ in 0..stop { let capacity = cancelled.next_capacity_byte_demand(); cancelled.advance(source, grant(1,` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🧪️tests/🦀️.rs:55 | `                let g=grant(owner.next_capacity_byte_demand(&source).unwrap(),owner.next_metadata_release_b` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:124 | `            let bytes=self.next_capacity_byte_demand(source)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:112 | `            1 => self.edit.next_capacity_byte_demand((source.publication.prepared_edit_id().unwrap().len(` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:113 | `            2 => self.inverse_encoder.as_ref().unwrap().next_capacity_byte_demand(),` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:114 | `            3 => self.forward.as_ref().unwrap().next_capacity_byte_demand(),` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:115 | `            4 => self.kernel.as_ref().unwrap().next_capacity_byte_demand(&source.kernel(self.index, &self` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:139 | `            } let birth = self.next_capacity_byte_demand(source)?; if grant.maximum_capacity_bytes < birt` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️payload/🦀️.rs:56 | `        self.next_capacity_byte_demand(additional)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️payload/🦀️.rs:63 | `        self.next_capacity_byte_demand(source.len())?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:14 | `            let capacity = owner.next_capacity_byte_demand(additional).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:86 | `            assert!(owner.next_capacity_byte_demand(1).is_err());` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:115 | `                let capacity = owner.next_capacity_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:207 | `                let capacity = owner.next_capacity_byte_demand().unwrap(); let release = owner.next_release_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🧪️tests/🦀️.rs:266 | `                let capacity = owner.next_capacity_byte_demand(&source).unwrap(); let release = owner.next_release_byte` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:63 | `        let capacity = self.next_capacity_byte_demand(additional)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:343 | `        self.active_bytes().map_or(Ok(0), \|bytes\| bytes.next_capacity_byte_demand(self.additional(source)))` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:361 | `            let capacity = self.next_capacity_byte_demand(source)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📚️members/🦀️.rs:109 | `        if self.final_phase==0{return final_receipt.next_capacity_byte_demand(&Self::final_source(&self.rows,self.parent` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs:57 | `            return self.issuer.next_capacity_byte_demand(self.metadata_source(parent_id, parent_dialect, existing, actor` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs:62 | `        self.genesis.next_capacity_byte_demand(store::MemberGenesisEnvelopeSource { schema: self.schema.unwrap(), expect` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🥣️companion/♻️metadata/🦀️.rs:18 | `  let copy=owner.next_copy_byte_demand().unwrap();let funding=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🎛️operation/📥️input/🦀️.rs:60 | `  while !owner.terminal_is_empty(){let demand=owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();let(ste` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:88 | `    assert_eq!(request.next_release_byte_demand().unwrap(),backing);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:55 | `            Ok(RetirementDemand { copy_bytes: self.close.next_copy_byte_demand()?, capacity_bytes: self.close.next_owner` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🧵️retained-work-and-child-emission-laws/🦀️.rs:170 | `            let admitted = lane.next_capacity_byte_demand(&*app.stores.store).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🔚️close-turn-wire-input-and-no-state-disposers/🦀️.rs:65 | `                let copy = store::ErasedSnapshotRetirement::next_copy_byte_demand(retirement.as_` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️private-child-metadata/🦀️.rs:36 | `                let capacity = input.next_capacity_byte_demand(&parent.artifact_id, &parent.dialect, None, "actor:privat` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️private-child-metadata/🦀️.rs:73 | `                    let demand = request.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🎞️host-events-media-cancel-and-download-seam/🦀️.rs:188 | `            let copy=retirement.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📤️segmented-download/🦀️.rs:88 | `        let birth = owner.next_capacity_byte_demand(copy).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/⚖️laws/🦀️.rs:393 | `                    if owner.terminal_is_empty(){break}let copy=owner.next_copy_byte_demand().unwrap();let release=owner` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs:94 | `        let copy = current.next_copy_byte_demand().unwrap().max(grant.maximum_copy_bytes);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs:240 | `            let copy=original.next_copy_byte_demand().expect("original completion alias copy quote").max(4096);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/✏️apply/🧪️tests/🦀️.rs:231 | `        let copy = remaining.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/✏️apply/🧪️tests/🦀️.rs:232 | `        let quote = exact(copy, remaining.next_capacity_byte_demand(copy).unwrap(), remaining.next_release_byte_demand()` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/💥️fault/🦀️.rs:26 | `            \|\| value.next_copy_byte_demand()? > grant.maximum_copy_bytes` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/💥️fault/🦀️.rs:27 | `            \|\| value.next_capacity_byte_demand(grant.maximum_copy_bytes)? > grant.maximum_capacity_bytes` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/💥️fault/🦀️.rs:28 | `            \|\| value.next_release_byte_demand()? > grant.maximum_release_bytes` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎞️media/💥️fault/🦀️.rs:33 | `        let capacity = value.next_capacity_byte_demand(grant.maximum_copy_bytes)?;` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs:208 | `    Ok(RetirementDemand { copy_bytes: recipient.next_copy_byte_demand()?, capacity_bytes: recipient.next_capacity_byte_d` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️origin/🦀️.rs:32 | `  Ok(RetirementDemand{copy_bytes:child.next_copy_byte_demand()?,capacity_bytes:child.next_capacity_byte_demand(body)?,re` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🧪️tests/🦀️.rs:25 | ` while !owner.terminal_is_empty(){let demand=owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap();let(step` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🦀️.rs:31 | `            let copy = owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🦀️.rs:40 | `                assert_eq!(owner.next_copy_byte_demand().unwrap(), copy);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:448 | `        if self.next_copy_byte_demand()? > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(empty)); }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🧪️tests/🔬️unit/🦀️.rs:18 | `        let demand = index.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🧪️tests/🔬️unit/🦀️.rs:58 | `        assert!(owner.next_copy_byte_demand().unwrap() <= grant.maximum_copy_bytes);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🧪️tests/🔬️unit/🦀️.rs:35 | `        let birth = encoder.next_capacity_byte_demand(source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🧪️tests/🔬️unit/🦀️.rs:40 | `            assert_eq!(encoder.next_capacity_byte_demand(source), Some(birth));` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🧪️tests/🔬️unit/🦀️.rs:86 | `        let birth = encoder.next_capacity_byte_demand(source).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs:199 | `            let demand = request.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs:211 | `                assert_eq!(request.next_release_byte_demand().unwrap(), demand);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs:294 | `        assert_eq!(request.next_release_byte_demand().unwrap(),lease);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs:345 | `        assert_eq!(request.next_release_byte_demand().unwrap(), backing);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/🦀️.rs:350 | `            assert_eq!(request.next_release_byte_demand().unwrap(), backing);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:149 | `        Ok(semio_framework_value::RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.nex` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:378 | `        if grant.maximum_release_bytes<self.next_release_byte_demand()?{return Ok(RetainedCloneStep::Progress(empty));}` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs:41 | `  if let Some(owner)=self.edit_close.as_ref(){let body=owner.next_copy_byte_demand()?;return nested(RetirementDemand{cop` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:14 | `    RetirementDemand { copy_bytes: owner.next_copy_byte_demand().unwrap(), capacity_bytes: owner.next_capacity_byte_dema` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:89 | `            let grant = hydration_fold_byte_grant(logical, owner.next_release_byte_demand().unwrap()).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:182 | `    assert_eq!(active.as_ref().unwrap().next_release_byte_demand().unwrap(), extent);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:437 | `            let demand = retirement.next_release_byte_demand().unwrap();` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/♻️pending/🦀️.rs:6 | `        return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byt` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🧪️tests/🔬️unit/🦀️.rs:38 | `        let copy = handoff.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🧪️tests/🔬️unit/🦀️.rs:39 | `        let capacity = handoff.next_capacity_byte_demand(copy).unwrap();` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧾️return/🦀️.rs:100 | `  Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,re` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧪️tests/🦀️.rs:158 | `            assert_eq!((cursor.next_capacity_byte_demand().unwrap(), cursor.next_close_byte_demand().unwrap()), (0, 0));` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🌳️immutable-root/🦀️.rs:109 | `        let copy=owner.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs:323 | `                let copy = retirement.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs:433 | `        if ticket.terminal_is_empty(){break;}let copy=ticket.next_copy_byte_demand().unwrap();let quote=semio_framework_` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs:301 | `        let copy=authority_close.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_c` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs:1006 | `            return child(RetirementDemand { copy_bytes: messages.next_copy_byte_demand(), capacity_bytes: messages.next_` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs:617 | `        let copy = owner.next_copy_byte_demand()?.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧪️tests/🔬️unit/🦀️.rs:295 | `    assert_eq!(owner.next_copy_byte_demand().unwrap(), 0);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧪️tests/🔬️unit/🦀️.rs:296 | `    assert_eq!(owner.next_capacity_byte_demand(4096).unwrap(), 0);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧪️tests/🔬️unit/🦀️.rs:297 | `    assert_eq!(owner.next_release_byte_demand().unwrap(), bytes);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs:53 | `                let body = retirement.next_copy_byte_demand().map_err(VcsError::InverseRefused)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs:54 | `                let capacity = retirement.next_capacity_byte_demand(body).map_err(VcsError::InverseRefused)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs:66 | `                let body = owner.next_copy_byte_demand();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs:157 | `            let capacity=settlement.next_capacity_byte_demand().map_err(VcsError::InverseRefused)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:44 | `        self.active.as_ref().map_or(Ok(0), ErasedSnapshotRetirement::next_copy_byte_demand)` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:47 | `        self.active.as_ref().map_or(Ok(0), \|active\| active.next_capacity_byte_demand(copy))` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:50 | `        if let Some(active) = self.active.as_ref() { return active.next_release_byte_demand(); }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs:27 | `            let capacity=self.cursor.next_capacity_byte_demand()?;control.charge(capacity)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs:39 | `            let capacity=self.cursor.next_capacity_byte_demand()?;control.charge(capacity)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs:23 | `        let release=self.next_release_byte_demand()?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🦀️.rs:52 | `        if grant.maximum_items==0\|\|grant.maximum_copy_bytes<self.next_copy_byte_demand()?\|\|grant.maximum_capacity_bytes<` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs:571 | `        let release = self.next_release_byte_demand()?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🪪️authority/🦀️.rs:28 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.0.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🪪️authority/🦀️.rs:30 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🪪️authority/🦀️.rs:31 | `    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_release_byte_demand().ok()}` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧮️decode-owner-bundle-and-bounded-array-authority/🦀️.rs:79 | `            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand().map_err(\|error\| diagnostic.with_nativ` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📸️native-snapshot/writing.rs:68 | `        Ok(RetirementDemand { copy_bytes: cursor.next_copy_byte_demand()?, capacity_bytes: cursor.next_capacity_byte_dem` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🔁️replay/🦀️.rs:161 | `                    d.copy_bytes = copy.next_copy_byte_demand(message)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🔁️replay/🦀️.rs:162 | `                    d.capacity_bytes = copy.next_capacity_byte_demand(message)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:258 | `            copy_bytes: owner.next_copy_byte_demand().unwrap(),` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:259 | `            capacity_bytes: owner.next_capacity_byte_demand(owner.next_copy_byte_demand().unwrap()).unwrap(),` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:260 | `            release_bytes: owner.next_release_byte_demand().unwrap(), depth: owner.next_depth_demand().unwrap(),` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/↩️retraction-and-bounded-history-read-laws/🦀️.rs:134 | `        let physical_demand = retirement.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:108 | `    assert_eq!(unsupported.next_release_byte_demand().unwrap_err().kind,semio_framework_value::ValueRefusalKind::Unsuppo` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:114 | `    assert_eq!(nested.inner.as_ref().unwrap().next_release_byte_demand().unwrap(),8192);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:26 | `        let physical = owner.retirement.as_ref().unwrap().next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:59 | `        let demand = owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:243 | `                if owner.next_release_byte_demand().unwrap()==N { found=true;break; }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:248 | `            let demand=owner.next_release_byte_demand().unwrap();let caller=row["callerBytes"].as_u64().unwrap() as usi` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs:80 | `        let demand=retirement.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🏁️checkpoint-alternative-history-column-and-backbone-laws/🦀️.rs:233 | `        assert!(retirement.next_release_byte_demand().expect("original backing release demand") <= grant.maximum` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/🔬️unit/🏁️checkpoint-alternative-history-column-and-backbone-laws/🦀️.rs:249 | `        assert!(retirement.next_release_byte_demand().expect("original backing release demand") <= grant.maximum` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/♻️retained-clone-preparation-lifecycle-laws/🦀️.rs:10 | `        publication.next_copy_byte_demand().expect("current publication copy frontier");` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/♻️retained-clone-preparation-lifecycle-laws/🦀️.rs:11 | `        publication.next_capacity_byte_demand(grant.maximum_copy_bytes).expect("current publication capacity frontier");` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/♻️retained-clone-preparation-lifecycle-laws/🦀️.rs:12 | `        publication.next_release_byte_demand().expect("current publication release frontier");` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/📸️group-envelope-read-canonical-revision-and-remote-ingest-laws/🦀️.rs:51 | `        owner.next_copy_byte_demand().expect("fallible current copy frontier");owner.next_capacity_byte_dem` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:243 | `            let copy_bytes = retained.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:245 | `            assert_eq!(retained.next_release_byte_demand().unwrap(), demand.release_bytes, "inquiry must retain the curr` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:250 | `            assert_eq!(retained.next_release_byte_demand().unwrap(), demand.release_bytes);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:518 | `                            let body = retirement.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:802 | `        let capacity = cursor.next_capacity_byte_demand().expect("operation wire capacity demand");` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️bounded-value-retirement/🦀️.rs:361 | `        let original_release_demand=owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️bounded-value-retirement/🦀️.rs:362 | `        let original_capacity_demand=owner.next_capacity_byte_demand(copy).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️bounded-value-retirement/🦀️.rs:367 | `            assert_eq!(owner.next_release_byte_demand().unwrap(),original_release_demand);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️bounded-value-retirement/🦀️.rs:368 | `            assert_eq!(owner.next_capacity_byte_demand(copy).unwrap(),original_capacity_demand);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🧪️tests/🔬️unit/🦀️.rs:145 | `                let release_demand = rejected.as_ref().unwrap().next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🧪️tests/🔬️unit/🦀️.rs:148 | `                assert_eq!(rejected.as_ref().unwrap().next_release_byte_demand().unwrap(), capacity);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🔤️text/🧪️tests/🦀️.rs:45 | `            let copy = retirement.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🔤️text/🧪️tests/🦀️.rs:46 | `            let capacity = retirement.next_capacity_byte_demand(copy).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs:42 | `                assert_eq!(catalog.next_release_byte_demand().unwrap(), demand);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🧪️tests/🔬️unit/🦀️.rs:61 | `    assert_eq!(catalog.next_release_byte_demand().unwrap(), 0);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🦀️.rs:74 | `        let bytes = self.next_release_byte_demand()?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:93 | `            let release = retirement.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:94 | `            let capacity = retirement.next_capacity_byte_demand(copy).unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:105 | `                assert_eq!(retirement.next_release_byte_demand().unwrap(), release);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:256 | `            let release = owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:262 | `                assert_eq!(owner.next_release_byte_demand().unwrap(), release);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:352 | `        let release = owner.next_release_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧪️tests/🦀️.rs:358 | `            assert_eq!(owner.next_release_byte_demand().unwrap(), release);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🔬️unit/🦀️.rs:12 | `        let copy=owner.next_copy_byte_demand().unwrap();let work=quantum.max(copy).min(remaining.maximum_copy_bytes);let` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:105 | `                let copy=cursor.next_copy_byte_demand().unwrap().max(4096);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:12 | `  let owner=slot.as_ref().unwrap();let work=copy.max(owner.next_copy_byte_demand().unwrap());` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs:44 | ` for turn in 0..1_000_000{let copy=owner.next_copy_byte_demand().unwrap().max(7);let grant=Grant{maximum_items:1,maximum` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs:490 | `        owner.next_copy_byte_demand().map_err(\|_\|ui_scene::math::Mesh3dFault::Closing)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs:491 | `        Ok(world_mesh_ownership_grant(copy_bytes,owner.next_capacity_byte_demand(copy_bytes).map_err(\|_\|ui_scene::math::` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs:517 | `            let copy=owner.next_copy_byte_demand().map_err(\|_\|ui_scene::math::Mesh3dFault::Closing)?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs:518 | `            let grant=world_mesh_ownership_grant(copy,owner.next_capacity_byte_demand(copy).map_err(\|_\|ui_scene::math::M` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🪟️wasm-session-and-input-application/🦀️.rs:183 | `  4=>self.status_insert.as_ref().unwrap().next_capacity_byte_demand(&self.statuses),6=>self.ports.insert_c` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs:40 | `                if grant.maximum_release_bytes!=0 && owner.next_copy_byte_demand().unwrap()==0 {let (denied,a,r)=observe` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs:353 | `            let copy=input.next_copy_byte_demand().unwrap();` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:142 | `    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:143 | `    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:144 | `    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:211 | `    let copy=4096.max(input.next_copy_byte_demand()?);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:235 | `        let capacity=update.next_capacity_byte_demand()?;` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:347 | `        Ok(update.next_copy_byte_demand())` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:503 | `        fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:504 | `        fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:520 | `        if !self.values.terminal_is_empty(){return self.values.next_release_byte_demand();}` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs:463 | `    let copy=4096.max(retirement.next_copy_byte_demand()?);` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🧪️tests/📔️registry/🦀️.rs:28 | `    let copy=copy.max(owner.next_copy_byte_demand()?);` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:87 | `        self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🧬️generation/🦀️.rs:27 | `  if let Some(copy)=owner.next_copy_byte_demand().unwrap().checked_sub(1){let under=RetainedCloneGrant{maximum_copy_byte` |
| c | 🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🦀️.rs:60 | `            let copy=4096.max(retirement.next_copy_byte_demand().expect("cold generation copy demand"));` |
| a | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:104 | `        let copy=orphan.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_byte` |
| a | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:154 | `    let layout=retirement.next_release_byte_demand().unwrap();` |
| a | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/⏱️tessellation/🦀️.rs:28 | `    port.begin_close();while !port.terminal_is_empty(){let copy=port.next_copy_byte_demand().unwrap();let grant=Retained` |
| c | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/⏱️tessellation/🦀️.rs:99 | `        while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let demand=owner.selected_deman` |
| a | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:65 | `        let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=` |
| c | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/⚙️retained-params/🦀️.rs:48 | `        let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: l` |
| c | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️params-2/🧳️parameter-leaf/🦀️.rs:184 | `            return Ok(RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacit` |
| c | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️params-2/🦀️.rs:312 | `            return Ok(RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_c` |
| c | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️params-2/🦀️.rs:499 | `                    let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.` |
| c | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️params-2/🦀️.rs:540 | `                let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next` |
| d | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🪜️base64/🦀️.rs:59 | `  let copy=size_of::<Base64DecodeCursor<'static>>()+cursor.next_copy_byte_demand(available);let capacity=cursor.next_` |
| c | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🪜️base64/🦀️.rs:133 | `   0=>RetirementDemand{copy_bytes:job.next_copy_byte_demand()?,capacity_bytes:job.next_capacity_byte_demand()?,depth` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:61 | `        self.owner.as_ref().map_or(Ok(0), \|owner\| owner.next_copy_byte_demand())` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:65 | `        self.owner.as_ref().map_or(Ok(0), \|owner\| owner.next_capacity_byte_demand(maximum_copy_bytes))` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:69 | `        self.owner.as_ref().map_or(Ok(0), \|owner\| owner.next_release_byte_demand())` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏭️direct-store-factory-and-child-group-route/🦀️.rs:246 | `            return owner.next_copy_byte_demand();` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏭️direct-store-factory-and-child-group-route/🦀️.rs:252 | `            return owner.next_capacity_byte_demand(maximum_copy_bytes);` |
| a | ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏭️direct-store-factory-and-child-group-route/🦀️.rs:258 | `            return owner.next_release_byte_demand();` |
| a | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🦀️.rs:84 | `        let copy = owner.next_copy_byte_demand().expect("CAD native cold copy demand");` |
| a | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🦀️.rs:85 | `        let release = owner.next_release_byte_demand().expect("CAD native cold release demand");` |
| d | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🦀️.rs:86 | `        let capacity = owner.next_capacity_byte_demand(if copy == 0 { release } else { copy }).expect("CAD native cold c` |
| a | ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧬️geometry-media-and-owners/🦀️.rs:312 | `                let demand=cursor.next_capacity_byte_demand().unwrap();` |
| c | ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🚦️controlled/🧪️tests/🦀️.rs:12 | `  let copy=owner.next_copy_byte_demand().unwrap().max(256);` |
| a | ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🚦️controlled/🧪️tests/🦀️.rs:16 | `   match axis{0=>denied.maximum_items=0,1 if owner.next_copy_byte_demand().unwrap()>0=>denied.maximum_copy_bytes=owner.n` |
| a | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔁️persistent/🦀️.rs:649 | `            let copy_bytes = retirement.next_copy_byte_demand()?;` |
| a | ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔁️persistent/🦀️.rs:663 | `    let copy_bytes = retirement.next_copy_byte_demand()?;` |
| a | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:457 | `        let slice = self.next_release_byte_demand()?;` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs:182 | `        let copy = owner.next_copy_byte_demand().expect("cold displaced copy demand").max(GENERATION3D_OWNER_BYTES);` |
| a | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs:190 | `    let copy = flow.next_copy_byte_demand()?;` |
| a | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️retirement-driver/🦀️.rs:27 | `    let copy = owner.next_copy_byte_demand()?;` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️retirement-driver/🦀️.rs:28 | `    Ok(RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧲️retained-view-commands/🦀️.rs:154 | `            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retire` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧲️retained-view-commands/🦀️.rs:157 | `            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capacity_bytes: retire` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️play-app-and-retained-command-contracts/🦀️.rs:273 | `            return Ok(RetirementDemand { copy_bytes: retirement.next_copy_byte_demand()?, capaci` |
| a | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🧰️support/🔬️retirement-driver/🦀️.rs:27 | `    let copy = owner.next_copy_byte_demand()?;` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🧰️support/🔬️retirement-driver/🦀️.rs:28 | `    Ok(RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner` |
| c | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🌉️apply/🦀️.rs:47 | `        let copy = owner.next_copy_byte_demand().expect("cold displaced copy demand").max(GENERATION2D_OWNER_BYTES);` |
| a | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️edit-command-work/🦀️.rs:274 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_copy_byte_demand())` |
| a | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️edit-command-work/🦀️.rs:277 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_capacity_byte_demand(maximum_copy_bytes))` |
| a | ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️edit-command-work/🦀️.rs:280 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_release_byte_demand())` |
| a | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/🦀️.rs:234 | `        if let Some(owner) = self.remnant.as_ref() { return owner.next_capacity_byte_demand(maximum_copy_bytes); }` |
| a | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:374 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_copy_byte_demand())` |
| a | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:377 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_capacity_byte_demand(maximum_copy_bytes))` |
| a | ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs:380 | `        self.remnant.as_ref().map_or(Ok(0), \|owner\| owner.next_release_byte_demand())` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🧪️tests/🦀️.rs:13 | `        let copy = owner.next_copy_byte_demand().unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🧪️tests/🦀️.rs:17 | `            maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).unwrap(),` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🧪️tests/🦀️.rs:18 | `            maximum_release_bytes: owner.next_release_byte_demand().unwrap(),` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:9 | `            let copy = retirement.next_copy_byte_demand().expect("window retirement copy quote");` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:13 | `                maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy).expect("window retirement capacity q` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:14 | `                maximum_release_bytes: retirement.next_release_byte_demand().expect("window retirement release quote"),` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:16 | `            let copy = retirement.next_copy_byte_demand().expect("window retirement copy quote");` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:20 | `                maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy).expect("window retirement capacity q` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:21 | `                maximum_release_bytes: retirement.next_release_byte_demand().expect("window retirement release quote"),` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/🦀️.rs:39 | `        let copy=retirement.next_copy_byte_demand().unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/🦀️.rs:40 | `        let capacity=retirement.next_capacity_byte_demand(copy).unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/🦀️.rs:41 | `        let release=retirement.next_release_byte_demand().unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:23 | `        let demand = cursor.next_release_byte_demand().unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:29 | `            assert_eq!(cursor.next_release_byte_demand().unwrap(), demand);` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:43 | `    let capacity = cursor.next_capacity_byte_demand(source).unwrap();` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:44 | `    let copy = if capacity == 0 { cursor.next_copy_byte_demand(source).unwrap() } els` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:50 | `        assert_eq!(cursor.next_capacity_byte_demand(source).unwrap(), capacity);` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs:56 | `        assert_eq!(cursor.next_copy_byte_demand(source).unwrap(), copy);` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:41 | `  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:43 | `  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:52 | `  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_capacity_byte_demand(body)}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:54 | `  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_capacity_byte_demand(body)}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:63 | `  if let Some(owner)=self.displaced_close.as_ref(){return owner.next_release_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🦀️.rs:65 | `  if let Some(owner)=self.candidate_close.as_ref(){return owner.next_release_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:53 | `  if let Some(owner)=self.pending_close.as_ref(){return owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:55 | `  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:64 | `  if let Some(owner)=self.pending_close.as_ref(){return owner.next_capacity_byte_demand(body)}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:66 | `  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_capacity_byte_demand(body)}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:75 | `  if let Some(owner)=self.pending_close.as_ref(){return owner.next_release_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/🦀️.rs:77 | `  if let Some(owner)=self.inverse_close.as_ref(){return owner.next_release_byte_demand()}` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎮️prepare/🧰️child/🦀️.rs:16 | `        match self {Self::Copy=>owner.next_copy_byte_demand(),Self::Capacity(body)=>owner.next_capacity_byte_dema` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:93 | `        if let Some(owner)=self.pending_close.as_ref() { return owner.next_copy_byte_demand(); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:95 | `        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_copy_byte_demand(); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:101 | `        if let Some(owner)=self.pending_close.as_ref() { return owner.next_capacity_byte_demand(body); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:103 | `        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_capacity_byte_demand(body); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:109 | `        if let Some(owner)=self.pending_close.as_ref() { return owner.next_release_byte_demand(); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️literal-id-inverse/🦀️.rs:111 | `        if let Some(owner)=self.inverse_close.as_ref() { return owner.next_release_byte_demand(); }` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:14 | `            let copy = retirement.next_copy_byte_demand().expect("window retirement copy quote");` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:18 | `                maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy).expect("window retirement capacity qu` |
| a | ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:19 | `                maximum_release_bytes: retirement.next_release_byte_demand().expect("window retirement release quote"),` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🏟️arena/🔓️owned-field-decode/🦀️.rs:31 | `        let copy_bytes = self.owner.next_copy_byte_demand()?;` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧹️retire/🧪️tests/🔬️unit/🦀️.rs:18 | `  let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap(` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:60 | `        let copy = cursor.next_copy_byte_demand().expect("Draw owner retirement copy demand");` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64 | `            maximum_capacity_bytes: cursor.next_capacity_byte_demand(copy).expect("Draw owner retirement capacity demand` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:65 | `            maximum_release_bytes: cursor.next_release_byte_demand().expect("Draw owner retirement release demand"),` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️completion/🧪️tests/🦀️.rs:16 | `  let copy=owner.next_copy_byte_demand().unwrap();let capacity=owner.next_capacity_byte_demand(copy).unwrap();let releas` |
| c | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs:21 | `        let copy=owner.next_copy_byte_demand().unwrap().max(4096);` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/✉️envelope-laws/🦀️.rs:49 | `        let copy_bytes = retirement.next_copy_byte_demand().expect("Drawing fixture envelope copy demand");` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/✉️envelope-laws/🦀️.rs:53 | `            maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy_bytes).expect("Drawing fixture e` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/✉️envelope-laws/🦀️.rs:54 | `            maximum_release_bytes: retirement.next_release_byte_demand().expect("Drawing fixture envelope rele` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:298 | `        let copy = retired.next_copy_byte_demand().map_err(refused)?;` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:299 | `        let release = retired.next_release_byte_demand().map_err(refused)?;` |
| c | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-image/🧵️admission/🦀️.rs:34 | `  Ok(RetirementDemand{copy_bytes:job.next_copy_byte_demand().map_err(\|_\|ValueError::literal(ValueRefus` |
| c | ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧱️learning-program-and-geometry/🦀️.rs:235 | `        Ok(RetirementDemand { copy_bytes: self.retirement.next_copy_byte_demand()?, capacity_bytes: self.retirement.next` |
| a | 🌎️hub/🧩️compositions/🀄️wfc/🧪️tests/🎟️mounted-owner-policy/🦀️.rs:55 | `            if owner.next_release_byte_demand().expect("release quote") > old.maximum_release_bytes {` |
| a | 🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🦀️.rs:103 | `        let copy = owner.next_copy_byte_demand().expect("Semio drawing cold copy demand");` |
| a | 🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🦀️.rs:104 | `        let release = owner.next_release_byte_demand().expect("Semio drawing cold release demand");` |
| d | 🌎️hub/🧩️compositions/📏️layout/🖊️drawing/🦀️.rs:105 | `        let capacity = owner.next_capacity_byte_demand(if copy == 0 { release } else { copy }).expect("Semio drawing col` |

## Appendix B. Forwarders and trait impls (683), class per row

| class | file:line | body |
|---|---|---|
| a | 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️testing/🧪️tests/🦀️.rs:18 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️testing/🧪️tests/🦀️.rs:19 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}` |
| a | 🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️testing/🧪️tests/🦀️.rs:20 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:211 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Self::next_release_byte_demand(self)}` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:330 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:112 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:113 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:165 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Self::next_copy_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:166 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self` |
| a | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:167 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:63 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Self::next_copy_byte_demand(self)}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:64 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Self::next_capacity_byte_demand(self,copy)` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:65 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Self::next_release_byte_demand(self)}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:67 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Self::next_copy_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:68 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:69 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:278 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:280 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_release_byte_demand(sel` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🧪️tests/🎮️controlled/🦀️.rs:20 | `        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.original.next_copy_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🧪️tests/🎮️controlled/🦀️.rs:21 | `        fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.original.next_capacity_byte_demand` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🧪️tests/🎮️controlled/🦀️.rs:22 | `        fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.original.next_release_byte_demand()}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/🦀️.rs:99 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.original.next_copy_byte_demand() }` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/🦀️.rs:101 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.original.next_release_byte_demand() }` |
| a | 🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs:216 | ` pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand(` |
| a | 🧰️framework/🔨️modules/◻️2d/🧹️retire/🦀️.rs:23 | `   pub fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_copy_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:140 | `        fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.inner.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:141 | `        fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.inner.next_capacity_byte_demand(c` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:142 | `        fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.inner.next_release_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:560 | `            pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.cursor.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:561 | `            pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.cursor.next_capacity_byte` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:562 | `            pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.cursor.next_release_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:105 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:107 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:20 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:21 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:22 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:40 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:41 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:42 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:23 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:24 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:25 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/📦️owned/🦀️.rs:22 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs:35 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs:37 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:581 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:582 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.inner.next_capacity_byte_demand` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:583 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:23 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:24 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:25 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:125 | `        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_copy_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:126 | `        fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.inner.next_capacity_byte_de` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:127 | `        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_release_byte_demand() }` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs:239 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.owner.next_copy_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs:240 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.owner.next_capacity_byte_demand(c` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🦀️.rs:241 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.owner.next_release_byte_demand()}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🦀️.rs:96 | `    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owned.next_copy_byte_demand(` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧹️retire/🦀️.rs:12 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧹️retire/🦀️.rs:13 | ` pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧹️retire/🦀️.rs:14 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:443 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:444 | ` pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)` |
| a | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:445 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}` |
| d | 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛫️encode/🦀️.rs:132 | ` pub fn next_copy_byte_demand(&self,writer:&ToolRunTickWriter,payload:Option<&dyn ToolRunTickPayloadSource>)->Result<us` |
| a | 🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:113 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_header_copy_bytes().map_or_else(\|\|self.residu` |
| a | 🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:114 | `    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if self.close_header_copy_bytes().is_some()` |
| a | 🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:115 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.close_header_copy_bytes().is_some(){return Ok(0` |
| d | 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:102 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let Some(pendin` |
| d | 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:103 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let ` |
| d | 🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs:104 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.action_pending(){return Ok(0)}if let Some(pendin` |
| d | 🧰️framework/🔨️modules/🚪️io/🔤️base64/🎮️cursor/🦀️.rs:107 | `    pub fn next_copy_byte_demand(&self,maximum_work_units:usize)->usize{` |
| d | 🧰️framework/🔨️modules/🚪️io/🔤️base64/🎮️cursor/🦀️.rs:123 | `    pub fn next_capacity_byte_demand(&self) -> usize {` |
| d | 🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/🦀️.rs:72 | ` pub fn next_copy_byte_demand(&self)->usize{if matches!(self.phase,"complete"\|"transferred"){0}else{32}}` |
| d | 🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/🦀️.rs:73 | ` pub fn next_capacity_byte_demand(&self)->usize{if self.phase=="validate"&&self.at==self.source_length&&self.quad_at==0{` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/♻️compaction/🦀️.rs:45 | `    pub fn next_copy_byte_demand(&self,body:&Body)->Result<usize,ValueError>{Ok(self.demand(body,0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/♻️compaction/🦀️.rs:46 | `    pub fn next_capacity_byte_demand(&self,body:&Body,copy:usize)->Result<usize,ValueError>{Ok(self.demand(body,copy)?.c` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/♻️compaction/🦀️.rs:47 | `    pub fn next_release_byte_demand(&self,body:&Body)->Result<usize,ValueError>{Ok(self.demand(body,0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🧭️reachability-walk/🦀️.rs:134 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🧭️reachability-walk/🦀️.rs:135 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demand(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🧭️reachability-walk/🦀️.rs:136 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/♻️retention/🦀️.rs:38 | `    pub fn next_copy_byte_demand(&self,kernel:&Brep)->Result<usize,ValueError>{Ok(self.demand(kernel,0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/♻️retention/🦀️.rs:39 | `    pub fn next_capacity_byte_demand(&self,kernel:&Brep,copy:usize)->Result<usize,ValueError>{Ok(self.demand(kernel,copy` |
| d | 🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/♻️retention/🦀️.rs:40 | `    pub fn next_release_byte_demand(&self,kernel:&Brep)->Result<usize,ValueError>{Ok(self.demand(kernel,0)?.release_byte` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🏭️schema/⏳️cursor/🦀️.rs:73 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(\|d\|d.0)}` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🏭️schema/⏳️cursor/🦀️.rs:74 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(\|d\|d.1)}` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs:128 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.phase==Phase::Expression&&self.expres` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs:92 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(\|v\|v.0)}` |
| d | 🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🦀️.rs:93 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(\|v\|v.1)}` |
| d | 🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎮️decode/🦀️.rs:44 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, InlineSymbolRefusal> {` |
| d | 🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎮️decode/🦀️.rs:49 | `    pub fn next_copy_byte_demand(&self) -> usize {` |
| d | 🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🦀️.rs:85 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { self.demands().map(\|demand\| demand.0) }` |
| d | 🧰️framework/🔨️modules/🗜️deflate/🛫️encode/⏳️cursor/🦀️.rs:86 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demands().map(\|demand\| demand.1) }` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🦀️.rs:67 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if !self.elements.is_empty() { size_of::<PathEl>` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🦀️.rs:68 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🦀️.rs:69 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🦀️.rs:75 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🦀️.rs:81 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🖊️stroke/🦀️.rs:85 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/📦️bytes/🦀️.rs:194 | `    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(!self.closed&&self` |
| d | 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/📦️bytes/🦀️.rs:195 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/📦️bytes/🦀️.rs:196 | `    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if self.closed\|\|self.len()!=0\|\|s` |
| d | 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️.rs:94 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owner.n` |
| d | 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️.rs:95 | `    pub fn next_capacity_byte_demand(&self,maximum_copy_bytes:usize)->Result<usize,ValueError>{match &*self.owned{OwnerS` |
| d | 🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🩹️update/🦀️.rs:96 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{match &*self.owned{OwnerState::Closing(owner)=>owne` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:154 | ` pub fn next_capacity_byte_demand(&self,source:ForeignStepSource<'_>)->Result<usize,ValueError>{self.check(source)?;Ok(i` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:155 | ` pub fn next_copy_byte_demand(&self,source:ForeignStepSource<'_>)->Result<usize,ValueError>{self.check(source)?;if self` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:183 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.capacity(self.owner.close_phase))}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:209 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_close_copy_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:210 | ` fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{self.next_close_capacity_byte_demand(b)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:223 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:224 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:225 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.owner.capacity(self.owner.close_phase))}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:267 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_copy_byte_demand()}els` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:268 | ` fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_capacity_byte_` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:269 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(c)=&self.child{c.next_release_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:341 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_close_copy_byte_demand(` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:342 | ` fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_close_capacity_` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🌐️foreign/🦀️.rs:343 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(v)=&self.copy{v.next_release_byte_demand()}e` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:110 | `    pub fn next_capacity_byte_demand(&self, source: &impl ArtifactMessageSource) -> Result<usize, ArtifactMessageCompose` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:121 | `    pub fn next_copy_byte_demand(&self, source: &impl ArtifactMessageSource) -> Result<usize, ArtifactMessageComposeRefu` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:291 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:328 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose/🦀️.rs:329 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/🦀️.rs:12 | ` pub fn next_capacity_byte_demand(&self,source:&MutationMessage)->Result<usize,ValueError>{self.source(source)?;if self.` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/🦀️.rs:13 | ` pub fn next_copy_byte_demand(&self,source:&MutationMessage)->Result<usize,ValueError>{self.source(source)?;if !self.al` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/🦀️.rs:39 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_close_copy_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/🦀️.rs:40 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.next_close_capacity_byte_demand(body)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📋️copy/🦀️.rs:41 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_close_release_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:21 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.rows.has_reserved_slot() { Ok(0) } el` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:79 | `    pub fn next_copy_byte_demand(&self) -> usize { usize::from(self.active.is_none() && (self.strings.iter().any(Option:` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:80 | `    pub fn next_release_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(\|\| if self.next_copy_byte_demand(` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:81 | `    pub fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:124 | `    fn next_copy_byte_demand(&self, ledger: &ReplayMessageAccumulator) -> usize { if self.pending.is_some() && !ledger` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🦀️.rs:125 | `    fn next_release_byte_demand(&self) -> usize { self.outcome.as_ref().map_or(0, \|outcome\| if self.index == self.origin` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:53 | `    pub fn next_copy_byte_demand(&self) -> usize {` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:60 | `    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:62 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:111 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(Self::next_copy_byte_demand(self)) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:132 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if self.active.as_ref().is_some_and(MutationMessa` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:133 | `    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🦀️.rs:134 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if self.active.as_ref().is_some_and(MutationMe` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/👁️borrow/🦀️.rs:98 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/👁️borrow/🦀️.rs:99 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/👁️borrow/🦀️.rs:100 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:198 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:199 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:200 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:274 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:275 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🦀️.rs:276 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🪢️join/🦀️.rs:160 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🪢️join/🦀️.rs:161 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📤️publication/🪢️join/🦀️.rs:162 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🦀️.rs:76 | `    pub fn next_capacity_byte_demand(&self) -> usize { match self.phase { Phase::Reserve => self.original.min(ROWS) * si` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🦀️.rs:77 | `    pub fn next_copy_byte_demand(&self, source: &impl FinalMessageSource) -> usize {` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🦀️.rs:81 | `    pub fn next_release_byte_demand(&self, source: &impl FinalMessageSource) -> Result<usize, ValueError> { if let Some(` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:69 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:70 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:71 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:151 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:152 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs:153 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:16 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.0.next_copy_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:17 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.0.next_capacity_byte_demand(copy)}` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:18 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.0.next_release_byte_demand()}` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:38 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner.` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:39 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(o` |
| d | 🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🦀️.rs:40 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{match &self.owner{ProtocolOwner::Conflict(owner)=>owner` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs:373 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.original_output.is_some()\|\|self.control_closed&&` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs:374 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.original_output.is_some()\|\|self.con` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs:375 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.original_output.is_some()\|\|self.control_closed&` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🥣️companion/🪙️work/🦀️.rs:21 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🥣️companion/🪙️work/🦀️.rs:22 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🧪️tests/🥣️companion/🪙️work/🦀️.rs:23 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🔲️pixels/📸️jpeg/📥️decode/🦀️.rs:35 | ` pub fn next_copy_byte_demand(&self)->usize{match self.phase{"complete"\|"transferred"=>0,"allocate"=>size_of::<Vec<u8>>` |
| d | 🧰️framework/🔨️modules/🔲️pixels/📸️jpeg/📥️decode/🦀️.rs:36 | ` pub fn next_capacity_byte_demand(&self)->usize{if self.phase!="allocate"{return 0;}if self.allocate_at<self.component_c` |
| d | 🧰️framework/🔨️modules/🔲️pixels/📸️jpeg/📥️decode/🦀️.rs:37 | ` pub fn next_release_byte_demand(&self)->usize{0}` |
| d | 🧰️framework/🔨️modules/🔲️pixels/📷️png/📥️decode/🦀️.rs:176 | ` pub fn next_copy_byte_demand(&self)->Result<usize,PngDecodeError>{` |
| d | 🧰️framework/🔨️modules/🔲️pixels/📷️png/📥️decode/🦀️.rs:187 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,PngDecodeError>{` |
| d | 🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs:29 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ImageDecodeError>{Ok(match self.phase{"complete"\|"transferred"=>0,"s` |
| d | 🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs:32 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ImageDecodeError>{Ok(match self.phase{"jpeg"=>self.jpeg.as_ref().` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🦀️.rs:20 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_cop` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🦀️.rs:21 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.sour` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🦀️.rs:22 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.source.terminal_is_empty(){self.source.next_rel` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🧫️owner/🦀️.rs:9 | `    fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusa` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🧫️owner/🦀️.rs:10 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,crate::ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🧫️owner/🦀️.rs:11 | `    fn next_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🧪️tests/🦀️.rs:14 | `  fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🧪️tests/🦀️.rs:15 | `  fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🧪️tests/🦀️.rs:16 | `  fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:27 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),\|owner\|Ok(facto` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:28 | `    pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),` |
| d | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🦀️.rs:29 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.retirement.as_ref().map_or(Ok(0),\|owner\|Ok(fa` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:63 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:64 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️lease/🦀️.rs:65 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:186 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:187 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:188 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:64 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:65 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🔗️read/🪪️authority/🦀️.rs:66 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧵️insert/🦀️.rs:12 | ` pub fn next_capacity_byte_demand(&self,index:&NumericIndex<K,V>)->Result<usize,ValueError>{if self.phase==2&&!index.nod` |
| d | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔗️immutable-alias/🦀️.rs:60 | `    pub fn next_release_byte_demand(&self) -> usize { if self.value.is_some() {Self::allocation_bytes()} else {0} }` |
| d | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:340 | `    pub fn next_copy_byte_demand(&self)->usize{let state=&self.state;if state.closing\|\|state.is_complete(){return 0}if s` |
| d | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:341 | `    pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{if self.state.closing\|\|self.state.is_complete(){re` |
| d | 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:508 | `    pub fn next_copy_byte_demand(&self)->usize {0}` |
| d | 🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs:84 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_copy_byte_demand() }` |
| d | 🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs:85 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.next_close_capacity_byte_demand` |
| d | 🧰️framework/🔨️modules/🌱️value/📦️paged/🎮️append/🦀️.rs:86 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_release_byte_demand() }` |
| d | 🧰️framework/🔨️modules/🌱️value/🧪️testing/👥️presence-retirement/🦀️.rs:89 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🔨️modules/🌱️value/🧪️testing/👥️presence-retirement/🦀️.rs:90 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🔨️modules/🌱️value/🧪️testing/👥️presence-retirement/🦀️.rs:91 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs:71 | `  fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.bytes.is_some(){std::mem::size_of::<Option<Ve` |
| d | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs:72 | `  fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🧪️tests/🦀️.rs:73 | `  fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.bytes.as_ref().map_or(0,Vec::capacity))}` |
| a | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🫴️recipient/🦀️.rs:37 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.copy_bytes)}` |
| a | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🫴️recipient/🦀️.rs:38 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(\|d\|d.capacity_byte` |
| a | 🧰️framework/🔨️modules/🌱️value/🛫️encode/🫴️recipient/🦀️.rs:39 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️owner/🦀️.rs:33 | `  fn terminal_is_empty(&self)->bool{!self.0}fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}fn next_capa` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs:26 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs:27 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🏭️factory/🦀️.rs:28 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:22 | `    }    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.next_demand(0).map(\|d\|d.copy_bytes)}` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:23 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.next_demand(copy).map(\|d\|d.capaci` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🦀️.rs:24 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if let Some(source)=self.source.as_ref(){return Ok` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:134 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:135 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:136 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:24 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:25 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:26 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:144 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_demand(0).map(\|demand\| demand.copy_byte` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:147 | `    pub fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.next_demand(m` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:150 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_demand(0).map(\|demand\| demand.releas` |
| a | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:279 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { ControlledRetirement::` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:61 | `    }    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.selected().map_or_else(\|\|if self.terminal_i` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:62 | `    pub fn next_capacity_byte_demand(&self,maximum_body_bytes:usize)->Result<usize,ValueError> {self.selected().map_or(O` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/📋️queue/🦀️.rs:63 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🚼️construction/🧪️tests/admission/native.rs:26 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🚼️construction/🧪️tests/admission/native.rs:27 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🚼️construction/🧪️tests/admission/native.rs:28 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:63 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:64 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🦀️.rs:65 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/🦀️.rs:100 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.original.next_capacity_byte_dem` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:164 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.prepa` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:165 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.preparation_is_complete(){return ` |
| d | 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs:166 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.preparation_is_complete(){return Ok(self.prep` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🛂️allocation/🧪️tests/🦀️.rs:39 | `  fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.bytes.is_some(){std::mem::size_of::<Option<Ve` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🛂️allocation/🧪️tests/🦀️.rs:40 | `  fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🛂️allocation/🧪️tests/🦀️.rs:41 | `  fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.bytes.as_ref().map_or(0,Vec::capacity))}` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🦀️.rs:92 | `        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🦀️.rs:93 | `        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🦀️.rs:94 | `        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.terminal_is_empty(){0}else{4096})}` |
| a | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🦀️.rs:86 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.copy_bytes)}` |
| a | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🦀️.rs:87 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_demand(copy).map(\|d\|d.capacity_byte` |
| a | 🧰️framework/🔨️modules/🌱️value/🛬️decode/🫴️recipient/🦀️.rs:88 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.release_bytes)}` |
| a | 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:332 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}` |
| d | 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:333 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🔨️modules/🌱️value/📝️text/📦️paged/🦀️.rs:334 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.bytes.is_empty(){Ok(0)}else{self.bytes.next_re` |
| d | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🦀️.rs:106 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.` |
| d | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🦀️.rs:107 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,bo` |
| d | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🦀️.rs:108 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.` |
| d | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🧮️expression/🦀️.rs:31 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{if self.phase==0&&!self.frames.has_reserved_slot(){se` |
| d | 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🧮️expression/🦀️.rs:32 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.phase==0&&!self.frames.has_reserved_slot(){retur` |
| d | 🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs:217 | ` pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_cap` |
| d | 🧰️framework/🔨️modules/◻️2d/🔍️trace/🦀️.rs:218 | ` pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_de` |
| d | 🧰️framework/🔨️modules/◻️2d/🧹️retire/🦀️.rs:24 | `   pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owner.next_c` |
| d | 🧰️framework/🔨️modules/◻️2d/🧹️retire/🦀️.rs:25 | `   pub fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owner.next_release_byte_` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/♻️ownership/🦀️.rs:36 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_ret` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/♻️ownership/🦀️.rs:37 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return ` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/♻️ownership/🦀️.rs:38 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_re` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:81 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:82 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:83 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:93 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),\|port\|if port.terminal` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:94 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),\|port\|i` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:95 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.port.as_ref().map_or(Ok(0),\|port\|if port.termi` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🦀️.rs:88 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🦀️.rs:89 | `    fn next_capacity_byte_demand(&self, _copy: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🎚️parameter/📨️intent/🦀️.rs:90 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.bytes.get(self.inde` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:46 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(\|\|self.queue.next_copy` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:47 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.root.as_ref().map_or_else(\|\|self.` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:48 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(\|\|self.queue.next_r` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:256 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:257 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs:258 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.prepared&&self.original.as_ref().is_some_and(` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:478 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_is` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:479 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {let state=&*self.owned;if !state.retiremen` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🦀️.rs:480 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_i` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs:121 | `        pub fn next_capacity_byte_demand(&self, source: [&str; N], leased: bool) -> usize { if self.closing \|\| self.fiel` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs:210 | `        pub fn next_capacity_byte_demand(&self, source: PrivateChildMemberMetadataSource<'_>) -> Option<usize> { Some(se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs:283 | `        pub fn next_capacity_byte_demand(&self, source: PrivatePublicationMetadataSource<'_>) -> Option<usize> { Some(se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs:15 | `    pub fn next_capacity_byte_demand(&self,source:store::MemberGenesisEnvelopeSource<'_>)->Result<usize,semio_framework_` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs:28 | `    pub fn next_capacity_byte_demand(&self) -> usize { if !self.finished && !self.closing && self.field == 8 && self.out` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs:114 | `    pub fn next_capacity_byte_demand(&self,source:&MountedGroupReceiptSource<'_>)->Result<usize,ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/📄️entry/🦀️.rs:69 | `    pub fn next_capacity_byte_demand(&self, source: MountedCommandEntrySource<'_>) -> Result` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🦀️.rs:47 | `    pub fn next_capacity_byte_demand(&self)->usize{if self.needs_page&&!self.admitted{size_of::<CommandPa` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:108 | `    pub fn next_capacity_byte_demand(&self, source: &MountedMemberReceiptSource<'_>) -> Result<usize, Val` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs:119 | `    pub fn next_release_byte_demand(&self, source: &MountedMemberReceiptSource<'_>) -> Result<usize, Valu` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️payload/🦀️.rs:46 | `    pub fn next_capacity_byte_demand(&self, additional: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:53 | `    pub fn next_capacity_byte_demand(&self, additional: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:133 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.pending_offset < self.pending_len { s` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:219 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.phase == MountedOperationFramingPhase` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:220 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { match self.phase { MountedOperationFramingPhas` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:337 | `    pub fn next_capacity_byte_demand(&self, source: &MountedKernelMutationReceiptSource<'_>) -> Result<usize, ValueError` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs:345 | `    pub fn next_release_byte_demand(&self) -> usize { if self.phase==MountedKernelReceiptPhase::ForwardRelease{self.forw` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📚️members/🦀️.rs:73 | `    pub fn next_capacity_byte_demand<M: SpaceMember + MemberFactory>(&self,group: &PrivateChildPublicationGroup<M>,actor` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📚️members/🦀️.rs:77 | `    pub fn next_release_byte_demand<M: SpaceMember + MemberFactory>(&self,group: &PrivateChildPublicationGroup<M>,actor:` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🦀️.rs:86 | `        pub fn next_capacity_byte_demand(&self, member: &impl SpaceMember) -> Result<usize, String> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs:54 | `    pub fn next_capacity_byte_demand(&self, parent_id: &str, parent_dialect: &ArtifactDialect, existing: Option<&Artifac` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:91 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.copy_bytes)}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:92 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.next_demand(body).map(\|d\|d.capacity_byte` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:93 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_demand(0).map(\|d\|d.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs:40 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs:42 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs:44 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📸️checkpoint-codec/🦀️.rs:101 | `    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(if self.bytes.is_some(){size_o` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📸️checkpoint-codec/🦀️.rs:102 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📸️checkpoint-codec/🦀️.rs:103 | `    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.bytes.as_ref().map_or(0,` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🎛️operation/📥️input/🦀️.rs:15 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.bytes.is_some(){size_of::<Option<Vec<u8>>>()}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🎛️operation/📥️input/🦀️.rs:16 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🎛️operation/📥️input/🦀️.rs:17 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.bytes.as_ref().map_or(0,Vec::capacity))}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs:78 | ` fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.copy_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs:79 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(bo` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs:80 | ` fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.release_b` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:388 | `    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:389 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs:390 | `    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.operation.as_ref().map_o` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🧵️recursive-wire-open-and-producer/🦀️.rs:202 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🧵️recursive-wire-open-and-producer/🦀️.rs:206 | `    fn next_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🧵️recursive-wire-open-and-producer/🦀️.rs:210 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🪦️snapshot-retirement-gesture-and-agent-transaction/🦀️.rs:23 | `        fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🪦️snapshot-retirement-gesture-and-agent-transaction/🦀️.rs:24 | `        fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_v` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🪦️snapshot-retirement-gesture-and-agent-transaction/🦀️.rs:25 | `        fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueErr` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📢️publication/🦀️.rs:43 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📢️publication/🦀️.rs:44 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📢️publication/🦀️.rs:45 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:42 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:43 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:44 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:69 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:70 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:71 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:108 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:109 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:110 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:142 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:143 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs:144 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🛫️encoder/🦀️.rs:88 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🛫️encoder/🦀️.rs:89 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🛫️encoder/🦀️.rs:90 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🪪️registry/🦀️.rs:41 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🪪️registry/🦀️.rs:42 | ` fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🪪️registry/🦀️.rs:43 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs:152 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demand().copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs:154 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs:156 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs:314 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs:317 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> ` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs:320 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs:193 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs:197 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs:201 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:155 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:159 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs:163 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/🗝️latest/🦀️.rs:16 | `    pub fn next_copy_byte_demand(&self,parts:[&str;4])->usize{if self.part==0\|\|self.part<=4&&self.prefix<16{1}else if se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:206 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:210 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:214 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧾️authority-and-journal-sink/🦀️.rs:134 | `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::Dur` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧾️authority-and-journal-sink/🦀️.rs:135 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(store::dur` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧾️authority-and-journal-sink/🦀️.rs:136 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(store::durable_group::Du` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:52 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_owner_demands(&s` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:53 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_o` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:54 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_owner_demands(&` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs:106 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.snapshot.next_capacity_byte_dem` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs:22 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs:65 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.retirement.as_ref().map_or(Ok(0), \|retirement\|` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs:143 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.as_ref().map_or(Ok(0), \|owner\| owner.n` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:654 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:655 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:656 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:109 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:110 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs:111 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📥️lifted/🚪️open/🦀️.rs:196 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📥️lifted/🚪️open/🦀️.rs:197 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retir` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📥️lifted/🚪️open/🦀️.rs:198 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:80 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:81 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:82 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:476 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:477 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:478 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:544 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_owner_demands(&s` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:545 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_o` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:546 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(semio_os_store_read::retirement_owner_demands(&` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:143 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:144 | `    pub fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:145 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(if self.allocated == 0 { 0 } else { PAGE_BY` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:172 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_copy_byte_demand(self` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:173 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_capaci` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs:174 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_release_byte_demand(` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:182 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:183 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:184 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:245 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:246 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:247 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:286 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:287 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:288 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:346 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:347 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:348 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:224 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:225 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:226 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:248 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:249 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:250 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🎟️admission/🦀️.rs:16 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🎟️admission/🦀️.rs:17 | ` pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.retirement_demands(body)?.capacity` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🎟️admission/🦀️.rs:18 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:69 | `    pub fn next_capacity_byte_demand(&self, source: MemberGenesisEnvelopeSource<'_>) -> Option<usize> { if self.closed \|` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:80 | `    pub fn next_copy_byte_demand(&self, source: MemberGenesisEnvelopeSource<'_>) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:143 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:144 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:145 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:445 | `    pub fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:467 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:468 | `    pub fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueErr` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:477 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_copy` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:478 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { MemberOpenReq` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:479 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_rel` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🏗️staged-root-retirement-and-commit-phases/🦀️.rs:91 | `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueErr` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🏗️staged-root-retirement-and-commit-phases/🦀️.rs:92 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usi` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🏗️staged-root-retirement-and-commit-phases/🦀️.rs:93 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::V` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🗺️map-assembly-and-decision-laws/🦀️.rs:416 | `        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🗺️map-assembly-and-decision-laws/🦀️.rs:417 | `        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🗺️map-assembly-and-decision-laws/🦀️.rs:418 | `        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/📔️fake-journal-and-map-assembly-fixtures/🦀️.rs:94 | `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/📔️fake-journal-and-map-assembly-fixtures/🦀️.rs:95 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError>` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/📔️fake-journal-and-map-assembly-fixtures/🦀️.rs:96 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.ret` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs:90 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs:91 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(self.close_demand()?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs:92 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demand()?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:76 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:77 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:78 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, Vec::capacity)) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:196 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:197 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs:198 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, \|owner\| owner.cap` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧹️hydration-retirement-drop-and-completion/🦀️.rs:67 | `fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧹️hydration-retirement-drop-and-completion/🦀️.rs:68 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capaci` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧹️hydration-retirement-drop-and-completion/🦀️.rs:69 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs:56 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.ne` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs:57 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs:58 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.n` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:60 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:61 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:62 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:98 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:99 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:100 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:167 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:168 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs:169 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs:381 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return art` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs:382 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if let Some(active)=self.active.as_r` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs:383 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return ar` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:92 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:93 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:94 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:208 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:209 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:210 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:349 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:350 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs:351 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🦀️.rs:68 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🦀️.rs:69 | ` fn next_capacity_byte_demand(&self,_body:usize)->Result<usize,ValueError>{Ok(self.demands()?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🦀️.rs:70 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📸️snapshot-alias/🦀️.rs:52 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.copy_bytes)}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📸️snapshot-alias/🦀️.rs:53 | ` fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{Ok(self.next_demand(b)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/📸️snapshot-alias/🦀️.rs:54 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📤️edit/🚪️admission/🦀️.rs:19 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.close_demands(4096)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📤️edit/🚪️admission/🦀️.rs:20 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original.close_demands(body)?.capacit` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📤️edit/🚪️admission/🦀️.rs:21 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.close_demands(4096)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/📋️operations/🦀️.rs:125 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/📋️operations/🦀️.rs:126 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/📋️operations/🦀️.rs:127 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/🦀️.rs:167 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/🦀️.rs:168 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/🦀️.rs:169 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs:221 | `    pub fn next_capacity_byte_demand(&self) -> Result<usize,ValueError> { if self.source_identity.is_some_and(\|identity\|` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🧹️hydration-retirement-drop-and-completion/🦀️.rs:72 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.retirement_demands(0).map(\|demand\| demand.copy` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🧹️hydration-retirement-drop-and-completion/🦀️.rs:73 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.retirement_demands(copy).map(\|d` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🧹️hydration-retirement-drop-and-completion/🦀️.rs:74 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.retirement_demands(0).map(\|demand\| demand.rel` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs:595 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs:596 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs:597 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs:130 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs:131 | ` fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs:132 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🦀️.rs:231 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.copy_bytes)}` |
| a | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🦀️.rs:232 | `    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🦀️.rs:233 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(4096)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:29 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if !self.closing&&!self.terminal_is_empty(){si` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:30 | `    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:31 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if !self.closing&&!self.terminal_is_empty(){re` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:237 | `    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), \|cursor\| if cu` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:238 | `    pub fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:239 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.cursor.as_ref().map_or(Ok(0), \|cursor\| if` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:43 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:46 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📥️lifted/🔗️read/🦀️.rs:49 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:192 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:193 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(copy)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:194 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:267 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:268 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(copy)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🛑️cancel/🦀️.rs:269 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs:54 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs:55 | `    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs:56 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.cursor.next_close_byte_demand()}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs:14 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs:15 | `    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs:16 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.active.as_ref().map(Vec::capacity).or_else(\|\|se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🦀️.rs:158 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands()?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🦀️.rs:159 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.demands()?.capacity_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🦀️.rs:160 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands()?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:76 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:77 | `    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:78 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🦀️.rs:58 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),\|owner\|if owner.termina` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🦀️.rs:59 | `    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),\|owner\|if ` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🦀️.rs:60 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),\|owner\|if owner.termin` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/♻️retirement/🦀️.rs:67 | ` fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/♻️retirement/🦀️.rs:68 | ` fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/♻️retirement/🦀️.rs:69 | ` fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs:562 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs:563 | `    fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs:564 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.schema.as_ref().map(\|value\| value.capacity` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/📦️owned/🦀️.rs:15 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs:36 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.owner.next_capaci` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🦀️.rs:250 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🦀️.rs:251 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacit` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🦀️.rs:252 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs:736 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs:737 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs:738 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:129 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, \|demand\| demand.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:130 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.demand_axis(maxim` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:131 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, \|demand\| demand.release_bytes)` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:218 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:219 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximu` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🚫️decode-rejections-and-erased-retirement/🦀️.rs:220 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🎟️field-decoder-registry-and-empty-array-authority/🦀️.rs:394 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.de` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🎟️field-decoder-registry-and-empty-array-authority/🦀️.rs:395 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🎟️field-decoder-registry-and-empty-array-authority/🦀️.rs:396 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs:501 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs:502 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs:503 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:174 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:175 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs:176 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:65 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:66 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:67 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:160 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:161 | `    fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📥️lifted/🔗️read/🦀️.rs:162 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:132 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:133 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:134 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:254 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:255 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs:256 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:90 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:91 | `    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs:92 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:80 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:81 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:82 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.buffer.as_ref().map_or(0,Vec::capacity))}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:98 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefu` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:99 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value:` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🔄️refused-reload-and-erased-close-demand-laws/🦀️.rs:100 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRef` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:208 | `        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:209 | `        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:210 | `        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:272 | `        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:273 | `        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:274 | `        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:324 | `        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:325 | `        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs:326 | `        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs:210 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs:211 | `    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs:212 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/💨️ephemeral-one-item-publication-laws/🦀️.rs:32 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/💨️ephemeral-one-item-publication-laws/🦀️.rs:35 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/💨️ephemeral-one-item-publication-laws/🦀️.rs:38 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🦀️.rs:303 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🦀️.rs:304 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🦀️.rs:305 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:96 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:97 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.ca` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:98 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs:93 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs:94 | `    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximu` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs:95 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs:56 | `            fn next_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs:59 | `            fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ::semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs:62 | `            fn next_release_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs:16 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{Ok(match self.phase{Phase::Select=>self.selection.nex` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs:17 | ` pub fn next_copy_byte_demand(&self,input:&ReplayMessageAccumulator)->Result<usize,ValueError>{Ok(match self.phase{Phas` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs:18 | ` pub fn next_release_byte_demand(&self,input:&ReplayMessageAccumulator)->Result<usize,ValueError>{match self.phase{Phase` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/📚️catalog/🦀️.rs:55 | `    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs:635 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs:636 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_byte` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs:637 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:95 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(\|demand\| demand.copy_bytes) }` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:96 | `    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.demands(copy).map(\|demand\| dema` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:97 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(\|demand\| demand.release_bytes)` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:169 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands(0).map(\|demand\|demand.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:170 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.demands(copy).map(\|demand\|demand.capac` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🦀️.rs:171 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.demands(0).map(\|demand\|demand.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/🪟️wasm-session-and-input-application/🦀️.rs:181 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{match self.phase{` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧭️topology/🦀️.rs:104 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demands(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧭️topology/🦀️.rs:105 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.next_demands(copy)?.capacity_by` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧭️topology/🦀️.rs:106 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demands(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:94 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.selected(0)?.copy_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:95 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.selected(copy)?.capacity_bytes` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:96 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.selected(0)?.release_bytes)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:339 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:350 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:362 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:517 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:518 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){se` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:519 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:78 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}else{self.registry` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:79 | `    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}els` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:80 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:140 | `    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:141 | `    pub fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError> {Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry/🦀️.rs:142 | `    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🦀️.rs:97 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,semio_framework_value::ValueError>{self.owned.next_capa` |
| d | 🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🦀️.rs:98 | `    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.owned.next_release_byte_dem` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:119 | `    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(!self.terminal))}` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:120 | `    fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError> {Ok(0)}` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:121 | `    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(0)}` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:197 | `    pub(super) fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.demand(\|owner\|owner.next_copy_byte_demand` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:198 | `    pub(super) fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.demand(\|owner\|owner.next_c` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:199 | `    pub(super) fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.demand(\|owner\|owner.next_release_byte_` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:777 | `    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_re` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:778 | `    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return` |
| d | ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:779 | `    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if !self.retain_terminal_is_empty(){return self.next_r` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/💾️binary/📸️snapshot/🦀️.rs:384 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/💾️binary/📸️snapshot/🦀️.rs:385 | `    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retir` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/💾️binary/📸️snapshot/🦀️.rs:386 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🫙️workspace/🦀️.rs:26 | `fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🫙️workspace/🦀️.rs:27 | `fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.` |
| d | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🫙️workspace/🦀️.rs:28 | `fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}` |
| d | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs:82 | `    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(` |
| d | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs:83 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) ->` |
| d | ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs:84 | `    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { ` |
| d | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:447 | `    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }` |
| d | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:448 | `    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> ` |
| d | ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs:449 | `    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retained_bytes.min(` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🖼️image/🦀️.rs:15 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ImageDecodeError>{match self.phase{0=>self.decoder.next_copy_byte_de` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🖼️image/🦀️.rs:18 | ` pub fn next_capacity_byte_demand(&self)->Result<usize,ImageDecodeError>{match self.phase{0=>self.decoder.next_capacity_` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs:154 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?[0])}` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs:155 | ` pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?[1])}` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧵️mounted/🦀️.rs:156 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?[2])}` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:574 | ` pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_copy_` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:575 | ` pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.ow` |
| d | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🦀️.rs:576 | ` pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.owner.terminal_is_empty(){self.owner.next_rel` |
