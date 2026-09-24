# WP-W2 — Canonical Hub 7800, All-Package Trusted Catalog, Final Rebuild

Session 11 slice W2 (sole all-plugin wasm owner). Ports: canonical hub 7800, hubs 8000–8009, serves 6500–6509.
Captures `wp-w2/generated/`. Scripts `wp-w2/*.ts|sh`.

## Hub Handoff

**CATALOG B, READY since 2026-09-25 12:45:06** (restart after the 12:29 sweep deleted the previous data root; boot 3 min 46 s). This is the post-landing tree: H9 replay-envelopes ABI, the Rust open-target rule, lazy on-disk
retention and compressed closed actors. Measured `/readyz` (`wp-w2/generated/readyz-7800-b.json`): `status: ready`, `artifactAuthority.ready`,
`features.openPlan`, `openPlanExchange`, `rebootstrap`, `mcpWorkspace`, `inferenceServices [s.gis.gismap.inference]`, `publicSessionIssuance: true`.
Open-plan probe (`wp-w2/w2-open-plan-probe.ts`, `generated/open-plan-probe-7800-b.txt`): the creation catalog lists **12 creatable kinds**, and
**12/12** pass server-owned creation → Ready → `POST …/open-plan` 200 with an editor surface and a closed browser actor: 2d.block, 3d.block,
5d.block, 2d.drawing, 2d.puzzle, 3d.puzzle, 5d.puzzle, 3d.wfcgrid3d, animate.presentation, s.gis.gismap, s.note.note, text.document.

| what | value |
|---|---|
| URL | `http://127.0.0.1:7800` (loopback, development) |
| binary | `.tmp-ticket/wp-w2/generated/bin/os-hub-w2-hub-7800-b`, a copy of the bootstrap build that validated and published catalog B (`⚡️cache/cargo/target/debug/os-hub` 11:14). Source sha256 `1a5cf10cff76bd71694f6a3f1ea3060d3c6c264f29ff6ea457405508a053b5c5`, signed copy `bf4b4293df8cdef09052fd868672029021ca37bc2a660f90e982348bf3db7da0` |
| data root (`OS_HUB_DATA`) | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-hub-7800-b` (canonical hub-data home, gitignored, outside the ticket `generated` sweep, `0700`, only this hub) |
| catalog | a real copy of `.🧬semio/🌐hub/w2-catalog-b` current generation: profile `local-stdio-gis-note-animate-block-writer-draw-puzzle-wfc-open-v1`, generation `e8167ce8ed3e61f0a4252661df9f2e3043aabb258e474b797fc3f0612c9ecc53`, bundle sha256 `6328ca516fabe57b180d4dadedbbddc7b88dd9c3257f520f58109bb1d3fffdc6`. Packages: **stdio, gis, note, animate, block, writer, draw, puzzle, wfc**, all wasm-RELEASE components. 12 open targets. Closed actors are 5.2–16.8 MB (gis 16 848 791 B, was 66 987 401 B) |
| pids | hold `22204` (bun `wp-w2/w2-hub-hold.ts`), os-hub `22208`, `wp-w2/generated/state-7800/pids.txt`, runId `345ceda4a4eec6da76d3f53ccc5a920f` |
| state/logs | `wp-w2/generated/state-7800/{hold.txt,status.txt,capture.txt,ready.json,admin-capability.json}` |

**Human credentials** (provisioned before boot with `os-hub credential set`, same as before): `user1@semio.dev` / `gm1-local-dev-pass-1`,
`user2@semio.dev` / `gm1-local-dev-pass-2`. This is a **fresh data root again (12:41)**: user1 `01a0d827-3315-7ec2-a938-98cd4b2106e0`, user2 `01a0d827-4c07-74d4-8939-68f10c4adb98`.
Everything created on 7800 between 11:50 and 12:29 was lost with the swept root. Catalog-A documents do not carry over, because their owner package hashes are the old guests. HTTP sign-in:
`POST /auth/sessions` `{"schema":"semio.hub.auth.credential-sign-in/v1","email":…,"password":…,"deviceInstanceId":"<32 chars>","clientClass":"browser"}`.

**Agent credential:** a signed-in human `POST /auth/agent-delegations` `{"schema":"semio.hub.auth.agent-delegation-create/v1","spaceId":…,"agentLabel":…,"audience":"edit","ttlSecs":900}`
→ `{delegationId, agentPrincipalId, token}`. Write it `0600` as `{"schema":"semio.hub.agent-credential/v1","hubOrigin":"http://127.0.0.1:7800","spaceId":…,"audience":"edit","token":…}`,
then run the semio MCP as `stdio --hub http://127.0.0.1:7800 --space <id> --credential-file <file> --scopes workspace.read,artifact.write`. Revoke with `DELETE /auth/agent-delegations/{id}`.

