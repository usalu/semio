# Process Workshop Native Footprint TDD

## Actual Gates

The canonical `@semio-tech/ui-runtime-rs:test` route now awaits its test runner and executes the strict Ajv/Node allocation oracle (10 checks) before native tests. No production reconciliation accounting has changed.

- `🧪️surface-ownership-red-r1-native-2026-08-27.txt`: actual semantic RED, 0 passed / 1 failed / 83 skipped / 1 not run, 0.022 seconds. A TreeItem `icon: None → Some(UiText)` leaves its allocation unchanged but adds **1,536 resident-credit bytes** (22,224 → 23,760) and three semantic items. The second binding vector did not execute after this assertion.
- `🧪️surface-ownership-inventory-r2-native-2026-08-27.txt`: actual native layout inventory PASS, 1 passed / 84 skipped, 0.013 seconds. The six capacity vectors agree with `std::alloc::Layout`, independently reproduced by Node Buffer and validated with Ajv. This is the current native ABI, not a Wasm size claim.
- `🧪️surface-ownership-red-r3-native-2026-08-27.txt`: the revised assertion collects both vectors before asserting; actual RED 0 passed / 1 failed / 84 skipped, 0.029 seconds. Observed inline deltas are **[1,536, 3,072]**, required **[0, 0]**. Both actual paths executed.
- `🧪️surface-ownership-inventory-r4-native-2026-08-27.txt`: expanded native inventory PASS 1 / 84 skipped. Includes all fixed cursor arrays, semantic traversal backing, and retained-job allocation below.
- `🧪️surface-ownership-launch-r1-2026-08-27.txt`: canonical plugin-registry launch generation PASS. The exact ownership gate is registered from the authoritative launch seed.

## Native Layout

| Owner | Bytes |
| --- | ---: |
| TreeNode | 6,384 |
| UiNodeRecord / UiPatchOp | 6,320 each |
| ActionBinding | 2,072 |
| RowAction | 3,104 |
| FlatPresentedNode backing, 128 slots | 822,272 |
| New retained record backing, 128 slots | 809,984 |
| Key-index backing, 128 slots | 69,632 |
| Traversal backing, 64 slots | 4,096 |
| Postorder backing | 2,048 |
| Seen-key backing | 68,608 |
| ID backing | 2,048 |
| Removal backing | 3,072 |
| Semantic-value stack | 101,888 |
| Each tree-retirement stack, 4,097 slots | 229,432 |
| UiPatchOps backing, 1,153 slots | 7,286,960 |
| Inline cursor | 39,272 |
| Inline reconciler | 680 |
| Boxed retained-job allocation | 54,736 |

The first patch operation lazily allocates the whole `UiPatchOps` backing. At that point the cursor still owns its flat backing and new-retained backing. Those **three real allocations alone total 8,919,216 bytes**, exceeding the unchanged 8,388,608-byte limit by **530,608 bytes**, before key indexes, traversal, retirement stacks, binding/row-action allocations, pending fields, or old state. Thus removing duplicate inline charges cannot establish honest acceptance under the current representation.

## Credit Meaning And Simultaneous Owners

`reserve_surface_reconcile` reserves bytes in the aggregate admission ledger. `take_ready` then calls `shrink_surface_reconcile(state.usage)` and splits the same credit between the candidate reconciler and ready patch; those owners release the ledger credit only after retirement. The credit is therefore retained allocation admission, not merely cumulative traversal work.

During discovery, the presented root/remaining child boxes and child-array backings coexist with the cursor's preallocated flat/seen/ID/postorder/new-record/new-index/removal storage. Moving a node from its child box to a flat slot releases the box but does not remove the fixed flat backing. Discovery must account for the still-owned tree separately from initialized flat payloads without charging the same inline field twice.

