# WP-W2 — Canonical Hub 7800, All-Package Trusted Catalog, Final Rebuild

Session 11 slice W2 (sole all-plugin wasm owner). Ports: canonical hub 7800, hubs 8000–8009, serves 6500–6509.
Captures `wp-w2/generated/`. Scripts `wp-w2/*.ts|sh`.

## Hub Handoff

**CATALOG B2, READY since 2026-09-26 03:59:22** (boot 16 min 41 s under load ~40: the hub interprets each package's guest codec at startup and got ~10 % of a core).
Current tree (restage4 + session-12 ABI-safe edits up to each lane build). Measured `/readyz` (`.🧬semio/🌐hub/s12-w2-logs/readyz-7800-b2.json`): `status: ready`,
`artifactAuthority.ready`, `features.openPlan`, `openPlanExchange`, `rebootstrap`, `mcpWorkspace`, `inferenceServices [s.gis.gismap.inference]`.
Creation catalog: **16 creatable kinds** (2d.block, 2d.drawing, 2d.puzzle, 2d.wfc2d, 2d.wfcbitmap, 2d.wfcgrid2d, 3d.block, 3d.puzzle, 3d.wfc3d, 3d.wfcgrid3d,
5d.block, 5d.puzzle, animate.presentation, s.gis.gismap, s.note.note, text.document). Open-plan probe (`s12-w2-logs/open-plan-probe-7800-b2.txt`): **16/16 PASS** (creation → Ready → `POST …/open-plan` 200, editor surface, closed browser actor). Footprint after 16 creations **669 MB** (`footprint-7800-b2-after-16-creations.txt`).

| what | value |
|---|---|
| URL | `http://127.0.0.1:7800` (loopback, development) |
| catalog | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-b2` (published 03:18:31, rc=0, 3689 s): profile `local-stdio-gis-note-animate-block-writer-draw-puzzle-wfc-open-v1`, generation `f485bf7e725255cd5b70787796e477a249578d788a49960564d7613dce46f2e1`, bundle sha256 `6f2d396e564a4cb3bd6f2e5b7617d01e141a2ee3de2affc750183e020e9e2bae`; packages stdio, gis, note, animate, block, writer, draw, puzzle, wfc (wasm-release) |
| binary | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-bin/os-hub-s12-w2-hub-7800-b2` = signed copy of the bootstrap's own build `⚡️cache/cargo/target/debug/os-hub` (02:31, inside the publish): source sha256 `771fc75b8b05f5fef00f417798dcf0b7e26c537bc93d5270355cd648c983288d`, signed `f29b155c9df64a026575279e6a0d9debe821b2894996804c05efc396b933bc08` |
| data root (`OS_HUB_DATA`) | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-hub-7800-b2` (fresh, `0700`, fresh `inference/` sqlite) |
| hold | supervised hold pid `31578` (NI 0, restarts the hub on the same root on a crash, ≤ 5 per 30 min), hub pid `31581`, runId `6e9ecb80bfc004c8b31255d3806f82b3` |
| state | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-state-7800/{pids.txt,status.txt,hold.txt,capture.txt,ready.json,admin-capability.json,restarts.txt}` |
| users | `user1@semio.dev` / `gm1-local-dev-pass-1` (`01a0db60-f3c5-7787-84cb-2a7c7124d5cf`), `user2@semio.dev` / `gm1-local-dev-pass-2` (`01a0db60-fb61-7150-a1b5-aa1323bc51e9`) |
| admin | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-state-7800/admin-capability.json` (`0600`, one `admin-relay` session, 15 min); fresh one: `touch /Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-state-7800/admin-request` (issued within 5 s; ≤ 60 per hub run) |
| stop / resume | stop: `touch /Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-state-7800/stop`; after a whole-process loss: `zsh .tmp-ticket/wp-w2/w2-hub-resume.sh s12-w2-hub-7800-b2` (same root, nothing re-derived) |

HTTP sign-in: `POST /auth/sessions` `{"schema":"semio.hub.auth.credential-sign-in/v1","email":…,"password":…,"deviceInstanceId":"<32 chars>","clientClass":"browser"}`.
Agent credential: a signed-in human `POST /auth/agent-delegations` `{"schema":"semio.hub.auth.agent-delegation-create/v1","spaceId":…,"agentLabel":…,"audience":"edit","ttlSecs":900}`
→ `{delegationId, agentPrincipalId, token}`; write `0600` `{"schema":"semio.hub.agent-credential/v1","hubOrigin":"http://127.0.0.1:7800","spaceId":…,"audience":"edit","token":…}` and run the
semio MCP as `stdio --hub http://127.0.0.1:7800 --space <id> --credential-file <file> --scopes workspace.read,artifact.write`.
Two-client e2e recipe: `OS_HUB_TRUSTED_CATALOG_SOURCE=/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-b2 OS_HUB_BINARY=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-bin/os-hub-s12-w2-hub-7800-b2`.

### Historical (session 11)

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

## Session 12

| # | Item | Status |
|---|---|---|
| S12-1 | Catalog B2 (9 B packages released from the current tree) → `.🧬semio/🌐hub/w2-catalog-b2` → hub 7800 on a fresh root `s12-w2-hub-7800-b2` → users + open-plan probe | **DONE** 04:35: published 03:18 (rc=0), 7800 ready 03:59:22, open plans **16/16**, coordinator messaged |
| S12-2 | Rest (25) → `--packages all` → `.🧬semio/🌐hub/w2-catalog-all` → 7800 once onto it (`s12-w2-hub-7800-all`), probe every kind, `/readyz`, RSS | RUNNING: publish 1 failed 06:34 (demonstrator `*` pins) → root-fixed + preflight; publish 2 chain started 08:52 |
| S12-3 | Requests inbox triage + one consolidated restage if needed | IN PROGRESS (triage) |
| S12-4 | Rebuild convergence root fix (describe consumes component-dev bytes) + one launch row | PREPARED, dry runs clean (23 files + codemod 120 files); lands after the publish |
| S12-5 | P1-4 `build-s-react-release` + RB1 release-bundle probes, measured | PENDING |

### Session 12 Request Triage (`wp-w1/requests/*.txt`, `wp-w2/requests/*`, read 23:0x)

The restage4 tree (describe 18:16–18:48, materialize, activate, verify 60/60) is the tree every B2/all package is released from; no
Rust/WIT/TOML source changed after 18:16 (measured: `find … -newermt '2026-09-25 18:16'` lists only the two generated registry `.rs`).

| request | verdict |
|---|---|
| c7, c8, wg8 (writer/draw/puzzle/block in the hub catalog, from the post-fix tree) | **served by B2** (all four are B packages); paths appended when B2 is live |
| g10 (wfc restage + stdio/gis/note/wfc catalog; `INPUT_MAX_BYTES` 1 MiB in the hub) | **served by B2** + the current-tree os-hub (built 00:10) |
| h7, h8 (tree-consistent catalog + matching os-hub for the two-client e2e) | **served by B2**; recipe `OS_HUB_TRUSTED_CATALOG_SOURCE=.🧬semio/🌐hub/w2-catalog-b2 OS_HUB_BINARY=<s12-w2-bin copy>` |
| g4, g5, g6, g7, g9, h4, m5b, p6, r1, t3, t7, t9, u5 (labels, space, hover label), s3 (15 missing core.wasm) | **done by restage4** (every one of the 60 components re-described, materialized, activated; verify 60/60 incl. every `core.wasm`) |
| t12 items 1–16 (incl. 16: space fold of the one space, file mtime 17:55 < 18:16), s15 guest items (stdio kit verbs, curation, trinity, norm, viewers, space fold) | **done by restage4** (sources landed before 18:16); release builds of B/all take them too |
| s15 13:0x/13:4x (plugin-module index `dialectArtifactKinds` + `extendsPluginId`) | **served by the current-tree os-hub** of B2 |
| wg7 (note release bytes == catalog note bytes) | superseded by S15's plugin-module delivery (hub documents run the catalog's own module); B2 note component sha appended for WG7 |
| r8 (flow_core wasm-pack bindings: `🕸️wasm/🖥️host/🏃️runtime/🟨️.js` 09-25 01:53 ≠ published `🫀️core/🕸️bindings/🖥️host/🟨️.js` 09-24 19:45) | **needs a run** of `semio-framework-os-flow-core:wasm` under the wasm mutex → in the post-publish consolidated pass |
| u5 surface-rs wasm-pack | done by someone at 09-25 06:12 (bindings newer than the 01:44 source) |
| t12 17 (stdio txt/tsv/html declare kinds) | withdrawn by T12 (coordinator 00:0x); stdio stays codec-only in B2/all per the `local-stdio-gis-open-v1` fence |
| t12 18 (wfc bitmap/grid2d/wfc2d/wfc3d + demonstrator playground declare their kinds) | landed before wfc's lane build → B2's wfc open-target count should rise; measured by the probe |

