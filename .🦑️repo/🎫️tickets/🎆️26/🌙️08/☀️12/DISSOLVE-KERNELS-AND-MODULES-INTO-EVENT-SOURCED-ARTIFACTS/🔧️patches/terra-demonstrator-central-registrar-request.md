# Demonstrator Central Registrar Request

## Owner Inputs Already Applied

- Removed the generic `🎪️demonstrator/🦀️component.rs` and `🎛️apps/🦀️component.rs` umbrellas.
- Added the only remaining runtime entry at `🛂manifest/🎪️demonstrator/🦀️component.rs`.
- Folded the zero/one-consumer `🧬️schema/💡️inferences/🧭topology/{🦀️,🟦️}component.*` leaves into the inference family roots.
- Removed the nonfunctional, zero-export `stdio.txt` importer/exporter leaves and their false capability declaration; both always returned an error and had no fixture or production exporter.
- Removed every source-level forwarding import/re-export; all source consumers now reference the canonical `standards::v1::subsets::any` path.

## Required Rust Glue Regeneration

Regenerate `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs` from the owner tree with these exact structural results:

1. Retain the canonical `inferences::text` and `inferences::binary` mounts, each exactly once; their Rust components remain authored. Delete only the `inferences::topology` mount that names `🧬️schema/💡️inferences/🧭topology/🦀️component.rs`.
2. Delete the import and export `txt::v_utf_8::any` mount branches that name the removed `📄txt/🔖️utf-8/✳️any/🦀️component.rs` leaves.
3. Delete the complete old-path shim block under `artifacts::playground`: `schema`, `io`, `op`, `dsl`, `spr`, `diff`, `mutations`, and `snapshot`, including every nested `schema`, `text`, `pack`, and `binary` forwarding module.
4. Delete the empty `apps` mount that points at `../../🎛️apps/🦀️component.rs`.
5. Replace the plugin entry mount with:

```rust
//#region 🛂️Manifest
#[path = "../../🛂️manifest/🎪️demonstrator/🦀️component.rs"]
mod manifest;
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(manifest::plugin);
//#endregion 🛂️Manifest
```

No forwarding mount or old path may remain. All canonical artifact source paths are already present and must remain mounted exactly once.

## Required TypeScript Package Index Regeneration

`✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript/📦️index.ts` currently forwards three nonexistent legacy facade paths (`playground_schema`, `playground_decomposer`, `playground_io`) and has no production consumer. Remove those exports instead of repointing or retaining a compatibility namespace. Leave only the registrar-owned minimal module surface required by package tooling.

## Verification Handoff

After regeneration, run:

```text
bun nx run @semio-tech/demonstrator-plugin:test-quick
bun nx run @semio-tech/demonstrator-js:test
```

The source owner must not edit either generated file directly.
