# Build Script Budgets

The reported WASM failure comes from `buildBudgetMs()` returning 1,200,000 milliseconds for the engine script subprocess. The same default feeds Cargo, WASM packaging, warm test compilation and exact Cargo law compilation. Generic command and Nx/script/daemon wrapper defaults also impose implicit deadlines on builds.

Use zero to mean unlimited across subprocess runners, matching the platform `spawnSync` timeout contract. Non-test budget classes default to zero; positive `budgetMs` and existing environment overrides remain opt-in deadlines. Test assertion defaults remain finite. Async build runners must omit their timers for zero, and the captured subprocess bridge must not turn zero into its own five-second deadline. Coverage compilation is separated from assertion execution: nextest uses --no-run and the cargo-test fallback uses --list, followed by execution with --no-clean under the existing test budget.

Validation will use language-neutral JSON vectors, actual child-process completion/timeout/cancellation, and the existing Execa package as an independent subprocess reference. Existing command-budget and exact Cargo law tests will also run through Nx. No full WASM rebuild is needed to verify timeout behavior; these tests exercise the shared runners directly.

Repository MCP access uses the configured local stdio server because this session exposes no repo MCP tools directly. Read `repo://goals`; reopened the existing spawned-script-budget ticket under the AI-optimized Repo goal. The server accepts a two-digit year in ticket paths and moved the ticket when its title changed.
