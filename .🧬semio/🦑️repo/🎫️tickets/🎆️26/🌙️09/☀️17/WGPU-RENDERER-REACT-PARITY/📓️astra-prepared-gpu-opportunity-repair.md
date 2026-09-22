# Prepared GPU Opportunity Timing Repair

Checkpoint20 entered a new quarantine at 19:40:07 UTC: four consecutive opportunities over the 2000us ceiling, with the final Commands opportunity reported at 3100us. That receipt did not identify the command or draw scalar, so it cannot establish which operation was expensive.

The read-only audit found five successful early-return paths before timing admission: exhausted Commands, metadata-only Commands, exhausted blur mips, empty glass clip, and exhausted glass clips. Those opportunities could separate slow work without resetting its accumulated overrun run. The old diagnostic therefore could not reliably establish consecutive overrun opportunities.

`prepared_present_step` now validates admission and invokes `measure_prepared_gpu_opportunity` around the private advancement method. Every successful transition is timed and revalidates its cursor. Re-entering an already Complete cursor remains an immediate terminal read. The common wrapper preserves the 2000us ceiling and four-consecutive-overrun threshold; advancing a command does not clear a real overrun. Terminal diagnostics include the pre-step phase, command kind, draw cursor, and before/after progress.

The shared opportunity fixture now includes successful transition samples and four advancing overruns. UI24 executed the new common-wrapper law and all existing prepared GPU laws successfully. The full UI24 gate was 668/669: its only failure was an unrelated malformed compact Tree fixture, subsequently corrected. No fresh browser GPU acceptance is claimed until wasm21 is built and exercised.

UI25 subsequently passed the full 669-law UI suite (6.609s, zero skipped). The workspace P5d live-source and hostile-mutation gate also passes after updating its scalar-call anchors for clipping and extending it to require the shared timing wrapper. Added mutations reject bypassing timing, returning before timing, and replacing either clipped or unclipped scalar encoding with whole-frame rendering. The first P5d attempt correctly refused a stale mutation anchor; it was not a product failure.
