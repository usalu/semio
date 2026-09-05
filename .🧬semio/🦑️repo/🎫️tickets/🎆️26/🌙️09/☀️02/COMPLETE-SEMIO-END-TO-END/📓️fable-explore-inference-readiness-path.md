# Explore — Inference Readiness Path (dispatch packet)

Lane `fable-explore-inference-readiness-path`, read-only, 2026-09-05 ~21:40. Reconciles
`📓️fable-ai-map-proposal.md`, `📓️fable-mcp-inference-bridge.md`, `📓️fable-gis-map-inference-ui-port.md`,
`📓️fable-execution-target-lease.md`, `📓️fable-vcs-native-provider.md`, `📓️fable-coordinator-repairs.md`,
`📓️sol-trusted-stdio-gis-bundle.md`, the three `📓️terra-*` blueprints, `📓️sol-map-durable-group-decision-codec.md`
(newest relevant Sol report, 19:37) and `📓️root-gis-map-frozen-binding.md` (newest relevant Root report touching
this exact chain, 04:55, with a same-day 21:xx addendum) against CURRENT SOURCE, read directly. Every claim below
is marked **[source]** (I read the exact lines myself, today, in this exploration) or **[report]** (I am relaying
another lane's own claim, not independently re-verified by running anything — I ran no cargo/nx/bun build).

## 1. The exact chain from a checked-in profile to `features.inference == true`

**[source]** Traced in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`:

```
env OS_HUB_TRUSTED_CATALOG_BUNDLE + OS_HUB_TRUSTED_CATALOG_PROFILE   (bin.rs:6543-6544, both-or-neither, bin.rs:407)
  → configured_artifact_authority(bundle_path, profile, providers)   (bin.rs:403-413)
      → TrustedCatalogLoader::load → ::load_selected                (🔏️trusted-catalog/🦀️.rs:411,415)
          reads bundle JSON + component/descriptor bytes from disk, hashes them, calls
          providers.preflight_selection — a REAL file-backed loader, not a fixture
  → verified_catalog = Some(catalog)                                 (bin.rs:6575)
  → gis_map_binding = verified_gis_map_binding(catalog)               (bin.rs:6577-6579, feature "native-artifact-execution")
      → 🌎️hub/💡️inference/📇️catalog/🦀️.rs:278 verified_gis_map_binding
          → :205 verified_gis_map_binding_with_service requires the catalog's SOLE open target
            (open_target_count()==1 is enforced inside the loader itself, 🔏️trusted-catalog/🦀️.rs:571)
            to be plugin="gis", package="semio:gis", artifact.kind="s.gis.gismap",
            role==Editor, grant.write==true (:208) — a VIEWER/read-only target returns Ok(None), not an error
  → inference_runtime = gis_map_binding.map(|b| HubInferenceRuntimeV1::new(b, ledger, UnavailableGisMapApprovalCommitterV1))
                                                                       (bin.rs:6584-6591, feature "sqlite"+"native-artifact-execution", both DEFAULT features)
  → inference_ready = inference_runtime.is_some()                    (bin.rs:6593)
  → hub_readiness(..., inference_ready) → features.inference          (bin.rs:1999, 6596)
```

**Every link that is missing or unqualified today:**

1. **Bundle materialization.** The bundle referenced by the two env vars must be produced by
   `materializeTrustedStdioGisBundle` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, which calls
   `produceFreshComponentV1` (`🧰️framework/…/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:317`) **once per
   package** (stdio, then gis) — **[source]** each build is `cargo …rustc -p <pkg> --lib --crate-type cdylib
   --target wasm32-wasip2 --profile wasm-dev`, with `CARGO_INCREMENTAL=0`, `RUSTC_WRAPPER=""`,
   `SCCACHE_DISABLE=1` (script.ts:324-330 in the describe module) — i.e. **cold, no incremental cache, sccache
   explicitly bypassed**, for every single invocation. **[report]** `📓️fable-ai-map-proposal.md`'s own Round-2
   evidence shows a plain `cargo check --lib` on just the hub crate (warm sccache) sat inside
   `semio_s_plugin_stdio` for **4h06m with zero diagnostics** under today's box contention (~20 concurrent
   rustc). A cold, sccache-bypassed `wasm32-wasip2` build of the same stdio crate (26-codec closure) plus GIS
   (2-codec closure) plus a full native `--bin os-hub` build (also invoked inside the same gate,
   script.ts:5407-5410) should be assumed to cost **on the order of hours**, and has in fact **never completed**
   in any of today's reports.
2. **Registered command.** `os-hub:trusted-stdio-gis-bundle-check -- --native` / `-- --process`
   (`TrustedStdioGisBundleCheckScript`, script.ts:5372) is the gate that does this end to end: builds `--bin
   os-hub` into a ticket-owned `SEMIO_TEST_ARTIFACT_DIR/hub-target`, calls `materializeTrustedStdioGisBundle`,
   then `validateAndPublishTrustedStdioGisCandidate` (spins a real candidate hub, waits readiness, issues an
   authenticated GIS Map plan, only then publishes `current.json`). **[report]**
   `📓️sol-trusted-stdio-gis-bundle.md`'s own "Remaining acceptance" section (as of 00:17 today) lists all of
   `--native`/`--process` as **never run to completion**; only `--source` (fixture/AJV/design-invariant checks
   over the *design*, not real bytes) is reproducibly green.
3. **Candidate readiness / current pointer.** **[source]** `DevScript.run()` (script.ts:5234-5261) already
   wires the *zero-touch* path: `bun ./📜️script.ts dev` reads `trustedBootstrapCurrent(dataRoot)`
   (script.ts:4828), and if absent, calls `materializeTrustedStdioGisBundle` +
   `validateAndPublishTrustedStdioGisCandidate` itself, then passes the resulting `{bundlePath, profileId}` into
   `startLocalHub(...)`, which sets the two env vars for the spawned hub child (script.ts:730-738). **This means
   the wiring for an ordinary `os-hub:dev` to bootstrap trust with no manual step is already complete in
   source** — the only thing that has never been proven is that the cold multi-package wasm build plus hub
   binary build actually finishes and the resulting candidate reaches readiness.
4. **`gis_map_binding` construction.** As of **today** this is aligned, not still broken:
   `📓️root-gis-map-frozen-binding.md`'s "September 5 Verification Continuation" section **[report]** records
   that the `local-stdio-gis-open-v1` profile *previously* selected a **viewer** target (which
   `verified_gis_map_binding_with_service` rejects at line 208, returning `Ok(None)`, silently producing
   `features.inference=false` even with a fully materialized bundle) and was changed today to select the
   **editor** target with `read+write+observe` and `wasm` renderer — matching what the binding constructor
   requires. This is a *near-miss* worth recording exactly because it shows how silently this chain can stay
   `false`: `verified_gis_map_binding` treats a wrong-role selection as `Ok(None)`, not an error, so a
   materialized-but-misconfigured bundle produces no diagnostic at all, just `features.inference: false`.
5. **Compile health.** Every lane's own evidence this session (fable-ai-map-proposal round 1/2, execution-target-lease,
   coordinator-repairs, gis-map-inference-ui-port's missed-exhaustive-match incident) shows the hub/plugin-host
   crate graph has been **repeatedly RED for hours at a time** from concurrent lanes editing shared files
   (`Effect::RequestInferenceProposal` non-exhaustive match, `WalWriterPermit` type churn in `🛢️db/🗜️compact/🦀️.rs`,
   stale directory-client imports). **[report]** No lane has reported a clean `cargo build -p semio-hub --bin
   os-hub` (default features) completing today. This is a precondition for step 2/3 above that is currently
   unverified, independent of the wasm build time problem.

**Bottom line for (1):** the chain is now *fully wired in source* (env vars → loader → binding → runtime →
readiness, plus a zero-touch dev-time bootstrap), and the one design mismatch that would have made it
permanently false (viewer vs. editor target) was fixed today. What remains is purely **execution**, not design:
run `os-hub:dev` (or the dedicated `--native`/`--process` gate) through a full cold two-package wasm32-wasip2
build plus a clean `--bin os-hub` build, on a box that is not fighting ~20 concurrent peer `rustc` processes for
the shared `target/` lock, and confirm the candidate reaches `/readyz`. **No one has done this successfully as
of this exploration.**

## 2. Smallest honest non-`cfg(test)` `test_support` bundle builder

**[source]** The only existing catalog fixture builder is `prepared_fixture()` in
`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1445`, which is inside `mod tests` gated
`#[cfg(test)]` (line 1172) — invisible to the `os-hub` **bin** target and to every other crate. It also builds a
**generic fixture artifact** (`fixture.document.catalog.<pid>.<seq>@1`, a fake stdio-derived descriptor, 3-byte
`"abc"` component bytes) — it does not, and structurally cannot, satisfy `verified_gis_map_binding`'s exact
identity checks (`kind == "s.gis.gismap"`, `plugin_id == "gis"`, `package_id == "semio:gis"`, the declared
`s.gis.gismap.inference` service, `native.executable_identity()` equal to the real
`gis_map_inference_service()`), because those checks compare against the **real, compiled GIS plugin's own
descriptor/service metadata** (📇️catalog/🦀️.rs:243-263), not an arbitrary fixture kind.

**What the smallest honest builder needs, concretely:**

- It must live in a crate the `os-hub` bin target and Rust-law tests can reach **without `#[cfg(test)]`** — the
  natural home is a new sibling module beside `prepared_fixture`, e.g.
  `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️test-support/🦀️.rs`, exported only under a Cargo
  **test-only feature flag** (not `cfg(test)`, which is per-crate and invisible cross-crate), e.g.
  `semio-hub = { features = ["test-support"] }` — mirroring how `semio-s-plugin-stdio`'s own
  `full-artifact-catalog` feature is already used to gate catalog-shaping code (Cargo.toml:30).
- It must produce a **real** `VerifiedGisMapArtifactBindingV1`-satisfying selection: `plugin_id="gis"`,
  `package_id="semio:gis"`, `artifact.kind="s.gis.gismap"`, `artifact.schema="gis.map"`, role=Editor,
  `grant.write=true`, `renderer_target` ∈ {wasm}, and a package whose `component_sha256`/`component_blake3`/
  `descriptor_byte_sha256` are **hash-bound to real bytes it actually writes to disk** (the loader reads real
  files and re-hashes them — `read_bounded` + `dual_hash`, 🔏️trusted-catalog/🦀️.rs:445-451). It does **not**
  need a real *executing* wasm component — `prepared_fixture`'s pattern of a short placeholder byte string
  hashed for real is sufficient, **as long as the descriptor's declared `s.gis.gismap.inference` service metadata
  and `GIS_MAP_NATIVE_EXECUTABLE` identity match the actual compiled `semio_s_plugin_gis::gis_map_inference_service()`
  in-process** (📇️catalog/🦀️.rs:19, :278) — i.e. it must reuse `semio_s_plugin_gis::artifacts::gismap::gis_map_inference_service()`
  as the service the fixture descriptor declares, exactly like `prepared_fixture` reuses the real
  `semio_s_plugin_stdio::plugin()` manifest today (lines ~1405-1411) rather than inventing one.
- "Reusing `prepared_fixture`'s data" concretely means: keep its JSON-shape helper
  (`fixture_json()`/the bundle-v2 JSON skeleton, its cancellation/rotation/hostile machinery) and its
  file-writing discipline (`root.join("components")`/`("descriptors")`, `std::fs::create_dir_all`), but swap the
  **stdio-manifest-derived fixture app row** for a **gis-manifest-derived one** whose `artifactKinds` includes the
  real `s.gis.gismap` `ArtifactKindSpec` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:176`) and whose
  `contributions.inference_services` includes the one real `s.gis.gismap.inference` row, so the checks at
  📇️catalog/🦀️.rs:220-225 (`services.next()` / no-duplicate) pass against real production code rather than a
  synthetic double.
- **Where it must live** so `os-hub` bin tests can construct `HubInferenceRuntimeV1`: the constructor call
  itself (`HubInferenceRuntimeV1::new(binding, ledger, committer)`, `🏃️runtime/🦀️.rs`) is `pub`, so once the
  test-support builder produces a real `Arc<VerifiedGisMapArtifactBindingV1>` via
  `TrustedCatalogLoader::load` (or a direct in-memory constructor that skips the file round-trip but reuses the
  **same verification code path**, to avoid a second, divergent trust check), `os-hub`'s own `#[cfg(test)] mod
  tests` in `🚀️bin.rs` can call it directly. The builder crate does **not** need to depend on axum/bin.rs; it
  only needs to depend on `semio-s-plugin-gis` (already an optional dep of `semio-hub` under
  `native-artifact-execution`, Cargo.toml:41) and `semio-hub`'s own trusted-catalog module.

**Risk to flag:** this exact ask ("non-`cfg(test)` `test_support` GIS Map bundle builder") is **owned by neither
lane today**. `📓️fable-ai-map-proposal.md`'s own "Next steps" #1 asks for it explicitly and says "it is not
written here." Sol's trusted-bundle lane (`📓️sol-trusted-stdio-gis-bundle.md`) is the natural owner because it
already owns `🔏️trusted-catalog/🦀️.rs` and the fixture-JSON producer, but its own remaining-acceptance list is
entirely about the **real, non-test** materializer/candidate path, not a test-only builder — so this item risks
falling into the gap between the two lanes unless explicitly dispatched.

## 3. Atomic parent+child composition transaction — status is further along than either fable report knows

**[source] This is the most important correction to make in the dispatch packet.** Neither
`📓️fable-ai-map-proposal.md` nor `📓️fable-mcp-inference-bridge.md` references
`📓️sol-map-durable-group-decision-codec.md` (mtime 19:37, i.e. it landed *before* fable-ai-map-proposal.md's own
20:33 report, but is not mentioned by it). Reading current source:

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` (3071 lines) is **not** a
generic three-member primitive — it is **specifically shaped for GIS Map**:

- `DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`
  (line 715) — a fixed, non-replaceable host slot for exactly one parent + `"gismap-drawing"` +
  `"gismap-value"` triple.
- Hardcoded identity checks: `self.anchor.parent.dialect.to_coordinate() != "s.gis.gismap@1/*"` (line 2137),
  child slots `"gismap-drawing"` / `"gismap-value"` (lines 1853-1854, 2003-2004, 2142-2143), parent recovery
  schema `"semio.store.one-item-outcome.gis-gismap-v1"` (line 25).
- This is **exactly** the shape `GisMapInference::create_region_group_work` in
  `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:92-164`
  produces: **a `CreateRegion` approval is not parent-only** — it is a fixed three-member group of
  `parent` (the `CreateRegion` mutation itself, tag 9), `drawing` (a `SemioDrawingMutation::CreateNode` on the
  paired `"gismap-drawing"` child, which every valid Map snapshot has per the `snapshot.drawing.child_id ==
  "gismap-drawing"` guard at line 100), and `value` (a `SemioValueMutation` `insertListItem` on the paired
  `"gismap-value"` child). The function's own internal invariant (line 105-108: applying `parent` alone must
  leave `after.drawing == snapshot.drawing && after.value == snapshot.value`) proves the Map's own apply
  function does **not** keep drawing/value in sync — they must be committed atomically alongside the parent or
  the Map becomes internally inconsistent (a new region with no corresponding drawing node / value entry).
