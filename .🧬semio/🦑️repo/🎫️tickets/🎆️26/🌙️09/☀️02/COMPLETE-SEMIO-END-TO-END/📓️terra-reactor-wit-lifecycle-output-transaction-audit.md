# Reactor WIT Lifecycle Output Transaction Audit

## Current concrete loss boundary

The WIT wrapper calls the semantic reducer and only then lowers its owned result:

1. [`⚛️reactor/🦀️.rs:1123-1131`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1123)
   calls `poll_kernel`, then `kernel_turn_result_to_wit`.
2. The reducer has already called `guest_lifetimes.finish_turn` at
   [`⚛️reactor/🔄️turn/🦀️.rs:713`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:713).
   That consumes a staged lifecycle ACK and can release the exact terminal owner.
3. `kernel_turn_result_to_wit` remains fallible because every effect conversion is collected with
   `?` ([`⚛️reactor/🦀️.rs:1398-1418`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1398)).
   Its only explicit current error branch is `Effect::SetTimer` without an `ARMED_TIMERS` row
   ([`:1542-1549`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1542)).

Thus an ordinary Rust `Err` can occur after lifecycle state has changed but before the component
caller receives its `TurnResult`. A retry cannot reconstruct the prior receipt/owner state.
This is a transactional boundary defect, not a WIT codec cosmetic issue.

There is one adjacent concrete representation loss: kernel `Effect::SetTimer.after_ms` is `u64`,
but the WIT conversion truncates it with `as u32` at [`:1550`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1550).

## Smallest coherent correction

Keep lifecycle commit in the domain reducer so native and component calls retain the same
semantics. Make the ordinary WIT result lowerer total instead of moving the lifecycle protocol
behind a WIT-only adapter.

1. Add a kernel-local `validate_turn_output_for_wire(&[Effect]) -> Result<(), Fault>` in
   `⚛️reactor/🔄️turn/🦀️.rs`. It runs immediately before line 713, while the staged lifecycle ACK
   and terminal owner are still retained.
2. Its initial exact checks are:

   - every `SetTimer.id` still has an exact `ARMED_TIMERS` row;
   - `after_ms <= u32::MAX as u64`.

   Timer effects normally enter only through `push_admitted_effect`
   ([`🔄️turn/🦀️.rs:850-865`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:850)),
   which installs the row before retaining the effect. The validation is nevertheless necessary
   at the one output boundary because the result bridge currently relies on that invariant after
   irreversible lifecycle work.
3. On validation error, return from `poll_kernel` **without** calling `finish_turn`. The staged
   ACK, receipt, and owner remain exact for retry/diagnosis.
4. After that preflight, change `kernel_effect_to_wit` and `kernel_turn_result_to_wit` to total
   functions: no `Result`, no `?`, no second ownership read. The `SetTimer` conversion may use
   `u32::try_from(after_ms).expect("validated wire duration")`, not `as`.

This is smaller and safer than an owned WIT-result transaction because the current conversion has
one semantic error source. A fully retained output owner would be required only if a later
component conversion legitimately awaits or has another recoverable fallible step. Allocation
trap/OOM is not an ordinary recoverable callback failure; it aborts the component invocation and
cannot be made transactional by a Rust `Result` wrapper.

## Other existing silent projections to fence separately

They do not currently return `Err`, but should not be mistaken for safe total conversion:

- `kernel_ui_patch_to_wit` maps malformed `surface` instance prefixes to `0`
  ([`⚛️reactor/🦀️.rs:1429-1434`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1429)).
- `kernel_endpoint_to_wit` maps nonnumeric Shell/PluginInstance ids to `0`
  ([`:1569-1575`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1569)).
- `kernel_effect_to_wit` and `pack_patch_field` turn a `serde_json::to_value` failure into
  `DslValue::Null` before packing ([`:1447-1450`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1447),
  [`:1502-1505`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1502)).
  That is payload corruption after lifecycle commit, not a harmless default.

The same preflight should reject these rather than alias a caller to instance zero: require an
exact `<u32>:<surface>` UI-patch prefix and lossless numeric parsing for Shell and PluginInstance
targets. It must also serialize every packed field into a fallible temporary before commit, then
make the lowerer consume those validated bytes (or make its packing `expect` the retained proof).
This is still a bounded validation extension, not a new lifecycle mechanism.

## Executable laws

1. A direct native turn-output law passes a retained `SetTimer` owned by `ARMED_TIMERS`, and its
   wire preflight accepts the exact `u32::MAX` duration.
2. A foreign/unowned timer and `u32::MAX + 1` duration are rejected before lifecycle finish. The
   test stages a real Captured ACK in the runtime cell, asserts the exact receipt/owner remains,
   then fixes/removes the invalid output and completes its normal ACK/close tail.
