# Result: relocate the hub GIS map binding (slice C, `relo-hubgis`)

Executor: Opus, foreground, no sub-agents, no modifying git command. Brief: `📓️RELOCATION-BRIEF.md`, slice C. Paths: `hub/` = `🌎️hub/`, `RT/` = `🌎️hub/💡️inference/🏃️runtime/`.

## 1. Verdict

**Done for the binding and everything cohesive with it; statically verified; not compiled.** The hub inference runtime, its WAL verifier, its catalog and its schema module no longer name any GIS crate. `semio-hub` dropped the three direct GIS/stdio plugin dependencies from `[dependencies]`. The GIS domain code now lives in the GIS plugin's hub-side crate `semio-hub-gis` (`hub/🧩️compositions/🌍️gis/💡️inference/`) and reaches the runtime through two neutral ports in a new hub-layer crate, `semio-hub-inference-binding` (`hub/💡️inference/🔗️binding/`). The retained fixed-three committer is now generic over the typed port. The proposal half of the runtime is injected through the object-safe port. Both are registered in one place, the hub composition root (`hub/🏗️bootstrap/🦀️.rs`).

The full-resolution differential shows no new in-crate resolution errors for any of the four touched crates (section 6). The only new errors are the expected unresolved external crate names in standalone rustc runs. No cargo build, cargo test or Nx gate could run: the sanctioned launcher stops in its own cache maintenance step before cargo starts, and the hub script gates stop on a stale schema catalog that a peer left. Section 9 lists GIS-concrete vocabulary that is still in the hub, with counts and a concrete plan.

## 2. Design and owner evidence

| Question | Evidence | Decision |
| --- | --- | --- |
| Which crate is the hub? | `hub/📦️packages/🦀️rust/Cargo.toml`: `semio-hub` (lib `hub/📦️packages/🦀️rust/🦀️.rs`, bin `os-hub` = `hub/🏗️bootstrap/🦀️.rs`), standalone `hub/Cargo.toml` workspace, role `hub`. | Consumer of the binding. |
| Where does GIS live, and does it already have a hub-side crate? | `✏️s/🔌️plugins/🌍️gis/**` holds the artifact crates (`semio-s-artifact-gis-gismap`, `…-gisterrain`, `…-gismap-mcp`), which are plugin layer. `hub/🧩️compositions/🌍️gis/` is `semio-hub-gis` (Nx `@semio-tech/gis-plugin`, role `hub`, inputs `✏️s/🔌️plugins/🌍️gis/**`). It is the GIS plugin's hub composition: plugin builder, native codec receipts, the wasm component's lib. `semio-hub` already depends on it (`🗿️artifact-authority/📇️native-openable-provider`). | **Owner: `semio-hub-gis`**, new module `💡️inference`. The task's crate in the plugin tree cannot work: a plugin-layer crate cannot depend on a hub-layer port, and a port owned by the consumer (the hub) is the only neutral home. `semio-hub-gis` is the existing GIS-owned crate that sits at the right layer. |
| Where does the port live? | The binding needs only framework and OS types (Store, durable group, replication timestamp, inference budget and service). Both `semio-hub-gis` (implementer) and `semio-hub` (consumer) must see it, and `semio-hub-gis` must not depend on `semio-hub`. | **New crate `semio-hub-inference-binding`** at `hub/💡️inference/🔗️binding/` (`🔗️binding` is a registered taxonomy kind). It depends only on `pack`, `semio-framework-os-kernel`, `semio-framework-plugin` and `semio-framework-value`. |
| How does bootstrap wire GIS? | `serve()` in `hub/🏗️bootstrap/🦀️.rs` builds `verified_gis_map_binding(catalog)`, `RetainedGisMapApprovalCommitterV1::new(..)` and `HubInferenceRuntimeV1::new(..)`. The routes in `🏗️bootstrap/💡️inference` and `🛣️routes` reach the runtime only through `HubInferenceRuntimeV1`. | **Registration point = the composition root.** One `GisMapInferenceBindingV1` value is created there. It supplies the catalog executable, the runtime's injected proposal port and the committer's type parameter. |
| Existing ports `GisMapApprovalCommitterV1` / `GisMapApprovalIngressAuthorityV1`? | `RT/🧾️approval-contract`: `HubInferenceRuntimeV1` holds `Arc<dyn GisMapApprovalCommitterV1>`. | Kept. The generic committer is erased behind this existing port, so `HubInferenceRuntimeV1`, the routes and `HubState` stay non-generic. |

