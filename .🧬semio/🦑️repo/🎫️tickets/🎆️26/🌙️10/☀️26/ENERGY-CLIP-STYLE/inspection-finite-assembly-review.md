# Finite Phase Assembly And Postcondition Review

Read-only review of presentation/🐍️.py and presentation/📚️collection/🐍️.py. No source edits, renderer or encoder run.

## Actual Missing Postcondition

Presentation _render_chapter lines211–216 treats an encoder exit with a nonempty partial file as success, then replaces output. It never independently decodes the candidate count. Collection publish_chapter line128 checks only size above1000 bytes before publishing its certificate; compose_lesson lines175–183 assembles the declared total and checks input currentness, but not actual output count. Therefore the stable EB07 short output can receive a structurally plausible publication certificate until the external verifier rejects it. This is a concrete missing native finite-count postcondition.

Existing collection.body_frames lines70–81 already performs a full native FFmpeg decode through runtime.ffmpeg_command/run_command, shares cancellation session, owns a unique short log and derives normalized count from decoded frames/rate. At the same final profile it can support an expected-total assertion before replacing/publishing a candidate. A direct native count helper should also enforce expected final frame rate and dimensions rather than silently normalize malformed final output. This can remain in presentation/collection and need not modify runtime.py or source-body certificates.

## Separated Finite Phase Option

The existing collection assembly pattern uses separately finite video inputs, normalized frame timestamps and SAR1, lossless H264/yuv420p, declared total frames/duration and existing body audio padding. The same approach can assemble three locally encoded phases: actual focus PNG sequence, normalized body and actual restore PNG sequence. Each phase must be fully decoded and equal its declared count before assembly; final output must also equal their sum. This moves image-sequence decoding out of the multi-input concat scheduling graph while retaining every actual UI raster and body frame. It does not by itself prove the isolated phase encoder cannot drop frames; its count assertion is necessary. Preserve existing body-audio delay/padding from the original source rather than adding independent AAC phase padding.

All tools remain behind the current runtime command interface. Use the same session for every expensive phase/decode/assembly, unique log names (ToolSession opens them exclusively), finite counts, and candidates inside the existing owned p-* work directory. Existing _render_chapter finally removes external partial and render_presentation finally verifies p-* cleanup ownership.

## Concrete Cleanup / Cancellation Implications

Cancellation is already checked before transition capture, shared into native commands, and checked before partial replacement. Preserve that path for each new command and before publication. A separated phase file must be scoped under work, not a new unowned output beside the final. Existing failure cleanup retains only compact JSON/log files and removes PNGs/media (presentation lines143–156), which explains why the actual failed EB07 phase sequence was unavailable for postmortem. Any bounded diagnostic capture must copy the exact failing inputs into ticket-owned generated diagnostics before ordinary cleanup; do not silently broaden permanent artifacts.

Actual PNG chunk evidence is in inspection-transition-png-parameters.md: current focus UI frames0–12 omit pHYs and final snapshot frame13 has1:1 pHYs, while dimensions/type/depth are equal. Executor owns the native FFmpeg parameter/reinit reproduction; no causal claim or speculative production change is made here.

## Actual RGB Normalization And Clock Guard Review

Read-only review after executor saved presentation._normalize_transition_frames and collection._decoded_clock/require_frames. No material failure found for the active480×270/4fps contract.

Normalization reads each declared actual PNG, rejects wrong format/viewport, obtains RGB channel bytes with Image.convert(RGB), and rebuilds a fresh RGB PNG in a separate owned normalized/i or normalized/o directory. It does not mutate actual browser/canonical assets, add a background matte, blend alpha or apply a gamma transform. Consequently the relevant guarantee is exact preserved decoded RGB channels under the existing opaque-video encoding contract, not an assumed visual alpha composite. Rebuilding removes varying alpha/mode and incidental PNG metadata; body input and normalization filter are unchanged. Boundary comparison must use decoded RGB, while original immutable asset checksums remain unchanged. Executor regression independently validates source immutability and exact64 body frames; actual EB07 replay remains required for integration.

Each image checks cancellation before read and before atomic destination replacement, and temporary files are removed in finally. All normalized images remain under the verified p-* attempt cleanup. Native require_frames fully decodes the candidate through the existing runtime port/session and a unique UUID log, then rejects wrong decoded count or profile frame rate before chapter partial replacement. The collection output is also decoded and checked against the sum before returning to its publication wrapper. These postconditions close the previously documented nonempty-output gap; no certificate rewrite is involved.

The native guard is deliberately a frame-count/rate postcondition; complete pixel order, body/boundary equality, dimensions/audio and source provenance remain the independent final verifier duties. Existing body/audio filter behavior is preserved. No speculative separated assembly change is needed once the actual mixed-mode cause is normalized and exact native replay passes.
