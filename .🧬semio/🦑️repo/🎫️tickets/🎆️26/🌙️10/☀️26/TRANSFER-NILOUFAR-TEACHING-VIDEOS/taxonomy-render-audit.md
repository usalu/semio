# Canonical Layout Render Audit

The corrected current-layout render, standalone verification and preview generation all finished successfully. The render owner confirmed exit 0 and zero descendants before verification began. The verification controller and both owned child processes exited 0. Product sources remained frozen throughout acceptance.

The first current-layout smoke at `generated/taxonomy-render` was cancelled after deterministic narrow Pango canvas failures. Its owner confirmed controller exit and zero descendants. The diagnosed, test-first scoped canvas correction is documented in [text-canvas-audit.md](text-canvas-audit.md). The corrected acceptance run uses the fresh `generated/taxonomy-fixed` directory and frozen font helper SHA256 `4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796`. The verifier input targets only that corrected directory; cancelled outputs are not acceptance evidence.

## Independent Verification Contract

Adapted retained `verify-videos.py` and `make-previews.py` to the canonical energy descriptor, authoring fonts/visuals, chapter timing facet and institutional intro/PDF coordinates. Added explicit width, height and fps profile options with preserved defaults 854×480 at15fps. Current smoke uses320×180 at5fps and final suffix `320x180p5`.

The verifier retains exact coverage of117raw segments,31logical clips and5movies, stable descriptor order, current required and recorded source hashes, Scene/script identity, nested settings/provenance equality, exact ordered frame totals, duration sums, increasing presentation timestamps and decoded boundary sample identity. It now also performs independent FFmpeg `-xerror` full decoding for every one of those153artifacts, in addition to full PyAV decoding of each artifact. There is no tolerance for missing or unexpected raw, logical or final movie files.

The new language-neutral `verification-profile.cases.json` defines original/default and smoke profiles plus invalid width/height/fps cases. `test_verification_profile.py` initially failed because the scripts lacked profile parsing, then passed all ten profile/invalid subcases after implementation. The profile case table records the117/31/5artifact contract.

The retained pinned provisioning input downloaded the FFmpeg9.0.2essentials archive and verified SHA256 `60f467265b1e312373dbcd92200c2618a74850f98d3d078e94296bb3fa2047ba` before extracting independent FFprobe into generated/f and deleting the archive. Its process ended0. Final independent decode uses the existing imageio-ffmpeg package's FFmpeg7.1binary, resolved in the ticket verification interface, and FFprobe9.0.2. Both tool families remain behind the retained repo-launched verification interface.

`taxonomy-verify.ts` is the stable verification input: it invokes the standalone verifier and then preview generation, preserves immutable per-attempt logs, records tool PID/start/end/exit metadata, and forwards cancellation to complete owned process groups. All output stays in this ticket's generated directory.

An early independent FFprobe preflight of completed raw segment1 confirmed actual320×180, declared5/1fps,73frames and14.600seconds. This confirms quality presets did not override explicit smoke dimensions/fps. It is not final five-movie acceptance.

## Final Results

The fresh acceptance root is `🗑️generated/taxonomy-fixed`. Every file is 320×180 with declared frame rate 5/1 and final filename suffix `320x180p5`. These are complete authored scenes at the explicit smoke profile; this does not replace the retained original 854×480 at 15 fps full-render evidence.

| Lesson | Raw Segments | Logical Clips | Final Frames | Duration (Seconds) | PyAV Full Decode | FFmpeg Full Decode |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| Physical Fundamentals | 9 | 2 | 5,005 | 1,001.0 | Pass | Pass |
| Ventilation | 23 | 7 | 8,158 | 1,631.6 | Pass | Pass |
| Heating | 36 | 7 | 8,709 | 1,741.8 | Pass | Pass |
| Cooling | 22 | 7 | 5,435 | 1,087.0 | Pass | Pass |
| Energy Balance | 27 | 8 | 10,613 | 2,122.6 | Pass | Pass |
| **Total** | **117** | **31** | **37,920** | **7,584.0** | **Pass** | **Pass** |

The verifier fully decoded all 153 artifacts with both PyAV and packaged FFmpeg `-xerror`. Each decoder therefore processed 113,760 frames across raw, logical and movie layers. Independent FFprobe format, rate, frame-count and duration assertions passed. All streams have strictly increasing presentation timestamps. Every logical chapter contains exactly the sum of its ordered raw segment frames, and every movie contains exactly the sum of its ordered logical clips. All duration sums passed the retained 0.2-second bound.

All 117 raw provenance sidecars match their canonical script/Scene identities and current required/recorded source SHA256 values, including fonts, visuals, existing chapter timing facets and the institutional intro PDF. Nested logical/movie settings and provenance equal their actual ordered inputs. Every raw sidecar records the final frozen font helper SHA256 `4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796`. The verifier checked 351 ordered first/middle/last beat sample positions against both chapter and final movie decoding, with no missing, reordered or altered samples. No unexpected raw, logical or final files were present.

The independent verification Python process (PID 46728) completed in 36.816 seconds, exit 0. Preview generation (PID 39988) completed in 0.898 seconds, exit 0. The stable repo-launched controller session 32714 closed normally after both. All FFmpeg/FFprobe subprocesses were awaited; no owned verification or preview process remains alive.

Five preview sheets were generated and visually inspected, covering 10%, 50% and 90% of each final movie. The 15 samples show the expected lesson titles, diagrams, German subtitles and mathematical labels. No obvious blank-render, missing-glyph box or new layout defect was observed in those samples. Full decoder coverage is separate from this sampled visual inspection.

The retained execution input is `taxonomy-verify.ts`; machine results, immutable attempt logs, process records and preview images are generated outputs and remain subject to the ticket cleanup rule. This Markdown report preserves the acceptance metrics and outcomes after that cleanup. No product/index mutation occurred during verification.
