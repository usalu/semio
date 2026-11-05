# Lossless Native Snapshot Owner Interface

Status: staged production changes; shared source is not coherent and no new native compiler has run.

The canonical API returns RetainedWorkOutcome and owns NativeSnapshotFailure. That failure retains the complete OriginalReceivingPrimary, an independent receipt refusal, an independent context refusal, and the original physical progress. The failure is inline and adds no allocation. Earlier preallocated-wrapper design notes are superseded.

## Implemented source

- IO control owner receive, receive_nested, drive_cursor and scopes return rich outcomes. Callbacks retain output in the original receiving frame before any later receipt refusal.
- Nested scopes borrow the existing cumulative wallet. They do not repost already posted native child receipts to the original native recipient.
- NativeSnapshotWork/native_snapshot_turn collect actual delegated work before observing success or failure. performed() is only called explicitly for a selected admitted action.
- ArtifactPackReceiving now requires a rich outcome.
- DSL synchronous field/record/variant receiving and derives now return rich outcomes. ReceivingWork/receiving_turn rename the canonical lower IO work collector.
- OriginalPackWriting and its BorrowedPackDocumentCursor expose actual last_turn_work for selected admitted events, denial/terminal zero, and partial failure preservation.
- IFC canonical writer and its ordinal index builder return rich outcomes and retain all child work.
- TXT raw Pack/text receiving is staged with explicit per-admitted-header, scan, copy, row move and typed transfer work.

## Validation so far

- New four-case generic owner conservation neutral law: AJV + independent SQLite, 1 test / 13 assertions passed before owner edits.
- Native conservation test authored in IO control tests/drive: genuine original caller, decoder, strict requested allocator, work-only callback and all three receiving causes, funded failure and cursor retirement. Not executed.
- IO control owner, shared helper, owning failure and native test: AST 4/4 passed.
- DSL receiving, derive, Pack cursor, OriginalPackWriting, IFC pack index and IFC writer: AST 6/6 passed.
- AST validates syntax only. No semantic/native pass claimed.

## Remaining coordinated ports

Root owns History/Fold and FEM2/FEM3. Runtime owns store receiving/pair/window and IFC canonical receiving/tests. Light owns the existing IO control snapshot test root.

Editor implementation retains Binary/TXT/GLTF and shared writers, outer IoError, and the remaining IO owner consumer surface. Core remaining consumers include DSL tests; sqlite-snapshot original receiving; OS snapshot codec capability; presence JSON; MCP JSON/guest import; SPR archive; IO mechanism. Existing original relational snapshot traits use Result<ValueError> and require an explicit lossless boundary decision before all domain overrides can compile. A Result-only compatibility wrapper or single-cause conversion is not acceptable.

Native sequential driver 86185/PID1905011 has completed. No editor compiler/driver is active. Future jobs are held behind actual metadata-routing-alias-native plus a fresh successful core gate. Existing failed results remain persisted; no artificial coherent success marker has been inserted.
