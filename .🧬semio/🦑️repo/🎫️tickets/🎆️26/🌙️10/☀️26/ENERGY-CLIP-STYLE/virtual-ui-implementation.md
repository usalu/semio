# Native Virtual Teaching UI

The presentation contract defines one canvas with a fixed semio navbar/footer and five localized lesson windows in curriculum order. Every lesson starts and ends on the same immutable overview. Focus is a presentation transform; no active/progress state changes the boundary frame.

## Production Files

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json: schema-first geometry, explicit English/German labels, full authored frame retention and timing.
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧬️schema.json: closed JSON schema for five windows and localized presentation settings.
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py: native Manim canvas/chrome rendering, short focus transitions and protected media composition.

The parent agent owns runtime/manifest/chapter typography integration. The test agent owns language-neutral test vectors and third-party/native verification in the adjacent tests domain.

## Public Interface

- presentation_contract() returns the canonical declarative contract.
- lesson_window(video_id) resolves one canonical window; unknown identities fail.
- overview_layout(width, height) and focus_viewport(width, height) expose projected pixel geometry.
- source_files() includes module, contract/schema, theme tokens/font assets, transitive font/visual/runtime Python dependencies, architecture profile and silhouette data.
- render_overview(output, settings) writes the immutable native PNG.
- render_shell(video_id, output, settings) writes a native RGBA focused shell with a transparent viewport.
- render_presentation(video_id, content, output, settings, session=None) publishes the complete lesson envelope.

Language is mandatory and explicit (en or de). Quality defaults agree with raw Manim rendering: ql 854×480 at 15 fps, qm 1280×720 at 30 fps, qh 1920×1080 at 60 fps. Resolution/fps overrides remain supported.

## Rendering and Audio

The native Manim Cairo renderer draws all semio fused title chips, window silhouettes, typography, abstract thumbnails, navbar/footer and focus interpolation. The current installed Manim ImageMobject camera path attempts to build an RGBA image from a two-dimensional float array and fails in Pillow. The presentation therefore keeps image compositing behind a private Pillow interface while native Manim continues to render all moving UI geometry. No library monkeypatch or new dependency is introduced.

The focused viewport contains the complete authored frame with aspect ratio preserved. No chapter labels, references, formulas or scientific content are cropped. First/last content snapshots blend into the short native transition to avoid a switch from an unrelated image at focus boundaries.

The existing runtime FFmpeg/ToolSession interface performs the media composition. Original narration receives the exact silent intro pad; silence continues through the outro. Every transition checks cancellation; native frame progress is emitted with the temporary debug prefix. A unique short attempt directory isolates every render. FFmpeg writes a unique partial movie, and the final output is atomically replaced only after successful encoding and nonempty output verification.

Lossless H.264 composition keeps the decoded first/last overview pixels identical across all five lesson IDs. The encoded frame rate controls quantized padding; the contract total is two seconds at each end for the tested and standard frame rates.

## Review and Verification

Schema/geometry tests were written and observed failing before the production module existed. Native overview and focused shell previews were rendered successfully. Initial native testing identified and corrected the upstream image camera path issue, a relative-opacity issue on abstract thumbnails, and publication/default/cache identity review findings. Final test and visual results are coordinated with the parent agent and test agent.

No authored scientific geometry or material implementation was edited for this presentation.

## Final Integration Notes

The six schema, geometry, locale, cache-identity and runtime-integration tests passed. The NumPy geometry oracle independently agrees with the projected rectangles. Native overview and transparent focused shell previews rendered at 960×540.

A native encode test additionally exposed FFmpeg concat inferring a one-million-frame time base when no final frame-rate constraint was present. The final concat now includes an explicit fps filter, and the encoder receives matching -r / fps_mode cfr settings. Partial movie names are short and independent of final lesson filename length. These changes are settled for the independent media verification rerun.

Production files owned by this implementation are exactly the new presentation module, contract and schema listed above. This ticket report is the only persistent implementation audit added by this agent. Test/media timing, waveform, cross-lesson decoded endpoint equality and integrated authored-preview results are recorded by the testing agent and parent task after the final native rerun.

## Verified Finite Media Composition and Cleanup

The final presentation suite passed all nine tests in 67.562 seconds. Each of the five synthetic two-second lessons decodes to exactly 24 frames at four fps (six seconds total). All ten first/last overview RGB frames are byte-identical within and across lessons. The original 440 Hz narration proxy retains its waveform/RMS and receives exactly two seconds of silent padding at each end. Focused full-source containment passes the independent Pillow comparison.

The media graph is finite. A fast compressed-packet FFmpeg pass measures authored video frame count and rate without decoding. Source timestamps are normalized at their authored frame rate; the last source frame receives a complete sample interval before output resampling. The concatenated video receives sequential frame timestamps before final resampling so missing packet-duration metadata cannot drop boundary frames. Video, focused shell, original audio padding and final output are all bounded by measured frame counts/durations. Original narration is still the source audio stream.

Every render owns an eight-hex-character attempt directory. Cleanup verifies that its resolved parent and exact owned directory pattern match before deleting anything. Success removes all owned attempt work. Failure or cancellation removes heavy PNG sequences, text caches and partial movies, preserving only bounded diagnostic logs/JSON. Cancellation is checked again before atomic publication. Encoding emits start/completed progress with elapsed time.

The final native media tests require no additional production source changes. The implementation is settled for the parent task's final authored previews and publication checks.