Why two ports:

- **`MapInferenceProposalBindingV1`** is object-safe and works on bytes. Its methods are `inference_service`, `commit_members`, `composed_children`, `infer`, `approval_effect`, `undo_effect` and `preview`. It is injected as `Arc<dyn …>` into `HubInferenceRuntimeV1::new` and into `verified_gis_map_binding`.
- **`ComposedMapCommitBindingV1`** is typed: six associated types (parent, drawing and value snapshot and mutation), `COMMIT_MEMBERS`, `decode_parent`, `derived_members`, `member_stores`, `executed_approval_work`, `approval_work`, `undo_work` and the three stamped preparation factories. `RetainedGisMapApprovalCommitterV1<B>` and the WAL decision matcher are generic over it.

The associated-type bounds are the union of the where-clauses on `ArtifactStore`, `DurableOwnedThreeStoreMapAssemblyV1`/`…CommitHostV1`/`…RecoveryAdmissionV1`, the db recovery owner and `verify_fixed_three_edits`. They are the same set that the hub's existing generic `close_store<P, M>` already declared, plus `PartialEq` on snapshots (the Store comparisons) and `Sync` on mutations (auto-trait proofs for retained futures).

Records that stay in the hub, with the reason (design principle 2): `GisMapDocumentStoresV1`, `GisMapCommitIdentityV1`, `GisMapCommitOperationV1`, `GisMapClosingStoresV1` and the store, assembly, host and recovery aliases. They hold hub-owned types (`InferenceDocumentFenceV1`, the write lease, the ingress authority, `InferencePrivateBytesV1`, `db::ArtifactHandle`). Once the aliases are projections over `B`, they name no GIS type, so they are committer state, not GIS vocabulary. Moving them to `semio-hub-gis` would require a dependency cycle. They now live in the neutral child `RT/🔗️binding/`, which replaces `RT/🌍️gis-map-binding/`.

## 3. Moved items (from → to)

| Item | From | To |
| --- | --- | --- |
| 6 snapshot/mutation aliases (`GisMapParentSnapshotV1` … `GisMapValueMutationV1`) | `RT/🌍️gis-map-binding` | associated types of `ComposedMapCommitBindingV1`, implemented as `GisMapSnapshot`, `GisMapMutation`, `SemioDrawingSnapshot/Mutation`, `SemioValueSnapshot/Mutation` in `semio_hub_gis::inference` |
| 7 Store/assembly/host/recovery/journal aliases | `RT/🌍️gis-map-binding` | `RT/🔗️binding`, as generic projections over `B` (no GIS name) |
| `GisMapDocumentStoresV1`, `GisMapClosingStoresV1` | `RT/🌍️gis-map-binding` | `RT/🔗️binding`, generic over `B` |
| `GisMapCommitIdentityV1`, `GisMapCommitOperationV1` | `RT/🌍️gis-map-binding` | `RT/🔗️binding` (unchanged, non-generic) |
| `deterministic_map_inference` | `RT/🌍️gis-map-binding` | `semio_hub_gis::inference::deterministic_map_inference(pack, job_id, budget)` (pub; used by the port impl and by tests) |
| `gis_map_inference_preview` | `RT/🌍️gis-map-binding` | split in two. The GIS decoder becomes `semio_hub_gis::inference::gis_map_region_preview` (= `MapInferenceProposalBindingV1::preview`). The hub keeps the exact proposal-hash check and the `GisMapInferencePreviewV1` wire wrapper in `RT/🛣️routes` (`gis_map_inference_preview(proposals, …)`, inside the existing `🛣️Routes` region) |
| `parent_store`, `drawing_store`, `value_store`, `derived_children`, the mount-time child ids and GIS dialect | `RT/🔐️committer` | `semio_hub_gis::inference` (`member_stores`, `derived_members`, private store fns) |
| approval work (native run + `create_region_group_work`), undo derivation (inverse decode, apply, rebuild) | `RT/🔐️committer` (`preflight`, `preflight_undo`) | `executed_approval_work`, `undo_work` |
| assembly work and the three `gis_map_*_stamped_one_item_preparation_factory` calls | `RT/🧱️document-preparation` | `approval_work`, `undo_work`, `*_preparation_factory` |
| `composed_children` (Map child ids) | `RT/🧭️job-types-and-port` | `MapInferenceProposalBindingV1::composed_children`; `InferenceMapBaseV1::composed_children(&dyn …)` delegates |
| native inference + `bounds_proposal`, stamped approval effect, undo effect | `RT/🏭️inference-runtime` | `infer`, `approval_effect`, `undo_effect` |
| literal fixed-three members `["gismap-drawing", "gismap-value"]` (2 sites) | `RT/🏭️inference-runtime`, `RT/🔐️committer` | `commit_members()` / `COMMIT_MEMBERS` |
| fixed-three WAL edit types (`verify_fixed_three_edits::<6 GIS types>`) | `💡️inference/🧾️wal` | `durable_decision_event_match::<B>`, stored as a `fn` pointer by `InferenceWalVerifierV1::new::<B>` |
| `gis_map_inference_service()` (2 sites) | `💡️inference/📇️catalog` | `MapInferenceProposalBindingV1::inference_service`; `verified_gis_map_binding(catalog, proposals)` |
| GIS undo wire types re-export | `💡️inference/🧬️schema` (from `semio_s_artifact_gis_gismap`) | re-exported explicitly by `semio_hub_gis::inference`; the hub schema re-exports from there |

