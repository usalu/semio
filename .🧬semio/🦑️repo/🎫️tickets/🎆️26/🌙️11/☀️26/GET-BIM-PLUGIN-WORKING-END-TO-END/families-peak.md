# Families Tests Peak

Crate: `semio-s-artifact-bim-model-entities-families-tests` (workspace `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`).
Problem: the host warden killed its test-profile rustc at 2944-3624 MB (ceiling 1536 MB).

## Root cause (hypothesis, not yet measured)

- The test called `from_json_str::<ModelSnapshot>` directly, so the whole `ModelSnapshot` decoder was monomorphized inside the test crate. That is the only generic instantiation in the 95-line source.
- The manifest also pulled in ~45 unused dependencies: stdio ifc/gltf/svg/xml/json/contract, `gltf`, `quick-xml`, `serde_json` and many framework crates, and enabled `testkit` features on several bim crates (more metadata for rustc to load, plus a large unneeded build graph).

## Changes

1. Test source: `from_json_str(.., JsonMemberPolicy::Reject)` replaced by the existing non-generic `decode_model_snapshot_json` owned by `semio-s-artifact-bim-model-snapshot` (`standards::v1::subsets::any::io::text::snapshot`). No new function was needed.
2. Manifest cut to what the test uses:
   - `[dependencies]`: `semio-framework-schema`, `semio-os-kernel` (the two `extern crate` aliases in the lib).
   - `[dev-dependencies]`: `semio-framework-test-fixtures`, entities, inferences, mut-any-190, snapshot, viewer, schema; none with `testkit` or other extra features.
   - `testkit = []` kept as an empty feature because the root artifact crate (`🏢️model/📦️packages/🦀️rust/Cargo.toml`) forwards `semio-s-artifact-bim-model-entities-families-tests/testkit`.

## Files changed

- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🧬️families/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🧬️families/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml`

The original manifest is saved at `🗑️generated/families-Cargo.toml.orig`.

## Measurements

- Before: not measured by me. Reported by the caller: 2944-3624 MB (warden kill). My baseline runs failed on an unrelated peer breakage (`semio-plugin-runtime-exchange`, `debug_retained_parts`) and compiler-image exhaustion.
- After: not measured by me. My build was stopped at the coordinator's request; the coordinator adds the peak from `model-test20.log`.
- Tests: not run by me; the edited tests still need to pass in that run. If the decoded fixture differs in behaviour from the old call, note that `decode_model_snapshot_json` uses the same `JsonMemberPolicy::Reject`.

## Siblings

- `entities-tests` uses `default_snapshot()`, no `from_json_str`; `utilities-tests` and `gestures-tests` have no snapshot decode. No same-pattern fix needed there from reading imports (not measured). Their manifests were not audited for unused dependencies.
- Other entity test crates (`zoning`, `components`) use the same `from_json_str` pattern but are not workspace members here, so I did not touch them.

## Logs

`🗑️generated/families-peak-before*.log`, `families-peak-after*.log` (incomplete builds).
