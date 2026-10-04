# Forms Exact Canonical Qualified Caller Roster

Read-only actual Forms owner inventory 2026-10-03. No production edits or Cargo. Machine roster is `🗑️generated/forms-canonical-qualified-reference-roster.json`, retained for Root requested review, with exact file/line/name/context/owner. Lexer masks line/nested block comments and ordinary/raw strings before locating exact dsl::Name/store::dsl::Name plus grouped use imports. It is lexical evidence, not Rust compiler name resolution or mounted runtime validation. No comment is counted as a failure.

## Direct canonical ownership and dependency pairing

| Current referenced vocabulary | Canonical crate / required direct dependency | Interface pairing |
|---|---|---|
| DslRecord,DslScalar (derive) | semio-framework-dsl-record-derive (lib semio_framework_dsl_record_derive) | Add direct normal dependency for production derives; replace actual macro paths. Do not resurrect OS macros. |
| DslField,DslVariants,FieldValue,Shape,RecordValue,RecordSpec,RecordSpecProducer,NativeSchemaControl,ParseOptions,SourceMode,JoinMode,parse,print | semio-framework-dsl-record | Add direct normal runtime dependency; explicit named imports. DslField/DslVariants are binding owner public exports; producer41 publishes control and spec producer. |
| DslValue,Number,ValueError,ToValue/FromValue runtime traits,NativeDecodeControl,NativeEncodeControl | semio-framework-value | Existing Forms direct dependency; use its actual public types. Keep same control references through record/SQL forwarders. |
| ToValue/FromValue in derive context | semio-framework-value-derive | Existing direct dependency; runtime traits and derive macros must not be confused. |
| DslArtifact,MutationLeaf,Mutations | OS product dsl_derive macro owner, kernel actual public product exports | Preserve envelope/mutation product domain. No new generic product reexport dependency is needed. |
| actual DslOps at config mutations11 | retired API; generic DslEnum plus existing authored product OpText/OpBinary | Replace actual derive, preserving two variant keywords and full current transport impls. Do not add a DslOps alias. |
| os_identity/os_pack | semio-framework-os-kernel | Existing direct dependency; genuine OS service/codec use stays product-owned. |
| __rt::field_error at config mutations29 | removed helper; semio-framework-diagnostic::TextError | Existing direct diagnostic dependency; construct typed InvalidValue with meaningful span rather than resurrecting field_error. |

Required new normal Cargo dependencies are exactly record and record-derive in Forms own 📦️packages/🦀️rust/Cargo.toml, with handwritten existing relative paths to framework 🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust and its ✨️derive package. Existing Value/Value-derive/Diagnostic/kernel entries remain adequate for their named owner vocabulary. Do not add runtime external libraries, compatibility kernel reexports or package-root shims. Public generated code calls canonical Record/Value/Diagnostic paths directly, so dependencies must be direct, not merely transitive kernel dependencies.

## Actual caller/API readiness

Forms root6 aliases kernel as dsl,7 as protocol,8 as store. Kernel root300–301 and OS DSL26–29 now private canonical imports; no generic external macro export exists. Config mutations owns real OpText/OpBinary and DslVariants operations. Canonical Cst is RecordValue alias440, so Record.parse return does not require a new conversion adapter. Both scalar tags and record attributes must keep actual helper vocabulary rather than product envelope helpers.

Forms native flattened records 🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs3 imports four obsolete kernel generic names and Octets5–12 has full controlled public DslField methods plus Shape/NativeSchemaControl. Move only bindings; preserve copy_bytes admission, InvalidValue kinds and word/octet fields. Kind4/ConditionKind18 use scalar derives; Member14,Value15,Condition19 and all typed document entities use Record. Document32 #[dsl(extension="forms")] is not generic envelope authority (canonical container parser84–107 handles keyword/layout/retire_with); authored native Forms framing owns the actual envelope.

Schema, dictionary and validation consumers use full Value Number words, exact ordered Object members and controlled constructions. Existing nine-value/finite-bound validations and constructor laws must not be lowered or replaced with String carriers during binding repair. Snapshot Pack projection/reconstruction, SQL native forwarders and public ArtifactDsl/ArtifactPack stay actual owner routes; lexical binding repair alone does not qualify their allocation or runtime behavior.

One local exception is resolved: Forms binary unit test imports crate::document_dsl as dsl at2. Its dsl::parse_dsl and three example constants are local Forms document APIs, **not** kernel generic migration defects. The machine roster flags local_alias=true. No store::dsl::Name qualified occurrence was found in this current snapshot of actual Forms source; grouped and ordinary dsl paths are listed explicitly.

## Name summary and exact references

