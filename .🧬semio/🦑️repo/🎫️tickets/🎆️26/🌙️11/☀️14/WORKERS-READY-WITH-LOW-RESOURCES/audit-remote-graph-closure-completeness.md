# Remote Graph Closure Completeness

A sealed zero-owner generation named a closure in its global fragment. The private loopback cache accepted an action result without the leaf. The reader classified that head as complete even though strict restore could not obtain the closure. The regression initially failed with expected `partial`, received `complete`.

The publisher compares every referenced closure with the entries admitted into the action result. The upload limit still bounds closure entries and the report limit still bounds missing diagnostics. Owners introduced by references outside the report limit count toward staleness. A missing global closure makes the head partial for every owner. When it has zero owners the stale count is omitted, so the reader still classifies the head as partial. An unavailable closure port or scan failure cannot certify closure completeness. Local upload reads are bounded by the size used in the digest request and must match both that length and SHA-256. The existing sent memo and validated server refusal retry remain in use.

An explicit fresh publication can upgrade its own partial head at the same signature and pointer time, including through the warm load's `ifAbsent` call. The existing head stays in place until the full closure set qualifies. Cancellation stays scoped to the attempt.

The private fixtures use a real local cache and an independent SQLite oracle. They cover missing global references with zero and one owner, strict restore refusal, same-manifest retry and cancellation. The entry-limit fixture verifies exactly 20,000 admitted closure entries, report truncation at 64, and an omitted owner that contributes to the stale count despite appearing beyond those 64 reports. The eight remote-exchange files passed 41 tests. The final focused test file passed 3 tests with 43 assertions after cancellation checks were added. No live fleet service or cache was changed.

- [Publisher](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts)
- [Regression tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🧹️lost-leaves/🟦️.ts)
