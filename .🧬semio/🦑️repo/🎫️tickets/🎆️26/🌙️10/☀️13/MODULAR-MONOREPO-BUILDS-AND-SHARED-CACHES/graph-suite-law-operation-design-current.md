# Graph Suite Resources and Law Operations

## Confirmed Actual Seam

The retained graph test fixture saves a suite AsyncLocal snapshot and invokes consumers inside it. The store, image and Native allocation are genuinely rooted in the suite Original. Namespace/pool admission, inventory progress, Native opens and consumer uses of `store.control` therefore use suite-root cancellation. A law alias can abort while these waits continue. Promise drainage is retained, but it is not per-law cancellation. The independent receiver14/172 does not qualify this actual graph seam.

The lower reservation is privately bound to its issuing control signal: `requireNativeStorageReservationV1(reservation, control)` checks exact issuance plus signal. Changing `store.control` or spreading the store into a child view would violate that contract. A narrowed control must not be represented as the reservation owner. Root store/image/control remain the real suite cleanup authority.

## Minimal Proposed Owner API

Add a small graph-operation owner beside the graph pool, with an explicit closed operation request (`version:1, kind:law`) and no numeric/deadline knobs. `withNxGraphOperationV1(retained, lawInvocation, body)` receives retained actual allocation facts `{storage, control, invocation, recheck}`; these fields alone grant nothing. It validates the actual private Native reservation against its root control, the genuine retained invocation, and the actual law invocation before and after awaited root rechecks. The parent allocation's own `recheck` is mandatory independently of an injected callback. Root and law capability repository identity must agree.

The owner enters canonical `withScriptInvocationSignalV1(lawInvocation, rootControl.signal, ...)`, then a canonical issued resource control under that real alias, requested from the minimum actual remaining duration of root control and law (never caller numeric data). An exhausted or submillisecond finite minimum refuses before issuance so rounding cannot turn zero into an unbounded request. Recheck both controls at every actual admission and post-await fence. Thus cancellation is the intersection of law and retained root, and remaining time never exceeds either live owner. It retains a privately issued frame keyed by the exact actual Native reservation and root control, not a caller root path or structural store. The immutable operation handle exposes only its actual law `control` and `invocation`; it does not claim the suite reservation was reissued to that control. The frame closes births and drains all accepted operation work before expiry, preserving Error and undefined. The underlying root reservation/image stays retained for suite cleanup.

Graph owner lookups resolve an operation control only through that live private frame for the exact store reservation/root control; unknown, copied, foreign, expired or escaped frames refuse. Existing genuine graph-store WeakSet and root reservation checks remain unchanged. Operations without a narrowed frame preserve their existing root-owned behavior; this is an explicit scope, not a global control fallback or runtime compatibility path.

## Exact Integration Obligations

- Each accepted asynchronous graph effect must enroll with the existing canonical `withIssuedResourceOperationV1` before its first effect; ignored returned Promises still drain and fail the actual operation. Late graph admission after the operation callback closes must refuse. No local Promise observation may replace canonical enrollment.
- Graph `check(store)` retains real store/root reservation checks and additionally validates the current privately issued law operation.
- Inventory publish/yield/check, pool queue acquisition, namespace lease acquisition, room/retirement checks, watcher/auxiliary/publication effects and Native child opens use the operation's issued control and signal. The reservation is always validated with the real root control.
- `withNxGraphPoolAdmissionV1` keeps root allocation authority separate from the effective wait/effect control. Genuine nested pool custody must recheck both active and narrowed frames after await; it must not wait behind its own retained writer or silently switch an active frame to a foreign operation.
- The fixture may enter its suite snapshot to recover genuine graph Native AsyncLocal resources, but must immediately enter the canonical law operation before namespace acquisition or consumer body effects. This restores the law's actual process context rather than only carrying a signal as data.
- Consumers receive the actual operation handle as a separate value. Existing direct effect uses of `store.control`/`store.invocation` in graph tests become `operation.control`/`operation.invocation`. They continue passing the actual root reservation/control to allocation authority checks; no cloned store is constructed.
- Namespace/image cleanup belongs to a separately retained accepted scope and still closes actual children before the operation expires. Later physical pool retirement keeps the root control so cancellation of one law cannot strand suite cleanup. No unissued longer cleanup control is created.

