# Media Value Error and Pack JSON Integration

## Scope

The media family had stale native SQLite APIs that erased typed refusal information into `String`, plus native tests that still called removed JSON aliases. The integration cut updates GIF, JPG, WAV, AVI, and MP4 together so later family compilation does not expose the same drift artifact by artifact.

## Typed SQLite Boundary

- GIF 87a and 89a, JPG, WAV, AVI, and MP4 native SQLite codecs now return `ValueError`.
- Invalid input, work limits, ownership limits, allocation failures, and unsupported owners preserve their existing `ValueRefusalKind`.
- Handcrafted AVI stream-format and WAV data/chunk controlled fields carry typed failures directly.
- WAV and AVI encode/decode paths measure the retained semantic snapshot with direct checked arithmetic before applying `max_value_bytes`; this remains independent from exact physical `max_file_bytes` admission.
- MP4 now applies the same semantic owner measurement on native encode and decode. Its first complete build ran 59 of 65 tests before fail-fast: 58 passed and the exact binary physical frontier exposed a refusal-kind defect in Pack's `max_file_len` append check. That physical byte ceiling now returns typed `OwnershipLimit` through `PackError::ValueRefusal` instead of entering the generic `LimitExceeded` work classification.

## Pack JSON Boundary

All 144 stale media-native JSON alias calls were replaced with the explicit current contract:

- `semio_framework_pack_json::from_json_str(text, JsonMemberPolicy::Reject)`
- `semio_framework_pack_json::to_json_string(value)`

No legacy alias or string-erasing compatibility adapter was added.

## Registered Evidence

- JPG: 133 of 133, `jpg-pack-json-test-2026-10-03-2.log`.
- GIF: 124 of 124, `gif-pack-json-test-2026-10-03-2.log`.
- WAV: 57 of 57, `wav-pack-json-test-2026-10-03-5.log`.
- AVI: 52 of 52, `avi-pack-json-incremental-test-2026-10-03-5.log`.
- MP4: `mp4-pack-json-test-2026-10-03-4.log` compiled all 65 tests, passed 58, failed the exact binary `max_file_bytes - 1` refusal-kind assertion, and left six unrun under fail-fast. `mp4-pack-json-test-2026-10-03-5.log` is the registered post-repair confirmation run.
- MP4 confirmation session `63842` was still waiting in Cargo build on the shared artifact directory lock at handoff. This is an active run, not passing evidence.

The earlier focused checks are `gif-value-error-check-2026-10-03.log`, `jpg-value-error-check-2026-10-03-2.log`, `avi-value-error-incremental-check-2026-10-03-2.log`, `mp4-value-error-check-2026-10-03.log`, and `wav-value-error-check-2026-10-03-3.log`.
