# Grid2d Preview Revision Binding

## Behavior

Grid2d previously accepted a completed assignment when coordinates and tile IDs fit the current document. A seed, rule or pin edit could retain all IDs and extents while making that assignment stale.

The completed cache now carries the original fill job's canonical source revision with its inference commit. Both completed cache and live tool payload require the current mounted render revision. An unbound render shows unsolved outlines. Shape and tile validation remain additional checks. No document mutation, legacy cache decoder or synchronous render solve was introduced.

The existing solveJson window config stores the new typed cache envelope. The fill completion owns its commit directly and stamps identity.base_revision; it does not substitute the revision current at delivery. Editor rendering reads render_operation_ref, not the command operation accessor.

## Tests and Schema

Authored a shared language-neutral six-case revision corpus and schema under plugin tests/preview-revisions before implementation. Grid2d native tests consume all six cache cases and all representable live identity cases. Cases cover matching, changed same-extent revision, missing render authority, missing source and wrong source lengths.

A mounted adapter regression uses actual ArtifactView::with_render_context, ToolRunView and ArtifactEditor::render. It requires fresh live content to differ from empty and stale content to equal empty, while command authority is absent.

The cache JSON schema references the canonical inference schema. Independent Ajv validation registered that schema, validated the revision corpus, and checked malformed nested commits; results are in run-r2-grid2d-audit.md. Pinned Rust parser-only checks pass for five changed native files. Native execution is still pending the existing consolidated run; no typechecking or runtime pass is claimed.

## Files

Grid2d editor root, fill module, preview module, preview unit tests, editor unit tests, new fill cache schema; shared preview revision schema/data; this report.

## Completion Envelope and Publication

The64×64 portable control fixture encodes to60306 cache bytes and68524 escaped solveJson argument bytes; the former65536-byte dispatch rejected it. Raw and output bounds are now1MiB and the proof uses the same constants as factory admission. The native regression compares our cache/argument encoding with serde_json and the independent fixed byte counts.

A second portable capacity corpus supplies100 and300 cells with legal1024-byte tile IDs. Completion stages its final tick before dispatching the cache effect; only a successful complete tick can set committed and enqueue the cache. Oversized ticks publish a named grid2d-fill-payload-capacity fault and no completion effect. The native test covers both sides; parser-only checks pass, native execution remains pending.

## Late Completion Publication Fence

Rendering fences do not prevent a late old completion callback from replacing a newer cache. CommitFill now carries the original32-byte source revision as separate small command metadata. Dispatch compares it with current command authority before creating any config mutation; stale, malformed and unbound callbacks emit nothing. This avoids decoding the full cached assignment merely to admit completion. The cache retains its own revision fence at render time.

The shared six-case corpus now also exercises actual dispatch publication. Completion arguments have an explicit JSON schema. Independent Node encoding and existing Ajv confirm the updated64×64 envelope68607bytes; cache remains60306bytes. Native codec tests compare the same arguments with serde_json. Pinned parser checks pass for threechangedRustfiles. Native execution still requires the pending repaired configuration ingress.

## Producer-to-Publication Regression

Added an actual fill finish→ToolRunJobPort effect→typed action bridge→command-authority publication law. It consumes the shared revision cases with representable32-byte run identities, verifies the original revision survives both command metadata and cached envelope, compares solved assignments, and requires stale/unbound delivery to emit no config/document row. This closes a coverage gap that manually constructed completion commands alone could not detect. Pinned Rust parse check passes; the actual grid2d native gate remains pending the current bitmap prerequisite.

## Completed Preview Authority

A public Complete run remains nonterminal until finalization. Grid2d previously overlaid every trace entry over its finished commit, so a historical/discarded choice could replace the solved tile. Runtime now overlays trace only while payload.done is false. Four schema-first portable completion cases under shared WFC tests cover partial/completed×observation/discard, preserving partial behavior while completed frames keep the authoritative assignment. The native consumer exercises actual ToolRunView Running/Complete and paint_commit adapters. Existing Ajv validates the corpus; pinned Rust parse checks pass. Actual grid2d native/runtime tests remain required. Other affected editors are assigned to bitmap_finish_r2; grid3d already guards final trace.
