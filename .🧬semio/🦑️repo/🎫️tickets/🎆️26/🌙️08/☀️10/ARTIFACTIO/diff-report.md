# Diff Codec Extraction Report

## Ownership

`DiffText` owns print/parse under replication text I/O; `DiffBinary` owns deterministic encode/decode under binary I/O. `DiffCodec` is the blanket marker requiring both representations. Replication and OS facades expose all three contracts.

The empty transport-only `DslDiff` derive was removed entirely. Function macros `diff_text!` and `diff_binary!` generate wire implementations at their representation call sites. Eight genuine deriving types have paired I/O invocations: BinaryDiff, TxtDiff, Mp4Diff, PngDiff, BmpDiff, DwgDiff, SpaceDiff and CollectionDiff. `DslArtifact` still emits metadata only.

51 explicit codec owners were split physically, including Writer's existing text owner. Processing covered 60 source owners, including one snapshot file whose prose mentioned a derive; comment-only macro matches were removed. Codec helpers are private to representation trees, with explicit cross-representation imports. No schema wire facade was introduced.

## Reverse Audit

Final helper classes: 1,361 text functions; 676 binary functions; 46 wire constants; three record mirrors. The named functions are encode/decode, print/parse, hexadecimal/string/list/tuple codecs, binary reader/writer and error conversion helpers. LAS constants are binary bit-mask tags. Record mirrors are PptxDiffRecord, ZipDiffRecord and PdfDiffRecord.

The dependency traversal initially selected 420 text-shaped helpers through a binary caller; these were reassigned to text I/O and binary callers now import their text authorities explicitly. The audit restored animation `indexed_is_empty`, drawing's test-only `transform`, and EPW's field-count assertion to schema. No apply, inverse, diff construction, validation, absorption, restoration or composition authority remains in the moved helper inventory.

262 consumer files received trait imports or diff helper ownership repairs. A separate ownership repair corrected 272 files with prior generic-name imports that pointed to an artifact standard absent from their local owner. Consumer rewriting is scoped to each artifact and excludes legitimate existing standards.

## Verification

- The package router `bun ./📜️script.ts test-exports-source` passed after updating the language-neutral macro export and owner rosters. The new neutral diff representation fixture is schema-validated with Ajv and independently checked using tree-sitter's Rust grammar. A temporary `[DEBUG]` runtime line confirmed both wire contracts and was removed.
- The first source test failed because its record-owner fixture still expected removed `expand_dsl_diff`; the fixture and both representation contracts were corrected before the successful rerun.
- A new native Syn test parses each actual generated token stream and compares its trait and exact method roster with the same neutral fixture.
- The native Nx derive test is still queued in shared project/workspace preparation. No native Rust test or artifact codec roundtrip pass is claimed.
- Source syntax validation parsed 5,397 Rust files under the affected owners. Two extraction-offset comment fragments in CSV/Docx were repaired. Final syntax validation after helper ownership refinement passed with zero classified errors. The existing valid DWG `500..` match pattern is unsupported by the installed tree-sitter grammar and is explicitly excluded from syntax-error classification.
- The comment/string-aware production audit inspected 11,437 artifact source files and found **zero** semantic DiffCodec/DiffText/DiffBinary implementations, DslDiff derives or diff_text!/diff_binary! invocations.

## Remaining Coordinator Work

Run the final live architecture gate with DiffText/DiffBinary and the two wire macro invocations included in purity rules; update the old same-schema DiffCodec coverage policy to check independent I/O owners. Shared native cargo preparation must complete before claiming full Rust build and codec runtime closure. The exact native command is recorded in generated logs and remains active.

## Files

See `diff-changed-files.md` for the deterministic source/representation/mount scope. The ticket `diff/📜️script.ts` retains temporary extraction and audit inputs; raw outputs remain under the ticket's `🗑️generated` until coordinator cleanup. No AGENTS files, worktrees or modifying Git operations were used.

## Final Handoff

The independent residual scan identified and moved video/audio/docx `parse_usize` into text diff I/O and PLY `write_bin_snapshot` into binary snapshot I/O. Native Nx verification remains active as session `51913`: `bun nx run @semio-tech/dsl-derive-rs:test --skip-nx-cache --outputStyle=static --lib diff_representations_generate_only_owned_methods`. Expected outcome is one native Syn test comparing the actual generated text and binary trait/method rosters with the neutral fixture. The log is `🗑️generated/diff-native-tests.log`; only project-graph preparation was printed at handoff. No native result is claimed.
