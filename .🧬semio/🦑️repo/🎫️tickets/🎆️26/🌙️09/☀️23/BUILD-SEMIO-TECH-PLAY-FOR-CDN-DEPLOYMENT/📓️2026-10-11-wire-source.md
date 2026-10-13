# 2026-10-11 Operation Wire Source: production authoring gap, root fix, laws

Executor: wire-src (Sonnet). Scope: the release-critical gap "authoring requires the installed original operation wire source". All cargo ran in the foreground through `🚦️cargo-slot.sh` with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/wire-src/target` (deleted at the end). Test binaries outside cargo ran with `RUST_MIN_STACK=67108864`. No git-modifying command, no ticket operation, no sub-agent.

## 1. Real scope (verified, not assumed)

The gap is wider than the generic factories. Two factory slots exist and only one of them matters for authoring:

| Slot | Where it is born | What reads its wire |
| --- | --- | --- |
| Store slot `ArtifactStore.one_item_preparation_factory` | the owner catalog (`build_document_store_owners` / `build_config_store_owners` / `build_draft_store_owners`, installed by `OriginalCatalogConstructor`) | `Store::authored_operation_rows` / `authored_operation_identity`, i.e. every `dispatch(Apply / Undo / Redo / transactions)`, genesis, composition parent rows, `authored_edit_operation_bytes` |
| App slot `VcsArtifactApp.artifact_one_item_factory` | `A::build_artifact_store_one_item_preparation_factory()` | gesture publication (`admit_apply_batch`); its wire is only read for member receipts and proposals |

Findings:

1. No SDK layer wraps anything in `operation_wire_preparation_factory`. The plugin crate installs the app factory untouched; the only wrapper user is the stdio editing contract macro (and its default `snapshot_operation_wire_source` answers `None`).
2. No catalog under `✏️s` or `🌎️hub` installs a preparation factory at all (`rg` of `with_one_item_preparation`, `canonical_authoring_store_owners`, `ArtifactCanonicalAuthoringFactory` over `✏️s` + `🌎️hub`: zero hits). The default catalog `bounded_artifact_store_owners` has no factory. Only framework flow (`FlowAuthoringFactory`) and framework dag (`ArtifactCanonicalAuthoringFactory`) catalogs carried a wire.
3. Therefore the Store slot was empty for the document, config, draft and interaction stores of every plugin app (and for every window-config partition store), regardless of whether the app factory is `config_apply`, `mutation_apply`, `Paged*`, or one of the ~45 bespoke factory impls in ~35 plugin files (raster, puzzle, process3d, fem, lowpoly, gis, energy document, generation, draw, block, vcs, forms, home, playbook, equation, space, curation, bitmap ... none of which declares `operation_wire_source`; only cad, flow, writer (delegating) and stdio do).
4. Consequence in production: `dispatch(Apply)` on the config store (`30341`), draft store (`30331`), document store (`30393`, transaction commits `35438/35795`), `Undo/Redo` (`32257/32304`, time travel), genesis (`27310`, which also runs before any catalog is installed) and the **interaction store** (`31100`, every selection/pick persists through `ApplyInLane`) all refused. Gesture publication kept working, which is why the gap hid.
5. A second layer was hidden behind the first: `InteractionConfigMutation` had no canonical tree and its hand-written `ToValue` twins (`SetInteractionState`, `InteractionState`, `DomainSelection`, `DomainHover`, `SelectionMode`) refused controlled native encoding ("0.value owner has no controlled native encoding implementation"). Fixed in the same wave.

### Per plugin / lane table

| Lane | Store | Before | After |
| --- | --- | --- | --- |
| document, every plugin artifact except flow and dag | `ArtifactStore` minted by the SDK | refused: no wire | SDK installs `ArtifactOperationWire::semantic::<A::Snapshot>()` (canonical tree + `(entity, kind)` schema pair) |
| document, flow | catalog `FlowAuthoringFactory` | worked (domain codec) | unchanged: factory codec wins, canonical is the fallback |
| document, dag | catalog `ArtifactCanonicalAuthoringFactory` | worked | unchanged (also gets the SDK wire) |
| document, cad / stdio (28 formats + semio) / writer | app factory with explicit wire | gesture + member receipts worked; Store slot refused | Store slot now has the canonical wire; their factory codecs still win where they supply one |
| config, every app | `ConfigStore` | refused | `ArtifactOperationWire::canonical()` |
| draft, every app | `DraftStore` | refused | `canonical()` |
| interaction, every app | `ConfigStore<InteractionState, InteractionConfigMutation>` | refused (no wire) and then refused again (no controlled encoding) | canonical tree + controlled output for the family |
| window-config partitions, every window kind | `ConfigStore<O::State, O::Mutation>` | refused | `canonical()` installed at partition creation |
| member stores (flow, dag, stdio-semio are the only `MemberStoreOwner`s) | catalog factory | own explicit wire | unchanged |

Compile verdict of the bound (below): every plugin artifact crate except puzzle-5d compiled with the new bound; puzzle-5d has 91 unrelated in-flight errors from another executor (rn-proc), its `Puzzle5dMutation` / `Puzzle5dConfigMutation` derive `CanonicalJsonTree` already.

## 2. Design decision and justification

Chosen: **the wire is the mutation's own canonical tree and belongs to the Store, installed once per store by the SDK; a factory can only override it with a domain codec.**

- `store::ArtifactOperationWire<M>` (`Copy`, two fn pointers: source + schema pair). `ArtifactOperationWire::semantic::<P>()` for document lanes (`M: ArtifactCanonicalJsonTree + SemanticMutation<P>`), `::canonical()` for config/draft/interaction/window lanes (`M: ArtifactCanonicalJsonTree`).
- `ArtifactStore::install_operation_wire` (exactly once, asserted) / `operation_wire()`.
- One resolution: `Store::resolve_operation_wire(factory, wire, mutation)` = factory domain codec (`operation_wire_source`) else the store wire. All authored sites use it, and every batch source (gesture publication, member wire items) captures the store wire, so member receipts and proposals also fall back to it.
- The canonical wire is `CanonicalJson { header: "op", body: ArtifactCanonicalTreeSource<M> }`: a `repr(transparent)` view (same pattern as `ArtifactCanonicalDecimalU64::from_ref`) that walks the mutation's ordinal tree by path, borrowed, no copy, no new cursor. Header and body bytes equal what `ArtifactCanonicalAuthoringFactory` already produced for dag.
- Type-level enforcement: `ArtifactApp`, `ArtifactEditor`, `ArtifactViewer` now require `Mutation`, `ConfigMutation` and `DraftMutation` to be `ArtifactCanonicalJsonTree`. An app whose mutation has no canonical tree cannot compile, which is what lets the SDK install the wire at its four store mint sites (document, config, draft, interaction in `VcsArtifactApp::with_registry_on_bus`) and at the window-config partition mint; a future mint site that forgets the install is caught by the store-level negative law and by the SDK law, not by the compiler.

Why not the coordinator's preferred option (canonical wire inside the generic factories):

- The Store slot comes from the catalog, not from the app factory, and no catalog under `✏️s`/`🌎️hub` installs a factory. A generic-factory wire would not have reached a single production Apply/Undo.
- Genesis dispatches before any catalog is installed, so a catalog/factory wire cannot serve it; a store-held wire does.
- ~45 bespoke factories (and the interaction/window/draft/config lanes that have no app factory at all) would each need per-plugin wiring. The wire is a pure function of the mutation, so it has exactly one correct owner.
- Why not "require a wire-sourced factory at install by type": every one of those 45 impls would break the release build until hand-edited and none of them has anything to declare except what the mutation already is.
- Factory-level `operation_wire_source` stays, documented as the domain-codec override (cad pack tags, flow, stdio-semio). It is not a second path for the same bytes: when a factory has a codec the bytes differ by design.

Follow-ups deliberately not done (no behavior change, only duplicate machinery): `ArtifactCanonicalAuthoringFactory`, `canonical_authoring_store_owners*`, `ArtifactCodec::of_authoring` and dag's path-to-tree bridge now overlap the store wire; space-history still needs the factory because `SpaceHistoryMutation` only has the path-based codec.

## 3. Changes (files)

Store / kernel:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs`: `ArtifactCanonicalTreeSource`, `ArtifactOperationWire`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: store field + `install_operation_wire`/`operation_wire`, `resolve_operation_wire`, `authored_operation_rows/identity`, batch source fallback (4 constructors), trait docs, exports, refusal text "authoring requires an installed canonical operation wire or an original domain codec".