### Session 12 Log

- 18:56 (session 11, recorded now) **restage4 DONE**: describe-all rc=0 (1899 s), materialize-all rc=0 (278 s, Nx cache hits for the components restage3 had built),
  generate rc=0 (29 s), check rc=0 (125 s), activate-s rc=0 (37 s), **verify rc=0: `components=60 consistent=60 diverged=0`**, receipt 60
  (`.🧬semio/🌐hub/w2-logs/restage.txt`, `restage4-verify.txt`). Every session-11 process died ~19:30 (desktop app restart), including hub 7800 on catalog B.
- 22:53 state: no wasm/hub lock, 0 rustc, 161 GiB free, load 9.8, port 7800 free.
- 22:54 **release batch b** launched from the restage4 tree: `w2-release-par.sh b-warm` (chain pid **83333**, setsid via `w2-detach.py`), 9 packages
  3-parallel in ONE wasm hold, then os-hub build (hub mutex) → publish `w2-catalog-b2` → rest-warm (25, one hold) → publish `w2-catalog-all`. The wasm lock is
  released between phases (each phase is its own `fleet-mutex.sh` call). Markers and logs moved to `.🧬semio/🌐hub/s12-w2-logs/` (`release-par.txt` +
  per-phase captures). `w2-restart-7800.sh` now keeps the binary copy in `.🧬semio/🌐hub/s12-w2-bin/` and hold state in `.🧬semio/🌐hub/s12-w2-state-7800/`.
