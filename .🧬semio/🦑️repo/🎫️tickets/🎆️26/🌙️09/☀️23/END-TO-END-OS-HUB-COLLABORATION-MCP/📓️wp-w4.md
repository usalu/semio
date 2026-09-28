# WP-W4 — Final Chain Owner (successor of [W3](📓️wp-w3.md))

Session 14 slice W4. Canonical hub 7800, hubs 8000–8009, serves 6500–6509. Scripts `wp-w4/`, captures `wp-w4/generated/`, durable
data `.🧬semio/🌐hub/s14-w4-*`. Handover: [📓️wp-w3.md](📓️wp-w3.md) (Session 13, phase `final`).

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | Layout leaves `📐update-grid` + `🔒set-frame-flags` | **LANDED** 18:3x–19:38 (authority, payload schema, wire tags 26/27, every mirror surface, fixtures + fixture tests, inverse-exact flag storage, typed empty fatal target); **wasm32 green** `-p semio-s-artifact-layout-layout` 19:47 + `-p semio-s-plugin-layout` 19:58; native `--lib --tests` queued in the lane |
| 2 | robotic + flow-text component builds | **PROVEN 14b**: chain a2 components step built `process-extension-robotic-rust` + `flow-extension-text-rust` describe + materialize-dev (all 26 extensions, no error, 12:37–13:1x). Was open: failure text lost in nx interleaving (no `could not compile`, no rustc error, both end in a truncated `…warningerror`); repro 1 (18:41) invalid (my zsh `$project:c` bug, fixed); repro 2 running since 19:58 but stalls in cargo lock waits (load 85, Codex default-build-dir cargos) → the chain's components step is the proof (w4_retry) |
| 3 | Preflight (leaf authority scan, untracked wired files, checks) | scan done (2946 leaf sources: every wired leaf has its `🔣️.json`); live peer edits listed below; checks blocked by item 7 |
| 4 | Stale session-13 mutex waiters (31879 native, 42023 wasm) | **DONE** 18:21 (TERM is trapped by the waiter loop → SIGKILL; queues empty, no lock held) |
| 5 | Relaunch prep | **COMMAND SENT** 20:4x: `wp-w4/w4-chain.sh final` (+ `w4-wasm-hold.sh`, `w4-green.zsh` retry-until-green, `w4-restart-7800.sh`, `w4-hub-hold.ts` rolling capture, `w4-hub-resume.sh`); failed logs archived as `s13-w3-logs/final-a1-*`; guards clean |
| 6 | Monitor chain → `final-publish.rc` → 7800 READY on ALL → readiness/open-plan probe | **RUNNING** (14b): launched 28 12:02:46 (pid 45604); rebuild-all a1 red 12:06 (stdio docx/xlsx, overnight peer) → fixed 12:12–12:25 → green-probe #3 rc=0 → retry a2 `--from guest-framework` 12:17 |
| 8 | 14b chain fixes (unowned overnight peer fallout) | stdio docx/xlsx + stdio-semio docx io **LANDED** (wasm32 stdio plugin rc=0); os-run AppCommand/AppFrame media-export arms written (G12 relay), native check queued; hub-prewarm rc=1 = H14's half-applied kernel-db `PlannedEntries`/`apply_one` (H14 fixing, coordinator 12:1x) |
| 7 | Codex overlap (value-derive typed-path set, 18:35–) | **RESOLVED by the peer 19:16:54** (it added `match *self {}` for empty enums itself; my prepared `w4-value-derive-empty-enum.py` was NOT applied). Was: `semio-framework-plugin` wasm32 6×E0004 (`#[derive(ToValue, FromValue)]` on the empty `No{Config,Presence,Transient}Mutation` enums now expands `match self {}` arms) — peer's in-flight `🌱️value/✨️derive/⚙️expansion` (+933 lines); earlier (18:4x) kernel/actor/3d E0282/E0034 from the same set |

### Relaunch Command (sent to main 20:4x)

`python3 .tmp-ticket/wp-w2/w2-detach.py .🧬semio/🌐hub/s14-w4-logs/chain-final.txt zsh .tmp-ticket/wp-w4/w4-chain.sh final`

### Codex Overlap (session 14)

A Codex fleet (ChatGPT app, codex app-servers from ~18:15; tickets 26/09/26 COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE and
COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE) edits guest-linked code while this slice prepares the chain. Measured from mtimes/`git status`:

