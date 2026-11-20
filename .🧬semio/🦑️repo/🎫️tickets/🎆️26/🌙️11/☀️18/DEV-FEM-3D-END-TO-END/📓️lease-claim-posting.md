# Lease claim posting versus the publication window

## Size check
- `SourceScope` measured natively: 72 bytes (binding 56, key 72, pointer 8), so the claim posts 144 bytes natively.
- wasm32 by layout: binding 48 (payload and clock_source are 4-byte usize), scope 64, claim 128 bytes. Not 112.
- Conclusion: the 1 item / 112 B / 0 capacity posting is not proven to be the lease claim. Another direct posting is possible (not identified; the claim never runs in the same window as a step because a paying claim always answers false and returns).

## Real cause
`advance_typed_operation_output_with_leftover` built `original_receiving` for ANY non-zero posting in the window, then only returned early when a receipt was pending. A window with direct posting and no receipt therefore carried a receiving with `family=None` and no receipt, which `collect_original_publication_receipts` correctly refuses (`families=[]`, `before+posted==retained`). That matches the browser probe exactly and is independent of which control metadata posted.

The working tree already gated this on `ownership_receipt_pending()` (uncommitted edit in `typed-operations/🦀️.rs`). The wasm that produced the probe predates it, or was built before it.

## Accounting decision
Option (c): control-metadata claims (command-scope lease, host-event scope, route capture) post straight into the recipient and are charged outside the window's receipt binding. Only a window that serves a family receipt binds its posting to exactly that family. `collect_original_publication_receipts` is unchanged.

## Change
- `PluginExchangeOutput::bind_window_posting` (typed-operations) holds the logic; the window calls it inside the pending branch.
- Test `a_lease_claim_posting_without_a_family_receipt_owes_no_receiving_and_with_one_binds_that_family` in runtime-ops tests.

## Tests (native)
- runtime-ops `lease_claim` test: 1 passed.
- instance-dispatch (host-event-routing lease tests): 5 passed.
- semio-plugin-command: 15 passed.
- `cargo check -p semio-plugin-instance-app`: clean.
