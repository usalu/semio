# Manifest Emoji Repair

Scope: `🧰️framework/🔨️modules/🛂️manifest`. The authored and generated naming repairs are complete. This is not a workspace-wide completion claim.

The authored tree has ten files. Its six neutral fixture/schema files formerly shared the same `🔣️` prefix in one directory. The six explicit choices below preserve the existing tree and distinguish the actual purposes. The unique format leaves elsewhere are retained. Dependency-managed `node_modules` remains literal. The three generated outputs are also repaired, together with their owning generator contracts and incoming references; ignored outputs are not exempt from naming rules.

## Handpicked Names

All six paths are relative to `🧪️fixtures` under this manifest root.

| Original | Handpicked | Meaning |
| --- | --- | --- |
| `🔣️action-semantics.json` | `⚖️action-semantics.json` | Action effects, permissions, and reversibility |
| `🔣️action-semantics.schema.json` | `📜️action-semantics.schema.json` | Formal action-policy contract |
| `🔣️tutorial-document-track.json` | `🎞️tutorial-document-track.json` | Recorded document-event sequence |
| `🔣️tutorial-document-track.schema.json` | `🛤️tutorial-document-track.schema.json` | Document timeline track shape |
| `🔣️tutorial-local-interaction.json` | `🖱️tutorial-local-interaction.json` | Recorded local selection and interaction state |
| `🔣️tutorial-local-interaction.schema.json` | `🎛️tutorial-local-interaction.schema.json` | Allowed local interaction axes and their shape |

## Verification

The six names were changed by exact no-clobber filesystem moves. No new naming scripts, modifying Git commands, global replacements, or fixture content edits were used.

- Audited all ten authored files and three authored directories: zero missing prefixes, multiple emojis, or sibling collisions. The six renamed JSON files have exactly the same SHA-256 content hashes as before.
- Independent Ajv validation passes for all three schema/data pairs, using the existing referenced local-interaction schema.
- Direct runtime verification passes for all six action-semantics defaults and all six local-interaction cases. The local interaction result is also compared with the independent Immer implementation. This diagnostic imports the actual current source and renamed fixtures; it is not a copied implementation.
- `bun nx run '@semio-tech/framework-rs:check' --skip-nx-cache` passes after the manifest changes. Framework Rust library tests compile, the type-export test passes, and the mirror is fresh. The existing target filters the other 267 tests; they are not claimed as executed.
- The renderer's focused Nx test command fails before test collection because of existing out-of-scope resident-value references in `UiDocumentStore` and `PluginRuntime`. The captured pre-edit failure names `📨️admission/🧬️contract.json` and `💾️resident/🧬️🧬️schema.json`. This test is not reported as passing.
- Source and documentation reference scans find no remaining references to the six former fixture names outside historical ticket records.
- A final audit including generated outputs covers thirteen files and four directories: zero missing prefixes, multiple emojis, or sibling collisions. The three former generated filenames are absent.
- `@semio-tech/ui-rs:check` passes: the UI-axes projection is fresh and its bytes are unchanged from before the exact filename move.
- `@semio-tech/ui-contract-rs:check` passes: one owned schema-export test, zero failures, and the contract projection is fresh. Its pre-edit check failed on exactly three stale stacked-emoji references in generated documentation; the explicit corrections below restore parity without changing any type or behavior.
- `@semio-tech/framework-kernel:test-quick` passes again after the generated filename changes: 49 tests, two files.
- `@semio-tech/framework-rs:check` also passes again after the generated filename and import changes: one schema-export test, zero failures, 267 filtered tests, and a fresh framework mirror.
- After the value-tree agent repaired the first two renderer imports, the focused renderer command still fails before collection on `PluginRuntime/🟦️.tsx` importing `../🖥️🛸️ShellHost/🧪️fixtures/🔣️extension-invocation.json`. This next foreign-tree blocker was reported to the parent; the renderer test is not claimed as passing.

## Source References Updated

Only two Rust `include_str!` paths in this tree's `🦀️.rs`, four incoming imports in renderer React `🧪️index.test.ts`, and the two incoming fixture/schema paths in the UI React local-interaction test were changed. Functional code and test expectations are unchanged.

## Generated Handpicked Names

All paths below are relative to `🤖️generated` under the manifest root. All three were exact no-clobber moves after inspecting their owning exporters.

| Original | Handpicked | Meaning |
| --- | --- | --- |
| `🟦️manifest.ts` | `🪪️manifest.ts` | Declared framework boundary identities and descriptors |
| `🟦️ui-axes.ts` | `🎚️ui-axes.ts` | Configurable locale and terminology axes |
| `🟦️ui-contract.ts` | `📜️ui-contract.ts` | Renderer-neutral UI wire contract |

The framework Rust generator output path and its two generated import strings, UI-axes generator target, UI-contract generator target, and their exact byte-comparison test paths now use these names. Direct imports were updated only in manifest, kernel, interaction, and mesh. The parent updated the exact central `generatorContracts` references. Relevant owning generator documentation was updated without a global basename replacement.

The moved UI-axes file is byte-identical. The moved manifest differs only in the two reviewed generated imports (`./📜️ui-contract.ts`, `./🎚️ui-axes.ts`). The moved UI contract differs only in three stale generated docstrings: `🦀️💎️action.rs` → `🦀️action.rs`, `🦀️🔤️label.rs` → `🦀️label.rs`, and `🦀️📃️document.rs` → `🦀️document.rs`. These are exactly the current owning exporter's output; all 693 lines were compared. No other bytes were changed. A source/reference scan excluding historical tickets found no remaining old generated path references.

Exact generator/source owners additionally edited: framework Rust `🦀️.rs`, `📜️script.ts`, and Cargo documentation; UI Rust `📜️script.ts` and WGPU label documentation; UI-contract Rust `📜️script.ts` and existing export test; manifest `🦀️.rs` and `🟦️.ts`; kernel, interaction, and mesh `🟦️.ts`.