| set | files | last edit | state |
|---|---|---|---|
| value derive typed-path | `🌱️value/✨️derive/⚙️expansion` (+933), `🌱️value/🔁️codec` (+245), `🌱️value/🦀️.rs`, derive `Cargo.toml`, new `✨️derive/🧪️tests/🧭️typed-path` | 18:58:42 | **red** (19:13 wasm32: plugin E0004 on empty enums) |
| schema fragment validation | `🧬️schema/✅️validator` (+408), component-unit tests, schema `📋️project.json`/`📜️script.ts`, new oracle + vectors | 19:13:43 | unknown (in flight) |
| raster `🎛️change-layer-adjustment-parameter` | leaf un-parked + codecs, grammar, protocol, inspection, patch-layer | 18:43 | unknown |
| draw `🎨️style/📝️update-text` | new leaf (authority present) + editor/properties/terminology/diff/owned | 18:44 | unknown |
| stdio contract `✏️editing/🩹️patch` | new module, wired from `✏️editing/🦀️.rs` | 18:44 | unknown |

Coordinator rule (19:0x): do not touch a set while it is being edited; a set red with NO edits for ≥ 30 min counts as abandoned and is
finished compile-atomically by W4. The chain copy tolerates transient reds (retry-until-green, below).

### Session 14 Log

- 18:20 state: disk 141 GiB free, swap 7.6/9.2 GB, load ~4.4, 0 rustc; no `/tmp/semio-*-build.lock` held; hub/native/wasm queues empty.
- 18:21 stale waiters 31879 (`fleet-mutex.sh native lb`) and 42023 (`fleet-mutex.sh wasm lb`), both ppid 1, both in the `sleep 15` loop
  with no queue ticket left (an earlier TERM had run their `trap 'rm -f ticket' … TERM`, which removes the ticket but does not exit
  the loop). TERM again → no exit (trapped); SIGKILL → both gone.
- 18:2x **Layout diagnosis.** The peer's 14:21 set (staged by the auto-stager) wires `UpdateGrid`/`SetFrameFlags` into the
  `LayoutMutation` enum, `KINDS`, the root `#[path]` tree, grammar + proto, oracle catalogue, `FramePatch.locked/visible` +
  `apply_frame_field_patch`, `Frame::locked()`, and editor (patch-document command, inspection, terminology, catalogue). Missing:
  each leaf's `🔣️.json` authority and `🧬️schema/🔣️.json`, the `💾️binary/📡️.protocol.semio` records (the ONLY source of wire tags →
  `encode_op` would fail at runtime), the kinds count law (26), the ebnf/g4/text-JSON/text-GraphQL/aggregate JSON/GraphQL/TS mirrors,
  the fixtures' `🔺️diff`/`🎯️outcome`, and every committed frame-patch `🔺️diff` lacked the two new always-on-the-wire fields (7
  fixture tests would go red). `rotate-frame` (09-25) had never been mirrored into those surfaces nor into the `🔺️diff` schema
  (`FramePatch.rotation`) either. A stray `📸️snapshot/🔤️splits-the-text-frame-into-two-columns/🔣️.json` (byte copy of `➡️after`) sat in
  update-grid's snapshot facet.
- 18:3x–18:5x **Layout edits** (scripts idempotent, dry runs clean after apply):
  - authorities: `📐update-grid/🔣️.json` (binaryTag 26, outcomes applied/no-op/rejected), `🔒set-frame-flags/🔣️.json` (27), both
    `textOpcode: null`, surfaces rust/json-schema/text/binary like every sibling; payload schemas `🧬️schema/🔣️.json` (camelCase wire,
    `baselineGrid` `exclusiveMinimum: 0`).
  - `💾️binary/📡️.protocol.semio`: `record update-grid tag=26`, `record set-frame-flags tag=27`.
  - `wp-w4/w4-layout-surfaces.py`: grammar (+rotate-frame), ebnf, g4, proto (+`RotateFrame` message, oneof 28), text GraphQL + JSON,
    aggregate JSON (`oneOf` +3, sorted), aggregate GraphQL + TS (28 leaves).
  - `wp-w4/w4-layout-diff-schema.py`: `🔺️diff` JSON Schema/proto/GraphQL/TS `FramePatch` += `rotation`, `locked`, `visible`.
  - `wp-w4/w4-layout-fixtures.py`: 7 frame-patch diffs gain `"locked": null, "visible": null`; update-grid + set-frame-flags gain
    `🔺️diff` + `🎯️outcome`; the stray snapshot moved to `wp-w4/moved-aside/` (not deleted).
  - fixture tests `📐update-grid/🧪️tests/📐️sets-an-18-point-baseline/🦀️.rs` and `🔒set-frame-flags/🧪️tests/🔒️locks-frame-1/🦀️.rs`
    (apply == after, inverse == before, canonical JSON, committed diff/outcome, rejection), mounted in the root `#[path]` tree.
  - `apply_frame_field_patch`: a flag equal to its default is stored as `None` (`locked` only `Some(true)`, `visible` only
    `Some(false)`), so `set-frame-flags`' inverse restores a default-flag document exactly (the payload cannot carry "unset").
  - kinds law: 28 variants.
