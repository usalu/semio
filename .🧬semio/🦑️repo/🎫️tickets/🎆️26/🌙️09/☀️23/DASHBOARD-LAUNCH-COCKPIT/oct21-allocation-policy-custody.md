# Explicit Paged Allocation Policy and Retirement Test Closure

## Observed Compiler Boundary

Affected-eight request 69671 settled whole1 after six real retirement library-test compiler errors and then a separate strict Source revision cleanup refusal. No named test body or result summary ran. The log is generated/e2e-value-required-close-affected-eight-retry.log. This is not a new red for the underlying allocator behavior.

## Deliberate First-Party Allocation Interface

The existing PageAllocation interface is now an unsafe first-party policy contract: preserve existing elements, order and ownership, and leave vector/capacity unchanged on Err. It exposes no external library API. The actual public reserve_one_using path retains the same admission, allocation strategy, limits and receipts. Returning insufficient capacity on Ok is recoverably refused. Metadata with zero slots is rejected before Vec.push can allocate implicitly. Partial payload storage stays owned and counted with zero published logical slots; retries refuse until that storage is settled.

All four relevant selectors now agree on physically retained non-ZST payload capacity even when logical slots remain zero: byte demand, depth demand, parent handback and physical release. There is no automatic unquoted disposal. Underpaid release/handback leaves ownership unchanged; a parent can receive the same allocation with zero physical release and must later fund its whole deallocation. Default and excess-allocation behavior remains unchanged.

## Authored Proof

Closed neutral rows cover zero, partial, exact and excessive capacity; independent SQLite computes admission, logical slots, retained bytes and quarantine. Three real native laws check zero-capacity metadata refusal without hidden allocation, partial16-byte retention/depth3 then metadata2/funded release, and parent handback with zero disposal followed by separately funded physical release. Existing valid allocation policies explicitly implement the unsafe contract, including allocation failure that preserves the vector. Mutation on Err is a contract violation and is not normalized as recoverable success. These laws are unrun.

The six compiler diagnostics are addressed through actual owners: the existing test-only factory owned export uses its semantic owned module; turn tests import current factory and queue owners with the correct erased-trait identity; the local failure fixture implements that actual queue trait; the intentionally unsupported fixture explicitly returns false; allocation-failure fixtures can use the real public policy port. No legacy namespace alias, duplicated owner module, grant padding, assertion removal or Cargo edge was added.

## Runtime and Source Plan

Original sixteen owners and sixty-nine advertised triples remain unchanged. Wire-core is the seventeenth supplemental whole-library owner and paged-list is the eighteenth; the three new reservation bodies remain separate supplemental evidence. The existing reservation Bun/SQLite test has no selected declared route yet. A genuine Source36 leaf is prepared for the next permitted manifest boundary; registration stays held while the sibling Source61470 is active.

## Strict Source Observation

A read-only current lstat of the revision MJS observed a regular nonsymlink file,8795bytes,nlink1. This does not establish the historical cleanup refusal cause or peer ownership, and no guard or file identity was changed.

## Coherent Compiler Retry 49246

The exact owned request settled whole101 after actual compilation, with zero named bodies/result summaries. All six previous retirement test-closure diagnostics are gone. Its only errors were two public integration fixture references to a private PageList import under the paged facade. Independent review confirmed the actual queue owner explicitly publishes the same first-party list module; both test references now use that existing public path. No export, alias, dependency, funding or assertion changed. The next affected-eight runtime remains required.

## Registered Reservation Source Oracle

At the released post-72855 metadata boundary, the existing value canonical script/project now declare @semio-tech/value-rs:test-paged-reservation-source. The no-argument route awaits the actual Bun test under the Original signal and its capped30-second remaining budget. Only the test module is a runtime entry; schema, fixture and inspected list Rust are freshness DATA. The target disables caching and forwarded arguments, has no dependency edges, and declares English/German labels. Independent source review accepted the real route/control/input shape. This is Source readiness only; its actual Bun/Ajv/SQLite bodies remain pending. The original26-row roster is preserved, with High retaining this as extra37 before CI38.
