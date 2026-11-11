# Course Preview Audio

The preview joiner validates every declared lesson in manifest order against current source fingerprints, master provenance and digest before assembling. It no longer accepts a missing-lesson override. Each source becomes 480p15 H.264 and 48 kHz stereo AAC; a silent stream is generated only when the source has no audio. Explicit source video durations and cloned end frames preserve the timeline through fps conversion. Chapter boundaries derive from the normalized video duration.

The retained preview-cases.json and preview-test.py generate three one-second colour lessons: 440 Hz, silence, and 880 Hz. The test passes against FFmpeg, PyAV and NumPy for stereo audio, presence of tones and silence, colour order, total duration, and chapter names/times. Generated media is under 🗑️generated/preview-test.

The joiner requires --work-root pointing to this ticket’s 🗑️generated folder. It writes a sibling partial, recomputes master freshness and digests after joining, then atomically replaces the preview; a race regression confirms an existing preview survives a changed master. Run through repo tools exec bun -- <Nov01 ticket>/🧰️r3-media/run.sh is not portable; invoke the teaching Python environment through repo tools with course_preview.py en --work-root <ticket>/🗑️generated. Both acceptance tests pass.

Source audio is preserved as authored. Inspection of Beat2_EnergieImAlltag found no English or German narration recordings; its current master audio is silent. The preview does not create missing speech.