During diffing, `build_record_owned` moves payloads from a flat node into a new record; `FreshRecordClone` creates a second payload for an Upsert patch. The old reconciler is a third independent owner during replacement, with an earlier persistent credit when admitted. `UiValue` list/map clones retain arena aliases rather than recursively copying their payload. The fixed patch-vector backing nevertheless reserves all 1,153 full enum slots even for one patch.

Before the first repair below, Finalize's `take(&mut new_retained)` and `take(&mut new_key_index)` invoked their eager Default implementations. The candidate received the original backing while the retiring cursor received newly allocated empty full-capacity backing. This was another real overlapping allocation, not a semantic copy to ignore. The first production unit below makes these move transfers allocation-free while preserving exact capacity on the moved owner. Initial fixed arrays still initialize every `Option<T>` even when no payload is present; initialized payload accounting must remain separate from allocation capacity.

After diff completion, cursor and old-state retirement precede Ready. The candidate record/index backing and ready patch backing remain alive together and share the new admission credit. Published renderer/document owners have their own lifecycles and cannot be assumed to be deep copies paid by the census's arbitrary multiplier of three.

## Repair Boundary

Keep traversal/copy work separately metered while removing inline-field duplicate resident charges. Do not replace true backing capacity with initialized length. A complete fixed-8MiB repair also needs a bounded patch-storage representation (or another exact ownership transfer design) that does not allocate 1,153 full 6,320-byte enum slots on the first operation. That storage change touches UiFixedList/UiPatchOps, owned by the UI-contract lane, and must be coordinated before claiming a complete physical census or Process acceptance. No quota was raised and no resident allocation was hidden in this TDD packet.

Fresh Process workshop acceptance and Wasm physical-layout validation remain unrun.

## First Production Unit: Allocation-Free Backing Transfer

The existing `SurfaceFixedVec::take_all` used `mem::take`, allocating a new full backing for the emptied source. Its new neutral transfer vector is independently reproduced with Node's transferable ArrayBuffer. Native R5 (`🧪️surface-ownership-transfer-red-r5-native-2026-08-27.txt`) actually failed 0/1/85 skipped, 0.016 seconds: the four-slot source retained 64 replacement bytes instead of zero.

The corrected `take_all` transfers the original boxed backing and replaces it with an empty zero-length box. The moved-from vector rejects writes without admitted storage, returning the exact payload. Both Finalize map/index transfers and both reconciler Drop handoffs now use this exact transfer instead of Default. Native R6 (`🧪️surface-ownership-transfer-green-r6-native-2026-08-27.txt`) passed 1/85 skipped, 0.012 seconds: original pointer and 64-byte backing preserved, source backing zero. This removes a real redundant allocation; it neither changes any quota nor treats allocated capacity as initialized length.

Native R7 (`🧪️surface-ownership-finalize-green-r7-native-2026-08-27.txt`) passed 1/86 skipped, 0.051 seconds: the actual active cursor reaches Finalize, moves both original record/index allocation pointers into the candidate, leaves both source backings at zero, and explicitly retires cursor/current/candidate/patch owners. This is a concrete active path, not only the container primitive. The original inline-charge RED remains intentionally unresolved until the complete physical/work census and paged patch-storage unit are joined.

Native R8 (`🧪️surface-ownership-rejected-payload-r8-native-2026-08-27.txt`) passed the strengthened primitive law, 1/86 skipped, 0.080 seconds. After transfer, an attempted push returns the exact original String value, allocation pointer, and 16-byte capacity; the source remains empty with zero backing. The language-neutral transfer case names this rejected payload, and the independent ArrayBuffer oracle rejects access to the detached source. No replacement allocation or hidden reserve was introduced.

Runtime regression R10 (`🧪️surface-runtime-regression-r10-native-2026-08-27.txt`) passed **86 tests / 1 skipped**, 2.345 seconds, using the explicit selector `not test(surface_ownership_inline_fields_)`. The one skipped test is the still-intentional physical-credit RED, so this is not a full runtime-suite or quota-completion claim. The preceding R9 invocation failed before tests because `--nocapture` lacked its Cargo-style `--` separator; R10 corrected only that invocation syntax.