- 18:34–18:47 native check (`build-fleet-b`, lane): **red outside layout** — `semio-framework-3d` E0282, `semio-framework-actor` 8×E0282,
  `semio-framework-os-kernel` 6 errors (E0034 `multiple from_value found`) = the Codex value-derive set mid-edit
  (`wp-w4/generated/check-layout-1.txt`). Routed to main 18:5x.
- 18:4x preflight scan: 2946 `MutationLeaf` sources; every wired leaf owner has its `🔣️.json` (flagged rows were aggregate files, and the
  draw `📝️update-text` leaf mid-creation — its authority landed 18:41). Untracked/modified guest-linked sets in flight: table above.
- 18:41 repro of robotic/flow-text `component-dev` INVALID: `"@semio-tech/$project:component-dev"` — zsh applied the `:c` modifier
  (`…-rustomponent-dev`, nx "Cannot find configuration"); fixed to `${project}`. Re-run pending (item 7).
- 18:5x–19:0x **chain copy** (coordinator: tolerate transient peer reds): `wp-w4/w4-chain.sh final` + `w4-wasm-hold.sh` (W3's phase
  `final` on names `s14-w4-logs`, `s14-w4-catalog-all`, root `s14-w4-hub-7800-all`, bin `s14-w4-bin/<root>`, mutex slice w4; b3 branch
  dropped). `w4-green.zsh`: `w4_retry <step> <wasm|native> <blind|strict> <resume>` — on failure name every `could not compile` crate,
  probe `cargo check --lib [-p …] [--target wasm32-wasip2]` every ≤ 5 min up to 45 min, then re-run (≤ 3 retries); rebuild-all resumes
  `--from` the failed step; a failure naming no crate gets one blind retry after 5 min (rebuild-all) or fails at once (publication,
  hub/mcp build); a publication retry moves the failed catalog aside. Every attempt log kept (`<step>-a<n>.txt`), START/RETRY-WAIT/
  GREEN-PROBE/RETRY/FAILED lines. Simulated (`wp-w4/generated/sim/sim.zsh`): red→probe red→probe green→resume `--from components`→green;
  strict no-crate fails at once; blind no-crate retries once; never-green times out → FAILED.
- 19:0x **rolling hub capture** (coordinator, C12 finding): `wp-w4/w4-hub-hold.ts` streams the hub's stdout+stderr into
  `capture.txt` as it arrives (events as `=== <time> <tag>` lines), rotating to `capture.1.txt` at 1 MB (the library keeps only the FIRST
  1 MB). `w4-restart-7800.sh` / `w4-hub-resume.sh` start it (resume admits `s14-w4-hub-7800-*` and `s13-w3-hub-7800-*`). Live test on
  8001 (B3 catalog copy, B3 binary): READY in 59 s, admin issued, capture streamed (hub JSON lines + `=== HOLD`), stop → clean
  `CHILD_EXIT code=0`, port free (`wp-w4/generated/holdtest-*.txt`); test root removed. Rotation itself not exercised (> 1 MB).
- 19:04–19:13 wasm32 check (wasm lane, default build-dir) `-p semio-s-artifact-layout-layout`: rc=101 in `semio-framework-plugin`
  (6×E0004 on `NoConfigMutation`/`NoPresenceMutation`/`NoTransientMutation`, `🔌️plugin/🦀️.rs` untouched since 15:38) = the value-derive
  set's new enum bodies `match self { #(#arms),* }` with zero arms (`wp-w4/generated/wasm-check-layout-1.txt`). Layout not reached.
- 19:1x failed session-13 final logs renamed `s13-w3-logs/final-a1-*`, `chain-final-a1.txt`; guards: no `s13-w3-hub-7800-all`, no
  `s13-w3-catalog-all`, no `s14-w4-*`.
- 19:16:54 the Codex peer fixed the empty-enum expansion itself (4 sites `if data.variants.is_empty() { return Ok(quote! { match *self {} }) }`);
  my prepared patch (`wp-w4/w4-value-derive-empty-enum.py`, trial-diffed on a copy) stays unapplied.
