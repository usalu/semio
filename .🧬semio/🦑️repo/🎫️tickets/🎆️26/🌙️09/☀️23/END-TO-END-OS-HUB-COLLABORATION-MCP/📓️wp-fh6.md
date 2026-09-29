# WP-FH6 — Fault Class Review, Family BD (stdio, fem, draw)

Slice: FH6 (session 15, helper of S20; spec `📓️fault-localization-api.md` §7). Work list
`.🧬semio/🌐hub/s14-s20-sets/class/family-BD.json` (515 codes, 73 `rule: default`). I edited only the `class` field: one entry per line, and
owner, code, rule and en stay byte-identical (checked by diff: 143 lines changed, all of them `class`). I did not edit any source file
or overlay. Tools: `wp-fh6/fh6-show.py` prints an entry with its owner-scoped census raise sites. `wp-fh6/fh6-apply.py` is the reviewed
map: it rewrites `class` and prints the counts.

## Session 15

### Counts

| class | before | after |
|---|---|---|
| input-invalid | 222 | 263 |
| precondition-failed | 228 | 178 |
| internal | 29 | 60 |
| conflict | 35 | 13 |
| unavailable | 1 | 1 |

143 changed: precondition-failed→input-invalid 60, precondition-failed→internal 25, input-invalid→precondition-failed 24,
conflict→input-invalid 12, conflict→precondition-failed 11, input-invalid→internal 8, precondition-failed→conflict 1,
internal→input-invalid 1, internal→precondition-failed 1. All 73 `default` entries were judged against their raise sites.

### Judgement calls (rules applied, by the check at the raise site)

- **Unknown name in the arguments → input-invalid.** Covers `*.field` (fem analysis/results/playback),
  `*.editor.unhandled-action` (fem, draw, stdio: the `match action` fallthrough) and `stdio.wav.audio-action`. §7 says "unknown
  action/kind".
- **Empty or missing argument → input-invalid.** Covers every `stdio.pdf.page.*-required` ("…needs a name; enter one", checks
  `is_empty()` on the argument), `drawing.fill/path.edit-missing`, `fem3d.camera.orbit-required`, `drawing-viewer-camera-required`,
  `stdio.docx.set-page.address-*`/`node-path*` (malformed address argument) and `stdio.zip.name-required`. It also covers
  `stdio.png.pixel-region.empty`, `bounds-overflow`, `out-of-bounds` and `patch-count-overflow`, where the region arguments are wrong.
- **Window not open → precondition-failed, not conflict.** `*.window-stale` and `drawing-canvas-window-stale` check "the window
  id is not among `view.window_instances`". No revision is involved, and §7 lists "window not open" as a precondition.
- **Document lookup with no revision guard → precondition-failed** (element gone). Covers `*.index-out-of-range` for
  mp4/png/tiff/wav, `stdio.epw.row-range`, pptx `stale-slide`/`stale-shape`, `stdio.pdf.set-page.stale-target`, xlsx
  `sheet-stale`/`cell-stale` (the lookup runs before the per-cell revision check) and `fem*.focus-entity.unknown-entity`.
- **The same lookup after the whole-document revision check has passed → input-invalid.** This applies to csv/tsv/bcf
  `*-stale` (every table command checks `revision` first) and to wav `channel/frame/sample-stale` (`audio-conflict` is checked before
  `AudioPlan::new`). The document is exactly the one the caller addressed, so the address itself is wrong. The real concurrent case
  keeps `conflict` via `*.table-conflict`/`audio-conflict`.
- **Wrong route, wrong step, or work already finished → internal.** Covers `*.command-mismatch` (bounded-native, snapshot-edit,
  docx, epw, zip, semio brep/mesh, xlsx native), `*retained-route-required/rejected`, `drawing.gesture.command`,
  `drawing.gesture.retained-route` (both sites are routing errors; the pointer-move site is only reachable idle when mis-routed),
  `drawing-window-work-terminal`, `stdio.zip.work-complete` and `docx.set-page.copy-incomplete`.
