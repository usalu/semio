# Pack Controlled Ownership Origin Census

Read-only mounted-source audit, no source changes or Cargo. Root context receipt Nextest `47c94fde-4fc3-4e3f-b820-1730466602b9` executes 20 laws, 19 passed, one failed, 90 outside selection, 3.531 seconds. Actual failure is Binary WorkLimit with message `limit exceeded: retained deflate physical ceiling`.

The exact earlier producer is `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`, DeflateRetainedCursor::try_new lines495–501. Its parameters distinguish expected raw bytes, segment raw-length ceiling, and physical allocation allowance. Pack controlled format line61 computes remaining physical allowance as min(Native maximum_bytes, Pack max_total_alloc) minus already admitted Native ownership; line62 passes that to the cursor. The codec checks expected>limit separately, then constructs retained history using the allocation allowance.

`🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` try_new_retained line485 sets target=min(maximum_history_bytes, RETAINED_INFLATE_WINDOW_BYTES). RetainedInflateHistory::new lines331–334 has exactly two refusals: target exceeds physical allocation allowance, or allowance exceeds isize::MAX. Both concern owned backing/addressability; its current DeflateError::OutputLimitExceeded is converted at codec line501 to PackError::LimitExceeded, then active PackError::into_value_error line59 yields WorkLimit. Narrow correction at the line501 constructor conversion to ValueRefusal/OwnershipLimit is justified by this exact producer authority. Keep expected>limit and global LimitExceeded behavior intact. Do not classify by the error message.

Additional controlled format producer sites, before any broader follow-up:

| Site in 🎒️pack/📐️format/🦀️.rs | Actual authority | Narrow typed origin |
| --- | --- | --- |
| 23–24 check_controlled_allocation | cumulative owned backing overflow or Native/Pack allocation ceiling | OwnershipLimit |
| 29 controlled_vec byte multiplication | concrete collection backing byte overflow | OwnershipLimit |
| 61 inflater remaining allowance subtraction | existing owned backing exceeds physical ceiling | OwnershipLimit |
| 65 reserve_allocation failure | physical allocator refusal; current blanket discards typed retained allocation kind | Preserve actual RetainedInflateAllocationError kind (AllocationFailed/OwnershipLimit/InvariantViolated), rather than blanket OwnershipLimit |
| 99 segment allocation sum overflow | concrete growing buffer backing | OwnershipLimit |
| 100 try_reserve_exact failure | system allocation failure | AllocationFailed |

These later sites are source defects/candidates, not the first reached failure in the contextual JSON receipt. NativeDecodeControl::charge and allocate_vec already distinguish OwnershipLimit, AllocationFailed, and work progression; no new blanket kind mapping is needed there. No `NativeAlloc` identifier was found in the inspected framework/OS source; that name must not be invented as a mounted implementation.

Preserve controlled CRC workload/address range39, inflation work59, source offset/header/range arithmetic90–95, max_segment_len94, declared output-length consistency97, frame counts/span limits, semantic/type/depth parsing limits. In particular admitted output capacity48 is an invariant/declared-length issue, not automatically the caller allocation ceiling; it requires its own authority analysis before changing taxonomy. Ordinary noncontrolled allocation guards and retained catalog owner APIs are outside this focused repair census. The real active PackError identity is the replication codec enum used by mounted format and Store, rather than a similarly named standalone legacy file.
