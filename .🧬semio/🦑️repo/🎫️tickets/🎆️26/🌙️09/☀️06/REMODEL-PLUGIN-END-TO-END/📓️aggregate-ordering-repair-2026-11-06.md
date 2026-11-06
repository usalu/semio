# Canonical Test Aggregate Ordering Repair — 2026-11-06

Full Remodel Vitest 38976 returned actual terminal exit 1 before any test body. Its retained log reports invalid build.test.aggregate-stale in graph preflight. Mutation contract 64314 later returned terminal exit 1 for the same graph refusal; no contract scenario is inferred.

Root invoked the actual current canonical derivation under normal repo-admitted Bun. The authored and expected aggregate each contain 196 dependencies, with identical sets and no other aggregate field difference. Actual isDeepStrictEqual was false because test-build-browser-preparation had been inserted out of canonical order, shifting later entries.

Root changed only the dependency array order in caching/📋️project.json to the canonical derivation. The edit asserted that the sets were unchanged, the complete parsed manifest differed only in that array, and source bytes were unchanged immediately before writing. The actual requireCanonicalTestAggregateV1 then accepted the result. A separate read-only agent independently ran that validator and confirmed 196 dependencies with browser preparation at index 20. No target, test requirement, cache policy or aggregate validation was removed.

Retained receipts: aggregate-drift-2026-11-06.log, aggregate-order-2026-11-06.log and aggregate-repair-2026-11-06.log. Independent review: 📓️aggregate-repair-review-2026-11-06.md.

One normal full Vitest replacement is 32016, vitest-aggregate-fixed-2026-11-06.log. One normal same-case mutation contract replacement is 76414, mutation-contract-fixed-2026-11-06.log. Both remain pending. Descriptor 30944 remains live; its ordinary implementation/ownership guard already performed one bounded restart on changed workspace ownership. No healthy process was canceled or manually duplicated.

This repair clears the demonstrated current-source ordering refusal, not full graph publication, Vitest, mutation parity, native compilation, component provenance or real host acceptance.
