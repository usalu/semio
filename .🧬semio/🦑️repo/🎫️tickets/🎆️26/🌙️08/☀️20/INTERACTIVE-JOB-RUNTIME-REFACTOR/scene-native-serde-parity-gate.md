# Scene Native Serde Parity Gate

Canonical command: `bun x nx run @semio-tech/ui-scene-rs:test-long --skip-nx-cache --args='--lib -- owned_scene_neutral_vectors_match_native_serde_packet --nocapture'`, using the existing master-ticket target and retained artifact directory.

- R1 actual RED: `🧪️member-scene-serde-parity-r1-native-2026-08-27.txt`, 0 passed / 1 failed / 93 skipped, 0.049 seconds. At `🦀️pack.rs:751`, a test producer used generic `serde_json::Value` map serialization, which ScenePack deliberately rejects with `Unsupported("map")`.
- The owning renderer lane replaced only the four map-containing test producers with concrete serde record types, including the exact renamed prototype key. No production codec behavior or fixture bytes changed.
- R2 actual GREEN: `🧪️member-scene-serde-parity-r2-native-2026-08-27.txt`, 1 passed / 93 skipped, 0.011 seconds. All 19 language-neutral byte vectors executed through native serde.

This is exact native codec parity, not a browser presentation or all-scene runtime gate.

The separately added typed catalog test `typed_scene_neutral_catalog_matches_native_serde_contracts` also passed: `🧪️member-scene-typed-contract-r3-native-2026-08-27.txt`, 1 passed / 94 skipped, 0.065 seconds. It exercises the existing SceneDoc decoder/native serde contracts against 15 valid typed vectors and six hostile vectors. It does not convert the earlier generic codec test into scene-evaluation or browser runtime coverage.

The subsequent numeric-width regression `scene_pack_numeric_widths_do_not_wrap` has an actual semantic RED: `🧪️member-scene-numeric-red-r4-native-2026-08-27.txt`, 0 passed / 1 failed / 95 skipped, 0.052 seconds. The first vector (`u8-overflow`) was incorrectly accepted by the unchecked integer conversion. Later vectors were not reached in that RED run.

After the renderer owner changed only integer conversion to checked `try_from` and preserved floating-point behavior, R5 passed the numeric test: `🧪️member-scene-numeric-green-r5-native-2026-08-27.txt`, 1 passed / 95 skipped, 0.024 seconds; all 12 numeric vectors reached their assertions.

Full Scene regression R6 passed **96 tests / 0 skipped**, 3.626 seconds: `🧪️member-scene-full-r6-native-2026-08-27.txt`. This includes both native parity siblings and the checked numeric boundaries. It remains native library evidence, not fresh component-Wasm or browser evaluation evidence.
