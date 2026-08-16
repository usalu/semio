# STEP Baseline Audit

## Scope

Read-only source audit of the current `stdio.step` artifact before Wave 4 implementation. This audit records current-tree facts only; it does not claim standards support or runtime closure.

## Current representation

- The mounted public tree is organized around a single `ap214` standard directory with `any` and `cc1` through `cc6` subsets.
- The schema model is a generic Part 21 record/value snapshot rather than an EXPRESS-compiled typed schema graph covering public ISO 10303 application protocols.
- Routing and validation contain hard-coded `AUTOMOTIVE_DESIGN` / AP214 checks rather than selecting a compiled schema from the actual `FILE_SCHEMA` declaration.
- The implementation exposes editor/viewer/schema/IO facets for the AP214 subset ladder, but no definition-driven catalog of public APs, editions, conformance classes, or physical representations is mounted.

## Open/closed violations

- The artifact root installs five `dsl::passthrough_hooks` language surfaces (`stdio.step`, operation, diff, pack, and SPR). These are not real STEP/EXPRESS parser-printer implementations.
- The mutation root contains `NoMutation`, `SetSnapshot`, generic `SetFile*`, `SetEntityName`, and `SetEntityArg` variants in one central enum and central text/binary dispatch.
- CC subset builders perform whole-snapshot replacement through `StepMutation::SetSnapshot`.
- Inverse paths substitute `NoMutation` when a target is absent instead of returning a typed rejection.
- The diff codec retains full-snapshot encoding helpers specifically for `SetSnapshot`.
- Schema and mutation behavior is aggregate in large root files rather than one public semantic leaf per inference or command.

## Required Wave 4 replacement boundary

1. Lossless ISO 10303 physical syntax model preserving exact decimals, lexical form, trivia, ordering, anchors, unknown records, and untouched bytes.
2. EXPRESS compiler and runtime for entities, inheritance, defined/select/enumeration types, aggregates, inverse/derived attributes, WHERE/UNIQUE rules, functions, procedures, and subtype constraints.
3. Definition-owned registry of every supported public AP/schema/profile/edition and physical representation, routed from actual `FILE_SCHEMA` values.
4. Typed resource/diagnostic/budget/cancellation contracts and deterministic lossless/canonical encoders in Rust and TypeScript.
5. Atomic inference leaves for identity/conformance/inventory/dependencies/security/lossiness plus geometry, assemblies, units, materials, PMI/GD&T, and topology.
6. Atomic semantic mutation triads with typed rejection, direct sparse diffs, inverse reconstruction, touched paths, reference repair, replay, and undo/redo laws. `NoMutation`, `SetSnapshot`, and generic `Set*` vocabulary must be absent.
7. Reuse the stronger Semio BREP kernels behind internal interfaces; public STEP contracts must not expose implementation-library types.

## Gate status

No STEP build, codec round trip, validator, or corpus gate was run in this audit. The artifact is not closed against the umbrella acceptance criteria.
