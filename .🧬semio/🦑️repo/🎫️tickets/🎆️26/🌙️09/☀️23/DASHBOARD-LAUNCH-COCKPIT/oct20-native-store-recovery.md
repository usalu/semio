# Native Store Recovery Diagnosis

## Retained Failure And Current Physical Observation

Controls-owned focused session 71631 settled whole-command exit 1 before any test body. Its retained log is generated/e2e-value-persistent-close-diagnostic.log. It ends with Resource lease timed out: cache-prune in acquireResourceLeaseBody, while automatic compact compiler headroom recovery is admitting maintenance. A registry queue earlier names 27700, but that is not evidence that 27700 holds cache-prune.

Read-only node system statfs observation at 2026-10-20T12:58:39.651Z of the actual existing backing /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m: available 4,260,958,208 bytes; filesystem 16,969,064,448 bytes. Exact compiler-prototype.json receipt requires 4,294,967,296 bytes, owner repo:compiler:prototype, allocation 77aa244eca741fcaf6808848249ad2f2. The current shortfall is 34,009,088 bytes. This current observation is not an invented exact statfs sample from the earlier failure. Authored strictStorage policy is 16 GiB physical capacity, 4 GiB maximum workspace, 1 GiB maximum artifact. No limit was raised.

## Exact Scope And Refusal Semantics

- Compiler capacity/withCompilerWorkspaceWriteV1 borrows the authentic workspace, then withCompilerInitialHeadroomV1 acquires the genuine writer before checking real statfs.
- The compiler store lifetime supplies recoverCompactCompilerHeadroomV1 through a privately retained callback. Recovery authenticates same Original/refined control, reservation and live writer; it checks allocation.kind compiler-workspace and requires the full allocation bytes.
- In the ordinary launcher admitted closure, withCompilerWorkspaceWriteV1 is entered before acquireCargoBuildLeaseV1. The direct initial recovery therefore has no newly acquired Cargo profile from that closure. Registry/configuration input custody can already be held. Nested operations can occur under a retained outer profile, so globally changing automatic recovery to wait:true would risk a self-wait and is unjustified.
- Automatic recovery deliberately calls retireInactiveCargoStoreV1 with wait:false. Its inactive maintenance acquires cache-prune exclusive before exact build/target exclusive leases; process inventory, physical locks, complete descriptor custody, recensus and byte limits remain strict.
- acquireResourceLeaseBody first acquires the directory admission gate. On directory-admission timeout, it rewrites the message to the originally requested resource. Thus the current cache-prune timeout alone does not prove that cache-prune itself is held. The nonblocking request can fail on the global directory gate.

## Existing Supported Owner And Reachability Gap

The existing InactiveCargoRetirementScript accepts preview,retire,preview-historical,retire-historical and is registered by the canonical caching 📜️script.ts. Current-domain preview/retire opens the exact rust partition and existing allocation, acquires compiler writer custody first, then calls the strict inactive retire owner with wait:true. It has no compiler profile before maintenance and is the appropriate quiescent recovery action. Default policy retains 4 GiB, targets 4 GiB available and deletes at most 4 GiB under the existing ownership guards.

There is currently no authored 📋️project.json target/workspaceCommand for cache-retire-inactive. The advertised repo:cache-prune target is a distinct generic age/budget operation using configured Cargo roots; it must not be presented as equivalent current compact-domain maintenance. No raw Cargo, filesystem removal, copied receipt, forced lease or foreign signal was used. Controls started unchanged retry 70494 after the first refusal; no maintenance job runs beside it.

## Relevant Physical Owners

- library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts
- library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/♻️recovery/🟦️.ts
- library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
- library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts
- library/⚡️caching/🧹️pruning/🏪️budget/🟦️.ts
- library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts
- library/⚡️caching/🧹️pruning/🟦️.ts
- framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts

All library paths above are relative to 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️. Source review is distinct from actual maintenance and subsequent native test acceptance.
