# Exact Resource Read Independent Review

Read-only source and retained-log review on 2026-10-08. No tests or compilers were launched by this audit.

Owners: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts`, adjacent `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, and `🧪️tests/🟦️.ts`.

The Rust extractor hashes original reader source and retains the actual operation token start as a UTF-16 offset. Core traversal selects ownership only for computed filesystem references with the same safe-integer offset, then requires exact source SHA. A second operation in the same file remains unresolved. Changed source retains both mismatch and unresolved evidence. Every selected file/directory input requires current typed digest; directory ownership also requires its captured directoryInputs identity. Fixture ancestry is checked before terminal directory handling and remains forbidden for actual fixture inputs.

Directory nodes validate current metadata and stop without reading directory bytes or claiming child-file consumption. The actual neutral test observes read calls and verifies this boundary; independent fast-glob and Bun SHA checks compare the owned directory digest. Six plain rows cover owned file, selective callsite, changed source, fixture file, directory metadata, and directory drift. The schema owns genuine per-input/per-operation contracts; it does not admit the example corpus.

RED: SHA256 `82e7486c32f38cbafe9029a5952bcbc110ca49cf26d117974cd5fd3b3bca9a5c`; retained terminal counts 12 pass, 74 filtered out, 6 fail, 112 expect() calls.
GREEN: SHA256 `319c3e6f4a908df213be3d92ed3c06bddb018e141f0489363e03e7a4761c9725`; retained terminal counts 18 pass, 74 filtered out, 0 fail, 132 expect() calls.

The GREEN log independently contains 18 passing tests, zero failures, 132 expectations, 74 filtered tests, and successful uncached Nx completion in 351ms. The RED retains 12 passes, six failures, 112 expectations and failed Nx completion in 373ms. These are focused core receipts, not the full runtime gate.

No concrete unsafe promotion was found in the reviewed core. Its deliberate trust boundary remains important: empty input arrays are allowed and discharge an exact computed operation, so they must originate from a verified complete producer observation proving that operation had no reads, never an arbitrary caller assertion. The core schema/digest checks alone do not establish capture completeness. Likewise directoryInputs must come from typed producer directory evidence, since terminal directory treatment intentionally avoids parsing those nodes as source. Runtime mapper/receipt verification owns these prerequisites and remains separately audited as it settles.

There is no file-wide filesystem waiver. This review does not establish unknown filesystem alias coverage, unobserved runtime paths, real Trunk publication, or a global runtime exclusion pass.