## Genuine RED and Bounded Proof Plan

First write a neutral closed request/corpus and independent Ajv/SQLite state oracle. Reproduce the missing law effect using the actual existing pool admission seam, a privately issued small Native reservation and a real held SQLite pool writer: abort the actual canonical law alias while its acquisition waits under suite resources. Current root-control behavior must remain waiting (bounded by an authored fixture fallback), demonstrating the RED; the expected implementation must cancel before callback/effect admission while the same suite/root reader remains valid.

The faithful fixture should use an actual private32MiB diagnostics image and actual Native allocation/Original controls, not a fake NxGraphCacheStore, public registrar, or production896MiB graph pool. This requires Root coordination before physical execution. Its exact lower pool admission function is shared with the actual graph owner, so the waiter exercises the real queue mechanism and resource identity. Independent Python SQLite proves root reader/writer exclusion, cancellation release and terminal availability. Full pool/namespace Native execution remains separately pending.

Hostile cases: live law cutoff during queue wait; abort after admission before inventory/effect; abort at inventory continuation; root cancellation; copied invocation/operation; foreign reservation/control; operation escape; ignored accepted failure; Error and undefined during body plus cleanup; nested same actual pool custody; later law uses the same retained suite resources without reprovisioning. Accepted close/drain must finish before the operation expires, while the suite reservation remains valid and cleanup can still run under root custody.

The existing receiver14/172/strict baseline and all failed SDK gates remain unchanged. Cheap registrar funding/done-throw/initialization cases may be added, but they must use actual lower issuance and must not manufacture a runner/Original success. No Source implementation, new Original, image, SDK/fence/full-size run, Cargo invocation, maintenance or publication was performed for this design.

## Review Boundary

This design changes graph effect plumbing and therefore potentially captured SDK Source. It is intentionally held for Root review before implementation. No broad Core reservation factory or global registrar change is proposed. A physical32MiB wait-seam proof and a later actual graph/SDK recapture require explicit scheduling; the existing tiny suite SQLite proof alone is insufficient.

## Approved Private Identity and Live Deadline Refinement

Storage's existing `requireIssuedResourceControlInvocationV1` binds the retained control to the exact root invocation. The only new Core API is read-only `requireIssuedScriptInvocationDescendantV1(ancestor, borrower)`, traversing privately issued signal-scope parent identities; matching repository, policies or capability bytes grants no lineage. Genuine unrelated roots and foreign module realms refuse.

Do not sample a parent remaining duration and recreate it after an await. The operation instead uses canonical `withRequestedIssuedResourceControlV1(rootPhaseControl, 0, body, actualLawSignal)`, preserving the live root phase ancestry and its original monotonic/absolute deadlines. An outer canonical signal scope rooted in the real law restores law async context and also awaits the complete accepted root-child scope. Because the law is a private descendant, its Original deadline is the same actual carried birth; its registrar signal narrows body time. The exact control-binding invocation remains the retained root; the operation separately identifies the law alias. This does not pretend a root-bound reservation was reissued to a law alias.

Root reservation control and an optional actual tighter root phase control remain separate; both require exact private root-invocation binding. Each post-await fence checks the real root allocation/control, live phase control and actual law. The effect control's parent chain enforces tighter phase expiry during waits without rebasing. Parent approved a private32MiB diagnostics proof; no large graph image is authorized.

## Implemented Checkpoint

The implementation and bounded actual proofs are recorded in `graph-suite-law-operation-current.md`. The earlier sampled-duration and alias-bound-control design paragraphs are superseded by the approved live phase refinement and the implemented root-bound effect loan. Actual proof also exposed pool-release-before-drain and canonical-parent-birth-close timing defects; both have genuine failing logs and final passing proofs. No large graph/SDK qualification is inferred.