- **This directly contradicts what actually landed in `HubInferenceRuntimeV1::infer`.** Per
  `📓️fable-ai-map-proposal.md` §C: "`HubInferenceRuntimeV1::infer` … derives the sole `bounds_proposal`
  `CreateRegion`" and `server_stamped_command` "rebuilds the `CreateRegion` **and** its inverse" — i.e. the
  landed approval path uses only `GisMapInference::bounds_proposal` (the parent-only mutation), **never**
  `create_region_group_work` (the real three-member group). **This is worth flagging as an open design question
  for whoever picks up the real `GisMapApprovalCommitterV1`**: either (a) the fable lane's parent-only approval
  is an intentional, acknowledged V1 simplification that will leave drawing/value out of sync and needs a
  follow-up widening to the three-member group before it is production-correct, or (b) `server_stamped_command`
  needs to be rebuilt on `create_region_group_work` so the committer this section describes has three members to
  commit, not one. Nothing in `📓️fable-ai-map-proposal.md` states this choice was made deliberately.

**What the durable-group module actually proves today, and what it does not:**

- 12 kernel-local laws pass (`durable_group::tests::*`, `sol-map-durable-group-decision-codec.md`'s own gate
  table) covering: the codec/hash/bounds fence, Store-owned prepare/bind/recover, the shared
  `ArtifactGroupVisibilityOwner` flip (staging → one-bit commit → adopt parent/drawing/value in order →
  retirement), cancellation-before-commit (restores all three old roots), and uncertain-journal retry.
