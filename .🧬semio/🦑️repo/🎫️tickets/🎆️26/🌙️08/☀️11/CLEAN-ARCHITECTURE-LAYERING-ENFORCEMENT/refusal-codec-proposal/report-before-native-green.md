# Controlled Value Refusal Codec

The actual Value codec traits require unchecked to_value/from_value methods, with controlled defaults refusing UnsupportedOwner. This feature does not implement those traits, clone through an unchecked codec, or supply a default control. IoError's existing derive remains unmounted until the controlled-only trait/derive boundary is retired.

The canonical schema is Value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json, identity https://semio.tech/schema/value/refusal/codec. Its wire definition is a closed record with mandatory kind and message. It preserves all eight existing camelCase kinds and complete Unicode/path prose; unknown, duplicate, extra, missing and incorrectly typed fields are invalidValue.

Eight valid and eight invalid language-neutral rows were authored before production. Rust tests require inherent methods taking actual NativeEncodeControl and NativeDecodeControl; TypeScript uses the intrinsic object/text representation and a mandatory first-party control interface. NativeDecodeControl structurally supplies that interface. No uncontrolled entry point or public external type is introduced. Ajv and SQLite JSON are independent portable references; serde is the independent native wire reference. Native tests remain unmounted through the bounded Semio source epoch.

Current status: schema/corpus/tests authored; no portable or native verdict yet. Production codec absent. Full actual existing source and registration originals are retained in refusal-codec-proposal/full-before.json.

Registered portable test-first attempt 1 is terminal RED: strict TypeScript GREEN, independent Ajv/SQLite schema law 1 pass, actual absent codec module 2 failures, Nx 7.5 seconds. Log: 🗑️generated/refusal-codec/portable-red-1.log. The schema/corpus was then expanded with five explicit shared cancellation/ownership rows and a shared long-message construction, before any production mount. That expanded corpus is not yet rerun. The first-party TypeScript control contract now requires checkpoint, charge, step, advance, beginStage and scopedStage; stages restore the parent workload and retain cumulative admission.

Root SQLite production owns the current finite native epoch. Value Rust production and native test registration remain unchanged through its terminal. The Rust and TypeScript production input proposals are complete in refusal-codec-proposal/🦀️.rs and 🟦️.ts, but unmounted.

Whole original Value native test-only attempt is actual compiler RED7 E0599, no discovery/runtime; all178 source inputs and69 compiler-declared failed-test inputs are current. Native receipt: generated/current-native-worker/value-refusal-codec-test-only-red-1-compiler-bindings.json.

Canonical production now has four source rows with exact full inverses: two new Rust/TS codec sources, the private Rust mount preserving all three test-only law bodies, and strict explicit TypeScript production-source inclusion. No Cargo or trait implementation changed. The registered portable production run is strict/noUncheckedIndexedAccess GREEN then3/3 runtime GREEN,425ms runtime/3.7sNx, including the expanded21 shared rows. Both DEBUG transport/identity/cancellation observations executed. Log: 🗑️generated/refusal-codec/portable-production-1.log.

Production SourceReady1 has181 full inputs, including the Parent-declared coherent IO vocabulary Cargo manifest. Whole native Value production remains pending; IoError uncontrolled derives remain unmounted. GUI56 is authored but normal generated publication remains pending.