- 22:59 **correction (measured):** the "3-parallel" batches were never parallel. `component-dev`/`component-release` are inferred with `parallelism: false`
  (`📚️library/🟨️.mjs` `componentTargets`), so `nx run-many --parallel=3` runs them one at a time: session 11's `release-par-rest-1.txt` shows dag, raster,
  architect, cad strictly in sequence, and tonight's batch starts with stdio alone. I keep it serial on purpose: concurrent wasm plugin cargos on the shared
  build-dir are the documented `prebuild_lock_exclusive` lock cycle (fleet incidents 09-15, 09-20). Each cargo already uses every core for the shared units.
- 23:15 stdio released serially: **20 min 46 s** (`Finished wasm-release … in 20m 46s`, 50 754 958 B). gis's cargo then recompiled `semio_framework_os_kernel` and
  `…_ui_contract` (a different feature set than stdio's), so shared framework units are NOT all fresh between packages.
- 23:16 **coordinator R4 (audit-s12-build-convergence §3): real lanes.** The audit's premise ("3-parallel cut batch B to 668 s/package") is wrong: that batch
  was serial (above), so 668 s/package IS the serial rate. Real parallelism needs N concurrent Nx processes in one hold: `w2-release-lanes.sh`
  (phases `b-lanes → b-publish → rest-lanes → all-publish`, N lanes round-robin over a priority order, big packages first; a package is ok only when its
  `dist/component-release/semio_s_plugin_<p>.wasm` is newer than the lane stamp). Memory before: swap 1.79/3.07 GB used, ~4.5 GB free + 8.9 GB inactive, the
  stdio plugin rustc 1.8 GB RSS, each Nx process ~0.9 GB. I stopped my serial chain (83333 → its nx 83350, which cancelled its cargo; lock released cleanly,
  WG7 took it at 23:16:07), kept stdio (`release-stdio.ok`), and launched **lanes chain pid 4212** with 3 lanes: [gis, note, animate], [writer, draw, wfc],
  [puzzle, block]. Risk: concurrent cargos on shared units can form the lock cycle; I watch for lane cargos at 0 % CPU with no rustc child and fall back to serial.
- 23:16–23:50+ lanes chain queued in the wasm FIFO behind WG7's hold (`wp-wg7/s12-wasm-hold.sh`: renderer wasm32 check, then `framework-renderer-wgpu:wasm-release`,
  which compiles plugin crates for wasm32-unknown-unknown; load 50–72 from peers' native test builds). Coordinator informed (23:5x).
- 23:51 coordinator preempted WG7's renderer hold; **lanes started 23:50:52**. 23:52 os-hub prewarm (`nice`, hub mutex, capture `s12-w2-logs/hub-build-prewarm.txt`)
  so the publish's hub build is a no-op: done 00:10 (14 min 11 s). At 00:00 load 199, swap 8.2/9.2 GB, but `memory_pressure` 58 % free (swap is the high-water mark,
  not active paging); 00:10 75 % free. Measured lock behaviour: the writer lane's cargo sat in `prebuild_lock_exclusive → LockManager::lock` (sampled) with no
  rustc child from 23:51 until the gis lane's cargo ENDED (00:22), then compiled at once: a cargo that needs to rebuild a unit another live cargo holds shared
  waits for that whole build, so lanes overlap only partly. Not a cycle (the holder progressed). Per package under load ~60: puzzle 28 min 54 s, gis 31 min 10 s.
