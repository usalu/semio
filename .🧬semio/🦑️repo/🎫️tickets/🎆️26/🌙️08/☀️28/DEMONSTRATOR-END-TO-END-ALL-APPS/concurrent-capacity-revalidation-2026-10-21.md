# Concurrent Capacity Revalidation

## Actual Race

Root quota57825 reclaimed seven authenticated selected publication pairs and observed1,136,500,736 available bytes above the unchanged1GiB floor. Unit19495 had already observed the deficient state, exhausted its selected nonwaiting attempt and queued image EX. Its stale cleanup request then blocked the root check's new shared admission behind unrelated retained reader64604. Only unit19495 was deliberately interrupted through its owned handle; complete29513-byte log retains Original Script process interrupted. Foreign work remained untouched.

## Safety Boundary

A bounded retry of busy selected states can reduce the race but cannot close the interval between a final shared observation and queued image EX. The existing whole-image recovery rechecks capacity only after EX arrives. An arbitrary raw statfs observer cannot admit capacity: the mount could change while the old shared receiver is in its maintenance gap.

## Proposed General Protocol

The Native owner may expose immutable physical-space DATA from an exact privately bound receiver, bracketed by its existing physical fact checks and Original cancellation checks, only while observing its own queued EX. The graph owner supplies a pure predicate over that DATA. A satisfied predicate withdraws only this request through a derived child signal and exact private withdrawal sentinel; it does not grant capacity, image custody or a writable port. The queued acquisition must settle and clean its own queue claim before returning a distinct withdrawn outcome. Parent cancellation, resource deadline, physical-identity failures and cleanup failures retain precedence.

After withdrawal the graph owner must close the consumed receiver, open a fresh ordinary canonical shared image through normal fair admission, and recheck actual floor. Only that fresh observation can complete admission; if space has been consumed again, normal guarded recovery resumes within the same Original deadline. Neither reader priority nor EX eligibility changes. A spurious observation may cause a harmless withdrawal/revalidation, never unsafe admission.

## Required Proofs Before Production Save

Schema-first neutral cases should distinguish continue, funded withdrawal, original cancellation, deadline, physical identity change and queue cleanup failure. Existing third-party SQLite/native locking oracles should verify that a separate real shared reader remains alive while the queued owned writer withdraws and a later shared request advances, that the writer body never runs on withdrawal, and that a fresh shared floor check is mandatory. Tests must cover capacity changing again before revalidation, exact queued-claim cleanup, primary thrown undefined, and both persistent macOS and fixed directory backend contracts.

No generic Native protocol or recovery implementation has been changed for this proposal. It remains coordinated with root and the independent read-only review.

## Saved Graph Withdrawal Integration

The actual graph recovery now supplies the Native wait predicate only for queued (`wait:true`) maintenance. A funded observation withdraws only that request, closes its consumed image, then opens a fresh ordinary shared image and rechecks the unchanged physical floor. At most eight such revalidations run inside the same Original control and deadline; a ninth withdraw fails closed. Cancellation, deadline, Native failure and even a thrown `undefined` remain primary through close. Ordinary nonwaiting cleanup passes no observer.

Twelve schema-first neutral cases exercise the actual extracted recovery function; Ajv and the owned schema validator validate fixtures, with SQLite deriving expected openings, closes and maintenance executions independently. Two macOS Native fixture scenarios retain a separate real image reader, use an external Node SQLite locking oracle, withdraw queued writers without executing maintenance bodies, and require fresh shared admission. The second scenario consumes the reclaimed fixture space again before fresh opening and therefore requires a second withdrawal. These are authored proofs, not execution results.

The existing Nx composition contract now includes selected-funded, scoped-optional-funded and coarse-fallback paths. Actual-function ports reject consumed-image reads, assert the same facade/control and exact floor request, and compare phase ordering to an independent SQLite projection. The real pressure fixture performs actual canonical operations for both concurrent probe steps before its consuming optional/fresh coarse proof.

Sources saved and independently reviewed for the withdrawal boundary; scoped composition delta remains under final review. No new Native command was launched, and no feature assertion or browser pass is claimed.

## Scoped Optional Native Proof Save

The scoped optional suite now declares a closed 32MiB/34MiB Native fixture and appends an actual macOS recovery proof. A distinct real image reader and exact busy selection fact writer remain active; the oldest unknown namespace is inspected before the idle dataset. External Node SQLite probes confirm both locks. The actual MJS recovery and physical inventory helpers retain genuine canonical binding, operation custody, durable optional ledger/gate, and namespace leases, with only the authored fixture budget projected. One idle 1MiB optional leaf must fund the actual previous free space plus512KiB, preserve busy and unknown leaves, and leave clean accounting verified through a separate Node SQLite connection. Production recovery must reject this fixture envelope and copied image authority. No execution pass is claimed.

The small Native scoped fixture explicitly projects only the extracted core request schema range to0..32MiB, alongside its fixture opening envelope. Production OptionalQuota request minimum remains1GiB, and the actual production API binding-refusal check uses that valid1GiB request. Ledger quota, durable intent/gate, accounting schema and physical filesystem remain actual production implementations with unchanged policy. The fixture proves safe concurrency mechanics and physical reclamation; it does not claim funding the production floor.
