# Calm Cursor Regression Verification

## Scope and Contract

Ticket-local focused preview only. The canonical lesson navigation still requires two native clicks. Root AGENTS.md was read. Inputs are cursor-motion.cases.json, test-cursor-motion.py, and test-cursor-schema.cjs. No production files, AGENTS.md files, or git state were changed by this verification work.

The authored contract selects the optimization topic chip at 1600×900 and 30fps. Stages last 1.5 seconds idle, 2.5 approach, 3 hover, 2.5 return, and 1.5 settle. These durations sum to 11 seconds. The initial coordination note incorrectly called the sum 11.5; fixture normalization was corrected before implementation acceptance while retaining all specified durations.

## Test-First Evidence

After JSON and schema existed and before cursor-motion.py was authored, the pinned teaching Python ran the authored regression test and exited 1 with FileNotFoundError for cursor-motion.py. The implementation author was notified immediately and began Python implementation only after that RED evidence.

Interpreter: C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe, selected through SEMIO_TOOL_PYTHON and invoked using repo tools exec --light python. No dependencies were installed.

## Portable Cases and Independent Oracles

Fifteen portable samples specify absolute seconds, expected points, reveal state, and stage. Coverage includes both normalized clamps, every transition, quarter and midpoint motion samples, and the stable hover plateau. Ten invalid portable model mutations cover wrong fps, dimensions, target identity/kind, short legs, lost hover, click data, missing target, and invalid stage identity.

NumPy independently evaluates a Polynomial object and its derivative over 602 motion points, reconstructing the expected position and speed of both legs. Additional assertions cover finite-input validation, constant points, zero rest motion, continuous zero endpoint velocity, and one uninterrupted 90-frame hover interval. Python bytecode is disabled, and temporary invalid JSON inputs live under the ticket generated directory and are removed by their temporary-directory context.

The existing AJV2020 library independently validated cursor-motion.schema.json: the authored model was accepted and all ten invalid mutations rejected. The invocation was repo tools exec --light node -- followed by the absolute test-cursor-schema.cjs path. This schema result passed before Python implementation.

## Acceptance Status

All eight Python tests passed in 0.240 seconds against the authored cursor-motion.py, exit 0. The first implementation run exposed an oracle harness mistake: it passed NumPy arrays rather than the agreed coordinate pairs. The harness now converts its independent oracle arrays to plain lists before calling the production function; production behavior was unchanged. All fifteen portable expected samples, ten invalid model mutations, 602 independent motion samples and their speeds, 90-frame single-hover plateau, endpoint/rest continuity, clamp behavior, and finite-input checks passed.

The existing AJV oracle passed independently, accepting the authored model and rejecting all ten invalid mutations. Actual browser metadata and full encoded-frame audit are owned by the separate audit agent.

## Native Capture and Delivery Resampling

Final standalone capture is 12fps, 132 actual browser images and DOM observations. Delivery remains 30fps, 330 encoded frames over 11 seconds. The output metadata must distinguish its 132 native observations from the 330 delivered frames and associate delivered frames with their native source index. The discarded original 330-frame browser attempt did not produce accepted metadata because concurrent global CSS edits invalidated the shared canonical handoff asset guard. The final standalone capture uses explicit initial root hierarchy with canonical_overview=false and does not certify that shared lesson cache.

Before frame_schedule was added to the renderer, the authored resampling fixture and test failed with AttributeError for the missing helper. The author added the helper only after receiving that RED result. The updated nine-test Python suite then passed in 0.267 seconds, exit 0. The fixture specifies 132 native frames, 330 delivery frames, 36 native hover observations, 90 delivery hover frames, explicit round=down semantics, and fourteen source-index boundary examples.

The independent NumPy oracle constructs quantized native frame start times using integer floor division, then independently finds the source frame at each of the 330 delivery timestamps with searchsorted. This avoids floating-point rounding at exact rational boundaries. It confirms all 330 native index associations and that the resampled stage/reveal values match the exact timeline throughout. The pure timeline schema and all previous samples remain unchanged.

A second integration oracle actually encoded four distinct 32×32 native PNG labels at 12fps through the registered existing FFmpeg tool with fps=30:round=down:start_time=0 and lossless FFV1 output. Existing PyAV decoded every one of the ten output frames. Each complete decoded image matched its original label exactly, and NumPy independently reproduced the observed source map [0,0,1,1,1,2,2,3,3,3]. This passed with exit 0. The input script test-cursor-resampling.py retains prepare and verify actions. FFmpeg was resolved from the existing imageio_ffmpeg binary through SEMIO_TOOL_FFMPEG; no dependency was installed. Generated synthetic images and video were kept under the ticket generated directory and removed after this verification.

These checks verify timeline/resampling behavior. They do not claim 330 native DOM observations or substitute for the coordinator's actual delivered-video frame audit.