**Admin:** `wp-w2/generated/state-7800/admin-capability.json` (`0600`) holds one `admin-relay` session (15 min, issued at readiness). For a fresh one, run
`touch wp-w2/generated/state-7800/admin-request`; the hold issues within 5 s. Issuance is on demand because **the hub admits at most 64 local-bootstrap pipe
exchanges per run and exits on the 65th** (`LOCAL_BOOTSTRAP_REPLAY_MAX`, a non-evicting replay set). Measured: the catalog-A hub died at 11:41 with
`Directory(Unauthorized)` after 63 periodic re-issues. Routed to H9 as a hub defect.

**Restart (W2 only):** `zsh wp-w2/w2-restart-7800.sh <catalog data root> <os-hub> <new data root name> <hold pid>`. It stops the hold, waits for the hub pid
(and kills it if it is still loading a catalog: measured, a loading hub does not notice pipe EOF), copies the catalog, provisions the users and starts the hold under `generated/`.
**Next:** the full `--packages all` catalog (25 remaining packages are compiling since 11:44, 3 at a time), then restart onto it.

## Status

| # | Item | Status |
|---|---|---|
| 1 | Canonical hub 7800 on catalog A + two human credentials + agent recipe | **DONE** 01:02 (Hub Handoff above; coordinator messaged) |
| 2 | All-package catalog blockers (derive-path, codec probes, descriptor bounds, bootstrap verbs) + request triage | Blockers found (§2); fixes IN PROGRESS per coordinator decisions 01:2x |
| 3 | Final rebuild (describe-all → generate → check → restage s → full catalog → hub 7800 restart → second hub) | describe-all 60/60, generate + check green, activate s + **verify 60/60** (05:57); **catalog B published + hub 7800 on it (11:50), open plans 12/12**; rest-warm (25) RUNNING → `--packages all` → restart → second hub |
| 4 | Requests (`wp-w1/requests/`, `wp-w2/requests/`) | PENDING |

## Log

