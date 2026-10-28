# IFC4x3 Core Review Fixes

The three actionable findings in `core-review.md` are resolved in the TypeScript editing core. This wave changed only the new IFC4x3 diff/mutation modules, their editing tests/feature, and ticket input scripts/reports. It preserved the shared BINARY implementation and the coordinator's native scalar refusal changes.

## Bounded Admission and Ownership

Graph validation uses lazy cursor frames rather than expanding every root/aggregate child into a frontier. Header/entity/complex root counts and proposed diff lanes are admitted before copies or large collection preparation. Aggregate child counts are compared with the remaining value budget before any child is read. Scalar identifiers and BINARY content use bounded chunks. Async root walking also yields while crossing empty entity/complex argument groups.

The retained independent `core-repro.ts` now reports **zero child reads** for the 100,000-child aggregate under `maxValues:3`, compared with the original 100,000 reads. New tests also inspect proposed command values and wide header roots through numeric-read-counting Proxies. Owned data is copied incrementally with one cursor frame per active object/array, without `structuredClone` or a wide per-child pending stack.

Direct diff/command results remain mutable public graph types and now own every returned branch. Mutating an unchanged result's entity, argument, complex member, header or scalar cannot modify the original input. History snapshots/events are likewise returned as owned copies.

## One Transaction and Atomic History

The central transaction constructs the next graph and inverse together while transporting original argument/entity coordinates. History executes that transaction once and uses its computed inverse; it no longer performs a full forward application in inverse construction and a second full forward application for publication. Command planning still handcrafts sparse target diffs and does not capture a whole-snapshot replacement diff.

Asynchronous command planning, command application, inverse command/diff preparation, sparse absorption/composition, history creation, snapshot/event reading, execute, undo and redo use the same bounded generator driver with progress and AbortSignal support. Sparse coordinate transport uses binary-search position mapping; sorting is a cooperatively advanced heap sort rather than one uninterruptible native sort. Expensive owned copies are also interruptible.

History preparation captures the original revision and state, constructs its next graph/event/owned public result, then checks cancellation and the revision immediately before changing snapshot, events, undo or redo. Two asynchronous commands targeting one revision publish exactly one event; the second refuses `staleRevision`. Cancellation during owned result copying leaves every history lane untouched.

History construction is explicit: `IfcEditHistory.create(...)` is asynchronous; `IfcEditHistory.createSync(...)` is the synchronous driver. Its constructor is private. Snapshot/event access has asynchronous `readSnapshot`/`readEvents` methods plus explicit synchronous variants. Existing synchronous execute/undo/redo methods accept options; their asynchronous counterparts are `executeAsync`/`undoAsync`/`redoAsync`.

## Public Facade Additions

The coordinator owns facade edits. Additional useful public exports from the diff module are:

- `applyIfcDiffTransaction`, `applyIfcDiffTransactionAsync`, type `IfcDiffTransaction`;
- `inverseIfcDiffAsync`, `absorbIfcDiffAsync`, `composeIfcDiffAsync`.

Additional public exports from the mutation module are `ifcMutationDiffAsync`, `applyIfcMutationAsync`, `inverseIfcMutationAsync`. Internal step drivers/helpers remain available to the sibling mutation module and need not be re-exported by the public artifact facade. `IfcEditProgress.phase` now also describes `admitInput`, `copyOwned` and `compare`.

## Verification

- Language-neutral scenarios were added to `🧪️tests/🧬️editing/🥒️.feature` before the ownership/async implementation. The owned-result test initially reproduced base mutation before the fix. The original standalone review reproduction provides the independent wide-frontier failure evidence.
- Final focused Bun editing suite: **23 tests, 191 assertions, zero failures**. Existing independent `fast-json-patch` output comparison and Ajv transport checks remain present. New async absorption also preserves independently applied patch output.
- Tests cover tiny budgets, proposed-input admission before cloning, mutable result ownership, all async command/inverse routes, transaction validation only once, cancellation during sorting and late owned-copy preparation, asynchronous creation/read cancellation, undo/redo preservation, returned-event isolation, and competing revision publication.
- Focused strict TypeScript check of both modules and their editing suite passes with `--noEmit --strict --target ES2022 --module ESNext --moduleResolution Bundler --allowImportingTsExtensions --skipLibCheck --types bun`.
- The retained independent `core-repro.ts` passes **1,864** sequential argument/composition/inverse combinations, refuses numeric binary content, reports **0** wide-child reads under the tiny budget, and reports `result mutation changes original false`.
- Whole-artifact build remains outside this evidence: the coordinator already recorded the unrelated framework-interaction dependency failure and enforcing Rust compiler ingress limitation. Neither was bypassed or repaired here.

Commands used the necessary focused Bun/TypeScript/source-tool fallback already documented for the unavailable repository execution route. No AGENTS, modifying Git command, worktree, build infrastructure or external runtime dependency changed. This wave created no generated output files; all retained input scripts and reports are inside the existing ticket.


## Subsequent Ownership Re-audit

A follow-up audit found a caller-alias race while the original implementation yielded between admission and owned copying. `core-ownership-fixes.md` supersedes that ownership boundary: bounded private capture precedes admission, captured command/event payloads stay consistent, final owned postconditions hold, and base-free coordinate/identity admission is strict. The latest focused suite has 31 passing tests and 244 assertions, with strict TypeScript and both retained reproductions passing.
