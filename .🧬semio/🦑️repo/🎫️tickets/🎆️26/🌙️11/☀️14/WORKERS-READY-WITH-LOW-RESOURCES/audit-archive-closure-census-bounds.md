# Archived Graph Closure Census Limits

The straddled read-only probe of archived generation 259 verified all 5,101 fragment digests. It measured 437,801,505 expanded bytes across the fragments, with the largest at 15,377,114 bytes, and counted 20,671 distinct typed closure references and 73,950 distinct owner-to-reference links. Bounded product scanning stopped after 4,553 fragments with the previous 20,000-reference and 65,536-link ceilings.

The repair evidence schema now permits 24,576 typed references and 98,304 links. The repair receipt still permits at most 64 missing leaves and 128 affected owners. The census retains one array per reference instead of allocating a set for each reference. Each owner is visited once and all of that owner's tokens are visited consecutively, so comparing the last entry suppresses duplicate tokens without losing shared ownership across fragments. The separate 512 MiB expanded aggregate and 16 MiB per-fragment ceilings remain enabled.

A private SQLite-backed regression reproduced refusal at 20,671 references before the policy change. It now verifies eligibility across both previous limits independently and at the measured combined shape. It also verifies that repeated tokens in an owner do not consume additional links, that exact new limits qualify, and that one over either limit refuses before verification or receipt publication. All seven repair tests passed with 85 assertions. This pass only used private input and did not mutate the live archive.

## Bounded leaf verification

The absence census and replacement qualification use the shared closure concurrency policy of eight. They run one typed-reference batch at a time and wait for all its started reads before advancing. An unknown or rejected result, cancellation, a disqualifying missing leaf, or an overfull missing list stops subsequent batches. Eight 64 KiB verifier buffers correspond to 512 KiB of stream scratch, excluding runtime and index overhead. Equal digest strings in different namespaces still receive distinct checks, and a repeated typed key receives only one check.

The red fixture observed a single active verifier where eight were required. The final fixture confirms the eight-check bound, the early batch edge for unknown, missing, overfull and cancellation, and no in-flight checks after refusal. A separate fixture reverses completion order and verifies the same canonical receipt digest. The combined receipt and original closure suites passed 15 tests and 170 assertions.

- [Closure repair limits](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🧬️schema/🔣️.json)
- [Ownership census](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🟨️.mjs)
- [Private regression](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🩹️closure-repair/🧪️tests/🟦️.ts)