- 19:17–19:35 wasm32 `-p semio-s-artifact-layout-layout`: rc=101, ONE error in my scope: `📐update-grid/🦀️.rs:32` E0283 (the peer's
  `MutationOutcome::fatal(…, [])` cannot infer the target item type) → `Vec::<String>::new()` (the repo's convention for an
  untargeted invariant fatal). 19:38–19:47 rerun **rc=0** (3 pre-existing warnings, `wasm-check-layout-3.txt`); 19:48–19:58
  `-p semio-s-plugin-layout` **rc=0** (`wasm-check-plugin-layout-1.txt`). Tree includes H13's 19:15 check-in retire set.
- 19:58– robotic `component-dev` repro 2 (`repro-2.txt`): compiles kernel/ui/3d after the derive change, then waits on unit locks with no
  rustc child. Machine at 20:40: load 85, swap 34.8/35.8 GB, disk 109 GiB (141 at 18:20), ~18–20 rustc.
- 20:2x **Codex build load** (pids from `ps`, all under codex app-server 32372): ~12 cargos in the DEFAULT build-dir — nx `test` of
  raster, value-derive, pixels, draw, flow (`cargo test --no-run` at 0 % CPU for 60–80 min), `serve-raster-react-dev` →
  `trusted-catalog-bootstrap --packages stdio,gis,note,writer,draw,puzzle` (its `cargo build --bin os-hub` idle 58 min), trunk
  `framework-renderer-wgpu:wasm`. Not ours to kill. A private chain build-dir was considered and rejected: default build-dir sizes
  wasm-dev 19 GB + wasm-release 12 GB + host ≈ 45 GB → ~60 GiB free, below the 80 GiB floor. The chain stays on the default build-dir.
- 20:1x F3 landed async activation hashing (dev activate TS, laws 56/56) → covered by the relaunch (activate-s runs it).
- 20:4x relaunch command sent to main (item 5); recommendation: launch now, the chain's components step is the robotic/flow-text proof.

### Session 14b

- 12:06 successor W4 started (predecessor cut ~20:45 by the usage limit). Chain launched by main 12:02:46 (pid 45604, logs
  `.🧬semio/🌐hub/s14-w4-logs/`); 7800 resumed on B3 12:03 (hold 45800 / hub 45803, `s13-w3-state-7800/pids.txt` = `hold=45800`,
  so the move step's `sed -n 's/^hold=//p'` hands 45800 to `w4-restart-7800.sh` → `touch stop` + wait on `hub-hold\.ts 7800 `).
- 12:03 hub-prewarm rc=1 (42 s): `semio-framework-os-kernel-db` 4 errors — `PlannedEntries` undefined (`🛢️db/🗿️artifact/🦀️.rs:2007/2020/2081`)
  and replay still calls the removed `apply_one` (l.1868): a half-applied 27 20:41 plan/commit split of `submit` (auto-commit 21:54).
  Coordinator: H14 owns + fixes it; it only gates the post-publish hub build.
- 12:06 rebuild-all a1 red in 3/11 guest-framework (`-p semio-s-plugin-stdio` wasm32): overnight peer (04:56–05:03) left
  docx `🧭️xml-address` without the `set_word_attr` import (E0425 ×2) and xlsx `NamedTripleDiff.order` `Option` → `Vec<K>` with 4
  `order: None` constructions (E0308 ×4). Fixed (landing row); my first docx import swap dropped the still-used
  `qualified_word_prefix` (check-1 red, my grep was cut by `head`) → restored (check-2). check-2 surfaced the next crate:
  `semio-s-artifact-stdio-semio` docx importer/exporter still on the old `DocxSnapshot.document` (E0609 ×2, E0308) → importer uses
  `project_document()`, exporter `build_minimal_docx`, tests follow. **check-3 wasm32 `-p semio-s-plugin-stdio` rc=0** (113 s).
- 12:17 chain green-probe #3 rc=0 → RETRY rebuild-all a2 `--from guest-framework`. The forward warm lane's stdio/gis releases
  failed on the same docx error (pre-fix); later lanes are fine.
- 12:2x G12 relay: peer's 04:30 store `AppCommand::{Submit,Poll,Cancel}MediaExport`/`TakeMediaExportChunk` + `AppFrame::MediaExport*`
  → `semio-framework-os-run` exhaustive matches (`frame_in_reply_to`, `app_command_seq`) completed. Other `AppFrame`/`AppCommand`
  matches: renderer-wgpu wasm32 passed guest-framework; os-mcp fixed by G12; plugin/host/os crates are in the queued native check.
  Native `--keep-going --lib --tests` of os-run + stdio semio/docx/xlsx queued in the native lane (pid 62084, 7th in queue).
- 13:1x coordinator: native check is chain-critical → waiter 62084 stopped (TERM removed its ticket but the loop kept sleeping →
  SIGKILL, my pid), requeued with `FLEET_TICKET_STAMP=20260928120000` (pid 50632). 13:36 result (`generated/s14b-native-check-2.txt`,
  `--keep-going --lib --tests`): **os-run lib+tests 0 errors; stdio-semio lib+tests 0 errors**; docx `lib test` 10 errors, xlsx
  `lib test` 107 errors = the overnight peer's tests never moved to the opc + xml_parts snapshots (`XlsxSnapshot.workbook`,
  `io::encode_docx`, missing fixture `🧹️clear-main-declaration/🔣️.json`) — test-only, not chain-linked → routed to main for ST2.
