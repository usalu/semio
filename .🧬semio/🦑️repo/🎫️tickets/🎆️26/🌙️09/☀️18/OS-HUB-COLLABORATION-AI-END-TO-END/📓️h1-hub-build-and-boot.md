# H1 — Hub build and boot (`🌎️hub`, crate `semio-hub`, nx project `os-hub`)

Slice H1, 2026-09-18/19. All commands run from `/Users/ueli/Documents/semio`. Captures under
`🗑️generated/h1-*.txt` in this ticket folder. This report replaces the audit's §3/§5 findings with the
current measured truth; where a claim is compile- or test-only rather than observed at runtime, it says so.

**Inherited work.** A previous H1 worker died mid-slice at ~22:54 on 09-18 (its captures are
`🗑️generated/h1-hub-serve.txt`, `h1-hub-test.txt`, `h1-hub-test-sqlite.txt`, `h1-tail.txt`, and the artifact
root `🗑️generated/h1-artifacts/`). Its edits were still uncommitted at the start of this session and are now
part of commit `cd96692e52` (auto-commit). Verified as already done by it, not redone here:

- the audit's two `E0560` errors (`SpaceArtifactCreationReadyV1 { document_id: … }` →
  `artifact_id: …`) at `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:64` and
  `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:249`;
- the audit's `os-hub:dev` import off-by-one at
  `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts:2-3` (5 × `../` → 4 × `../`);
- the `--all-features` removal from `TestScript` plus the `test-all-features` nx target.

## 1. Current build truth

| command | result | capture |
|---|---|---|
| `cargo check -p semio-hub` (default features) | **fails, not in hub code** — 6 errors in `semio-s-artifact-stdio-semio`, a dependency of hub's `native-artifact-execution` feature | `h1-check.txt` |
| `cargo check -p semio-hub --no-default-features --features sqlite` | **green**, 7 warnings (proof the expansion really ran) in 2m52s | `h1-check-sqlite-only.txt` |
| `cargo test -p semio-hub --lib --no-default-features --features sqlite` | see §5 | `h1-test-sqlite-only.txt` |

