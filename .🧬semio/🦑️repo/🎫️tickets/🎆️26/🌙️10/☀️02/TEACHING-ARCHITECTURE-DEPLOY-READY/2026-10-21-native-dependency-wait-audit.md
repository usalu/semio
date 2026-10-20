# Native Dependency Wait Audit

Observed 2026-10-20T22:33:13.629Z. Static Source and existing log reads only. No process polling, commands, lease/cache/Source changes or peer interaction.

## Concrete Log State

The captured log enters `build:dependencies` after capacity admission and Source enumeration, then emits schema-shaped `waiting` events for resource `build` every roughly 10 seconds. The captured final event is `elapsedMs:370315`. Its exact message is “Another process is building the dependencies (370 s). Other commands keep running; waiting for that build instead of starting a second one.” This identifies the JavaScript dependency build lease, not a Cargo compilation/test terminal or the short publication gate. It is normal expensive waiting evidence; it does not establish failure or deadlock.

## Bounded Loop and Completion

`🚦️concurrent/💾️storage/🟦️.ts:20–47` constructs zero-timeout lease attempts, treats only the explicit resource-lease timeout as busy, and retries. Every round checks cancellation, probes the lease, and returns immediately if acquired. On a busy probe it checks elapsed time, optionally asks a readiness callback whether another owner completed the work, reports waiting progress, and sleeps with an abort signal. Schema cadence is 500 ms probes, 5,000 ms optional readiness rechecks and 10,000 ms progress. Actual intervals include effect/runtime costs. A readiness success emits `coalesced` and returns null; acquisition instead lets this caller perform its body.

The build wrapper `💾️storage/🟦️.ts:93` passes `SourceOwnerStoreSyncProtocol.maximumQueueWaitMilliseconds = 21600000` (six hours), not the separate 900000 ms publication or 600000 ms receipt-writer limits. Expiry after a busy probe throws `Dependency build lease wait exceeded its declared maximum`. Cancellation aborts sleep/acquisition. This is a bounded acquisition policy, not a guaranteed exact wall-clock deadline: awaited acquisition/readiness/report operations must return before the next elapsed check, and acquisition is attempted before expiry is checked. No timeout is inferred from 350–370 seconds.

A legitimate next outcome is lease release followed by acquisition, or optional verified readiness followed by coalescing. The producer wrapper drains pending owned work, rechecks store custody and releases gate/pins/build lease at settlement; error paths abort and also attempt release. A timeout/cancellation/error terminal would require a fresh authorized invocation after its concrete cause is resolved. This audit does not justify deleting a lease/cache or interrupting another owner.

## Nesting and Liveness Limits

The observed waiting wrapper has not created its producer frame yet: it acquires the build lease before constructing that frame or entering the producer body. It holds shared dependency-store access while waiting, and does not acquire the bindings gate in that sequence. The schema explicitly permits a builder with shared image access and short later publication gating. Therefore no own nesting cycle is concretely shown by this sequence or log. The inspected producer settles its own frame rather than reacquiring the build lease. A truly nested separate wrapper call under an already held exclusive build lease would need further call-chain evidence; the log contains neither owner identity nor such a nested invocation. No such risk is established here.

Repeated waiting events prove the waiter loop/report path is running; they do not prove progress by the lease owner. Resource-directory admission activity in the same log also does not prove owner completion. Owner liveness, current lease identity and eventual admission remain runtime uncertainties deliberately not investigated by process/lease polling.

## SHA256 Observations

Sequential reads, not an atomic Source seal. Paths relative to `/Users/ueli/Documents/semio`. Log hash binds the captured extent only.

| Path | SHA256 |
| --- | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/2026-10-21-native-combined-neutral5.log` | `695a0084684b37c1c02edc6ff44a61b6b6f15ac675c7cad41ec246cab83eaccc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` | `4a5c26be6ccc77adc8c7b8085797b1b2feb21cfd41877117a95ecfb73eb3879c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/💾️storage/🟦️.ts` | `e5fd0fad12758550d5fcd817525ce210373a0485bee60bf6be2adf5a7b4f6010` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts` | `1c02977601c014182299af4f680748c0da85da955d1cee6dca6bc717630871be` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🧬️schema/🔣️.json` | `8e2cbc731b1458c67f11fdf2a0289cf166be6a7f02033422a93168b99aac036a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json` | `f664649dc5802daec41a899f1eb7bca929e406eeb50e5234188abdf59a6cfcf7` |

## Follow-up: Actual Readiness Callback and Exact Receipt Observation

Observed 2026-10-20T22:38:08.960Z. Bounded follow-up, no Source evaluation or process/lease polling.

The actual `🏗️build/🚀️bootstrap/🟦️.ts:46–50` default installer DOES pass `{ready:recheck}` to `withJavascriptDependencyStorageV1`. The callback at line 95 is `inspectIn(false,true)(store,control) !== "material-required"`. Cheap inspection first requires a matching generation receipt; a non-match returns material-required immediately. A match continues through current Source-owner evidence, the exact root node_modules symbolic binding and its v2 ready receipt, locked TypeScript/TOML versions and bounded installed-file digests, unchanged root package.json/bun.lock, store recheck and generation custody confirmation. Thus this is verified coalescing rather than mere lease disappearance or elapsed time. `receipt-required` intentionally counts as sufficient material to coalesce: the final `inspect(true)` then publishes/verifies the parser bootstrap receipt and must return ready. `installUnlessReadyV1` also invokes the same predicate after actual acquisition, avoiding a duplicate install when readiness appeared between probes.

From the witness path construction in that Source, the current exact root symbolic link was read once: `/Users/ueli/Documents/semio/node_modules` resolves to `/Users/ueli/Documents/semio/.cache/b/d/m/j/e7c8cff97e60b5961d2d/node_modules`. Its derived adjacent ready receipt `/Users/ueli/Documents/semio/.cache/b/d/m/j/e7c8cff97e60b5961d2d/bindings.json` is a regular non-symlink file, 75414 bytes, SHA256 `ffda10ba447e7d2f3084e3caa11e32438f5bd6bc6a5aa035fcc8e0c91b5db7f4`, declaring version 2, purpose current-dependency-source-owner-ready, scope `.`, full generation `e7c8cff97e60b5961d2dfe4041e77adfa304420011d6380d0da53ea238f4ac07`. Receipt mtime is 2026-10-20T20:59:53.463Z and link mtime is 2026-10-20T21:02:43.763Z. These existing artifacts predate the observed wait and cannot prove fresh owner progress or current Source readiness. This audit did not execute their verifier.

The new captured log extent ends at `waiting/build elapsedMs:711358`; SHA256 `b2e806779edb23bc52c340440873fd4191d467f2b3ccc9d9e62e59733b9929bd`. No coalesced/acquired/build-completion evidence appears in that final extent. The exact bootstrap Source read SHA256 is `af683b99468cf790367b64a938c6cf8a8b0ad03e9f864d5a6caeb4ceb5020579`. The log still proves waiter progress only. No exact shared owner log path/identity was present in these bounded inspected bindings, so investigation stops without attributing any PID or scanning foreign processes. There is no concrete new failure/deadlock finding; readiness or actual ownership completion remains outstanding.
