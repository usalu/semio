# Virtual UI Continuity and Publication Audit

Read-only source audit, 2026-10-26. Scope: presentation compositor, runtime orchestration, domain ordering and obsolete intro paths. This is not a pixel review. Parallel edits and the running native tests may change the referenced lines.

## Findings

### P2: Final Movie Replaced Before Provenance Validation

presentation/🐍️.py:341-345 runs the encoder and replaces output immediately. runtime/🐍️.py:546 then checks whether presentation inputs and content provenance still match the state captured before rendering. If an author edits a dependency during rendering, the check raises only after the previously published movie has been overwritten. No success sidecar is written, but the uncertified movie remains visible at its final path. Encode to a unique candidate, validate the complete content and presentation state, and only then atomically replace the final movie. Keep the old final movie and certificate intact on cancellation or stale-input failure. Also check the session cancellation flag immediately before publication.

### P2: Native Frame Workspaces Accumulate After Success and Failure

presentation/🐍️.py:301 creates a unique p-* workspace containing two full sets of native transition PNGs, reversed copies, text SVG caches, probe frames and logs. The only finally cleanup at 346-347 removes the partial movie; the workspace survives every call. Repeated full-resolution builds accumulate large image trees. Put disposable presentation work under the declared generated/cache area and remove the owned workspace on success and cancellation/failure, preserving only deliberately requested diagnostics.

### P3: Encoder Progress Is Hidden for the Most Expensive Phase

Native frame creation prints periodic progress at 282-283. Final FFmpeg composition writes all diagnostics to compose.log through ToolSession, so the caller sees no progress during potentially long lossless full-lesson re-encoding. Provide a parsed progress stream or phase progress through the same neutral runtime interface. Keyboard interruption does propagate to ToolSession.run, which terminates its process tree.

### P3: Deleted Intro Still Has an Orphan Speech Generator

🎬️clip/👋️intro/🐍️.py is absent and no current curriculum segment points to it, but 🎬️clip/👋️intro/🔊️speech/🐍️.py remains an executable generator for the old NGS intro card and intro_physical_fundamentals audio. Remove it and its unused old intro assets if the instruction is to remove the intro production path completely. Runtime segment_plan also retains an obsolete intro reference in its docstring at line 79. Narration clauses named intro, Heating_01_Introduction and local introduction diagrams are authored teaching content, not obsolete standalone intro scenes.

## Verified Design Properties

- Manifest order and presentation window order both match physical_fundamentals → ventilation → heating → cooling → energy_balance.
- compose_videos selects requested IDs using a set but iterates the manifest, preserving curriculum order for arbitrary CLI selection order.
- No standalone intro is selected by the manifest. The shared fonts.scene_title now creates a small left-aligned window chip; beat_subtitle aligns below it. The legacy helper name remains necessary because scenes call it, while its centered title-screen behavior has been replaced.
- Overview construction does not depend on active lesson identity. At alpha=0 the transition renders _canvas(None), so first and last native frames use the same five-window canvas and fixed navbar/footer for every lesson.
- Transition rendering uses full authored first/last snapshots. Source content remains uncropped and is scaled/padded into the focused viewport. Intro/outro soundtrack is silent through delayed original audio and padding.
- Input/output equality is rejected; unique partial movie output protects the previous final from encoder failures. The provenance-validation issue above is separate from encoder atomicity.
- presentation_state includes explicit language and settings, theme font/token assets, presentation contract/schema/module, silhouettes, visuals, fonts and recursively discovered runtime helper sources. Content state describes ordered chapters and ordered source selections. Locale changes therefore invalidate presentation without changing scientific source selection.
- ToolSession tracks process ownership under a lock, rejects commands after cancellation, and kills descendant process groups on interrupt. Native frame loop checks cancellation once per frame.

## Verification Limits

No additional native rendering was launched because the dedicated test agent was already running the five-window continuity suite and correcting CFR/timebase behavior. Exact decoded first/last-frame identity, duration and absence of audio padding truncation should be judged by those native integration results. This audit intentionally does not duplicate the visual-review agent.

## Follow-up Verification After Publication and Cleanup Fixes

Latest read-only source check confirms:

- The runtime now renders to a unique .vui-* candidate, checks presentation state, raw-content sidecar and recursive authored dependency hashes, and validates candidate size before replacing the final movie. Stale-input rejection and encoder failure preserve the previous final movie and certificate. The original provenance-before-publication finding is resolved.
- The compositor now uses short p-8hex work directories, validates their resolved parent/name before deletion, cleans successful attempts completely, and removes heavy generated frames/caches on failure/cancellation while keeping bounded individual diagnostic files. The generated-frame accumulation finding is resolved.
- The obsolete standalone intro speech generator is absent. The obsolete production intro-path finding is resolved.
- Encoding now reports phase start and completion duration. It still writes detailed FFmpeg progress only to compose.log, so continuous encoder-progress reporting remains a minor usability limitation.

### Remaining P2: Old Certificate Survives a Failed New Certificate Write

runtime/🐍️.py:564 replaces the final movie, then line 565 writes its new .render.json. The old certificate remains in place until that write succeeds. If the state write fails or execution is interrupted between these operations, the old certificate certifies different movie bytes. A later reusable_clip call for the old state can accept this mismatched pair because it compares only sidecar state and file size. Remove the old certificate after all validation and immediately before final replacement, or implement an atomic publication record/content identity. Removing it at that late point preserves the previous pair on stale-input and rendering failure, and ensures a certificate-write failure cannot leave a falsely reusable movie. This was reported directly to root for correction.

No new render was started during this follow-up; native exact-frame tests were already running in the test agent.

## Final Resolution

After the final audit, publication now removes the previous certificate only after all candidate/source checks succeed and immediately before movie replacement. The new certificate-write-failure regression passes: a failed certificate write cannot attach old certification to new pixels. Rejected input changes preserve the previous movie/certificate pair. The deep nested source-hash rejection also passes. Owned temporary presentation work is cleaned on success, failure and cancellation; only bounded diagnostic logs survive a failure until ticket cleanup. The obsolete intro speech executable was removed. Native zooms emit frame-count progress and encoding emits bounded phase start/completion timing; cancellation remains attached to the owned ToolSession. All reported correctness actionables are resolved.
