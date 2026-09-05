# Value Subtree Hand Repair

Scope: `🧰️framework/🔨️modules/🌱️value`. No Git mutation, migration script, generated emoji selection, or broad text replacement is permitted. The inspected singleton format leaves (`🦀️.rs`, `🟦️.ts`, `🔣️.json`) retain their meaningful format roles. Cargo.toml remains literal. All names below were selected individually after inspecting the files and sibling groups.

## Initial Check

The read-only Unicode grapheme/sibling check found one missing prefix (`derive/.../tests`) and nine sibling groups with repeated prefixes. It found no stacked emoji in this subtree. The wheat prefix of the flattening test was also semantically unrelated to its field-flattening behavior.

## Explicit Decisions

Paths below are relative to the scope root. A directory rename also changes the corresponding prefix of its descendants.

| Old Path | New Path | Reason |
| --- | --- | --- |
| `✨️derive/📦️packages/🦀️rust/tests` | `✨️derive/📦️packages/🦀️rust/🧪️tests` | Integration tests are explicitly registered in Cargo.toml, so this directory is configurable. |
| `✨️derive/📦️packages/🦀️rust/🧪️tests/🌾flatten-with-skip.rs` | `✨️derive/📦️packages/🦀️rust/🧪️tests/🪗️flatten-with-skip.rs` | The accordion represents flattening and expansion of nested fields. |
| `🗂️ordered/🧪️fixtures` | `🗂️ordered/🧫️fixtures` | Specimen data differs from executable tests. |
| `🗂️ordered/🧫️fixtures/🔣️.schema.json` | `🗂️ordered/🧫️fixtures/🧬️.schema.json` | Schema structure differs from its JSON specimen. |
| `🗂️ordered/🧫️fixtures/🧪️shared-owner` | `🗂️ordered/🧫️fixtures/👥️shared-owner` | Multiple owners share retained values. |
| `🗂️ordered/🧫️fixtures/👥️shared-owner/🔣️.schema.json` | `🗂️ordered/🧫️fixtures/👥️shared-owner/🧬️.schema.json` | Schema structure differs from its JSON specimen. |
| `🗂️ordered/🧺️set/🧪️fixtures` | `🗂️ordered/🧺️set/🧫️fixtures` | Set specimens differ from executable tests. |
| `🗂️ordered/🔢️numeric/🧪️fixtures/🔣️numeric-index.json` | `🗂️ordered/🔢️numeric/🧪️fixtures/🔢️numeric-index.json` | Numeric ordering cases. |
| `🗂️ordered/🔢️numeric/🧪️fixtures/🔣️numeric-index.schema.json` | `🗂️ordered/🔢️numeric/🧪️fixtures/🧬️numeric-index.schema.json` | Numeric specimen structure. |
| `🗂️ordered/🔢️numeric/🧪️fixtures/🔣️references.json` | `🗂️ordered/🔢️numeric/🧪️fixtures/🔗️references.json` | Reference capture and ownership cases. |
| `🗂️ordered/🔢️numeric/🧪️fixtures/🔣️references.schema.json` | `🗂️ordered/🔢️numeric/🧪️fixtures/🛡️references.schema.json` | Guards exact rejection/preservation contract. |
| `💾️resident/🧪️fixture` | `💾️resident/🧫️fixture` | Resident specimen data. |
| `💾️resident/🧪️schema` | `💾️resident/📐️schema` | Structure constraints for the resident test fixture. |
| `💾️resident/🧬️contract.json` | `💾️resident/🤝️contract.json` | Ownership agreement differs from capacity schema. |
| `💾️resident/📨️admission/🧪️fixture` | `💾️resident/📨️admission/🧫️fixture` | Admission specimen data. |
| `💾️resident/📨️admission/🧪️schema` | `💾️resident/📨️admission/📐️schema` | Structure constraints for admission test fixture. |
| `💾️resident/📨️admission/🧬️contract.json` | `💾️resident/📨️admission/🤝️contract.json` | Admission agreement differs from contract schema. |

## Verification

All seventeen listed moves are applied. Every destination was checked absent before an exact filesystem move; no source file was reconstructed from Git. Incoming references were patched explicitly in their own context.