- **[source, report's own words]**: "The mounted laws still use demo Stores and the fake journal sink, so this
  remains Store-contract evidence rather than end-to-end durable Map acceptance." The "Remaining proof" section
  is explicit: "the remaining integration boundary is **the public Store admission that constructs this host
  from typed GIS Map preparation**, plus **a storage-owned WAL sink with exclusive lifetime writer authority**,
  append/sync receipt verification, and reopen recovery."
- `mount_map` (line 852) and every constructor on `DurableOwnedThreeStoreCommitV1` are `pub(super)` — **not
  public outside the `os_store` module** — so nothing outside `🏪️store/🦀️.rs` (in particular, nothing in the
  hub crate) can call this today.
- `DurableOwnedGroupJournalSinkV1: Send` (line 265) is a synchronous, kernel-owned port —
  `fn begin_commit(&mut self, decision_pack: Vec<u8>, decision_sha256: String) -> Box<dyn
  DurableOwnedGroupJournalCommitV1>` — deliberately dependency-free of `db`, matching
  `📓️terra-durable-artifact-group-visibility-wal-p0.md`'s stated constraint ("there is intentionally no kernel
  → db dependency… the executor must therefore consume a kernel-owned journal port"). **No implementation of
  this trait against the hub's real per-document WAL exists yet.** The natural candidate is the hub's own
  `db_wal::ArtifactWal` (parent-anchor, `Fsync`, per `terra-durable-artifact-group-visibility-wal-p0.md` §5), but
  that requires the **exclusive-writer-permit** work in progress (`📓️root-wal-writer-authority.md`, newest at
  20:26 today): "the six mutating WAL APIs now require a non-cloneable writer permit... Postgres and Neo4j
  currently fail closed... ArtifactWal holds its permit from before inventory through terminal close" — i.e. the
  storage layer this sink would need to wrap is itself **mid-migration today**, actively touching
  `🛢️db/⚙️engine/🦀️.rs`, `🛢️db/🗄️storage/🦀️.rs`, `🛢️db/🗜️compact/🦀️.rs` (all currently `M`/`MM` in `git status`).

**Can a first `GisMapApprovalCommitterV1` honestly commit a Map with zero children through
`ArtifactGroupVisibilityOwner`/the WAL witness path?**

**[source]** Yes, for a **narrower** scope than the durable-group module's three-member shape, and this is
architecturally distinct from it:

- `commit_prepared_approval` (`🏃️runtime/🦀️.rs:350`) only requires a `GisMapApprovalCommitterV1::commit(...)` that
  returns a `GisMapApprovalReceiptV1 { witness: CommittedInferenceWalWitnessV1, document_generation }`.
- `CommittedInferenceWalWitnessV1` (`🌎️hub/💡️inference/🧾️wal/🦀️.rs:71`) has **no public constructor** — only
  `InferenceWalVerifierV1::verify` (line 161) mints one, and it does so by replaying **one single document's**
  existing `db::wal`/`WalReplayCursor` (line 7, `use db::wal::{WalCursorControl, WalRecord, WalReplayCursor,
  WalReplayStep}`) looking for a record whose `job_id`/`proposal_hash`/`mutation_id`/`command_hash` match
  (`InferenceWalTargetV1::validate`, lines 46-61). This machinery is **document-scoped, not group-scoped** — it
  is exactly the "green per document" primitive `terra-durable-artifact-group-visibility-wal-p0.md` §5 already
  certifies (`WalStorage`/`ArtifactWal`, one `ArtifactId` at a time), **entirely separate from the
  parent+drawing+value durable-group module in section 3 above.**
- Therefore: a `GisMapApprovalCommitterV1` that (a) submits the server-stamped command through the **existing,
  ordinary single-document `ArtifactEngine`/`ArtifactWal` commit path for the Map document alone** (no new child
  artifact created — i.e. literally "zero children" in the sense of "no drawing/value composition, no group
  frame") and then (b) calls `InferenceWalVerifierV1::verify` against that same document to mint the witness, is
  **buildable today without waiting for the durable-group module or the writer-permit migration**, because it
  only needs the already-green single-document WAL path. **This is the honest smallest first implementation**,
  and it is explicitly what the fable-ai-map-proposal.md packet's own boundary ("GisMapApprovalCommitterV1 is the
  private port… `UnavailableGisMapApprovalCommitterV1` is registered today") leaves open for a follow-up lane.
- **The tradeoff to state explicitly in the dispatch packet:** this parent-only commit reproduces the exact gap
  identified above — the committed Map gains a new region whose paired drawing node / value entry were never
  written, which `create_region_group_work`'s own invariant treats as a broken composition. Shipping this as the
  "first" committer is honest only if the dispatch packet says out loud that it is **not** the semantically
  complete GIS CreateRegion commit and that the real one needs the section-3 durable-group module (public Store
  admission + real WAL sink) once the writer-permit migration lands.

## 4. Two-user process law design — `gis-map-proposal-check --process`

**[source]** The registered `--process` mode of `GisMapProposalCheckScript` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:5145`)
currently only prints: `"the two-user authenticated journey needs the trusted profile and the atomic composition
transaction; it is NOT run or claimed here. No external model provider, no WGPU rendering."` (line 5176) — it is
a placeholder, not a runner, today.

**Exact runner reuse available for the real implementation:**

- `spawn_server(state: HubState) -> SocketAddr` (`🚀️bin.rs:7257`) and `issue_test_session(state: &HubState,
  email: &str) -> TestIssuedSession` (`🚀️bin.rs:7727`) are the **same two helpers** the currently-registered
  binary law `gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding` already uses (per
  `📓️fable-ai-map-proposal.md` §"Tests and gate"). They start one real in-process axum server against a real
  `HubState` and mint real authenticated sessions — exactly the primitives a two-user law needs, with **no new
  test harness required**.
- **[source]** `SpaceRole` (`🌎️hub/📇️directory/🦀️.rs:101`) has exactly two variants, `Author` and `Spectator` —
  there is no third "Admin"/"Viewer" role at this layer. `check_live_inference_author`
  (`🌎️hub/💡️inference/🛂️authorization/🦀️.rs:10-29`) admits only `SocketSessionBindingStatus::Active { role:
  Some(SpaceRole::Author), .. }` on every one of submit/events/cancel/approve. So the two-user design is
  necessarily: **user A = Author** (submits, polls, approves) and **user B = Spectator** (the "second user") in
  the *same* space+document.
- **The exact reading of the goal's "→ committed typed event to a second user"**: it cannot mean user B calling
  the inference routes (`check_live_inference_author` denies a Spectator outright with `inference.denied`, and
  `identity_of` in `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` "returns the frozen identity to the original owner only" per
  fable-ai-map-proposal.md §B). It must mean: after user A's approval commits (once section 3's committer is
  real), **user B observes the committed mutation through the ordinary document-open/sync path** — i.e. the
  second user's own document snapshot/WAL replay picks up the new region as a normal committed edit, proving the
  effect is a real shared-document mutation and not merely internal to the inference ledger.
- **Concrete law shape** (not yet written anywhere): `spawn_server` once; `issue_test_session(state, "author@…")`
  and `issue_test_session(state, "spectator@…")` against the same space/document, roles Author/Spectator
  respectively; drive A through submit → poll events → (once offered) approve; then, using the **existing**
  document-open/sync route (not an inference route) authenticated as B, assert the new `CreateRegion` region
  (and, if section 3 widens to the full group, the paired drawing/value change) is present in B's own read —
  proving cross-user visibility of the committed effect. This reuses the same `spawn_server`/`issue_test_session`
  pair `gis-map-proposal-routes-fail-closed…` already established; no new server-boot or auth machinery is
  needed, only (a) a real trusted GIS Map **editor** binding (section 1/2) and (b) a real committer (section 3).

## 5. Dependency-ordered file list, change-size estimates, collision risk

Ordered so each step's prerequisite is proven before the next is attempted; sizes are estimates from the
structural shape of what exists, not measured diffs.

| # | File(s) | Change | Size | Collision risk |
|---|---|---|---|---|
| 1 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | Run `os-hub:trusted-stdio-gis-bundle-check -- --native`/`--process` to completion (no code change expected if source is already correct; the risk is purely execution time/contention) | 0 (execution only) | **High** — Sol's lane is the active, sole owner and per `git status` `📜️script.ts` is currently `M ` (staged). Anyone else touching either file collides directly with an in-flight lane. |
| 2 | new `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️test-support/🦀️.rs` (or a `test-support` Cargo feature module beside `mod tests`) | Non-`cfg(test)` GIS Map bundle builder (§2) | Small–medium (~150-300 lines, reusing `prepared_fixture`'s JSON/file-writing skeleton) | **Medium** — same file Sol edits for the real materializer; must land as an additive module/feature, never edit `mod tests` itself. Coordinate timing with #1. |
| 3 | `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` (new `GisMapApprovalCommitterV1` impl, e.g. `SingleDocumentGisMapApprovalCommitterV1`), `🌎️hub/💡️inference/🧾️wal/🦀️.rs` (no change expected — `InferenceWalVerifierV1::verify` is already generic) | First honest parent-only committer (§3, narrow scope) | Medium (~100-200 lines: wraps the existing single-document `ArtifactEngine`/`ArtifactWal` submit + `InferenceWalVerifierV1::verify`) | **Low-medium** — this file is currently only fable-ai-map-proposal's own; check for fresh edits before starting, since the lane that owns it is "still running" per the goal. |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`, `…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` (public admission API), a new `🛢️db/🧩️group/🦀️.rs`-shaped `DurableOwnedGroupJournalSinkV1` impl over the real per-document `db_wal`/writer-permit primitive | Real (non-demo-Store, non-fake-sink) `GisMapApprovalCommitterV1` for the full parent+drawing+value group (§3, complete scope) | **Large** — this is the `terra-durable-artifact-group-visibility-wal-p0.md` P0 in full, now partially pre-built by Sol; still needs the public Store admission seam plus a real journal sink | **Very high** — `🏪️store/🦀️.rs` is enormous and shared by nearly every lane touching Store; `🛢️db/` is mid-migration under `root-wal-writer-authority.md` (six mutating APIs, all backends touched, `M`/`MM` in git status today). Do not start this before #3's narrower committer proves the route end to end, and not before the writer-permit migration reaches a stable state. |
| 5 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (`GisMapProposalCheckScript`, `--process` branch) | Real two-user process law (§4) | Small–medium (~80-150 lines using existing `spawn_server`/`issue_test_session`) | **Medium** — same file as #1/#2; land after #1 and (at least) #3 are proven, since the law needs a real editor binding and a real committer to be meaningful rather than another "not run" placeholder. |

**Fable AI lane status**: `📓️fable-ai-map-proposal.md` (20:33) and `📓️fable-mcp-inference-bridge.md` (21:28) are
both dated within the last hour of this exploration and describe themselves as mid-flight (Round-2 cargo build
still running detached at the time of the AI-map report; the coordinator-repairs log entries at 20:45/21:05 are
newer still). Treat `🏃️runtime/🦀️.rs`, `🧾️command/🦀️.rs`, `🪶️sqlite/🦀️.rs`, and `🚀️bin.rs`'s `//#region 💡️Inference`
block as **live** before editing.

## Honest nonclaims of this exploration

- I ran no `cargo`/`nx`/`bun` command and built nothing; every "chain" claim in §1/§3/§4 is a static read of
  current source, cross-checked line-by-line against the cited files, not a runtime observation.
- I did not re-verify whether `os-hub:trusted-stdio-gis-bundle-check -- --source` is green on the *exact current*
  tree (it last passed before the VCS lane's provider-set count moved 28→29 and before the stdio stale-path
  repairs landed at 20:45/21:05 per `📓️fable-coordinator-repairs.md`); I only confirmed the source text these
  gates assert against is internally consistent as of the files I read.
- Section 3's "open design question" (parent-only vs. full three-member `CreateRegion` commit) is a reading of
  two lanes' source that never reference each other; it is possible a peer has already reconciled this since
  either report was written, and this packet's job is to surface it for the dispatcher, not to adjudicate it.
