# Original Native STEP Import Review Point

The source implementation and tests are ready for independent review. Rust execution and typechecking remain unavailable: the focused IFC4x3 cargo check still refuses before rustc with the authentic original compiler-ingress requirement. No build or enforcement bypass was attempted. The ticket remains active and generated evidence is retained.

## Actual Interfaces And Route

Public `receive_ifc4x3_step_txt(&TxtSnapshot, &mut NativeSnapshotDecodeOwner) -> Result<IfcSnapshot, NativeStepReadError>` receives the existing shared owned graph. The error preserves its authentic `ValueError` cause and original UTF-8 byte offset. Invalid source errors also carry their byte location in a bounded, originally funded cause message when the original remaining wallet admits that diagnostic work; cancellation/authority causes are preserved without inventing diagnostic authority.

New `TxtIntoIfc4x3Native` implements the real native `Deserializer<IfcSnapshot>`. It accepts the actual binary UTF-8 artifact Pack, calls the TXT owner's existing `ArtifactPackReceiving`, and retains that snapshot in the original paired run. It passes the same original decoder/wallet into STEP receiving. Its ninth normal IoEntry uses the already implemented IFC `original_pack::deserializer` wrapper and `IfcSnapshot::encode_exchange_pack` to publish the final actual IFC Pack. Root's eighth TXT export row remains present. Normal root registration and central capability claims are unchanged.

The actual route does not call `to_body`, the cold Part21 parser, reconstructed per-record documents, `from_part21_document`, default native controls or graph clones. Existing public reference source helpers and pure ComposerEntry functions remain separate reference paths; their old callback/cold behavior is not claimed as original-owned import proof.

## Receiving And Lexical Contract

The source cursor visits borrowed TXT line bytes and the actual declared LF/CRLF endings. It carries original byte position directly and does not join the text. Ignored C0/DEL bytes still count toward source offsets. Every source visit is bounded and debits original work/copy authority before mutation; token payload copying, collection allocation, numeric conversion and physical retirement use the same original body wallet and decoder. The original decoder advances completed work after accepted effects.

A lexical census first counts list/entity/reference extents. A second scan constructs one list-size catalog; the graph pass consumes it to allocate exact argument/component vectors. Payload decoding replays only the relevant original token into admitted retained storage. The catalog stores counts rather than tokens or text. There is no origin-token/value tree mirror. Identity/reference offsets are retained only beside their necessary funded indexes; headers use three fixed offsets. In-place heapsort and binary reference search keep population validation bounded by O(N log N), with original cancellation checks.

All allocated partial text, lists, pending values, headers, primary/complex entities, graph and indexes remain in the actual retained receiving state. Owned values move between these slots only after admission. The effect interface returns only Copy values, preventing owned results from escaping a slot before its post-work cancellation check. Numeric scratch backing is explicitly released under the original release currency. On cancellation/refusal the same original recipient owns cleanup. The operation is a synchronous original receiving operation with bounded, cooperative inner effects; it publishes no partial snapshot. It does not claim a separate resumable parser continuation across successful job turns.

The lexer implements uppercase STEP names, exact positive u64 identities/references, signed i64 integers, finite binary64 real conversion including signed zero and subnormal syntax, mandatory decimal mantissas for exponent reals, canonical binary high-padding validation and lowercase-hex normalization, doubled apostrophe/slash, N/F print directives, all nine owned ISO 8859 pages, XHH/X2/X4 escapes, paired UTF-16 surrogates and raw UTF-8 scalars. Print directives inside binary and Unicode hex runs are handled. Original mandatory HEADER ordering and exact ADD2 FILE_SCHEMA are required. Complex source components map to the shared graph convention: first primary name/args, subsequent `complex` components.

This imports one declared ISO-10303-21 HEADER/DATA exchange structure. It does not add optional ANCHOR/REFERENCE/SIGNATURE sections or multiple named DATA sections. EXPRESS population/MVD validation remains the separate schema owner; parsing the pinned basin fixture with a known Identification domain error does not certify that population.

## Explicit Limits

Source body: 256 MiB; source native Pack: body ceiling plus 64 bytes of envelope admission; record: 8 MiB; graph value nesting: 64 plus enclosing grammar frames; entity count: 1,000,000; list count: 2,000,000; lexical token count: 8,000,000. All remain subordinate to the actual original five-axis grant and cumulative native ownership ceiling.

The normalized numeric token bound is deliberately **65,536 bytes**. Standard-library exact numeric conversion is admitted as one bounded token span, with cancellation before/after conversion; the source scans themselves remain granular. This differs from TypeScript's **4096 default / 16384 maximum configurable numeric code units**. No identical limit policy is claimed. The native bound remains below the pre-existing 8 MiB record and 256 MiB source ceilings. Non-numeric names/strings/binary retain the record bound and original physical grant.

## Tests And Current Evidence

Test-first `native-step-read.feature` covers actual native TXT → IFC Pack, lexical parity, original byte locations and all denial currencies. Written Rust tests cover exact complex/nested/binary/i64/u64/signed-zero values; original multibyte/CRLF source offsets; every zero currency; all 855 owned alphabet positions and seven neutral lexical cases; all 46 immutable official sources against reference graphs and retained IfcOpenShell entity-count witnesses; actual heap/physical receipts and completed-counter cancellation prefixes with same-ledger/recipient drain; and real registered native TXT → IFC Pack execution. These Rust tests have not run or typechecked.

The registered-route test writes `native-step-import.ifc` only from the graph decoded from its actual returned IFC Pack, and only when `SEMIO_IFC_NATIVE_ORACLE_OUTPUT` is set during genuine native execution. The existing test-only `ifc-oracle.py` now checks that file's 35 entities, mm project unit scale, twelve triangles, metre bounds and validation. The appended `native-oracle.test.ts` case independently reads its actual output and IfcOpenShell report. Existing oracle rows were preserved. No new native file/report has been generated or substituted from a reference implementation.

Focused Bun oracle gate: **0 pass, 6 skip, 0 fail**, because genuine native output is absent. Log: `🗑️generated/native-step-reading-oracle-test.txt`. The five new Rust source leaves parse through rustfmt, with exact path lengths below 256; `🗑️generated/native-step-reading-syntax.json` explicitly records `nativeExecuted:false`, `typechecked:false`. Current cargo refusal: `🗑️generated/native-step-reading-cargo.txt`.

Independent source lexical/corpus evidence remains the existing pinned Unicode/ICU and IfcOpenShell reports, including the documented 48 page-F discrepancies in original shifted encodings. The new Rust tests use those exact normative cases. This does not establish that this new native implementation already produced matching runtime results.

## Touched Files

New compact IFC4x3 IO leaves: `native-step-read.rs`, `native-step-lex.rs`, `native-step-parse.rs`, `native-step-import.rs`, `native-step-read-tests.rs`, `native-step-read.feature`. `step.rs` only mounts/reexports the new receiver/importer alongside existing export modules. `native_exchange.rs` only adds the normal TXT import row beside the existing export. `🧪️tests/native-oracle.test.ts` appends one genuine-output-gated oracle; the retained ticket `ifc-oracle.py` adds its assertions. Plan/report and logs stay in this ticket. No native emitter/writer/export, EXPRESS/editor, shared graph model, AGENTS, git/worktrees, build infrastructure or generated input cleanup was changed.
