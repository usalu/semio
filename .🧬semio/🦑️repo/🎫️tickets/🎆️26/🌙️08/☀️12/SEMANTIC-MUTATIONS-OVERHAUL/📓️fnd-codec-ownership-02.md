# FND-CODEC-OWNERSHIP-02

## Enforcement

The mutation structural policy now reports high `mutation/codec-ownership` breaches only for executable root aggregate codec behavior found by the Rust token inspector. It covers whole-aggregate serde serialization (`to_vec`, `to_string`, `to_value`, `to_writer`), aggregate deserialization (`from_slice`, `from_str`, `from_reader`, `from_value`) through explicit aggregate/type-alias/`Self` targets or local inferred aggregate result types, and explicit root aggregate variant match arms, including `Self::Variant` inside an aggregate impl.

Bindings are limited to each function and nested lexical block. Direct aggregate receiver/parameter/local bindings, obvious renamed imports, and local type aliases are followed. A generic wrapper such as `impl Wrapper<PageMutation>` is not treated as an aggregate impl. Generic framing bytes, leaf registry callbacks, metadata iteration, unrelated enums, comments, and string contents remain accepted.

## Bound

This is lexical source enforcement, not a Rust type checker. It deliberately does not resolve opaque cross-file aliases, macros/generated bodies, trait-selected serializers, dynamic dispatch, arbitrary result-flow inference, or non-obvious assignment/dataflow. The listed serde call paths and direct aggregate patterns are the packet boundary.

## Evidence

The pre-implementation TDD seam failure is retained in [the compile/import red log](🧪️fnd-codec-ownership-02-red.log). It was not a behavioral runtime red.

The independent nightly Rust AST parser, type-solver ownership oracle, unchanged inspector, and structural-policy harness agree on all 16 language-neutral vectors; see [isolated oracle evidence](🧪️fnd-codec-ownership-02-isolated.log), the replayable [ticket harness](🧪️codec-ownership-oracle/📜️script.ts), and its [stdout/stderr transcript](🧪️codec-ownership-oracle/🧾️stdout-stderr.log). The type solver uses only test-local `CodecAtom` bounds and deprecated aggregate variants; it does not reuse the TypeScript lexer. The inferred-result rule only follows a verified `return` or tail expression; an explicitly typed nonaggregate `let` is authoritative, as covered by the scalar framing-before-leaf-decode regression. The registered Nx repo-lib test is still pending because its module loader currently rejects unrelated missing jcoprobe generator outputs before tests execute; no taxonomy files were changed or fabricated. Its result will be added once the concurrent workspace state settles.

No production codec/root compose source was inspected or modified. The ticket's prior root-consumer audit identifies the known live codec bypass population; this packet adds enforcement only and does not repair those production sources.
