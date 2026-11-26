# MCP process arena source law

The focused suite reproduced the failing law. Its final assertion required the literal compact sequence `if let Err(error)=result{error.exit()}`. The actual bootstrap handles the same failure inside its dedicated serving worker using normal formatted Rust.

The Tree-sitter test now verifies the real stdio and HTTP mounts call the process arena’s `serve` function. It checks that the actual serving worker routes an `Err` result to `error.exit()` and that these files declare no static arena owner. The pinned parser checks the relevant bootstrap function independently of foreign declarations elsewhere in that file. The custody fixture still uses Ajv and SQLite as independent oracles.

The pinned Bun file suite passed 2 tests, with 12 assertions. The ordinary repository preload attempted a process-table read denied by the sandbox, so the focused run used the pinned Bun executable through the existing repository tool interface with a ticket-local test configuration.
