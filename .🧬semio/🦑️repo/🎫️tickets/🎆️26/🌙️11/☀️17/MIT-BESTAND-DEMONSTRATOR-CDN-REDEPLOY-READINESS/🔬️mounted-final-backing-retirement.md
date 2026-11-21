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

The mounted App close was verified turn by turn. A fresh run with the corrected measurement also balanced both normal registered-window lifetimes before it advanced to cancellation.

## Current contiguous batch admission

The next native build reached a live batch-pages change in the Store cursor. The member admission still invoked the superseded lower method. It now calls the current paged admission with no page owner, matching the facade’s existing contiguous input path. That path still quotes and transfers the original contiguous mutation owner.

## Cancellation assembly field limits

The first fresh funded run completed and balanced both full registered-window lifetimes, reaching the cancellation cases. A cancellation assembly turn then requested 48,209 copy bytes against a whole-turn copy grant of 48,057 (26,704 still available that turn). The refusal formatted its own error, accounting for the observed 1,540 requested and 660 released allocator bytes. The free and allocation stacks identify that formatting path through the native instance admission assembler.

The monomorphized native layout shows that the 24,096-byte ConstructorState was quoted as two copies plus a 17-byte cursor. The same path would later quote 53,937 bytes for the 26,960-byte HostEvents state. Each now passes independently funded field placements through the existing retained final-address assembly cursor. The compiled old host event layout’s largest selected field is 14,352 bytes. The existing original grants, setup and backing caps, receipt assertions and cancellation cases are preserved. The current source-closure check is 45,204 bytes for construction, 136,749 for stores, and 38,528 for history. The complete native cancellation witness is rerunning.

## Whole assembly quote census

The next linked fixture advanced from phase 3 to phase 4. The old phase 4 quote combined two copies of the 16,512-byte Store state, 16,568 bytes of donor headers and the 17-byte cursor for a total of 49,609 bytes. The exact ready cancellation slice was still 26,704 copy bytes after outer caller work. The reported 48,236-byte whole grant was the remaining aggregate stock when repeated blocked turns had consumed it. It was not the usable per-turn slice.

The linked native layout also identified over-slice quotes for whole typing and emission defaults and for the host route field after the first HostEvents split. The corrected constructor places typing, emission, host route and constructor defaults one field at a time through a shared typed initializer. Native Store fields retain their original objects from registration until each exact move into the private final address. This moves at most the selected byte range without constructing another value or relinquishing the original active cursor. The terminal handoff quoted 21,072 metadata bytes plus its cursor in the previous binary.

The neutral cancellation fixture now checks the exact production quotes of every reachable phase and field against the 26,704-byte measured copy slice before starting either identity case. It then covers the existing funded cancellation boundaries and measures every close turn. The updated admitted source closures include 49,457 bytes for construction, 134,300 for stores, 37,219 for history, 258,317 for the instance app, 192,808 for typed operations and 238,670 for the instance job crate. No component build was started for this verification.

## Native cache space

The active Window test’s exact offline Cargo package roster identified 381 native cache package trees that its dependency graph could never reference. The Energy owner confirmed those trees were idle. The current Cargo unit graph and on-disk feature fingerprints identified 49 additional inactive variants of packages shared by both witnesses. The cleanup removed 3.84 GiB of logical files while preserving every selected Window variant and the completed native Energy snapshot witness. The browser build queue was running concurrently, so physical volume samples also reflect its filesystem changes.

## Remaining cancellation placement bounds

The registered native quote census found three remaining entries beyond the measured 26,704-byte cancellation copy slice.

| Phase | Original demand | Funded placement |
| --- | ---: | --- |
| Command sources | 45,617 bytes | Place the typed and host command sources and their two scopes separately. |
| Latest-wins registry within ToolOperations | 32,625 bytes | Place each empty owner field separately. Range-copy absent update and retirement cursors while retaining the outbox field index and the remaining cursor order. |
| Media transfers | 174,577 bytes | Copy the absent export and download preparations by retained byte range. Place the five registry headers and remaining default fields separately. |

The Media registry capacity remains 64 slots per owner. Normal construction quotes each backing when it installs its header, and cancellation puts unallocated registries into the private frame. The updated semantic source closures are 50,162 bytes and 354 active lines in 5 files for construction, 202,637 bytes and 2,306 active lines in 10 files for command, 250,998 bytes and 2,391 active lines in 19 files for jobs, and 242,366 bytes and 2,443 active lines in 16 files for documents. Production source admission accepted these crates and the changed Store and history crates using the pinned WASI target configuration and unchanged 262,144-byte, 32-file and 4,000-active-line limits.

## Exact cancellation boundaries

The fresh linked Window fixture cleared the whole assembly quote census and both normal Window lifetimes. It then reached a fixture mismatch at the child backing marker. The marker compared one field’s exact quote with the full turn receipt, which also includes receiving and settlement work. The fixture now reads the actual read-only private initialization cursor and cancels after the specified frame, child backing, tool outbox, or partial absent media preparation. The per-turn allocator comparison still uses the full receipt. The resumed partial case separately checks zero, sub-cursor and cursor-only grants preserve the actual byte offset.

The opening range selector also used all remaining caller copy bytes before invoking the funded native constructor. Its receiving and settlement reservation therefore made full-size chunks permanently unaffordable for narrow opening grants. Opening now reserves that documented construction headroom before selecting the private placement chunk. Cancellation continues to use its direct Close grant. Production WASI source admission accepts the updated construction owner at 50,861 bytes and its reactor interface at 71,375 bytes.

## Verified native Window result

The fresh pinned native registration test passed in 5.08 seconds after linking. It checks every phase and field’s measured cancellation copy allowance, both original registered Window lifetimes and the funded post-capture maintenance turn. The actual native allocator matches the full receipt for every observed turn and balances each end-to-end graph through declaration, runtime, caller and decoder closure.

All four requested cancellation cases retire fully and leave the App unpublished. The final case advances partway through Media export preparation under the interactive copy allowance, holds that same retained byte offset through zero, sub-cursor and cursor-only opening grants, then closes the partially initialized native frame through its original caller. The positive native test used the original production factory and declared sources. The optional browser build remained with the release queue.

## Standalone native witness replay

The idle private Energy verification target and build tree were reclaimed after preserving hard links to the two linked witnesses. The immediate observed volume gain was 3,823,165,440 bytes. Both binaries reran independently from the retained witness directory after the target and build tree were removed. The Window test passed in 6.51 seconds and the Energy snapshot test passed in 22.09 seconds with its original 64 MiB test-thread stack. The default Darwin test-thread stack is insufficient for that debug Energy witness. These executions provide native evidence for the two focused fixtures.
