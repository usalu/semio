# RESULT-W4-FLOW-FLOW: carve of `semio-framework-artifact-flow-flow`

Executor: Sonnet 5.5. Tools and scripts: `flowflow-tools/`. Logs: `🗑️generated/flowflow-*.log`. Slug: FLOW-FLOW.
Artifact dir (all paths below are relative to it): `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow`.

## 1. Outcome

The crate (39 files, 321,804 bytes, 4,971 lines; test closure 94 files, 516,333 bytes, 6,724 lines) is carved into three producers, two test-only laws crates and a thin facade. Every crate is within 3500 lines, 28 files and 230000 bytes for both its lib and its test closure. The facade keeps the old crate name, so no consumer Cargo.toml changed.

Not achieved: a real `cargo check` / `cargo test`. The launcher refuses the peer-owned `semio-framework-os-kernel` source boundary before it reaches any flow crate (section 6). All equivalence evidence is the full-resolution rustc differential, which does not reach the privacy or type-check phase.

## 2. The cycle and the design chosen

### 2.1 Why the cycle cannot be cut by "inverting the registration"

The three options named in `RESULT-W4-OS-FLOW.md` section 7 all fail, for reasons that were not visible from the registration site alone:

- `#[derive(Mutations)]` on `FlowMutation` emits `impl Mutation<FlowHostSnapshot> for FlowMutation` and `SemanticMutation<FlowHostSnapshot>` at the enum, so the enum's crate must see `FlowHostSnapshot`.
- `impl ArtifactStoreSnapshot for FlowHostSnapshot` (orphan rule: it must sit with the type) returns `bare_artifact_codec::<Self, FlowMutation>`, so the snapshot's crate must see `FlowMutation`.
- The store's hydration path calls the generic `io::publish_native_snapshot::<P>()`, which calls `P::native_snapshot_registration()`. The producer-declared `ArtifactStorePublication` route covers only `ArtifactStore::new`, so moving registration to the producer (option 1) would change os-kernel and four sibling artifacts (dag, run, space, collection) that use the same pattern. Not done: peer-owned.

So `FlowHostSnapshot` and `FlowMutation` necessarily live in one crate: the **document** crate. The god-object rule then applies to everything else that was glued to them.

### 2.2 Decomposition

The snapshot is already a composition of component values (`Widget`, `SynapseSpec`, `WidgetLayout`, `CameraJson`, GUI maps). Everything that does not need the snapshot or the mutation catalogue moves below it:

- **model** (`🧱️model`): the component value types and their helpers (`Widget`, `SynapseSpec`, `FlowUi`/`FlowGui`, `NodeChrome`, `FlowArtifact`, `FlowLayoutEntry`, descriptors, IO-port and DAG helpers, slider semantics, cluster tree), the typed retirement owner (`FlowOwner`, `FlowRetirement` with the field `RetireOwned` impls), `graph_parameter`, the collection identity impls (`Identified`/`Patchable` for `Widget`/`SynapseSpec`, orphan-pinned to the types) and the **component text codecs** (`WidgetDsl`, `SynapseDsl`, `TreeDsl`, `ValueDsl`, `NeuronNodeDsl` with `DslField`/`BorrowedDslField` for `Widget`/`SynapseSpec`), mounted as `component_dsl`.
- **document** (`🧾️document`): `FlowHostSnapshot` (now in `🧬️schema/📸️snapshot/🧾️host`, with `to_artifact`, `tree_from_host_snapshot`, keyboard traversal and its retirement), the nine mutation leaves, `FlowMutation`, `FlowDiff`/`FlowDelta`, canonical wire, store glue (aliases, retirement and authoring factories, `FlowSnapshotPackProducer`, history store constructors), the host-snapshot text mirror (`FlowHostSnapshotDsl`), the binary pack codec and the SQLite relational/native capability. All impls on the two pinned types stay here.
- **copy** (`🧵️retained/📑️copy`): the retained scene copy cursor. It needs the snapshot and `FlowOwner`, nothing needs it, so it sits above both. The existing file is now the crate root.
- **mutation-laws** (`🌿️vcs/⚖️laws`) and **snapshot-laws** (`🚪️io/⚖️laws`): test-only crates over the public document API.
- **facade** (`🦀️.rs`, 47 lines): explicit `pub use` lists reproducing every previously public path (`artifact`, `vcs`, `retained` incl. `retained::copy`, `graph_parameter`, `io::binary::snapshot::FlowSnapshotPackProducer`, root names).

### 2.3 API changes the cut forces (greenfield, no shims)

