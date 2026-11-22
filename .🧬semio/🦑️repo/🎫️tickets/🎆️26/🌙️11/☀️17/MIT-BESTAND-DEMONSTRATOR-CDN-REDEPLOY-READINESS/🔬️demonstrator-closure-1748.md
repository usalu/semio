# Demonstrator closure check after 17:48 failure

## Findings
- `semio-plugin-instance-jobs`: the three errors (E0432 `ReservedCommitMetadata`, E0425 `FrameworkImportNaturalSourceJobFactory`, E0599 `retire_born_candidate_step`) were already resolved by another agent's refactor (items now exist in `semio-plugin-job` reserved jobs and in `🧵️job/🏭️construction`). The crate checks clean (warnings only), release wasm32-wasip2.
- `semio-framework-surface-paint`: E0027 in `🧹️host-retirement/🦀️.rs`; `RasterHost` gained `keyboard: PixelKeyboard` (ephemeral). Fixed by adding `keyboard: _,` to the destructuring in `RasterHostRetirement::new`.

## Status
- Re-check of surface-paint after the fix and the `-p semio-hub-demonstrator-component` check were refused by the gate: `Closed host capacity Policy required: /wait/starvedMilliseconds is not allowed (ref ab67e4fc)`. Not a source error; needs a capacity-policy fix or retry once the running release build finishes.