- 00:52 lanes are effectively serial on this moving tree: block, note and draw cargos all sat in the lock wait while writer (50 min 49 s incl. ~30 min
  waiting for gis) built, because each package re-plans shared units (stdio_semio, kernel, framework) that peers' 23:23–00:11 framework edits invalidated.
  Real overlap only happens for package-specific units. **Rule 17 (BG_NICE):** my lane cargos run at NI 5 (the inner `zsh -c "… & … & wait"` backgrounds them);
  the chain zsh, the publish and the hold (python `Popen`, no `&`) are NI 0. Not restarted (coordinator). `w2-release-lanes.sh` now runs the lanes under
  `setopt no_bg_nice` (new inode, the running chain keeps the parsed old one); the running chain is stopped once it enters the B2 publish, and the rest
  batch starts from the fixed script (it also yields the wasm FIFO once between B2 and rest, as asked).
- 02:14 lanes B END (10 709 s wall incl. 34 min FIFO wait): gis 31m10s, puzzle 28m54s, writer 50m49s, block 49m44s, note 74m03s, draw 75m58s, animate, wfc; all 8 `ok`
  (dist newer than the stamp). 02:14–02:17 hub build (2m18s, the prewarm made it short). **02:17–03:18 publish B2 rc=0 (3689 s**, bootstrap 48m26s; it rebuilt os-hub at
  02:31 for its candidate). ~03:40 usage-limit cut; the chain survived and started the rest lanes at 03:18 (inner `&` → NI 5, left running per coordinator).
- 03:42 7800 restarted with `w2-restart-7800.sh` → supervised hold 31578 (NI 0), hub 31581, fresh root `s12-w2-hub-7800-b2`, binary copy of the 02:31 bootstrap build.
  **Ready 03:59:22** (16 min 41 s: `GuestCodecExecuting` progress at ~10 % of one core under load ~40, `sample`: one thread 100 % in the owned interpreter's
  `CoreInstance::step`). `/readyz` ready with openPlan, openPlanExchange, rebootstrap, mcpWorkspace, gis inference. Coordinator messaged at once.
- 04:02–04:35 open-plan probe: **16 creatable kinds (was 12: +2d.wfc2d, 2d.wfcbitmap, 2d.wfcgrid2d, 3d.wfc3d from T12 req 18), 16/16 PASS.** First-use creation times under
  load: 3d.puzzle 422 s, 2d.puzzle 354 s, 5d.puzzle 308 s, gis 229 s, 2d.wfc2d 177 s, others 7–94 s. Footprint after 16 creations 669 MB (session-11 catalog B: 1002 MB after 12).
