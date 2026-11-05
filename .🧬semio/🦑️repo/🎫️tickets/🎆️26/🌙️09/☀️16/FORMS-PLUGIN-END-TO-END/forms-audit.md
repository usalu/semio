# Forms Read-Only Audit

Forms owner: `✏️s/🔌️plugins/📋️forms`. Read-only exploration on 2026-11-04; no source edits or tests executed. All commands entered through repo. Applicable instructions are root AGENTS.md and the descriptive ✏️s/AGENTS.md; no forms-local AGENTS.md appeared in the bounded inventory.

## Ownership and Contracts

Artifact owner A is `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms`; subset S is A/🏅️standards/🔖️1/🪆️subsets/✳️any. The 657-file plugin inventory contains production Rust, TypeScript twins, JSON Schema, GraphQL, Protobuf, physical IO, fixtures, and split Rust packages. A/🦀️.rs is a large facade over the split crates; A/🟦️.ts exports artifact parsing, intrinsic model types, JSON and SQLite surfaces.

S/🧬️schema/🔣️.json requires schema, id, version, definition, responses, structure and results. Structure/results are independent child references to stdio semio value/table dialects. Definition owns ordered steps and blocks; immutable response records preserve submission time, definition version, original question labels, kinds and values. Document mutations include structural changes, answer commit/discard, inverses and diffs. The bridge inventories production mutation descriptor aggregates for document and editor config lanes.

## Runtime and UI

S/✏️editor/🦀️.rs dispatches 29 commands. Its 🕹️play-app/🦀️.rs mounts window config/transient owners, action decoding, retained job factories, media IO, rendering and interaction topology. Blueprint builder, Fill Form wizard, Responses history, catalogue, document tree and inspector already exist. Try values are exact window-local transient owners; current step and submitted response id are window config. Submit validates all visible fields and emits CommitResponse. Reset and repeat submission, response discard, JSON/CSV export, selection and extension slots already have implementations.

All command names are registered in 🧵️retained-commands/🦀️.rs. Response export advances incrementally with progress. Store preparation in 📬️store-preparation/🦀️.rs admits mutation byte envelopes, then computes inverse/diff/application synchronously during one advance; base-size cost deserves separate bounded-work review. This is an observed implementation shape, not a measured performance failure.

The Try renderer assigns accessible field labels and explicit sibling keys, routes every answer to concrete windowId/windowKindId and binds change/delta actions. Render with request context uses actual transient answers; plain render uses an empty transient fallback.

## Extensions

S/✏️editor/❓️questions/🧩️extensions owns schemas and TS/Rust payload twins. Topic forms.questionKind contributes appId, questionKind, localized labels, icon, paramsBodyKey, previewBodyKey and default JSON. S/✏️editor/🧩️contributions/🦀️.rs resolves contributions for catalogue, inspector and Try. Both provider slots receive authored exampleId, exact current params, question/controller identity, surface, interactive flag and concrete Try window routing. Explicit null answers are preserved. Missing providers render localized unavailable diagnostics.

## Actual Missing Behavior and Exact Change Owners

The clearest functional gap found is vector validation: numeric fields expose min/max/step in the Try NumberStepper, but S/🧬️schema/✅️validation/🦀️.rs accepts every finite vector component of the correct arity without checking the question min/max/step. Scalar number/slider validation enforces those constraints in the same function. Consequently a directly supplied or imported vector answer outside authored bounds can pass step_errors and durable submission. Inspect and update the vector branch in both S/🧬️schema/✅️validation/🦀️.rs and its 🟦️.ts twin together; first extend shared vectors and both language suites under S/🧬️schema/✅️validation/🧪️tests (inventory confirms Rust and TS suites there). Keep range/step semantics consistent with scalar constraints and validate expected outputs using an independent test oracle. This conclusion is from direct source inspection; no failing test was run by this explorer.

A smaller localization gap is the literal Unsupported kind: diagnostic in S/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs. Move it through S/✏️editor/🗣️terminology/🦀️.rs when addressing multilingual diagnostic coverage.

Do not infer absent import/export features solely from forms_io empty import_formats/export_formats: exportSnapshot/exportResponses and separate serializer implementations already exist. Inspect intended host integration before changing those lists.

## Test Routing

Projects are @semio-tech/forms-js and @semio-tech/forms-forms-rs in A/📦️packages/{🟦️typescript,🦀️rust}/📋️project.json. Both expose build/check/test. TypeScript adds verify-physical-codecs. Rust adds verify-forms-document-contract and test-snapshot-sqlite with native/source variants. S/🧬️schema/🧪️tests/🪪️document/🟦️.ts already compares native parsing with Ajv JSON Schema and diff outcomes with fast-json-patch. Shared JSON fixtures and paired suites cover response events, authoring, extension payloads and placement. Framework renderer coverage exists in 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📋️renderer-types-and-forms/🟦️.ts. No tests were run during this read-only audit.
