# Scope Transfer Audit

## Proven from current code

- `🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs:32` moves the existing scope claim and calls `rebind_stock_destination` from the scope address to the target `Option` address and extent. `CreditedOutput::adopt` at line 92 allocates that target in a box. The claim remains held until settlement.
- `🧵️job/⏱️context/🛬️owned/🦀️.rs:70-73` adopts only if the current decoder has a scope requiring retirement. Production `🔌️plugin/📜️declaration/👷️job/🎟️request/🏗️construction/🦀️.rs:34-45` invokes it after tool construction settles, attaches any credit to the `ErasedToolJob`, then yields the candidate. The retained-command producer and command staging call this construction route. This establishes a producer/tool construction route, not that the current Note/WFC Input dispatch uses it.
- Host `🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🪙️guest/🌱️history.rs:195` verifies each existing serial, and `🌱️escrow.rs:48-49` requires source, birth, recipient, destination, extent, serial, origin epoch, producer and purpose to match. Thus a previously recorded row with changed destination or extent is refused, even when the next row is settled. Counters must remain monotonic, and settled totals cannot change.

## Hypothesis and boundary

If one host book validates the scope row before the production tool-construction transfer and later validates the same serial after the rebind, the second snapshot is rejected. This is a code-level conditional, not proof that Note/WFC Input reaches that transfer or that both snapshots contain this row. The row might first appear only after transfer, retire or be acknowledged before publication, or belong to a different guest operation/book. No live trace, publication sequence, or heavy build was run in this audit. Do not attribute the current Input receipt failure to this hypothesis without correlating operation, source birth, serial, destination and publication revision on both sides of the transfer.

## Safe law and options

Under the current host law, an observed serial's full identity including destination and extent is immutable through settlement; recipient, issuer/source birth, producer and purpose also stay fixed, serials must not replay, and currency totals remain conserved and monotonic. A safe design can keep a stable physical claim slot for the whole lifetime while transferring only custody of its handle, or introduce an explicitly authenticated, ordered, one-time ownership-transfer record with old and new destinations bound to the same issuer, recipient and serial and validated atomically before replacing host identity. Another option is a funded settlement and fresh serial issuance, provided the old claim is retired exactly once and no overlapping hold or replay is possible. Simply ignoring destination or accepting arbitrary row changes loses the current identity guarantee.

The decisive regression should force a live receiving claim across two validated SRE1 snapshots with transfer between them, then retire it; assert unchanged issuer/recipient, exactly one outstanding hold and settlement, rejection of stale destination, duplicate transfer and replay, and no double accounting. First establish whether that is the actual Note/WFC Input path.