- Rest lanes are fast (units shared with B, tree quieter): 21/25 by 04:32, mostly 1–5 min each; norm's plugin crate took 34 min at NI 5.
- 03:18–05:23 **rest lanes 25/25 ok** (7499 s wall; most 1–5 min, norm 34m41s, playbook 24m37s, dag 21m50s, vcs ~25 min, NI 5). 05:23–05:29 hub build.
  **05:29:46 `--packages all` publish started** → `.🧬semio/🌐hub/w2-catalog-all`. T12 req 20 (trinity rewriting declares `text.rewriting`) and 22 (note verb args)
  landed before it, so the bootstrap's own fresh builds carry them. Found in passing: an uncommitted peer change (file mtime 04:18) adds a persistent
  guest-codec verification cache (`<hub data>/guest-codec-verifications/`, keyed by component sha256 + schema + hub executable), which makes a warm restart on
  the same root skip the interpreter; B2's binary predates it (its root has no such directory). Coordinator: ask before building the restart binary (H9 authority
  gates + H10 residency fix).
- 06:34 **`--packages all` publish FAILED rc=1 after 3856 s** (usage-limit cut ~06:0x–08:40): `trusted descriptor dependency is not exact and bounded`
  (`trustedBootstrapDescriptorClaims`) on demonstrator, the 8th of 34 packages, AFTER 8 fresh release builds. Demonstrator declared
  `depends_on(cad|gis|procedural|process|puzzle|sourcing, VersionReq::Any)` → descriptor `version: "*"`; the trusted catalog admits only exact `=x.y.z`
  pins inside its closure. Census of committed descriptors: demonstrator is the only PLUGIN (catalog member) with non-exact pins; 20 extensions carry `*`
  and 2 carry `^0.1.0` (extensions are not catalog packages); gis and vcs pin stdio exactly through stdio's own `native_artifact_catalog_dependency()`.
