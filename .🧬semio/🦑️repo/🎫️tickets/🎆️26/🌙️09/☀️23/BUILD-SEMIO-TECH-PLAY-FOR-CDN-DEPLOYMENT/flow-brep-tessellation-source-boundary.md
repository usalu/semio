# Flow B-Rep tessellation source boundary

The Play supervisor stopped before compilation when the B-Rep extension reached 19 Rust source files and 284,073 bytes against a ceiling of 32 files and 262,144 bytes. The authored tessellation request, query, result and retirement logic already lived together in one file. Its only parent-module imports were the session types, which the spatial-session library already owns.

The new `semio-s-plugin-flow-brep-tessellation` library now owns that file at its original path and imports the session types from their defining crate. The extension privately selects the library under its existing `tessellation` module name. The existing inference context still owns the resource and dispatches tessellation through it. The public extension entry points, wire format, fixtures, and source-boundary policy remain the same. The change transfers two direct dependencies to the library and introduces no external runtime dependency.

## Source closure measurements

| Cargo owner | Files | Bytes |
| --- | ---: | ---: |
| B-Rep extension before the split | 19 | 284,073 |
| B-Rep extension after the split | 18 | 254,969 |
| Tessellation library | 1 | 29,137 |
| B-Rep component | 1 | 280 |

The release preflight emitted no errors or over-budget rows for the B-Rep tree. The extension has 7,175 bytes of remaining headroom. The filesystem paths to all ten dependency manifests fit the 256-character limit, and each manifest resolved to the expected first-party crate. Bun parsed the workspace and all four lockfiles, confirming the new library and existing parent have the expected dependency rows.

Both tessellation tests passed with 22 assertions. The original fixture validates the request through Ajv, preserves its exact unsigned 64-bit hash, and checks the existing handoff. A previously stale assertion now names the Flow SDK's current failure admission method. The new law checks that the extension consumes a separate library with its own Rust root and no reverse dependency. The adjacent guest retirement and geometry inference oracles passed 10 and 18 checks respectively. The separate channel oracle stopped at existing schema loading because its Ajv2020 instance was asked to load a Draft-07 schema. It reads packaged channel descriptors outside this edit. No Cargo or release process was started for this check. The replacement release will provide native compilation evidence.

## Changed files

- Extension dependency and private import. [Extension manifest](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/Cargo.toml) · [Extension root](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs)
- Tessellation library package, existing implementation, and tests. [Library manifest](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/📦️packages/🦀️rust/Cargo.toml) · [Tessellation source](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/🦀️.rs) · [Tessellation tests](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/🧪️tests/🟦️.ts)
- Workspace membership and four manually updated locks. [B-Rep workspace](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.toml) · [B-Rep lock](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock) · [Standalone Flow lock](../../../../../../../✏️s/🧑‍💻dev/🌊️flow/Cargo.lock) · [Development composition lock](../../../../../../../✏️s/🧑‍💻dev/🧩️composition/Cargo.lock) · [Procedural extension lock](../../../../../../../✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock)

## Packaged channel oracle

The channel oracle reproduced the missing Draft-07 meta-schema error from the package's actual working directory. It now uses the strict Draft-07 Ajv constructor already imported for the retirement oracle. The geometry inference oracle continues to use Ajv2020 for its own schemas. The channel change affects only the test implementation and keeps its schema and descriptor fixtures intact. [Channel oracle](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🟦️.ts) · [Declared schema dialect](../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧬️schema/🪪️channels/🔣️.json)

All three existing oracles passed from the package working directory in Bun. They reported 328 channel identity, 10 retirement, and 18 geometry inference checks. The tessellation and mesh Bun suites passed all eight tests. No Cargo or release process was started for this change.
