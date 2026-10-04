# Canonical DSL Macro Entrypoint Prerequisite Audit

Urgent read-only audit 2026-10-03. No Cargo, macro edits or production edits. Root actual Forms13 current gate reached compiler and reported four E0425 at OS proc-macro package root17/49/56/67, zero assertions, Nx21. These are compiler prerequisite diagnostics, not Forms domain failure.

## Exact competing authorities

OS compiled root `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs` selects component ../../🦀️.rs at3–4. It still registers DslRecord14, DslScalar46, DslOps53, DslEnum64, whose wrappers call now-absent expand functions17/49/56/67. Product implementation owner now starts with envelope/diff/mutation responsibilities; canonical_product_macro_tests is mounted at bottom1365.

Actual mandatory Source/native test contracts make this removal intentional. `✨️derive/🧫️fixtures/📤️macro-exports/🔣️.json` requires product root registeredDerives exactly CompositeMutation,DslArtifact,DslDiff,MutationLeaf,Mutations; facadeExports exactly DslArtifact,DslDiff,MutationLeaf,Mutations. Syn oracle in `✨️derive/🧪️tests/📤️macro-exports/🦀️.rs:17–32` parses both real root and real facade and compares exact rosters. Canonical record-owner fixture `✨️derive/🧫️fixtures/🪆️record-owner/🔣️.json` explicitly retires expand_dsl_record/scalar/enum/ops, record_codegen,dsl_variants_codegen. Actual test `✨️derive/🧪️tests/🪆️record-owner/🦀️.rs:28–38` prohibits those functions in product owner. Restoring the four missing wrappers or old generator would contradict these contracts, rather than repairing their pairing.

Current OS facade `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:32` still explicitly exports DslEnum,DslOps,DslRecord,DslScalar from dsl_derive. Thus deleting wrappers alone leaves unresolved imports next. These root/facade bindings are the safe exact prerequisite locations, paired with current consumer ownership; do not change the required export fixtures to bless retired APIs.

## Canonical implementation route

Generic owner `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs:8–17` exports DslRecord→emit_record, DslScalar→emit_scalar, DslEnum→emit_enum. It also exports record_binding20–21, record_projection24–25, variant_binding28–29 for explicit composition. Its package Cargo lib name is semio_framework_dsl_record_derive and proc-macro=true. Current real generic tests call this macro crate explicitly, including framework Value decode binding tests6/31/33. Generic runtime record package Cargo declares this macro only as a dev dependency; it does not currently reexport these macros as a public runtime facade. Do not assume a generic facade import already exists.

Product expand_dsl_document783–811 now emits only __DSL_ENVELOPE_ID/__DSL_EXTENSION. It requires #[artifact(id/extension)] and does not emit record binding. Compiled root22 currently permits attributes(dsl), so its helper-attribute registration also needs **attributes(artifact)** pairing. Consumers that previously relied on DslArtifact generating record bindings must derive canonical DslRecord separately and preserve full field attributes, controlled/retained constructors and lifecycle; ordinary envelope-only success cannot replace these obligations.

Product expand_dsl_diff815 onward emits DiffCodec over canonical __dsl_spec/__dsl_to_record/__dsl_from_record and actual OS pack_rt. It no longer owns generic record generation; full Diff consumers need canonical record binding separately. Keep product envelope and transport behavior intact.

DslOps is explicitly retired and **has no corresponding canonical DslOps export**. Generic DslEnum compiles the shared tagged variant contract; authored product OpText/OpBinary behavior remains separate. A blanket alias DslOps→DslEnum would keep a retired API and hide whether caller transport obligations are satisfied. Current consumer migration is visibly incomplete: rg found275 textual dsl::DslOps matches (comments included, not275 derives), with actual derives in workflow/run mutation enums, Trinity command/config enums, DAG config/presence and Block config/presence. Replace actual derives/imports through genuine canonical enum plus existing handwritten codec ownership, not message/record forwarding stubs.

## Concrete coherent repair sequence

1. Pair OS proc-macro root with exact five-product export roster by removing the four retired registration/wrapper entries. Pair OS DSL facade32 with the exact four-product facade export roster. Keep CompositeMutation direct macro registration as required by fixture, not a facade addition.
2. Correct DslArtifact helper attribute registration to artifact, in agreement with actual product parser. Preserve DslDiff attributes needed by its consumers and canonical record derive.
3. Update consumer imports/dependencies/derives to canonical semio_framework_dsl_record_derive::DslRecord/Scalar/Enum. Use authored canonical record_binding/variant_binding where product composition deliberately requires that seam; do not add old forwarding functions to OS component.
4. Validate exact export Source law and canonical product-owner laws through existing Bun/Nx route, then rerun authentic unchanged owning selection after source pairing. No removal of Forms13 assertions, feature lanes or typed scalar guards is authorized by this compiler prerequisite.

This spans an active generic/product authority migration. Root can safely identify the two stale macro export bindings, but a two-file deletion alone is not evidence all dependent artifact packages now compile. Shared authority should coordinate consumer pairing before the next owning lane; no universal blocker conclusion is drawn from these four compiler errors.