Line accounting (`relo-hubgis-tools/moved.py`, log `🗑️generated/relo-hubgis-moved.log`): 297 non-trivial lines left the touched hub files. 76 reappear verbatim (after crate-alias normalisation) in the GIS module, 29 elsewhere in the hub, and 192 were adapted (generic signatures, projection aliases, port calls, refusal mapping). Size: `RT` production tree 4,041 → 3,928 lines. The port crate has 146 + 67 test lines. The GIS module has 279 + 49 test lines. `RT/🔗️binding` has 74 lines.

## 4. Registration point and its tests

- Composition root, `hub/🏗️bootstrap/🦀️.rs`:
  - `let gis_map_inference_binding: Arc<dyn MapInferenceProposalBindingV1> = Arc::new(GisMapInferenceBindingV1);`
  - That value goes to `verified_gis_map_binding(catalog.clone(), gis_map_inference_binding.as_ref())`.
  - `let committer: Arc<RetainedGisMapApprovalCommitterV1<GisMapInferenceBindingV1>> = Arc::new(RetainedGisMapApprovalCommitterV1::new(..))`.
  - `HubInferenceRuntimeV1::new(binding, ledger, committer, gis_map_inference_binding.clone())`.
- Refusal taxonomy: `MapBindingRefusalV1 {Undecodable, Execution, Arity, Inapplicable, Composition, Bounds}`. Routes map it through `impl From<MapBindingRefusalV1> for InferenceRouteErrorV1` (Invalid, Invalid, Conflict, Conflict, Conflict, Bounds). `infer` keeps the old "interrupted → recorded failure or Invalid" rule. Each committer call site keeps its original Rejected/Conflict split (for example `undo_work` in assembly: Undecodable or Arity → Rejected, else Conflict; in `preflight_undo`: Undecodable → Rejected, else Conflict). Every original error and its ordering is preserved.
- Non-default implementation honoured. New test `the_inference_runtime_honours_a_non_default_proposal_binding` in `RT/🧪️tests/🔬️unit`: a `FixedMembersBindingV1` stub answers `composed_children` and the preview through the runtime's own helpers, the hub still enforces the proposal hash, and the refusal mapping holds.
- Port object safety and dispatch: `the_proposal_port_dispatches_every_byte_level_call_through_a_registered_object` in `hub/💡️inference/🔗️binding/🧪️tests/`.
- Moved behaviour at its new home, in `hub/🧩️compositions/🌍️gis/💡️inference/🧪️tests/`:
  - canonical preview against the language-neutral fixture `hub/🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json`;
  - every substitution refused by kind;
  - default Map members equal to `commit_members()` and `COMMIT_MEMBERS`;
  - `derived_members` equal to the gismap derivations.
  
  The existing hub preview law still runs end to end through the wrapper with `&GisMapInferenceBindingV1`. The hub script gate validates the same fixture independently with Ajv.
- None of these tests ran (section 7).

## 5. Callers, fixtures and readers updated