- 12:37–13:36 rebuild-all a2: guest-framework green, components (60 projects) running, no `could not compile` yet.
- 13:50 a2 components red: `semio-s-artifact-playbook-playbook` E0432 ×2 (27 23:5x peer added `🪟️windows/🗂️files` + its imports, no mount in
  the root `#[path]` tree) → mounted, wasm32 rc=0 13:54; `semio-s-artifact-shooting-shooting` E0308 (stdio svg `write_svg_xml` now returns
  `Result`) → all 6 unconverted callers fixed (animate ×3, shooting, layout, note). Proactive wasm32 `--keep-going --lib` check of the 24
  plugin crates not yet built in a2 (pid 81134) so every remaining red surfaces before the retry.
- 14:06 proactive check: 23 of 24 plugin crates green; writer red (`docx` importer on `.document`) → `project_document()`; 14:08 wasm32
  writer + shooting + animate + layout + note + playbook **rc=0**. Every plugin crate now passes `cargo check --lib --target wasm32-wasip2`
  on the live tree (component features not covered). ST2 fixed the svg `📡️.protocol.semio` SHA pins ~14:1x; a2 had not described
  stdio/gis/vcs yet.


### Session 14c

- 16:57 successor W4 started (predecessor cut ~14:37 with the app; chain a2 had reached components). Chain RELAUNCHED by main 16:55:46
  (pid 76013, same command/log). hub-prewarm running (os-hub build-dev compiling host deps), wasm hold → rebuild-all a1 at 2/11
  mutation-authority 16:57 (1/11 provenance 66 s, 0 foreign units). Load 23, 9 rustc, disk 84 GiB free, swap 4.9/6.1 GB. 7800 down (the
  move step starts it on ALL). Old `final-rebuild-all-a1*.txt` (14b) will be overwritten by this run's attempt logs.
- 17:13 **hub-prewarm rc=0** (1039 s; H14's kernel-db fixes hold — the 14b rc=1 is gone). Forward warm lane starts now. rebuild-all a1 at
  3/11 guest-framework (2/11 mutation-authority 165 s); only lint-style warnings so far. Load 46–60 from peer native tests (LB2 stdio
  `shipped_fleet`, Cursor peer norm `--no-run`, a `cargo test --bin os-hub`).
- 17:32 rebuild-all a1: **3/11 guest-framework done (1401 s, no error)**; 4/11 components (descriptors) running (playbook procedural build).
  Forward warm lane: stdio release building since 17:13 (no error).
- 18:11 components: 32 plugin crates compiled (all 26 extensions incl. robotic + flow-text, then cad/demonstrator/imperative/mathematical/
  process/sourcing), no error. Forward warm lane: **stdio release rc=0** (17:13–17:48), gis since 17:48.
- 19:00 components still running (raster now), no error in 1 h 28 min; warm lane released stdio, gis, animate, architect, block, cad (all rc=0),
  dag running. Load ~32.
- 19:1x **components a1 has two reds** (step continues, fails at its end → blind retry `--from components` after 5 min):
  (1) **stdio describe refused**: dev component 274 393 279 B > 256 MiB `FRESH_COMPONENT_MAX_BYTES` (s13 core 253 MiB → 262 MiB now;
  `name` section 54.6 %) → `Cargo.toml` `[profile.wasm-dev.package.semio-s-plugin-stdio] strip = "symbols"` (norm's precedent; norm dev
  145 MB, no name section) LANDED 19:21.
  (2) **`process` component-dev failed** with the old `…warningerror` signature (no crate named; nothing edited on its path since launch).
  Root cause measured: `buildCargoArtifacts` spawned cargo with stderr `inherit`; Bun marks its stderr `O_NONBLOCK` once written, so cargo
  shared a non-blocking pipe and its burst of replayed warnings failed with EAGAIN when the nx prefixer lagged under load
  (`w4-nonblock-probe.ts`: inherit → BlockingIOError 35 after 64 KiB; pipe+forward → 4 000 000/4 000 000 B). Fix LANDED 19:23 in
  `native-build/🟦️.ts` (stderr piped + forwarded); tsc 0 new errors. gis describe passed (235 MB dev).
- 19:28:41 rebuild-all a1 rc=1 (9175 s): nx "Failed tasks" = exactly `process-plugin:component-dev` + `stdio-plugin:describe` (both fixed above);
  vcs/wfc/… after the native-build edit built + described fine. RETRY-WAIT blind → a2 `--from components` at ~19:33:42.
- 20:03 **rebuild-all a2 components GREEN** (19:33–~20:0x): process component-dev + describe passed; **stdio dev component 124 447 025 B**
  (core 119 MiB, was 274 MB) and **stdio describe passed** = proof of the strip override AND of LB2's p4 (`plugin()` no longer panics at
  describe). 5/11 generate done, 6/11 check running. Blind retry is spent: a no-crate failure from here ends the chain.
