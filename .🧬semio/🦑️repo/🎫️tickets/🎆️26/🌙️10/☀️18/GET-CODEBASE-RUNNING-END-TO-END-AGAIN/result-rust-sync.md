# Result rust-sync (executor rust-sync, wave 1021a)

## Finding
The blocker was not the cursor but the derive: `#[derive(RetireOwned)]` on a struct emits a shared-original cursor whose struct and impl carry `Self: Sync` (a trivially false bound for any non-Sync concrete owner, E0277 at the derive site). `RetirementCursor` is deliberately Send-only (peer design: "shared access belongs to an explicit owner gate", `RetainedOwnerGate`, `retirement/🔐️mutex` test with a `Cell` cursor); a `Sync` bound on `DomainOwner`/`OriginalDomainFailure` (uncommitted, 07:10) contradicted it. Decision: keep cursors Send-only and the peer's gate untouched; make shared-original custody opt-out.

## Edits
- Derive: new container flag `#[value(send_only)]` (`✨️derive/⚙️expansion` `ContainerAttrs.send_only`, `declared_send_only`; `✨️derive/🧬️retained-clone` skips the shared-original cursor and methods; trait defaults then apply). Test: `✨️derive/🧪️tests/🧬️generic-owner` (`send_only_owners_derive_...`, passes with the existing test).
- job: `DomainOwner: Send` (Sync removed), `OriginalDomainFailure` Sync bound removed; `#[value(send_only)]` on `OriginalTurnFailure`, `OriginalCallerCloseReturn`, `OriginalClosureFailure`.
- tool-run: `#[value(send_only)]` on step-ring-insert `State`; `validate_tick_payload`, `ToolRunPayloadUpdate::wire_field(_count)`, `ToolRunTraceSubject::ordinal` made `pub` and the first exported from the package root; `ToolRunTickWriter` read accessors (`original_progress/_retract_to/_payload`, `steps()`, `pages()`, ...), `payload_update` testkit-visible; `ToolRunStepRing::original_steps(_mut)`; tick-wire encode/receive use them; tick-wire dev-dependency on tool-run with `testkit` (carve fallout, not Sync).

## Verified (lane `rustsync`, `bun repo cargo --store=shared`, check)
- `semio-framework-job --lib`: green. `semio-framework-tool-run`, `-tick-wire` `--lib --tests`: green.
- Plan line 80 (pack-format-retained ... job, `--lib --bins --tests`): green. Plan line 104 (ui-contract set): green.
- Plan line 88 (tool-run set): tool-run and tick-wire green; remaining reds are not Sync: `semio-framework-pack` test `pack_record_laws` imports `semio_framework_value::paged_text` (crate `🌱️value/📝️text/📦️paged` not re-exported), and `semio-framework-pack-record-encode` lib test misses a `blake3` dev-dependency.
- Not run: derive/expansion unit suites beyond `generic_retained_owner`, the rest of the plan.
