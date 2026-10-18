# Joint SIGKILL: Repo Source Boundary Audit

The abnormal Tools13831/Dependencies20897 saved terminals were bound separately. Their terminal save-file mtimes differ3.270956ms, not independently measured signal times. No actual killing caller or external/system cause is established.

## Owned child execution

Core owned-execution spawns POSIX children detached and calls terminateOwnedChildTree only while its child handle is still live. The terminator snapshots ancestry from the supplied positive owned root PID, derives descendants and groups whose leader PID lies in that owned set, then kills deepest groups/PIDs. Windows uses taskkill /pid <owned root> /T /F. Failure fallback addresses only -rootPid/rootPid. Capture uses detached owned child roots too. This is not a global Bun-name sweep. A missing/wrong historical owned root or runtime group reuse would require actual evidence; no such event is proved here. Nx bootstrap cancellation similarly derives descendants from its actual children and targets their groups/PIDs after signal/output/timeout conditions.

## Concrete global-name Source candidate

Repo workspace-cleanup/stray-processes contains a materially broader path. `isDevLeftoverRow` immediately accepts live executable names bun/node/cargo/etc or command markers, without actual ownership or Original-liveness proof. `planStrayProcessRemovals` protects the cleanup caller's own ancestry, recognized IDE trees, active semio-tech Nx trees and dashboard trees; an independent plain Bun ticket helper can fall outside those sets. `applyStrayProcessRemovals` then sends SIGKILL to every planned live PID. Full clean command calls this after workspace cleanup/cache prune; the clean stray-processes mode invokes it directly. The local clean skill explicitly documents the same global executable sweep and repetition. No cleanup operation or skill was invoked by this audit.

Thus an independent cleanup caller can, in Source, kill both genuine long-running helper Originals by classification alone. That is a concrete unsafe concurrency boundary, but NOT proof that cleanup ran at these actual terminal times. Search was bounded to Core process owners, repo infrastructure and cleanup guidance; no live process table, signal, Native/protected data or broad runtime inventory was inspected.

Before repeating long attempts, actual diagnosis should correlate any already-saved cleanup applied/killed-PID records or owning caller evidence. A future design should not equate executable name/command marker with abandoned work: only a genuinely owned descendant or positively authenticated termination target may be killed; unknown independent live work must be retained. PID age, lack of Nx spelling or absence of recognized IDE ancestry is not proof of abandonment. Preserve existing exact own-child cancellation semantics and do not infer an external cause merely because this Source candidate exists.

## Proposed owner correction

Bazel proposes removing automatic generic process-name termination and refusing explicit stray termination before process-table/effects because this owner has no authentic retained child handle. This reviewer agrees: neither live Original-looking DATA nor a new structural token registry can safely substitute. Existing locally retained owned-child termination remains unchanged. Schema-first no-kill cases must include independent genuine helper lookalikes, cleanup caller ancestry variations, zombie-parent targets and normal/dry/full-clean entry paths. This repairs the Source hazard without attributing past signals.
