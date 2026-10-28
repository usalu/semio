# Native IO Audit Fixes

Both P2 findings in `native-route-audit.md` are addressed in the native source implementation, with tests and language-neutral scenarios added first.

## Complete Output Bounds

Export now uses the same physical record scanner for admission of the generated header and the complete final source. Every header record is bounded after STEP escaping, including a zero-entity graph. Data append checks reserve bytes for the complete final ENDSEC/exchange terminator, and a final common scan checks total bytes and every physical record before publication. The scanner's record-size comparison now rejects the next byte after the exact allowed extent rather than accepting a semicolon one byte beyond it.

The internal `write_with_limits` path lets focused tests exercise the real implementation at small configurable limits rather than allocating 8–256 MiB just to test a boundary. Tests cover 200 ASCII header characters, 30 characters expanding through STEP Unicode escapes, a data-bearing output one byte above the complete total limit, exact total acceptance, exact record acceptance, one-byte-small record refusal, and reimport of accepted output. Public admission bounds remain 8 MiB per physical record and 256 MiB per complete source.

## Original Source Token Offsets

Import retains original header and instance origins. A small token-span cursor runs beside the shared parser and mirrors its already-parsed typed/aggregate/complex value tree; it does not implement an alternative STEP data model or grammar. It skips comments and quoted string/binary payloads while retaining each value's original UTF-8 byte position. Final id/reference/name checks use those spans. Decimal finite-domain checks traverse the retained original nested spans before graph projection. Parser errors translate synthetic-frame character offsets back to original UTF-8 byte offsets, including Unicode earlier in the record; header parse segments likewise retain their original source locations.

Tests use earlier comments and Unicode strings containing decoy `#999` and `1.E999` text, followed by a later duplicate id, nested reference, or typed nested overflow. Expected offsets are the original source's exact final token byte positions. Export diagnostics refer to produced byte positions when physical source exists, or zero for graph refusals before source creation. Cancellation outside byte-scanning phases is explicitly unlocated rather than misrepresenting a node counter as a byte offset.

## Verification

- **Passed:** rustfmt parsing and formatting for `step.rs` and `native.rs`.
- **Passed:** existing STEP TypeScript regression suite, now **14 tests/32 assertions**; direct optional `complex` reads were normalized to the graph's actual optional field. No compatibility wrapper or model change was introduced.
- **Passed independent representability baseline:** `native-source-oracle.py` with IfcOpenShell **0.8.3** round-trips ordinary ASCII, escaped Unicode and short header names in real IFC4X3_ADD2 source, preserving point coordinates and schema. Independent produced source extents were 394, 322 and 199 bytes. This demonstrates representable escaped header output and remains separate from native Rust execution.
- **Unverified:** the new Rust tests and native generated-source differential cannot execute while authentic repository compiler ingress remains unavailable. No compiler/configuration bypass was attempted, and no native runtime success is claimed.

Owned production edits are restricted to `🚪️io/step.rs`, `step.ts`, and `🧪️tests/{native.rs,native.feature}`. The parent/sibling geometry, MVD, catalog and registration work was preserved. Token-span storage adds bounded source-analysis memory and still uses the already-disclosed synchronous per-record grammar/writer path; original receiving-ledger allocation accounting and retained registered composition cancellation remain open work in the earlier native IO report.
