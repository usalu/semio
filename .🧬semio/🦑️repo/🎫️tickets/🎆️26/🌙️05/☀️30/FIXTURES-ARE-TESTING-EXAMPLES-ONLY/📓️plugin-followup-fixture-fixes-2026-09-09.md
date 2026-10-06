# Plugin Fixture Reader Follow-up — 2026-09-09

## Result

The four runtime-confirmed stale readers now resolve canonical fixture owners, the Stdio wiring and Norm taxonomy corpora moved out of Rust implementation-package roots, and eleven glTF inference vector corpora moved from `🧪️contract` to their inference leaves' `🧫️fixtures` directories. No compatibility aliases remain at the old paths.

The first corrected Stdio and Process executions exposed bounded inconsistencies that had been hidden behind the original `ENOENT` failures. Stdio's Flow oracle now uses the draft-07 AJV implementation, compiles the two actual `$defs`, and reads the live `🧪️tests/💾️binary/🦀️.rs` law source. Process's retained-route fixture and schema now describe the 31 routes actually declared by the Rust editor, omit the absent `setActiveUtility` route, and prove cancellation through the two live retained-job `cancel` implementations.

## Exact Moves

1. `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🧭️wiring-fixture/🗂️subset-directory-wiring/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring/🔣️.json`
2. `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📇️mutation-leaf-taxonomy-v1.json` → `✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`
3. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🧫️fixtures/🔣️.json`
4. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🧫️fixtures/🔣️.json`
5. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🧫️fixtures/🔣️.json`
6. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧫️fixtures/🔣️.json`
7. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧫️fixtures/🔣️.json`
8. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🧫️fixtures/🔣️.json`
9. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🧫️fixtures/🔣️.json`
10. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🔢️genus/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🔢️genus/🧫️fixtures/🔣️.json`
11. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🕳️holes/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🕳️holes/🧫️fixtures/🔣️.json`
12. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🥯️handles/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🥯️handles/🧫️fixtures/🔣️.json`
13. `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🧪️contract/🔣️.json` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🧫️fixtures/🔣️.json`

The two large corpus hashes remained unchanged after relocation and after running the Norm generator:

- Stdio wiring: `aace4b2df9552650db98a4e292ca3e7b49405ef9818f9638e5ecfffe8e200563`
- Norm taxonomy: `0dd3ea92286a1a3956532b47919de8b8c0b7e0b2c7bdb9f65cfe577eb2cfdd60`

## Updated Readers and Contracts

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🧫️fixtures/⚖️retained-route-laws.json`
- Four proportion inference Rust unit readers, two size inference Rust canonical-vector readers, and the topology aggregate canonical-vector reader listed in the Authored Paths array below.

The pre-existing Norm `SurfaceRenderSourceScript` change from `🧪️tests` to `🧫️fixtures` shares the Norm package script but is not authored by this follow-up.

## Verification

Every runtime command ran through `bun nx exec --projects=fixture-probe -- bun <absolute-script> <command>` with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, `NX_WORKSPACE_ROOT_PATH` set to the ticket's private `🧑‍💻coordination/🧫️nx-fixture`, `SEMIO_FIXTURE_REPO_ROOT` set to the repository root, and isolated workspace data, cache, temporary, and test-artifact directories under `🗑️generated/plugin-followup`.

- Stdio `flow-retained-decode-check --oracle-only`: passed; 2 exact snapshots, 12 hostile denials, 5 lifecycle admission states, 2 input pages, and 8,472 retained bytes.
- Stdio `subset-directory-wiring check semio mesh`: passed; 4 schema-valid cases and 0 stale files.
- GIS `component-cold-map-patch-check`: passed; AJV, Node/WebCrypto SHA-256 agreement, 5 hostile vectors, and 9 Rust-source markers.
- Norm `config-mutation-source`: passed; 5 cases, 5 hostile payloads, 4 undeclared forms, 13 text vectors, and 25 binary vectors.
- Norm `mutation-leaf-taxonomy-generate` followed by `mutation-leaf-taxonomy-check`: passed; 392 payloads and an unchanged canonical corpus hash.
- Process TypeScript `test`: passed; 2 example tests and the 31-route retained-route schema/independent/source oracles.
- glTF Rust package `test` filtered to the four affected inference test names: passed 7 tests; 252 unrelated tests skipped.
- Static stale-path audit: no old Stdio singular fixture, wiring-fixture, Process retained-route test path, or glTF `🧪️contract` reader/reference remains; all 13 old corpus files are absent and all 13 canonical targets exist.

Generated Nx, Cargo, and Nextest outputs remain under `🗑️generated/plugin-followup` for coordinated final cleanup.

## Authored Paths

```json
[
  "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts",
  "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts",
  "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts",
  "✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🧫️fixtures/⚖️retained-route-laws.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️tests/🔬️canonical-vectors/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🧪️tests/🔬️canonical-vectors/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧪️tests/🔬️canonical-vectors/🦀️.rs",
  "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🧭️wiring-fixture/🗂️subset-directory-wiring/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring/🔣️.json",
  "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📇️mutation-leaf-taxonomy-v1.json",
  "✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/↔️elongation/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📃️flatness/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/📏️slenderness/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📏️proportion/🖼️aspect-ratios/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/📦️size/📏️overall-size/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/➰️boundary-loops/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🔢️genus/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🔢️genus/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🕳️holes/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🕳️holes/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🥯️handles/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🥯️handles/🧫️fixtures/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🧪️contract/🔣️.json",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🕸️topology/🧮️euler-characteristic/🧫️fixtures/🔣️.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️plugin-followup-fixture-fixes-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📋️plugin-followup-authored-paths-2026-09-09.json"
]
```
