# Key lookup and owned rows review

Key lookup accepts an arbitrary row_key function and calls it within a turn quoted as one work unit. The function pointer identity does not itself bound the work or prevent allocations; use a bounded projection contract with truthful quote/receipt. Source bindings and close use the current owned APIs. No test run.

The trace-memory-testing crate is not declared in the root workspace Cargo.toml or current Remodel diff manifest. Its firstparty manifest is framework modules trace/memory/testing/requests/packages/rust/Cargo.toml; existing replication wire uses a relative path. OwnedRowsBuffer requires actual allocation observation for reserve behavior; no additional concrete defect established in this bounded source review.

Revised direct-key trait has a documented constant-time stored-String projection contract; row scan selects two work units and max-work one refuses before projection after bindings. Current native tests explicitly exercise budgets zero and two, but not row-phase budget one. Memory test package lib path is ../../🦀️.rs from its Cargo directory; relative path from Remodel diff package is thirteen parent components followed by framework/modules/trace/memory/testing/requests/packages/rust with the repository emoji names. No native execution.