- Hub: `RT/🦀️.rs` (mount `🔗️binding`, port imports, `inference_budget()` replacing three literal budgets), `RT/🔐️committer`, `🧱️document-preparation`, `🔁️commit-turns`, `🔎️checkpoint-verify`, `📕️commit-and-close`, `🧭️job-types-and-port`, `🏭️inference-runtime`, `🛣️routes`, `🧾️approval-contract`, `💡️inference/🦀️.rs` (explicit re-export of the two ports and the refusal), `🧾️wal`, `📇️catalog`, `🧬️schema`. `RT/🌍️gis-map-binding/` was deleted.
- Tests:
  - `RT/🧪️tests/🔬️unit`: concrete alias `GisRetainedCommitterV1` (11 sites), binding argument (8 sites), new law.
  - `🧾️wal/🧪️tests/{🔬️unit,⛓️chain}`: `InferenceWalVerifierV1::new::<GisMapInferenceBindingV1>` (7 sites).
  - `📇️catalog/🧪️tests/🔬️unit`: `expected` argument (3 sites).
  - `hub/🧪️tests/🔬️bin-unit` (2 runtime constructions).
  - `hub/🧪️tests/🔏️trusted-catalog-profile` and `🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit` (binding argument).
- Source-text readers in `hub/📦️packages/🦀️rust/📜️script.ts`:
  - `proveGisMapProposalApprovalFixture` now also reads `hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs` through `readModuleTreeSource`. Store construction, the drawing and value child ids, the stamped factory and the two `impl … for GisMapInferenceBindingV1` blocks are asserted there.
  - The runtime must name no `semio_s_artifact_` crate, must contain `impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B>`, `B::member_stores(&key, snapshot, actor)` and `B::parent_preparation_factory(`.
  - `proveGisMapFrozenBindingFixture` now requires `proposals.inference_service()` in the catalog and the literal `semio_s_artifact_gis_gismap::gis_map_inference_service()` in the GIS binding.
  - The other three GIS gates (undo, retained runtime, checkpoint control) needed no change: their anchors are still in the runtime tree.
- Fixture: the `rustReceivers` list of `🧰️framework/…/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` replaces the deleted child with `RT/🔗️binding/🦀️.rs`. This one-line edit in a peer area was forced by the deletion.
- Manifests: `semio-hub` drops `semio-s-artifact-gis-gismap`, `semio-s-artifact-stdio-semio-drawing` and `semio-s-artifact-stdio-semio-value` from `[dependencies]` and adds `semio-hub-inference-binding`. Dev-deps gain `semio-s-artifact-stdio-semio-drawing` and `semio-s-artifact-stdio-semio-value-mutations`; the WAL test fixture builder uses them, and the hub previously used `…-value-model/-mutations` without declaring them. `semio-hub-gis` adds the port, `semio-framework-artifact-reference`, `semio-framework-pack-json` and the three stdio semio crates. `hub/Cargo.toml` adds the member (byte-sorted first) and the workspace dependency.
- New crate registrations per `PRODUCER-CRATE-RECIPE.md`:
  - `Cargo.toml` with role/id and the three boundary keys.
  - `📋️project.json` `@semio-tech/hub-inference-binding-rs` (`scope:hub`, `language:rust`).
  - `📋️build-owners.json` row after `@semio-tech/hub-compositions`.
  - Six routes `🦀️ semio-hub-inference-binding 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, orders 900.01611/12, 922.04611/12, 903.58611/12 (checked unused), inserted after the `repo-cli-verbs` block in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`.
  - `semio-hub-gis`'s `📋️project.json` default inputs gain `🌎️hub/🧩️compositions/🌍️gis/**/*.rs`. Before, its own lib sources (and so the new module) did not invalidate its cache.

## 6. Verification actually run

