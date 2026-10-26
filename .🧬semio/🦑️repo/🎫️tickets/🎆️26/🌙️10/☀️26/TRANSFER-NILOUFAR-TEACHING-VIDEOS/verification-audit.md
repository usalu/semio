# Nested Video Verification Audit

Read-only review of the updated ticket `verify-videos.py`. The verifier was not executed and no source, cache, process, or rendered artifact was changed.

## Findings

### P2: Validate All Required Authored Dependency Hashes

Lines 55–57 validate only `manim_fonts.py` and the descriptor entry script. A raw render made before a change to `manim_visuals.py`, an imported authored helper, or a referenced asset can retain matching values for those two checked files and pass. The report would then imply current source provenance despite stale recorded dependencies. Validate the required dependency set, and recompute each recorded source hash against the current file. The existing runtime fingerprint representation already stores multiple dependency hashes.

### P2: Logical Clip Contents Are Not Verified

Lines 86 and 97–99 inspect logical intro/chapter frame totals and durations only. Equal-length wrong or reordered logical clips can pass those checks when the final movie is independently correct relative to raw units. This leaves the 31 logical clip coverage assertion as file presence and metadata coverage, rather than content verification. Decode each logical clip and compare the corresponding raw-unit sample hashes using chapter-local cumulative offsets, with full decoded frame counts.

## Checks That Pass Source Review

The raw-unit-to-final cumulative offsets are correct: the intro starts at zero; each beat starts after the complete intro, prior completed chapters, and earlier beats in the current chapter; completed chapter frame totals then advance the next chapter start. The frame total equality chain prevents gaps or overlaps in this calculation. Raw sample selection handles one- and two-frame units through a set without creating out-of-range indices. The final decoding checks sample hashes at zero-based decoded frame positions, matching the raw unit enumeration.

The expected output names, counts of 117 raw paths and 31 logical paths, and exact enumerated path sets reject extra/missing files. Each raw file is fully decoded and counted; each final is fully decoded through PyAV and FFmpeg with error escalation. First/middle/last pixel matches establish sampled order/content evidence, not equality of every frame between raw and final; the report accurately names these `ordered_frame_samples`.

No additional offset error was identified.

## Follow-Up Resolution Review

Both findings are resolved in the current verifier by source review. The raw-unit check now requires current font, shared visual helper, entry script, existing local timing manifest, and intro PDF hashes, then recomputes every recorded dependency hash and requires recorded paths to stay within the teaching root. This closes the stale recorded dependency gap.

Every logical intro/chapter is now fully decoded with frame-count and monotonic timestamp checks. Intro samples use offset zero; chapter samples subtract `chapter_start` and restrict positions to that chapter interval, correctly mapping raw movie-global positions to chapter-local positions. The logical and final provenance sidecars also require the exact ordered raw/logical states and matching settings. This closes the logical clip content gap at the documented first/middle/last sample granularity. Movie-global sample offsets remain correct.

No additional actionable issue was identified in the bounded follow-up. This was a source-only review; the verifier was not executed against active or rejected outputs, and no compile/render/background process was launched. Full runtime execution and final artifact validation remain the parent run responsibility.
