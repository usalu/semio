# Mounted App final backing retirement

The native registered-window fixture previously reached its App frame release at turn 3350. Its original receipt reported 1,003,424 released bytes, while the allocation witness measured 29,726,680. The 28,723,256-byte discrepancy was in empty container storage still owned by the mounted frame. A Darwin free stack trace identified the allocations by symbol, including the store replacement registry (8,732,672 allocator-usable bytes), envelope decoder (5,636,096), both media registries (5,210,112 each) and peer roster (1,376,256). Smaller registries, child membership, reserved commits and typed-operation queues were also released by the final frame's generated drop glue. An additional trace including allocations below 256 bytes identified the two operation-owner Arc backings in that frame.

The active close ladder retires live values before the final backing rung. That rung now releases each empty backing under its own exact release grant. Large fixed registries retain their authored slot count and identity rules across independently releasable allocations. Smaller registries use one flat allocation. A paged registry returns one actual page at a time and retires its pointer table separately. Other library consumers that close fixed registry backing use the same exact operation. Final mounted operation aliases transfer into the existing bounded alias-retirement owner so exclusive payloads and Arc shells have their own receipts.

The native identity fixture checks physical heap releases and reported releases separately on every funded turn. A neutral registry vector covers 64 large payload slots, denied grants, page release and release of the original pointer table. The mounted fixture exercises the actual registered window and its original sources.

The source admission closure measures 258,248 bytes across 17 files and 3,018 active lines for the instance app, 245,419 bytes across 13 files and 2,519 lines for ownership, and 198,591 bytes across 11 files and 2,686 lines for maintenance. All three fit the configured 262,144-byte, 32-file and 4,000-line limits. An independent C-layout oracle computes 64 releases of 131,080 bytes and one 1,024-byte pointer-table release for the neutral vector, totaling 8,390,144 bytes on this native host.

## Store factory and roster remainder

After paging and retiring the container and operation alias backings, the current app advanced to close turn 4585. Its post-capture scheduler witness passed. The original receipt covered the 1,003,936-byte App frame, and the heap witness observed 1,004,368 bytes. A second free trace identified all of the 432 outstanding bytes in the mounted store factory aliases and the peer-roster cancellation root.

The closing instance now advances those aliases through the existing bounded factory and cancellation retirement cursors before it returns the App frame. A fresh native compile and exact fixture run passed every local receipt check, including the final App frame and the post-capture maintenance witness. The fixture then reached its first aggregate lifetime check.

The first rerun after the factory and roster change encountered a concurrent dependency revision. The store caller saved at 06:33:40 UTC and called the borrowed refusal projection before the receiving outcome module saved its public form at 06:37:04 UTC. The latest receiving crate already exports that method on the exact type the store imports. A fresh Cargo invocation compiled this production store batch successfully for the native Energy witnesses and again for the scoped window fixture.

## Focused registry laws

Both native registry laws passed against the actual compiled ownership crate. The slot law checks both sides of each allocator page boundary, direct-map collision refusal, ordered disjoint iteration and in-place mutation. The physical law checks that a one-byte-short grant changes neither the next quote nor the allocator, then observes exactly one allocation release at a time and reconciles all 65 releases with the original birth. The independent C-layout result matches the observed native allocation structure.


## End-to-end native allocation balance

The aggregate check observed 50,425,635 bytes born and 50,383,986 bytes freed. An independent Darwin allocation trace joined malloc, calloc, aligned allocation, realloc and free events by address within the fixture’s physical observation scope. It attributed all 311 surviving allocations and their exact requested total of 41,649 bytes.

- The declaration bootstrap installed 22,033 bytes of process-global action-bus registrations, maps and interned action names. The fixture primes this same registration once outside its instance accounting, then creates and retires each measured declaration through its actual sources.
- The fixture caller had 6,864 bytes still live at the assertion, including its original return escrow and recorded clock. Its funded retirement now runs before the aggregate comparison, and the heap witness includes its admission and retirement.
- Store genesis skipped the final frame-release turn after the child payload became terminal. Four original root tickets (9,616 bytes) and four pack-lease tickets (3,136 bytes) were lost when the active slot was overwritten. The owner now retains the terminal child until the existing boxed-retirement helper quotes and releases that exact frame.

The completed native run verified the mounted App close turn-by-turn. The aggregate correction awaits a fresh linked fixture run.

## Current contiguous batch admission

The next native build reached a live batch-pages change in the Store cursor. The member admission still invoked the superseded lower method. It now calls the current paged admission with no page owner, matching the facade’s existing contiguous input path. That path still quotes and transfers the original contiguous mutation owner.
