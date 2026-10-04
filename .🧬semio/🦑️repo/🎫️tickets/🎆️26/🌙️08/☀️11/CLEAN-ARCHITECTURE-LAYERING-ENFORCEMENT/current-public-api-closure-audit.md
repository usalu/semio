# Current Public API Closure Audit

Readonly source audit; no Cargo, runtime verification, source edits, or Git mutations. Snapshot follows actual Cargo `[lib]` entries and path mounts, rather than assuming conventional `lib.rs` files. Full bounded input captures are temporary under `🗑️generated/current-public-api-audit`; hashes below retain the receipt after cleanup. Generated tree excludes tests, fixtures, and derive implementation; derive manifest is included separately.

## Findings

1. **Value has actual foreign public identity exposure.** Its real package root `🌱️value/📦️packages/🦀️rust/🦀️.rs:6–7` explicitly reexports whole `serde` and `serde_json` crates. These are transparent foreign identities, not owned interfaces. The reexport exception exists in repository instructions, but no demonstrated client need justifies these broad reexports. Core `DslValue` and `Number` are genuinely owned enums; they are not aliases. Nevertheless `🌱️value/🦀️.rs:280–359` exposes foreign `From`, `PartialEq`, `Serialize`, and `Deserialize` implementations in every normal build. First-party `ToValue`/`FromValue` interfaces are already available, so foreign implementations are not required by their signatures.

2. **RetainedOrderedMap adds an unconditional foreign public trait surface.** `🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs:5,70` imports and implements serde `Serialize` for the owned map with foreign `K: Serialize, V: Serialize` bounds. This is mounted through the actual public retained-clone module. Renaming this trait or aliasing its map would leave the identity exposure. Preserve owned retained map and implement its first-party encoding law under `ToValue` with actual controlled progress/cancellation needs assessed by the active owner work; use serde only as a separate test oracle over projected owned output.

3. **Diagnostic reaches foreign traits transitively through DSL.** `⚠️diagnostic/📍️span/🦀️.rs:4,8` makes `TextSpan` implement serde Serialize/Deserialize unconditionally. DSL reexports the same canonical Diagnostic identity, including TextSpan; this does not create a false owned alias, but preserves its foreign trait implementation surface. Diagnostic has runtime serde and serde_json dependencies. Its public `encode_fault_bytes`/`decode_fault_bytes` functions have clean owned signatures (`Fault`, slices, Vec), while their implementation calls serde_json. These require an owned JSON provider underneath, not changes to caller-facing Fault identity.

4. **Actual DSL direct API is clean of foreign names; dependency closure is not.** Its Cargo runtime dependencies are exactly owned Diagnostic and Value; serde_json is dev-only. Public token, lexer, grammar, trust, and idiom declarations use owned types and std primitives/collections/function pointers. `NativeDecodeControl` comes from canonical Value and uses owned types. No regex import/dependency, foreign aliases, or serde public signatures were found in the captured actual DSL tree. DSL transitively inherits the unconditional Value/Diagnostic runtime dependencies and TextSpan serde impl through canonical reexports.

5. **OrderedSet serde is explicitly test-oriented, not an unconditional runtime leak.** `🌱️value/🗂️ordered/🧺️set/🦀️.rs:80,89` gates foreign impls with `cfg(any(test, feature = "ordered-set-serde"))`. Replication forwards this feature. Flow enables it under its actual `[dev-dependencies]` only for test-gated OutputPreview/FlowPreviewGui derives. Preserve third-party differential laws; avoid promoting the feature to runtime dependencies. After removing Value runtime serde, this test feature must have an intentional test-only provider or projection, not a broad runtime serde reexport.

6. **Derive dependencies have host/build role, not client runtime identity.** Actual Value derive Cargo `[lib]` is `proc-macro = true` and dynamically points to `../../🦀️.rs`; syn/quote/proc-macro2 support macro expansion on the host. This manifest alone does not establish a target runtime dependency or foreign public Value type. Do not classify those as runtime violations merely because they occur in `[dependencies]`. Serde/serde_json in this derive manifest are dev-only differential oracles. Expansion implementation was excluded from this bounded audit, so generated token closure is not certified here.

