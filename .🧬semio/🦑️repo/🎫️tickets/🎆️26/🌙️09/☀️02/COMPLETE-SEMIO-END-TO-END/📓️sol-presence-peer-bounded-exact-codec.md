# PresencePeer Bounded Exact Codec

Status: green for the bounded Rust/TypeScript codec. Production decoders, the schema-first neutral corpus, registered source oracles, and both exact Rust native laws are implemented and verified. The separate normalized WGPU projection source law is green and awaits the retained-WGPU native cache.

## Contract

- The existing `decode_presence_peer(&[u8])` and `decodePresencePeer(Uint8Array, [number])` APIs are hardened in place; there is no compatibility decoder and the binary wire remains unchanged.
- Both implementations reject an entry over 4,096 bytes before decoding, every text field over 1,024 bytes, a presence pack over 2,048 bytes, more than 16 views, more than 16 interaction domains, more than 64 selected or hovered ids in one domain, and a connection timestamp beyond `Number.MAX_SAFE_INTEGER`.
- Lengths and counts are validated against the fixed ceiling and remaining input before allocation. Varints must be canonical u64 values; UTF-8 is fatal; booleans are exactly zero or one; every view number is finite; flags outside bits 0 through 9 and trailing bytes are rejected.
- The TypeScript caller cursor advances only after a fully valid exact entry.
- The former private unbounded view/UI decode chain was removed after the exact reader replaced it; no compatibility path can bypass the limits inside `PresencePeer` decoding.

## Neutral corpus

`🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1` contains a strict AJV 2020 schema and 18 vectors. Two accepted rows cover every field absent and every field present and require byte-identical canonical re-encoding plus the same semantic record. Sixteen denied rows cover low and high unknown flags, truncation, trailing data, a noncanonical varint, u64-max view/domain/selected/hovered counts in short inputs, entry/text/pack ceilings, non-finite f64, invalid bool, invalid UTF-8, and a connection timestamp beyond the shared exact range.

## Exact laws and gates

- `wire::frames::presence_codec_tests::presence_peer_decoder_matches_neutral_bounded_exact_corpus`
- `wire::frames::presence_codec_tests::presence_peer_decoder_rejects_hostile_counts_before_allocation`
- `@semio-tech/framework-replication-rs:presence-peer-codec-check`
- `@semio-tech/framework-replication-rs:presence-peer-codec-native-check`
- launch order 411.059/411.060

## WGPU normalized projection

The WGPU footer now requires every decoded peer's Hub-normalized `surface` to equal the displayed target surface, in addition to the shell attachment fence, and carries the Hub-normalized `color` into `PresencePeerRow`. A mixed editor/viewer/no-surface law proves cross-surface rows and a missing normalized surface are absent and exact colors 7/8 survive.

- `shell::command_registry_tests::presence_rows_require_each_normalized_surface_and_preserve_hub_color`
- `@semio-tech/framework-renderer-wgpu:normalized-presence-rows-source-check`
- `@semio-tech/framework-renderer-wgpu:normalized-presence-rows-native-check`
- launch order 411.061/411.062

## Evidence

- Registered source gate `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/framework-replication-rs:presence-peer-codec-check --skip-nx-cache`: GREEN, 18 shared vectors and 16 exact hostile denials.
- Registered WGPU projection source gate `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run @semio-tech/framework-renderer-wgpu:normalized-presence-rows-source-check --skip-nx-cache`: GREEN6.
- Plugin registry generation refreshed 59 plugin crates, 60 playgrounds, and 45 framework packages. The direct registered `check-generated` gate is GREEN and the generated launch contains codec entries at lines 5202/5209 and normalized WGPU row entries at lines 5222/5229 with ticket-local artifact and target paths.
- `rustfmt --check` parses both changed Rust owners but reports extensive pre-existing whole-file formatting differences, so no formatting-green claim is made.
- The first exact attempt proved the build and exposed that the registered selectors lacked the nested `frames` module. After correcting the qualified selectors, the next run reached the real semantic law and exposed JSON's language-level `1` versus Rust `1.0` representation difference even though canonical wire bytes matched. The law now normalizes both JSON numeric trees before semantic equality rather than weakening the byte assertion.
- Final exact native receipt `presence-peer-codec-exact/exact-cargo-laws-OYoMqw/00` is GREEN2 with both qualified laws above, Nx exit 0, and executable SHA-256 `dfaeaa36975ae7fed508dcddd7afa811335d11de9395c91df597e5be6b733caa`.