Interaction family (second layer):
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`: canonical trees for `SelectionMode`, `DomainSelection`, `DomainHover`, `InteractionState` (keys and order identical to their `ToValue` twins) and `to_value_controlled` for the same four.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🦀️.rs` and `.../🔁️set-state/🦀️.rs`: `CanonicalJsonTree` derive (+ controlled output delegation for the leaf).

Derive:
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs`: uninhabited enums derive a canonical tree (`match *self {}`), so `NoConfigMutation`, `Block2dConfigMutation`, `Block5dConfigMutation`, `FlowMutation`, the host test-component mutations just derive.

SDK:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: the three assoc-type bounds (8 lines), the four wire installs, `NoConfigMutation` derive, `mod authoring_law` + re-export.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs`: partition store install.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚖️authoring-law/🦀️.rs` (new): `author_through_document_store::<A>` / `author_through_config_store::<A>`: mount the app's real catalog on a store minted like the runtime, apply, undo, redo one mutation, return the authored rows and the three snapshots, close the store.

Types that needed a canonical tree because of the bound:
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d|🖐️5d/.../✏️editor/🎚️config/🦀️.rs`, `✏️s/🔌️plugins/🌿️vcs/.../✏️editor/🎚️config/🦀️.rs`, `✏️s/🔌️plugins/🌊️flow/.../🧬️schema/🧬️mutations/🦀️.rs` (derive only), host test component `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/{🧬️schema/🧬️mutations,🗿️artifacts/🚫️snapshot-refusal/🧬️schema/🧬️mutations}/🦀️.rs` (derive only; that crate has older unrelated compile errors, none of them from this change).

