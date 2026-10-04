# Value and Diagnostic Public Boundary Audit

Closed scope: current Framework Value and Diagnostic Rust owners, their TypeScript contracts, owned schemas and lexical direct consumers. No production, Cargo or native edits. Other agents’ authorship remains unknown. Captured epoch: 2026-10-02T21:02:10.029802+00:00. The capture preserves 3,797 complete sources totaling 56,637,654 bytes; 3,651 files contain direct Rust owner references, including fixtures/tests. This is a lexical dependency inventory, not a claim that all files are reachable in one native build.

## Actual Public Foreign Boundaries

| Owner and source span | Published boundary | Required disposition |
|---|---|---|
| Value package Rust root lines 6–7 | `pub use serde; pub use serde_json;` | Explicit intentional reexports satisfy the user’s reexport condition today, but retain the foreign public/runtime boundary. Remove with real caller conversion changes for the neutral cut; no exemption allowlist. |
| Value main lines 280–335 | Four From impls joining DslValue and serde_json::Value by value/reference | Public external type obligations; replace shipped callers with owned DslValue and controlled JSON/Record contracts. Move independently authored foreign oracle conversion into test code if still needed. |
| Value main lines 338–348 | Serde Serialize/Deserialize with foreign generic Serializer/Deserializer and associated error/output types | Hidden method signatures are still public external generic contracts. Remove shipped implementations with caller changes, not only the crate reexports. |
| Value main lines 351–360 | Two cross-type PartialEq impls with serde_json::Value | Foreign comparison API; owned structural laws remain, foreign comparisons belong to test oracles. |
| OrderedMap main lines 513–534 | Serialize/Deserialize with foreign generic bounds | Both current impls are cfg(test), so these are test oracle boundaries, not shipped default runtime API. Preserve the whole ordering/ownership laws while keeping Serde test-only. |
| OrderedSet lines 80–104 | Serialize/Deserialize under cfg(any(test, feature=ordered-set-serde)) | Feature-enabled API is real, not unconditionally test-only. Replication forwards the feature; OS Flow enables it in its dev dependency. Remove feature and forwarding only after replacing actual downstream test projections. |
| Diagnostic TextSpan lines 4–10 | Public owned TextSpan derives foreign Serialize/Deserialize | Replace all real span wire callers with owned ToValue/FromValue or controlled floor contracts; retain line/column/length keys and scalar range refusal. Diagnostic does not explicitly reexport Serde itself. |
| Diagnostic main lines 624–630 | encode_fault_bytes/decode_fault_bytes use serde_json internally | Public signatures are owned Vec<u8>/Fault, but shipped runtime still depends on foreign implementation. Replace codec internally via an owned interface retaining canonical wire and malformed-byte fault behavior; do not silently change fallback laws. |

Value and Diagnostic Cargo both list serde and serde_json as normal dependencies, not dev-only. Value also has repository normal dependencies on value-derive and io-base64; Diagnostic depends normally on Value. Thus today’s OrderedMap test gating does not remove the normal Serde runtime dependency from these packages. This audit does not calculate a whole workspace foreign dependency closure. A repo-wide direct named `semio_framework_value::serde`/`serde_json` search found no use sites at this epoch; that does not establish removal safety because implicit From/PartialEq/derive obligations remain.

Production TS import census under these owner roots found only first-party imports/reexports: intrinsic schema uses SchemaRecord and the owned Binary64 codec, controlled bytes explicitly reexports the first-party Base64 control/progress contract. TypeScript ValueType and ValueKind expose owned structural types; no external library type is exposed there. No Diagnostic TypeScript implementation was found under its owner root; its language-neutral fault JSON schema is present. Standard-library interfaces (Iterator, Arc, FnMut, Future where applicable) are platform contracts, not third-party leaks.

## Current Controlled Type and Diagnostic Floor Constraints

The canonical Type owner uses ValueType/ValueKind without Serde or OS types. Controlled Rust decoding is iterative: closed kind/of fields, borrowed traversal, cumulative ownership charging per ValueType, controlled String construction, checkpointed work and guarded retirement of built children. Encoding builds owned DslValue through NativeEncodeControl and guarded output. The original semantics must remain: null never matches, Decimal matches Boolean/Integer/Decimal, Any matches non-null, list matches Dictionary(Some("list")), schema matches its exact dictionary schema, decimal ID remains "number" and any ID "value". Preserve strict malformed/duplicate/extra-field refusal and all seven wire variants, including nested lists.