The audit's `E0560 ×2` are gone: hub's own source compiles. What fails now is a **peer's live refactor**, not
hub: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/**` was rewritten between 22:52 and 23:31 on 09-18
(`PdfPage::text` became a method, `PdfSnapshot` grew from 6 to 28 fields), and the callers in
`semio-s-artifact-stdio-semio` were left on the old API:

```
✏️s/…/🧿️semio/…/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs:38  E0615 `text` is a method, not a field
✏️s/…/🧿️semio/…/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs:74   E0063 missing 25 fields in `PdfSnapshot`
```

`semio-hub`'s `native-artifact-execution` feature depends on `semio-s-artifact-stdio-semio`
(`🌎️hub/📦️packages/🦀️rust/Cargo.toml:31`), and `semio-s-plugin-stdio`'s `full-artifact-catalog` lists it too,
so this break blocks `cargo check/build/test -p semio-hub` on default features, `os-hub:build-dev`, and the
trusted stdio+GIS bundle the dev boot materializes. It is slice P3's to fix; per the preamble (rule 3) I did
not touch it.

**Session 3 update (2026-09-19 02:0x–02:1x).** Re-measured, not inherited. `cargo check -p semio-hub`
still failed on the same dependency but with a *smaller* footprint — 4 errors, all in the `📑️document`
subset, because P3 had already ported the `🖊️drawing` subset at 01:20:

```
…/🪆️subsets/📑️document/…/📥️import/…/📖️pdf/🔖️1.7/✳️any/🦀️.rs:38  E0615 ×2   `text` is a method, not a field
…/🪆️subsets/📑️document/…/📤️export/…/📖️pdf/🔖️1.7/✳️any/🦀️.rs:46  E0615
…/🪆️subsets/📑️document/…/📤️export/…/📖️pdf/🔖️1.7/✳️any/🦀️.rs:74  E0063 missing 25 `PdfSnapshot` fields
```

P3 then edited both remaining files at 02:11/02:12 while this slice was running. Final state: see §6.3.

## 2. Root fix — a zero build budget is unlimited, not expired

`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9312-9340`. `trustedBootstrapBuildControl(deadlineMs)` computed
`remainingMs: () => Math.max(0, deadlineMs - elapsed)`, so the repository default
`SEMIO_BUILD_BUDGET_MS` unset → `BUILD_BUDGET_MS = 0` ("zero leaves compilation and Cargo lock waits
unlimited", `🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:13-19`) made the very first `check()` throw
`trusted codec capture cancelled`, and `os-hub:dev` against an empty `OS_HUB_DATA` exited 1 before doing any
work (C1's §4 diagnosis, confirmed).

Fixed inside the control, which is where the misreading lives, so all eight of its call sites are covered:

```ts
const UNLIMITED_TRUSTED_BUILD_DEADLINE_MS = 86_400_000;
const deadline = deadlineMs > 0 ? Math.min(deadlineMs, UNLIMITED_TRUSTED_BUILD_DEADLINE_MS) : UNLIMITED_TRUSTED_BUILD_DEADLINE_MS;
```

`Infinity` is not an option: `freshRun` (`🧰️framework/…/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts:17-18`)
refuses a budget outside `1..86_400_000 ms`, so the unlimited budget is expressed as that contract's own
ceiling. The call shape `trustedBootstrapBuildControl(buildBudgetMs())` is kept because the script's own
source fence (`📜️script.ts:12304`, "materializer bounded budget") pins it.

Second site fixed: `📜️script.ts:9635` computed `Math.min(buildBudgetMs(), 300_000)`, which turns the
unlimited default into `0` *before* the control sees it; it now reads
`Math.min(buildBudgetMs() || 300_000, 300_000)`, matching the existing repo idiom at
`🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts:64`.

## 3. Root fix — the staged dev binary stages itself

`🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:82-96`. `hubDevBinaryPath` used to read
`dist/build-dev/os-hub` and throw when it was missing. It now stages it through its own Nx target and only
fails if that staging fails, naming the target in the message:

```ts
export const HUB_DEV_BINARY_TARGET = "os-hub:build-dev";
export function hubDevBinaryPath(root: string, staging: HubDevBinaryStaging = nativeHubDevBinaryStaging): string
```

The staging step is injectable (`HubDevBinaryStaging`) precisely so the law below can exercise all four
outcomes without running a 5-minute build.

## 4. New unit test and nx target

`LocalBootstrapLaunchCheckScript` (`📜️script.ts`, registered as `local-bootstrap-launch-check`) proves both
fixes:

- `buildBudgetMs()` is `0` with `SEMIO_BUILD_BUDGET_MS` deleted, and `trustedBootstrapBuildControl(0)`
  then reports a positive, safe-integer `remainingMs()` within one minute of the 24 h ceiling (with the old
  code this assertion reads `0`);
- an opted-in budget (`5_000`) still counts down, and a negative budget is treated as unlimited rather than
  as expired;
- `hubDevBinaryPath` against a missing binary: staging is attempted exactly once; a non-zero staging status
  and a zero status that produces no file both raise a message naming `os-hub:build-dev` and the exact path;
  a staging that produces the file returns it; a second call with the file present does not stage again.

```
$ cd 🌎️hub/📦️packages/🦀️rust && bun ./📜️script.ts local-bootstrap-launch-check
local-bootstrap-launch: unlimited-budget=1 opted-budget=1 negative-budget=1 staging-cases=4 passed
```
Capture: `h1-launch-check.txt`. Registered as nx target `os-hub:local-bootstrap-launch-check`
(`📋️project.json`). Running it *through* nx did not complete: the nx daemon timed out computing the project
graph under the current fleet load (`h1-launch-check-nx.txt`, `NX The daemon timed out while processing
REQUEST_PROJECT_GRAPH`) — an environment condition, not a target defect; peers' `nx show projects` calls were
20 minutes old at the time.

## 5. Hub tests

All runs below are session 3, 2026-09-19 02:0x–03:xx, from `/Users/ueli/Documents/semio`, with
`SEMIO_TEST_ARTIFACT_DIR` supplied (the hub's own `TestScript` now defaults it — §5.4).

### 5.1 Measured results

| run | result | capture |
|---|---|---|
| `cargo check -p semio-hub` (default `sqlite,native-artifact-execution`) | **green**, lib + bin `os-hub` warnings emitted (proof the expansion ran) in 36 s once §5.2's three external blockers cleared | `h1-check.txt` |
| `cargo test -p semio-hub --lib --no-default-features --features sqlite` | **117 passed / 1 failed** → after the §5.3 fix, that one law passes | `h1-test-sqlite-only.txt` |
| `cargo test -p semio-hub --no-run --bins` | **green** after the §5.3 bin-test fixes (was 39 errors) | `h1-bin-test-errors.txt` |
| `bun ./📜️script.ts test` (the `os-hub:test` route, default features, nextest) | 263 tests; run 1 stopped at 2 failures (both fixed in §5.3); see §5.4 for the level-budget finding | `h1-test-default.txt` |

`--all-features` is *not* required by any of these: the `test` target runs the crate's default feature set,
and `--all-features` lives only behind the separate `test-all-features` target (it additionally links the
postgres/neo4j drivers whose laws need a live Docker daemon).

### 5.2 What was blocking, and who owned it

Three independent external breaks had to clear before `cargo check -p semio-hub` could even reach hub code:

1. **stdio-pdf `PdfSnapshot` caller drift** (slice P3). At 02:03 it was still 4 errors, all in the
   `📑️document` subset; P3 had already ported `🖊️drawing` at 01:20 and landed the rest at 02:11/02:12.
   Verified cleared at 02:22 — not touched by this slice.
2. **`semio-s-artifact-gis-gismap`, 78 errors** — `MutationLeaf source authority failed: descriptor owner
   does not exactly match source owner`, cascading into 12 `E0277: SetCamera/SetLodMode/… : MutationLeaf is
   not satisfied`. Root cause: the peer's `⚙️config` → `🎚️config` directory rename left the `owner` string
   inside each mutation's `🔣️.json` descriptor pointing at the old path, and `expand_mutation_leaf`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:557`) refuses a descriptor whose declared owner
   is not its own directory. A repo-wide sweep found **exactly 7** such descriptors, all under
   `✏️s/🔌️plugins/🌍️gis` (6 in `🗺️gismap`, 1 in `🏔️gisterrain`); each `owner` is now its own directory. Fixed
   here because it is pure rename drift and it blocked the whole slice; re-verified zero stale owners repo-wide.
3. **`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1814`** (`?` returned `MutationGuard` where `Connection` was wanted) —
   a peer's live credential-audit edit at 02:37, gone by 02:40 without intervention. Recorded, not touched.

### 5.3 Hub test defects fixed here

| file | defect | fix |
|---|---|---|
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:8` | 5 × `E0425 cannot find type PathBuf` — the module reaches `PathBuf` only through `use super::*`, and `trusted_catalog` no longer imports it | explicit `use std::path::PathBuf;` (this also covers `🧪️tests/📤️publication/🦀️.rs`, which is `include!`d at line 53) |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:44-55` | the inherited `std::fs::canonicalize(path)` resolved the configured root *itself*, which made `trusted_catalog_opened_root_rejects_linked_roots_leaves_intermediates_and_actors` fail with "linked initial data root was admitted" | canonicalize only `path.parent()` and re-join the leaf, so link-refusing `O_NOFOLLOW` still applies to the configured root and everything under it, while a symlinked *ancestor* (macOS `/var` → `private/var`) resolves |
| `…/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:154-170` | the inherited regression law asserted the opposite of the committed security law | rewritten as `server_owned_data_root_resolves_linked_ancestors_and_still_refuses_a_linked_root`: link-through-ancestor reads its pointer, a symlinked configured root is refused, a pointerless root reports "no catalog" |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:1-7` | 25 × `E0425/E0422/E0433` — the bin-test module inherits only what `🏗️bootstrap/🦀️.rs` imports, and production does not use `DirectoryCommandClaimV1`, `DirectoryCommandResultKindV1`, `DirectoryCommandDispositionV1`, `DirectoryCommandReceiptRecord`, `DirectoryCommandReceiptCompletion`, `replay_directory_command_receipt`, `same_lease_fields_v1`, `CheckpointPublicationBlobV1`, `CheckpointPublicationFrontierV1` | explicit imports in the test module (never widening production imports) |
| `🌎️hub/🧪️tests/🔬️standalone/🦀️.rs:1` | `E0425 CommandResult` | explicit `use semio_hub::directory::CommandResult;` |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (8 sites) | `E0308` — `DirectoryCommand::AnnounceDocument.descriptor` and `DirectoryStreamMessage::Event.event` are now `Box<…>` in the kernel; the tests still passed them unboxed | `Box::new(…)` at 4 descriptor sites, `Box::new(…)` at the event construction sites, `*event ==` at the two `matches!` guards |
| `🌎️hub/🏗️bootstrap/🦀️.rs:4776-4790` | **production defect**: `shutdown_with_deadline` held a `std::sync::MutexGuard<ArtifactCreationHttpTaskOwnerStateV1>` across `tokio::time::timeout_at(...).await`, so the future was not `Send` and `tokio::spawn`ing a shutdown did not compile (`error: future cannot be sent between threads safely`) | the guard is confined to a block that yields a `drained: bool`; the await now happens with no guard in scope |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json:49,51,52` | `linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure` expected preview frontier `["stdio"]` for `gis-dependency`/`gis-trailing-byte`/`gis-duplicate-field`, observed `[]` | the loader now validates **every** package (raw decode + `validate_descriptor`) into `staged` before it previews **any** package (`🔏️trusted-catalog/🦀️.rs:595-621`), so a descriptor-level failure is refused before the first private preview. Refusing earlier strictly strengthens the property the law is named for, so the three recorded frontiers are corrected to `[]` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3319` | `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` failed with `Preparation { role: "parent", reason: "validation failed: one-item semantic edit disagrees with its immutable Store authority" }` | `durable_group_test_outcome` (feature `durable-group-testing`, which `semio-hub`'s dev-dependency on `directory` enables) derives its `ArtifactStoreOneItemLiveAuthority` from the edit — `next_sequence_number: edit.sequence_number`, `next_clock: meta.timestamp` — but hard-coded `group_id: None`, while `validate_semantic_edit` (`🏪️store/🦀️.rs:14107`) requires `meta.group_id == authority.group_id` and the three-store GIS commit stamps every member's meta with the decision digest. Now `group_id: meta.group_id.clone()` |

### 5.4 Post-outage re-measurement (2026-09-19 11:0x, quiet tree)

The whole fleet was cut by the account session limit at ~08:15. On resume no `🌎️hub/**/*.rs` had changed
since 04:30, so this is a stable measurement rather than a snapshot of churn. AU1 had reported **141 passed
/ 33 failed** for the lib; with `SEMIO_TEST_ARTIFACT_DIR` exported — which the hub's own `TestScript` does
automatically and a bare `cargo test` does not — the same tree gives

```
cargo test -p semio-hub --lib --bins --no-fail-fast
test result: FAILED. 170 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 40.36s
```

Capture `h1-test-classify.txt`. **21 of AU1's 33 were the `SEMIO_TEST_ARTIFACT_DIR` panic**
(`.expect("ticket-owned catalog fixture root")`), i.e. an artefact of invoking cargo directly instead of the
`os-hub:test` route; they are not hub defects and they do not reproduce through the target.

The remaining **9 lib failures**, re-run single-threaded to separate load from logic
(`h1-test-isolate.txt`, `--test-threads=1`):

| law | reason | classification |
|---|---|---|
| `inference::wal::tests::inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces` | `"exact-committed-command" left "absent" right "verified"` | **one shared root** — see below |
| `inference::wal::tests::inference_wal_proof_rejects_hash_matched_noncanonical_or_wrong_actor_commands` | `a matching durable hash cannot bypass "trailing-byte-after-hlc"` | same root |
| `inference::wal::tests::chain::inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch` | `actual retained WAL "one-segment-three-commits" left false right true` | same root |
| `inference::wal::tests::chain::inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof` | `assertion failed: verifier.close_steps() > 0` | same root |
| `inference::sqlite::tests::gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once` | `Option::unwrap()` on `None` at `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:338` | same root |
| `inference::runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `got Some(Denied)`, wanted `Storage` | §7.1 |
| `inference::runtime::tests::gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission` | `worker pool shutdown: Busy { retained_uses: 1 }` | §7.7 |
| `inference::runtime::tests::gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | `approval reaches its retained cancellation phase: Elapsed(())` | §7.7 |
| `artifact_authority::trusted_catalog::tests::linked_stdio_gis_descriptor_failures_never_publish_a_partial_codec_closure` | `partial public codec after gis-catalog: stdio.xml` in the full run, **passes in isolation** | §7.8 (shared-registry test isolation) |

**The five WAL/sqlite failures are one defect, not five.** All of them funnel through
`committed_fixture_witness()` (`🧾️wal/🧪️tests/🔬️unit/🦀️.rs:332-341`), whose single `verifier.verify(target(&fixture,
&fixture["traces"][0], &durable), …).await.unwrap().unwrap()` is what returns `Ok(None)`. `verify` reaches
and parses the record — `record.verify_fixed_three_edits::<…>()` succeeds, otherwise the outcome would be
`Err(Invalid)` and the traces would read `"invalid"`, not `"absent"` — so exactly one of the three
`verify_committed_record` gates (`🧾️wal/🦀️.rs:255-279`) returns `false` for a record the same test just
wrote:

1. parent shape / `meta.mutation_id` / `parent.actor` / `meta.author_id` vs `target` (`:255-261`),
2. `sha256(to_json_string(&parent.forwards[0])) != target.proposal_hash` (`:264-266`),
3. `sha256(encode_server_stamped_command_v1(…)) != target.command_hash` (`:269-279`).

Gates 2 and 3 hash a **`GisMapMutation` JSON encoding**, and `target.{proposal,command}_hash` come from
`durable_fixture_record`, which builds the same mutation through `semio_s_artifact_gis_gismap`. The two
sides must therefore be computing the digest over differently-shaped values — that asymmetry is the defect,
and it is where the next session should put its first `eprintln`. This slice localized it but did not fix it
(§7.4); it is *not* caused by §5.3's `durable_group_test_outcome` change, because this suite's
`durable_edit` builds every `MutationMeta` with `group_id: None`, for which the new expression is
identical to the old hard-coded `None`.

### 5.5 The `os-hub:test` level budget does not fit the suite

`runCargoTestBudgeted` runs the active level's budget over the whole nextest **run** phase, and the
`fundamental` level is `15_000 ms` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1085-1090`).
Every one of hub's 263 tests is unscoped, i.e. `fundamental`, and single laws in the suite cost seconds on
their own (`linked_stdio_gis_…` alone is 4.6 s). The first run ended with

