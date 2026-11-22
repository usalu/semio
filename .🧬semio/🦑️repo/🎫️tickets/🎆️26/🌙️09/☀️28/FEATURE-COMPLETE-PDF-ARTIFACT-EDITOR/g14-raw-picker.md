# G14 Raw Picker: Source Audit and Remaining Mount

## What the source shows (re-read this session)

The task premise (picker asks for an inline data URL capped at 24,546 bytes) describes `admission.rs::picker()`, which no caller used: the registered native work is `PngPickerWork` (`image-authoring/📂️picker-work.rs`), wired by `📥️local-image::native_work`. It already issues the raw lane: `RawMediaIntent::issue` under a funded Original turn, `issue_args` projects `RequestFileOpen.args.originalMedia`, `read_as: None`, `max_file_bytes: PNG_GRAPH_MAXIMUM_BYTES`. The retired inline `picker()` was deleted (greenfield, no legacy); `place_sampled_image` and `IMPORT` stay as the small inline lane the selector names `Inline`.

Already mounted end to end for PNG, found in source (not staged): WIT `original-media-control-event` (begin, complete, cancel) in `events.wit`, kernel `Event::OriginalMediaControl`, host conversion and retained-turn codec, reactor lift/bridge/dispatch/`drive_raw_media` (fixed pages through `raw_media.page`, picker receipts), renderer `QueuedOriginalMedia`, `PickerPngWork` registered via `build_original_picker_job`, `PngFormatWork` driving `RawPngWork<RawMediaTarget, RAW_MEDIA_BYTES>` into `PngGraphEdit` and paged mutation rows. The 521 by 257 PNG (133,897 pixels) is inside the paged PNG limits, so PNG needs no new event: the typed folder-archive event is a different lane (whole-archive pack) and is not the model for per-file picker pages.

## Not mounted (blocking the JPEG half of the law)

- Paged DCT route: `select_picked_raster_route` returns `PagedDct` but nothing consumes it. The picker's `accept` is `image/png,.png`; widening it before a DCT owner exists would let the host stream JPEGs the format owner refuses. A JPEG owner needs the durable install event (`paged-dct-install-pack.md`: registered native event, revision-checked store transaction with `PendingDctPlacement`, digest-indexed lookup, history lease and paid retirement). That store-side event registration is absent from the repository.
- Law test placing the 521 by 257 PNG and a 2.1 MB JPEG and comparing with MuPDF/Pillow: the Pillow/MuPDF oracles exist in the ticket (`pdf-raw-png-history-oracle.py`, `paged-jpeg-oracle.py`, `g14-image-route-oracle.py`), but the in-repo funded harness that drives `PickerPngWork` through Begin, Page and Complete and exports the PDF is the staged `raw-png-handoff-test.rs`/`image-authoring-browser.ts` chain, not in the repository, and the workspace does not build the dependent crates reliably (other agents' breaks; see `g14-route.log`).

## Verified this session

- Route law test `picked_raster_routes_match_the_pillow_measured_law` (Pillow measured 402,534-byte PNG routes `PagedPng`; 2,101,918-byte JPEG routes `PagedDct`; oversize refuses). It runs inside the `g5-snapshot7` selection (`pdf-snapshot-value-tests`, log `🗑️generated/g5-snapshot7.log`); no result had been produced when this note was written, so it is unverified. A first attempt with a wrong cargo selector (`g14/route2`) was stopped.

## Next step (ordered)

1. Land the DCT install event and store transaction, then add `PickerJpegWork` beside `PickerPngWork` and widen `accept` to `image/png,image/jpeg`, calling `select_picked_raster_route` once the host reports format and extent in `Begin`.
2. Port the staged raw-png handoff harness into the editor's `🧪️tests` and add the two-image MuPDF/Pillow law.

## Second request: DCT install event, store transaction, harness (not landed)

Audit of what adoption needs, none of which exists in the repository (all of it is ticket-staged overlays compiled only against cached rmeta in isolated probes):

- Leaf codecs: `paged-jpg-baseline.rs`, `paged-pdf-dct-{view,lookup,resource,writer,placement}.rs`, `paged-dct-install-pack.rs` (inferences crate overlay plus a new `semio-framework-pack-source` edge).
- Schema: draft PDF mutation tag 61 (`paged-dct-placement.schema.json`) in the base aggregate, which declares tags 1 to 60; every mirror, fixture and language binding of the schema moves with it.
- Store: `ArtifactStore::resolve_operation_wire` override (`operation_wire_source`) with the sealed typed operation, plus the canonical compare-and-install that admits the install owner and the two prepared rows against the captured revision, and a digest-indexed lease for undo, redo and reopen.
- Editor: a `PickerJpegWork`/format owner beside `PngFormatWork`, `Begin` carrying format and extent so the selector can run, then `accept` widened to JPEG.
- Law: port `raw-png-handoff-test.rs` into the editor test crate and add the 521 by 257 PNG and 2.1 MB JPEG placement law with MuPDF/Pillow.

Decision: these were not landed. Each layer changes the shared mutation schema or store wire and cannot be compiled together here (builds fail in other agents' crates and run 15 minutes or more per attempt); a partial landing would break the live-replicated worktree for every machine. Land them as one compiled change set in the order above.
