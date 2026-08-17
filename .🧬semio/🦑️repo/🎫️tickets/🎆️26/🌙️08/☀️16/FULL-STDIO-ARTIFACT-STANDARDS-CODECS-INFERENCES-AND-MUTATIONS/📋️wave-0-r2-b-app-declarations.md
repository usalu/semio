# Wave 0 R2-B App Declarations

## Result

The eight remaining app/artifact setup registrars are replaced by leaf-owned, fallible `ArtifactDeclaration`s. Each plugin root now declares its artifact and its typed document app; each app returns its own config/presence schema descriptor through `ArtifactApp::app_schema`.

| Plugin | Artifact leaf | Document app |
| --- | --- | --- |
| `reasoning` | `s.wires` | `ReasoningWiresPlayApp` |
| `sequence` | `s.sequence` | `SequencePlayApp` |
| `writer` | `s.writer` | `WriterPlayApp` |
| `process` | `s.process3d` | `Process3dPlayApp` |
| `flow` | `s.flow` | `FlowPlayApp` |
| `animate` | `s.present` | `AnimatePresentPlayApp` |
| `vcs` | `s.vcs` | `VcsPlayApp` |
| `architect` | `s.program` | `ArchitectPlayApp` |

## Declaration Contract

Every owned artifact leaf now supplies:

- `definition() -> Result<ArtifactDefinition, ArtifactDefinitionError>` with direct literal schema, inference, composer, codec, and `en`/`de` localization claims;
- `declaration() -> Result<ArtifactDeclaration, ArtifactDefinitionError>` with its actual schema descriptor, inference descriptor, composer entries, and owned document codec;
- no setup callback, process-global schema registration, codec registration, or raw language registration.

The plugin root registers the same typed app with `.document_app::<App>(create_app())`; the builder obtains the app schema directly from `ArtifactApp::app_schema`.

The config-schema leaves for all eight apps expose only `app_schema_descriptor()`. The `ArtifactApp` implementations return `Some(app_schema_descriptor())`; none relies on the trait default or a side effect.

## Language Boundary

No artifact claims a grammar in this wave. The former registrations used `dsl::passthrough_hooks`, which are not executable grammar hooks and were rejected by the R2-A contribution contract. There is no literal-language API. Grammar declarations remain absent until an artifact can supply a real lexer/parser/printer hook table.

Writer's existing jack completion behavior no longer depends on globally registering an idiom: `jack_completions_json` calls `JackWriterIdiom` directly. This preserves the direct completion path without retaining the removed app setup callback.

The final R2-A API includes `PluginBuilder::foreign_document_codec::<App>(schema)` only for app codecs outside an artifact declaration. It does not apply here: each codec belongs to its owning artifact and is declared with `.document_codec::<App>()`.

The flow tree contains no `FlowExtensionDeclaration` or `.flow_extension(...)` owner descriptor, so no flow-extension registration is added.

## Source Validation

Completed:

- Scoped source scan found no `register_app_schema`, pilot-language register function, raw `dsl::register_language`, `dsl::passthrough_hooks`, old codec registration, or old app setup entry in the eight owned plugin trees.
- Scoped source inventory found all eight fallible `definition()` and `declaration()` leaves, all eight `.document_app::<App>(...)` bindings, and all eight `ArtifactApp::app_schema` implementations.
- `rustfmt --edition 2021` completed successfully for the changed Rust files.

Not run:

- Cargo checks are deferred by the root task until the concurrent GLTF/framework transition closes. No compile or runtime success is claimed by this report.

## Scope

Only the eight assigned plugin trees and this ticket report were changed. Framework, stdio, procedural, space, GIS, and unrelated plugin trees were not edited by this lane.
