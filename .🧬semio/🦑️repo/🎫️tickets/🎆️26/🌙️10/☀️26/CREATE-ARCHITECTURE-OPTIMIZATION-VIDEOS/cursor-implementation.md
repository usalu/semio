# Calm Cursor Implementation

The ticket-local review preview approaches only the optimization topic Ansehen chip. It idles for 1.5 seconds, approaches for 2.5 seconds, hovers for 3 seconds, returns for 2.5 seconds, and settles for 1.5 seconds. The explicit durations sum to 11.0 seconds. Delivery is 1600×900, 30 fps, 330 frames. English or German must be selected explicitly. No production source, prior ticket input, or delivered lesson video is changed.

## Schema and Test-First Work

cursor-motion.json and cursor-motion.schema.json were written before Python. The independent test agent established a missing-implementation RED result before cursor-motion.py was authored. The pure module validates the exact portable schema and samples finite normalized time with smoothstep interpolation. It clamps time to [0,1], rejects booleans and nonfinite numbers, and returns position, reveal state, and half-open stage. Tests compare language-neutral fixtures and an independent NumPy polynomial oracle; AJV independently validates the same model mutations. See cursor-tests.md.

## Standalone Browser Snapshot

render-calm-cursor.py uses the existing production browser render_batch port and real DOM chip_rect measurements. It solves the target center with the actual production camera and uses the direct, short route from the parked cursor to the bottom-right optimization topic chip. Every captured frame is checked against all other root chip bounds. The full three-second dwell must remain inside the measured target bounds. All opened_topic/opened_video/revealed_video paths stay empty; revealed_topic alone is optimization during hover. Cursor pressed is always false, and actual browser pointerdown, pointerup, or click events cause rejection.

A first 330-native-frame draft finished capture but the shared lesson handoff cache rejected a concurrent source-set change before returning metadata. That draft is not published or certified. The active preview is one fresh compiled standalone browser snapshot. Its packets explicitly set canonical_overview=false because the standalone review does not request a shared canonical lesson handoff asset. The production browser source and its shared lesson certification remain unchanged.

Native capture is 12 fps (132 actual DOM observations), resampled to the contractual 30 fps (330 delivered frames) using FFmpeg fps=30:round=down. All stage boundaries are exact at both rates. The three-second dwell contains 36 actual native observations and 90 delivered frames. Portable schedule fixtures and an independent timestamp-quantization oracle verify output-to-native indexing. The renderer never claims that repeated delivery observations are fresh DOM captures.

Source root endpoint pixels must be equal or reach PSNR 45 dB. PNG byte identity is not required for this standalone snapshot. The root agent independently measured the discarded draft endpoints at PSNR 70.9843 dB; their PNG byte difference was negligible pixel variation. Encoded endpoint verification is separate.

## Outputs and Metadata

All intermediate outputs live in ticket/🗑️generated/calm-cursor-final. The discarded first attempt remains in sibling calm-cursor until root cleanup. Retained ticket inputs and this report stay outside generated output.

metadata.json contains language, target, controls (park,target,route), totalSeconds, fps (30), captureFps (12), resolution, captureScope, source_files, frames (132 complete actual browser DOM observations), timeline (132 native sample states with actual seconds and coordinates), and deliveredObservations (330 delivery timestamps explicitly referencing nativeFrameIndex and sourceSeconds). Native browser metadata includes every layer/card rectangle, hover IDs, opened panes, cursor asset/hotspot, and pointer-event observations. plan.json retains actual native browser requests; probes.json retains actual control probes. endpoint-stability.json records source pixel verification.

The root agent independently audits and publishes calm-cursor.mp4 and calm-cursor.png to .🧬semio/🎓️teaching/⚡️cache/previews/optimization/calm-cursor.de.mp4 and calm-cursor.de.png. The six lesson exports remain untouched.

## Reproduction

Select the provisioned teaching Python through SEMIO_TOOL_PYTHON and run:

```text
repo tools exec --light python -- <ticket>/render-calm-cursor.py --language de
```

The renderer uses the repository browser and encoder process interfaces with progress and cooperative cancellation. It writes no retained media to the ticket root.

## Completed Render

The fresh native batch captured all 132 actual DOM observations and encoded calm-cursor.mp4 (711,791 bytes). The automatic guards passed: no pressed/click browser events, no opened topic/video pane, no unrelated root Ansehen hit, and every stationary dwell observation inside the measured optimization chip. Source endpoint pixels match exactly, recorded as exactPixels=true and unbounded PSNR. The hover keyframe is retained as calm-cursor.png.

Nine Python tests pass after the schema and helper RED stages. An actual FFmpeg/PyAV synthetic color oracle independently confirmed the round-down mapping [0,0,1,1,1,2,2,3,3,3] for four 12-fps source frames into ten 30-fps output frames. The exact encoder filter includes start_time=0.

The completed run encountered a Windows cp1252 exception only when printing the full emoji-containing output path after encoding, metadata writes, validation, and keyframe copying had succeeded. The retained script now prints only the ASCII output filename. No media rerender is required for that console-only correction. Independent encoded-media and final publication audits are owned by the root agent.

## Independent Media Verification

The root agent decoded all 330 H.264 delivery frames with PyAV and FFmpeg. The output is exactly 11 seconds at 30 fps and 1600×900. Source endpoint RGB pixels match exactly; encoded endpoint PSNR is 50.94 dB, exceeding the 45 dB acceptance threshold. Encoded hover and return keyframes passed root visual inspection. All six original lesson videos retain their prior hashes. Independent DOM audit, final publication, and generated-output cleanup remain with the root agent.
