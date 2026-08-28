# GIS Configuration Direct Contract — Checkpoint 41

## Root Decisions From Current Source

The existing seven operations are confirmed in the actual map editor configuration source. The current whole-record diff is not retained: MutationOutcome::empty returns the default config, and impl_whole_record_config applies that value without consulting base. Thus a warning/no-op can reset unrelated populated config fields. Sparse structural deltas must have a genuine identity and compose without losing unrelated writes.

The existing inverse also conflates an absent map entry with an explicitly stored default (true visibility or1.0 stroke scale). Both are valid current config representations. The direct visibility and scale payloads therefore use required nullable values: null removes an entry; a non-null value writes that exact entry, including explicit defaults. Inverses capture the exact optional entry. Existing UI commands preserve their intentional default-normalizing behavior by choosing null explicitly; no backwards-compatible decoding alias is added.

Canonical leaf kinds/text opcodes are set-layer-visibility, set-camera, set-render-mode, set-vector-style, set-lod-mode, set-layer-stroke-scale and set-locale. The old one-word text keys are not preserved as aliases. Binary tags remain the current aggregate-local0..6 roster. The pure aggregate mechanically delegates descriptor, behavior and generic codecs.

The config snapshot stays structurally unchanged. New Gis2dConfigDiff stores sparse scalar writes and keyed optional map writes. Null map values are actual removals, not an Option<Option<T>> serde ambiguity. No mutation replay payload, full config snapshot fallback or fabricated Noop is permitted.

Root owns Rust source/tests/consumer integration. The JSON author owns only descriptors, actual schemas and neutral vectors. Both write sets are disjoint. GIS plugin lifecycle, terrain, Flow admission, runtime Interaction and Kernel sources remain excluded.

## Verification Required

Prove a no-op preserves a populated config; inverse restores missing and explicitly default map entries; independently computed two-field diffs compose; all seven operations round-trip actual serde/text/binary and descriptor provenance. JSON nullable payload fields must be required, and unknown/malformed fields must reject. Capture source/neutral results separately from native execution. No GIS or all-app readiness is claimed by this plan.
