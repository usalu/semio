# Recursive Retirement Capability Verification

The actual registered AppDefinition cleanup regression initially overflowed the ordinary Rust test stack. GDB identified a type-level cycle through `ArgSchema::controlled_retirement_supported` and `Box<ArgSchema>`; another cycle runs through `ActionArgDef.schema`. The overflow occurred before payload retirement. Increasing the stack would not terminate this query.

The defining RetireOwned protocol now passes a borrowed type-path capability query through derived owners and generic containers. An active type revisited along the same path is a recursive ownership edge. Every other reachable sibling is still checked, including an explicit unsupported owner. Frames are local stack borrows; the query has no mutable global, heap allocation, cached answer, or fixed-depth assumption. Actual payload cursor birth, progress, and disposal are unchanged.

The focused defining production-source regression passes direct and mutual recursion, rejects a mutually recursive graph with an unsupported sibling, observes zero allocations during each capability query, and retires an actual nested mutual payload under 4096-byte grants with exact physical birth/release receipts. Its neutral schema and fixture live under value/retirement/support/tests. A separate Bun oracle validates that fixture using Ajv and evaluates the same reachability verdicts using SQLite recursive queries: 1 test passes, 4 expectations.

The registered AppDefinition source regression also passed after this repair: original row identity, window declaration precedence, every independent denied currency, cancellation at six prefixes, shared-alias return, and paid declaration graph closure. A further reserved-but-empty ordinal backing guard was corrected from logical emptiness to `PagedList::terminal_is_empty`; the final rerun of that stricter guard remains recorded in generated/native-registry-source-test.log until verification completes.

Commands use the ticket energy-cargo wrapper and the ticket-installed Bun. No production source is copied into the ticket; harnesses include owning production test files.

This evidence does not establish completed native Energy app opening: the concrete immutable tool registration source and per-instance registration producer are still required, as are mounted ToolRun and full editor checks.

The native original declaration registry regression subsequently passed with strict 4 KiB copy/capacity and exact physical receipts, including reserved-empty ordinal backing closure.
