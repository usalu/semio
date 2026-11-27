# PDF workspace lib failures after Slice 15 (2026-11-26)

Crate of the tests: `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests` (mirror modules). Gated runs: `run-pkg-retry.sh`, logs `🗑️generated/l2-hist.log`, `l3-clock.log`.

## Fixed and verified (l2-hist: 22 passed)

- **History acceptance law (`history_edits_end_to_end`, `conflict_history_edits_end_to_end`, pdf14, 14a, 14x, 17, 17a, 17e, ...).** Root cause: `ArtifactApp::persisted_operation_wire()` replaces the owners' wire of every document Store. PDF mutations registered `artifact_persisted_binary_operation!` (no Pack-value comparison), so replacement comparison took the paged-encoder path ("replacement input is not the admitted canonical operation wire"), and the editor's own conflict target and original edit message were overwritten by the whole-document target. The editors already installed cold-replay owners; that was not the gap.
  Fix: new `artifact_persisted_pack_value_operation!` (publication crate, re-exported by the store crate); PdfE/H/X/UA/Vt use it; base 1.7 and 1.4 `PdfMutation` implement `ArtifactPersistedOperation` by hand (Pack-value receiving, conflict target, 1.7 also the edit message); the default `persisted_operation_wire()` keeps a vocabulary's own conflict target and falls back to the whole document only when none is set.
- **`controlled_annotations_...`**: annotations with `/AP` are lifted by design through the reference builder (`cold_appearance`). Test: `appearance` mode must complete and equal the cold lift of the same graph; new `popup` mode keeps the page-barrier deferral.
- **`owned_pdf_snapshot_borrowed_projection_...`**: the framework refuses to materialise paged octets in the field projection (`UnsupportedOwner`, retained export route encodes them). Test now asserts the lent original octets and that refusal; byte equality stays in `genuine_pdf_paged_snapshot_export_matches_reference_pack_and_digest`.

## Fixed and verified later (l7, l8, l10, l14)

- **History acceptance law, rest of the family**: pdf17h, pdf17ua, pdf17x (snapshot-value crate) and pdf17vt (viewer crate) pass with the fix above.
- **`controlled_shapes_...`** passes. Two causes: (1) fonts and images register through the reference builder, which reads by value, so the `get` refusal now applies to annotation dictionaries only (unless the graph has `/AP`); (2) the fixture objects came from `parse_indirect_at`, which keeps `/Length` in stream dictionaries, while the importer drops it and the controlled lift only accepts an empty stream dictionary for content roles ("defers at Roles"). The test now strips `/Length`, like the importer.
- **`paged_natural_decode_*` (3) pass.** Two independent framework root causes, both found with temporary probes (removed):
  1. "original clock Source or action changed": the recorded clock decision requires its fuel never to rise within a decision, but it was given `cx.fuel_remaining()`, which is `min(turn fuel, Source ledger)` and the ledger rises when return claims release their holds (a 47-unit rise observed). `poll_recorded_clock_decision`, `take_recorded_clock_decision` and `recorded_clock_allows` (`job/⏱️context/🎟️source/🦀️.rs`) now use `original_turn_fuel_remaining()`, the turn's own fuel that never rises; the Source ledger's exhaustion is still enforced by `should_yield()`.
  2. "job child receipt exceeds original remaining retained grant" (9 MiB image): the Source ledger now refuses work beyond its fuel (`raw_remaining` errors when `work > fuel`, so the remaining grant collapses to zero). `OriginalWorkFixture::hosted(lifetime_fuel)` built its Source with the fixed 1<<20 fuel and ignored the lifetime stock it promises; it now builds the Source with `lifetime_fuel` (`🧪️testing/🎟️caller/👷️work/🦀️.rs`).

## Open: fresh-app footprint

6,823,571 B against 6,815,744 B; 6,127,047 B measured on 2026-11-25. Census (temporary probes, removed): the app is born in `VcsArtifactApp::with_registry_boxed` (6.84 MB); definition 0.36 MB; the PDF document envelope 0.55 MB; envelopes of config, draft and interaction plus the document store 1.98 MB in total; the three other stores 1.59 MB (about 0.53 MB each); tool registries 0.17 MB; instance fields about 1.0 MB (tools 262 KB, stores 195 KB, time_travel 193 KB, pending 190 KB, tool_machines 77 KB); about 2.1 MB between these marks is unattributed. The growth of about 696 KB is not attributed: no single eager structure stood out, a per-store fixed cost (about 174 KB times four stores) is the leading hypothesis and needs a probe inside `ArtifactStore::construct`, which had to be reverted because a probe there used a dependency the engine crate does not link (it broke other sessions' builds). The limit is left unchanged and the test still fails. Next step: measure inside the store engine with a probe that only uses plain atomics (a counting allocator in the test), not a crate dependency.

## Stopped

All temporary probes are removed (job recorded clock, job context, artifact-app-laws, instance ingress, store engine). Disk guard: 12 GB.
