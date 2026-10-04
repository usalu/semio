# Current I/O Vocabulary Extraction Audit

Read-only source audit on 2026-10-03. No compiler, runtime, Git mutation, or production edits were performed. Findings describe the source observed during inspection; concurrent work may change it.

## Actual Ownership Hazards

- The new package entry mounts the existing schema source, which still mounts `reference` and imports undeclared `serde`. Its `ArtifactRef::parse_uri_controlled` signature also names undeclared `dsl::NativeDecodeControl`. Removing only the binding mount does not close the package.
- The reference binding implements OS-owned `DslField` for schema-owned `ArtifactRef`. Moving the implementation into the OS crate is valid because the trait remains local to that crate; putting it in the neutral schema crate would require the OS dependency. Explicit `crate::io_schema` and `crate::os_dsl` paths make both identities reviewable.
- OS currently mounts the schema source as a module while framework reexports the OS module. Rebinding both facades directly to the neutral package avoids a second nominal `ArtifactRef` definition. Leaving the old OS mount while adding the package would create incompatible type identities.
- Value already publicly reexports `serde` and `serde_json` at its package entry. Importing serde derives through `semio_framework_value::serde` additionally requires `#[serde(crate = "semio_framework_value::serde", rename_all = "camelCase")]` on `ArtifactDialect`: the macro's generated serde namespace otherwise remains undeclared. This is the only observed serde-derived schema type. Value derives default to `::semio_framework_value`, which the neutral package already declares.
- The OS DSL reexports native decoding control from its protocol value owner. Lowering the URI parser signature to `semio_framework_value::NativeDecodeControl` preserves the intended lower-level owner; confirm that OS protocol still reexports the same nominal control before claiming identity preservation.
- The moved refusal test currently includes `../../🧫️fixtures/🚦️refusals/🔣️.json` relative to its source. Moving its source without updating that literal breaks input ownership. The general ownership test separately includes the original shared reference fixture: keep that general fixture in place when relocating only the OS binding and refusal test.

## Namespace Scanner Scope

The actual TypeScript owner scanner recursively parses mounted `mod` source from the neutral package entry, including `cfg(test)` modules without evaluating cfg. Therefore it reaches the current private reference binding and detects its `dsl` namespace, and detects the root's `serde` namespace. It does not traverse Cargo dependency source. Its result establishes direct mounted-source namespace closure, not transitive package deletion readiness.

`serde_json` is explicitly admitted to the scanner as a development namespace independently of manifest parsing; the package currently declares it as a dev dependency. Macro-generated namespaces are invisible to its AST scan, so serde crate attributes need direct inspection. It does not inspect `include!` expansion, macro token internals, or resolve imported aliases. File-local module names are admitted without checking declaration visibility at each use. These are limits of the asserted closure, not evidence that the current repair is wrong.

Every traversed source must satisfy tree-sitter's grammar before namespaces are checked. Neutral traversal begins at its own package entry and cannot reach the OS crate root once the binding mount is removed. The reported baseline OS crate-attribute grammar error is therefore independent of this neutral scanner; no new run was performed to verify either result here.

## Deletion Implications

The existing generic schema file remains the vocabulary definition owner even after package extraction, because the package mounts it by path. Do not delete that file or its general fixtures. Only the old binding mount and old binding/test source locations become candidates for removal after their new OS ownership is mounted. Snapshot extraction and transitive general dependency audits remain separate work; this audit provides no snapshot completion or compiler correctness claim.