Tests / laws:
- Store: `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (3 laws).
- Replication: `📡️replication/📡️wire/🧪️tests/🔬️interaction-wire/🦀️.rs` (new, registered in `📡️wire/🦀️.rs`).
- pack-json: `🎒️pack/🔤️json/🛫️encode/🧭️tree/🧪️tests/🦀️.rs` (uninhabited derive).
- Plugin: `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` (SDK law), fixture stores that mint outside the SDK now install the wire: `🧪️tests/🧪️time-travel`, `🧪️bounded-reload`, `🧾️document-archive-load-legs` (+ `InertLabelOp`/`ReloadCountedOp` derive a canonical tree and forward controlled output).
- Plugin artifacts: new `[[test]] authoring_law` (+ `✏️editor/🧪️tests/⚖️authoring-law/🦀️.rs`) in shooting, energy-model, raster-raster (their `Cargo.toml`).

## 4. Proof

### 4.1 Laws (all run)

| Law | Where | Result |
| --- | --- | --- |
| Canonical wire of every demo leaf == `"op"` + independent `serde_json` document, byte for byte; schema pair `("n", kind)` | os-kernel `the_canonical_operation_wire_of_every_demo_leaf_is_the_header_and_the_independent_serde_document` | ok |
| A store with a catalog but no factory and no wire refuses and names what it lacks | os-kernel `a_store_without_any_wire_refuses_to_author_and_names_what_it_lacks` | ok |
| The same store with the wire installed authors Apply, rows (equal to the serde oracle, forward and inverse), Undo, Redo (time travel) | os-kernel `a_store_with_no_preparation_factory_authors_apply_undo_redo_and_its_rows_through_its_installed_canonical_wire` | ok |
| Interaction family: controlled native output == `ToValue` document; canonical tree == `ToValue` document parsed by `serde_json`; absent anchor omitted | replication `interaction_wire_tests` (3) | ok |
| Uninhabited vocabulary derives a canonical tree | pack-json `an_uninhabited_vocabulary_derives_a_canonical_tree_that_is_never_walked` (whole lib 100/100) | ok |
| The runtime mints document, config, draft and interaction stores with their wire; config and interaction rows author through it; interaction row == the independent serde_json document | plugin `tool_run_tests::the_runtime_installs_its_canonical_operation_wire_on_every_store_it_mints_and_each_authors_through_it` (fixture leaked on purpose: the registered-fixture close ladder is pl-c's open law) | ok |
| Energy config edit through the energy editor's real config catalog: Apply, Undo, Redo, forward and inverse row == serde document | energy `an_energy_config_edit_authors_applies_undoes_and_redoes_through_its_real_store` | ok |
| Raster document edit (create layer) through raster's real catalog and bespoke factory family: Apply, Undo, Redo, rows | raster `a_raster_document_edit_authors_applies_undoes_and_redoes_through_its_real_store` | ok |
| Shooting document edit (create shot) through shooting's real catalog (`mutation_apply` family): Apply, Undo, Redo, rows | shooting `a_shooting_document_edit_authors_applies_undoes_and_redoes_through_its_real_store` | ok |

The row oracle in the plugin laws compares parsed values (independent `serde_json`), not bytes: key ORDER of the canonical tree is the declaration order the sealer already uses, while a `DslValue` printed by `serde_json` sorts keys. The byte-exact differential is the os-kernel law above (serde-derived twin).

### 4.2 Plugin crate `--lib`, one process per test (60 s to several minutes each pass, `RUST_MIN_STACK=64 MiB`, 120 s alarm)

| Run | ok | FAILED | ABORT | TIMEOUT / EXIT133 | tests |
| --- | --- | --- | --- | --- | --- |
| Before (tree at start, identical to the fw-tests numbers) | 759 | 420 | 46 | 0 / 2 | 1227 |
| After the wire fix only (binary built before any concurrent lifecycle edit landed) | 762 | 459 | 4 | 0 / 2 | 1227 |
| Final tree (wire fix + my 3 new plugin tests + concurrent edits from other executors in the same files) | 772 | 450 | 3 | 3 / 2 | 1230 |

The wire bucket itself: **102 tests failed with "authoring requires the installed original operation wire source" before; 0 do now** (neither run contains the string). The 102 did not turn green because each one then reaches the next layer, which is a different owner:

- `edit history insertion requires its exact mutation retirement factory` (fixture stores/apps whose owner catalog is never installed or settled): 28 (store/pl-d fixture lifecycle; a concurrent edit added `settle_store_constructor` to `new_registered_app`, which already moved the toy tool-run tests past their first `Apply`)
- tool-run jobs that never settle / driver faults ("never settled; state Some(Running)", "cancellation token and terminal dynamic scaffold retain their exact retirement authority"): ~25 (pl-b/pl-c, tool-run)
- `registered fixture did not reach its exact terminal-empty witness` (close ladder): 11 (pl-c)
- single digits: window config save camera, archive status, interaction inline verb, extension descriptor close.

Status flips in the final run vs the baseline: 36 ABORT to FAILED, 7 ABORT to ok, 3 FAILED to ok (extension invocation failure and `reactor::jobs` tests, from other executors' concurrent edits, not from this change), 3 FAILED to TIMEOUT (tool-run tests that now progress past the wire and spin to the 120 s alarm), 0 regressions of any previously passing test.

### 4.3 os-kernel `--lib` (1466 tests, one process per test)

1348 ok / 104 FAILED / 8 ABORT / 6 TIMEOUT. Baseline (fw-tests): 1346 ok / 105 / 8 / 4 of 1463. The only status changes are the 3 new laws (ok), 4 stamped-publication tests that used to time out and now fail fast with the known "fixture authoring catalog prepares no retained gesture", and `hot_path_tests::local_steps_cost_o_change_however_long_the_history`, which passes alone in 179 s and exceeded the 120 s alarm only under the 19-load-average fleet.

### 4.4 Compile matrix (all `Finished`, 0 errors)

| Target | native `--lib` | `wasm32-wasip2 --lib` |
| --- | --- | --- |
| `semio-framework-os-kernel` | ok | ok |
| `semio-framework-plugin` (also `--features artifact-app-testing`) | ok | ok |
| `semio-s-artifact-shooting-shooting` | ok | ok |
| `semio-s-artifact-energy-model` | ok | ok |
| `semio-s-artifact-raster-raster` | ok | ok |
| `--tests` of os-kernel and plugin | ok | not run |

Bound blast radius: `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --lib --keep-going`: no `ArtifactCanonicalJsonTree` error remains in any plugin artifact or hub composition. Hub crates that did not compile are independent of this change: `semio-framework-os-kernel-db` and `semio-framework-os-mcp` (older migration debt), the nine `semio-hub-stdio-*` family roots (`plugin_exports!(plugin, X)` is still the 2-argument spelling, the macro now takes the mounted policy first), and `semio-s-artifact-puzzle-5d` (rn-proc in flight; blocks `semio-hub-puzzle`). The first run of that check showed the five types that really lacked a tree (block2d/5d config, vcs config, flow mutation, wires mutation); the first four were fixed here, wires got its own tree from a concurrent edit.

## 5. Risks and open items

1. Stores minted outside the SDK still need either a factory wire or `install_operation_wire` before they can author: `os/🏃️run` bootstrap `persist_run` (its `dispatch` call is also stale, no `identity` argument), `os/🖥️host` `OsWorkflowStore`, the space-history meta store (has the factory), `ArtifactCodec` reduction stores (`bare` / `of`), mcp `HeadlessWorkspace`, `create_member_store` / `open_member_store` (only flow, dag and stdio-semio are members and carry their own wire). Recommended follow-up: an `ArtifactStore::new_authoring` constructor in the `M: Tree` impl block for these sites.
2. `InteractionConfigMutation` / `InteractionState` have no controlled FROM-value decoding yet; the encode side (authoring) is done. If interaction history is ever reloaded through the controlled decoder it will hit the same "no controlled native construction implementation" refusal.
3. The plugin SDK law leaks its fixture app on purpose; replace `std::mem::forget` by the close call once the registered-fixture close ladder (pl-c) is green.
4. The canonical wire lists object keys in the order of the mutation's tree (the sealer's order, the declaration order for derived records; the hand-written interaction twins mirror their `ToValue` order exactly). A TS or other-language twin that re-derives operation identities must use the tree order of each mutation, not a sorted order.
5. Identity strings of operations authored through the new default are new (nothing could author before); factories that supply a domain codec keep their bytes.

## 6. Running these targets

- Plugin lib: `cargo test -p semio-framework-plugin --lib --no-run`, copy the printed executable out of the shared build dir (other executors rebuild the same path), then run `--exact <name> --test-threads=1` per test with `RUST_MIN_STACK=67108864`, `CARGO_MANIFEST_DIR` and `SEMIO_TEST_ARTIFACT_DIR` set to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust`.
- Artifact laws: `cargo test --manifest-path <artifact>/Cargo.toml -p <crate> --test authoring_law`.
- Raw results: `.🧬semio/🦑️repo/⚡️cache/play-fleet/wire-src/{each-base,each-new,each-final,kernel-each}/results.tsv` (`status<TAB>test<TAB>md5`; the per-test logs, the copied test binaries and the private target dir were deleted for disk), the 102-test baseline list `base-wire-tests.tsv`, cargo logs `ck-*.log`, `f-*.log`, `g-*.log`, `h-*.log`.

