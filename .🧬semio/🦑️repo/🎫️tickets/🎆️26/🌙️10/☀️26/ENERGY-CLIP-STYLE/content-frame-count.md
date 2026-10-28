# Decoded Source Frame Count

The retained real source was read without modification. Its SHA-256 is `ba484742c40cbd584a6270d257bf55bca00604a5792018ccf0981f25daea24be`.

FFmpeg compressed copy probe: 48 frames. FFmpeg decode probe: 47 frames. Independent PyAV decode: 47 frames.

The composition frame count must use decoded video frames. Compressed packets can include a frame excluded by the container edit list. The probe must decode through the private FFmpeg interface before computing body duration and audio padding.

The portable regression constructs an eight-fps, sixteen-frame H.264 MP4 and applies a stream-copy edit-list cut starting at 0.125 seconds for 1.5 seconds. The result contains thirteen compressed packet samples but twelve independently decoded PyAV frames. The previous production count selected nonexistent normalized frame twelve for the restore snapshot, establishing a functional failure. The production probe now decodes video to the null output with audio disabled before deriving frame count. The retained real preview therefore uses forty-seven body frames and fifty-eight transition frames, totaling 105 frames at eight fps, or 13.125 seconds.

Final verification passes: the portable edited MP4 now preserves twelve decoded body frames plus four mocked transition frames, with matching finite audio duration and full source-frame order. All three fast production media regressions pass together in 9.935 seconds, covering edited MP4 sample counts, 4-to-2 fps endpoint selection, and equal-fps silent content.