- `BUILDING_COMPONENT_EXAMPLE_TEXT`: 1 lexical references; owners: Forms document_dsl local module
- `DEFAULT_EXAMPLE_TEXT`: 1 lexical references; owners: Forms document_dsl local module
- `DslArtifact`: 2 lexical references; owners: dsl_derive product macro owner
- `DslField`: 3 lexical references; owners: semio_framework_dsl_record
- `DslOps`: 1 lexical references; owners: retired DslOps; canonical DslEnum plus authored Op codecs
- `DslRecord`: 47 lexical references; owners: semio_framework_dsl_record_derive
- `DslScalar`: 2 lexical references; owners: semio_framework_dsl_record_derive
- `DslValue`: 153 lexical references; owners: semio_framework_value
- `DslVariants`: 4 lexical references; owners: semio_framework_dsl_record
- `FieldValue`: 23 lexical references; owners: semio_framework_dsl_record
- `FromValue`: 65 lexical references; owners: semio_framework_value, semio_framework_value_derive
- `JoinMode`: 4 lexical references; owners: semio_framework_dsl_record
- `MutationLeaf`: 15 lexical references; owners: dsl_derive product macro owner
- `Mutations`: 2 lexical references; owners: dsl_derive product macro owner
- `NativeDecodeControl`: 9 lexical references; owners: semio_framework_value
- `NativeEncodeControl`: 4 lexical references; owners: semio_framework_value
- `NativeSchemaControl`: 1 lexical references; owners: semio_framework_dsl_record
- `Number`: 32 lexical references; owners: semio_framework_value
- `ONBOARDING_EXAMPLE_TEXT`: 1 lexical references; owners: Forms document_dsl local module
- `ParseOptions`: 4 lexical references; owners: semio_framework_dsl_record
- `RecordSpec`: 5 lexical references; owners: semio_framework_dsl_record
- `RecordSpecProducer`: 1 lexical references; owners: semio_framework_dsl_record
- `RecordValue`: 8 lexical references; owners: semio_framework_dsl_record
- `Shape`: 10 lexical references; owners: semio_framework_dsl_record
- `SourceMode`: 4 lexical references; owners: semio_framework_dsl_record
- `ToValue`: 51 lexical references; owners: semio_framework_value, semio_framework_value_derive
- `ValueError`: 24 lexical references; owners: semio_framework_value
- `__rt`: 1 lexical references; owners: removed field_error helper; canonical diagnostic TextError
- `os_identity`: 2 lexical references; owners: kernel OS authority
- `os_pack`: 1 lexical references; owners: kernel OS authority
- `parse`: 4 lexical references; owners: semio_framework_dsl_record
- `parse_dsl`: 3 lexical references; owners: Forms document_dsl local module
- `print`: 4 lexical references; owners: semio_framework_dsl_record

