# Wave 0 R2 D — Global Plugin Closure

## Scope

`✏️s/🔌️plugins/**`, excluding `🗄️stdio/**`. The scan is code-aware: Rust comments and string literals are masked before matching, and only artifact-root components (`🗿️artifacts/*/🦀️component.rs`) count as artifact declarations.

## Census

| Residual | Initial | Final | Closure |
| --- | ---: | ---: | --- |
| Raw `ArtifactDeclaration::builder("…")` | 0 | 0 | Every artifact root already passes its fallible `definition()` into the declaration builder. |
| Infallible declaration `.build()` | 0 | 0 | All 54 non-stdio artifact-root declarations terminate in `.try_build()`. |
| Executable `Plugin::builder(...).setup(...)` | 1 | 0 | CAD now contributes its mesh/DWG importer with `HostMediaHandlerDeclaration::mesh_dwg_bridge`. |
| Artifact-root `io_registry` alias module | 26 | 0 | Removed the forwarding roots; declarations consume the owning leaf `io_registry::entries()` facet directly. |
| Void/global schema, codec, composer, or language registrar call | 29 | 0 | Removed 26 forwarding composer registrars, Playbook procedural's global codec registrar, and two Note test-only global composer registrations. |
| Legacy `Plugin::new` plus `register_document_app` bundle | 1 | 0 | Playbook procedural now has a fallible `Plugin::builder` bundle with `foreign_document_codec` and `document_app`. |
| Global CAD host-media registrar | 1 | 0 | Removed `register_host_io`; the declared typed bridge is assembled transactionally with the plugin. |
| Ignored typed registration result | 0 | 0 | No discarded registration result remains. |
| Runtime-derived definition helper | 0 | 0 | The 26 local row definitions and 15 norm `assemble_definition` calls use only leaf-local compile-time identity/capability/claim/localization literals; their validator/assembler reads no runtime schema, app, registry, inference, composer, language, hash, or descriptor data. |
| Dummy or alias definition | 0 | 0 | All 54 artifact roots own direct `definition()` and literal `ArtifactDeclaration::builder(definition()?)` surfaces. |

## Validation

- `rustfmt --edition 2021` completed for all touched Rust sources.
- The final code-aware census is zero for every residual class above.
- Cargo checks are intentionally deferred while GLTF inference owns the shared Cargo lane. Run the serialized workspace/plugin checks after that lane is released.

