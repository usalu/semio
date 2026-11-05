# Declared Record Shape Work

The existing owned receiving declared_record loop invokes DeclaredRecordShape::step but discards its actual last_turn_work on both success and refusal. The generated receiving_fields caller currently receives only Result, while ReceivingWork::take already preserves canonical outcome work before folding failures.

A shared recordShape corpus was appended to schema/🫴️receiving/📥️owned/🎟️work/📐️.json, and its existing native test file now checks successful/refused probes against independently decoded serde shapes, exact computational counts, denied item/depth grants, zero allocation, and unchanged input. Production is unchanged pending genuine red execution.

Gate 89544 ran the focused declared_record_work_matches_neutral_success_refusal_and_independent_serde test through ordinary repo Cargo, framework manifest, package semio-framework-dsl-record-model. It terminated 1 during source admission, before rustc or test bodies. Exact refusal: reason files, path schema/🛫️encode/🦀️.rs, observedBytes26755, maximumBytes37845, remainingFiles0, expectedBytes null. This is neither the expected work-counter failure nor evidence of a passing test. Raw output: generated/dsl-shape-work-red.log.

A bounded followup using existing readRustSourceClosureV1 under explicit conservative Linux lib/test cfg found lib28files253720bytes2466active-lines and lib-test42files306248bytes2732active-lines. Relative to unchanged32file/262144byte limits, lib-test is over by10files and44104bytes. This collector approximation is not compilation; its raw report is generated/dsl-shape-source-census.json.

## Current Closure Classification

Concurrent edits changed the fresh test snapshot to36 Rust files plus8 embedded JSON files (44total). Runtime fixture extraction cannot admit36 Rust files under the unchanged32-file boundary. No fixture conversion, source relocation, or cap change was applied. Root will plan the production ownership boundary; this worker switches to existing IO-control test adaptation.

Largest additional Rust test sources:
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️receive/🧪️tests/🦀️.rs: 8372 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎟️work/🧪️tests/🦀️.rs: 7289 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🧪️tests/🦀️.rs: 6852 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🔬️json-schema/🦀️.rs: 5678 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🎟️work/🧪️tests/📥️bank/🦀️.rs: 5105 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/📋️persistent/🧪️tests/🦀️.rs: 4504 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/🦀️.rs: 2056 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/📝️shared-utf8/🧪️tests/🦀️.rs: 1170 bytes

Embedded data:
- 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🧪️tests/🧫️fixtures/🔣️.json: 596 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧪️tests/🧫️fixtures/🪆️binding/📝️shared-utf8/🔣️.json: 130 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🧪️tests/🔣️.json: 2496 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎟️work/📐️.json: 2680 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/📋️persistent/🧪️tests/🧫️fixtures/🔣️.json: 354 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/scalar.json: 176 bytes
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️receive/🧪️tests/🧫️fixtures/📦️bytes.json: 108 bytes
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/🧪️tests/🧫️fixtures/🔣️.json: 7194 bytes

## Extracted Ownership Successor

After descriptor-owned production ownership extraction and runtime fixture custody repair, collector snapshot model lib-test32files258742bytes2147lines and ownership6files38603bytes624lines fit unchanged bounds. Gate28946 then passed target source admission and reached actual model lib-test rustc, but terminated1/Cargo101 on one existing fixture E0277 at receiving/🧪️tests/🦀️.rs:12: receiving_issued_grant()? applies Result propagation to an Option. Descriptor owns that explicit required-source refusal repair. No intended shape-work regression body ran; production declared_record remains unchanged. Log generated/dsl-shape-work-red-2.log.
