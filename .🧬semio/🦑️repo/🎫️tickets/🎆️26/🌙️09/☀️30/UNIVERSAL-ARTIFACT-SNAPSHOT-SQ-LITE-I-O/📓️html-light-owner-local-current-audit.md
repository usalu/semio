# HTML Light and Owner Local Current Audit

Read-only production inspection on 2026-10-03. Only this report was created. No Cargo, Native test, Source rerun, Git mutation or worktree operation occurred.

## HTML Correction and Scoped Comparison

Current HTML snapshot SQLite owner (`🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`) line 122 contains `let mut built = OwnedNodeMap(BTreeMap::new());` after stack and visited declarations. A narrow original raw-map comparison replaces that initializer declaration only with `let mut built = BTreeMap::new();`; existing remove/insert call sites need no edit. Preserve OwnedChildren and all metadata transfer ordering. Line 132 enters SqlLateCopyScope; line 133 copies schema and optional doctype before removing root. Concurrent source must be reread before any replacement.

Lifecycle Native test line 47 reads cancelUtf8ByteBoundary, captures production VISITS immediately after from_sqlite_database, determines refusal, retires any Ok candidate, retires the original fixture, then asserts reached/refused/visits. The neutral JSON retains depth 8192, stackBytes 65536, lateTextUnit 界, repeat 32768, cancelAfterBytes 65536, and corrected cancelUtf8ByteBoundary 65535. This is 98304 UTF8 bytes; copy_text's first complete UTF8 frontier is 65535. The old predicate requiring a completed frontier at least 65536 and below 98304 cannot observe either real frontier.

Exact Native law basename: sqlite_snapshot_html_late_sql_metadata_cancellation_retires_complete_deep_tree. It is mounted as schema::snapshot::sqlite_lifecycle_tests; the isolated helper derives its full selector from module_path, passes --exact --test-threads=1 --nocapture, and requires exactly one passed test. Existing registered Source selector is @semio-tech/stdio-html-rs:test-snapshot-sqlite-source. Its preserved green receipt at 🗑️generated/html-late-utf8-witness-source-green.log shows 15 pass, 0 fail, 151 assertions, Bun 886ms, uncached Nx 5.9s. This audit read that receipt; it did not run tests. The correction does not prove the Native small-stack guard behavior.

Native lifecycle temporary DEBUG markers are absent. DEBUG console oracle messages remain in lifecycle Source line 33, cohort Source line 34, and the main Native SQLite test's independent HTML parser oracle line 70. These are existing runtime witness markers; Root should distinguish them from removed diagnostic lifecycle markers before cleanup.

## Nine Current Allocation Frontiers

The prior owner-local report remains accurate for the directly reread SQLite modules. Typed ValueError closures and terminal into_message projections do not admit allocation. These are representative source findings, not allocator receipts or exhaustive censuses.

| Owner | Current unadmitted owner backing |
| --- | --- |
| Binary | SQLite lines 14/20 direct cloned schema and row/value vectors; lines 37–41 byte output pushes. |
| CSV | Lines 30–42 direct projection row/value/schema/field clones; lines 64–70 record-id BTreeSet and grouped field BTreeMap/vector; lines 72–88 output records/fields, stable sort and text copies. |
| TSV | Lines 31–43 direct projection; lines 66–72 owner indexes; lines 74–89 output fields/records, stable sort and text copies. |
| BMP | Lines 24/29/35 direct row/value projection; palette pushes 56–60; pixels and seen allocations 62–63. |
| Deflate | Payload Vec and pushes 29–31; direct schema text conversion. Projection delegates to shared Projection, which is a separate provider obligation. |
| PNG | Direct schema conversion 73; palette/alpha growth 80/84; text and unknown indexes 96/104; RGBA and seen allocations 101–102; grouping and stable sorting 106–108; chunk marker/output growth 113. |
| LAS | ordered_refs vec and BTreeSet 19; component map 23; VLR/octets indexes 56–57; output reserves/data collect/text copies 58; point indexes and output reserve 59. |
| GLTF | Read indexes 54, consume/ensure_keys/groups 63–75 and rows Vec::with_capacity remain local backings. Native override line 95 still instantiates NativeDecodeControl from max_value_bytes with no allocation_stage settlement. Shared Reconstruction text usage does not cover those indexes. |
| SVG | Current line 30 delegates to typed reconstruct_xml_document, then validates root. Allocation repair belongs to XML reconstruction, not a duplicate SVG map implementation; named-subset diagnostics are another owner path. |

Admission must precede actual requested backing and remain cumulative across phases and retries on the same control. Row/value checks, guards, String error categories and native draft allocation methods do not prove this. BTree internals cannot be charged by guessed ABI. Replace indexes with explicit admitted storage or provide an actual controlled provider. Stable sorting may own scratch and requires its own admitted implementation or allocation-free ordering.

Related evidence: 📓️owner-local-construction-admission-readback.md, 📓️nine-owner-sqlite-typed-api-prerequisites.md, 📓️html-residual-runtime-diagnosis.md, current-html-corrected-ownership-readback.md. Root owns the Native red/restored guard comparison and all compilation/runtime verdicts.
