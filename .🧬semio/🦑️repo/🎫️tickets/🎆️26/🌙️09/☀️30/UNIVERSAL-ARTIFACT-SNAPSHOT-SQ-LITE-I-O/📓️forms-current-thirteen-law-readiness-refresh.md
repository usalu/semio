# Forms Current Thirteen-Law Readiness Refresh

Read-only current source inspection, 2026-10-04. No Source, compiler, Cargo, or Native execution occurred. The full owning Forms13 gate remains pending; this report identifies source prerequisites rather than claiming a fresh compiler diagnosis. Prior detailed inventories are `📓️forms-exact-canonical-qualified-caller-roster.md` and `📓️forms-current-public-constructor-validity-audit.md` in this ticket.

Paths below are relative to `/Users/ueli/Documents/semio`. Let F be `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms`; A is F followed by `/🏅️standards/🔖️1/🪆️subsets/✳️any`.

| Current prerequisite | Exact location | Canonical pairing |
| --- | --- | --- |
| Two absent direct dependencies | F/📦️packages/🦀️rust/Cargo.toml, dependencies beginning line18 | `semio-framework-dsl-record`, path `../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust`; `semio-framework-dsl-record-derive`, path `../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust`. Current production code already names both crates; generic semio-framework-dsl is a distinct dependency. |
| Five private product-facade references in one production file | A/🧬️schema/🧾️dictionary/🦀️.rs:5,8,14,40,41 | Two `dsl::DslRecord` derives belong to `semio_framework_dsl_record_derive::DslRecord`; three `dsl::NativeSchemaControl` bounds belong to `semio_framework_dsl_record::NativeSchemaControl`. |
| Twenty-five private Value references in one owning test file | A/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:3,12–20 | One ToValue import, eighteen DslValue occurrences, six Number occurrences belong to `semio_framework_value`. Keep the complete comparator: float bits, bytes, array order, and ordered duplicate object occurrences. |

This is a narrow current roster: thirty residual qualified references in two files plus two manifest bindings, not the previous 492-reference migration inventory. Comments mentioning DslOps or DslValue are not blockers. Current Forms has no executable DslOps derive or field_error call found. Authored OpText/OpBinary implementations remain in mutation text, config mutations, try transient/config, and viewer owners; restoring DslOps would bypass that authored authority.

## Derive Attributes

FormsConfig at A/✏️editor/🎚️config/🦀️.rs:3–7 is already paired: canonical Record derive, public kernel DslArtifact, `#[artifact(extension = "formscfg")]`, and `#[artifact(id = "forms.config")]`. Try-window config likewise uses the product artifact attributes. The product macro declaration is `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs:16`, `proc_macro_derive(DslArtifact, attributes(artifact))`.

Two stale `#[dsl(extension = "forms")]` annotations remain: A/🧬️schema/📸️snapshot/🦀️.rs:17 and A/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:35. Canonical Record derive's container parser (`🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs:84–106`) does not implement extension and discards the nested parse result. These are ignored stale annotations, not established compile errors. Remove obsolete vocabulary coherently; do not add artifact derivation to the internal Pack Document projection merely to replace its annotation. Snapshot's authored ArtifactDsl/Pack authority must remain intact.

## Constructor Validation Follow-Up

F/🦀️.rs:361–364 still exposes `forms_snapshot_with_state(schema:String,id:String,version:String,title:Option<String>,steps:&[FormStep])->FormsSnapshot`. It creates children and clones steps without validation. A/🧬️schema/📝️definition/🦀️.rs:11–27 has actual guards for nonempty unique step/question identities, nonempty question kind, finite numeric bounds/vector values, ordered ranges, positive step, unique nonempty option values/vector keys, and Object extension parameters. Snapshot FromValue at A/🧬️schema/📸️snapshot/🦀️.rs:65 calls validate; snapshot validate:72–77 guards definition and response identities. This does not guard direct public construction.

The meaningful follow-up remains a public fallible constructor returning the canonical typed ValueError after validating the complete snapshot, with all existing callers paired directly. Keep valid complete Value literals unchanged; finite constraints apply only to authored question constraint fields. Test invalidSteps, NaN/infinite bounds and field values, reversed ranges, nonpositive steps and invalid identities through the public constructor; include valid literal/word cases. This semantic follow-up is separate from the measured compiler prerequisites and must not be used to lower the thirteen existing laws.

## Owning Gate

F/📦️packages/🦀️rust/📜️script.ts:28 mounts runArtifactRustPackageMain for `semio-s-artifact-forms-forms`, including the actual snapshot SQLite Source owner. The existing owning package target is `@semio-tech/forms-forms-rs:test-snapshot-sqlite-native`, invoked through Bun/Nx with the shared runner's `sqlite_snapshot_` lib selection. Parent owns the unchanged full thirteen-law replay and required isolated environment. After the named prerequisites are paired, that runtime replay must establish the next actual result; this source audit supplies neither compile success nor an allocator/whole-module proof.