The entire subtree contains 44 files and 24 directories. Its final read-only check finds zero missing, multiple, duplicate, generic, presentation, or reserved-name violations. Only the two literal Cargo.toml manifests are exempt. `Intl.Segmenter` and the independent `emoji-regex` package agree on the emoji count of every basename. The repository's `pathEmojiStatuteFindings` also returns no findings for all 68 entries with no subtree exemptions.

Passed:

- `bun nx run @semio-tech/value-numeric-index:test --skip-nx-cache`: numeric-index laws, lifecycle, ordinal, stress, and reference cases with Immer/Map/NodeAssert oracles.
- `bun nx run @semio-tech/value-resident:test --skip-nx-cache`: resident/admission fixtures and runtime ownership checks with Ajv/Immer/Buffer/BigInt.
- `bun nx run @semio-tech/framework-replication-rs:test-source --skip-nx-cache`: ordered-map, shared-owner, and ordered-set fixture validation with fast-json-stable-stringify.
- `bun nx run @semio-tech/repo-lib:test --skip-nx-cache -- long -t 'resident native metadata binds'`: one test, 69 assertions, zero failures after the negative fixture correction below. This is metadata validation, not native execution.
- Cargo.toml parsed identically with Bun and `@iarna/toml`; all three explicitly named integration-test paths exist.

Native status:

- `@semio-tech/value-resident-rs:test` fails because three previously mounted test inputs are absent: `INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️baseline.rs`, `.../🦀️resident-release.rs`, and `.../🦀️resident-primary.rs`. The same mounts exist in baseline commit `03100691d5`, whose tree also does not contain those files. The mounts and tests were preserved; no substitute test bodies were invented and no coverage was removed.
- `@semio-tech/value-derive-rs:test` reaches the registered `newtype_transparent` integration test but fails with Rust `E0464`: multiple `rlib` candidates for `semio_framework_os_kernel`. No native pass is claimed. Its build artifacts are isolated in this ticket's generated target directory.

Generated command logs remain under this ticket's `🗑️generated/value`. The metadata test created its own uniquely named no-follow fixture in an older ticket; that exact new directory was moved into this ticket's generated folder after the test completed. No preexisting fixture was removed.

## Exact Reference and Fixture Corrections

Within the subtree, Cargo's three test paths, ordered-map/set fixture includes, resident fixture includes, script imports, and resident package named inputs were updated. The ordered-set source controller already referred to nonexistent `🧪️fixture/🔣️s.json`; it now resolves the actual `🧫️fixtures/🔣️.json` specimen.

Outside the subtree, only these incoming-reference or matching-regression files were edited:

- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts`: ordered fixture source-controller import.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx`: resident fixture and admission contract imports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️.tsx`: corrupted double-emoji capacity-schema import corrected to the existing single-emoji schema. The file's unrelated literal NUL byte was preserved. Read-only resolution of all 71 dynamic value imports across both renderer files succeeds.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️package-language-kind-handoff/🧪️resident-package/🔣️.json` and its `🧬️schema/🔣️.json`: exact positive paths and two wrong-leaf negative cases.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`: two exact source/include expectations.

The resident metadata fixture's rejected-entry list already contained the valid positive `../../🦀️.rs` and a duplicate `../../../🦀️.rs` in baseline `03100691d5`. The first rerun failed on that contradiction. Those two cases now use nonexistent `../../🦀️missing.rs` and `../../../🦀️missing.rs`. All six rejection cases remain distinct, both POSIX and Windows assertions remain unchanged, and the actual positive source-path assertion remains unchanged.

## Retained Names Reviewed

The value root retains `✨️derive` (macro derivation), `🗂️ordered` (ordered collections), `💾️resident` (resident capacity/ownership), and `🔁️codec` (bidirectional value conversion). Ordered collections retain `🔢️numeric` (numeric index), `🧺️set` (membership), and `🧪️tests` (executable laws). Admission retains `📨️admission` (resource admission). The single numeric specimen directory remains `🧪️fixtures`; it has no sibling collision. Packages retain `📦️packages` and `🦀️rust`. Project/task files retain `📋️project.json` and the required `📜️script.ts`. The denial and newtype tests retain their distinct `🛡️` and `🆔️` prefixes. Every remaining format-marker leaf identifies its real format and is unique in its own sibling group.

The parent repair lane registered the exact new semantic-role names centrally. A final filesystem walk through `semanticDirectoryKindId` resolves all 24 directories, including the preexisting codec member. This lane did not modify the shared taxonomy.