## 7. Phase 2: the wire is guaranteed by type, every minted store authors

Supersedes risks 1 and 2 of section 5.

### 7.1 Design

- A store can only author through a catalog (`reserve_edit_history_slot` refuses without the mutation retirement factory). So the wire now lives IN the catalog: `DocumentStoreOwners.operation_wire`, filled by every owner-catalog constructor (`plain_document_store_owners`, `bounded_artifact_store_owners*`, `funded_bounded_artifact_store_owners`, `ArtifactCodec::of/bare/assemble`, plugin `bounded_config_store_owners` / `bounded_document_store_owners`, dag `member_store_owners`, `space_history_store_owners`). `install_document_store_owners_exact` copies it into the store with `get_or_insert`, so an explicit domain codec or earlier wire still wins.
- The constructors are bound `M: ArtifactCanonicalJsonTree`: a mutation vocabulary without a canonical tree cannot get a catalog, hence cannot record history, hence cannot be minted as an authoring store. A store without any catalog is "never authors by design" and says so in the refusal text.
- Duplicate paths deleted (greenfield, no compat): `ArtifactCanonicalAuthoringFactory`, the canonical-codec-catalog trait/structs, `ArtifactCodec::of_authoring/bare_authoring`, the path-based space-history `🔏️canonical` module (the aggregate and 6 leaves derive `CanonicalJsonTree`; the SDK's per-store `install_operation_wire` calls for config, draft, interaction and window partitions became redundant and were removed, the document install in `with_registry_on_bus` stays for the app slot).
- Codec reduction stores (`apply_ops_binary_impl`, `replay_envelopes_impl`) now build through `funded_bounded_artifact_store_owners`, so they carry the wire like every other catalog.

### 7.2 Controlled decode

- `InteractionConfigMutation` / `InteractionState` / `DomainSelection` (replication `📡️wire`) and `InteractionSetState` (plugin) now derive `ToValue, FromValue, CanonicalJsonTree` (with `#[value(transparent)]` where it is a newtype) instead of hand-written pairs, so `from_value_controlled` and `retire_decoded` exist. Their serde twins list fields alphabetically now (derive order); nothing persisted this shape yet.
- Fixtures with hand-written pairs gained `from_value_controlled` / `retire_decoded` (time-travel and bounded-reload `TestMutation` family; `FailClosedOp` Tree delegates to `operation`).
- The derive supports uninhabited enums (`NoConfigMutation`).

### 7.3 Laws (all run, one process per test, `RUST_MIN_STACK=64 MiB`)

| Law | Result |
| --- | --- |
| OS workflow store `host_core::host::tests::the_os_workflow_store_authors_apply_undo_redo_and_its_rows_through_its_catalog_wire` (`--features os-host-full`) | ok (plus `workflow_host_preserves_the_opened_actor_context` ok) |
| Codec reduction store `document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store` (now through `ArtifactCodec::of`) | ok |
| By-type: `a_store_without_a_catalog_cannot_author_and_a_catalog_alone_gives_it_the_canonical_wire` (replaces the removed "store without any wire" law, whose premise is now impossible for a catalog store) | ok |
| Space-history leaf trees equal their value documents parsed by an independent serde_json | ok |
| Replication interaction wire: decode round trip, serde twin equality, refusals (7); whole replication lib 346 passed | ok |
| Plugin `the_interaction_mutation_round_trips_its_serde_json_document_through_the_plain_and_the_controlled_decoder` | ok |
| Plugin SDK `the_runtime_installs_its_canonical_operation_wire_on_every_store_it_mints_and_each_authors_through_it` (still green with the redundant installs removed) | ok |
| `authoring_law` energy-model, raster-raster, shooting (rerun after the catalog change) | 1/1 each ok |

### 7.4 os-kernel `--lib` (1465 tests, one process per test, 300 s alarm)

Full run: 1349 ok / 103 FAILED / 8 ABORT / 5 TIMEOUT. Versus the phase-1 run: two tests FAILED to ok (`apply_undo_redo_transfers_each_snapshot_root_to_one_exact_retirement_owner`, `prefix_ring_evictions_retire_through_the_snapshot_factory`), one TIMEOUT to ok (`hot_path_tests::local_steps_cost`), one new ok (space-history tree law), two path-based tests deleted with their module. The single regression, the "store without any wire" law, was premise-obsolete by design and is replaced by the by-type law above (re-run alone: ok); net 1350 ok / 102 FAILED. The failures that remain (including `prepared_operation_wire_borrowed_json/text`) fail identically in the pre-phase-1 baseline.

Plugin `--lib`, 78 tests that touch wire, history, interaction, time-travel and bounded reload: no status got worse versus phase 1 (9 FAILED/ABORT to ok; 3 TIMEOUT to FAILED fast on the known retirement-frontier / displaced-owner / initializer-outcome faults of the other agent's buckets, none mention the wire). 24 ok / 54 FAILED, all 54 failed in the pre-phase-1 baseline too.

### 7.5 Compile matrix after the final edits

| Target | native `--lib` | `wasm32-wasip2 --lib` | `wasm32-unknown-unknown --lib` |
| --- | --- | --- | --- |
| os-kernel, plugin, os host (also `--features space-guest` wasip2), os-infinite, os-flow | ok | ok | os-flow, os, actor, editor, node-graph ok |
| replication | ok | ok | |
| energy-model, raster-raster, shooting | ok | ok | |
| dev-cad, jack-lsp, spatial-kernel session (`browser-publication`) | ok (hub check) | | ok |
| test targets: os-kernel `--lib`, plugin `--lib`, os host `--lib --features os-host-full`, the three `authoring_law` | ok | | |

Hub workspace check (`--workspace --lib --keep-going`): everything compiles except three crates whose errors are unrelated to the wire: `semio-framework-os-kernel-db` (66, `!`/`SharedUtf8` drift), `semio-framework-os-mcp` (37, own errors plus the broken db dependency) and `semio-s-artifact-puzzle-5d` (91). `semio-framework-os-run` has lib API drift (Budget/TurnResult/job) unrelated to the wire. Consequently native and wasip2 compile of mcp and of hub-puzzle (via puzzle-5d) and of the os-run bootstrap edit could not be proven.

### 7.6 Per-site status of "stores minted outside the SDK"

- `OsWorkflowStore` (`funded_bounded_artifact_store_owners`): authors, law above.
- Codec reduction stores: authors (catalog), law above.
- Space-history meta store: `space_history_store_owners` uses the funded catalog; leaf trees proven.
- Dag member stores: `DocumentStoreOwners::admit_source_constructor` carries the wire.
- Probe stores: mcp workspace `ProbeMutation` derives the tree (edit not compiled, mcp blocked).
- os-run `persist_run`: rewritten to install the funded catalog, create an `EntityIdentityAuthority`, dispatch Apply with identity, snapshot and close. NOT compiled (crate blocked by unrelated drift): treat as unverified.

### 7.7 Risks and open items

1. mcp and os-run edits are unverified by the compiler (blockers above).
2. The SDK law still leaks its fixture app (`std::mem::forget`) pending the registered-fixture close ladder.
3. Serde twin field order of `InteractionState` / `DomainSelection` is now alphabetical; any external twin must follow the derive order.
4. The catalog Tree bound is a breaking source change for any out-of-tree mutation type; in-tree all call sites were fixed (many derives added to test fixtures and to Author, SpaceMemberPin, SpaceCheckpoint, SpaceAlternative, block 2d/5d config, vcs config, flow mutation, host test mutations).