## Paged Patch Storage Prerequisite

The UI-contract owner authored a separate native first-payload allocation law. Actual R13 (`🧪️member-ui-patch-storage-red-r13-native-2026-08-27.txt`) failed 0/1/116 skipped, 0.040 seconds: the first initialized patch owned 7,286,960 bytes, exceeding the intended exact directory-plus-one-page footprint of 33,992 bytes. This is a real allocation-representation RED, not a renamed quota. The specialized page representation is now being implemented in that owner lane; logical capacity 1,153 is unchanged.

The shared UiValue arena has separately fixed pages/collection/free-index backings. Cloning its List/Map handles retains aliases and does not allocate another complete value tree. The forthcoming resident census must keep that global allocation authority distinct from reconciliation-owned arrays/pages, while retaining actual traversal/copy/encoding work as a separate metric. Initial presented-tree ownership and current/candidate/patch/retirement lifetimes must all be counted before a fresh Process acceptance claim.
# Paged Runtime Admission Checkpoint

The next schema-first native law is an actual RED: `surface_ownership_binding_clone_requires_bounded_backing_and_copy`, 0 passed / 1 failed / 89 skipped, 0.197s, log `🧪️surface-binding-clone-red-r14-native-2026-08-27.txt`. One fresh-record bindings field allocated and initialized 66,304 bytes while reporting `Yield { nodes: 0, bytes: 0 }`. The independent Node Buffer oracle validates the 32 × 2,072-byte footprint.

The root coordinator assigned the shared generic UiFixedList paged backing and retained clone/comparison prerequisite to this lane. The design uses fixed-fanout metadata pages rather than an all-N directory allocation, separately admitted payload pages, actual allocated-byte counters, and in-place typed retirement. Logical N and wire order remain unchanged; a payload larger than the actual caller grant remains owned and refused. Full resident accounting waits for this actual clone/allocation authority, not a changed copy multiplier.

Runtime GREEN R12: **2 passed, 0 failed, 87 skipped**, 0.060s. The actual allocator trace has two allocation turns, maximum 27,672 bytes; operation placement reports its full 6,320 bytes. Cancellation after pending/directory/page/placement starts with 0/27,672/33,992/33,992 bytes and reaches terminal without any allocation during close. Quota refusal before the directory retains the exact pending SetRoot operation. Log `🧪️surface-patch-allocation-green-r12-native-2026-08-27.txt`.

Regression R13: **88 passed, 0 failed, 1 explicitly skipped**, 1.945s, `🧪️surface-runtime-regression-r13-native-2026-08-27.txt`. Only the already-established inline-resident-census RED is excluded; this is not a full 89-test pass.

Production runtime changes replace all four active cold patch pushes with an owned pending operation and separate physical pre-admission/allocation/placement turns. Cursor/retained/ready-patch close now invokes typed in-place patch retirement. The pending owner closes without manufacturing a payload page. Existing generic non-patch retirement and the full simultaneous resident-owner meter remain separate obligations.

The specialized UI patch storage passed its four native laws (R15, 4/0, 0.131s) and the full UI-contract suite (R16, 120/0, 5.512s). This alone does not make the runtime path bounded.

An independent active runtime test then demonstrated the remaining cold-builder bypass: `surface_ownership_patch_backing_is_admitted_in_separate_turns` failed with one allocation opportunity of 33,992 bytes (27,672-byte directory plus 6,320-byte operation page), exceeding the unchanged 32,768-byte runtime grant. Log: `🧪️surface-patch-allocation-red-r11-native-2026-08-27.txt`, 0 passed / 1 failed / 87 skipped, 0.103s. The schema-first fixture is checked against separate Node Buffer allocations before native execution.

The in-progress runtime correction stages the exact pending operation, then separately admits directory, page, and inline placement. It retains allocation-error byte ownership. Cancellation must retire an unplaced operation without first allocating a page; the UI-domain owner for that lifecycle is being added by Dag. Full resident/work census and fresh Process workshop acceptance remain open.

