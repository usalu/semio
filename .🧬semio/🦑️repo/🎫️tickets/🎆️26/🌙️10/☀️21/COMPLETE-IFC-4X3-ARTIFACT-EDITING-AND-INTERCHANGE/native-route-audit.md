# Native IFC4x3 and Route Audit

## Findings

### P2 — Export can succeed with output that its own importer refuses

File: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/step.rs`, lines 326–345 (particularly 327, 329, 339 and 344).

The header is serialized and copied without any source or record bound check. Only entity bodies are checked against MAX_RECORD_BYTES and MAX_SOURCE_BYTES, and the final terminator is appended after the total check. A syntactically valid native graph with zero entities and a FILE_NAME string of 8 MiB or more returns source successfully although Records::next rejects the FILE_NAME record on reimport. A still larger zero-entity header can exceed 256 MiB and export successfully. Even with normal headers, a final entity which takes the current output exactly to MAX_SOURCE_BYTES passes line 339, then the terminator exceeds the admission limit.

Repro contract: start with the valid minimal native graph used by native.rs; set FILE_NAME attribute zero to a string with 8 * 1024 * 1024 ASCII `a` characters; retain no entities; export, then reimport. Static control-flow evidence proves there is no export-side bound check on that header while the parser bounds every physical record. Add header-record checks and include terminator bytes in the overall bound. No large allocation was executed for this read-only audit.

### P2 — Source validation reports byte zero for errors in later instance records

Same file, lines 178–179, 226, 298 and 309.

The public error documents an original source byte offset, but graph-validation failures use literal zero. A two-record input with a duplicate second id, a nested missing reference in the second record, or an overflow real in that record reports offset zero. Decimal validation also loses the record origin before returning. This defeats source navigation for exactly the malformed large-file cases this boundary is designed to diagnose. Retain instance origins during import or validate each record with its original offset and carry the relevant origin into final reference checks. Export diagnostics can separately represent graph positions rather than call them original byte offsets.

Repro contract: `minimal("#1=IFCA();#1=IFCB();")` and `minimal("#1=IFCA();#2=IFCB((#3));")` from native.rs both take the literal-zero refusal branches. This is a static path analysis, not a claimed executed Rust result.

## General Capability and Discovery Review

No additional actionable registration defect found in this pass. Inspected:

- Plugin identity `🪪️identity/🦀️.rs`, lines 256–341 and owner admission near 633: byte framing uses Rust string byte length; decoding checks nonzero canonical decimal counts, UTF-8 slice boundaries, exact coordinate reconstruction and trailing content; foreign composer claims require owned sources and reject mixed foreign ownership.
- Declaration builder `📜️declaration/🏗️ready-declaration-builder/🦀️.rs` and capability collection `📜️declaration/🧮️codec-spec-and-capability-sources/🦀️.rs`: both use composer_identity_claims.
- Framework IO `📇️composer-entries/🦀️.rs`: deterministic BTreeMap/BTreeSet union retains all readers for a target and removes duplicate discovery rows.
- IFC root declaration and registry: IFC4x3 importer/exporter entries are contributed normally. Export receives exact source-to-text route authority rather than text dialect ownership.
- Three valid/six invalid route fixture vectors: framing counts are consistent by inspection. No additional Ajv execution was necessary; the prior report clearly limits that evidence to fixture/schema checking.

The central dialect coordinate parser deliberately accepts arbitrary nonempty standard/subset strings. The new route validation inherits that vocabulary and adds whitespace rejection; exact reconstruction does not independently impose a tighter standard/subset identifier grammar. No tighter grammar is asserted here without an authoritative framework requirement.

## Rust API and Source Review

Static inspection found the new source signatures consistent with the actual ArtifactComposition, ArtifactComposer, ArtifactSerializer, ArtifactDeserializer and IoRunControl APIs. Plugin io is a reexport of kernel IO; Confidence/Composition/AnalyzeSource imports match that facade. The IfcSnapshot pack implementation is mounted in the explicitly included IFC4 IO crate and delegates to graph-native SQLite encoding; the package has that dependency. Packed TXT import decodes TxtSnapshot before reading its source, native graph import validates schema and graph, and serializer output is a packed TxtSnapshot containing natural STEP.

Source record scanning tracks apostrophe escaping, quoted binary, comments and nested parentheses; final framing and no trailing records are enforced. Graph checks cover unique positive ids and references recursively through complex constituents and typed/aggregate values. Shared writer escapes control characters, so its string content cannot inject a raw newline DATA delimiter into the writer extraction logic.

## Limits and Existing Disclosed Gaps

No production edits, build-system edits, compiler enforcement bypass, git mutation or worktree was performed. Actual Rust compilation/tests remain blocked by authentic compiler ingress as documented in native-io-implementation.md; therefore type compatibility is static evidence only. Independent Rust execution, erased-route execution and IfcOpenShell validation of generated Rust output remain unverified.

Registered composition is synchronous, has no retained caller IO control, and uses an always-accept callback. The direct `_io` bridges preserve cancellation checks but discard detailed phase progress and do not debit allocations through the original receiving ledger. Those limitations are already candidly disclosed and are not newly discovered defects in this pass.
