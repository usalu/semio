# Product and Plugin Fixture Separation

## Scope and Taxonomy

This lane covers `✏️s/**`, including all product and plugin trees, and excludes `🧰️framework/**`. The root, `✏️s`, Flow, Stdio, Shooting, and Lowpoly `AGENTS.md` instructions were read before their respective changes.

- `🧫️fixtures` contains language-neutral, testing-only example inputs and expected outputs under the nearest semantic owner.
- `🖼️assets` contains static data used by production or shipped examples.
- `🧬️schema` contains contracts and schema support, not example instances.
- `🧪️tests/<case>/<implementation>` contains the canonical feature and executable implementations. No fixture bundle remains inside a test case.

The test URI contract now resolves `shared://` only from `<owner>/🧫️fixtures`, resolves `asset://` only from `<owner>/🖼️assets`, and rejects `local://`.

## Authored Migration

The flat authored-file ledger contains 17,252 entries: 13,822 moves, 21 creations, and 3,409 source modifications. Every one of the 13,822 moves is byte-identical by before/after byte count and SHA-256. The complete exact old path, new path, byte count, before hash, after hash, and preservation result for every move is retained in `📓️plugins-authored-file-manifest-2026-09-09.md`; the machine-readable working copy is `🗑️generated/plugins/📊️moves.json`. The only retained migration program is `🔌️plugins/📜️script.ts`.

The migration includes these substantive groups:

- 361 files from 63 case-local `🧫️fixtures` bundles moved to owner fixtures, with 172 feature/implementation files converted from `local://` to `shared://`. The scoped resolver now encounters no `local://` compatibility path.
- 12,151 mutation example-data leaves moved from schema test cases to `<subset>/🧫️fixtures/🧬️mutations/...`. Canonical Rust, TypeScript, Python, Go, C#, and feature implementations remain under their mutation test cases. Direct and grouped cases were renamed to the oracle manifest's scenario names, and their callers were repaired to the final paths.
- 105 shipped example assets moved from per-example directories to the semantic owner's `🖼️assets/<example>/...`, with 404 consumers updated. Energy's 14 formerly empty BESTEST placeholders were moved there and filled from the model builders; its 14 language-neutral model JSONs were generated under owner fixtures. The existing demo asset remains under owner assets.
- CAD and Infinite runtime static routes now serve owner assets as `/cad-assets` and `/infinite-assets`. The CAD PNG and Infinite media/PDF bytes were preserved. Cargo manifests no longer deliver fixture directories as product static content.
- Sequence protocol test ledgers were removed from production `include_str!` dependencies and are compiled only by its canonical Rust test implementation.
- Flow's executable test support moved out of `🧫️fixtures` into its canonical TypeScript test implementation. Three package-script imports now use that implementation.
- Stdio's Rust package fixtures moved to the plugin owner, and the binary snapshot owner's singular `🧫️fixture` directory became canonical `🧫️fixtures`.
- Shooting and Lowpoly production command modules were renamed from fixture terminology to document/snapshot terminology. Lowpoly's public command vocabulary is now `ReplaceSnapshotJson` / `replaceSnapshotJson` / `replace-snapshot-json` across manifests, dispatch, tests, and corpora.
- Trinity retirement's Rust and TypeScript tests moved to `🧪️tests/🔬️document-retirement`; five other inline Rust test modules moved to their owners' `🧪️tests/🔬️unit` cases.
- GIS's production app descriptor now includes the real presence JSON Schema contract instead of a presence example fixture.
- GLTF material fixtures are owned as 18 feature scenario bundles such as `create-material-applied` and `reorder-textures-applied`; 36 before/after documents were preserved. Note's copied mutation vector no longer carries a physical or URI-level `🧪️tests` segment inside its fixture bundle.
- The generated Puzzle fixture catalog records actual corpus paths and digests. A missing DAG demo asset was restored from its exact prior bytes. Four framework Flow/Infinite paths moved by the framework lane were updated in plugin consumers.

## Runtime Resolution Evidence

The final `discoverTestCases` → `buildCasePlan` → `resolveFixtures` pass covered 237 scoped cases. Of those, 210 contain fixture URIs. It resolved 9,057 URI occurrences with zero missing fixtures and zero resolver errors.

Representative FEM resolution:

```json
{
  "owner": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis",
  "case": "📈️mutate-fem2d-1-analysis",
  "uri": "shared://🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/📸️snapshot/⬅️before/🔣️.json",
  "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/📸️snapshot/⬅️before/🔣️.json",
  "digest": "ff65f02f62925ab62760af0e6865c9fe"
}
```

Representative Energy resolution:

```json
{
  "owner": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any",
  "case": "🏛️export-epjson-runs-in-energyplus",
  "uri": "shared://🏛️bestest-600/🔋️model.json",
  "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏛️bestest-600/🔋️model.json",
  "digest": "539303ce44e24aae251d66e6f9dc9c74"
}
```

## Verification

- `bun …/🔌️plugins/📜️script.ts fixture-resolution-census`: 237 cases, 210 with URIs, 9,057 resolved, 0 missing, 0 errors.
- `SEMIO_ENERGY_BESTEST_REGENERATE=1 bun nx run @semio-tech/energy-model-rs:test --output-style=static`: reached Nextest; 14 tests passed and 6 failed before fail-fast skipped 5,917. The failures exposed concurrent fixture generation and unrelated store/simulation faults. This was an intermediate run before the generated model fixtures existed.
- The first exact `regenerate_bestest_fixtures` Nx run reached its single test and failed because `subset_root` appended a duplicated artifact path. The run nevertheless generated all 14 model JSONs and 14 DSL assets; the root was corrected and those exact generated bytes were placed in their canonical owner directories.
- The final exact Energy regeneration retry ran through Nx for 10m53s but did not reach the Energy test: compilation stopped in the unrelated `semio-framework-os-kernel` dependency with ten pre-existing generic `P` `Send`/`Sync` bound errors. This retry therefore supplies no passing Energy runtime claim. The zero-gap resolver independently proves that every generated Energy URI resolves to the corrected owner fixture root.

The lane's duplicate full physical scan was stopped during inspection at the coordinator's request because the coordinator was already running the same 114,308-path terminal scan. Its result is therefore owned by the coordinator report rather than claimed here.