- `FlowOwner::HostSnapshot` now carries `FlowHostComponents`, an owned record with exactly the five fields of the snapshot and the same `artifact_retire_struct!` cursor, so retirement accounting is identical. `impl From<FlowHostSnapshot> for FlowHostComponents` (and back) lives in document; `FlowHostSnapshot::retire_cold` is unchanged for callers. The model therefore never names the snapshot or the mutation catalogue.
- `FlowOwner::Mutation` is removed. It was constructed only by one test; mutation retirement is driven through `ControlledRetirement` directly in that test.
- `FlowLayoutEntry` moved from the mutation module to the model (the retirement owner has a `Layout` variant); the facade still exports it at the root and under `vcs`.
- Consumer edit: `host-evaluation-history` built `FlowOwner::HostSnapshot(baseline)` twice just to retire a surplus baseline; both now call `baseline.retire_cold()`, which performs the same sequence.

### 2.4 Within-budget folds (no logic change)

To bring document to 27 files: the four aggregator mod files under `🚪️io` became inline `#[path = "dir"]` modules in `🚪️io/🦀️.rs` (same scopes, same module paths); `OpText`/`OpBinary` for `FlowMutation` and the `owned_record!` retirement impls moved into the existing empty `🔤️Codecs` region and a new `🧹️OwnedRetirement` region of the mutation aggregate file; `FlowProjection` became an inline module of the diff file. Nine one-test leaf files were merged into one laws file. The SQLite relational code stayed whole in document: the pair fits (27 files, 188,641 bytes, 2,526 lines), so the god-object rule did not require decomposing it into component codecs.

## 3. Census (canonical counter `readRustSourceClosureV1`)

Log: `🗑️generated/flowflow-census-final.log`. Before (single crate): lib 39 / 321,804 / 4,971; test 94 / 516,333 / 6,724.

| Crate | Lib (files / bytes / lines) | Test closure |
|---|---|---|
| model | 7 / 86,491 / 1,824 | 16 / 107,373 / 2,039 |
| document | 27 / 188,641 / 2,526 | same (tests live in the laws crates) |
| copy | 1 / 47,608 / 610 | 9 / 128,250 / 1,069 |
| mutation-laws (test only) | 1 / 852 / 17 | 26 / 57,998 / 649 |
| snapshot-laws (test only) | 1 / 493 / 8 | 6 / 37,623 / 462 |
| facade | 1 / 3,809 / 47 | same |

No cfg-dependent module exists in the closure; the wasm32 cfg census of document gives the same numbers. Longest new path: 225 bytes (pre-existing fixture), limit 240.

## 4. Crate graph (arrows point to dependencies)

- model: leaf (first-party only: directed-presentation, dsl-record(+derive), pack-json, value(+derive), os-kernel, replication, neural engine, graph-manifest).
- document -> model.
- copy -> model, document (tests keep the allocation-observing retirement laws: the old `retained` and `fields` tests moved here, mounted by `🧵️retained/🧪️tests/🦀️.rs`, because the copy tests share the observed global allocator).
- mutation-laws, snapshot-laws -> document (and through it the model).
- facade -> model, document, copy.

Dependency lists: the four literal-unused crates of the old manifest (`canvas`, `semio-framework`, `dsl`, and `schema-state` where no derive needs it) were dropped from the crates that do not use them; `schema-state` stays in document because the os-level derives reference it. `depcheck.py` (declared versus literal use) is clean for all five crates apart from document's `async` (used as `::semio_framework_async::`, which the checker's regex skips) and `schema-state`.

## 5. Files changed

