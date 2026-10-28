# Persistent Boundary Asset Review

Read-only review of browser/🐍️.py, browser/💾️handoff/🧬️schema.json, and registered persisted fixture. No production edit or duplicate render.

## Concrete Finding

The first producer metadata includes its output PNG path (render.mjs constructs metadata with output: frame.output). _boundary_asset stores the complete metadata and returns it on reuse. render_batch replaces the current metadata entry with that returned object. Consequently a consumer entry metadata.output identifies the first producer PNG, while result.outputs and the actual consumer PNG identify the current request. That producer path may already have been cleaned. Preserve shared geometric metadata but bind output to the current frame output after reuse. The four existing tests do not exercise public render_batch metadata routing. Root notified.

## Reviewed Guarantees

The key binds catalog, explicit language, viewport, actual UI dependency contents, hierarchy (including camera), and cursor. Canonical eligibility requires explicit flag, no menu/media/pressed cursor, ancestor opened path and no revealed path. Content checksum and PNG format/dimensions are verified before reuse; malformed records fail closed. Blob and record publication use complete temporary files plus exclusive same-directory hard links, making the record first-writer choice atomic across processes. Concurrent losing producers read the winning record. Resolved root must remain inside the workspace, and longest record path is checked before publication. Cancellation is checked before native capture and before each canonical asset operation; native browser receives the session.

The mechanism guarantees equality to one actual UI capture, rather than claiming independently painted frames are deterministic. Fresh public producer/consumer native geometry acceptance remains owned by verifier.

Resolution checkpoint: root captured the metadata.output regression RED, corrected current consumer output routing while retaining winning DOM geometry, and reports 4 tests GREEN in 0.198 seconds. Native Windows four-producer same-directory hard-link race passed. Other platform execution was not performed in this audit.
