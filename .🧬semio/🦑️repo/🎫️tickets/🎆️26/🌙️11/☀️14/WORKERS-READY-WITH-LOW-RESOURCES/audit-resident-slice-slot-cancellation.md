# Resident Slice Slot Cancellation

The 07:13 UTC Ready mirror for g1-m06 showed the generic graph-service heartbeat at 18,609 seconds. [The earlier audit](audit-m06-held-0412.md) explains which physical records reach the mirror. The newer heartbeat establishes that a connection was still receiving a hold signal. It exposes no current planner, capacity owner, slice worker or implementation identity.

## Progress path

The invocation progress callback in `🧭️invocation/🛰️plan` prints the selected language of each progress frame. The resident client in `🚪️client/🛰️plan` resets its 120-second silence deadline on frames, forwards progress and sends a cancellation frame when its own signal aborts. The resident service emits `host-alive` after a quiet interval while identification or attachment still owns the request. Current service code identifies startup and scope separately, sends host governor positions, and replays the latest row per job phase to a joining connection.

The planner port calls the slice pool inside the revision-capture hook. The pool reports the same-root leader, its own worker queue, reservation, and worker progress distinctly as `slice` frames. The ordinary worker-queue line explicitly says it is waiting for a free slice worker and gives occupied slots and queue depth. Generic host keepalives can occur during other quiet parts of planning, including capture and task state checks. The current mirror cannot distinguish these from an older resident revision. In particular, the newly reproduced leaked-slot state would send a named pool queue message under the current code. The physical delay remains unattributed.

## Reproduced failures and fixes

- A private instance of the actual slice pool ran a leader and queued a second root at capacity one. The worker completion resolved the next slot before the awaiting callback, then the second caller aborted in that event-loop turn. The old pool reported one active slot with an empty worker ledger. Its second cancellation gate rejected after slot transfer but before the load entered its release scope. Subsequent roots could queue indefinitely behind that workerless slot. Slot waiting now uses the allocator’s own cancellation, and a caller cancelled during transfer releases the slot from within the owned scope. The follower starts and finishes.
- A private subscriber reattached to an active load after the pool stored an owner progress row, then aborted synchronously in its replay callback. The old order missed that cancellation and delivered the leader’s success. The pool now registers the subscriber’s abort listener before replay. The late subscriber receives its exact cancellation reason and the original caller receives the existing worker’s reply.

## Verification

Both tests failed before their corresponding changes. They use the real production pool with controlled worker events to fix the event-loop order. SQLite independently derives the number of active workers from the recorded starts and completions. The selected reservation and resident keepalive suite also uses real private sockets to check a held slow request stays audible, missing replies become a typed silence outcome, and a joined host client receives real queue progress. All 17 selected tests passed with 71 assertions. The scoped whitespace check passed. No physical worker or resident service was contacted or operated.