- New crate nodes: `🧱️model`, `🧾️document` (root `🦀️.rs` + `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`), copy package under `🧵️retained/📑️copy/📦️packages`, laws nodes `🌿️vcs/⚖️laws` and `🚪️io/⚖️laws`.
- New module files: `🧬️schema/📸️snapshot/🧾️host/🦀️.rs` (snapshot owner), `🚪️io/📝️text/📸️snapshot/🧾️host/🦀️.rs` (host text codec), `🌿️vcs/🧪️tests/🔬️leaf-contracts/🦀️.rs` (nine leaf contract tests), `🧵️retained/🧪️tests/🦀️.rs` (retirement-laws wrapper). 18 new files in total.
- Deleted (folded, see 2.4): 17 files.
- Modified: 25 files (facade root, manifest and `📜️script.ts`; `artifact`, `retained`, `fields`, `vcs`, `mutations`, `diff`, the nine leaves, the text component codec, the copy file and three test files).
- Registration: root `Cargo.toml` (+5 byte-sorted members), `Cargo.lock` (5 new packages, via `bun repo cargo ... update --workspace --offline`, exit 0, only the five flow packages and the facade edge changed), `📋️build-owners.json` (+5 rows after the facade), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (+30 routes each: build/check/test x prototype/release per crate, unused order keys).
- Outside the artifact dir: `🖥️host/🌳️host-evaluation-history/🦀️.rs` (two retirement sites), `🦑️repo/.../📚️library/📦️packages/🟦️typescript/📜️script.ts` (the slider-label law now runs in the model package).
- Facade `📜️script.ts`: runs all six packages; `test-snapshot-sqlite` runs the snapshot-laws crate; the physical-retirement native law runs in the copy crate.
- Region markers: none lost (127 before, 135 after: `🧹️OwnedRetirement`, `🔖️CollectionIdentity`, `🔖️HostSnapshot`, `🧊️HostSnapshotRetirement` added; the nine `🧪️Tests` and the `🧪️Ownership` and `🧪️RetirementLaws` pairs moved with their tests).

## 6. Verification (real output)

- **Patched full-resolution differential** (`rustc +nightly-2026-07-20 --emit=metadata`, union sandbox where every crate and the facade is a module, external-crate globs and `extern crate` neutralized, first-party crate names mapped to modules; scripts `ffo.py`, `ffu.py`; baseline from the pre-split tree). Log `🗑️generated/flowflow-union-final.log`, run on the real tree:

  | Config | baseline errors | split errors | only-split | only-baseline |
  |---|---|---|---|---|
  | native | 124 | 124 | 0 | 0 |
  | test (+ `feature="protocol-laws"`) | 138 | 138 | 0 | 0 |
  | wasm32-unknown-unknown | 124 | 124 | 0 | 0 |
  | wasm32-unknown-unknown + test | 138 | 138 | 0 | 0 |
  | wasm32-wasip2 | 124 | 124 | 0 | 0 |
  | wasm32-wasip2 + test | 138 | 138 | 0 | 0 |

  Sensitivity: a seeded unknown facade export and a seeded private `widget_chrome` were both reported (2 new errors); a seeded private method was not (the oracle does not reach method privacy, hence the manual scan below). This run also resolves every `#[path]` mount, including the inline `#[path = "dir"]` modules.