| File/line | Actual path | Context | Canonical owner |
|---|---|---|---|
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:58` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:186` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:186` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:190` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:190` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:196` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:196` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:197` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:199` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🦀️.rs:199` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧩️extensions/🦀️.rs:36` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🧩️extensions/🦀️.rs:36` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:8` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:10` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:38` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:43` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:43` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:44` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/❓️questions/🫥️visibility/🦀️.rs:44` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:3` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:3` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:3` | `dsl::DslArtifact` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:18` | `dsl::parse` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:18` | `dsl::ParseOptions` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:18` | `dsl::SourceMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:22` | `dsl::print` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:22` | `dsl::JoinMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:42` | `dsl::RecordSpec` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs:2` | `dsl::os_pack` | runtime_or_import/qualified | kernel OS authority |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🦀️.rs:5` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace/🦀️.rs:5` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:11` | `dsl::DslOps` | derive/qualified | retired DslOps; canonical DslEnum plus authored Op codecs |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:11` | `dsl::Mutations` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:22` | `dsl::DslVariants` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:25` | `dsl::parse` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:25` | `dsl::ParseOptions` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:25` | `dsl::SourceMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:26` | `dsl::DslVariants` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:29` | `dsl::__rt` | runtime_or_import/qualified | removed field_error helper; canonical diagnostic TextError |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:32` | `dsl::DslVariants` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:33` | `dsl::DslVariants` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:35` | `dsl::print` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs:35` | `dsl::JoinMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧩️set/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧩️set/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧩️set/🦀️.rs:5` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧩️set/🦀️.rs:5` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:5` | `dsl::DslArtifact` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:58` | `dsl::parse` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:58` | `dsl::ParseOptions` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:58` | `dsl::SourceMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:62` | `dsl::print` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:62` | `dsl::JoinMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs:85` | `dsl::RecordSpec` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs:44` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs:70` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs:204` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧬️schema/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧬️schema/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:94` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:95` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:96` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:96` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:96` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:100` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:101` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:101` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:102` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:102` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:103` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:106` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:107` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:107` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:110` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:110` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:111` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:121` | `dsl::DslField` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:122` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:122` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:122` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:122` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:123` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:124` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:124` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:124` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:126` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:127` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:130` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:133` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:338` | `dsl::RecordSpec` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🧬️schema/🦀️.rs:6` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🧬️schema/🦀️.rs:6` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🧪️tests/🔬️unit/🦀️.rs:13` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🧪️tests/🔬️unit/🦀️.rs:42` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🦀️.rs:39` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🦀️.rs:40` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🦀️.rs:41` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🧪️tests/🦀️.rs:26` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️move-step/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️discard-response/🦀️.rs:6` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️discard-response/🦀️.rs:6` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️discard-response/🦀️.rs:6` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️reset-try/🦀️.rs:11` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️next-step/🦀️.rs:12` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/◀️previous-step/🦀️.rs:9` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️update-form/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🦀️.rs:9` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🦀️.rs:30` | `dsl::os_identity` | runtime_or_import/qualified | kernel OS authority |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:10` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:10` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:11` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:11` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️submit/🧪️tests/🦀️.rs:59` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-step/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❓️add-question/🦀️.rs:11` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⭕️remove-question-option/🦀️.rs:17` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️patch-question-options/🦀️.rs:10` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:73` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:74` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:74` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:77` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:78` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:78` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:79` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:79` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:80` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:85` | `dsl::DslField` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:86` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:86` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:87` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:87` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:88` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:89` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:95` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:111` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs:310` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📃️add-step/🦀️.rs:9` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️add-vector-field/🦀️.rs:23` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs:9` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-responses/🦀️.rs:6` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-responses/🦀️.rs:6` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-responses/🦀️.rs:6` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-active-example/🦀️.rs:11` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔘️add-question-option/🦀️.rs:20` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔣️set-spec-json/🦀️.rs:23` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-try-values/🦀️.rs:10` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️remove-question/🦀️.rs:9` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-question/🦀️.rs:10` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚫️remove-vector-field/🦀️.rs:19` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🦀️.rs:11` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🦀️.rs:34` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-questions/🦀️.rs:35` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩺️patch-vector-field/🦀️.rs:27` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️drop-question-kind/🦀️.rs:22` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧬️schema/🦀️.rs:2` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧬️schema/🦀️.rs:2` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧱️controls/🦀️.rs:95` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧱️controls/🦀️.rs:101` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧱️controls/🦀️.rs:101` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🫥️visibility/🦀️.rs:41` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🫥️visibility/🦀️.rs:41` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🫥️visibility/🦀️.rs:41` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🫥️visibility/🦀️.rs:41` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🫥️visibility/🦀️.rs:47` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:335` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:335` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:344` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:346` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:346` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:347` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:347` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:347` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:347` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:349` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:349` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:350` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:350` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:359` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:359` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:360` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:361` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:377` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:382` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:385` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:385` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:389` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:419` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:421` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1158` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:88` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:121` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:153` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:156` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:167` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:167` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:169` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:169` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:214` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:214` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:214` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:214` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:454` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:477` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:6` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs:34` | `dsl::RecordSpec` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:15` | `dsl::parse_dsl` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:15` | `dsl::BUILDING_COMPONENT_EXAMPLE_TEXT` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:23` | `dsl::parse_dsl` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:23` | `dsl::DEFAULT_EXAMPLE_TEXT` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:31` | `dsl::parse_dsl` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs:31` | `dsl::ONBOARDING_EXAMPLE_TEXT` | runtime_or_import/qualified | Forms document_dsl local module |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:34` | `dsl::parse` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:34` | `dsl::ParseOptions` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:34` | `dsl::SourceMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:40` | `dsl::print` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs:40` | `dsl::JoinMode` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs:3` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs:6` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs:6` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:15` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:15` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:17` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs:17` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🦀️.rs:4` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🦀️.rs:4` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🦀️.rs:4` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🦀️.rs:23` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:6` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:6` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:6` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:12` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:15` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:15` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:15` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🦀️.rs:49` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📤️projection/🦀️.rs:34` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📤️projection/🦀️.rs:34` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📤️projection/🦀️.rs:34` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📤️projection/🦀️.rs:5` | `dsl::DslValue` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📤️projection/🦀️.rs:5` | `dsl::NativeEncodeControl` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📥️reconstruction/🦀️.rs:29` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📥️reconstruction/🦀️.rs:30` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📥️reconstruction/🦀️.rs:31` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📥️reconstruction/🦀️.rs:6` | `dsl::DslValue` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/📥️reconstruction/🦀️.rs:6` | `dsl::NativeDecodeControl` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:11` | `dsl::RecordSpec` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:12` | `dsl::RecordSpecProducer` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:13` | `dsl::NativeEncodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:13` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:16` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:18` | `dsl::NativeEncodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:20` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:20` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:20` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:20` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:21` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:21` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:21` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:21` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:22` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:22` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:22` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:24` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:24` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:29` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:29` | `dsl::FieldValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:32` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:32` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:35` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:37` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:40` | `dsl::RecordValue` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs:40` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:4` | `dsl::DslScalar` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:7` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:7` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:8` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:8` | `dsl::Shape` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:14` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:15` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:18` | `dsl::DslScalar` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:19` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:22` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:23` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:24` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:29` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:30` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:31` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:32` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:3` | `dsl::DslField` | runtime_or_import/grouped_import | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:3` | `dsl::FieldValue` | runtime_or_import/grouped_import | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:3` | `dsl::NativeDecodeControl` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🧬️records/🦀️.rs:3` | `dsl::NativeEncodeControl` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:15` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:15` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:41` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:42` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:42` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:51` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:53` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:54` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:55` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:56` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:57` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:58` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:59` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:60` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:61` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:64` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:65` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:3` | `dsl::ToValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:12` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:12` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:13` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:13` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:14` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:14` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:14` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:14` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:15` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:15` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:15` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:15` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:16` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:16` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:16` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:16` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:17` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:17` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:19` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:19` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:20` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs:20` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:16` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:18` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:26` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:62` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:62` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:82` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:82` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:90` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🦀️.rs:5` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🦀️.rs:29` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🦀️.rs:30` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🦀️.rs:31` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🦀️.rs:68` | `dsl::NativeDecodeControl` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📏️admission/🪪️identities/🦀️.rs:3` | `dsl::NativeSchemaControl` | runtime_or_import/qualified | semio_framework_dsl_record |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:13` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:81` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:81` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:82` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:6` | `dsl::DslValue` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/📥️reconstruction/🦀️.rs:6` | `dsl::NativeDecodeControl` | runtime_or_import/grouped_import | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:5` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:19` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:19` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:19` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:21` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:21` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:21` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:10` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:42` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:43` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:43` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:52` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:54` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:55` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:56` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:57` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:58` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:59` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:60` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:61` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:62` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:66` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:91` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:91` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:98` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:98` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:108` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:108` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:120` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:120` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:25` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:50` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:51` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:51` | `dsl::ValueError` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:52` | `dsl::FromValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:199` | `dsl::os_identity` | runtime_or_import/qualified | kernel OS authority |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:211` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:216` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:222` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:228` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️discard-response/🦠️mutation/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️discard-response/🦠️mutation/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️discard-response/🦠️mutation/🦀️.rs:5` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-step/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🦠️mutation/🦀️.rs:12` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🦠️mutation/🦀️.rs:12` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-block/🦠️mutation/🦀️.rs:12` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-block/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦠️mutation/🦀️.rs:12` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦠️mutation/🦀️.rs:12` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦠️mutation/🦀️.rs:12` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🔺️diff/🦀️.rs:38` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🔺️diff/🦀️.rs:46` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:27` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:28` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:143` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:143` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🦠️mutation/🦀️.rs:143` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-step-description/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🦠️mutation/🦀️.rs:14` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🦠️mutation/🦀️.rs:14` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦move-block-to-step/🦠️mutation/🦀️.rs:14` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📨️commit-response/🦠️mutation/🦀️.rs:5` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📨️commit-response/🦠️mutation/🦀️.rs:5` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📨️commit-response/🦠️mutation/🦀️.rs:5` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-step/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🦠️mutation/🦀️.rs:12` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🦠️mutation/🦀️.rs:12` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-block/🦠️mutation/🦀️.rs:12` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦠️mutation/🦀️.rs:11` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦠️mutation/🦀️.rs:11` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦠️mutation/🦀️.rs:11` | `dsl::MutationLeaf` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:31` | `dsl::ToValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:31` | `dsl::FromValue` | derive/qualified | semio_framework_value_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:31` | `dsl::Mutations` | derive/qualified | dsl_derive product macro owner |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:263` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:264` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🦀️.rs:5` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🦀️.rs:8` | `dsl::DslRecord` | derive/qualified | semio_framework_dsl_record_derive |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:50` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:52` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:53` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:54` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:55` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:56` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:57` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:58` | `dsl::Number` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:60` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:61` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:62` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:63` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:66` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:68` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:69` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:70` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:71` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:72` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:73` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:74` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:75` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:76` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs:130` | `dsl::DslValue` | runtime_or_import/qualified | semio_framework_value |

Current native prerequisites remain compiler-qualified only after Root authentic unchanged selections. No test success, universal blocker or field-domain simplification is asserted here. Generated roster is requested handoff input; Root can remove generated output at ticket closure after pairing review.
