# WASI 1005 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":2,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T10:32:16.701Z","finishedAt":"2026-09-09T10:34:27.765Z","packages":160}

## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:3839:101
     |
3839 | ...      let step = advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, grant.maximum_items.min(...
     |                     ------------------------------ required by a bound introduced by this call   ^^^^^^^ `P` cannot be sent between threads safely
     |
note: required by a bound in `os_store::component::advance_returned_snapshot_read`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1564:38
     |
1564 | fn advance_returned_snapshot_read<P: Send + Sync + 'static>(
     |                                      ^^^^ required by this bound in `advance_returned_snapshot_read`
help: consider restricting type parameter `P` with trait `Send`
     |
3776 | impl<P: std::marker::Send, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
     |       +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:3839:101
     |
3839 | ...      let step = advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, grant.maximum_items.min(...
     |                     ------------------------------ required by a bound introduced by this call   ^^^^^^^ `P` cannot be shared between threads safely
     |
note: required by a bound in `os_store::component::advance_returned_snapshot_read`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1564:45
     |
1564 | fn advance_returned_snapshot_read<P: Send + Sync + 'static>(
     |                                             ^^^^ required by this bound in `advance_returned_snapshot_read`
help: consider restricting type parameter `P` with trait `Sync`
     |
3776 | impl<P: std::marker::Sync, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation> {
     |       +++++++++++++++++++


