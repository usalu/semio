# Publication and Durable Export Review — 2026-11-05

Read-only scoped review; no build/test execution or production edits.

## Concrete QC Export Contract Drift

Current `✏️editor/🎮️commands/🧾️export-qc-report/🦀️.rs:14` returns an App Fault `remodeling.qc-report.missing` when results.qc is absent, with text Run the quality check before exporting its report. With QC present, line 16 emits DownloadMediaExport with filename `remodeling-qc-report.json`, MIME `application/json`, and pretty JSON data.

The current manifest action description at `✏️editor/📜️manifest/🦀️.rs:276` instead promises `remodeling-qc-report.ops` and says nothing happens without a report, in both English and German. This is observable UI/documentation drift against the actual handler. The existing editor integration law `export_qc_report_is_a_no_op_without_a_report` (import-frames test source) asserts only no document edit and no requested effects. It does not assert absence of faults, so it cannot distinguish silent no-op from this App Fault. No positive report-download assertion was found in the scoped export references.

Recommendation to owning lane: keep JSON export semantics and align visible manifest description with actual JSON/error behavior, or explicitly choose intended no-op and implement it. Add a neutral expected filename/MIME/payload row and assertions on actual effect plus missing-report fault/no-op semantics. The exact contract decision belongs with the owner; this review changed no production files. Root was notified immediately. The browser checklist was corrected to actual `.json` output and explicit missing-report behavior.

## Publication and Cancellation Source Facts

Reconstruction product preparation publishes append-content leaves before the final CommitReconstruction operation, then queues checkpoint and complete. RunJob advance_products marks ResultReady and schedules terminal complete; actual job completion uses admit_complete. These bytes are delivered through retained preview/checkpoint publication; this module does not itself mutate the persisted host document on complete. Document panel text correctly distinguishes Complete (ready to finalize) from Finalized (one undoable committed change). Actual host finalization/rollback remains outside this Remodel-only review.

RunJob and RevalidateJob check closing/cancelled context before advancing their outbox. The original-caller cancellation source law checks no further outcomes, stable decisions and empty retired job ownership. This supports the intended local ordering but does not establish a mounted browser abort suppresses every host delivery race. Browser acceptance must still observe no committed result after abort and no late result reappearance.

## Cold Durable Mesh and Images

`export_media("mesh:out")` resolves the current mesh through resolve_bounded_remodeling_mesh over doc.snapshot.durable_artifacts; missing composed content returns a payload error, then GLB export occurs through the MeshExporter interface. Model world mesh rendering uses the same durable resolver, not an ephemeral mesh cache. Frames rendering resolves remodeling_asset(scene, frame.asset_id), preserving MIME/dataURL/dimensions. Asset chunk source obtains the child handle and snapshot-owned durable image content, validates each durable chunk and cumulative byte envelope, and reconstructs UI/export ImageAsset only at the facade. Missing handles/content return None, so visible blank content alone is insufficient to identify whether a reload lost durable state.

These current resolvers support genuine reopen acceptance and avoid the earlier test cache dependency. Native ownership/reopen laws and codec roundtrips corroborate source intent; this review does not claim their execution. No concrete cold-resolution mismatch was found in these paths. A full persistence reopen and actual downloadable GLB/JSON remain necessary browser observations.

Frames source explicitly documents that runtime keypoint/match/track overlays are not distilled into durable scene state. Do not count a nonempty image canvas as evidence those algorithm diagnostics are visible; verify the actual provided progress/trace surface separately.

## Owner Follow-up and Atomic Commit Review

Root has taken ownership of the QC correction: selected established successful no-op behavior when report is absent, JSON filename description alignment, portable positive/negative fixture and regression checks. This lane made no QC source edits. The discrepancy above describes the reviewed state before that correction; recheck its final tests and regenerated descriptor rather than treating it as an unresolved source issue after root completion.

Reviewed CommitReconstruction diff afterward. Sparse and mesh content handles must resolve as complete durable content in the base document; every bound image content id must likewise be complete and image-kind, or the diff returns mutation.target-mismatch before returning any result/binding diff. Valid asset bindings and result lane changes are returned together in one RemodelingDiff; equal payloads become mutation.no-op warnings. This coheres with earlier append-content preparation and final committed result binding. No additional concrete atomic/cold-persistence source mismatch was established. It remains necessary for the host to finalize the preview edit atomically; this Remodel-owned source review does not substitute for that live observation.

## Read-only Review of Root QC Repair

Read exact current handler, new export-qc-report native unit source, neutral cases JSON, external editor module registration and appended schema TypeScript suite. Missing QC now returns Ok(Emit::default()), matching chosen successful no-op. Existing serde error path remains explicit. Download JSON filename/MIME/pretty serialization agree with positive fixture.

The portable native test invokes the actual public handler and requires success for both cases, so missing-report Err can no longer hide behind only no-effects assertions. It checks all four mutation lanes are empty, unchanged scene, exact effect count, actual DownloadMediaExport variant, filename/MIME/no encoding and parsed downloaded JSON equality. Its relative include_str path correctly reaches tests/🧫️cases. The external editor entry mounts the new module alongside existing command tests; public owning imports and ArtifactView/HistoryView construction match established nearby test patterns. No concrete type or serialization error was found by source review; compilation remains authoritative.

The TypeScript law uses the actual snapshot decoder/encoder and compares canonical document QC with explicit IEEE-word fixture values, independently validates report shape using Ajv and checks both locale descriptions. Mean track/ratio binary32 words, reprojection/GCP binary64 words are separated correctly. This tests document canonical storage independently of plain report JSON; it does not conflate export and document representation. The new native test separately checks plain output JSON. The original no-op integration test remains weaker by itself but the direct portable-handler test now establishes successful behavior.

Root reports its separate executable QC checker passed; this review did not rerun it or native/JS builds. No source changes were made. Fresh generated component/descriptor metadata remains pending the actual publication gate, as root noted; neither binary nor JSON descriptor was handpatched in this review.