| # | Check | Result |
| --- | --- | --- |
| 1 | Full-resolution differential (`relo-hubgis-tools/ho.py` + `neutral.py` + `mksbx.py`, extended to recreate files the current tree deleted). `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`. Baseline = 23 touched files plus the deleted child from `🗑️generated/relo-hubgis-baseline/`. | **`semio_hub` lib**: native 1,539 → 1,432, `--test` 3,060 → 2,967, `integration-fixtures` 1,611 → 1,505, test+feature 3,123 → 3,031. Non-external 34/34, 83/83, 36/36, 85/85. Every split-only key is an unresolved external crate: `semio_hub_inference_binding` 10, `semio_hub_gis` 14, `db`/`serde_json`/`semio_framework_plugin` at new use sites 8. The first run caught a real regression, 3× E0425 `_state` in the WAL `scan` test hooks after I renamed its parameter. It was fixed and re-run. **`os_hub` bin**: bin, `--test`, feature, test+feature: non-external 4/4, 6/6, 4/4, 7/7; the only difference is the new external `semio_hub_gis` import. **`semio_hub_gis`**: native, `--test`, `wasm32-wasip2`, test+wasip2: non-external 7/7 each; split-only = external crates of the new module only. **`semio_hub_inference_binding`** (no baseline): native, `--test`, wasip2: 8 errors, all external, 0 non-external. Logs: `🗑️generated/relo-hubgis-oracle-{hub,bin,gis,port}.log`. |
| 2 | Field/method privacy (`famscan.py` + new differential `famdiff.py`, baseline vs current) | hub: 243 → 242 candidates, **0 new**. gis: 0. port: 0. Seeded regression (dropping `pub(super)` from `GisMapDocumentStoresV1::fence` and `GisMapCommitIdentityV1::journal_now_ms` in a sandbox copy) gives **9 new rows**, so the scan is sensitive (`relo-hubgis-famdiff-{hub,seed}.log`). |
| 3 | `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` | 28/28 touched or new Rust files parse. The coordinator's fix of the `<_ as ArtifactPack + ArtifactStoreSnapshot>::` paths is **confirmed**: the 13 baseline runtime files parse and no such path remains under `💡️inference`. Region markers: 15/15 open/close now against 16/16 before. The one pair removed is the `🛣️Routes` pair of the deleted child; its code now sits inside `🛣️routes`' existing `🛣️Routes` region. Path length: longest new path 90 bytes. Emoji: `🔗️`, `💡️` are RGI, carry U+FE0F, are unique among siblings and are already used as folder prefixes. `#[path]`/`include*!` in touched files: 143 checked, 0 missing. |
| 4a | dependency-cruiser on `📜️script.ts` | 1,208 violations after, all pre-existing. Import statements before and after are identical (`diff` of import lines is empty), so no edge was added or removed. Log `relo-hubgis-depcruise-after.log`. |
| 4b | Static Cargo direction (`relo-hubgis-tools/cargodir.ts`, `Bun.TOML`) | port (4 path deps), `semio-hub-gis` (26), `semio-hub` (35): **0 upward, 0 missing**. Port → framework/OS; `semio-hub-gis` → port + framework + plugins; `semio-hub` → port + `semio-hub-gis` + …; acyclic. |
| 4c | Static Rust source direction | Port and GIS module: no `#[path]`/`include!` outside their own trees except the GIS test's `include_str!` of the hub fixture (same hub layer). No use of `semio_hub` from either. The runtime production tree has 0 `semio_s_artifact_` mentions. |
| 5 | Scoped `tsc --noEmit -p 🗑️generated/relo-hubgis-tsconfig.json` (script + ambient) | 39 errors. 6 are in `📜️script.ts` at lines 4832, 4867, 4872, 13007, 13035, 16236, and none is in my changed ranges (7094–7098, 7120, 7133, 7145–7147, 7338–7339, 7346). The rest are in peer modules. Gate logic: the five GIS gates were exported from copies of the script (`relo-hubgis-tools/gates-make.py`, `gates-run.sh`). All five stop before any source check on `hub schema module 🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema is absent from …/🔣️schema-catalog.json`, a peer change. Their source conditions were therefore replayed verbatim on the current tree with `relo-hubgis-tools/source-gates.ts`: proposal-approval (new) PASS, frozen-binding (new) PASS, undo PASS, retained-runtime PASS, checkpoint-control PASS. The pre-relocation proposal-approval conditions FAIL on the current tree (`runtime missing gis_map_parent_stamped_one_item_preparation_factory`), which shows the readers had to be re-pointed. `bun test` of the descriptor-digest suite (`rustReceivers` parse): **8 pass, 0 fail**. Both launch files parse (9,389 / 6,421 configurations; 6 new routes each, unique commands); `📋️build-owners.json` parses with the row present once. |
| 6 | `bun repo cargo --store=shared --configuration=prototype -- check -p semio-hub-inference-binding` / `-p semio-hub-gis` / `-p semio-hub` / `--manifest-path 🌎️hub/Cargo.toml -p semio-hub-inference-binding` | All four exit 1 before cargo starts: `error: Finite original maintenance authority required` (`📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts:43`). The launch-route form `bun repo run @semio-tech/hub-inference-binding-rs:check …` exits 1 with `Active graph readers retain required Nx pool headroom`. Lock files were not refreshed: offline update needs the same launcher. No compile around the gate was attempted. |

