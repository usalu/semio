# Wave 0 R1-D Runtime Caller Remediation

## Scope And Inventory

This follow-up lane owns non-framework-plugin, non-stdio callers. It did not edit the framework plugin, the store/IO implementations, or stdio.

Fresh constructor census:

```text
rg -n -P 'ArtifactStore::new\(' --glob '*.rs' --glob '!target/**' \
  --glob '!🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**' \
  --glob '!🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**' \
  --glob '!✏️s/🔌️plugins/🗄️stdio/**' .
```

The census has **49** constructor calls:

- **3 production boundaries** propagate the `Result`: run persistence, database VCS creation, and Space ZIP import.
- **46 valid test fixtures** explicitly use `.expect("valid artifact store fixture")` (or a more specific fixture message). No runtime constructor caller silently drops failure.

The subsequent negative audit found no non-fixture constructor use that ignores or unwraps the fallible result.

## Migrated Runtime Files

| File | Runtime propagation |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` | `OsWorkflowStore::new` returns `Result<Self, VcsError>`; reset and initial store construction propagate. Format descriptor/accept-filter queries propagate their typed results, require first MIME/extension claims, and use canonical dialect ids during workflow negotiation. Mesh export registration defers descriptor resolution into its fallible handler, preserving the void setup API while returning descriptor/claim failure during actual export. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | Binary media reconstruction propagates format registry failure, rejects unknown formats, and requires a MIME claim. |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | `export_process3d_model` is now `Result<Option<_>, String>`: absent replay remains `Ok(None)`, while codec, format lookup, claim, and UTF-8 failures propagate. Filenames/MIME/encoding come from plural descriptor claims. |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎮️commands/📤️media/🦀️component.rs` | Export command maps process export failure to a stable application `Fault`. |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs` | `brep:out` maps process export failure to `MediaError::Payload`. |
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️export-media/🦀️component.rs` | Descriptor result, unknown format, and host export failures surface as stable application faults rather than an empty emit. |
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️import-media/🦀️component.rs` | Descriptor and accept-filter results surface as faults; the raw extension fallback was removed. |
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️import-media-payload/🦀️component.rs` | Descriptor, base64 decoding, pending-node lookup, and host import failures surface as faults; pending-import state clears only after successful validation. |

## Fixture Migration

The 46 fixture calls were migrated in the non-stdio test sources for Writer, Mathematical, Procedural, Flow, GIS, VCS, Sequence, Lowpoly, Forms, Layout, CAD, Norm, Playbook, Imperative, Remodel, Trinity, DAG, Draw, Raster, Note, Space Home, Sourcing, OS Flow VCS, OS Space, and OS Host. Each uses an explicit valid-fixture expectation; production code does not.

## Exact Deferred Document-Codec Registrars

`register_document_codec_for_app` now returns `Result`. These **16 imperative callers** remain outside this lane because their current plugin setup/register hooks return `()`. They require the framework-plugin declaration/startup migration to carry startup failure without a wrapper or runtime `expect`:

1. `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🦀️component.rs:104`
2. `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:837`
3. `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs:67`
4. `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs:556`
5. `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:35`
6. `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:36`
7. `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:38`
8. `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:39`
9. `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🎪️demonstrator/🦀️component.rs:40`
10. `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🦀️component.rs:134`
11. `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs:86`
12. `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:142`
13. `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🦀️component.rs:110`
14. `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs:639`
15. `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:2804`
16. `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs:109`

The framework-plugin implementation and all stdio callers are deliberately excluded from this list and this lane.

## Verification

| Command | Result |
| --- | --- |
| `bun 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts check` | Stopped in concurrently edited framework-plugin code: `AppDefinition` initializer at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4665` is missing `dialect` and `role`. |
| `bun ✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📜️script.ts test` | Stopped at the same framework-plugin `AppDefinition` initializer error before Process compiled. |
| `bun ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts test` | Stopped in concurrently edited framework-plugin code: five invalid inner doc comments at lines `11537..11541`, plus the `AppDefinition` initializer error at line `4665`. |
| `cargo check --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml` | Stopped at the same five framework-plugin doc-comment errors and `AppDefinition` initializer error. |
| `rustfmt --edition 2021 --check` over edited callers | Parsed all edited files. It reports existing repository-wide formatting drift, including untouched regions; no formatting rewrite was applied. |

No check reached the R1-D callers after the framework-plugin dependency failed, so none are claimed passing. The failures above belong to the concurrently owned framework-plugin source, not to a caller diagnostic.
