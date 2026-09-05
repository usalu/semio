# Lane J — `s` studio/home/space-index interactive-job migration

Ticket `26/09/05/S-END-TO-END`, lane J (`space-studio-migration`), Opus. Scope: `✏️s/🔌️plugins/🪐️space`
only (plus this plugin's `📋️project.json` / `📜️script.ts` and the shared `.vscode/🧩️launch.seed.jsonc`).
No framework crate was edited.

## Headline

The boot trap `app-definition.interactive-job-classification: unclassified interactive command
's.space.studio@1/*#editor:setAppRegistrations'` is a **stale-core symptom**, not the current source
state — current source classified that id `BatchOnlyPendingRewrite`, and
`validate_interactive_job_classification` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:952-968`) only
rejects `Unclassified`. The cached 09-02 `semio_s_plugin_space` core predates line 975 entirely.

That does not make the lane unnecessary — it makes it **necessary for a different reason**. Once the
core is rebuilt, the same command dies one layer later: `dispatch_command` runs
`validate_ui_dispatch_classification` (`🔌️plugin/🦀️.rs:12041`), which admits only `Migrated`. And two
further, previously unreported blockers were found that would have killed the very next boot anyway:

1. **Every one of the three `🪐️space` apps was unconstructable**, independently of the classification.
   `validate_tool_job_rows` (`🔌️plugin/🦀️.rs:12232-12296`) requires exactly one proof row per migrated
   id, each joined to a live registered factory by **owner witness, controller id, document schema and
   exact `ToolExecutionContract` equality**; the join failure is an `expect(...)` at app construction
   (`:19551`), i.e. a guest panic, not a degraded action.
   - `s.space.studio@1/*#editor` and `s.space.home@1/*#editor` both declared `controller:` as their
     **manifest UI controller id** (`"s-play"` / `"s-home"`) while `tool_job_registration` is invoked
     with the **surface app id** (`&app_id`, `:19551`). Every working plugin in the repo (lowpoly
     `:1610`, generation3d `:742`) uses the surface app id. Both were guaranteed
     `interactive-job.catalog-authority`.
   - Home additionally declared `contract: bounded_first_step(8_192, 64, 1, 65_536, 7_500)` in its
     proof macro while its factory published `home_retained_contract()` =
     `resumable(131_072, 256, 1, 16_777_216, 7_500, 1, 1)` — the equality join could never succeed.
   - `s.space.space@1/*#editor` (the space-index editor, what Home opens when you open a space) had
     **12 `Migrated` ids and no factory, no proof catalog and no store preparation factory at all** →
     `interactive-job.catalog-incomplete` at construction.
2. **The studio's config lane rejected every mutation `openSpace`/`openInstance`/`spawnApp` emit.**
   `space_config_mutation_bytes` admitted only six variants; `SetSpaceId`, `SetActiveNode`,
   `SetClipboard` and `SetFocusedNode{Some}` all fell into the `_ => Err(...)` arm.

All of that is fixed. 47 ids across the three apps are now `Migrated`, each backed by an owned
`ArtifactOwnedToolJobFactory` with a publication contract read off the handler's own `Emit`.

## Audit — every `BatchOnlyPendingRewrite` in `🪐️space`, and who dispatches it

Source of truth for "the shell dispatches it": `grep` by id over
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**` (`.ts`/`.tsx`, tests excluded).

### `s.space.studio@1/*#editor` — `⚙️engine/🪐️space/🦀️.rs` (40 declared ids)

| id | was | now | lane | dispatched by |
|---|---|---|---|---|
| `setAppRegistrations` | BatchOnly | **Migrated** | HostOnly | `🏛️ShellHost/🟦️.tsx:3266-3268` — `handleCommand` on every catalog change, i.e. **every boot** |
| `openSpace` | BatchOnly | **Migrated** | Config | `🏛️ShellHost/🟦️.tsx:3955` |
| `openInstance` | BatchOnly | **Migrated** | Config | `🏛️ShellHost/🟦️.tsx:3961`, `🕸️NodeGraph/🟦️.tsx:732,1061,1952,2367` |
| `importSpacePackPayload` | BatchOnly | **Migrated** | HostOnly | `🏛️ShellHost/🟦️.tsx:7490` |
| `spawnApp` | BatchOnly | **Migrated** | Artifact + Config | `🏛️ShellHost/🟦️.tsx:7178` (the program launcher this ticket's `verify catalog` smoke drives), `🕸️NodeGraph/🟦️.tsx:989,2341` |
| `goHome`, `navigateVirtualFileSystemNode`, `setActivePanelTab`, `nodeGraphViewport`, `presenceHeartbeat`, `workflowEngagementInput`, `compiledDagEngagementInput`, `closeFocusedInstance`, `setActiveExample`, `importSpacePack` | Migrated | Migrated | unchanged | already covered |
| `patchParameter`, `addParameter`, `removeParameter`, `moveMediaNode`, `connectMediaPorts`, `disconnectMediaEdge`, `removeAppInstance`, `deleteSelection`, `copyAppInstance`, `duplicateAppInstance`, `pasteAppInstance`, `renameAppInstance`, `patchMediaNodes`, `patchAppInstances`, `bindParameterField`, `unbindParameterField`, `reorganizeWorkflow`, `workflowEngagementSubmit`, `compiledDagEngagementSubmit`, `nodeGraphEdit`, `exportMedia`, `importMedia`, `importMediaPayload`, `exportStudioPack`, `exportStudioDsl` (25) | BatchOnly | **BatchOnly, honestly** | — | **zero** renderer call sites |

Why those 25 stay batch-only, honestly: they are the workflow-graph and media editing surface. Each
either walks the whole graph, the whole selection, the whole parameter set or a whole media/pack
payload in one dispatch (`delete_selection`, `reorganize_workflow`, `paste_app_instance`,
`export_studio_pack`, …), so none of them has a defensible bounded first step yet — the work is a real
per-command reducer/extent design, not a label. They are also dispatched only from the studio's own
Workflow window, which is not on the boot path or the catalog-smoke path this ticket gates. Each row
carries its own one-sentence blocker in the fixture (`⚙️engine/🪐️space/🧪️fixtures/🧫️retained-command-limits/🔣️.json`).

### `s.space.home@1/*#editor` — `🗿️artifacts/🏠️home/…/✏️editor/🦀️.rs` (18 declared ids)

All six remaining `BatchOnlyPendingRewrite` ids are now `Migrated`; Home is 18/18.

| id | lane | why it is reachable in normal use |
|---|---|---|
| `importSpace` | Artifact | `🏛️ShellHost/🟦️.tsx:7497`, plus keybinding `mod+o` |
| `foldDirectoryEvents` | Config | `🏛️ShellHost/🟦️.tsx:4864` — the live `/directory/ws` lane |
| `createStudio` | Artifact | Home's own window action + keybinding `mod+n` |
| `deleteVirtualFileSystemNode` | Artifact | Home's own VFS row action |
| `renameSpace` | HostOnly | Home's own row action (`🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:83`) |
| `bindSpaceFile` | HostOnly | Home's own window action |

### `s.space.space@1/*#editor` — `🗿️artifacts/🪐️space/…/✏️editor/🦀️.rs` (14 declared ids)

`renameArtifact` and `foldDirectoryEvents` were `BatchOnlyPendingRewrite`; both are now `Migrated`, and
the whole app got the factory it never had. 14/14.

## File:line changes

### Shared — `✏️s/🔌️plugins/🪐️space/🦀️.rs` (new region `🧵️RetainedStore`, `:575-762`)

One generic exact one-item Store preparation authority for every lane of all three apps, replacing what
would otherwise have been three hand-written copies (the shape lane D proved out for norm, generalized
over `protocol::Mutation::diff`/`inverse` + `MutationDiff::apply`):

- `space_retained_edit<M>` — the single `protocol::Edit<M>` one publication step commits.
- `space_retained_mutation_bytes<M>` / `admit_space_retained_mutation<M>` — `OpBinary`-encoded footprint.
- `prepare_space_retained_one_item<P, M>` — post state + declared inverse.
- `pub struct SpaceOneItemPreparationFactory<P, M>` + `SpaceOneItemPreparation<P, M>` — the
  `store::ArtifactStoreOneItemPreparationFactory` / `…Preparation` impls.
- `pub fn space_retained_store_preparation<P, M>(prefix, maximum_bytes)` — the `ArtifactApp`/
  `ArtifactEditor` store override, one line per lane.

New test region `🧪️InteractiveJobCatalogTests` (`:865-1010`), see **Tests**.

### Studio — `⚙️engine/🪐️space/🦀️.rs`

| what | line |
|---|---|
| `SPACE_BOUNDED_TOOL_IDS` 10 → 15 (`spawnApp`, `openSpace`, `openInstance`, `importSpacePackPayload`, `setAppRegistrations`) | `:294-311` |
| `SPACE_BATCH_ONLY_TOOL_IDS` 30 → 25, with the honesty rationale as its docstring | `:312-320` |
| `SPACE_BOUNDED_RAW_BYTES` 65 536 → 4 MiB (the catalog JSON and the `.pack` data URL both travel this wire) and new `SPACE_BOUNDED_OUTPUT_BYTES` | `:322-330` |
| `space_bounded_reduce` now takes `interaction` and routes `openInstance` through `open_instance::open_with_selection`, so the retained path keeps the live-`graph`-selection fallback the macro-generated 3-arg `dispatch` cannot see | `:340-362` |
| `PUBLICATION_CONTRACTS` + 5 rows | `:425-429` |
| config envelopes widened (`768→65_536`, `64→256` items, `96→8_192` text) | `:434-441` |
| `space_config_mutation_bytes` + `prepare_space_config` admit `SetActiveNode`, `SetFocusedNode{any}`, `SetSpaceId`, `SetClipboard` | `:483-485`, `:510-513` |
| proofs macro: `controller` `"s-play"` → `"s.space.studio@1/*#editor"`, per-tool literal contracts → one `contract: space_bounded_contract()`, 10 → 15 tools | `:635-667` |
| new `build_artifact_store_one_item_preparation_factory` | `:669-671` |
| 5 `.action_interactive_job(…, Migrated)` flips | `:989`, `:1019-1021`, `:1025` |
| `SpaceCommandJobFactory` made `pub` (the plugin-root catalog test reads its `TOOL_IDS`/`PUBLICATION_CONTRACTS`) | `:364` |

`⚙️engine/🪐️space/🎮️commands/🔍️open-instance/🦀️.rs:18` — `open_with_selection` made `pub`.

### Home — `🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

| what | line |
|---|---|
| `HOME_RETAINED_TOOL_IDS` 12 → 18 | `:61-64` |
| `HOME_RETAINED_PUBLICATION_CONTRACTS` + 6 rows | `:83-88` |
| `home_retained_extent` arms for the six new commands | `:113-118` |
| `HomeRetainedCommandJobFactory` made `pub` | `:130` |
| Home's config preparation admits `FoldDirectoryEvent` — preflight, `begin`, and an `advance` arm that takes the post state from the mutation's own diff and its declared `Snapshot` inverse | `:243`, `:255`, `:301-306` |
| proofs macro: `controller` `"s-home"` → `"s.space.home@1/*#editor"`, `contract:` literal → `home_retained_contract()`, 12 → 18 tools | `:386-402` |
| new `build_artifact_store_one_item_preparation_factory` | `:404-406` |
| 6 `.action_interactive_job(…, Migrated)` flips | `:625-641` |

### Space index — `🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

New region `🧵️RetainedCommands` (`:154-265`): `SPACE_INDEX_RETAINED_TOOL_IDS` (14),
`…_PAYLOAD_SCHEMA` = `s.space.index.tool-command.v1`, `…_RAW_BYTES` = 128 KiB,
`…_PUBLICATION_CONTRACTS` (4 × Artifact, 2 × Config, 8 × HostOnly), `space_index_retained_contract`,
`space_index_retained_extent`, `space_index_retained_reduce`, and
`pub struct SpaceIndexRetainedCommandJobFactory` with its `ToolJobFactory` +
`ArtifactOwnedToolJobFactory` impls.

In `impl ArtifactEditor for SpaceIndexEditor` (`:290-360`): the proofs macro
(`controller: "s.space.space@1/*#editor"`, `document_schema: "s.space"`,
`contract: space_index_retained_contract()`, 14 tools), both store preparation overrides,
`register_tool_job_factories` and `build_tool_job`. Two flips (`renameArtifact`, `foldDirectoryEvents`)
at `:498`, `:508`.

### Fixtures (language-agnostic owners) + registration

- `⚙️engine/🪐️space/🧪️fixtures/🧫️retained-command-limits/{🔣️.json,🧬️.schema.json}` — 5 routes flipped to
  `bounded`/`migrated`, 5 publication contracts appended (schema `prefixItems` 10 → 15), limits
  `maxRawBytes`/`maxOutputBytes` → 4 194 304, `oracle.expected` → `{routes:40, bounded:15, batch:25,
  migrated:15}`.
- `🗿️artifacts/🏠️home/…/🧪️fixtures/🧫️retained-command-limits/{🔣️.json,🧬️.schema.json}` — the six flipped
  rows now carry `Migrated` + their real lanes and an empty blocker; the missing `manageSpace` row was
  added (17 → 18 routes), so the fixture is finally complete against the app.
- `🗿️artifacts/🪐️space/…/🧪️fixtures/🧫️retained-command-limits/{🔣️.json,🧬️.schema.json}` — **new**, same
  shape as the studio's: 14 routes, 14 publication contracts, its own pinned schema.
- `📦️packages/🦀️rust/📜️script.ts` — new `interactiveJobCatalogOracle(repoRoot)` + the
  `interactive-job-catalog-check` script (`--native` runs the five Rust laws through `runExactCargoLaws`).
- `📦️packages/🦀️rust/📋️project.json` — targets `interactive-job-catalog-check` and
  `interactive-job-catalog-native-check`.
- `.vscode/🧩️launch.seed.jsonc:3754-3782` — `⚖️gate🧵️space-interactive-job-catalog` and its `🦀️native`
  twin, group `4_gate`, orders 411.044/411.045 (immediately after the sibling `home-directory-*` gates).

## Tests

### Rust — `✏️s/🔌️plugins/🪐️space/🦀️.rs`, `interactive_job_catalog_tests`

Walks the built app definitions, not the source text:

- `studio_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory`
- `home_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory`
- `space_index_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory`

Each collects every id an interactive dispatch can address (`window_kinds[].actions`, `commands`,
`modes[].commands`) with its `semantics.execution.interactive_job`, then asserts: no `Unclassified` id
survives; the app-owned migrated set equals the fixture's; the factory's `TOOL_IDS` equal that set;
`PUBLICATION_CONTRACTS` covers exactly it; the fixture's HostOnly-only ids equal the factory's; and
`bounded_first_step_tool_proofs().len()` equals the tool count. The studio/home tests additionally name
the shell-dispatched ids explicitly, so removing one of them from the factory fails loudly.

- `tool_proof_catalogs_match_the_runtime_identity_they_are_joined_against` — the proof macro only takes
  literals, so the literals are pinned here against `S_PLAY_APP_ID`, both editors' `DIALECT.artifact_kind`
  and all three `DOCUMENT_SCHEMA`s (app vs factory).
- `manifest_plugin_id_matches_the_cargo_component_package` (coordinator's addendum) — parses
  `[package.metadata.component] package` out of `📦️packages/🦀️rust/Cargo.toml` via `include_str!`, splits
  `<namespace>:<id>`, and asserts `plugin().manifest.plugin_id == id`. No literal duplication of `"s"`;
  a repeat of the 03:53 `"space"` regression fails natively in seconds instead of after a 90-minute
  wasm build.

Updated in `⚙️engine/🪐️space/🦀️.rs`: `retained_command_catalog_matches_the_serde_json_oracle`
(10/30/10 → 15/25/15, hostOnly 4 → 6), `retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures`
(+2 expected hostOnly ids), and `retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one`
(the four session mutations are now asserted **admitted** and round-tripped through `prepare_space_config`;
`SetLocale` is the remaining rejected witness).

### TypeScript — `interactiveJobCatalogOracle`

Third-party oracle (`ajv` 2020, already a dev dependency of this script). For each of the three apps it
validates the fixture against its own schema, proves every migrated row has at least one publication
lane and no duplicate ids, cross-checks the Rust source for `factory_type:` and for
`controller: "<surface app id>"`, asserts no migrated id is still declared
`BatchOnlyPendingRewrite` in source, and finally compares the fixture against the **committed**
`✏️s/🔌️plugins/🪐️space/🔣️.json` descriptor: a descriptor row that publishes an `interactiveJob`
disagreeing with source is a hard failure, while a row with no `interactiveJob` at all is counted and
reported as descriptor staleness.

## Commands and real outputs

```
$ cd ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust && bun ./📜️script.ts interactive-job-catalog-check
interactive-job-catalog: descriptor rows without an interactiveJob disposition: 47 (regenerated by `describe` after a wasm build)
interactive-job-catalog-check: checks=12 clean
```

47 = 15 studio + 18 home + 14 space-index. The committed descriptor carries **no** `interactiveJob`
field anywhere — it predates the field entirely, so it cannot be used as evidence about any app's
classification until the coordinator's rebuild re-emits it. That is exactly the drift this gate exists
to expose, and it turns into a real assertion the moment the descriptor is regenerated.

### `cargo check` / `cargo test` — BLOCKED, not skipped

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e \
    cargo check -p semio-s-plugin-space --lib --keep-going --message-format=short
(no output; still running after 3 h)
```

The lane's `cargo check` (pid 25855) never reached this crate. `semio-s-plugin-space` depends on
`semio-s-plugin-stdio`, and a peer session's `rustc --crate-name semio_s_plugin_stdio` in the **same**
shared `target-s-e2e/debug` (pid 18527) has been compiling for **4 h 39 m at 1.8-3.1 % CPU** under a
load average of 100-145 (four other cargo/rustc fleets on the box: sourcing, demonstrator, block, vcs).
Nothing in this lane can compile until that unit lands; starting a private target directory would only
re-do the same 4 h stdio compile from scratch.

**So: this lane's Rust is written and reviewed but NOT compile-verified.** Do not treat it as green.
The check is still queued in `target-s-e2e`; re-run exactly the command above (and then
`cargo test -p semio-s-plugin-space --lib -- interactive_job_catalog_tests`) once the stdio unit
finishes. The TS oracle above is real, executed output.

## Blockers / notes

- **Machine contention is the only thing between this lane and a verdict.** See above.
- **The wasm rebuild is still required and still the coordinator's.** Every finding here is invisible
  to the running shell until `semio_s_plugin_space` is rebuilt: the cached core predates the
  classification calls entirely.
- **Boot order after the rebuild.** `setAppRegistrations` is dispatched on every session, so the studio
  app must construct before anything renders; Home constructs immediately after (directory bootstrap
  sends `setClient` then `applyDirectoryEventPage`); the space index constructs the first time a space
  is opened. All three were failing at construction before this lane; all three now declare a complete,
  self-consistent proof catalog.
- **`ArtifactBoundedFirstStepProof`'s fields are private to the framework crate** (no accessors,
  `🔌️plugin/🦀️.rs:12609-12619`), so a plugin-side test can only assert the row *count*. The identity a
  row carries is pinned indirectly, by asserting the macro's literals against the runtime constants
  they must equal. A framework-side accessor (or a `#[cfg(test)]` witness) would let a plugin test
  assert the join directly; worth a framework lane.
- **`SPACE_BOUNDED_RAW_BYTES` is now 4 MiB.** If the live catalog JSON for 59 plugins ever exceeds that,
  `admit_command_json` rejects `setAppRegistrations` with `ToolDispatchError::RawWireLimit` instead of
  faulting obscurely — the ceiling is one constant, and the fixture pins it.
- **Home's undo granularity changed for `foldDirectoryEvents`.** Its declared inverse is a whole-config
  `Snapshot`, which is what the mutation itself declares (`🎚️config/🦀️.rs:396-398`); the retained lane
  publishes one store edit per mutation, which is the framework's one-item publication contract, not
  something this lane chose.
- **Not touched:** `✏️s/🔌️plugins/🪐️space/🦀️.rs:603` (`Plugin::<SpaceApps>::builder("s")`) beyond
  reading it; the descriptor pair (`🔣️.json` / `🛂️.descriptor.semio`) was **not** hand-edited.
