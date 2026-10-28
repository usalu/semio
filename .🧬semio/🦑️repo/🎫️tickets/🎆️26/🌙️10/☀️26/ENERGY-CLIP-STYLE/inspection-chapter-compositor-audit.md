# Chapter Compositor Audit

Read-only review of collection.compose_lesson, presentation._render_chapter and ToolSession. No render duplicated.

## Concrete Multi-Chapter Blocker

compose_lesson calls body_frames(raw,settings,work,session) once per chapter. body_frames always chooses work/body-probe.log. ToolSession.run opens supplied logs with mode x, so the second chapter in one lesson raises FileExistsError. A single-chapter physical smoke does not exercise this. Root was notified before multi-chapter publication.

## Cancellation Scope

compose_lesson checks its session before each chapter and uses it for body probes/final concatenation. Each nested runtime.publish_presentation, however, creates a separate ToolSession and has no caller-session parameter. Cancelling the outer session cannot directly cancel an in-flight nested chapter browser/encoder; it becomes visible after that call returns. _render_chapter itself checks its own session before atomic replacement and cleans partial output. This is a concrete cancellation-boundary limitation, not a request for broad redesign.

## Finite Equality And Paths

_render_chapter fixes normalized body count, focus/restore counts, total frames and duration, encodes lossless H264 at yuv420p, and pads/trims explicit audio. compose_lesson concatenates those exported chapter videos with setpts=N/fps and setsar only; it applies no rescale, uses lossless H264, and fixes final frames to the sum of chapter clocks. Thus exact decoded video concatenation is the intended invariant, but must be tested with independently selected chapter exports using binary-identical endpoint snapshots. AAC audio is decoded and re-encoded, so compressed-file identity or exact audio sample identity is not implied.

Current chapter export paths max183 UTF-16 units; chapter work132 and sampled focus-frame paths147, below256. Browser work limit220 is also respected. Existing production candidate publication remains atomic. Chapter cursor intervals are shifted by explicit focus padding and lesson offsets use exact frame counts.

Neighbor boundaries use the common ancestor prefix and canonical parked cursor, allowing both sides to request the same overview raster identity. Exact decoded equality still requires the actual browser snapshot/concat oracle; source review alone does not certify it.
