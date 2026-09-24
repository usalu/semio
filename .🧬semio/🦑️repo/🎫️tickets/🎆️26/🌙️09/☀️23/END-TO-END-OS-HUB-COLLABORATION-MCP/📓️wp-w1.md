# WP-W1 — All-Plugin Wasm: Descriptors, s Guests, Trusted Catalog + Hub

## Hub Handoff (for H2, C7/C8, G4)

- status: PENDING. Stage 1 hub 7800 on the stdio,gis,note,draw catalog (C7 run 3), then the 16-package catalog
  (`w1-catalog`), then extended as p4 lands the 18 derive-path packages.
- binary: `.tmp-ticket/wp-w1/bin/os-hub` (copy of H5's `wp-h5/bin/os-hub`, debug, 4 features, sha256 b3c6b3f2…0b1b)

## Status

- [x] 1 Inherit session-9 state (see §1)
- [ ] 2 Descriptors: generate → check → descriptor_is_fresh
- [ ] 3 s guests rebuilt/materialized/activated, receipt sha == fresh build
- [ ] 4 Trusted catalog published, hub 7800 ready, second hub binds by content hash
- [ ] 5 Requests served

## Log


## 1. Inherited State (measured 22:53)

- Wasm/hub mutex queues empty (`/tmp` wiped by reboot); no session-9 build alive. Peer semio-tech play
  runs `cargo rustc` demonstrator (pid 9910/12554) outside the mutex — left alone.
- Shared `wasm-dev` target: `semio_s_plugin_wfc.wasm` still 2026-09-22 23:37 (129383519 B).
- **GJ3 §10 "cargo did not uplift" — root cause:** GJ3's raw `cargo build` ran with a private
  `CARGO_TARGET_DIR` (`wp-gj3/target`, preamble rule 6). Cargo did uplift, into
  `.tmp-ticket/wp-gj3/target/wasm32-wasip2/wasm-dev/semio_s_plugin_wfc.wasm` (19:43, 129582308 B), because
  `target-dir` is the uplift root and only `build-dir` is shared. Consumers (describe's own hash,
  MCP freshness row, dev host) read the shared target. Not a cargo bug. The fix is to run product verbs
  (`<plugin>:describe`, `component-dev`, `materialize-dev`, `activate-s-react-dev`) with no target-dir override. That's
  what W1 does. The GJ3 hand uplift + `hashes.wasmSha256` patch get superseded by `describe` output.
- Session-9 hand-synced descriptors (norm, wfc) get superseded by `describe` (before/after hashes in
  `wp-w1/generated/descriptors-before.sha`).
- S14's claim that "activate staleness ignores framework changes" is stale: `component-dev` Nx inputs include
  `nativeSources` of every dependency project (incl. `@semio-tech/framework-plugin`).
- 60 components carry a `describe` target (34 plugins + 26 extensions).

## 2. Descriptors

Chain `wp-w1/w1-describe-all.sh` (one wasm-mutex hold per component, shared target), log
`wp-w1/generated/describe-chain.txt`, per-component `describe-<name>.txt`.

### 2.1 Root fix: described bytes == shipped bytes (landed 23:29 / 23:48)

Measured on wfc (`generated/component-probe-wfc.txt`, `generated/describe-then-component-wfc.txt`):

- `describe` built `cargo build -p <crate>` (rlib+cdylib unit, shared target), while `component-dev` built
  `cargo rustc --lib --crate-type cdylib` (captured into a private dir, then `dist/component-dev`). Those are two units
  and two compiles (wfc: 9 min + 4 min), and the result was two different wasm hashes. The committed descriptor
  said `ce48bc9f…` while the staged/served component was `a740304a…`.
- After switching describe to the cdylib unit, a split remained: `incremental` is in the unit's profile hash
  (build-dir `semio-s-plugin-wfc/{b8af…,7f55…}` differ only in `profile`), so callers with and without
  `CARGO_INCREMENTAL=0` still got two units.

Fix:
- `pluginComponentRustcArgs(package, profile)` in `📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`
  is the one argument list. `component-<profile>` and `buildPluginComponent` (describe) both use it. The
  `rootCdylib` switch is removed from `buildPluginComponent`/`describePluginComponent` (stdio's `true` dropped).
- `Cargo.toml` `[profile.wasm-dev] incremental = false` makes the unit independent of the caller's env.

The combined chain `wp-w1/w1-stage-all.sh` does, per component and in one wasm hold: `describe` → `materialize-dev`
(whose `component-dev` reuses the same unit). Log: `generated/stage-chain.txt`.

Concurrent-edit caveat: peers keep editing `🔌️plugin/🦀️.rs` (every guest links it), so guest hashes legitimately
move. The final proof is a verification pass after the chain.

### 2.2 Other fixes

- `🌎️hub/📦️packages/🦀️rust/📋️project.json` `trusted-catalog-bootstrap`: `"cache": true` → `false`. The target
  publishes into `OS_HUB_DATA` (side effect, `outputs: []`), so a cache replay would print success and publish
  nothing into a fresh data root.

## 4. Trusted Catalog

Script `wp-w1/w1-catalog.sh` (warm = `component-release` per package, one wasm hold each; hub = `cargo build
--bin os-hub` under the hub mutex; publish = `nx run os-hub:trusted-catalog-bootstrap --packages <list>`).
C7-first lane `wp-w1/w1-c7-first.sh` (stdio,gis,writer,draw,puzzle → `.🧬semio/🌐hub/w1-catalog-c7`), log
`generated/catalog-c7-chain.txt`.

- 01:05 machine load avg 276 (1-min), 211 (5-min): many peer rustc (os_hub, wasmtime, semio_*) alongside W1's single mutex-held build; W1 builds progress but slowly (draw release >10 min).

- ~01:08 every W1 detached process (stage chain 57638, C7 catalog lane, their mutex wrappers + nx/cargo children) vanished at once without END lines (release-draw.txt last write 01:08:0x; imperative-extension-control stage log empty). Not killed by W1. Cause unknown (external sweep or process-group teardown). Relaunched 01:23 with `wp-w1/w1-detach.py` (setsid, own session). Resume uses .ok markers.
- Launch law `📇️registry/🧪️tests/🚀️launch/🟦️.ts` (flagged by R1) now follows the new owners: `[profile.wasm-dev]`
  pinned as `{inherits:"dev","codegen-units":1,incremental:false}`, describe must call
  `pluginComponentRustcArgs(packageName, "wasm-dev")` exactly once, and native-orchestration owns the cdylib arg list.
  `bun ./📜️script.ts test` (plugin-registry) → **60 passed, 1 skipped (61)**, 8 files (`generated/registry-test.txt`, 02:4x).
- Launch entry `🚚️publish-catalog🗄️os-hub` (`bun nx run os-hub:trusted-catalog-bootstrap --packages all`) added to
  `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` (order 206.16015).

### 4.1 C7-first publish run 1: FAIL, root-caused and fixed (02:54)

`trusted-catalog-bootstrap --packages stdio,gis,writer,draw,puzzle` (51m54s) failed at writer's codec probe:
`codec.pack-schema-hash(writer.document) … throwaway artifact codec app close faulted: … "artifact store has no
owner-supplied bounded disposer"` (`generated/publish-w1-catalog-c7-run1-viewer-owner-fault.txt`).
Root cause: `ArtifactViewer` still defaulted `build_document_store_owners`/`build_config_store_owners` to `None`
while its disposers default to `Some(bounded_*)`. R1's 00:05 alignment covered `ArtifactApp` + `ArtifactEditor`
only. The codec resolution constructs and closes the viewer too, so every plugin whose viewer declares no owners
faults. Fix: `ArtifactViewer` owner defaults are now `Some(bounded_document_store_owners)` / `Some(bounded_config_store_owners)`
(`🔌️plugin/🦀️.rs`, framework, so every guest changes). C7 publish re-queued 03:0x.

### 2.3 Proof of the root fix (measured)

| component | committed `🔣️.json` wasmSha256 | shared wasm-dev | dist/component-dev | staged s module | time |
|---|---|---|---|---|---|
| note | 6cd7dcc5…5cb7 | = | = | = | 03:04 |
| wfc | e36b9d88…8d77 | = | = | = | 03:09 |

For both, the second cargo in the stage (`component-dev`) reported `Finished … in ~1 s` (unit reused), so there's one compile per
guest instead of two. note links `wasi:random/random@0.2.9` (H4). Requests g4/g5/h4 answered in `wp-w1/requests/`.

### 4.2 BLOCKER (root-caused): about 20 editor kinds have no pack-schema identity, so they cannot enter any trusted catalog

C7 publish run 2 (45m27s, `generated/publish-w1-catalog-c7-run2-writer-record-spec.txt`) got past the viewer fix and
then failed at writer's probe: `codec.pack-schema-hash(writer.document) … artifact codec schema has no structural
record specification`.

The chain:
- `plugin_artifact_pack_schema_hash` → `ArtifactApp::artifact_pack_schema_hash` → `<Snapshot as ArtifactPack>::record_spec()`
  → `os_pack::schema_hash(spec)`. `record_spec()` defaults to `None`, and only `#[derive(DslArtifact)]`/`DslRecord`
  overrides it.
- The composition refactor (composed `ArtifactChild<S>` / `ArtifactLink` slots) dropped `DslRecord`/`DslArtifact` from
  these snapshots, because "`ArtifactChild<S>`/`ArtifactLink` have no `dsl::DslField` impl" (the doc comments in each
  snapshot file say so). They got handcrafted `ArtifactPack` impls with no `record_spec`.
- The trusted bootstrap requires a nonzero `packSchemaHash` for every declared kind. The hub also pins it: the
  `SocketHelloV1.pack_schema_hash` must equal the descriptor's.

Snapshot kinds without a spec (source scan of hand-written `ArtifactPack` impls, excluding config/transient/presence/draft
and stdio's linked codecs): writer `WriterSnapshot`; puzzle `Puzzle{2d,3d,5d}PlaySnapshot`; cad `CadSnapshot`; lowpoly
`LowpolySnapshot`; dag `DagSnapshot`; sequence `SequenceSnapshot`; forms `FormsSnapshot`; mathematical `EquationSnapshot`;
animate `PresentationSnapshot`; imperative `ProcedureSnapshot`; reasoning `WiresSnapshot`; trinity `JackSnapshot`; raster
`RasterSnapshot`; process `Process3dSnapshot`; playbook `PlaybookSnapshot`; energy `EnergyModelSnapshot`; layout
`LayoutSnapshot`; norm `En1990Snapshot`, `Din18599Snapshot`. That's 18 of 34 packages.

Root fix (framework + about 20 plugin crates, pack formats change so fixtures must be re-authored):
- implement `dsl::DslField` for `ArtifactChild<S>` and `ArtifactLink` in the framework;
- restore `#[derive(DslArtifact)]` on these snapshots;
- delete the handcrafted codecs.

The alternative is to define pack-schema identity schema-first from each kind's committed snapshot JSON-Schema facet
(`SchemaVersion`) for every kind. That changes the linked stdio/gis identities on the hub side (H2's area).
It's a design decision across framework, hub and plugins, so it's out of W1's build/publish scope and gets routed
to the coordinator. W1 did not hand-write 20 guessed RecordSpecs; that would give fingerprints that don't
describe the pack bytes.

Consequence: the full catalog is limited to packages whose kinds all answer. That's stdio, gis (linked), and every plugin
not listed above (source says: architect, block, demonstrator, draw, fem, flow, note, procedural, remodel,
shooting, sourcing, space, vcs, wfc), each still to be proven by the probe.

## 5. Plan after coordinator decisions (06:07)

- p4 restored `record_spec` in 21 plugin crates. Process3d is still out.
- p5 is making `schema_hash` structural and recursive, which changes every descriptor and packSchemaHash.
  The full 60-component describe/stage/generate/check/activate pass plus catalog B waits for p5's "landed" message.
  The stage chain stays paused (9/60 + gis, draw, note, wfc restaged).
- Live-proof work until then: C8 gis+draw stage → `activate s react dev` → catalog A
  (`stdio,gis,note,draw,writer,puzzle` → `.🧬semio/🌐hub/w1-catalog-a`) → hub 7800. G6: wfc restage.

### 4.3 Peer compile/assembly breaks met in the chains (not W1's, routed)

- 06:08 stdio-semio: `semio_s_artifact_stdio_png` used without the dep. A peer fixed it at 06:12.
- 06:22 kernel store `pub use crate::mounted_pack_session` (E0432) broke os-hub/gis/draw. A peer fixed it at 06:30 (`super::`).
- 07:01 gis describe: `plugin assembly failed — no declared composer capability owns the runtime claims`. The gis io edit
  (06:17–06:19) added `EXPORT_TXT` composer entries to gismap + gisterrain but no `composer.txt` capability, and both would
  claim `s.stdio.txt@utf-8/*`. Routed via C8 to the coordinator; the owner is unknown.

### 4.4 Catalog A (stdio,gis,note,draw,writer,puzzle), 08:29–08:52

- 08:29 gis (T3 io + C8 fixes + p5 hash) and draw staged; all four hashes agree for each. gis2d + s activated (C8, T3 answered).
- 08:37 publish run 2 died at the wasm-release build: a peer's transient edit (08:35) enabled `os_pack::cli` on wasi
  while `pack::io` is still native-only (`FilePackSource` E0433). The peer reverted it at 08:39. The final pass that
  auto-started then also hit the same transient kernel break (`final-1-activate-run1-peer-kernel-break.txt`).
- 08:5x relaunched: catalog A publish, then `w1-final-pass.sh` (log `generated/final-pass.txt`). Also fixed a sed
  delimiter bug in the RELEASE project list.
- 09:46 catalog A run 3: stdio, gis, note, draw and writer passed the codec probe. **puzzle** failed with `codec.pack-schema-hash(puzzle.2d) … artifact codec
  schema is owned by no app`. Root cause: `artifact_kind()` in `🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs` hard-codes the kind
  schema literal `"puzzle.2d"`, while the apps' `DOCUMENT_SCHEMA` (and the 181 committed documents) are
  `"puzzle.2d.fixture"`. Fix: all three puzzle `ArtifactKindSpec.schema` now come from their `PUZZLE_{2D,3D,5D}_SCHEMA`
  constants (one source). Run 4 queued at 09:58 (`generated/catalog-a.txt`).
- 11:45 catalog A run 4 was SIGKILLed (rc=137, 6649 s) in the same external sweep that killed C8's hub and serves. Its packages had all reached `complete`. Relaunched 11:5x as catalog A, then the final pass (`w1-a-then-final.sh`).
- 12:07 the catalog A waiter (queued behind h4) was SIGKILLed while still waiting in the FIFO (empty log, rc=137), at the
  moment h4 released the lock. External again. The final pass then took the lock, and activate-s attempt 1 failed with a **real race**:
  `buildCargoArtifacts` copied every dependency rlib that cargo reported
  (`…/build/semio-framework-ui-viewport/9e66…/out/lib…rlib` ENOENT) while concurrent builders on the shared build-dir
  replaced it, even for cdylib component builds that never use those copies. Fix
  (`📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`): dependency paths are only recorded during the build and
  copied after success, and only when the primary artifact is a library. Component builds no longer touch other units' outputs.
  Catalog A re-queued 12:2x.
- 12:20 final pass `activate-s-react-dev` retry 2 → **rc=0** (760 s). `w1-verify-staged.ts`: the s receipt lists all 60, and
  **dist == staged for 60/60** (`generated/verify-after-activate.txt`). Committed descriptors are still the old ones until describe-all.
- 12:35 SIGKILL again (rc=137) for both catalog A (mid stdio wasm-release build) and describe-all. External.
  Catalog A now runs as warm (one hold per package) then publish, with up to 3 retries (`w1-catalog-a.sh`). Reported to the coordinator.

### 4.5 External sweep log (for the coordinator)

| time | what died / vanished (measured) |
|---|---|
| ~01:08 | every W1 detached process (stage chain, C7 lane, mutex wrappers, nx/cargo children); no END lines |
| 11:45:02 | catalog A run 4 (`trusted-catalog-bootstrap`, rc=137 after 6649 s); C8's hub 7820, serves 6380/6381 and wait loops |
| 12:07:27 | catalog A waiter still queued in the fleet FIFO (rc=137, empty log) |
| 12:35:12 | catalog A run 5 (mid stdio wasm-release cargo, log stops 12:28) + final-pass describe-all (rc=137) |
| ≤12:43 | **the shared cargo cache was deleted**: `.🧬semio/🦑️repo/⚡️cache/cargo/target` is gone entirely, and `cargo/build` only has `debug/` (recreated 12:43:13 by a live cargo). `wasm32-wasip2/{wasm-dev,wasm-release}`, `wasm-dev`, `wasm-release`, `tmp`, `wasm32-unknown-unknown` are gone. Disk free went from 45 GiB (22:5x) to **504 GiB**. Staged `dist/` modules, activation lanes and ticket `generated/` survived. A 12:44 stdio release build then failed with `extern location … does not exist` (deleted under it). |

Reading: this is a disk-space cleanup of the cargo cache (the disk was at 96 %). It kills the processes holding those files.
Consequences: every guest now rebuilds cold. The shared `wasm-dev` files the MCP freshness rows read are gone until
describe-all re-creates them. The s react dev lane itself is intact (60/60 activated, dist == staged).
