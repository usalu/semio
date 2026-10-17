# Producer Ownership Progress Delivery Batching

This change coalesces only observable progress publication and continuation yield for the fixed current Dependencies producer plan/probe. It preserves complete ALL membership, physical IO, current Source observation/replay and per-operation live/issued/refinement/signal/held-writer checks. No Native operation, protected Source/cached asset read, ALL production capture, installer, binding transition or actual producer retry was performed by this agent.

## Verified Premise And Contained Scope

The requested worker-pool conversion was already satisfied: the current private Bun workspace adapter uses synchronous `lstatSync`, `opendirSync`/directory read/close and descriptor `openSync/fstatSync/readSync/closeSync`; `controlledOwnership` awaits the caller's progress callback, not a filesystem promise. The AST/system-call summary and input hashes are saved in Generated `bazel-planner/ownership-sync-ast-baseline.json`. No production synchronous IO rewrite was made.

The actual fixed producer callbacks, however, awaited publish and continuation yield for **every** ownership operation. Root explicitly authorized a shared private ADMIN progress driver instead. Its batch size is the existing schema-owned `InstalledPathProtocol.progressWorkUnits` (64); no new/increased budget or cap exists. New closed `ProducerOwnershipProgressProtocol` and eleven portable cases describe this progress-only contract.

The only production change is the private helper `createRepositoryDependencyProducerOwnershipProgressV1` and its two producer plan/probe call sites in the existing explicit ADMIN owner. Every actual monotonic progress row is schema validated, and every operation invokes the actual original checkpoint. Publication threshold uses completed-count delta rather than modulo, so increasing jumped counts cannot suppress progress. The helper serializes delivery, snapshots the actual current row, and advances its published marker only after successful publication, yield and checkpoint. Failed delivery remains failed; no reset/retry/renewal substitutes another Original.

Normal explicit terminal flushes occur before invoking the real custody body, after each **successful awaited** custody recheck, and after **successful** outer ownership return (covering the final internal verification). There is no finally flush after a failed body/Original: existing exact failure/closure paths remain in control. Frozen and undefined publication failures are retained exactly; a subsequent flush cannot publish again or mask them. The current two private producer call sites await each callback/recheck in sequence; `controlledOwnership` and its validation loops await each advance before another physical operation. The current progress row therefore cannot race delivery in these fixed callers. The helper adds no public callback/provider/override or foreign store/image capability.

The physical CJS owner is byte-identical to the pre-change input: SHA256 `d8dcedd5b8ba1f7a70233c772ba9db6977b27dc4f16507e06a1f7d890de8fa85`. Observation codec, global replay reference order, currentFence, directory/manifest identity/byte/path bounds, ALL declaration/membership and every real system call remain untouched. Graph/Tools/other dependency retirement bodies and runtime recipe were not edited.

## Actual Source Qualification

Schema/corpus/test preceded implementation. Genuine Source RED is Generated `bazel-planner/producer-progress-red.log`: missing actual private helper, with 21 prior tests passing. Initial integration failures were fixed by including the actual new private helper in the existing Source owner extraction; the public first-party owner was never replaced with a fake authority. A later test-only recursive SQLite oracle incorrectly returned an aggregate null for an exact64 terminal; changing WHERE to HAVING corrected that oracle without a production change. Both failures are retained in Generated logs.

Portable law coverage includes 1/63/64/65/128 operations, jumped counts, repeated/nonmonotonic/foreign rows, cancellation at the real yield boundary before another physical operation, undefined/frozen delivery failures and no retry after poisoned delivery. Ajv validates closed vectors and recursive SQLite independently computes threshold+terminal publication counts. All per-operation checkpoints remain exercised under actual issued controls.

A second law runs the **actual unchanged CJS owner** on one real Generated24-package Source tree twice, using the exact same system filesystem API with instrumented call inputs. It compares the complete ordered syscall trace and actual ownership callback trace between original every-operation delivery and the new driver; it does not replace physical observation with a fabricated tree provider. The physical Node system APIs perform real lstat, directory and descriptor reads. SQL independently checks complete operation counts. Source manifests remain unchanged; each run performs actual custody recheck and final fences.

Final gates:

- Current unmodified workspace ownership suite: 52 tests /705 assertions /0 failure, 2.64s (`ownership-sync-current-baseline.log`). Existing fast-glob/Ajv/SQLite laws include current topology, descriptor growth/closure, exact global fact order and arbitrary primary/cancellation handling.
- Full dependency-storage Source suite: 31 passed /9 isolated-worker definitions skipped /2,097 assertions, 13.93s (`producer-progress-full-final.log`).
- Genuine Root0 producer plan helper Source: five laws /206 assertions /0 failure, 4.82s (`producer-progress-plan-workers.log`); its existing owned isolated worker terminal/SQL/FD/Original laws remain active.
- Genuine Root0 probe helper Source: seven laws /821 assertions /0 failure, 7.65s (`producer-progress-probe-workers.log`); its three actual isolated scope worker terminal laws remain active.
- Final six-root strict TypeScript: natural0 /zero diagnostics /12.159s (`producer-progress-strict-held.log`).

Source worker coverage keeps substantive same Original transport, actual owned terminal/closure and exact failure evidence. None of those Source fixtures is a Native image or actual funded installer acceptance.

## Honest Cost Evidence

The final small Generated24-package comparison recorded **7,292 identical ordered filesystem calls and 466 identical ownership callbacks**. Original publication/yield count was 466/466; batched count was 8/8 including normal boundary flushes. In the final full gate, measured elapsed was **108.83725ms original versus174.254958ms batched**. An earlier run likewise measured99.109709ms versus174.54225ms. Thus these measurements establish delivery-count reduction and trace preservation, **not an elapsed speedup**. They use a bounded fixture with Source publication ports and real control continuation; actual expensive protected publication/log overhead is not reproduced. No reduction in physical scan work or Source freshness is claimed. Root must observe any actual ALL elapsed effect separately.

The Generated timing JSON is `producer-progress-Generated-timing.json` at ticket Generated root; full Source logs retain earlier measurements when later fixtures overwrite that latest ordinary DATA file. Complete trace hashes include the ephemeral Generated path and naturally differ across fresh fixtures; equality is asserted within each same-tree comparison.

The final eight-file manifest binds ADMIN, existing install schema/corpus/test, unchanged CJS owner, existing Source helper runners and this report. No new Source/script file or launcher was introduced; existing producer Source and registered dependency test entry cover the laws. Nx remains primary.
