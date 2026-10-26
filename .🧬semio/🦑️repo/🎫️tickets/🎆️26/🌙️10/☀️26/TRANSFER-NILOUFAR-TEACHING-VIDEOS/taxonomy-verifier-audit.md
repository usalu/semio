# Canonical Verifier Adaptation Audit

Read-only review of current ticket `verify-videos.py`, `make-previews.py`, profile fixtures, and canonical runtime output naming. No verifier/render/decoder was executed and no product/index/generated file changed.

## Actionable Finding

### P2: Explicit-Resolution Movie Suffix Differs From Actual Runtime

`verify-videos.py:37,171` and `make-previews.py:33` expect `<output>_180p5.mp4` for the 320×180@5 profile. Canonical runtime `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:394,402` builds explicit-resolution names with `resolution.replace(",", "x")`, yielding `<output>_320x180p5.mp4`. The verification profile fixture currently repeats the incorrect height-only suffix. Consequently exact movie-name coverage will reject the planned smoke outputs before decoding. Align the ticket verification/preview naming with actual runtime naming, including the independent fixture. Original no-override defaults `_480p15` remain correct. Parent and portability agent were notified.

## Remaining Checks

Canonical manifest, energy authoring font/visual dependencies, local timing leaf and institutional PDF paths match the frozen product layout. Required and every-recorded dependency digests, contained source paths, raw identities, explicit resolution/fps settings, nested ordered states, and zero-audio final streams remain enforced.

The exact 117 raw/31 logical/five final file-set assertions remain intact. Raw unit samples are stored at movie-global cumulative offsets; chapter samples subtract that chapters start and restrict to its interval. These offsets are unchanged and correct. Every raw and logical/final artifact is fully decoded with PyAV, counts match metadata, and timestamps must increase. FFprobe independently counts final frames. FFmpeg `-xerror` is now invoked once per distinct artifact and a set asserts coverage of all 153 paths. Report per-video FFmpeg count `beats + logical clips + 2` correctly includes one raw intro and one final movie.

Preview dimensions and sheet layout follow profile width/height; exact movie coverage, format/fps checks, and selected-frame assertions remain in place. No further concrete correctness issue was identified in this bounded review. Final staging audit remains pending parent signal. This audit launched no subprocess and has no owned process.

## Profile Finding Resolution

The portability agent corrected verifier, preview and neutral profile fixtures to use explicit-resolution suffix `320x180p5`, retaining no-override defaults `480p15`. Source re-review confirms both scripts construct names from the corrected suffix policy. This resolves the only actionable finding above. No decoder or render was launched by this source review.