## Generic Paged List Checkpoint

The shared list now owns a fixed-fanout-16 metadata tree with separately admitted metadata and payload allocations. A payload page targets 4,096 bytes; an element larger than that page size requires its full physical size and is refused when the caller's grant is insufficient. There is no all-N descriptor allocation. Actual metadata and payload capacity are both retained in the resident counter. Safe mutable iteration uses a machine-width-bounded stack of borrowed iterators, without unsafe pointer aliasing.

Native generic-list R21 was a real compile RED (16 absent-API errors). R22 passed the first two laws, 2/0 with 122 filtered, 0.158s: 600 ordered u64 values used five separately admitted allocations and five exact releases; a 32,769-byte payload remained owned under the unchanged 32,768-byte grant. The subsequent full R23 ran 36 passing tests before the old three-word owner-layout assertion failed. The new fixed owner is six words, independent of logical N and payload size; this assertion was corrected without changing payload or quota limits. Its new binding-sized law passed with 32 × 2,072-byte payloads and 79,744 actual bytes including metadata.

R24 did not execute tests: the existing native helper rejected the requested --no-fail-fast flag. R25 ran 66 passing tests before the old UiPatchOp footprint fixture failed. The actual new operation size is 6,416 bytes, not the historical 6,320 bytes; directory plus first payload is consequently 34,088 bytes. The exact native fixture/schema and runtime footprint vector were updated to these observed representation sizes. Historical logs above retain their original measurements.

The cold conveniences try_reserve, try_push and Clone still perform whole-list work and are explicitly not a retained proof. Active runtime field cloning/comparison, original-owner cancellation, the full current/candidate/patch/retirement resident meter, and fresh Process acceptance remain open. No WGPU or generated plugin output was changed.

R26 completed the full shared UI-contract suite: **126 passed, zero failed or skipped**, 4.726s. Its four generic paging laws include zero logical capacity, seven zero-sized payloads, and releasing/reusing an empty tail while preserving the exact allocation pointer of a 512-item live prefix. Log: `🧪️member-ui-generic-pages-full-r26-native-2026-08-27.txt`.

The binding-copy owner was developed through R27's six missing-API compile errors, then R28's **2 passed / 126 filtered**, 0.039s. It owns the original list, candidate and pending single binding, separates physical allocation, one binding clone and placement, and uses nonblocking exact UiValue alias admission. Its cancellation law covers 13 frontiers at 1/64/4096-byte close grants, keeps a separately held reader alive, and verifies the actual arena mutex contention path returns unchanged progress. The copy and placement totals are each 66,304 bytes, split across turns of at most 4,096 bytes. No generic binding-list clone or whole-list drop is counted as one turn.

The active fresh-record field-5 path now uses that owner. Runtime R15 passed **1 test / 89 filtered**, 0.016s: 132 turns, candidate resident backing 79,744 bytes including metadata, 66,304 initialized bytes, maximum per-turn allocation 2,072 bytes and maximum placement 2,072 bytes. Both original and candidate reproduce the serde oracle and the cursor closes explicitly. Log: `🧪️surface-binding-clone-green-r15-native-2026-08-27.txt`. Targeted existing-record binding comparison and other component/field copies remain open; this is not a full renderer or Process completion claim.

### Counter And Actual Cancellation Gates

The addressable-counter law first failed natively in R29, 0/1 with 128 filtered, 0.019s: the previous grant-only precheck allowed a new allocation after the owned counter was already at the signed addressable limit. The corrected preflight checks requested ownership before allocating. Actual additions remain checked; the preflight and Vec's individually addressable backing bounds prove their sum fits usize. R30 passed two laws, 2/0 with 128 filtered, 0.031s. Its private test allocator actually reserves double Vec capacity for both metadata and payload: each over-admission error reports and retains the actual capacity, and later bounded release returns exactly those bytes. No production allocator override is exported.