## 7. Not verified

- No cargo check, build or test of `semio-hub`, `os-hub`, `semio-hub-gis` or `semio-hub-inference-binding`. None of the new or moved tests ran. The oracle cannot type-check (every external crate is unresolved). Trait-bound sufficiency of the generic committer (auto traits for retained futures, impl where-clauses), method resolution on `B::*` projections and `private_interfaces` all wait for the first real compile.
- The hub script gates and Nx targets did not run end to end (section 6, row 5).
- `🌎️hub/Cargo.lock` does not yet list `semio-hub-inference-binding` or the changed dependency edges.

## 8. Pre-existing API drift found in the moved code (not introduced here)

The GIS module now owns these lines, so the GIS-store migration owner meets them next to the GIS code:

1. `semio_s_artifact_gis_gismap::…::io::binary::mutations::gis_map_document_store_owners()` was deleted in `03c902fa488` (grant-based store owners). `parent_store` keeps the stale call.
2. `infer_gis_map_controlled` now takes `cx: &mut ArtifactInferenceStepContext` and returns `ArtifactInferenceExecutionStep`. Both GIS binding calls keep the hub's two-argument form. The peer migration of the job and inference API is in progress, so I did not port to it.
3. `ArtifactHandle::durable_group_recovery_retained` now takes `document: ArtifactId` first. The hub committer's `mount_document` still passes one argument (hub-side).
4. `ActorId(pub SharedUtf8)`: `protocol::ActorId(actor.to_owned())` still passes a `String` (moved verbatim).
5. `inverse_gis_map_mutation` returns `Result`, and the hub serialised the `Result` itself. **Fixed** in `approval_effect` (`map_err` → `Composition` → route `Conflict`, the same answer an inverse refusal gives in `create_region_group_work`).
6. Hub `close_store<P, M>` bounds name `::semio_framework_os_kernel::…`. `semio-hub` cannot name that crate (its dependency is renamed `directory`). Left untouched.
7. The hub declared `semio-s-artifact-stdio-semio-value` but used `…-value-model`/`…-value-mutations`. **Resolved** by the move: `semio-hub-gis` declares the right crates, and the hub tests declare the right dev-deps.

## 9. GIS-concrete code still in the hub, with counts and plan

Production references to a GIS crate in the hub lib are down to 2:

- the explicit re-export of the three undo wire types from `semio_hub_gis::inference` in `💡️inference/🧬️schema`;
- the frozen-binding wire string `"semio_hub_gis::gis_map_inference_service"` in `📇️catalog`, which is part of the `🧊️gis-map-frozen-binding-v1` digest.

The bin's composition root names `semio_hub_gis::inference::GisMapInferenceBindingV1` by design.

Residual GIS-named vocabulary (identifier occurrences, `relo-hubgis-residual-counts.log`):

| Area | `GisMap` | `gis_map` | `GIS_` | What it is |
| --- | --- | --- | --- | --- |
| `RT` production (committer bands, contract, routes, runtime) | 651 | 87 | 23 | hub-owned names (`RetainedGisMapApprovalCommitterV1`, `GisMapApprovalCommitterV1`, `GisMapCommitConflictArmV1`, `gis_map_storage_refusal`, …), now generic or neutral in content |
| `💡️inference/🧬️schema` | 30 | 0 | 16 | `GIS_SERVICE_ID/ROUTE/PACKAGE_ID/ARTIFACT_KIND/DOCUMENT_SCHEMA/EDITOR_SURFACE_ID/GRANTED_MODE`, `GisMapFrozen*V1`, `GisMapInferencePreviewV1`, `GisInferenceCheckpointControl*` (scope schema exports, JSON schema and TS twin) |
| `💡️inference/📇️catalog` | 21 | 8 | 11 | frozen GIS selection checks (`"gis"`, `"s.gis.gismap"`, surface, grant, service literals) |
| `💡️inference/🪶️sqlite` | 21 | 6 | 0 | ledger tables and methods `gis_map_approval_undo_*` |
| `💡️inference/✉️command`, `🧾️wal` | 0 | 0 | 2 + 2 | `GIS_DOCUMENT_SCHEMA` in `matches_fixed_three_parent` and in the WAL command rebuild |
| `🏗️bootstrap` | — | — | — | `💡️inference` routes `/inference/gis-map/jobs`, `📌️check-in` `GisMapApprovalCheckpointPublisherV1Impl`, `🧠️state` `gis_map_binding` |