```
[budget] cargo nextest run … --profile fundamental … exceeded 15000ms — killed.
        Trim it, or assign it to a higher level (quick/long/exhaustive).
```

This is a real, honest gap in the target rather than a flake: even fully green, `os-hub:test` at the default
level cannot finish inside its own budget under fleet load. Nothing in this slice re-tiers 263 laws into
`mod quick`/`mod long`; §7 records it as the one hub-test item left open.

The inherited `TestScript` change is kept and is the reason the suite runs at all: `fixture_root()` in the
trusted-catalog, publication and browser-actor laws `.expect("ticket-owned artifact root")` on
`SEMIO_TEST_ARTIFACT_DIR`, so `📜️script.ts:2510-2517` now defaults it to a package-owned
`🗑️generated/test-artifacts` and `os-hub:test` reports assertions instead of panics.

## 6. Boot from a fresh `OS_HUB_DATA`

Probe: `🐍️h1-hub-boot-probe.ts` (this folder). It creates a brand new `OS_HUB_DATA` with `mkdtemp`, asserts
`SEMIO_BUILD_BUDGET_MS` is unset, resolves the binary through the fixed `hubDevBinaryPath`, starts the hub
through the real `startLocalHub` local-bootstrap handshake, polls `/readyz`, then curls one route per class
and stops the child by pid.

