# Drawing Text Companion Readiness Proposal

Proposal only. No Source or runtime change. Rust owns the funded production chain; dependency_layout may own the agreed test/fixture follow-up after the API plan is settled. Native11 checkers remain closed unused.

## Inner Documentation

Move the existing fitting emoji module docstring to the first line of scene/🔤️text/🧪️tests/🦀️.rs, before the imported public APIs. Preserve its text and every import/assertion. This restores legal inner-doc placement without any production or test semantics change.

## Private Partial-State Assertion

The aggregate overflow law currently inspects job.owners.plan to assert that at least one node became Glyphs while at least one remains Text. The production Owners type and its plan field are private. Do not make either public or add a test-only production accessor.

Use the actual existing public DocumentTextProgress.nodes frontier instead. Advance this overflow case in single-item turns, retain each successful public progress value, and at the expected segment-limit failure assert:

- The exact aggregate segment-limit error remains.
- The last successful public nodes frontier is greater than zero and less than the authored input node count.
- No successful progress reports done.
- Public result remains an error and into_retirement returns no completed output.
- The genuine controlled retirement still drains under exact quoted grants, with physical allocation/release observations and zero-cost final drop unchanged.

The first public frontier assertion establishes that one authored node completed publication; the upper bound establishes that a later authored node remained uncompleted. This preserves the partial-progress/nonpublication intent without exposing internal representation. The code maps nodes to self.at and increments it only after text replacement/publication, so it is the appropriate public semantic frontier. Keep the independent ttf-parser count proof that each node individually fits and the aggregate authored count exceeds the cap.

Single-item stepping here is intentional to observe the frontier immediately before a possible failure; a large batch can publish several nodes then fail without returning the intervening progress. This changes observation granularity, not limits or acceptance. When Rust defines the funded API, use its actual next-demand/grant/receipt protocol for these turns; never manufacture byte/depth budgets from the integer one. Exact refusal and physical receipt accounting must still be asserted.

If the repaired production API instead offers error progress explicitly, prefer its authentic public error receipt/frontier rather than broadening production visibility. The final test shape requires joint review once that API is agreed. This proposal does not claim current compilation or test passage.