ValueType still publishes uncontrolled ToValue::to_value and FromValue::from_value methods alongside controlled ones. Its uncontrolled list conversion recursively constructs/clones nested values, so controlled-path success does not prove every public entry point bounded. TS readValueType requires a checkpoint control and rejects cycles, accessors, inherited/nonenumerable or symbolic hidden keys; the callback can cancel by throwing, but its schema currently has no allocation bound. Keep these actual contracts separate from a promised fully bounded type API.

Diagnostic TextError, FaultCode, Severity, Diagnostic and Fault use hand-written owned ToValue/FromValue conversions. Their current generic controlled defaults fail closed (“owner has no controlled construction/encoding implementation”), rather than falling back to unchecked conversion. Simply routing fault bytes through a mandatory controlled JSON codec will therefore reject these owners until real controlled Diagnostic implementations are supplied. Implement the floor’s controlled scalar/span/list/cause construction and iterative retirement first; do not add a generic fallback, default unlimited control, foreign exemption or compatibility adapter.

## Laws and Caller Cut

Preserve Value JSON projection, numeric precision/nonfinite behavior, byte representation, ordered-map ordering/paging/retirement, neutral-owner mirrors, retained-clone/retirement, and Type classification/wire/refusal/deep controlled rows. Their foreign libraries remain independent test oracles and can be dev-only after removing shipped obligations. Removing DslValue conversions requires updating implicit `.into()`, `DslValue::from`, cross-type assertions and serialized containing types; deleting the reexports alone cannot close that boundary.

Diagnostic fault-describe tests currently parse a language-neutral corpus with serde_json, deserialize DslValue and independently decode encoded fault bytes. Retain all expected wire fields, descriptions and fallback/refusal assertions while providing a test-owned projection oracle. TextSpan’s owned conversion already provides the neutral field shape; preserve original bounds and diagnostics when implementing its controlled path. The normal fault-byte fallback uses message bytes on encode failure and constructs an OS-origin `os.fault.decode` fault for malformed bytes; changing either requires an explicitly authored new closed law, not an incidental codec swap.

Fullbytes owner sources, schemas and current direct consumer files: `🗑️generated/current-framework-io-audit/value-diagnostic-current-full-sources.json`. Exact names/line spans and direct consumer matches: `value-diagnostic-current-api-census.json`; TS production imports: `value-diagnostic-production-ts-import-census.json`; direct Rust file roster: `value-diagnostic-direct-rust-roster.txt`. No native checks were run and no whole workspace absence claim is made.

## Pinned Main Source Hashes

- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native/🌱️value/🦀️.rs`: `b258a668a5242922ffe5bbccbe12ae7d409fe38aead2196cf68a2ab4b573b3cf`
- `✏️s/🔌️plugins/🧩️puzzle/🌉️wasm/⚠️diagnostic/🦀️.rs`: `66f09ffdad0068380399da73192f81c2c4dad597a73bcde6eec858b3fc8a1bdf`
- `🧰️framework/🔨️modules/⚠️diagnostic/📍️span/🦀️.rs`: `c682fdbbf6f3cd9a3180e19f6fb052c026b89322ded1ec2b0e04467f7be58d2c`
- `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs`: `d95e21780a9d205e4b39dc00137c24effbf758c1e6171bea7fb8443d0e7e527d`
- `🧰️framework/🔨️modules/🌱️value/🏷️type/🛬️controlled/🦀️.rs`: `e8eea243f7b839e3a9a8991362f74ac0ff15d72d02fa896758ebc3f3627ba6a8`
- `🧰️framework/🔨️modules/🌱️value/🏷️type/🦀️.rs`: `d4e3962cf6cbe52363f63af9113e4c1295d8587cea59d8ee6687f7843455f8ef`
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs`: `9116f2cfbbd1d441df1411c705420a80abc9669df267535d5b6dbe2ada86593d`
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs`: `ee5efe8b9430b52e3bb4706e6ab06560a426b9afe490da0b16e5491ddb9d7055`
- `🧰️framework/🔨️modules/🌱️value/🦀️.rs`: `82ab3779de00ed4f965ff156577469f9c8b1363f1efbac2cefa315d9f256067c`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs`: `d7a7388141fc5d1c725b6493e92260fba927699207a88847260dea4749212e18`