- 08:4x **root fix, demonstrator (coordinator's freeze exception):** `PLUGIN_VERSION = env!("CARGO_PKG_VERSION")` (every plugin crate is
  `version.workspace = true`, workspace 0.1.0 = every builder's version) and `same_tree_pin()` = `VersionReq::Exact(<that version>)` for all six composed
  plugins (`🪪️manifest/🎪️demonstrator/🦀️.rs`). `cargo check -p semio-s-plugin-demonstrator --lib` rc=0 (4 pre-existing warnings). The SDK-type narrowing
  (the manifest `VersionReq` admits only `Exact`, `depends_on` pins at the tree version, the 22 extension declarations, emitter refusal) is NOT landed now:
  `VersionReq` lives in `semio-framework`, which every artifact crate links, so it would force a recompile of every unit of all 34 packages inside the publish
  (hours). It is prepared for the post-publish landing window, where the consolidated restage rebuilds everything anyway.
- 08:4x **fail fast:** `trustedBootstrapPreflightDescriptorsV1` (hub `📜️script.ts`) runs first in `materializeTrustedCatalogBundle`: every selected package's
  COMMITTED owner-root descriptor must pass the same claim rules as its fresh build (canonical bounded Pack, exact pins, dependencies inside the selected
  closure at that package's own version), naming the `describe` fix. Measured on the committed tree: **REFUSED in 129 ms** with the demonstrator message
  (was 63 min). tsc on the hub script: only the pre-existing leb128 typing error.
- 08:52 `w2-publish-all.sh demonstrator` (pid 14734, NI 0, ONE wasm hold): describe demonstrator → preflight → `--packages all` → `.🧬semio/🌐hub/w2-catalog-all`
  (capture `s12-w2-logs/publish-w2-catalog-all-2.txt`). Disk 90 GiB free.
- 09:02:57 demonstrator described (9 min 10 s): committed descriptor now pins `cad|gis|procedural|process|puzzle|sourcing` at `=0.1.0`. Preflight over all 34
  committed descriptors **PASS in 731 ms**. 09:02:59 publish 2 started. Sampler `w2-publish-sampler.sh` → `s12-w2-logs/publish-all-2-samples.txt` (every 20 s:
  packages complete, stage, wasm rustc crates).
- **What the bootstrap recompiles and why (measured 09:0x–09:2x):** everything from `semio_framework` up, for stdio first (`semio_framework`, `…_ui` →
  20 stdio artifact crates → `stdio_semio` → `semio_s_plugin_stdio`, ~13 min). Cause 1: the `semio-framework` wasm-release unit got a NEW unit hash
  (`build/wasm32-wasip2/wasm-release/build/semio-framework/485589…`, 09:03); its fingerprint differs from the 03:35 rest-lane unit in `profile`
  (2342545200074580010 → 8924815852372134537) and in dependency hashes; the 23:51 B-lane unit had 8924…, so the profile hash flipped twice during the
  session. Root `Cargo.toml` was last edited 04:34 (H9: `[profile.dev.package.semio-framework-hash] opt-level = 3` + a `wasm-dev` override); rustflags,
  features, target and config hashes are identical. Cause 2: a rule-20 breach — the guest-linked kernel `#[path]`-includes
  `📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs`, edited 08:45 after the freeze. Coordinator informed.
- 09:25 stdio alone took ~20 min in publish 2 (its plugin crate ~10 min single-threaded LTO). Bootstrap and lanes share the unit identity at any given
  time (23:51 lanes and 09:03 bootstrap both profile 8924…, 03:18 and 03:35 both 2342…), so lanes DO warm the bootstrap. Launched two warm-behind lanes
  inside my own hold (`w2-warm-behind.sh`, NI 0, pids 37011/37017): lane a = writer, vcs, space, shooting, reasoning, puzzle, procedural, norm, lowpoly,
  imperative, fem; lane b = wfc, trinity, sourcing, sequence, remodel, raster, process, playbook, note, mathematical, layout, forms, flow, energy — the
  publication order from the END while the bootstrap walks it from the front (gis, animate, architect, block, cad, dag, demonstrator, draw, …).
- 11:0x **third full-recompile trigger (measured, kernel dep-info parsed):** the guest-linked `semio_framework_os_kernel` wasm-release unit tracks 102 inputs
  incl. `📚️library/🔣️taxonomy.json`, root `nx.json` and `📋️project.json` (read at compile time by the `🗣️dsl/✨️derive` mutation-source-authority proc macro).
  `🔣️taxonomy.json` was edited 10:44 → the kernel unit went dirty (fingerprint cleared 10:52, recompiled 10:57 under demonstrator), so every package not yet
  built recompiles from the kernel up. Coordinator asked to freeze those files too. Structural follow-up: the derive macro should read a narrow generated
  input instead of the repo taxonomy.
- 09:3x–09:5x **item 4 prepared** (`wp-w2/item4/`, applied only after the publish): `w2-item4-apply.py` (anchored edits, aborts on any missing anchor;
  dry run **23 files OK**) + `w2-item4-describe-codemod.py` (dry run **120 files, 0 problems**). Contents: (a) `describe` = ONE inferred component target
  (`📚️library/🟨️.mjs` `componentTargets`, `dependsOn: ["component-dev"]`, command `🖨️describe/…/📜️script.ts component --manifest <Cargo.toml>`), which reads
  `<crate>/dist/component-dev/<crate>.wasm` and builds nothing (`describeComponentDeliverable`; `buildPluginComponent` deleted; the 60 hand-declared
  targets and 60 `DescribeScript` wrappers deleted); (b) every runtime reader of cargo's internal `wasm32-wasip2/wasm-dev` path moves to the crate
  deliverable `dist/component-{dev,release}`: MCP gateway (Rust `PluginRegistryEntry.crate_path` + TS preflight), `os run` bootstrap (generated
  `PLUGIN_WASM_ARTIFACTS` now carries the crate path, `PLUGIN_COMPONENT_PROFILE_DIRS`), root `os run` preflight, norm's budget law, three host/run tests;
  (c) the chain (`🔁️rebuild/🔣️.json`): `components` = ONE `nx run-many -t describe materialize-dev` (component-dev once per component), generate, check,
  activate-s, **verify-s** (new `plugin-registry:verify-staged --variant s`, the product port of `w2-verify-staged.ts`: committed == dist == staged +
  receipt + every staged file incl. `core.wasm`), publish-catalog; `rebuild-all [--from] [--to]` holds ONE queued exclusive `wasm-build` lease for the
  whole span; (d) the repo library gains `acquireQueuedResourceLease` (arrival-order tickets over the existing SQLite lease, crashed waiters swept,
  cancel-safe) + `⚡️caching/📜️script.ts lease <mode> <resource> <owner> -- <cmd>` for shell callers (the fleet mutex as a product primitive), with a
  FIFO/crash/cancel law in the leases suite; laws `🧪️tests/🔁️rebuild` (chain + `--from/--to`), registry launch + generated-projection laws rewritten
  for the deliverable rule. The launch row `🔁️rebuild-all🔌️plugin-registry` stays the one registered command.
- 23:2x **R2 supervised hold (ticket scripts, used from B2 on):** `w2-hub-hold.ts` now restarts the hub child on the SAME data root and binary when it exits
  without a stop request (5 s backoff, at most 5 restarts per 30 min, then `HOLD_END crash loop`), re-issues the admin capability at each readiness, and stops
  cleanly on `<state>/stop` (pipe EOF). `w2-hub-resume.sh <data root name>` brings 7800 back after a whole-process loss (desktop restart) on the existing root
  + binary copy, copying/provisioning nothing. `bunx tsc --noEmit` on the hold rc=0. Surviving a desktop restart without anyone running `resume` would need a
  launchd agent (persistent user configuration), which I did not create.
- 23:3x **item 4 prepared (not applied):** `wp-w2/item4/w2-item4-describe-codemod.py` (dry run: 120 files, 0 problems) deletes the 60 hand-declared `describe`
  targets and the 60 per-crate `DescribeScript` wrappers; `describe` becomes one inferred component target (`dependsOn: component-dev`) that reads
  `dist/component-dev`. Every reader of cargo's internal `wasm32-wasip2/wasm-dev` path (MCP gateway Rust + TS preflight, `os run` bootstrap + generated
  `PLUGIN_WASM_TARGET_DIR`, root `os run` preflight, norm's budget law, 3 host tests) moves to the crate's `dist/component-{dev,release}` deliverable, because
  after the change nothing writes the shared path any more. Lands after the all-package publish.

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
- 14:32 restage: describe-all rc=0 (4137 s), generate rc=0, check rc=0, activate rc=0 (41 s), **verify rc=1**. Root causes: (1) `activate-s-react-dev` depends only on the
  `s` host plugin's (space) component-dev/materialize-dev. It stages whatever is already materialized ("60 completed components"), so the other 59 kept their 05:5x
  dist/staged modules while describe had moved committed + shared wasm-dev to the new tree (59 DIFF). (2) The 12:1x sweep deleted 15 plugins' `dist/component-dev`
  and 13 staged `*_component.core.wasm` (`dist=-`, MISS). Nothing was ever "never produced". (3) The verify double-counted a component that was both DIFF and MISS
  (`consistent=-12`): it now counts distinct components. **Fix:** the restage runs `nx run-many -t materialize-dev` for every component in one hold after describe
  (component-dev reuses the describe unit), then generate → check → activate → verify (`w2-restage.sh <tag> materialize`). 15:23 launched (`restage2`).
- 16:24 restage2 materialize: 58/60 rc=0. demonstrator and vcs `component-dev` failed on transient build-dir file errors (`failed to create file encoder: No such file
  or directory` in `semio-s-artifact-stdio-semio`), nx rc=130. The retry (restage3) was queued behind WG7 (16:24–17:11) and has held the lock since 17:11. Its materialize is recompiling
  most components again: peers' edits to shared crates since 15:23 invalidate the component-dev units. Measured, not assumed: the nx log shows fresh
  `component-dev` runs, not cache hits.
- Plan after the verify (coordinator + G10): release B packages from this tree (`w2-release-par.sh`, B list = stdio, gis, note, animate, block, writer, draw, puzzle, wfc; markers in
  `.🧬semio/🌐hub/w2-logs`) → publish **w2-catalog-b2** → restart 7800 ONCE on the current-tree os-hub with a fresh data root (fresh
  `inference/gis-map-jobs.sqlite3`, for G10's 8 MiB stack + inference fixes) → rest → `--packages all`.
- 18:15 restage3: materialize-all rc=0 (5944 s, all 60), generate/check/activate rc=0. **Verify: every staged module and `core.wasm` is present, and dist == staged for 60/60.**
  But committed == shared (the 13:24–14:32 describe) ≠ dist (the 16:25–18:04 component-dev) for all 60: peers' shared-crate edits landed between the two runs, so the
  descriptors describe an older build. The verify counts 60 distinct components now (`consistent=0 diverged=60`, no more negatives). 18:1x restage4: describe-all again, which reuses the
  units component-dev just built wherever the tree hasn't moved, then materialize (Nx cache hits for the unchanged ones), generate, check, activate, verify.