## Concrete Consumer Needs And Lower Owner Fixes

- DSL callers need owned tokens/spans/errors, parsing/printing, recognizer hooks, and native decode controls. Keep those canonical lower owners and remove foreign identities from their underlying implementations.
- Fault host boundaries need JSON wire bytes. `🎒️pack/🔤️json/🦀️.rs` already owns JSON Value, Number, serialization, and DslValue projection. Its implementation imports replication as `protocol`, so adding Diagnostic → Pack blindly would form a higher-layer coupling and potentially a cycle. Extract/mount the canonical JSON provider at a neutral lower owner and update Pack/Diagnostic to consume it directly; do not duplicate it or add an adapter alias.
- Dynamic data conversion callers need `ToValue`/`FromValue`, integer fidelity, ordered object entries, retained ownership, and controlled work. Those are already owned in Value. Replace serde-bound consumer behavior with those contracts and test JSON projections against a third-party dev-only oracle.
- DslValue conversion comments reference old host configuration serde requirements, but comments are not proof that current consumers still require serde. Root should inspect current actual consumer signatures before preserving any exception.
- No runtime behavior or test pass is claimed. Exact source snapshot may become stale while others edit; re-read changed owner files before applying cuts.

## Exact Input Receipt

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs` | 15531 | `4888ac6e5da9bad26f1713faffbd65f4a9eec733f39a7d4ab8c5fef6f8d90731` |
| `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs` | 1957 | `3e3bec58126c9b4ae84b7c17848c6d4b19bf0ba66367989ec9ea6dde82e6f61d` |
| `🧰️framework/🔨️modules/🌱️value/🏷️type/🦀️.rs` | 3940 | `b7c55b65b35982a8633e109df1021171109f6af5384bdde80b2c0a1f03fe2746` |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/Cargo.toml` | 376 | `cf3012d75a1ca3dc95015229fc19e4efce23c134850707135f10feeff6b89d30` |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs` | 71811 | `c8a5b173f6c33483bf137278af035daf7d6bc9b8b0afa40c4739d6a98e69d142` |
| `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` | 18043 | `57068790d91ea3804a58c0079bcf39fe3982c8c2ec28aea568a1c9267f9c0c53` |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml` | 729 | `2f10ca5328d67a68f407d2ab20b63f7b17ed332d1ec53119885fdebc3f244057` |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs` | 763 | `9116f2cfbbd1d441df1411c705420a80abc9669df267535d5b6dbe2ada86593d` |
| `🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs` | 15186 | `250c416bd3876b2e019269dd0d2cb8015971fc124b4532a73d82796c99c9ce4b` |
| `🧰️framework/🔨️modules/🌱️value/🔁️codec/🛫️controlled/🦀️.rs` | 11777 | `f894fe4fc24b73b9ae74b07bec76ba29e9db24d5b9ffe83af70b90af3455860e` |
| `🧰️framework/🔨️modules/🌱️value/🔁️codec/🛬️controlled/🦀️.rs` | 13191 | `7e4d7070bd828734c8bbe42694df0a4691604a325d6fadca591427af210e8aeb` |
| `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` | 73627 | `39baf915eeaf99bf5111830e469ddf24fb044c60cb82e13367c53b33eb90b8e8` |
| `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` | 34619 | `ee5efe8b9430b52e3bb4706e6ab06560a426b9afe490da0b16e5491ddb9d7055` |
| `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️.rs` | 6258 | `400873eccaa3f76e2e1e40727bffe1772589906103b5052645e1ad81f6285a16` |
| `🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs` | 5901 | `b7b1baf0f1c4b8781566820b6b5a708320f413ea50bb0987b961ad9f037f62b6` |
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs` | 6839 | `37d6189aa5af7299e3c015fc0845aca93c1476cd20353faaf6d87448e0814a7a` |
| `🧰️framework/🔨️modules/🌱️value/🦀️.rs` | 18696 | `82ab3779de00ed4f965ff156577469f9c8b1363f1efbac2cefa315d9f256067c` |
| `🧰️framework/🔨️modules/🌱️value/🧬️bytes/🦀️.rs` | 2773 | `1f65437fb53b5788a04364d1cf78fbe5d8efd8fc237b6929ac49735dca0d5989` |
| `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs` | 17095 | `89f85beaefff9d04b02649690457ccf8fa4b3a95210b86ee9cd9de1df4da9e5a` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs` | 14404 | `673299709fc07362cee320f0cb924fb5a8e0569f0b18b26b8b8cbb986d794fd2` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🦀️.rs` | 43033 | `37e2f50255983df81855c41c4af5ee8bd08e1d1c3f91711dafad5a6ba4d4240a` |
| `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs` | 49660 | `12af6c5175880041a1bcd1b7ea7c9444af8425f9097deffc5fe6374e65351fbf` |
| `🧰️framework/🔨️modules/⚠️diagnostic/📍️span/🦀️.rs` | 2663 | `0e6e62bad8097c48992d89e72825ecfb005a401e9ca34acdfc709218b9d55010` |
| `🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/Cargo.toml` | 576 | `8b42de2fa4ec607c66a25082bec0c6a4b5a3935dad3bbe4340f8617018440dfc` |
| `🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/🦀️.rs` | 253 | `43e59f28840170311092ce7c3c52e4ea3783e7b8921f575ac48dff373730bf9d` |
| `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs` | 27232 | `123b7272c5ca8d10d3343a4a94e0891109f696a73f68166db90c1bfd2f09b2ab` |
| `🧰️framework/🔨️modules/⚠️diagnostic/🧬️retirement/🦀️.rs` | 311 | `99e3412ad76adc4cb076ca2df0d8337dbc73ecd3b9294bc139a531f6f5c1065b` |
| `🧰️framework/🔨️modules/🗣️dsl/🎖️trust/🦀️.rs` | 1005 | `820dad5c386ea98498f8dc4c9c1e52d71eb88860be09c49554d8140e248862fd` |
| `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/📡️literal/🦀️.rs` | 7528 | `32e9239e7e9be45f9b3725edaae32b7fb608d28b029a17593f1462ebcd9e3da7` |
| `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs` | 118570 | `24ce4beb262ef2679d1836418e626054da5d7b9d283f7dd615253ad5d95115e6` |
| `🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/Cargo.toml` | 834 | `89d5afac04ce686df4cde241fff06dea269eb57c041e9fd97deb0538843c0c15` |
| `🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/🦀️.rs` | 1126 | `45aef6268809f9b84566d86f745e62bc5c21dd63e94008b879d480c5f4832803` |
| `🧰️framework/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs` | 36524 | `c018119d3d5c6ce3c7feb496567813101cb1f27f81e3b9640c8f87d47b9dd48d` |
| `🧰️framework/🔨️modules/🗣️dsl/🔤️token/🦀️.rs` | 16838 | `1f616c82bf84c9b523c5f2636b10e376645599e91da6c0c93506a4f9b0c46a57` |
| `🧰️framework/🔨️modules/🗣️dsl/🗣️idiom/🦀️.rs` | 13324 | `56fa6600ec141f333cd34b6538c677811fe341722a1c2fdbb22aef8573886109` |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` | 1842 | `ab0a8953a3838b805a58777d550ecd83b86699690ff4d0bfce57af089b048ddb` |
| `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml` | 1363 | `c21811c952ddd0b8f37205b2485cdf080ab80d02682dea7956085de4c2ec2f57` |
| `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs` | 1796 | `b1b31526379686d2f55a7dfceec34e35b0b670f73a3d6efff0ad46aec0d3018d` |
| `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` | 76901 | `65c771273827c85ab9dedfc9f007d8c5caa0538a140ac2c5f952d86f8810da47` |
