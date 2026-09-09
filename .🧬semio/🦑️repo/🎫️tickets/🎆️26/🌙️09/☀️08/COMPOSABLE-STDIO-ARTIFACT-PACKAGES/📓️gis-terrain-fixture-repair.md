# GIS Terrain Fixture and Ownership Repair

## Red evidence

The retained default Terrain binary in `🗑️generated/gis-terrain-default-full-pre-fixture-repair.txt` ran all 50 tests with `--no-fail-fast`: 46 passed, 4 failed, 0 skipped in 0.683 seconds, and Nextest exited 100. The four failures were:

- the document Store round trip failed because Apply had no exact mutation retirement factory;
- both mutation vectors failed their canonical JSON law first on their `before` snapshot because a typed `f64` encoded as `1.0` while the fixture declared integer `1`;
- structural correspondence failed because its catalog lookup still started from the former GIS plugin manifest layout.

The imported-features case checks both `before` and `after` in one loop. Its first failure concealed the same integer spelling in `after`, so all three affected typed snapshots now declare `1.0`.

## Repair

The Store test installs `bounded_document_store_owners::<GisTerrainSnapshot, GisTerrainMutation>()` before dispatch. This is a type-specific catalog with separate exact snapshot, initial snapshot, and mutation retirement factories. The test then drives `ArtifactDocumentStoreDisposer` with one item and one page per step, rejects blocked or awaiting-input states for its unshared fixture, proves terminal emptiness, and only then drops the Store.

The correspondence test now starts at the extracted Terrain package's own `CARGO_MANIFEST_DIR`, reaches its taxonomy owner with `../../🏅️standards/…`, and reads the language-neutral catalog at `../../🔮️oracle/🔣️.json` from the mutation root.

The component fixture audit found a real lifecycle mismatch with the registered Map fixture. Terrain commands publish both document and config changes, but `Gis3dPlayApp` supplied retained preparation factories without the corresponding owner and disposer factories. Its testkit also built registryless, unbound instances and dropped them without the framework close protocol. Terrain now exposes exact bounded document/config owner and disposer factories, the canonical no-draft owner/disposer pair, bounded local and peer presence retirement with a disposer whose terminal is the declared default presence, and the canonical no-transient disposer/root retirement pair. The shared fixture constructor builds the actual registry, binds the local instance, and every fixture consumer drives the app to its terminal-empty close witness.

The registered typed-command API returns an admission receipt before its retained operation publishes. The shared framework testkit now provides `settle_registered_typed_operation`: it advances one maintenance item and one publication unit per turn, enforces the exact envelope-page byte grant, acknowledges each result page with its exact token, drains effects/events/UI scope, acknowledges any returned local-interaction page, and waits for the operation's terminal-empty pending witness. Its receipt exposes the actual publication lanes and forwarded output. Terrain's dispatch fixture uses this protocol, so command tests observe completed Config or Artifact publication instead of reading the intentionally empty admission receipt. Dispatch and render also target the manifest's real `gis3d-main` window through a `ViewModel` carrying both the active window id and exact window instance. The existing neutral retained-command fixture declares the ordered Config/Artifact, UI, and Terminal lanes for both commands.

## Language-neutral and independent checks

The artifact's committed oracle catalog records that no external geospatial library is authoritative for its scalar exaggeration and opaque imported JSON string. Its accepted differential oracle is the independent Python implementation at `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏔️mutate-gisterrain-1/🐍️.py`.

Independent checks completed without compiling Terrain:

```text
[DEBUG] independent Bun JSON oracle accepted 3 canonical Terrain snapshots
[DEBUG] independent Python JSON oracle preserved f64 spelling for 3 Terrain snapshots
[DEBUG] independent Python oracle matched 2 Terrain mutation kinds to owner descriptors
[DEBUG] owner-relative terrain oracle path exists
[DEBUG] Terrain neutral publication oracle accepted 2 commands / 6 ordered lanes
```

A source search found no remaining integer `1` spelling for `exaggeration` in the Terrain mutation JSON tree. `git diff --check` passes for the Terrain owner tree.

The coordinator owns the already planned ordinary Nx default and component reruns. This lane did not start a duplicate Terrain native build and does not claim a native pass.

## Exact files

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏔️exaggeration/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️view/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-exaggeration/🧪️tests/⛰️raises-exaggeratio-8ebcb8/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/📥️imports-harbor-94979a/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/📥️imports-harbor-94979a/📸️snapshot/➡️after/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
