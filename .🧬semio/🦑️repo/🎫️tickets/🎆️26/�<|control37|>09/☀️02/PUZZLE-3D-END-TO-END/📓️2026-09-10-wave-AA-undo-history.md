# Wave AA — example-switch history hygiene (2026-09-10)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Repo MCP unavailable (noted like other waves).
Serve `:6014` wasm #31 left running; no rebuild/deploy this wave.
Coordinator took over defect 1 (shell Undo mount). This wave owns **defect 2 only** (guest).

## Stake

- Defect 2: Forest→Nakagin leaves TWO History rows (`Resize Window` + `delete-object id=seed-left-001`) instead of one row labeled "Set Active Example".
- Files: guest `✏️editor/🦀️.rs` (`Puzzle3dSetActiveExampleWork`), `🎮️commands/🛍️set-active-example/� (`Puzzle3dSetActiveExampleWork`), `🎮️commands/🛍️set-active-example/🦀️.rs`, `🧪️tests/🔬️example-switch/🦀️.rs`.
- Native law-prove only. Flag: **needs wasm rebuild**.