- **Our own codec, inverse or registry invariant → internal.** Covers snapshot-edit `inverse-invalid`, `inverse-mismatch`,
  `publication-codec`, `publication-invalid`, `publication-mismatch`, `invalid-schema-contract` and `schema-unregistered`; gltf
  inference `snapshot-decode` (canonical pack) and `unknown-leaf` (`&'static` id from our table); `drawing.example.parse` (bundled
  example); `drawing.mutation.rejected` (our planned mutation fails structurally on our working copy); `drawing.child-projection`
  (loaded composition fields); and zip `checkpoint-capacity/invalid`.
- **Document state rejects a valid input → precondition-failed.** Covers drawing `fill.stop-invalid`/`stop-limit` (checks the
  stored fill), `path.contour/geometry/point-invalid` (checks the stored path segments), `arrange.*-count`/`bounds-missing`
  (selection), `arrange.capacity` (a 4096-layer traversal of the whole document, not of the selection), `group.compositing` and
  `selection.stack-invalid`. It also covers `png.pixel-region.raster-too-large`, `wav.format-copy`, `format-data-mismatch`,
  `zero-channels`, `docx.set-page.paged-owner-required` (the address does not resolve in the current document), `pptx
  unsupported-target` and `snapshot-edit.path-invalid`.
- **Entered value is wrong → input-invalid.** Covers snapshot-edit `ambiguous-object`, `depth-exceeded`, `descendant-move`,
  `lossy-conversion`, `path-shape`, `root-operation` and `schema-identity`, plus `stdio-txt-schema-is-immutable`,
  `txt.replace-text-unrepresentable`, `pdf.page.image-samples-size`, `pdf.page.annotation-unaddressable` (object id does not parse)
  and `pdf.page.resource-owner-unknown` (the `owner` grammar hit was wrong: it is an unknown token in the argument).
- **`drawing.path.points-changed` → conflict.** Point references carry the path geometry id; a mismatch means the reference went
  stale after another change.

### Ambiguous → S20

- **Window-stale consistency across families.** I set it to precondition-failed. Helpers of other families may keep `conflict` by
  code grammar, so one call should hold everywhere.
- **`fem.window.mismatch`** (kept precondition-failed): the requested window differs from the captured window config. It could be
  input-invalid, because the request contradicts its dispatch window.
- **`drawing.gesture.closing`** (kept precondition-failed): it could also be `cancelled`.
- **`drawing.gesture.draft-owner`, `query-owner`, `owner`** (kept internal): these are only reachable when gesture commands
  interleave. They should be `conflict` if the runtime can legitimately interleave them.
- **`drawing.selection.ancestry-changed`** (kept precondition-failed): impossible within one revision, since the op is cancelled
  on a revision change, so it may be internal.
- **`stdio.zip.checkpoint-context`** (set internal): the binding includes `entries.len()`, so a concurrent archive change would
  make it `conflict`.
- **`stdio.xlsx.projection-invalid`** (kept input-invalid, treated as a malformed user file): internal if projection is meant to be
  total.
- **Same code raised for different reasons** (never split; reported here):
  - `drawing.delete.capacity`: selection size vs a whole-document traversal cap.
  - `drawing.path.contour-invalid`: stored path malformed vs edit index outside any contour.
  - `snapshot-edit.ambiguous-object`: duplicate keys in the stored document vs in the entered source.
  - `snapshot-edit.schema-identity`: the edit changes the schema vs the stored schema does not match its descriptor.
  - `stdio.wav.block-align-overflow`, `byte-rate-overflow`: stored format vs requested rate or channels.
  - `stdio.docx.set-page.copy-refused`, set input-invalid: allocation admission vs source changed during copy.
  - `fem3d.result-animation-tick.window-context-required`: batch route vs view state missing on the retained route.
