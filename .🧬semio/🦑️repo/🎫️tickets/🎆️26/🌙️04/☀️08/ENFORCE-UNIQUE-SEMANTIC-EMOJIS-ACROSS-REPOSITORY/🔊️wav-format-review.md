# WAV Format Hand Review

The raw RIFF/PCM tree now has 23 individually specified moves. Its original file hashes are saved in `🗑️generated/wav-before.json`; final evidence is `wav-final-audit.json`.

Handpicked choices from the actual mutation/test contents:

- `🔮️oracle` and `🧪️tests` distinguish reference readers from subject tests.
- `☑️options` distinguishes window choices from `🎚️config`.
- Fixture pairs belong under `🔊️set-data`, `📎️set-other-chunks`, and `🎚️set-fmt`, each containing `⬅️before.json` and `➡️after.json`.
- The snapshot operation is `📸️set-snapshot`; its real resampling/amplification case is `🔊️resamples-to-16-khz-and-doubles-the-pcm16-amplitude`.
- The two suites become `🎛️mutate-wav-riff-pcm` and `🎚️create-and-retune-wave`, replacing unrelated flower/cactus icons.
- The three leaf mutation names retain their meaningful sound/chunk/format identities with canonical presentation: `🔊️set-data`, `📎️set-other-chunks`, `🎚️set-fmt`; duration uses `⏱️duration`.
- The real recording specimen is `🎙️bauen-mit-bestand-ausschnitt`, distinct from a generic test marker. Both existing owner-local directory instances are preserved.
- The bundled listening example is `🎧️example`; its exact Rust byte include was updated and its audio bytes are unchanged.

Current source paths, shared-fixture locators and manifests must move together. Frozen source-mutation evidence stays unchanged, and no sample bytes, expected values or oracle requirements may be altered. The existing snapshot descriptor points to an absent payload-schema file before this review; that is an explicit unresolved ownership issue, not grounds to fabricate a passing schema.

The final physical census retains 253 entries and all 165 files, with zero naming findings, unresolved roles or taxonomy errors. Of those files, 154 retain their complete original hashes and eleven differ only in the explicit owner/fixture/include coordinates or adjacent comments. All six fixture file sizes and SHA-256 records are unchanged, and the normal Nx fixture verifier reports three fixtures with zero file problems.

The shared-fixture Rust constant, adjacent comments and five feature inputs now resolve the handpicked recording. Two historical oracle rationale passages retain the old locator as part of their original verification account; they are not active path authority. The frozen source-mutation name remains unchanged while current mutation/scenario fields identify the repaired paths.

The separate Flow task encountered the in-flight duration-directory move and repaired the exact Stdio aggregate mount at line 9441 before this task's matching patch could apply. That concurrent change is preserved; no fallback source directory or alias was created. Native Stdio compilation is still queued behind a shared build lock, so no native WAV result is claimed. Missing pre-existing payload-schema ownership remains open.

Both actual moved WAV assets were opened by FFmpeg's independent `ffprobe`: signed PCM16 little-endian, 8 kHz, mono; the recording is 12 seconds and the listening example is 1 second. A separate read-only RIFF chunk interpretation agrees on all five fields for both files. This validates the moved audio at runtime without claiming that the repository's native codec tests have run. Diagnostics: `wav-recording-ffprobe.json` and `wav-example-ffprobe.json`.

The separate end-to-end task later observed four source-authority failures whose compiler spans still named the pre-move WAV leaves. Direct inspection confirms all five current aggregate mounts resolve, and its newer metadata build successfully compiled Stdio before reaching unrelated Flow errors. No source-authority semantics were weakened and no extra repair to the already-current WAV mounts was needed. That task's warmed backend retry is separate from this ticket's still-queued benchmark check.