- **Lossless line multiset** (all `.rs` outside manifests): the only lines lost or gained are mount lines, the facade, header imports, `pub(crate)` to `pub` on the component text codecs, the merged leaf tests and the new owner/record code (reviewed one by one).
- **Orphan rule** (manual audit of every `impl ... for` header per crate): all foreign-trait impls sit with a local type or a local generic argument, including `From<FlowHostSnapshot> for FlowHostComponents`. The compiler cannot confirm it here (coherence is not reached with unresolved externs).
- **Definition scan**: `xscan2.py` (direction-aware: non-pub items, structs with private fields and private inherent methods of an upstream crate whose type name a downstream crate mentions). Log `🗑️generated/flowflow-defscan.log` (61 candidate lines, all name collisions with local variables or same-named local items of other crates; the one real boundary type with private fields, `SynapseDsl`, is only passed through, never destructured downstream). Seeded check (`flowflow-defscan-seeded.log`): a private function, a private field and a private method are all reported. Items found genuinely cross-crate were widened: `widget_chrome`, the component codec items and `apply_flow_collection_delta` are `pub`.
- **rustfmt parse** of all 52 `.rs` files (`rustfmt +nightly-2026-07-20 --check --config skip_children=true`): 0 parse errors. TOML (Bun) of the six manifests, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (Bun JSONC): all parse; owner names unique; members byte-sorted; the 30 new routes unique in each file and colliding with no other order key. Both edited `.ts` scripts transpile (`bun build --no-bundle`).
- **TS tests**: `🧪️tests/🏷️slider-labels/🟦️.ts` passes (1 pass, 10 expect calls). `flowSelectedCopySelfTests` and `flowTypedRetirementSelfTests` fail with "source linkage missing/failed" **identically on the untouched original tree** (sandbox with the original artifact dir), i.e. they assert source strings that are already absent (for example `owned_variant!(Bytes,Strings,...` versus the real `owned_variant!(Spec,Neuron,...`); they are stale, not broken by this carve.
- **Real compile**: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-artifact-flow-flow-model --message-format=short` first failed on the stale lock (`--locked`); after the lock refresh it fails with `Rust crate Source refused before compiler: semio-framework-os-kernel (Rust crate Source boundary exceeded ...)` (peer-owned). Logs `flowflow-cargo-check-start.log`, `flowflow-cargo-check-model.log`, `flowflow-lock-update.log`. Nx route `bun repo run @semio-tech/framework-flow-flow-model-rs:check --configuration=prototype --parallel=1 -- --lib --offline`: see the final status in section 8.

## 7. Residual risks

- **Privacy and type-check unverified.** Field and method privacy across the new crate boundaries, trait-in-scope method resolution in the rewritten headers and test scopes, and coherence are reasoned and scanned, not compiled. The first real `cargo check` may report a missed `pub` or a missing trait import in a laws crate header (`mutation-laws` and `snapshot-laws` replicate the old module scopes by `pub use` of the document crate plus a few `os_spr` imports).
- **Pre-existing derive paths.** `#[derive(Mutations)]` and `#[derive(MutationLeaf)]` emit `::semio_framework_os_command::...` / `::semio_framework_os_store_one_item::...` paths and a `#[cfg(test)]` payload-law test that need those crates as direct dependencies; the original manifest did not list them either, so the document manifest mirrors the original (os-kernel with `mutation-testing` as dev-dependency). The payload-law test computes its fixture paths from `CARGO_MANIFEST_DIR`, so it follows the new manifest automatically.
- **Plugin workspace locks.** About 45 per-workspace `Cargo.lock` files (plugins, `✏️s`, `🌎️hub`, dev workspaces) still list the old facade dependency list and need the same refresh when the launcher allows. Only the root lock was refreshed.
- **Dependency ratchet.** `serde` is now declared by model and document, `serde_json` by model's, copy's and both laws crates' dev-dependencies (new `users` rows only, no new third-party crate). `verify dependencies` is red repo-wide for unrelated reasons (section 8).
- **Behavioural deltas.** `FlowOwner` lost the `Mutation` variant and its `HostSnapshot` payload became `FlowHostComponents` (same cursor; the enum's size may differ, which only shifts the exact queue-frame byte counts that the retirement laws compute relative to the API). Merged and inlined files change only file layout.
- **Tests not run.** No Rust test could run. The retirement laws are mounted in the copy crate, the mutation catalogue and snapshot laws in two laws crates; the allocation observer (`#[global_allocator]`) must stay alone in the copy test binary.
- **Tool leak check.** All analysis ran in `/private/tmp/.../scratchpad/ff` sandboxes; the repo writes were the applied work tree (rsync from the scratch work tree), registration, and the three consumer edits. Do not re-run `mkcrates.py`/`s*_*.py` against the repo: they are one-shot and assert on the original text.

## 8. Final status of the Nx route and the repo gates

- `bun repo run @semio-tech/framework-flow-flow-model-rs:check --configuration=prototype --parallel=1 -- --lib --offline`, attempted three times: a 280 s capped run was still in graph construction at the cap; two uncapped runs (logs `flowflow-nx-check-model-attempt1.log` and `flowflow-nx-check-model.log`) ran for 20 to 25 minutes in graph construction (`selected-compile-imports`, more than 30,000 progress events) and then failed with `Original selected owner registry changed` (`🕸️graph/🟦️.ts:273`). `📋️build-owners.json` is edited by the fleet during that window (2,217 rows when I started, 2,221 later, all five flow rows still present), so the selected-scope key it compares no longer matches. The route never reached `cargo`, and the direct launcher route is refused earlier by the `os-kernel` boundary anyway.
- `bun ./📜️script.ts verify taxonomy report`: started, did not finish within the 250 s cap (log `flowflow-verify-taxonomy.log`, still in file inventory). The new directories reuse emoji already registered in `🔣️taxonomy.json` (`🧱️`, `🧾️`, `🔬️`, `⚖️`) and the repo's own Rust path resolver (`resolveRustPathAttributes`) understands the inline `#[path = "dir"]` modules I introduced; the slugs `model`, `document`, `host` and `leaf-contracts` are not individually registered, as is the case for the existing `🧱️core` / `🎞️session` carve directories.
- `bun ./📜️script.ts verify dependencies` (log `flowflow-verify-dependencies.log`, about 13 minutes): exits 1 repo-wide with 2,416 "NEW dependencies not in `🔒️dependencies.json`" (it scans `.cache` toolchain manifests and many plugin manifests; the baseline lists 256). None of the five new manifests nor the facade appears in that output, and the new crates declare no third-party crate that was not already declared by the old crate (`serde`, `serde_json`). The gate is red at the base for unrelated reasons, so it cannot confirm or refute this carve.
