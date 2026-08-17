# Dirty Quarantine Snapshot: UI-Adjacent Wave 4

## Read-Only Sample

- HEAD remains `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`.
- Repository short-status count: 3,719 paths.
- Active plugin short-status count: 3,456 paths.
- stdio plugin short-status count: 465 paths.
- Framework UI short-status count: 45 paths, including the serialized semantic leases recorded in this ticket.

## Decision

The plugin and stdio waves are still advancing relative to the prior 3,263/446 sample. Keep plugin registrars, stdio, full deterministic census regeneration, workspace lock regeneration, and global formatting quarantined. Continue only current-hash graph-colored owners outside that moving closure.