Next steps, in order (one ticket, compile-checked once the launcher admits the crates):

1. First real compile, `check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-inference-binding -p semio-hub-gis -p semio-hub --lib --bins --tests` (+ `--features integration-fixtures`, + `--target wasm32-wasip2 -p semio-hub-gis`). Fix only missing bounds on `ComposedMapCommitBindingV1` associated types, where needed, and the drift in section 8. Then refresh `🌎️hub/Cargo.lock` offline.
2. Add `fn descriptor(&self) -> MapInferenceServiceDescriptorV1` to `MapInferenceProposalBindingV1` (service id, route family, package id, artifact kind, document schema, editor surface, granted mode, native executable). Parameterise `📇️catalog` validation, `✉️command::matches_fixed_three_parent`, the WAL command rebuild and the bootstrap route family by it. Delete the `GIS_*` constants from `🧬️schema` and regenerate the scope schema JSON, the TS twin and the `🧊️gis-map-frozen-binding-v1` fixture digest in the same pass.
3. Move `GisMapInferencePreviewV1` and the `GisMapApprovalUndo*` wire contracts behind the descriptor (neutral `MapInferencePreviewV1`, the undo contracts owned by the binding crate), with their schema exports.
4. Rename the runtime and ledger vocabulary in one pass: `GisMap*` → `ComposedMap*`, `gis_map_*` → `composed_map_*`, the sqlite tables. That is about 650 runtime identifiers plus `🏗️bootstrap/{💡️inference,📌️check-in,🧠️state}`, the hub tests and the `📜️script.ts` gate literals (the gate anchors listed in section 5).
5. Unify the two registrations into one value. A blanket `impl<B: ComposedMapCommitBindingV1 + MapInferenceProposalBindingV1>` helper builds both the committer and the runtime from one `B`, so the composition root cannot pair mismatched halves.

## 10. Files

Created:
- `hub/💡️inference/🔗️binding/{🦀️.rs, 🧪️tests/🦀️.rs, 📦️packages/🦀️rust/Cargo.toml, 📦️packages/🦀️rust/📋️project.json}`
- `hub/🧩️compositions/🌍️gis/💡️inference/{🦀️.rs, 🧪️tests/🦀️.rs}`
- `RT/🔗️binding/🦀️.rs`

Deleted:
- `RT/🌍️gis-map-binding/🦀️.rs`

Edited:
- `RT/{🦀️.rs, 🔐️committer, 🧱️document-preparation, 🔁️commit-turns, 🔎️checkpoint-verify, 📕️commit-and-close, 🧭️job-types-and-port, 🏭️inference-runtime, 🛣️routes, 🧾️approval-contract, 🧪️tests/🔬️unit}/🦀️.rs`
- `hub/💡️inference/{🦀️.rs, 🧾️wal/🦀️.rs, 🧾️wal/🧪️tests/🔬️unit/🦀️.rs, 🧾️wal/🧪️tests/⛓️chain/🦀️.rs, 📇️catalog/🦀️.rs, 📇️catalog/🧪️tests/🔬️unit/🦀️.rs, 🧬️schema/🦀️.rs}`
- `hub/🏗️bootstrap/🦀️.rs`
- `hub/🧪️tests/🔬️bin-unit/🦀️.rs`
- `hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs`
- `hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`
- `hub/🧩️compositions/🌍️gis/{🦀️.rs, 📦️packages/🦀️rust/Cargo.toml, 📦️packages/🦀️rust/📋️project.json}`
- `hub/📦️packages/🦀️rust/{Cargo.toml, 📜️script.ts}`
- `hub/Cargo.toml`
- `📋️build-owners.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- the descriptor-digest fixture `🔣️.json`

Tools: `🪓️monolith-split/relo-hubgis-tools/` (edit scripts `e1`–`e8`, `edit.py`, oracle `ho.py`/`neutral.py`/`scan.py`/`rsitems_lex.py`/`mksbx.py`/`spec.json`, `famscan.py`/`famdiff.py`, `moved.py`, `cargodir.ts`, `source-gates.ts`, `gates-make.py`/`gates-run.sh`, `touched-rs.txt`). Pristine copies: `🗑️generated/relo-hubgis-baseline/`. Logs: `🗑️generated/relo-hubgis-*`.
