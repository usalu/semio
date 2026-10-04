# Current Office canonical prerequisites

Read-only refresh after the supplied Rewriting12 baseline stopped before assertions with five DOCX compiler errors. No compiler, Source test or runtime was run in this audit. Current concurrent source repairs supersede four of those measured errors: DOCX native imports `NativeEncodeControl`, `NativeDecodeControl` and root `DecodedValue` from `semio_framework_value`; DOCX SQLite projection imports the Value control. The fifth measured error remains in DOCX subset code.

## Exact current production pairs

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs:4`: remove `NativeEncodeControl` from the kernel grouped import and import `semio_framework_value::NativeEncodeControl`. Existing `&mut NativeEncodeControl<'_>` parameters and `new`/stage/checkpoint calls remain unchanged.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛡️subset/🦀️.rs:8`: remove `NativeEncodeControl` from the kernel grouped import and import `semio_framework_value::NativeEncodeControl`. Existing `&mut NativeEncodeControl<'_>` parameters and `new`/stage/checkpoint calls remain unchanged.

The PPTX subset occurrence is the same concrete private-facade issue, although the preceding compiler stopped in DOCX. Both subset files belong to the actual base snapshot module rather than an unmounted draft. DOCX, PPTX and XLSX Cargo manifests already directly bind `semio-framework-value`; these two pairs need no added dependency or alias. Canonical controls and `DecodedValue` are Value exports, while `SqliteSnapshotControl`, `SqliteSnapshotPhase`, `ArtifactSqliteSnapshot`, `SnapshotEncoding`, database limits and payload remain kernel-owned.

## Selected scope and current exclusions

DOCX native and projection repairs are already correct; exclude them from a new capsule. XLSX native/backing/reconstruction use canonical Value controls/root DecodedValue. PPTX native/backing and transform use canonical Value controls and errors. Current subset diagnostics are authored `Diagnostic { code: FaultCode::new(...), severity, span: TextSpan::at(1,1), message, expected, scope }`; no removed `__rt::field_error` was found in these Office roots. No Record/Record-derive dependency change is justified by an active derive in this narrow inspection. PPTX already has both direct Record dependencies.

OPC is a ZIP child authority under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs`; there is no separate OPC Cargo artifact discovered. This inspection found no direct retired kernel Native/DecodedValue or dsl::DslValue qualification in the OPC source path scan. This is bounded lexical source readiness, not a compiler or whole module proof.

## Conditional own-package tests

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:1036` still uses `dsl::DslValue::String` inside `#[cfg(test)] fn demo_mutation_cases`; root `dsl` aliases the kernel. Pair to `semio_framework_value::DslValue::String` only when executing that owning package's tests. Rewriting12 compiles these crates as dependency libraries and does not activate dependency cfg(test).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:463` still uses `dsl::DslValue::String` inside `#[cfg(test)] fn demo_mutation_cases`; root `dsl` aliases the kernel. Pair to `semio_framework_value::DslValue::String` only when executing that owning package's tests. Rewriting12 compiles these crates as dependency libraries and does not activate dependency cfg(test).

Authored exact-before/after capsule: `📥️inputs/office-current-canonical-prerequisites.json`, two production pairs and two explicitly conditional test pairs. Preserve all validation/codec assertions and diagnostic fields. The prior zero-assertion failure and parser-only concurrent checks supply no runtime pass.