3. A hostile Shell/PluginInstance id and malformed patch surface fail preflight instead of
   lowering to zero. Their failed turn preserves the same lifecycle receipt.
4. A component-target law maps a validated result with Captured, Accepted, and Retired receipt
   variants; it proves `kernel_turn_result_to_wit` has no `Result` branch after reducer commit.

The native law belongs beside the real bundle factory in
`🔌️plugin/🦀️.rs`'s `plugin_builder_contract_tests`; the component law belongs with the WIT
conversion tests. Neither needs a second reducer or a mock `PluginApp`.

No build or runtime test was run for this audit.

## Current-source update — callback-before-finish and native opening transaction

This section supersedes the first section's ordering observation for the current source. The
reducer now exposes crate-private
[`poll_kernel_output`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:99): it prepares the retained lifecycle receipt,
calls its `lower(TurnResult)` callback, and only then calls the registry's
[`finish_turn`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:752).
The WIT bridge is now that callback at
[`⚛️reactor/🦀️.rs:1132`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1132).
Therefore a normal WIT lowering `Err` retains the same staged ACK, receipt, slot, and native
owner. The only other current caller is native `poll_kernel`, with `Ok` as its lowerer. This
fixes the original ordinary-WIT-`Err`-after-commit defect in source.

### Registry/open acceptance

The new native registry has the right fixed-slot properties in the paths inspected:

- [`NativeLifecycleRegistry::admit`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:263)
  checks the exact existing open before allocating a serial; an identical retry returns `false`,
  while a modulo collision is rejected without replacing the live slot.
- [`remove_uncreated`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:276)
  refuses to detach either a created native owner or an installed lifecycle owner. The outer
  `InstanceOpen` path removes metadata and this still-Opening slot on either metadata admission
  or native-open failure ([`turn`:142-154](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:142)).
- [`plugin_open_actor_instance`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29675)
  preflights every fixed row, then installs the lifetime owner before publishing the app and
  actor rows. After `install_owner`, its remaining registry writes are `insert_admitted` calls
  whose capacity/collision checks were already made; there is no ordinary fallible operation
  that could produce a half-created owner.
- The exact instance/allocation key remains retained through close. `NativeLifetimeOwner` only
  releases reactor, patch, and pending reservations one at a time after the inner app lease
  reports retired ([`lifetime`:226-250](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:226)). The registry removes the slot only once the
  final ACK has successfully committed `Released` ([`:300-305](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:300)).

The direct cell law already proves a terminal-owner destructor is run once even when the clock
rejects the first final ACK
([`lifetime/🧪️tests`:39-80](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🦀️.rs:39)). The native runtime law covers a live modulo
collision and reopened lifetime after terminal ACK
([`runtime`:77-105](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime.rs:77)). No current slot replacement, owner Drop, or retry
escape was found in those source paths.

### Remaining bounded hardening

`GuestLifecycleCell::finish_turn` still reads and validates the real clock *after* the generic
`lower` callback ([`lifetime`:142-166](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:142)). The two present callers are safe enough in
the narrow sense that neither exposes a successful output before `poll_kernel_output` returns:
an outcome from `lower` is simply dropped if the clock is absent/backward/late. This is not a
current WIT commit-loss regression.

It is nevertheless an API footgun because the generic crate-private callback has no stated
purity/transaction rule. A future callback could send or persist its `T`, then see a late-clock
fault which retains the receipt and makes the same lifecycle output retryable. The smallest
durable boundary is one of:

1. restrict `poll_kernel_output` to a sealed, pure lowering trait and document that `lower` may
   not externally publish; or
2. preflight a single `ClockVerdict` before `lower`, then pass it to a nonfallible lifecycle
   commit method so no new clock read can fail after lowering.

The second form is preferable if more output adapters are expected. It must preserve the
intentional Retired behavior: after an already-issued `Retired` receipt is ACKed,
`prepare_turn` may release its terminal owner before that turn's output; a failed lower/clock
retains the Retired receipt for the next exact ACK, but must never attempt that release twice.

Add three native laws beside the existing real `TestRuntimeApps` path rather than a mock:

1. Inject a fallible lower for Captured and prove its `Err` retains the exact lifecycle slot and
   app allocation; a following native poll produces the same Captured receipt and one later ACK
   reaches Live.
2. Stage the final Retired ACK, make the first registry-level clock verdict fail after the
   one-time terminal release, then retry. It must release the exact registry slot once, retain no
   app/quarantine/close participant, and permit the same numeric instance to reopen.
3. Make the first real `InstanceOpen` factory/admission failure and a modulo-colliding open;
   both must leave no metadata, app, actor, or lifecycle row for the rejected request while the
   first allocation remains unchanged.

This update is source review only; `native53937` was already running and was not used as a
qualification receipt.
