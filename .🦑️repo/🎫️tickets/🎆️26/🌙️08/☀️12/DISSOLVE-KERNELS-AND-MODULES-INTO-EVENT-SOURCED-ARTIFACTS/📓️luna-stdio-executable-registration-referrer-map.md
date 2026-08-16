# Stdio Executable-Registration Referrer Map

## Scope

Read-only map of the currently failing executable-registration contract.

## Ownership and Dependency Direction

| Surface | Owner | Role |
| --- | --- | --- |
| Artifact definition schemas | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🧬️schema/📜️artifact-definition.json` | Thirty-six source definitions; catalog order is authoritative in `📇️registry/📇️catalog.json`. |
| Rust registry | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` | Reads definitions, validates catalog, builds definitions/assemblies/descriptors, imports every artifact assembly. |
| Stdio plugin assembly | `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` | Sends registry definitions and runtimes to `PluginBuilder`. |
| Artifact runtime declarations | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🦀️component.rs` | Twenty-six roots call `runtime_assembly`; ten are definition-only. |
| Structural validator | root `📜️script.ts` | Validates definition fields, catalog/ledger parity, facades, manifest wiring, and the stdio structural gate. |
| Capability contract | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | Owns typed artifact capability, executable identity, definition registry, declaration builder, and registration plan. |
| Capability assembly | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` | Preflights and commits registrations through `PluginBuilder`. |

## Current Facts

- All definitions currently have `runtime_capabilities: []`; root `📜️script.ts` rejects this field as unknown.
- Rust parses the field, but empty arrays yield no format descriptors.
- The executable-mapping map is empty, so a true registration row cannot pass mapping-key validation.
- glTF has six codec, eighteen mutation, and fifteen inference rows. Every row is `unimplemented` with `executable_registration: false`. Rust accepts that condition; TypeScript currently rejects every nonempty unregistered row.
- Registry representation capabilities lack MIME/extension claims needed by runtime language/format declarations.
- A fresh `cargo check -p semio-s-plugin-stdio --lib` is blocked before stdio assembly by dirty framework capability API drift: missing registration-plan/builder methods and changed signatures. This is an upstream concurrent blocker, not evidence that stdio compiles.

## Dependency Result

The code-level SCC is the stdio Rust registry plus all thirty-six artifact root components: the registry imports every root assembly, while roots call registry assembly helpers. The JSON schemas and TypeScript validator are contract boundaries; the framework capability API is downstream. Treat the work as ordered cross-owner leases rather than a false single SCC:

1. Contract field and registration semantics in schemas plus root validator.
2. Stdio registry and artifact-root assembly mapping SCC.
3. Framework capability/assembly API, once its concurrent drift is stable.

## Required Validation

```text
bun ./📜️script.ts stdio quick
bun nx run @semio-tech/stdio-js:test-quick
RUSTC_WRAPPER= cargo check -p semio-s-plugin-stdio --lib
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

Add one focused end-to-end test for `registry::artifact_definitions()` through `registry::artifact_assemblies()`, `stdio::plugin()`, and `PluginBuilder::try_library()`, asserting glTF capability claims and typed executable identities.
