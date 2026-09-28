# OPC Publication Integrity

The office container writer currently builds a `HashMap<String, Vec<u8>>` by inserting typed metadata and then every content part. Duplicate content paths and content parts named `[Content_Types].xml` or an existing relationship path overwrite earlier payloads. A source object can therefore lose a member during save without a refusal. The custom order callback also receives every member name but is not checked as a permutation: omitted names disappear; repeated or unknown names reach an `expect` panic. Finally, explicit empty relationship lists are skipped during encoding and absent after reopening, violating exact package equality.

The neutral publication fixture names valid and invalid path permutations, collision paths, duplicate archive members, and an explicit empty relationship owner. Four native tests were authored first. Positive cases inspect the emitted archive through the independent `zip` reader, and compare the complete decoded package. Negative cases require typed refusal without source mutation. The focused pre-fix run is `🗑️generated/opc-publication-red-1.log`; it was started through the existing Nx ZIP test target. Execution is pending at this checkpoint.

The implementation will reject duplicate archive names before OPC interpretation, reject content/member collisions before encoding payloads, preserve explicitly present empty relationship lists, and require the custom order to contain each known member exactly once. This is a narrow integrity repair. It does not claim full ZIP header metadata preservation, arbitrary XML extension preservation, or bounded asynchronous encoding.

## Publication Guards Implemented

The pre-fix native attempt exited before assertions after 36m50s: one incorrect OPC fixture include depth and the in-flight ZIP metadata expansion prevented compilation. The fixture include is corrected. No failing assertion was observed in that run. The tests were authored before the guard implementation.

Production now rejects duplicate ZIP member names before OPC interpretation, ambiguous content/metadata paths and duplicate relationship owners, and any ordering callback that omits, repeats or invents a member. The callback no longer reaches an `expect` panic. Explicit empty relationship parts are emitted and can roundtrip. Independent ZIP inspection and exact package roundtrip assertions remain queued for the next coherent ZIP checkpoint.

## Office Snapshot And Diff Fidelity Follow-Up

Source inspection of the existing DOCX, XLSX and PPTX base `*OpcDiff` types shows they carry content types, parts and relationships, but omit the existing `OpcPackage.comment`. Their `diff_opc` comparisons likewise do not consider comment-only edits. Several hand-authored snapshot/mutation decoders reconstruct `OpcPackage` from only these three channels and supply `..Default::default()`, losing a nonempty archive comment. This remains a source finding, not a newly executed failing law.

The new ZIP header model also needs an authoritative OPC entry-metadata owner covering content parts, `[Content_Types].xml`, and relationship members; attaching headers only to `OpcPart` is insufficient. Root and core execution agree on a separate entry metadata map plus archive-comment encoding. Its normal form, authored constructors, complete edit/diff/inverse/codecs, schema/TypeScript definitions, fixtures and cooperative DOCX copy/retirement must be changed coherently. This follow-up is required before claiming every office artifact detail survives editing and save/reopen. The narrow publication guards do not solve it.

## Archive Comment Replay Repair — 2026-09-28

A shared neutral fixture and archive-comment change schema now cover exact Unicode, newline, delimiter and empty text. DOCX, XLSX and PPTX each have a focused law requiring a comment-only diff to apply, invert, absorb an explicit clear, and survive text/binary replay; full SetSnapshot mutation replay must retain the same comment. The independent serde_json projection verifies the exact field value. These tests were authored before the implementation and are not yet executed.

The three office OPC diff types now include an optional explicit comment edit. DOCX/XLSX hand-authored text and binary codecs include that field in both diffs and full snapshots. PPTX already uses value codecs for these payloads, so its missing diff field/semantics were the necessary repair. All authored subset literals explicitly preserve comment when unrelated namespace/part mutations run. This changes the current protocol without compatibility parsing. Rustfmt syntax checks ran; focused native validation is pending.

A separate source finding is that office JSON/TypeScript snapshot and diff schemas still contain old entries/bytes stubs inconsistent with actual Rust opc/document/workbook/presentation data. Text execution owns the accurate snapshot schema follow-up; root retains the diff/codec follow-up. No schema completeness claim is made here.

## Remaining OPC Representation Gaps

Current OPC content-type and relationship XML readers retain declared semantic fields but skip unknown elements, root/entry extension attributes, comments and processing instructions; serializers regenerate those XML files. The content-type reader also normalizes extension case. The current writer places metadata members before content parts, so it does not preserve arbitrary original cross-category ZIP member order. Source comments claiming unconditional losslessness were corrected. Authoritative ordered header metadata alone will not fix these XML/order gaps; a complete representation must retain and edit them explicitly. These gaps remain open and must not be hidden by passing canonical sample-package tests.

## Metadata Namespace and Identity Slice — 2026-09-28

Root authored a neutral prefixed metadata fixture and a save/reopen law with independent ZipArchive inspection. OPC content type and relationship parsing now resolves names against the declared namespace, accepts valid prefixed roots/children, refuses wrong roots/namespaces, rejects case-insensitive duplicate Default extensions, duplicate Override paths and duplicate relationship IDs, and rejects unknown TargetMode values. Default extension spelling is retained while lookup and replacement remain case-insensitive. Rustfmt and scoped whitespace checks passed. This new native law has not executed; no observed red/green is claimed. It is separate from pending header/order/full metadata XML preservation.