### 6.1 A second structural boot defect: a symlinked `OS_HUB_DATA` ancestor kills startup

First run, capture `h1-boot-fresh-data.txt`:

```
fresh OS_HUB_DATA=/var/folders/pm/…/T/h1-os-hub-data-hKCBbS   →   hub exited before readiness
child output: Error: ArtifactAuthority(Catalog("Not a directory (os error 20)"))
```

`configured_artifact_authority` → `TrustedCatalogLoader::load_current` →
`TrustedCatalogDataRoot::open_server_owned` walks the configured data root component by component with
`O_RDONLY | O_NOFOLLOW | O_DIRECTORY`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:245-285`). On macOS the default `TMPDIR`
is `/var/folders/…` and `/var` is a symlink to `private/var`, so `openat` returns `ENOTDIR` and the hub exits
1 **before binding a port** — for any data root under `/var`, with or without a catalog. This is exactly the
shape C1's collaboration e2e uses (`mkdtempSync` data dir per run), so it is a second independent reason that
suite never reached the hub.

Fixed at the root by resolving the operator-configured root once, before the walk
(`🛡️opened-root/🦀️.rs:44-52`): the ancestors of the configured root are not server-owned, while every path
*below* it keeps the descriptor-rooted, link-refusing walk unchanged. Regression law appended to
`🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs`
(`server_owned_data_root_resolves_symlinked_ancestors_and_still_refuses_linked_pointers`): a symlinked root
resolves and reads its pointer, a symlinked `current.json` *inside* the root is still refused, and a root
without a pointer still reports "no catalog".

### 6.2 Fresh-data-dir boot, observed

Second run with a canonical (non-symlinked) parent, capture `h1-boot-fresh-data-canonical.txt`, zero
environment overrides beyond `OS_HUB_DATA`/`OS_HUB_PORT`:

```
fresh OS_HUB_DATA=/private/tmp/h1-os-hub-data-2XvM2L port=8814 SEMIO_BUILD_BUDGET_MS=<unset>
readyz: status=not-ready directory={"ready":true} storage={"ready":true} adminAssets={"ready":true}
        artifactCasBarrier/artifactPublication/artifactCasSweeper={"ready":true} artifactAuthority={"ready":false}