Runtime cancellation R16 exposed a real earlier cleanup bug: retire_record_one's guarded children arm fell through to terminal when the children list was empty, skipping the bindings phase. The corrected children phase advances only after its values and empty backing pages are gone; original and partial binding owners then close through the exact binding-copy retirement authority.

R17 passed seven ownership laws, 7/0 with 84 filtered, 0.208s. Ten actual runtime cancellation frontiers, from untouched original through complete source/candidate copy, retired 79,744 through 159,488 retained bytes without any allocation during close and without releasing more than one admitted page per turn. Log: `🧪️surface-paged-ownership-r17-native-2026-08-27.txt`. The pre-existing inline resident-charge RED remains excluded and unresolved.

### Refreshed Native Physical Inventory

R17 measured current native sizes, not the historical layout:

| Owner | Bytes |
| --- | ---: |
| TreeNode | 6,456 |
| UiNodeRecord / UiPatchOp | 6,416 |
| ActionBinding / RowAction | 2,072 / 3,104 |
| Flat / retained record backings | 834,560 / 822,272 |
| Key index / seen backings | 69,632 / 68,608 |
| Traversal / postorder / IDs / removal | 4,096 / 2,048 / 2,048 / 3,072 |
| Semantic value stack / each tree-retirement stack | 101,888 / 229,432 |
| Patch directory / first payload page | 27,672 / 6,416 |
| Generic list / binding copy / pending patch fixed owners | 48 / 2,752 / 7,000 |
| Reconcile cursor / retained job allocation / reconciler | 43,712 / 59,920 / 680 |

The 7,397,648-byte full inline patch vector is now explicitly labelled hypothetical in the inventory test; it is not the current paged patch backing. These values are measurements, not an assertion that all simultaneous live owners fit the fixed 8MiB credit.

### Remaining Live Cold-Call Census

The active runtime still has whole component cloning in fresh field 1, children cloning in fresh field 3, and whole component/children/bindings comparisons and cloning in diff_record_field. AllocateIds still invokes the cold child-list builder; document snapshot capture still calls whole record.credited_clone. Fresh accessibility/menu and value-census aliases still need their exact work/lock audit. These sites must migrate to owned retained cursors; the new generic storage does not make their cold APIs bounded. Test-only reconcile/snapshot helpers are separately cfg-gated and are not production evidence.

### Coherent Regression And Width Checkpoint

R31 passed the complete shared UI-contract suite: **130/0, no skipped tests**, 3.384s, `🧪️member-ui-paged-copy-full-r31-native-2026-08-27.txt`. R32 then passed the canonical check-wasm route for wasm32-wasip2 (3.59s), wasm32-unknown-unknown (3.38s), and wasm32-wasip2 with typegen (4.07s), `🧪️member-ui-paged-copy-wasm-r32-2026-08-27.txt`. This compiles the current paged lists, copy owner, actual capacity counters and AtomicU64 across both guest targets; it is not fresh browser execution.

Runtime R18 passed **90 tests / 1 explicitly excluded test**, 1.835s, `🧪️surface-runtime-regression-r18-native-2026-08-27.txt`. The one excluded test is still the deliberate inline-resident-census RED, not a newly suppressed failure. Native source diff checks for the owned generic list, copy, typed retirement and runtime paths passed. No heavy Plugin/WGPU/app build, shared generated plugin publication, cleanup, ticket close or git mutation occurred.

The binding copy now stays borrowed in the retained record during its callback, rather than being moved to a temporary local while advancing. R19 passed three binding laws, 3/0 with 89 filtered, 0.104s. Its additional eight injected callback-unwind frontiers keep the same owner address and original backing reachable, then complete explicit cancellation. This structural retention—not a suppressed Drop panic—is the recovery authority. Log: `🧪️surface-binding-unwind-r19-native-2026-08-27.txt`.
