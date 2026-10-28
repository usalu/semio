# IFC4x3 TypeScript Core Re-audit

Source-read-only review of `core-fixes.md` and current diff/mutation implementation on 2026-10-21. Only retained ticket repro/report inputs were added. Existing focused tool fallbacks were used because the repository launcher execution limitation remains recorded by the coordinator. No broad build or compiler-enforcement bypass was attempted.

## P1 — Validation precedes the owned copy while caller aliases remain live

File: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts:94`, `prepareIfcDiffSteps`, especially the final `validateIfcGraphSteps(next,...)` followed by `copyIfcOwnedSteps({snapshot:next,inverse,diff},...)`. History independently copies the original command at `🧬️mutations/🟦️.ts:79`, then publishes at `:82`.

`next` still refers to caller-owned base/proposed values during result validation. The incremental owned copy then yields, permitting ordinary user changes between validation and copying. The copied transaction is returned without validating its owned graph. The same problem applies to command values; history publication's final revision check cannot detect mutation of those caller objects.

Retained `core-reaudit-repro.ts` supplies an `onProgress` callback that changes an initially valid integer to the string `invalid` on the first `copyOwned` progress event. The direct async applier resolves successfully with `{kind:'integer',value:'invalid'}`. A second reproduction creates history, executes a valid `setEntityArg` command, and makes the same change to the caller's command value. `executeAsync` resolves and publishes revision **1**, then `history.snapshot` throws `invalidValue: integer must be an i64`. Thus the invalid graph is already committed and undo/history state has advanced.

The callback makes the interleaving deterministic; with `yieldEvery:1` the same mutation can happen through another task while the operation awaits. The synchronous driver invokes the same callbacks and is also vulnerable. Fix the ownership boundary: capture caller data into bounded owned inputs before exposing external callbacks, validate that owned authority, and ensure that subsequent work never rereads unowned caller values. Validate the final owned transaction before publication, including replay diff/inverse/event consistency, with no externally observable callback or await between final admission and publication. Simply validating the aliased graph a second time before another yielding copy leaves the race intact. Command event payloads are separately copied from the original mutable mutation after preparing the transaction, so valid caller changes can also make the event's recorded command differ from the applied diff.

There is an ownership tradeoff: arbitrarily large caller-mutable inputs cannot be captured atomically while also yielding to arbitrary caller mutation. A documented ownership/immutability contract or explicit ownership transfer is necessary for a coherent start-of-call snapshot; bounded incremental capture can otherwise guarantee only a validated owned result, not atomic capture of an externally mutating graph.

## P2 — Public absorption admits invalid runtime coordinates

File: same diff module, `admitIfcDiffSteps:59` and `absorbSteps:129`/`:137`.

Admission checks lane counts and proposed value domains, but not coordinate/identity domains, uniqueness, overlapping lanes or names on modified entities. `absorbIfcDiffAsync({entities:{modified:[{id:1n,diff:{args:{removed:[NaN]}}}]}},{})` resolves and retains `NaN` in the result. Such a diff is neither a valid native coordinate nor serializable to the declared decimal transport (JSON changes it to null). Duplicate coordinates/identities can also be silently coalesced by absorption's Maps/Sets. A base-free operation cannot check entity existence or index upper bounds, but can and should reject malformed primitive coordinates, duplicate targets and internally overlapping lanes. If intentionally unchecked algebra is required, keep it internal to the validated compose path instead of presenting the public operation as admitted data.

## Verified Improvements and Remaining Evidence Limits

- Existing focused editing suite rerun: **23 passing tests, 191 assertions, zero failures**.
- Cursor-based graph traversal refuses a 100,000-child aggregate before child reads under a tiny value budget, as retained earlier repro confirms. Wide root/frontier expansion was removed.
- Owned results no longer alias unchanged base branches after completion. The unresolved issue is mutation *during* yielding capture/validation, distinct from ordinary post-return ownership.
- Async history publication performs its final cancellation and revision check immediately before synchronous internal publication; existing competing-revision tests pass. No separate same-revision publication defect was found.
- Async command, inverse, history and absorption routes exist and yield through their normal work. Heap sorting, graph traversal and ordinary copying cooperate.
- `copyIfcOwnedSteps` and comparison still trust native object shapes beyond declared graph fields; arbitrary additional own properties are copied and `equalIfcValueSteps` calls `Object.keys` for ordinary objects. These are lesser hardening concerns; no additional production failure was established for valid declared graph shapes.
- No full Rust or whole-artifact compilation claim is made. Strict TypeScript was not repeated because no sources changed in this review and the focused Bun suite loads the current modules successfully; the prior worker's strict check is recorded separately.

## Reproduction

Run the explicit ticket-relative `core-reaudit-repro.ts` with Bun from the repository root. It logs the invalid returned scalar, published unreadable revision and accepted malformed absorption under `[DEBUG]`. The input script writes no generated files. The report and input remain in the ticket.