/readyz                                    200 … "schema":"semio.hub.readiness/v1","mode":"development","bindScope":"loopback"
/healthz                                   404   (no such route — see the route list below)
/auth/sessions/me  (no credential)         401
/auth/sessions/me  (local credential)      200   {"schema":"semio.directory.session-authority.v1", … "sessionKind":"development-local"}
/directory/spaces                          200   []
/directory/event-page/v1                   400   (route live, rejects a cursorless request)
/admin/api/connections                     401   (route live, requires an admin subject)
/spaces/probe-space/documents/probe-doc    401
```

So on a fresh data root the zero-touch defaults really do stand themselves up: the sqlite directory
(`{data}/directory.db`) and the filesystem document store (`{data}/db`) are created and report ready, a
local-bootstrap credential authenticates, and the directory listing answers `[]`. `artifactAuthority` is
`false` because no trusted catalog has ever been published in this workspace (`find .🧬semio/🌐hub -name
current.json` → nothing); publishing it is what the dev route's materialization does, and that is what §1's
peer break blocks.

**Routes the router declares** (`🌎️hub/🏗️bootstrap/🦀️.rs:8085-8159`, 45 `.route(` entries): health `/readyz`
only (**there is no `/healthz`**); auth `/auth/sessions/me` (GET+DELETE); directory `/directory/commands`,
`/directory/spaces`, `/directory/spaces/{id}`, `/directory/invites/{token}/redeem`, `/directory/events`,
`/directory/event-page/v1`, `/directory/socket-grants`, `/directory/socket/v1`, and the scoped
`/directory/spaces/{space}/documents/{document}/{socket-grants,socket/v1}`; admin/presence `/admin/api/*`
(overview, spaces, users, **connections**, documents, events, operations, audit, intents) plus the `/admin`
SPA; documents/db `/spaces/{space}/documents/{id}` (+ open-plan, socket-grants, execution-target manifest/
component/descriptor/browser-actor, `socket/v1`, active-checkpoint pair), blobs
`/spaces/{space}/blobs/{hash}`, artifact creation `/spaces/{space}/artifact-creations*`, inference
`/spaces/{space}/documents/{doc}/inference/gis-map/*`, extension modules. Presence itself is not a REST
route: it rides the document and directory websockets and is surfaced read-only through
`/admin/api/connections`.

Shutdown: the probe stops the child through `finishLocalHub` (pipe close, wait, then `kill`), and the pid is
gone afterwards. Note the child reports **exit status 1 on a clean launcher-initiated stop**: closing the
local-bootstrap pipe is classified as `local bootstrap endpoint closed`
(`🏗️bootstrap/🦀️.rs:8484`), which is also why `DevScript`'s own exit promise rejects on Ctrl-C. Left as-is —
that is a fail-closed policy decision about losing the credential channel, not something to change from this
slice.

### 6.3 The real `os-hub:dev` route

Driver: `📜️h1-dev-boot.sh` (one boot: fresh `OS_HUB_DATA`, `OS_HUB_PORT=8816`, `SEMIO_BUILD_BUDGET_MS`
explicitly unset, poll `/readyz` with `curl`, dump the data root, stop by pid, then boot the **same** data
root a second time) wrapped in `📜️h1-dev-boot-retry.sh`. Both run the launch.json row's own command,
`bun nx run os-hub:dev`, not the probe of §6.2 and not `📜️script.ts dev` directly. Captures:
`h1-dev-first.txt`, `h1-dev-restart.txt`, `h1-dev-*-readyz.json`, `h1-dev-attempts.txt`.

**Finding A — the row hangs with the Nx daemon.** The first invocation sat at **0 % CPU for 20 minutes**
without leaving project-graph computation; `h1-dev-first.txt` is 105 lines of repeated
`Creating project graph nodes/dependencies with @repo/emoji-project-json` and nothing else, while
`.nx/workspace-data/d/daemon.log` showed the daemon busy processing file changes from the rest of the fleet.
Re-running the identical command with `NX_DAEMON=false` cleared the graph in ~30 s and went straight into
`os-hub:build-dev`. Every measurement below therefore ran daemonless; §7.6 records this as the Nx owner's
call, since the launch.json row as written is what a developer would run.

**Finding B — the fleet's churn is the dominant boot hazard.** Four of the retry driver's attempts died in
`cargo build` on a *peer's* half-saved file, none of them hub's own source: `semio-framework-plugin-host`
(`no method named diagnose_fault`), `semio-framework-server` (`missing field sagas` ×2), and
`🌎️hub/🗄️stores/🦀️.rs` (6 × `E0271`, W3b's new `ProjectionStore` impls against D1's in-flight
`impl Future + Send` port change). Each cleared on its own within minutes.

**Finding C — §2's zero-budget fix is confirmed at runtime, by the real route.** With
`SEMIO_BUILD_BUDGET_MS` unset, the route reached `os-hub:build-dev`
(`[nx-native] staged 1 deliverables in 🌎️hub/📦️packages/🦀️rust/dist/build-dev`) and then entered the
trusted stdio+GIS materialization, emitting

```
trusted-stdio-gis-bootstrap capture-codecs: 0/8
trusted-stdio-gis-bootstrap build: 0/8
```

and creating `{data}/trusted-catalog/{staging,build}-f6208fc81543da9eea575fec9b878c67/` in the fresh data
root. Before the fix this same path threw `trusted codec capture cancelled` on the **first** `check()`, with
no build and no data-root writes at all (C1 §4). That is the load-bearing difference, and it is now observed
through `bun nx run os-hub:dev` rather than through a probe.

**Finding D — the materialization is gated by the shared Cargo build-dir lock.** The route's next step is
`cargo rustc -p semio-s-plugin-stdio --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release`.
Under the running fleet that invocation sat in `prebuild_lock_exclusive` with **no rustc child of its own**
for over 25 minutes while a peer's hour-old `semio_s_plugin_stdio` rustc held the lock at ~30–50 % CPU.
Per the preamble this is a healthy holder, so it was not killed. See §6.5 for where this run ended.


## 7. Open gaps

1. **`inference::runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply`
   still fails** — the one hub law this slice could not land. §5.3's `group_id` fix moved it forward one
   whole stage: the `[DEBUG] GIS fixed-three assembly terminal failure=Preparation { role: "parent",
   reason: "validation failed: one-item semantic edit disagrees with its immutable Store authority" }` is
   gone and the three-store assembly now mounts, but `commit_prepared_approval` returns
   `Err(InferenceRouteErrorV1::Denied)` where the law wants `Err(InferenceRouteErrorV1::Storage)`
   (`💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:494`). `Denied` at that point can only come from
   `GisMapApprovalCommitErrorV1::Rejected` (`🏃️runtime/🦀️.rs:3042`), and since the assembly branch no longer
   prints its terminal, the rejection is now raised in the **journal/publication** turn rather than in
   assembly — i.e. the law's "public checkpoint refusal is a retained *nonterminal* publication" contract is
   not what the current three-store committer does. That needs the GIS-map commit owner, not an import fix.
   The assertion now prints the actual error so the next run states the stage directly.
2. **`os-hub:test` cannot finish inside its own level budget** (§5.4): 263 unscoped `fundamental` laws
   against a 15 s run budget. Re-tiering them into `mod quick`/`mod long` is a hub-wide edit this slice did
   not make.
3. **`os-hub:build` (release) was not executed.** The target now routes through the hub's own
   `📜️script.ts build` (§6.4) and the router registration is verified by reading, but a release build of
   `semio-hub` into a fresh `CARGO_TARGET_DIR` costs more than the fleet had to spare; only `build-dev`
   (the same `buildCargoArtifacts` path, dev profile) was actually run, by `os-hub:dev`.
4. **`--all-features` still appears inside seven non-`test` script checks** (`📜️script.ts:6223, 12548,
   12568, 12572, 13801, 14312, 14700`). The `test`/`test-quick`/`test-long`/`test-exhaustive` targets are
   clean, and `test-all-features` is the single opt-in; the remaining call sites belong to individual
   `*-check` targets and were left to their owners.
5. **`[DEBUG]` prints survive in hub test paths** (e.g. `🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:151`,
   `🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:582`, `🏃️runtime/🦀️.rs:1423`). They are load-bearing progress
   receipts for those laws today; removing them is a separate pass.
6. **The `os-hub:dev` launch.json row hangs with the Nx daemon** (§6.3). `NX_DAEMON=false` is a workaround
   this slice used to get the runtime proof, not a fix: under fleet load the daemon never answers
   `REQUEST_PROJECT_GRAPH` and the row sits at 0 % CPU indefinitely. The previous H1 session hit the same
   thing on `local-bootstrap-launch-check`. Whoever owns the Nx layer should decide whether the repo's
   launch rows set `NX_DAEMON=false` or the daemon's watcher is bounded.

## 8. Files changed

Session 3 edits by this slice (the session-2 edits it inherited — `📜️script.ts`'s build-budget control,
`🚀️local-bootstrap/🏃️execution/🟦️.ts`'s staging, the `TestScript` artifact root — are §2–§5.4 and are still
in the tree):

| file | change |
|---|---|
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs` | `open_server_owned` canonicalizes only the configured root's **parent** and re-joins the leaf, so a linked ancestor resolves while the root itself and everything below stay `O_NOFOLLOW` |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs` | inherited law rewritten as `server_owned_data_root_resolves_linked_ancestors_and_still_refuses_a_linked_root` |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs` | explicit `use std::path::PathBuf;` (fixes 5 × E0425 here and in the `include!`d publication module) |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json` | `gis-dependency`, `gis-trailing-byte`, `gis-duplicate-field` preview frontiers corrected to `[]` (loader stages every package before previewing any) |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | explicit imports for the nine drifted directory/checkpoint names; `Box::new` at 6 boxing sites; `*event ==` at 2 `matches!` guards |
| `🌎️hub/🧪️tests/🔬️standalone/🦀️.rs` | explicit `use semio_hub::directory::CommandResult;` |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `shutdown_with_deadline` no longer holds a `std::sync::MutexGuard` across its `await` — the future is `Send` again |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | the checkpoint-refusal assertion prints the actual route error |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | new `BuildScript`, registered as `build` |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `build` now runs `bun ./📜️script.ts build` in the hub package instead of the shared caching-cargo script |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` | `durable_group_test_outcome` derives `group_id` from the edit's meta like every other authority field |
| `✏️s/🔌️plugins/🌍️gis/…/🗺️gismap/…/🎚️config/🧬️schema/🧬️mutations/{🎥️set-camera,🎨️set-vector-style,👁️set-layer-visibility,📏️set-layer-stroke-scale,🔽️set-lod-mode,🖼️set-render-mode}/🔣️.json` | `owner` repointed from the pre-rename `⚙️config` path to each descriptor's own directory |
| `✏️s/🔌️plugins/🌍️gis/…/🏔️gisterrain/…/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json` | same |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | new `📦️build🗄️os-hub` row (`bun nx run os-hub:build`, group `4_build`, order `206.159`) in both the generated file and its seed |

Ticket-folder artefacts: `📜️h1-dev-boot.sh` (the real-route boot + restart prover),
`📜️h1-dev-boot-retry.sh` (retries it through the fleet's transient breakages), plus the inherited
`🐍️h1-hub-boot-probe.ts` and `📜️h1-dev-boot-attempt.sh`. Captures: `🗑️generated/h1-*.txt`.

<!--H1-FILES-->
