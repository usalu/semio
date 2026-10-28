# Calm Cursor Final Standalone Source Audit

Independent read-only review of current cursor-motion.py, render-calm-cursor.py, the retained JSON/schema/cases, cursor-tests.md, and production browser/🐍️.py. Commands used repo tools exec --light node. No browser captures, production edits, git mutations or original export writes were performed.

## Result

No blocking source issue found in the final standalone strategy. This supersedes the earlier audit of generated/calm-cursor and its canonical endpoint expectation. The discarded first 330-image attempt is not accepted evidence: it lacked returned metadata after the existing shared handoff source-change guard rejected it. Actual final DOM frames and encoded output remain the separate frame auditor’s responsibility.

## Motion, Language and Interaction

- --language remains required with exactly en/de. That explicit value reaches every browser packet and final metadata.
- The exact schema still selects only optimization as a topic, 1600×900, 30fps delivery and 11s total. The five stages remain 1.5s idle, 2.5s approach, 3s hover, 2.5s return, 1.5s settle.
- The pure sampler still holds a constant target throughout [4s,7s). Final native capture samples this at 12fps: 36 stationary native hover observations become 90 delivered hover frames. Both travel legs use smoothstep.
- Every packet starts at initial_hierarchy, preserving opened_topic/opened_video/revealed_video null. Only revealed_topic can be optimization. The direct measured route introduces no extra reveal, click or navigation stage. Every planned cursor has pressed=false.
- Post-capture checks reject actual native pointer_events, any opened pane, any unrelated chip containing the current point, and any hover observation outside the actual optimization chip. The direct route no longer has coarse candidate probes; full native-frame collision checks remain. No source claim should imply 330 distinct DOM checks.

## Honest Native and Delivery Counts

The final movie batch contains 132 actual PNG/DOM observations at 12fps. metadata.frames and metadata.timeline represent that native batch. metadata.fps=30 is accompanied by captureFps=12; deliveredObservations separately contains 330 entries at frame/30 timestamps and records nativeFrameIndex, sourceSeconds, source point, source stage and reveal for each. The console message explicitly distinguishes native and delivery rates/counts. The frame_schedule integer mapping matches the requested FFmpeg fps=30:round=down:start_time=0 filter and the encoder caps output to 330 frames. There is no interpolation claim: delivered images repeat quantized native observations.

The retained verification report records nine passing Python tests, the existing NumPy timeline/index oracle, AJV validation and a real FFmpeg/PyAV lossless image-label oracle. This audit inspected that evidence without rerunning green tests.

## Standalone Snapshot and Cache Boundary

Every frame, including probes/rest states, explicitly sets canonical_overview=false. Production browser._canonical_root therefore returns false for these packets. browser.render_batch calls its normal _call path then returns directly when no canonical frames exist: it does not request _handoff_identity, perform a shared handoff certificate, or read/write _HANDOFF_ASSETS. The existing production source-change guard remains present and unchanged. The final 132-frame movie batch runs through one compiled browser capture; preceding measurement probes are separate finite captures and are reported separately. captureScope honestly describes the movie batch as one compiled standalone DOM snapshot with no shared canonical handoff asset.

The script never invokes render_segment, compose_videos, presentation publication, reusable_clip, source_files certification or a producer cache publication function. Its source_files field is the browser-returned dependency list, not a freshness certificate for the six lesson exports. Endpoint validation now records actual exactPixels/PSNR, enforcing at least45dB when endpoints differ; it does not claim bit-identical canonical cached endpoints.

## Write Scope

All authored output paths resolve beneath ticket/🗑️generated/calm-cursor-final: probe PNGs, native frame PNGs, metadata.json, probes.json, plan.json, endpoint-stability.json, calm-cursor.mp4, selected calm-cursor.png and encoding log. SEMIO_TEACHING_OUTPUT_ROOT is set to that same owned root; the production browser creates its request/response/log attempt subdirectory there and removes successful attempts. No output path references videos/de/speaker/delivery or rendered. No source calls can overwrite any of the original six films. Before/after original export hash verification remains a coordinator concern; source inspection alone is not a byte preservation claim.

## Final Acceptance Boundary

Actual frame audit must verify the returned132 observations, mapped330 delivered frames, optimization-only reveal, zero native clicks/opens, stable plateau, readable final film and measured endpoint stability. No additional browser capture was started by this audit.