- 20:05 **chain run c1 FAILED** at rebuild-all a2 6/11 `check` (registry): forms modes `📨️responses` / `✍️fill` (overnight peer, 01:0x–01:4x)
  missing required mode lanes (blind retry already spent → hold exited, chain exited 20:05:18; warm lane had released stdio…process).
  Fixed 20:1x (landing row: 9 `📌️.empty.md` lanes copied from the sibling `📝️blueprint` mode, fill's `🪟️windows` from architect `🔍️review`);
  **registry check rc=0** (201 s, `generated/s14c-registry-check-1.txt`). Logs of run c1 archived as `s14-w4-logs/c1-*`.
- 20:1x `w4-wasm-hold.sh`: `W4_REBUILD_FROM=<step>` resumes rebuild-all at a step (zsh -n ok). Relaunch command sent to main:
  `python3 .tmp-ticket/wp-w2/w2-detach.py .🧬semio/🌐hub/s14-w4-logs/chain-final.txt env W4_REBUILD_FROM=check zsh .tmp-ticket/wp-w4/w4-chain.sh final`
- 20:1x pre-validation on the tree the relaunch uses: `os-hub:trusted-catalog-preflight --packages all` **rc=0** (39 s: 34 committed descriptors
  with bounded deps; 60 component-dev deliverables = 34 packages + 26 extensions match their descriptors; `generated/s14c-preflight-1.txt`).
- **20:11:59 chain run c2 RELAUNCHED** by main (pid 95069, `W4_REBUILD_FROM=check`; a 20:10:55 start left only a log line, no processes).
  rebuild-all `--from check` + hub-prewarm running.
- 20:10:55 correction (coordinator): the first relaunch failed in 26 s — MY bug: zsh does not word-split `${W4_REBUILD_FROM:+--from $W4_REBUILD_FROM}`,
  so rebuild-all got `--from check` as ONE argument and rejected it. The coordinator killed that process group (pgid 94710), changed both
  expansions in `w4-wasm-hold.sh` to `${VAR:+--from} ${VAR:+$VAR}` and relaunched 20:11:59 (pid 95069). Lesson: in zsh, a `:+` word with a
  space is one word — split it into two expansions (or `${=…}`). c2 hub-prewarm rc=0 (77 s) 20:13:16; rebuild-all 6/11 check running.
- **20:23:08 rebuild-all c2 rc=0** (669 s: check, activate-s, verify-s, flow-core-bindings all green). Preflight started 20:23:08.
- **20:36:12 publish-all c2 FAILED** (758 s, strict): `trusted fem closure carries no artifact codec` → **final-publish.rc = 1** (sent to main).
  Reproduced offline with the publish's own probe (`w4-codec-probe.py`: descriptor ArtifactKindSpecs asked of each component-dev via
  `semio-framework-plugin-describe codecs`; `generated/s14c-codec-probe-1.txt`): **9 of 34 packages own 0 codec rows** — the guest resolves a
  codec owner only by an app's `DOCUMENT_SCHEMA` or dialect kind, and each of these declares a spec schema that is neither:

  | package | declared ArtifactKindSpec schema(s) | app `DOCUMENT_SCHEMA` / dialect kind |
  |---|---|---|
  | fem | `computation.fem2d`, `computation.fem3d` (results OUTPUT kinds of `results:out`) | `fem.2d`, `fem.3d` / `s.fem.fem2d`, `s.fem.fem3d` — no document kind spec at all |
  | flow | `flow.artifact` (kind `computation.flow`) | `FLOW_DOCUMENT_SCHEMA` / `s.flow.flow` |
  | mathematical | `computation.equation` | `semio.equation/v1` / `s.mathematical.equation` |
  | shooting | `shooting.scene`, `2d.image` | `shooting.shooting` / `s.shooting.shooting` |
  | lowpoly | `lowpoly.fixture` | `lowpoly.document` / `s.lowpoly.lowpoly` |
  | forms | `form.dictionary` | `forms.form` / `s.forms.forms` |
  | norm | `norm.<std>.document` ×15 | `semio.norm.<std>/v1` ×15 / `s.norm.<std>` |
  | imperative | `procedure.document` | `procedure.document/v1` / `s.imperative.procedure` |
  | sourcing | `sourcing.curation`, `catalogue.kinds`, `kit.catalog` | `sourcing.curation/v1` / `s.sourcing.curation` |

  24 packages answer ≥ 1 owned row (+ stdio via linked codecs). demonstrator owns 5 rows but depends on sourcing (descriptor dependency) →
  excluded from the subset too. **Coordinator DECISION B (20:4x):** publish the codec-complete subset now; the 9 spec fixes (each spec's
  schema = its app's DOCUMENT_SCHEMA constant, fem gains a document kind spec beside its results kinds) + a describe-time law refusing any
  package whose closure owns 0 codec rows are W4's window-3 items; the next chain republishes all 34.
- 20:4x partial `s14-w4-catalog-all` (empty `trusted-catalog/`) moved to `s14-w4-catalog-all-failed-c2-2036`. Preflight `--packages <25>`
  refused (demonstrator → sourcing); **preflight p24 rc=0** (24 descriptors, 33 component-dev deliverables = 24 packages + 9 extensions;
  `generated/s14c-preflight-p24.txt`). Chain scripts gained `W4_PACKAGES`, `W4_CATALOG`, `W4_ROOT`, `W4_REBUILD_FROM=skip` (lanes warm only
  the subset; `zsh -n` ok; split/ordering tested; pre-edit copies `generated/*.pre-p24`). Run c2 logs archived as `s14-w4-logs/c2-*`.
- **20:45:25 chain run c3 RELAUNCHED** by main (p24 publish-only: `W4_REBUILD_FROM=skip`, catalog `s14-w4-catalog-p24`, root
  `s14-w4-hub-7800-p24`); rebuild-all skipped, preflight started.
- **20:56:26 publish-all c3 rc=0 → final-publish.rc = 0** (628 s; preflight p24 33 s; immutable 24-package generation
  `d1099ba98c89995f…`, bundle `d527af60…`, profile `local-stdio-gis-…-writer-open-v1`, catalog `s14-w4-catalog-p24`). Sent to main.
- **21:00:00 7800 READY on p24** (hold 9448 / hub 9459, spawned 20:59:52 → HOLD status=ready in 8 s; admin issued; root `s14-w4-hub-7800-p24`,
  binary sha256 `587ed4a2…`, generation `d1099ba9…`). hub-build rc=0 (20 s), mcp-build rc=0 (157 s), restart rc=0.
  **Chain bug (mine):** `move_7800` polls `status.txt` for `HOLD `, but the hold overwrites it 4 ms later with `ADMIN issued` → the loop
  would have waited its full 7200 s. Released 21:10 by appending the true HOLD line (from `hold.txt`) to `status.txt`; the chain logged
  "READY boot=616s" (inflated by the race; real boot 8 s). Fix after the chain ends (never edit a running zsh script): poll `hold.txt`.
- 21:10–21:29 chain probes: /readyz ready (no unready component); hub-freshness "unverifiable" = harness bug (lsof raw UTF-8 under nx locale,
  latin1 round trip mangled the emoji path) → fixed (landing row) → **verdict fresh** (`generated/s14c-hub-freshness-2.txt`). **Open-plan
  36 kinds: 34 PASS, 2 FAIL** — procedural `2d.generation`/`3d.generation` genesis: guest panic "ordered-map root must be explicitly retired
  before drop" (`🌱️value/🗂️ordered/🦀️.rs:81`, hub capture l.178/359) → procedural owner. **Footprint idle 1157 MB → 1622 MB** after
  creations. Chain: "s14-w4-catalog-p24 SERVED on 7800" 21:29:15; release plugin-root re-materialize running. READY summary sent to main.
- 21:00 WINDOW 3 OPEN (coordinator). Next W4 item: the 9-package kind-spec scripts + the zero-owned-codec describe gate for L1 (T3).
- 21:3x–22:1x **window-3 item: ONE schema identity per document kind** (coordinator DECISION 21:4x: unify on the io identity; rewrite the pinned
  laws to pin equality; keep any real media information as a separate declared field). Findings behind it: every platform reader keys a kind on
  ONE schema — hub `🗿️artifact-authority` adapters (`document_codec(&kind.schema)`, `codec.schema != kind.schema` refused), trusted-catalog
  open targets, MCP workspace (`storage.write(id, &kind.schema, …)`), host-media contributions (`artifact_kind.schema == artifact_schema`) — and
  each app's `io` already presents that identity (`ArtifactPresentation.id` + `artifact_schema` = DOCUMENT_SCHEMA). Four packages (flow,
  imperative, shooting, sourcing) had laws pinning the split ("deliberately NOT"); math had one too. **Old media strings are not lost:** each
  stays declared as `source_format` (flow `flow.artifact`, shooting `shooting.scene`, lowpoly `lowpoly.fixture`, forms `form.dictionary`,
  imperative `procedure.document`, sourcing `sourcing.curation`, norm `norm.<v>.document`); math's was the kind id itself. Port payload schemas
  (forms `dictionary:out` = `form.dictionary`, fem `results:out` = `computation.fem2d/3d`) are separate `Media` payload fields — unchanged.
  Static spec/io law over all 34 current descriptors: violators = the 9 + demonstrator (embeds sourcing's app) + **trinity** (rewriting io
  presents undeclared `trinity.rewriting`; its spec is `text.rewriting`) → fixed too.
  - `wp-w4/w4-kind-spec-identity.py` (`--dry-run|--write|--revert`, **dry-run 42 pending / 0 bad**): 9 spec schemas → DOCUMENT_SCHEMA
    constants (norm: `artifact_kind_spec(variant, label, artifact_schema)` + 15 callers pass `<V>_DOCUMENT_SCHEMA`); 5 laws rewritten to pin
    equality + `source_format`; shooting editor io → `crate::SHOOTING_DOCUMENT_SCHEMA` (+ its test); forms editor inline spec →
    `crate::artifact_kind()` (unused `ArtifactKindSpec`/`OsMediaCapability` imports dropped); fem `document_artifact_kind()` 2d/3d (`2d.fem`/
    `3d.fem`, FEM_xD_SCHEMA, io media type, stdio lists copied from the results kinds) stitched before the results kind + `OnArtifactKind`
    activations; trinity rewriting io id → `crate::artifact_kind().id`.
  - `wp-w4/w4-kind-identity-gate.py` (**dry-run 8 pending / 0 bad**, land AFTER the spec script): describe-time gate in
    `🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` — static law `kind_identity_faults` over `descriptor_kind_identity_apps`, and
    `owned_codec_census` on the SAME compiled component (execute_describe_owned now returns runtime + CompiledHandle; no second compile):
    ≥ 1 declared kind ⇒ ≥ 1 owned codec row, other per-kind faults printed; language-neutral fixture `🧫️fixtures/🪪️kind-identity/🔣️.json`
    (5 cases) + unit test `kind_identity_law_matches_the_fixture`. Written, NOT compiled (landing = L1, T3).
  - Open risk for the gate: stdio's guest answers `stdio.xml` with "resolves more than one app of the same role" (two editors own the
    schema) — the gate prints it but only refuses 0 owned; tolerant stdio census running to confirm stdio owns ≥ 1 row.
- 22:00:46 **chain c3 ended by SIGTERM** (not by itself): `release-modules` (`nx run-many -t materialize-release`, 63 min in, at playbook
  component-release) exited 143 and every chain process is gone (no further chain-final line; hub hold 9448 untouched, 7800 still READY). The
  shared RELEASE plugin-module root is therefore partially re-materialized (dag/fem/gis/vcs/wfc/playbook/sequence… not done) and
  `w3-release-root-check` never ran — only matters for release-variant serves; rerun `framework-plugin-web:support-release` +
  `run-many -t materialize-release` + `w3-release-root-check.ts release` when the wasm lane is free (or with the next all-34 chain).
- 22:0x `w4-chain.sh` readiness poll now reads the hold's append-only `hold.txt` (zsh -n ok; the status.txt race is documented in the header).
