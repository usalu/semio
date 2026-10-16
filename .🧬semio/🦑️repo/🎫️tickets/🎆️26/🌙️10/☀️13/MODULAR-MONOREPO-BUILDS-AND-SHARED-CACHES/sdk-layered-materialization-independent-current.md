# SDK Layered Materialization Independent Audit

Source-only review, 2026-10-15. No probes, tests, SDK runs or image reads.

No concrete new lifetime, parent-order, accounting or authority defect established in the inspected production pool change.

`derivative/🟦️.ts:45` now awaits the existing pure layer walker over every captured stamp. Each operation retains actual source/output materialized-parent checks and path ceiling. Directory mkdir, symlink target resolution and creation, owned copyFile plus chmod, and shared link all remain awaited inside the accepted operation. Successful counters increment once after their actual operations; there is no await between a counter increment and the modulo progress decision, so concurrent completions do not lose increments. The same captured entries/owned extent allowance and namespace remain.

`derivative/📚️layers/🟦️.ts:7-11` validates unique paths and total count before effects, retains schema-owned maximumParallelOperations=16, completely drains parent-depth layers before children, and records failure separately from its thrown value. Accepted operations settle before rejection for Error or undefined. Thus an accepted copyFile/chmod or link cannot remain running after the pool rejects. New operations stop after the first observed failure; accepted sibling operations may finish, which is necessary for draining.

Copy still resides in the original private single-use projection state.copy promise and Root accepted-copy scope. The pool changes no base SH/EX transition, provision registry custody, store/control identity, quote scope, or outer publication/projection draining. Final actual source recheck occurs after all successful materialization operations complete. Failure unwinds only after accepted writers settle, before accepted-copy scope retires.

The new `derivative/📤️copy/🧪️tests/🟦️.ts` invokes the actual projection, issued quote, retained publisher and production copy on a tiny owned base. It observes actual copyFile/link promises, independently checks byte totals via SQLite and actual SHA/inode/mode results, and asserts active=0, minimum parallelism and upper width. The old serial implementation can falsify the minimum width assertion. This is a meaningful actual materialization test, not a Source string check. Its synthetic paths have nested parents, so actual mkdir absence would fail writes.

The inspected new actual copy test covers successful completion, not an injected filesystem Error/undefined. Generic production layer tests previously address that failure-drain contract; an actual fault-injected materialization test would strengthen cross-boundary validation but no failure behavior defect is established by this review. Root's reported RED/GREEN/strict outcomes were not rerun or independently claimed here. Full SDK runtime performance remains unqualified.
