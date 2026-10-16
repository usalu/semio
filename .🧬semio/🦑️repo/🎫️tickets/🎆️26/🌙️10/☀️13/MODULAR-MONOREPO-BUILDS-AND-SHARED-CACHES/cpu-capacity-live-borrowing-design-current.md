# CPU Capacity Live Borrowing Design

Read-only Source design exploration, 2026-10-15. No implementation, probes, compiler activity, images or new global namespace. This extends the prior bypass report rather than repeating it.

## Existing Authority and Lifetime Pieces

`framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:44-54` maintains private invocation/control scopes, accepted operations and drain-before-retirement. Lines 76-92 validate actual in-process issuance and retain narrowed child operations. Lines 103-110 serialize the same admitted process envelope and deadline. That wire preserves original policy/time but is not itself a cross-process proof that a particular CPU reservation is still held; private WeakMap issuance does not cross processes.

`process/🔒️leases/🟦️.ts:73-75,135` authenticates actual in-process live exclusive/shared lease identities. The paid compiler admission owns its gate/core leases, so the borrowing issuer must start from those private actual objects, not resource-name strings. `process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts:9-30` already offers finite GNU-compatible FIFO/semaphore tokens. It distributes paid work concurrency, but an environment endpoint/count alone does not prove original capacity custody.

`process/🎛️owned-execution/📥️capture/🟦️.ts:34-43` owns child spawn, cancellation/tree termination, output bounds and awaited close. Cargo launcher `:123-134` is the immediate supported parent/child admission seam. Its current borrowing branch must be replaced by authenticated handoff; compiler unit and scheduler descendants must use the same issuer rather than independently interpreting a marker.

## Narrow First-Party Design

Keep independent builds on the existing real gate/core acquisition. Wrap an actual paid grant in a private callback scope with accepting/live state, exact original control/deadline, total reserved cores, outstanding child allocations and pending launches. Expose no public registrar that can issue paid grants from serialized data.

Add a child-launch borrowing operation to the existing owned-execution boundary. It must validate the actual paid grant and register the whole child operation before spawn. Transfer a dedicated kernel-backed parent/child channel through that actual spawn's inherited handles, bound to the child's actual process handle and the exact original envelope/deadline. Use an extra stdio pipe or existing private IPC transport, with a platform interface; do not create a global discovery directory or trust a pathname/PID/environment token. Environment may carry version/routing hints only. A child without the actual inherited channel cannot complete a live grant handshake by minting environment fields.

Parent service issues a single-use child allocation only through that channel. Requests carry a finite desired count; the parent validates total outstanding child allocations plus any parent-owned runnable allocation never exceeds its actually paid cores. Do not give every nested process an implicit extra worker. All compiler workers draw from the same paid jobserver or explicitly accounted subgrant. A nested child may narrow its allocation through the same issuer, never enlarge it or manufacture a second paid pool.

The child receives a private local borrowed-grant object after handshake. Its checks include active channel, exact allocation identity, allowed cores, original deadline and cancellation. Core admission callsites accept that object through first-party runtime context; they do not treat successful schema validation of a token as custody. Closing a child callback stops births, drains accepted work and waits for its compiler tree to close before returning allocated cores. Parent callback return also stops births and drains all accepted borrowed child launches before releasing the actual gate/core leases.

## Failure Boundary Requiring Explicit Implementation

An inherited channel plus private counter solves ordinary forgery, stale callback reuse and over-allocation. Parent crash needs more than a heartbeat claim: parent SQLite leases release on process death while a child compiler could still run briefly. A strict no-overlap crash guarantee requires a paid-grant custodian whose lifetime retains the actual gate/core ownership until every accepted child tree is dead, or equivalent platform-backed ownership retained across the child lifetime. EOF-triggered child cancellation alone has a scheduling gap and must not be described as atomic paid custody. Design and test that boundary explicitly; do not replace it with PID liveness checks or a renewable deadline.

The initial narrow implementation can refuse unsupported inheritance and independently acquire real capacity rather than claim an unproven grant, but must avoid self-deadlock when a known parent retains all cores. Authenticated owned-spawn delegation is the intended path for those children.

## Actual Neutral Proof Plan

Extend existing `build/🎟️capacity/🧪️tests/🟦️.ts` through its registered compiler-capacity route, using a real admitted Original and bounded test lease directory. No CPU burn is needed.

1. Hold an actual all-core exclusive parent grant. Spawn a separately owned child with the old boolean/count and replayable envelope but no inherited grant channel. Its borrowed admission must refuse before a worker callback. Independent Python SQLite read/lock observations prove the real parent gate remains exclusive and no child core lease was acquired.
2. Spawn a genuine child through the new production owned-launch operation with one allocated core. Assert it can perform an inert acknowledged callback while the parent's actual reservation remains held, and independent SQLite still observes only the paid parent ownership. Try over-parent allocation and copied channel/claim metadata; callbacks must remain unentered.
3. Let the parent callback return while a genuine child is paused on a message gate. Assert the parent grant scope has not returned/released, and an independent competing capacity request remains blocked. Release the child, await actual process close, then prove the independent requester can acquire. Try escaped child admission after drain and canceled/deadline-expired child admission; refuse without refreshing time.
4. Crash the launch parent while a child is held, and independently test that a competing actual reservation cannot overlap a live accepted compiler child. This is the falsifiable custodian-lifetime requirement; a test that only waits for eventual child EOF is insufficient.

Ajv validates closed protocol/count cases; SQLite supplies an independent actual reservation oracle. Promise/message gates establish ordering; avoid timing-only sleeps, process-name inventories and fabricated successful CPU workload. No such proof was executed in this audit.
