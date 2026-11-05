# Peer Root Handoff Contract

Permanent inputs and Bun test: plugin/👥️peer/🧪️tests/📤️handoff (JSON, schema and TypeScript). Actual final pre-fix run: 3 pass, 1 fail, 62 assertions, 138 ms. The source regression fails at close-ladder's PeerPresenceRoot::empty_shared substitution. This is the intentional source red before root's repair. An earlier broad source scan timed out under load; it was replaced with the exact authoritative close-ladder read, and the final failure is an assertion rather than timeout.

Fourteen neutral cases cover populated and empty owned roots against five denied admission currencies and accepted admission, an aliased retained root, and cancellation before any root was initialized. SQLite independently enforces unique root custody and verifies handoff changes its owner without introducing a replacement owner. Absent reads return a stable borrowed empty value; absent roots cannot admit aliases/publication. Empty roots remain real owners requiring funded retirement, just like populated roots. Separate accounting verifies a paid retirement frame and root release conserve physical ownership. The neutral grant and frame sizes are illustrative model quantities, not production demand quotes or proof of Rust physical allocation. Zero capacity is only a denial where the admitted operation actually requires a frame; inline root transfer must not invent a capacity requirement.

## Actual native retirement API

PeerPresenceRootRetirement lives in plugin/👥️peer/🦀️.rs. new(Arc<PeerPresenceRoot>)->Self retains the actual root inline; owned(PeerPresenceRoot)->Self accepts an already unique payload. retirement_demands(body:usize)->Result<RetirementDemand,ValueError>, close_step(RetainedCloneGrant)->Result<PluginLifecycleStep,ValueError>, terminal_is_empty()->bool. Arc aliases block without dropping the owner. Unique Arc handoff quotes sizeof(PeerPresenceRoot) copy and actual shared allocation release; subsequent entry cursors retire entries before final inline payload removal. No requirement to allocate a replacement empty root exists in this owner. The outer host must retain the cursor until terminal and release actual owned frames under their own quote.

The neutral tests do not prove actual Rust no-allocation behavior. Existing native test and GDB supply the real red: lazy empty_shared allocation during close. Fresh native close execution must establish green after the production repair. Root and runtime agent own those changes.


## Current production recheck

After root repair, exact permanent Bun command completed 4 pass, 0 fail, 62 assertions, 119 ms. The source invariant now passes without changing it. LadderRef and LadderCtx share peer_presence_detach_demand. Admission occurs before taking the actual root. Capture has zero capacity demand; an existing inline cursor delegates to its actual retirement_demands. Terminal cursor removal and close flag are a separately quoted transition after terminal_is_empty. The final-root path does not depend on retirement registry admission, so absent/cancelled partial initialization can close without registry backing. PeerPresenceRoot::EMPTY is an actual constant fixed Option array, replacing the allocating shared empty root. No concrete defect was found in this bounded review.

A zero capacity grant does not deny an operation whose actual capacity demand is zero. Neutral five-currency denials apply to a positive-demand abstract admission; they must not be read as a requirement to manufacture capacity cost on inline capture. Exact native target sizeof/32 KiB grant fitness and allocator receipt parity still require the actual native run. Source review alone does not certify those physical properties.

## Actual Metadata RED And Production Fix

The native peer metadata selector executed with an actual HeapWitness: zero passed, three failed, seven filtered. The empty owned root changed under zero copy allowance; sparse cursor progress reported zero copied bytes against its quote of eight; a terminal child owner was removed despite denied copy allowance. This is genuine executed law RED, not a compilation refusal. The log is 🗑️generated/[DEBUG]repair-peer-presence-metadata-red-fault.log under this ticket.

Root then applied the prepared production patch. It prices original Arc/Option-to-inline transfer, sparse index and Option writes, and final inline-owner clearing. A completed child cursor remains owned until a separate funded parent clear; no child completion drops the parent Option for free. The redundant root.len write before terminal removal is removed. AST parsing passed. The exact same native three laws and full app close are queued to validate this source; no native GREEN is claimed yet, and no budgets or loop bounds were increased.

## Actual Native Metadata GREEN

The exact peer metadata selector has now executed after the production fix with the real HeapWitness:3passed0failed7filtered. Empty-owned clear, sparse cursor and separately funded terminal child cursor clearing all passed under the unchanged grants. The native test output is in 🗑️generated/[DEBUG]repair-peer-presence-metadata-green-work.log; outer repository wrapper was finalizing when reported. Full actual app close remains a separate pending gate, so these three laws do not establish end-to-end editor completion.
