# Bun Ownership Diagnostic Static Review

Accepted for the bounded frozen diagnostic change. No confirmed guard/control/provenance regression found by static inspection. This reviewer did not execute tests, repository imports, factories, Stores, initializers, registered routes or native processes. Existing generated receipts were retained.

Independently verified current bytes against Runtime's three-file freeze:

- Defining Bun `🟨️.cjs`:22519bytes, `525e20d7fd96fc4385fe8bcfdb554a69c013865e373abd01fc3a8d2ce832e0ae`.
- Nearest `🧪️tests/🟦️.ts`:34865bytes, `affac9f7931b6d3787dcc600ecce146ef88d90fe2c2c28ddba64d5eb54f3d3bb`.
- Existing corpus `🧪️tests/🧫️fixtures/🔣️.json`:19415bytes, `bdd5819155dbca3da6607678bc446f386e21cbc47cd2d9f2b974f972d2ca6439`.

The original retain key and JSON equality remain unchanged; a mismatch still throws at the original refusal. Observation membership/filtering, byte-name sorting, original signal checks, awaited control.advance operations, currentFence replay and manifest/policy fences remain in place. The new helper only derives error evidence from already observed previous/current values; it does not reread, replace, reset or publish ownership facts.

For list mismatch, a linear merge over the existing sorted rows deterministically reports the first eight changed names across added/removed/typeChanged categories combined. Total counts every difference; omitted is max(0,total−8). Each emitted row contains only existing child name/kind or before/after kind. State mismatch reports its existing fixed-length identity arrays. No writer, actor, process identity, timing explanation or provenance attribution is inferred. The row count is bounded; this is not a new arbitrary-string byte truncator and retains the actual filesystem names/path needed for diagnosis.

The nearest corpus covers mixed membership/type changes, unchanged membership and truncation. Tests trigger change through controlled callback timing and inspect the actual refusal payload. The independent fast-json-patch compare oracle is imported only by the test file, derives changes from separate name-to-kind maps, sorts oracle rows and compares totals/retained category prefixes; production helper has no external dependency. Authored expected payloads separately check deterministic combined truncation. This is test-source review, not this reviewer's assertion that tests passed.

Actor15's original lost old/new values cannot be retroactively recovered. The diagnostic prepares a fresh original authorized reproduction to preserve the next differing facts while retaining the same strict guard. It does not justify attributing Actor15 to any writer or changing cache, Source namespace or ownership policy.