- 00:43 copied the run-10 os-hub, made a fresh data root with a catalog A copy, and provisioned two users with `credential set`. First boot (hold 58407) was ready at 00:51:07 (6.5 min).
  Restarted at 00:57 to add the admin-relay issuance (hold kill → hub exit in 2 s). Second boot was ready at 01:01:41 (4.5 min), admin API 200, sign-in 200 ×2.
  `OS_HUB_ADMIN_TOKEN` (C7's hold) is a no-op: no Rust code reads it. Admin access comes only from an admin-subject profile plus a pipe-issued `admin-relay` session.

## 2. All-Package Catalog Blockers (measured 01:05–01:20)

Every package is selectable (`TRUSTED_BOOTSTRAP_ALL_PACKAGES`, 34 plugins; extensions are not catalog packages: the bundle role
`Extension` exists in the schema but no extension is a standalone document owner). No full catalog was ever published (TC5's
`--packages all` never finished), so these bounds and rules were never exercised beyond 6 packages.

1. **Open targets: the bootstrap's TS re-implementation is stricter than the hub's Rust rule.** `trustedBootstrapDescriptorOpenTargetsV1`
   only pairs a kind with an editor whose `dialect.artifactKind === kind.id`. Only gismap, note and one demonstrator kind satisfy that
   (committed-descriptor census: 89 kinds, **3 openable**). Measured in catalog A's bundle: **2 open targets** (gismap, note).
   draw/writer/puzzle are codec-only, with no open plan and no closed browser actor, so writer/draw/puzzle collaboration over the hub
   cannot work on catalog A. The Rust `validate_descriptor_open_target` accepts a kind the app declares itself. **Decision (coordinator):** one rule, Rust is
   the authority, and the TS twin is deleted.
2. **vcs kind id drift.** `ArtifactKindSpec.id = VCS_DOCUMENT_SCHEMA = "vcs.vcs"`, but the hub-linked native receipt
   (`stdio+gis+vcs/native-codecs/v1`) binds `s.vcs.vcs`. The loader would refuse vcs ("binding outside its exact declared package
   closure"), and vcs has no open target. **Decision:** one id, `s.vcs.vcs`, fixed at the source.
3. **Hub closure bounds cannot hold 34 packages.** Estimates from catalog A's release/dev ratio (0.21–0.24×) over the current wasm-dev sizes:
   the 34 release components total ≈ **791 MB**, against `TRUSTED_COMPONENT_CLOSURE_MAX_BYTES` = 512 MiB. Closed actors are ≈1.34× their component
   (gis 66,987,401 B for a 49.9 MB component, note 20.4 MB for 14.9 MB), so the actor total is ≈ 1 GB against `TRUSTED_BROWSER_ACTOR_CLOSURE_MAX_BYTES` = 128 MiB.
   gis's actor is only 121 KB under the per-actor `DOCUMENT_BROWSER_ACTOR_MAX_BYTES` (64 MiB). The loader also holds every component
   and actor in memory (`Arc<[u8]>`) plus each compiled guest (`OnceCell`). **Decision:** make the limits schema-declared hub configuration with per-component
   and per-actor bounds. The catalog total is bounded by storage, and components load and instantiate lazily on first open. Measure hub RSS before and after.
4. **Codec probe** (`w2-codec-probe.ts`, the bootstrap's `semio-framework-plugin-describe codecs` call on the current wasm-dev components):
   vcs `vcs.vcs` PASS 147 s, note PASS 30 s (`generated/codec-probe-vcs-note.txt`). A full per-kind sweep is folded into the rebuild
   (fresh components).
5. stdio: its 26 kinds are plugin-level `stdio.<x>`, and its editors' dialects are `s.stdio.<x>`. There is no declared pairing, so stdio stays codec-only
   (consistent with the hub's own `local-stdio-gis-open-v1` fence). This is recorded, not changed.

### 2.1 Fixes landed (source, compile-atomic, measured)

- **vcs one id** (`s.vcs.vcs`): `ArtifactKindSpec.id = VCS_DIALECT.artifact_kind`. The `VcsNativeOpenableIdentityAuthority` schema const and the fixture
  moved to `s.vcs.vcs`. Laws: `native_openable_identity` 1/1 and `native_codecs` 2/2 (`generated/vcs-kind-laws-1.txt`). TS oracles:
  identity `positive=1 hostile-denied=11`, codec `receipts=1 hostile=9`. The descriptor, registry and dev modules regenerate in the rebuild (vcs is staged last).
- **Open targets: one Rust rule.** `app_opens_kind` + `descriptor_open_targets` + `descriptor_open_targets_answer` live in
  `🔏️trusted-catalog/🦀️.rs`, and `validate_descriptor_open_target` uses the same predicate. An editor or viewer app opens a kind it declares itself,
  or a plugin-level kind its own dialect names (this closes the old loophole where a sibling dialect could open a plugin-level kind). The new hub verb is
  `os-hub trusted-catalog open-targets` (descriptor on stdin). The answer is the schema-first `TrustedCatalogDescriptorOpenTargetsV1` / `TrustedDescriptorOpenTargetV1`
  (`🧬️schema/🔣️.json` + Rust projection, exports 17 → 19). The bootstrap's TS rule (`trustedBootstrapDescriptorOpenTargetsV1`) is **deleted**:
  `trustedBootstrapHubOpenTargetsV1` asks the validating binary, and `materializeTrustedCatalogBundle(…, hubBinary)` got 4 callers
  re-ordered so the binary exists first. The language-agnostic fixture `🧫️fixtures/🎯️descriptor-open-targets/🔣️.json` (4 cases) drives the law
  `descriptor_open_targets_follow_the_one_pairing_rule_and_validate_as_published` (every answered target validates).
- **Lazy retention (no closure in memory).** The closure totals are deleted (`TRUSTED_{COMPONENT,DESCRIPTOR,BROWSER_ACTOR}_CLOSURE_MAX_BYTES`).
  The per-file bounds are now the schema-declared execution-target bounds (`TRUSTED_COMPONENT_MAX_BYTES = DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`,
  `TRUSTED_DESCRIPTOR_MAX_BYTES = DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES`, actor = `DOCUMENT_BROWSER_ACTOR_MAX_BYTES`, all in the directory
  JSON schema that the browser enforces too). Components and closed actors are `TrustedCatalogAsset`s: a retained file (opened generation root + path + verified
  digests), reread and re-verified on use, resident only while a reader holds it (`Weak`). Load verification hashes and drops the bytes. A guest-codec
  package is compiled once to pin its fingerprints and that compile is dropped. Runtime codec ops compile lazily (`OnceCell`) on first use. The
  execution-target route reads assets on demand (`document_execution_target_asset_bytes`). A tampered backing file is refused, never served
  (law renamed `…_refuses_tampered_retained_bytes`).
- Checks: `cargo check -p semio-hub --lib --bin os-hub --tests` has no error of mine. The one remaining error is a peer's in-progress bin-unit edit
  (`gis_map_test_snapshot` is cfg(all(sqlite,integration-fixtures)) but is used under cfg(any(integration-fixtures,native-artifact-execution)), `🧪️tests/🔬️bin-unit/🦀️.rs:1380`,
  file mtime 02:21), which I left to its owner. `cargo test -p semio-hub --lib -- trusted_catalog`: **33/33 serial** (`test-hub-trusted-catalog-2-serial.txt`).
  In parallel 32/33: `linked_stdio_gis_…_partial_codec_closure` observes a codec another concurrent test registered in the process-wide registry (inter-test
  race, passes serially). Hub script tsc: 0 new errors (the 2 remaining are pre-existing: leb128 typings, ReadableStream in `🕸️wasm/📜️script.ts`).
- Baseline footprint before lazy retention: hub 7800, catalog A, old binary = **578 MB** (`footprint`, Malloc Large 304 MB ≈ retained components + actors,
  Malloc Small 261 MB ≈ compiled guests; `generated/footprint-7800-catalog-a-old-binary.txt`). The "after" numbers come from the rebuilt hub.

### 2.2 Request triage (`wp-w1/requests/`)

| file | status |
|---|---|
| c7, c8 | still needed: writer/draw/puzzle open targets + actors → served by the full catalog (open-target fix above) |
| g10 (g9 superseded by it) | still needed: wfc describe/restage + catalog with stdio,gis,note,wfc from the post-landing tree → full catalog |
| g4, g5, g6, g7, h4 | superseded: their guest changes are in the tree; the full rebuild re-describes/restages wfc and note |
| h7 | now: catalog A root + `wp-w2/bin/os-hub` are one tree. After the rebuild: the new catalog + new binary |
| h8 | request 1 landed as H9's landing row and is in this rebuild; request 2 (catalog + hub) is answered after publish |
| m5b, p6, r1, t3, t7, t9 | superseded by describe-all (every listed crate is re-described from the current tree) |
| s3 | still needed: 15 deleted `core.wasm` → the restage of all 60 + activate |

## 3. Final One-Tree Rebuild (running)

- 01:19 stage chain free-8 (pid 87800) and 01:21 stage chain 52 (pid 89335): `w2-stage-all.sh`, one wasm hold per component (describe →
  materialize-dev). Order: note, gis, draw, writer, puzzle, stdio, the rest alphabetically, vcs last. Logs `generated/stage-chain-{free,52}.txt`, per component `stage-<n>.txt`.
- 01:55–02:52 **stall**: my first describe cargo (18087) and a peer cargo (14955, under an orphaned `dev mcp stdio os`) sat in `prebuild_lock_exclusive → flock`
  with no rustc child (sample). I killed only mine; the coordinator killed the orphan. The 03:02 retry of cad-extension-aec-building-energy passed.
- 03:35 state: 10/60 staged (3 cad ext, 2 imperative ext, note, gis, draw, writer, puzzle); stdio running.
- 03:35 `w2-after-stage.sh` (pid 81273, detached) waits for both chains, re-stages puzzle (S15's `[DEBUG]` strip in the puzzle3d editor landed after
  its stage: `cargo check -p semio-s-artifact-puzzle-3d` rc=0, `check-puzzle3d-debug-strip.txt`), refuses to go on if any component is unstaged, then runs
  `w2-final.sh`: generate → check → activate-s-react-dev → `w2-verify-staged.ts` (committed == dist == shared == staged, receipt lists all, every
  plugin module's `.nx-artifact.json` files exist incl. `*_component.core.wasm`) → component-release ×34 (one hold each) → os-hub build (hub mutex) →
  `trusted-catalog-bootstrap --packages all` → `.🧬semio/🌐hub/w2-catalog-all`. Log `generated/after-stage.txt`.
- Pre-rebuild verify (current staging, 03:3x): 45/60 consistent, 15 diverged (expected before describe), no missing staged file.
- 03:50 two more open-target root fixes found by running the new verb over every committed descriptor
  (`generated/open-targets-committed-descriptors.txt`): **architect** and **energy** editors never stitched their own `ArtifactKindSpec`
  (`.artifact_kind(crate::artifact_kind())` missing), so they declared no kind and had no open target. Fixed at the source
  (`cargo check -p semio-s-artifact-architect-program -p semio-s-artifact-energy-model` rc=0). Both were staged after the fix (04:20, 04:33).
  Their fresh descriptors answer `data.program@s.architect.program@1/*#editor` and `data.model@s.energy.model@1/*#editor`.
  vcs answers `s.vcs.vcs`; draw/writer/puzzle answer 1/1/3 editor targets. Every one of the 34 packages except stdio now has at least one open target.
- 05:22 both stage chains DONE: **59/60 rc=0** at the first attempt, plus cad-extension-aec-building-energy on its retry (lock-cycle stall above).
  The puzzle re-stage (DEBUG strip) is running, then `w2-final.sh` starts by itself.
- Private test binary `wp-w2/target/debug/os-hub` (03:48, with all hub fixes) is used only for the open-target census. The catalog's binary is
  the bootstrap's own build (shared target).
- 05:32 **`plugin-registry:generate` rc=0** (231 s), 05:35 **`plugin-registry:check` rc=0** (164 s): `generated/final-1-{generate,check}.txt`. Then activate-s-react-dev started.
- **One registered command for the chain (build-health P2-8):** `bun nx run @semio-tech/plugin-registry:rebuild-all [--from <step>]`,
  launch row `🔁️rebuild-all🔌️plugin-registry` (seed + launch.json, 4_build 206.16016). The chain is declared as data in `📇️registry/🔁️rebuild/🔣️.json`
  (describe-all → generate → check → activate-s-react-dev → trusted-catalog-bootstrap `--packages all`, stages in dependency order) and runs
  through `RebuildAllScript` (`🔁️rebuild/🟦️.ts`), with a numbered progress line per step and `--from` to resume. Law `🧪️tests/🔁️rebuild`: 2/2 plus the launch law
  10/10 (`generated/registry-test-rebuild.txt`). The ticket run uses the same product verbs under the fleet mutex. The single command was **not
  run end to end** (it would duplicate this rebuild).
- 05:36 activate-s-react-dev rc=0 (63 s). Verify: **59/60 consistent**, receipt lists 60, no missing staged file. **stdio diverged**: the committed descriptor and shared wasm-dev
  (describe cargo 03:32) differ from dist/component-dev and staged (component-dev 04:01, same hold). A peer edit to stdio-linked source landed inside
  stdio's 37-min stage hold (`final-2-verify-attempt1-stdio-diverged.txt`). 05:4x re-staged stdio, then `w2-final.sh 1` (generate → check → activate → verify → …).
- 05:43 stdio re-staged (315 s); 05:44 generate rc=0, 05:48 check rc=0, 05:49 activate rc=0. Verify **59/60**: stdio is now consistent, but **note diverged**:
  its dist/component-dev was rebuilt at **05:39:16 by someone else**, inside my stdio hold and outside the wasm mutex (likely a peer's note activation or serve;
  WG7 asked for note wgpu), from a newer tree than the 03:04 describe. 05:5x re-staged note, then the final chain resumes from generate.
- 05:52 note re-staged (101 s) → 05:52 generate rc=0 → 05:56 check rc=0 → 05:57 activate-s rc=0 → **verify rc=0: 60/60 consistent** (committed descriptor
  `wasmSha256` == dist/component-dev == shared wasm-dev == staged module, s receipt lists 60, every plugin module's files incl. `*_component.core.wasm`
  present; `generated/final-2-verify.txt`). 05:57 component-release ×34 started (one hold each), then os-hub build, then publish `--packages all`.
- 06:2x releases run 10–20 min each cold at load ~39 (stdio 662 s, gis > 20 min), so the full catalog is hours away. At 06:30 I started **interim catalog B**
  (`w2-catalog-b.sh`, pid in `generated/catalog-b.pid`): it warms note, draw, writer, puzzle, wfc, block first (one hold each, sharing `.ok` markers with the final
  loop), then builds os-hub (hub mutex) and publishes `stdio,gis,note,draw,writer,puzzle,wfc,block` → `.🧬semio/🌐hub/w2-catalog-b`. Hub 7800 moves to B at once,
  because every current-tree client needs post-H9 guests (C10 writer/draw/puzzle, G10 wfc, WG8 block, WG7 note). Then the full catalog.
- **Registry race (H9's 372/375, 2 trusted-catalog laws): root-fixed.** The process-wide codec registry is monotonic: real linked stdio/GIS/VCS schemas
  stay registered for the process lifetime. `gis_native_provider_selection_…` asserted `is_none()` on real GIS schemas while `long::gis_map_binding_…`
  registers them, and `long::linked_stdio_gis_…` / provider `linked_consumer_descriptors_…` snapshot before/after around loads that a concurrent law
  could change. Fix: one test-only guard `artifact_authority::REAL_LINKED_CODEC_REGISTRY` (tokio mutex) held by every in-process law that registers
  or observes the real schemas. The selection law now states its real claim, that preview publishes nothing (registry unchanged across every case),
  instead of assuming a fresh process. `cargo test -p semio-hub --lib -- trusted_catalog native_openable_provider`: **39/39 in parallel ×3**
  (`generated/test-hub-registry-guard-parallel.txt`).
- 07:2x **plan change (coordinator):** releases now run 3 at a time for distinct packages inside ONE wasm hold per batch (`w2-release-par.sh`,
  `nx run-many -t component-release --parallel=3`, unchanged wasm-release profile), in this order: batch B = block, writer, draw, puzzle, wfc (stdio, gis,
  note, animate already warm) → catalog **B** `stdio,gis,note,animate,block,writer,draw,puzzle,wfc` → restart 7800 → batch rest (dag, raster first) →
  `--packages all`. The old serial loops were stopped while they were only queued in the mutex (no cargo running; I killed my own two wrappers).
  Log `generated/release-par.txt`. The 7800 restart moves the data root, binary copy and hold state under `wp-w2/generated/` and deletes the old tracked
  `hub-7800`/`state-7800`/`bin` (`w2-restart-7800.sh`).
- 07:22–08:18 batch B released 3-parallel in one hold (3342 s, rc=0; block, writer, draw, puzzle, wfc). 08:20 os-hub built. **08:34 publish B failed**:
  `browser actor artifact: closed byte bound`. gis's closed actor over 50 299 594 core bytes exceeded 64 MiB, because the actor embedded its core modules as
  **base64 text** (1.34× the core, gis in catalog A was already 66 987 401 B, 121 KB under the bound).
- **Actor root fix (10:2x):** the closed actor now embeds each core **deflate-raw compressed** (`node:zlib` `deflateRawSync` level 9 at build,
  deterministic) and inflates it with the browser's native `DecompressionStream("deflate-raw")` before `WebAssembly.compile`, with length checks,
  per-chunk progress and cancellation. The actor stays closed (no fetch, no external resource). Measured: gis release 50 043 411 → **12 174 315** B (0.243), so
  its actor is ≈16 MB instead of ≈67 MB, 4× less hub disk and browser transfer. The per-actor bound stays the ONE schema-declared authority
  `DOCUMENT_BROWSER_ACTOR_MAX_BYTES` (64 MiB, directory schema + Rust + TS twins). The bundler's own literal `64 * 1024 * 1024` is deleted and it imports
  the schema constant. I briefly raised the kernel constant to 96 MiB and reverted it byte-identically: the kernel is linked by every guest, so with checksum
  freshness any content change there would recompile every warm release guest (hours), and compression removes the need.
- Also fixed (peer drift from H4's `wasi:random/random@0.2.9`): browser-bundle schema `importInterfaces` `maxItems` 18 → 19 (×3) and the codegen-manifest
  pattern admits `wasi:random/random`. Laws: `closed-browser-component-factory-check` **all green** (compiler capsule, sources, policy, WASI 17, host 22,
  actor factory 10 + 20 artifact laws, component factory: native-Wasm oracle, 2 actors, 11 hostile, 3 cancellation; runs under node with the real
  `DecompressionStream`; `generated/factory-law-3.txt`).
- 10:29 publish B relaunched (`w2-release-par.sh b-publish`, then rest-warm, then `--packages all`).
- 10:39 publish B attempt 2 failed at block's codec probe: `codec.pack-schema-hash(kit.catalog) … artifact codec schema is owned by no app of this bundle`.
  block's editors DECLARE `kit.catalog` (an input they read) without owning its codec. Other declared-but-foreign kinds show the same pattern: sourcing/demonstrator `kit.catalog`,
  `catalogue.kinds`; raster/shooting `2d.image`. **Fix (native emitter, no guest change):** `semio-framework-plugin-describe codecs` answers such a kind
  as `unowned` (`UNOWNED_ARTIFACT_CODEC_SCHEMA`, the plugin crate's own fault text, with a law that keeps them in step) instead of failing. The component-codec-rows
  document gains `unowned: [{artifactKind, artifactSchema}]`. The bootstrap takes codec rows = owned kinds only and drops hub-answered targets of
  unowned kinds, because a target needs a codec of its own package (the hub's `validate_bundle` rule). Measured on the release components: block 3 rows + 1
  unowned (`kit.catalog`); note 1, animate 1, writer 1, draw 1, puzzle 3, wfc 1, all owned. `cargo check -p semio-framework-plugin-describe --lib --bins --tests` rc=0.
- 10:4x publish B attempt 3 launched.
- 11:11 publish B attempt 3: build, codec probes, 8 closed actors (compressed, e.g. puzzle 8.9 MB) and the candidate hub all passed (**candidate ready**). The GIS probe creation
  then got an empty-body refusal, because the gis descriptor now answered **two editor targets for `s.gis.gismap`**: the terrain editor lists the map kind as an input,
  and the hub's creation selection requires exactly one writable target per kind. **Rule refinement (Rust, one place):** a plugin-level kind is opened only by the app
  whose dialect names it, even when a sibling app lists it. Any other kind is opened by the app that declares it. The verb answers **editor** surfaces only
  (read-only viewer targets would be a feature change of the catalog and of the hub's `local-stdio-gis-open-v1` fence, so I routed that to the coordinator). Fixture 6 cases
  (+2: plugin-and-editor, sibling dialect). Laws `trusted_catalog` + `native_openable_provider` **46/46 parallel**. Re-census: gis 1 target (was 3), writer 1,
  block 3 owned + 3 `kit.catalog` (dropped as unowned at publish). 11:2x publish B attempt 4 launched.
- 11:44 **catalog B published** (attempt 4, 1774 s): generation `e8167ce8…`, 9 packages, 12 open targets, compressed actors 5.2–16.8 MB. 11:4x hub 7800 restarted on
  it (the catalog-A hub had already died at 11:41, see the Hub Handoff admin note). Two hubs briefly overlapped on the new root at 11:47: my restart
  killed the hold while the first hub was still loading its catalog, and a loading hub ignores pipe EOF. Both were stopped (only mine) and one was relaunched at 11:47:52.
  `w2-restart-7800.sh` now waits for and kills the old hub pid. Ready 11:50:32. Open plans 12/12 (`generated/open-plan-probe-7800-b.txt`).
- **Hub memory (lazy retention), measured with `footprint`:** catalog A (6 packages, old eager binary) idle = **578 MB**. Catalog B (9 packages, lazy) right after
  ready = **346 MB** (`generated/footprint-7800-catalog-b-lazy.txt`). After 12 creations, which compiled all 7 guest packages, = **1002 MB**
  (`…-after-12-creations.txt`): compiled guests stay cached per package (`OnceCell`) once used. Component and actor bytes are no longer resident. An
  idle-release of compiled guests is the next bound (follow-up, not done).
- Old tracked `wp-w2/hub-7800`, `wp-w2/state-7800` and `wp-w2/bin` were deleted (coordinator hygiene). Everything is under `wp-w2/generated/` now.
- 11:44 rest-warm started: 25 packages, 3-parallel, one hold (`release-par-rest-1.txt`), then `--packages all` → `.🧬semio/🌐hub/w2-catalog-all`.
- 12:16 and 12:24 rest-warm attempts 1–2 were SIGKILLed (rc=137, no compiler error). The disk was at 96 % and an external prune deleted build-dir rlibs under running
  builds (measured as `clang: no such file … libsemio_framework_job-….rlib` in my own hub test link). The coordinator freed it to 137 GiB. 12:4x rest-warm relaunched.
- **Viewer targets (coordinator decision 12:1x):** the single Rust rule now also answers viewer surfaces. A viewer opens the plugin-level kinds its dialect names
  and the kinds its dialect's editor declares, with grant read + observe and no write. The `local-stdio-gis-open-v1` fence is now exactly the map editor
  (`gis2d-main`) plus the map viewer (`gis2d-view-map`). The TS gates that took `openTargets[0]` select the editor role. Fixture expectations are editor + viewer; the linked profile
  law is renamed `…_one_map_editor_and_viewer` and gains a writable-viewer rejection. `cargo check -p semio-hub --lib --bin os-hub --tests` rc=0. Laws
  `trusted_catalog` + `native_openable_provider` **46/46**. The synthetic TS fixture proof `🧫️fixtures/🧬️stdio-gis-bootstrap` still describes the single-editor
  profile (a self-contained digest chain); regenerating it is a follow-up, recorded and not done.
- S15's plugin-module bundles (schema v3) were already part of catalog B (`plugin-modules/` 62 blobs, 9 `plugin-module.json`) and stay in `--packages all`.
- **12:19–12:35 external low-disk sweep** (coordinator's diagnosis: the same 96 % cleanup as on 09-24) killed my rest-warm (rc=137 ×2) and **deleted `wp-w2/generated/` and
  `wp-w1/generated/`**, including hub 7800's data root, state and binary copy and every capture this report cites before 12:30. Hub 7800 died. 12:41 I restarted
  it on the same catalog B + binary with its data root at `.🧬semio/🌐hub/w2-hub-7800-b` (outside the sweep). **Ready 12:45:06.** The `generated/…` paths cited above for
  earlier measurements no longer exist. The numbers in this report were read from them before the sweep. Chain logs now live in `.🧬semio/🌐hub/w2-logs/`.
  The binary copy and hold state are still under `wp-w2/generated/`; they move out of the sweep's reach at the full-catalog restart (a restart now would disrupt peers).
- 13:02 rest-warm relaunched (`.🧬semio/🌐hub/w2-logs/release-par.txt`).
- 12:48–13:0x open-plan probe on the restarted 7800: **12/12 PASS** again (fresh root).
- 13:23 **priority restage (coordinator):** the sweep deleted 15 plugins' dev wasm, so the staged `s` guests no longer match. I stopped rest-warm (dag, raster, architect
  had completed; their Nx/cargo units are kept) and launched `w2-restage.sh`: describe-all in one wasm hold (Nx re-describes only changed components, 2 parallel) →
  generate → check → activate-s-react-dev → verify, with logs in `.🧬semio/🌐hub/w2-logs/restage-*.txt`. It is queued behind WG7, T12 and c10 in the wasm FIFO. Then rest-warm → `--packages all`.
