# Generation Parser Inference Harness Integration

The new generation parser changed the real payload function closure. Existing selected language-agnostic compressed-role vectors already exercise generation-index and generation-frontier, but the actual-body extraction omitted readGenerationClosureV1 and GENERATION_POLICY_V1. Both read-only audits identified the resulting ReferenceError before any runtime launch.

The test harness now extracts the actual parser declaration and imports its real generation owner policy, passing both alongside the existing exact actual payload and genuine test controls. Existing independent SQLite, real Brotli inflation, exact primary preservation and bounded partial FD2 transport assertions remain selected. No production policy or runtime code changed in this repair.

Before: 11761/02b356358ea28d73ee376616dc9c39e94699305d978555384269770914f8e9bf

After: 11927/4d295d44fef10cc9105571c8fb781ddf409eae55f70b6c944ae2b92c1ca9be74

Static TypeScript parsing passed. No Source/test function was evaluated here; actual Native execution remains required.

## Actual Generation Shape Schema Port

The concurrent current parser now consumes the actual generationSchema for closed producer shapes. The harness imports and passes that exact schema alongside its already-correct actual parser declaration and GENERATION_POLICY. Existing inflation-negative cases stop before parser invocation, so no failing runtime assertion is claimed for this subsequent closure-completeness correction. After: 12074/f37effb46239b0df320598dba6d3312273618b6fea8bbda5b9c3bf6bb855a022. Static parse0; Native unrun.
