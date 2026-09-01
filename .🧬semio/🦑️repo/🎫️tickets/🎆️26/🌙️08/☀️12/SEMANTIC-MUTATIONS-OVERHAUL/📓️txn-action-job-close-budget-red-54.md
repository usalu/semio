# TxnActionJob Close Budget Native RED Plan

Source examined: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs`, `TxnFixtureJob::close_step`. Its retained owners are `Option<Box<TxnCommand>>` and `Option<ArtifactToolCompletion<TxnApp>>`. The current method checks only `closing` and `maximum_items`; it calls `take()` on either owner, returns `Pending { released_items: 1, released_bytes: 0 }`, and ignores `maximum_bytes`.

The replacement neutral cases and schema are [vectors](🧪️txn-action-job-close-54/🔣️.json) and [schema](🧪️txn-action-job-close-54/🧬️schema/🔣️.json). They use only measured-command grant relations, never guessed completion allocation bytes.

Proposed mounted Rust footprint: a `#[cfg(test)]` module in the transaction fixture beside `TxnFixtureJob`, with one native law per vector. Each law constructs a closing job with independent command and completion ownership, invokes `close_step(maximum_items, maximum_bytes)`, asserts the exact close-step, retained options, released item count and released bytes, and drops only after `terminal_is_empty`. It must keep the command boxed and must not unbox or enlarge stack storage.

## Revised Native RED Source

The prior numeric estimates are superseded, not evidence. [Native RED source](🧪️txn-action-job-close-54/🦀️native-red.rs) uses the already visible `ArtifactToolCompletion::new`, `has_mounted_consumer`, `complete`, and `take_emit` APIs. It measures `size_of_val(command.as_ref())` at native execution; it never unboxes the command or claims an Arc allocator size.

Its first close has both owners and therefore must release the command before it can consider completion. The external-clone case stops at command release; it does not certify a completion close. The unresolved boundary is the production authority that assigns bounded-close bytes to the Arc/Mutex completion payload. The independent Ajv controller `🧪️txn-action-job-close-54/📜️script.ts` ran through Nx and reported `neutral 5`. This source remains ticket-only and unmounted; no test was run and no source/API was edited.
