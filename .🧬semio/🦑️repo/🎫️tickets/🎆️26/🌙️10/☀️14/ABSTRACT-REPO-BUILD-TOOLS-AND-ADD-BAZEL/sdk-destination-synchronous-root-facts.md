# Destination Synchronous Root Facts

The destination checker now compares the recorded empty-path directory against the current root facts already returned by its enforcing-prefix guard in the same synchronous span. Only the duplicate asynchronous root lstat is removed. Nonempty parent metadata awaits, immediate Original checks, root kind/mode/device/inode comparisons, final roster proofs and operation draining remain.

## Actual Evidence

- Semantic RED: `🗑️generated/sdk-destination-root-stat-red.log`, expected zero asynchronous root reads, observed one. The second failed registration propagated the same primary; it is not a separate defect.
- Focused GREEN: session 7472, `🗑️generated/sdk-destination-root-stat-green.log`, terminal 0, 1 test / 5 assertions / 997 ms. Actual asynchronous root reads 0, synchronous root reads 1; independent SQLite same-span oracle and Ajv closed corpus agree.
- Full destination: session 39664, `🗑️generated/sdk-destination-root-facts-full.log`, terminal 0, 10 tests / 274 assertions / 8.87 s. Includes root replacement at the retained nonempty-parent await, arbitrary undefined primary with accepted work drained, exact prefix syscall counts, initial roster/capacity/mode/link guards, and cancellation at the actual production continuation cadence.
- Full derivative: session 68975, `🗑️generated/sdk-derivative-root-facts-full.log`, terminal 0, 20 tests / 424 assertions / 4.22 s.

Both full gates used the existing standalone genuine Original 120-second wrapper and actual repository tooling store, without SDK use. All accepted work and store scopes closed before the public SDK lane was released. Production Source is held.

## Scheduling Scope

The root-only cancellation law extracts the exact production clone IfStatement `(files+directories+links)%128===0` with TypeScript and executes its unchanged progress/control/yield/store-recheck block with an actual issued narrower control and actual store. During 256 intended root-only requests, an armed event-loop cancellation is delivered at entry 128 and the parent Original remains active. This proves that caller/production-fragment cadence; it does not claim the raw helper still performs the removed I/O yield or that a full large clone was measured here. Existing production cadence is unchanged.

The retained 11,701-entry namespace implies at least 23,402 avoided root observations across parent and created calls in the operation model. This is not an elapsed-time or CPU saving measurement. No end-to-end public SDK speedup is claimed.

Temporary retirement test CWD/queue observation logs were removed after SDK 50587 terminated. The permanent process route's previously observed framework CWD and 9 tests / 260 assertions / 2.99 s remain recorded in `native-retirement-registration.md`; the behavior was not rerun solely for log removal.

## Captured Source SHA-256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts`: `6a0179f893798d10b0ff094b98e607c5fd51802151551e3a2ee06a1c50e8d79e`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests/🟦️.ts`: `8088aa420be19400524921c250ad6c0780a0c328bc654c879950612ff9bf671c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests/destination.json`: `16aea2ea2b12567ceed3988975ee1e0c338172c606c310f2495f4cfd28cecc64`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests/destination.schema.json`: `46f71048da6911850c8c4f18af0681e0befa9c67e3a88c5302c780eb917d1158`
