# Original Native API Audit

Read-only source audit of the current native export retained owner, Drawing custody route, metadata cursor, borrowed render lookup, and shared output queue hook. No confirmed actionable type/signature, lifetime, queue custody, or paid denial/cancellation blocker was found in the inspected source. This is not compilation evidence: the required repo command is unavailable, and no compiler or alternative command was invoked.

## Checked Evidence

- `editor/📤️export-tool-job/🧵️retained/🦀️.rs`: lines 39 and 44–46 transfer the supplied queue once into retained owners, then core, then CompleteDownload. Subsequent shared hook calls reject a replacement queue. Lines 51–64 close the core before source owners and the metadata cursor.
- `editor/📤️export-tool-job/🧵️retained/🧩️drawing.rs`: lines 23–27 agree with actual vector read registration signatures: original read receives the retained parent, ChildContentView, member key, operation, lookup and grant; qualification receives the real member read; source admission receives that read and its stamp. Lines 34–36 agree with the custody provider and routed Drawing read interface. Paid child success and failure receipts flow through the shared paid helper.
- Native window configuration retrieval in retained work line 17 returns each owner's associated State. Both Blueprint and Preview owners declare State = LayoutWindowConfig, so these match arms have the same actual type.
- Framework child vector `read_original_render` borrows the ArtifactView for the returned read lifetime and calls OriginalChildMemberOperation::from_view. The latter borrows render_operation_ref rather than copying a context; ArtifactView supplies that borrowed method. The return read borrows actual operation/revision and child entry storage under the view borrow.
- Shared retained-command-job lines 558–562 call the output hook before work_demands/step and pass actual snapshot_owner and original snapshot_read into work inputs. Hook progress ends that turn before normal work.
- The metadata cursor checks original source identity/revision between advances, funds backing capacity before byte copying, validates UTF8 before String transfer, and retains allocated fields for funded close.

## Limits

The source was actively edited concurrently. This audit examined the observed snapshot and does not establish a successful Rust build, integration execution, or third-party oracle result. Page-limit derivation was intentionally excluded because core Plan already enforces MAX_LAYOUT_EXPORT_PAGES and the root task is replacing the duplicated bound with the central constant.
