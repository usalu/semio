# WP-H14 — Hub Performance + Operations (successor of H12)

Slice: H14 (session 14, 2026-09-27 18:3x). Coordinator = main chat. Rules: `📓️session-14-preamble.md`. Ports: hubs 8160–8169,
serves 6660–6669 (keeps H12's hub 8161, pid 79070). Private cargo target: `.tmp-ticket/wp-h14/target` (build-dir
`build-fleet-b`, native lane only). Captures: `wp-h14/generated/` (expendable). Durable data/logs: `.🧬semio/🌐hub/s14-h14-*`.
Handovers: [📓️wp-h12.md](📓️wp-h12.md), [📓️wp-h10.md](📓️wp-h10.md), [📓️wp-db1.md](📓️wp-db1.md),
[📓️acceptance-s13.md](📓️acceptance-s13.md) §2. Neighbours: H13 (hub correctness/security, fuzz row 2.13, interpreter
cancellation), R10 (launch generator, harness productization), W4 (chain, 7800).

## Session 15

Successor (2026-09-29 19:5x). Predecessor's last step (hold 12, 18:13) was complete and green; no h14 lane ticket, hub or serve alive;
nothing half-applied in the tree → nothing to reconcile. Ports 8160–8169 / 6660–6669. Target `.🧬semio/🌐hub/s14-h14-target`.

| # | Item | Status |
|---|------|--------|
| 1 | P1 hub creation of multi-subset + hosted kinds (LB2 finding) | **LANDED 20:56** hold A (priority stamp 183255): check EXIT 0, trusted_catalog 60/61 (red = pre-existing census, REGEN), os-hub bin 172/172 |
| 1b | Creation catalog 64-kind cap (t6 ≈ 90 kinds → 409) + silent omissions | **t6-queue row 34 (ROUND-4 FIRST)** `h14-catalog-bound.py`, 53 hunks / 18 files: complete by construction (1024 = open-target ceiling, bytes derived, hub const assert) + typed `withheld` reasons en/de through worker + React shell; dry-run clean on scratch = live + A; TS twin/Ajv/copy 0 disagreements (bun); Rust half → L1 train |
| 1c | ONE definition `ArtifactDialect::covers` in io schema (guest + hub) + guest fixture law | after T7d lands; next free t6-queue row (35 = S19, 36 = per-standard) |
| 1d | Creation entries per (kind, standard) (coordinator decision 21:1x) | **t6-queue row 36** (right after 34) `h14-creation-standard.py`, 80 hunks / 37 files; overlay `s15-ov36`: kernel/mcp/wgpu rc 0; hub fixes → re-proof `s15-ov36d` pending; dry-run clean on scratch = live + A + 34; bun/Ajv/twin checks 0 disagreements; Rust half → L1 train. stdio owner rows / pdf mapping / dialect-aware genesis → LB2 p20 (census above); hub genesis-coordinate switch after p20 |
| 2 | Live re-measures on t6 (boot, residency, creation latency, codec laws) | waits for T6 HUB READY; creation probe `wp-h14/h14-t6-creation.ts` ready (generation from `/trusted-catalog/plugin-modules`, kinds derived from the bundle by the one rule; transpile ok, decision logic exercised on p33: 65/65 creatable incl. json/xml) |

#### Session 15 — stdio creatability census (for LB2, coordinator 21:0x)

Measured 21:0x over the W4 precheck family descriptors (`.🧬semio/🌐hub/s14-w4-precheck/stdio-*/descriptor/🔣️.json`, the chain's
fresh describes with `hostedArtifactKinds`), the committed stdio core descriptor (`✏️s/🔌️plugins/🗄️stdio/🔣️.json`) and the stdio
linked codec registry (`✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json`, 29 receipts; the hub publisher binds
stdio = linked closure only, `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `trustedBootstrapHostedKindsV1` / `linkedCodecRegistry`):
- **36** stdio kinds have an editor surface; **29** have an owner codec row → a hub open target.
- **7 have NO owner codec row** → the family's hosted row is `unbound`, no open target: not openable, not creatable on any hub:
  `s.stdio.binary`, `s.stdio.bmp`, `s.stdio.epw`, `s.stdio.gif` (87a → `stdio.gif`, 89a → `stdio.gif.89a`), `s.stdio.ifc`
  (2x3 → `stdio.ifc.2x3`, 4 → `stdio.ifc`), `s.stdio.semio`, `s.stdio.wav`. An owner row with no app in the `stdio` component would
  also fail the hub's guest verification of that row (the owner component answers `codec.pack-schema-hash`), unless it is linked
  natively.
- **dwg**: ONE schema `stdio.dwg` for `ac1018/*` and `ac1024/*` → the hub offers one entry per standard (row 36), but guest genesis
  refuses (p17 `artifact_codec_owner`: two whole-standard owners of one schema) → needs dialect-aware genesis (`codec.genesis` given
  the entry's dialect; guest ABI).
- **pdf**: hub row `(s.stdio.pdf, stdio.pdf)` but every pdf app (1.4 `*`/a/x, 1.7 `*`/a/e/h/ua/vt/x) opens `stdio.pdf.1.7` → genesis for
  `stdio.pdf` answers UNOWNED (LB2 p12 "pdf native mapping" follow-up); both standards share `stdio.pdf.1.7` → also dialect-aware
  genesis.
- json/xml (stdio core) and the families' jpg/svg/tiff/step/docx/pptx/xlsx/zip multi-subset kinds: hub side fixed (hold A); guest
  side = p17 (T7d).
- Hub pointers: creation rule `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` (`artifact_creation_selection`,
  `owner_preferred_creation`, `most_general_dialect`, `hosted_codec_targets`), genesis `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs`
  (`materialize_genesis` → `GuestArtifactCodecBinding::genesis` → `codec_genesis_observed(schema, document_id)`), live probe
  `wp-h14/h14-t6-creation.ts` (kinds from the bundle by the one rule, POST to ready, open plan package/surface check).

### Session 15 log

- 19:5x read preambles 15/14, fleet-15 tail, t6-queue rows 3/3b, LB2 §15. Reconcile: nothing in flight.
- 20:0x–20:2x **root cause (code + p33 catalog + W4 precheck family descriptors, `s14-w4-precheck/stdio-*/descriptor/🔣️.json`):**
  (a) `owner_preferred_creation` demanded exactly one editor per tier → json/xml (stdio owns `rfc8259/*` + `i-json`, `1.0/*` + `valid`)
  and every family multi-subset kind (jpg, svg, tiff, step, docx/pptx/xlsx, zip, ifc, semio, pdf) never creatable;
  (b) **every hosted creation failed before genesis**: `materialize_genesis` resolves the codec by the selection's (HOST) identity,
  but a family record carries `nativeCodecs = []` (publisher: hosted-only package) and the catalog registered codecs only under
  owner identities → `descriptor identity is absent`; the same for Check In / checkpoint of any document whose descriptor names the
  host (clients: descriptor owner = the package whose component executes it, e.g. MCP remote `lease.package == descriptor.owner`);
  (c) `artifact_creation_catalog` looked the kind label up only in the host's own declarations with `?` → the FIRST hosted kind
  (family descriptors carry only `{id, schema, owner}`) turned the WHOLE creation catalog into `None` (409 "no verified creatable
  artifact kind"); (d) `resolve_document_open` without a requested surface (the semio MCP workspace sends `None`) refused every
  multi-subset document (two editors match). (e) Separate: t6 will offer ≈ 90 creatable kinds (58 non-stdio on p33 + ≈ 32 stdio)
  but the creation catalog wire contract caps at 64 kinds / 64 KiB (Rust `space-artifact-creation-v1` + TS twin) → `validate()`
  false → the whole catalog 409 on t6 even with (a)–(d) fixed. Multi-STANDARD kinds (dwg ac1018/ac1024, gif 87a/89a, pdf 1.4/1.7,
  ifc 2x3/4) stay ambiguous by the same rule as the guest (creation names a kind, not a dialect).
- 20:2x **set `wp-h14/h14-creation-rule.py`** (+ `.json` hunks, generator `h14-creation-rule-gen.py`; `--dry-run/--write/--revert`,
  round trip byte-identical, backups `.🧬semio/🌐hub/s14-h14-backup/creation-rule/`), hub-only + one test-only fixture:
  ONE rule `most_general_dialect` / `dialect_covers` (= LB2 p17 guest `artifact_codec_owner` narrowing) with the shared
  language-agnostic fixture `🧰️framework/🔨️modules/🚪️io/🧫️fixtures/🌳️most-general-dialect/🔣️.json` (13 cases, both readers);
  creation tiers by the descriptor's hosted rows, most general editor within the deciding tier; default open surface = most general
  of the role; hosted codec identities registered under the HOST (owner's linked native codec, HOST component for genesis/guest
  calls; unlinked hosted rows become verification rows of the host component; progress `rows` counts them); creation catalog labels a
  hosted kind from its owner's declaration and skips (never aborts on) an unlabelled kind. Laws: `the_most_general_dialect_rule_answers_the_shared_fixture`,
  `a_hosted_multi_subset_kind_is_created_and_executed_by_its_hosts_most_general_editor` (loaded catalog, linked + unlinked),
  `creation_prefers_the_owners_editor_over_a_hosts` (extended). Tree left UNCHANGED (applied only inside the hold, revert on red):
  hold A `wp-h14/h14-lane-s15a.sh` queued native 20:26:41 → `.🧬semio/🌐hub/s14-h14-logs/s15a-lane.txt`.
- 20:28 coordinator PRIORITY → hold A re-queued (stamp 20260929183255, own waiter 63910/63913 stopped first); verdict to main 20:3x:
  hub-only cannot serve > 64 kinds (os-kernel body type + both client twins refuse) → T6 live creation checks need row 34 + a hub
  refresh; POST creation with the generation from `/trusted-catalog/plugin-modules` works meanwhile.
- 20:3x–20:5x row 34 (design approved: complete by construction, no paging — paging adds cross-page generation races for ≤ ~300 KB);
  grown on coordinator request to name every unoffered kind with a typed reason (`ambiguous-editors` / `unlabelled` / `unpresentable`)
  instead of dropping it: wire (JSON Schema `anyOf` nonempty, Rust enum + struct + validate, TS twin), 5 new vectors (+`schemaValid`
  where JSON Schema cannot express order/disjointness), hub `artifact_creation_choice` + catalog, `💻️os/🟦️.ts` worker wire (its own
  hard-coded `> 64` too), store worker, ShellHost projection + `ArtifactCreationCatalogNotice` list + copy fixture/schema en/de, laws
  (hub capacity + loaded-catalog withheld law, TS vectors + capacity, engine-contract i18next render law), test literals.
  Checks on the scratch (`$SCRATCH/r34` = live + A + row 34): `bun build` of 6 TS files ok; Ajv + TS twin over all 15 catalog vectors
  and 2 raw rows agree with the fixture; copy fixture valid against `ArtifactCreationProgressUiV1`; bounds 2 412 B/row, 2 470 314 B.
- 20:43–20:56 **hold A** (`s15a-lane.txt`): applied 15 hunks; check EXIT 0 20:46 (warnings present, none in changed regions);
  trusted_catalog 60 passed / 1 failed (the committed-descriptor census, 79 stale family descriptors — pre-existing, REGEN); os-hub bin
  172/172; kept (new laws green). Landing row added. Note: once any hub is rebuilt from this tree, even a p33-like catalog offers 65
  kinds (json + xml now creatable) → the 64-kind cap 409s the catalog there too until row 34.

- 21:0x–21:3x coordinator: multi-standard kinds must be creatable → **row 36**: request gains `dialect` (existing
  `SpaceArtifactCreationDialectV1`), hub `artifact_creation_selection(kind, dialect)` / `artifact_creation_choice(kind, standard)` /
  `artifact_creation_standards`, catalog entries per standard (label + " {standard}" when a kind spans several), withheld rows carry
  the standard; all request producers updated; census law over committed descriptors. Stdio census (above) relayed: 7 kinds have no
  hub open target, dwg/pdf genesis need dialect-aware genesis → LB2 p20 (LB2 relay: hub passes `parent_dialect.to_coordinate()` as
  `codec.genesis` identity — hub-only switch after p20). Scratch `$SCRATCH/r36` = live + A + 34 + 36: 78/78, round trip clean, 22 TS
  files `bun build` ok, Ajv + twin over all catalog/request vectors + raw rows 0 disagreements, worker generation fixture strict-Ajv
  valid, operation fixture digest recomputed (`13d07dd7…`) and verified.
- 21:1x–21:26 coordinator: overlay proof of 34 + 36 → `wp-h14/h14-overlay-r36.sh` (setup 21:17–21:25: APFS clones of
  framework/plugins/hub + root manifests, build outputs pruned, 6.2 GB; hold A present, rows 34 + 36 applied 53 + 78 edits, 0 problems);
  `proof s15-ov36` queued on the overlay lane 21:26 (behind s20; both slots s20). Captures `s14-h14-logs/s15-ov36*.txt`. Overlay to be
  dropped (and recorded) after the proof.
- 21:52 proof `s15-ov36` (21:30–21:52): kernel rc 0, os-mcp rc 0, renderer-wgpu rc 0; hub + hub-laws rc 101 — `E0063` row 34's
  capacity law pushes a withheld row without row 36's `standard`, `E0308` the census law typed `descriptor_open_targets` rows as
  `TrustedBundleOpenTargetV1` (they are `TrustedDescriptorOpenTargetV1`). Row 36 regenerated (79 hunks: + the literal's `standard`,
  `Vec<_>` map), reverted/re-applied in scratch and overlay, dry-run clean. Hub-only re-proof `s15-ov36b` queued overlay 21:53
  (behind cd1, s20 ×2).
- 22:21–22:44 `s15-ov36b` (22:20): hub check rc 0, hub laws 67 passed / 3 failed — the 3 read committed descriptors, stale in the 21:17
  overlay; fresh descriptors copied in (69/69 identical to live, linked registries unchanged). `s15-ov36c` (22:40, the 3 laws): old editor
  census GREEN; withheld law red only because the test fixture's schema `…@1` is not an identity (row unpresentable) → row 34 law makes the
  fixture schema an identity; new (kind, standard) census: 8 of 112 entries ambiguous — gif 87a/89a, ifc 2x3/4, pdf 1.4/1.7, semio v1 (each
  editor opens every schema of its hosted kind by dialect naming → the same dialect twice per standard) and block `kit.catalog@1`
  (block2d/3d/5d all declare it). Row 36 refined (80 hunks): creating editors = editors whose app's own document schema is the target's
  (`AppIo.artifact_schema`, the schema guest genesis answers; census over the fresh 🔣️.json: 153/153 kind-opening editors keep one);
  census law counts creating editors only; hosting test helper sets the fixture apps' own schema. Re-proof `s15-ov36d` (hub check + all hub
  laws) queued overlay 22:44.
### Session 14c

Successor agent (wave B, 2026-09-28 21:1x, WINDOW 3 OPEN). 7800 READY on `s14-w4-catalog-p24` (24 packages). My hub/serve ports
8160–8169 / 6660–6669. Binary for every live run: copy of the chain's p24 `os-hub` (build-dev, 20:59) →
`.🧬semio/🌐hub/s14-h14-bin/os-hub-p24-2059` (sha256 `9731d5f4…` after re-sign).

| # | Item | Status |
|---|------|--------|
| 1 | 7800 "616 s" cold boot | **closed** (coordinator: W4's chain polling race; 7800 READY in 8.0 s). Own numbers (debug `os-hub`, p24, 583 MB components read + SHA-256/BLAKE3-hashed serially per package): cold ready 61.6 s (load ~40) / ~20 s (lower load), warm ready 48.9 / 34.7 / 84.8 s (load 40 / 40 / 54), background pinning 201 s cold (puzzle codec 112 s), +50 ms warm |
| 1b | Coordinator 21:2x: commit cost vs document size + lost Ack (H13 hub 8010) | **measured**: hub CPU per 256-envelope batch flat 295 → ~370 ms over 0 → 17 k edits; wall 0.7–5 s = ~13 serial F_FULLFSYNC barriers per batch (WAL + 12 index-run replacements) under machine I/O; commit CPU 56 % in the unused `vcs` version graph. Lost Ack = 30 s frame deadline dropping a frame whose batch had reached the engine |
| 1c | (a) Ack every committed batch (frame deadline = admission only) + bin law | **landed + proven**: hub check EXIT 0, bin laws 4/4 PASS 23:14 (hold 3) |
| 1d | (b) hub builds kernel-db without `vcs` | **landed** 21:47, checks green 22:07/22:14/23:01 |
| 1e | (c) index owned appends + level folds; (d) index flush after the receipt; (e) outbox/commit_log removed, receipts bounded | **landed** 22:05, **proven** holds 7 + 8: `seventy_thousand_single_envelope_commits_stay_writable_with_bounded_index_runs` (70 016 commits, ≤ 300 runs), `owned_appends_fold_full_runs_into_levels_and_every_entry_still_resolves`, `a_fold_beyond_one_read_credit_is_skipped_…`, `index_runs_follow_the_receipt_and_a_reopen_refills_them_from_the_wal` (crash/reopen), `applied_receipts_keep_a_bounded_window_…` all ok |
| 1f | P1 hello of a long document refused | **landed + proven** (holds 8–9, 07:32–07:42): (A) streamed tail through ONE WAL reader + one-page handoff, stall-bounded hello deadline (re-armed only on a renewal) — db_sync 44/44 incl. **50 k-edit fresh replica within one frame of backing**; (B2) frontier-less hello from a `closed-browser-actor` plan resumes at the plan checkpoint baseline; (B1) declared `features.checkpointPolicy` (1 024 edits / 512 KiB, `OS_HUB_CHECKPOINT_POLICY_*`) → auto Check In through the ordinary claim/fold/fenced publication **as the committing author**, traced `server.document.check-in` started "checkpoint-policy …" — hub check EXIT 0, bin laws 17/17, Check In laws 7/7, hub TS 18/1 skip. en/de check 07:5x: no user surface shows the policy (only `/readyz`, README, event counts), so there is nothing to label; root `verify interactivity p1z` source gate still names the pre-stream design |
| 1g | LW1 kernel `retained_clone` reds (coordinator 07:0x) | **root-caused 07:3x**: (i) `production_snapshot…` = O(bytes) retirement, not a livelock — `Vec<T>` retired one element per step even without drop glue (2 MiB `Vec<u8>` → ~6.3 M one-item pump turns); (ii) `paged_list … close_turns > 1` = the law's error (a spent cursor closes in one step). Set `wp-h14/h14-retire-pages.py` (kernel `🏪️store` = guest: page-per-step retirement for no-drop-glue collections, law, fixture + schema 9 → 11 cases, paged law `== 1`), Ajv-validated fixture on a scratch copy, dry-run clean → first post-chain train (L1) |
| 1h | Laws on process-global state deterministic under the default runner (coordinator 07:4x) | **landed + proven** hold 10 (07:49–07:53, test-only): 8 `vcs_integration` admission laws process-isolated (`TEST_LOCK` removed), the history + round-trip laws shut their database down; kernel-db lib 729/729 default runner, `db_engine::` 136/136 ×3, 8-way stress 0 failures (was 4/40, 5/24) |
| 1i | C13 relay 16:5x: p33 document socket idle ~2 min closed `4401` on its next Commands frame | **root-caused + landed** (hold 11, 17:38–17:47): cause = every Check In (and any catalog publication) revoked every other open plan socket within one authorization tick — not an idle expiry; hub live authority no longer re-checks bootstrap facts (`409 socket-grant-stale` at consumption); check EXIT 0, plan/socket laws 36/36 incl. the 10-idle-minute law, os-hub bin 171/171, Check In laws 8/8. Rust twin (both Rust actors re-planned every live socket ~30 s after admission): guest set `wp-h14/h14-rust-link-live-socket.py` = T6 row 17 (L1) |
| 2 | Idle-release / residency-LRU under real memory pressure, 24 packages (row 2.7) | **measured 23:5x** (hub 8161, `OS_HUB_GUEST_RESIDENCY_BYTES` = 128 MiB, `residency-watch --rounds 2`, `s14-h14-logs/residency-8161-p24-128mib-2.txt`): resident guests held at 6 / **133.2 MB ≤ 134.2 MB budget** both rounds; TinyLFU keeps a stable set — compiles 31 → 21, hits 34 → 44, bypassed 22 → 20, released 3 → 1; RSS 210.8 → **peak 629.5** → **settled 27.7 MiB** after 90 s idle (601.8 MiB released); pairs agree; 68/72 creations (2d/3d.generation genesis trap ×2 rounds = S19 `gen-archive-load`, T1) → verdict FAIL only on those. Finding: peak RSS ≈ 4.9× the residency budget — the budget charges component bytes + measured footprint (0 today; the T4 codec origin makes it real), interpreter working memory of a running creation is uncharged |
| 3 | Warm restart + `/readyz` timings, creation latency per package | warm ready above; creation latency per kind (default budget, round 1): 1.0 s computation.sequence … 5.1 s 2d.block, 12.6 s data.program, 22.7 s 3d.cad, 26–53 s gis/vcs/stdio md+csv, 56 s 3d.puzzle, 82 s 2d.puzzle, 126 s 5d.puzzle = baseline for the T4 codec-origin set |
| 4 | README/metrics parity law (row 4.9) | done in session 14 (7/7 PASS) |
| 5 | channel-version census vitest law bounded | **landed** 22:09: census reads the source files under the roots of the pin + registered consumers; vitest 5/5 in 7.5 s, check 0 findings, os tsc 0 |
| 6 | Codec-origin set (T4) dry run after G12's plugin-host change | **clean** 22:2x: plugin-host 3-way merge clean, owned-instance laws + hub footprint `replace` |

#### Session 14c log

- 21:1x read preamble 14 (rules 1–24, 14b, 14c), AGENTS.md, fleet tail, this report, `📓️wp-h13.md` §14c (kernel-db: WAL readers,
  retirement wake, close-ring routing are H13's, landed). Load 34–41, swap 9.7/11 GB.
- 21:13–21:19 **boot timeline** (`wp-h14/h14-boot-timeline.ts`: APFS-cloned root `.🧬semio/🌐hub/s14-h14-boot-p24`, every hub line
  stamped ms-since-spawn, `/readyz` every 100 ms, admin `catalog` every 250 ms, per-package phase transitions; captures
  `.🧬semio/🌐hub/s14-h14-logs/boot-timeline-1-{0,1,2}.{log,json}`), port 8160, 3 boots:
  cold ready 61 606 ms / settled 262 672 ms / exit 2 233 ms; warm ready 48 905 ms and 34 690 ms (settled +50 ms: verification memory
  pins every row), 0 refused. Cold: packages read + dual-hashed one after another (0.5–6.7 s each; stdio 6.7 s, gis 5.6 s, vcs
  3.3 s incl. their linked codec rows), `CatalogResolved` 41.55 s → `server.boot` 61.54 s (**19.9 s cold-only gap**, warm 0.2 s), then
  background verification 201 s (compile 1–3 s + codec per package; puzzle `GuestCodecExecuting` → 500 M fuel = 112 s).

- 21:2x **commit growth** (coordinator item): own hub 8161 (`wp-h14/h14-hub.sh`, p24 clone `.🧬semio/🌐hub/s14-h14-hub-8161-p24`,
  binary `os-hub-p24-2059`), probe `wp-h14/h14-commit-growth.ts` (one writer + one observer socket, chained 256-envelope batches of
  s.note.note, Ack latency + hub CPU per batch via `ps cputime`), runner `h14-growth-run.sh` (+ `sample` of the hub 3 s every 20 s,
  folded by `h14-sample-tree.py`). Runs (captures `.🧬semio/🌐hub/s14-h14-logs/commit-growth-{1,2,3}.txt`, `samples-{1,2}/`):
  run 1 40 batches 2.5–7.2 s wall (load 40, background verification still interpreting puzzle), run 2 60 batches 0.6–10.9 s wall
  (load 55 → 30), run 3 67 batches: **hub CPU/batch 295 ms (0–2.5 k edits) → 350–397 ms (2.5–17 k)**, wall 0.7–5.0 s at the same
  sizes; batch 67 refused `hub.unavailable` (socket DoS budget: 128 envelopes/s sent vs 60/s refill — the declared transient path).
  Clean commit profile (`samples-2/s3.txt`): **vcs version graph ≈ 56 %** (`VcsVersionGraph::record_change` per envelope → store
  `Apply` → `reproject`/`replay_mutations` over the 64-edit window, `edit_digest` = JSON `to_string` per edit), WAL CRC32C 13 %,
  rest alloc/memmove; wall ≫ CPU because `FsDbIoExecutor::replace_step` makes every index run write durable (file fsync + drive
  `F_FULLFSYNC` + rename + dir fsync), 12 runs per 256-envelope batch + the WAL sync. `checkpoint_document`/`merge_base`/`head`
  have no production caller (db tests only) → the graph is pure overhead on the hub. **Lost Ack root cause:** H13's writer socket
  closed exactly 30 s after batch 23 was sent — `tokio::time::timeout(DOCUMENT_SOCKET_FRAME_DEADLINE, handle_client_frame(..))`
  drops the frame future while the engine still commits the batch (head 6144), so the socket closes `1013 frame-deadline` without
  the Ack. **O(document) found:** `IndexHandle::append_run` lists every run of the document (`kind_run_ids` → `list_runs`) on
  each append; runs are never merged (full 64-entry runs never pair), so a document holds N/64 runs per kind and the list output
  is capped at `DB_IO_LIST_ITEMS` = 4096 → past ~65 k single-envelope commits (4 runs / 64 commits) every run write fails, the
  backlog passes `INDEX_BACKLOG_ENTRIES_MAX` and every `submit` returns an error AFTER its WAL write was durable. Engine memory
  grows with history: `outbox` (never drained on the hub), `commit_log`, `applied_receipts`.
- 21:4x coordinator GO: (a) frame deadline = admission only, (b) hub db without `vcs`, (c) index append without listing + bounded
  run count (law > 70 k single-envelope commits), (d) index flush off the Ack path + crash law, (e) drain outbox/commit_log/
  receipts; H13 keeps submit credit/admission — wait for its region relay before editing near `submit`.
- 21:4x **(a) + (b) written and applied** (hub-only, H13's concurrent `undeclared_batch_refusal` hunk in `handle_client_frame`
  preserved): `handle_client_frame` returns `ClientFrameStepV1` (`Continue`/`End`/`Commit(AdmittedCommandsV1)`); an admitted batch
  leaves the deadline holding the document write gate; `commit_admitted_commands` submits, relays and always sends the Ack;
  `document_socket_frame_deadline(state)` (law override via `TestLiveGate.socket_frame_deadline`), commit-phase test gate.
  Law `a_batch_committed_past_the_frame_deadline_is_still_acknowledged` (bin-unit quick: deadline 200 ms, commit held 600 ms after
  the engine committed, Ack Accepted + relay, socket keeps serving a second batch). Hub `Cargo.toml`: kernel-db (normal + dev) without
  `vcs`. Native lane queued 21:47 (3rd, behind l1 + g12).

- 21:5x (c)(d)(e) written: `wp-h14/h14-index-levels.py` (db_index levels/owned runs), `h14-index-off-ack.py` (backlog via owned runs, runner
  `maintain_index` after the reply, pre-WAL backlog bound, outbox/commit_log/DrainOutbox removed, receipts window), `h14-index-laws.py`.
  Hold 2 (`wp-h14/h14-lane-db.sh`, queue stamp kept, applied inside the hold, restore-on-red): kernel-db check EXIT 0 22:06, no-vcs EXIT 0
  22:07, semio-hub check EXIT 0 22:14 (incl. the Ack set); db_index + db_artifact laws 92/96 — 4 new laws red on the harness: the memory
  backend holds 64 WAL/index owners (1 034 buffered commits failed at WAL close; 300 Fsync commits refused), typed lookups carry a fixed
  8 192-grant fuel (an old key behind two level-3 runs exhausts it — pre-existing: ~117 level-0 runs did too; no production reader) →
  laws moved to the fs backend with `Os` durability, lookups via the handle with an explicit budget + `verify`. Hub bin laws: test build
  red on a peer's stdio tiff/bmp (`ArtifactKindSpec` has no field `label`), not ours.
- 22:0x coordinator P1 (H13 capture `transient-probe-8010-3.txt`): hello of a 13 201-edit document refused. Root cause
  (`🔄️sync/🦀️.rs` `replay_sync_state_retained`): the hello decodes every command past the replica's head into one Vec, charged
  cumulatively (65 536-item ledger ≈ 8 k envelopes), then sends ONE Commands frame; a fresh client (null frontier) always gets the whole
  history. A db `SnapshotPub` cannot bound it for app documents (their envelopes put nothing into the db pathmap state) — told main;
  the valid snapshot is the guest's verified checkpoint (CAS pair + RebootstrapRequired). Clients: React worker `requireArtifactRebootstrap`
  (`🏪️store/👷️worker/🟦️.ts` ~l.4643) and the wgpu shell/store sync (`🏪️store/🔄️sync/🦀️.rs` l.3255/4804) handle RebootstrapRequired.
- 22:1x (A) written (`h14-hello-tail-stream.py`): pass 1 hashes every command and decodes none (O(1) memory); `DatabaseSyncHelloTail`
  streams frames through page futures (`database_sync_hello_tail_page`: resume at the segment of the first unemitted command, skip what the
  chain covers, hash, decode what the replica lacks, stop on the first commit boundary after 256 envelopes); frame backing charged while
  out, released on close (`database_sync_hello_returned_frame_credit` counts Commands frames); `DatabaseSyncHelloFollowUpStep` (`Waiting`
  parks the hello on the page future's waker instead of spinning). Channel-version census bounded (item 5) — vitest/check/tsc green 22:10.
- 22:2x codec-origin dry run after G12's plugin-host change: clean (3-way merge). 2d/3d.generation genesis trap = S19 `gen-archive-load` (T1).
- 22:3x hub 8161 restarted with `OS_HUB_GUEST_RESIDENCY_BYTES` = 128 MiB (warm ready 84.8 s at load 54); residency-watch 2 rounds running
  (`s14-h14-logs/residency-8161-p24-128mib-1.txt`).

- 22:42 **machine reboot** (coordinator): every process died (hub 8161, hold 3 had not started). Torn-write check 22:46: every applied
  set present exactly once (ack set, hub Cargo.toml, index levels, index-off-ack; marker counts 1), the streamed tail NOT applied; nothing
  to revert. /tmp lane state was wiped.
- 22:47 hold 3 (`h14-lane-db3.sh`, load 77 → 47): applied the stream set, kernel-db `--lib --tests` check **red on MY post-hold-2 law
  fixes** (test-only: a shadowed `control()` helper, an untyped range, an `Arc` moved into `create`) → the script restored the sync files
  as designed; fixed the three test errors at once (test-only, kernel-db lib unaffected). The laws then ran on the fixed tests (sync
  reverted): **db_index + db_artifact + db_sync 135 pass / 3 fail** — the 3 new artifact laws (index after receipt + reopen refill, receipt
  window, **70 016 single-envelope commits on fs**) passed every assertion and failed only in the test helper's WAL close (`Os` durability
  leaves pending records: "force_flush is required before close") → helper flushes first. semio-hub `--lib --bins --tests` **EXIT 0
  23:01** (the peer's stdio tiff/bmp break is gone).
- 22:5x stream follow-up `wp-h14/h14-hello-tail-fix.py` (applied after the stream set, simulated on scratch copies: clean): the first page
  reads from the first retained segment (a compacted WAL has no segment 0 — `cursor.segment: Option`); the frame emitting the last
  command carries the welcome's commit sequence (transactions without a command after it count in the server frontier — the React worker
  `equalFrontiers` and the Rust store `frontier_reaches` would otherwise never finish catch-up); dead `close_tail`/`retire_vec` removed;
  law variant with a trailing snapshot-marker transaction. Both clients already accept a multi-frame tail (React worker: every Commands
  frame sets `state.frontier` and `finishCatchupIfReady` waits for the welcome frontier; store sync: `frontier_reaches`).
- (B) findings: the React worker's contract (`forgetMountedDocument` doc, `🏪️store/👷️worker/🟦️.ts`) says a frontier-less hello is
  answered "with the tail from its active checkpoint" and the child is seeded from the checkpoint pair — the hub does NOT do that today
  (it always passes the client's frontier to `db.hello`, so a fresh client gets the whole history on top of the seeded pair). The
  seeding happens only for `closed-browser-actor` leases with a checkpoint. React worker, wgpu directory client
  (`📇️directory/🔌️client/🪢️canonical-checkpoint-pair`) and the MCP workspace (`🌉️mcp/🏠️workspace/🔗️remote/🧩️pair`) read the pair route.

- 00:10–07:0x (usage cut 00:1x, kernel panics 00:31–00:48, external sweep 01:14 deleted `wp-h14/generated/` incl. hold 4/6 captures).
  07:09 resumed: reconcile — A present in the tree (every marker once, dry-runs "done"), B1/B2 absent (hold 6 never applied), no torn
  hunk. T5's `@emoji` codemod changed doc lines my prepared anchors quoted → scripts rebased (`/// @emoji ` → `/// `), dry-runs clean;
  backups of the pre-T5 scripts under `.🧬semio/🌐hub/s14-h14-backup/`. Captures from now on under `.🧬semio/🌐hub/s14-h14-logs/`,
  target `.🧬semio/🌐hub/s14-h14-target`. Residency 128 MiB watch (row 2) survived (hub log dir).
- 07:2x LW1 reds root-caused (row 1g). The policy Check In now emits its own `server.document.check-in` started record naming the
  policy (coordinator: attributed to the committing author, an event, thresholds declared in the schema).

- 07:20–07:27 **hold 7** (`h14-lane-p1-land.sh`, capture `.🧬semio/🌐hub/s14-h14-logs/hold7-1.txt` + `hold7-db-laws.txt`): kernel-db
  check default + no-vcs **EXIT 0**; db laws 274 pass / 2 fail — green: seventy_thousand (70 016 commits), owned_appends, a_fold_beyond,
  index_runs_follow (receipt before index + WAL refill), applied_receipts_keep, streams_exactly, derives_the_server_frontier; red:
  (i) my `a_fifty_thousand_edit_document_streams…` → `Timeout("database sync hello deadline")`: the hello's 30 s deadline bounded the
  WHOLE session incl. streaming, and each frame's page future re-read its segment from the start (~7× over a 50 k tail); (ii)
  `db_engine::…artifact_history_empty_and_two_batch_replay_are_deterministic` → kernel `🏪️store` Drop "artifact store reached Drop without
  its exact terminal-empty shallow-shell witness" (process-isolated; default `vcs` path, the database dropped without shutdown; kernel
  store changed at 01:40 by the window-3 trains — not an H14 file; relayed). B1+B2 applied → hub check **red on my B1 only** (the two
  test `HubState` initializers lacked `checkpoint_policy`) → restore-on-red put all five hub files back (verified: 0 markers).
- 07:2x fixes: (A2) `wp-h14/h14-hello-tail-reader.py` — ONE tail reader walks the WAL once and hands frames over through a one-page slot
  (`DatabaseSyncHelloTailHandoff`, reader parks while the slot is full, the driver wakes it when it takes a frame), and the hello deadline
  is a STALL bound (`progress_deadline_ms`, renewed by every produced frame; the deadline callback re-arms at the renewed time; retry
  expiry reads it; the now-unused `deadline_ms` field dropped). B1 script gains the test initializers. Note: root `verify interactivity
  p1z` (source-shape gate) is red on the live tree for reasons older than my change (it looks for `database_sync_hello_allocate_envelope_vec`,
  `returned_generation.fetch_update`, … that the code no longer has) — not in any chain gate; a rewrite of that gate for the streamed
  design is follow-up work. Hold 8 queued 07:29 (`h14-lane-p1-land2.sh`).

- 07:32–07:40 **hold 8** (`h14-lane-p1-land2.sh`, `.🧬semio/🌐hub/s14-h14-logs/hold8-*`): A2 applied → kernel-db check default + no-vcs
  EXIT 0; db laws 268 / 8 — **50 k-edit law PASS**, reds: `retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service`
  (my callback re-armed whenever the renewed deadline was in the future; the law drives the callback directly before the deadline and
  requires expiry) + 7 `db_engine::vcs_integration` laws ("vcs operation capacity exhausted" then a PoisonError cascade; green in hold 7).
  B2 + B1 applied → semio-hub check **EXIT 0 07:35**, bin laws **17/17**, integration-fixtures Check In laws **7/7**, hub TS long
  observability + integration 18 pass / 1 skip (the lane's first vitest call passed root-relative filters → "no test files"; rerun from
  the package script). **LANDED** — relayed to main 07:4x.
- 07:40:56–07:42:44 **hold 9** (`h14-lane-hold9.sh`, `hold9-*`): `h14-hello-deadline-armed.py` (`deadline_armed_ms`: re-arm only when a
  frame renewed the deadline after arming) → checks EXIT 0/0, **db_sync 44/44**, vcs_integration **11/11** on one thread and
  `artifact_history_empty_and_two_batch_replay_are_deterministic` **1/1** alone (both hold-8 reds = parallel interference on
  process-global state, not regressions; the Drop-witness panic of hold 7 does not reproduce alone), semio-hub check EXIT 0. New warnings
  in my files: none (the kernel-db warnings sit in history replay, runner retirement and catalog work).
- 07:4x coordinator: laws on process-global state must not depend on the runner. Root causes: (i) `vcs_integration::retained_tests`
  claim from the process-global `VCS_ADMISSION` table (64 slots) and assert exact counts / slot identity, but their module `TEST_LOCK`
  cannot exclude the engine laws that commit through the default `vcs` graph (`🗿️artifact` commit → `record_change` claims a slot) — in
  hold 8 my 70 016-commit law held a slot while the aggregate law claimed all 64 → "vcs operation capacity exhausted", and the poisoned
  `TEST_LOCK` failed six more; (ii) `artifact_history_empty_and_two_batch_replay_…` (already process-isolated) and
  `full_submit_durable_query_round_trip_…` dropped their `Database` without `shutdown`, so the last `Arc<VersionGraphs>` dropped a live vcs
  `ArtifactStore` on whichever thread released it; on the test thread the store's Drop witness panics (backtrace: `Arc<VersionGraphs>::
  drop_slow` → `VcsStoreCell` in the test closure) — reproduced 4/40 and 5/24 with 8 concurrent isolated runs; 6 sibling laws with the
  same shape passed 24/24 each (latent, untouched). Fix `wp-h14/h14-law-isolation.py` (test-only): the 8 admission laws become
  process-isolated, `TEST_LOCK` removed, the two laws shut their database down.
- 07:48:56–07:53 **hold 10** (`h14-lane-hold10.sh`, `hold10-*`): check EXIT 0 (no new warnings), whole kernel-db lib **729/729** under the
  default runner, `db_engine::` 136/136 three times, 8-way stress 0/40 (+ 0/120 by hand) for both shut-down laws and 0/40 for the aggregate
  law. Finding (production, not fixed — test-only scope): the `🗿️artifact` commit's vcs step says "best-effort … never blocks a commit" but
  returns any error other than `Unimplemented` AFTER the WAL write was durable and before the receipt is remembered; only kernel-db's own
  tests build `vcs` (the hub builds without it since (b)), so the default `vcs` feature has no production consumer.
- 07:5x checkpoint-policy en/de check: no user-facing surface shows the policy — it appears in `/readyz` `features.checkpointPolicy`
  (machine readout), the README env rows, and as `server.document.check-in` counts in the admin observability events table (an event
  name, aggregated with manual Check Ins; the `checkpoint-policy …` detail is only in the trace record). So there is nothing to label; an admin
  view of the policy would need `admin.*` en + de keys in `🛡️admin/🧱️elements/📚️I18n/🟦️.tsx`.
- 16:5x coordinator (C13 relay): "a document probe socket idle ~2 min gets 4401 on its next Commands frame on p33 (not on p24)".
  **Cause (not time):** `document_plan_socket_validity` re-proves a plan socket's authority on every 1 s authorization tick, admitted
  frame and broadcast — and it re-checked the plan's BOOTSTRAP facts: active checkpoint == `plan.checkpoint` (since 09-20,
  `48b9d63cf60`) and catalog generation == the plan's. Any Check In therefore closed every other plan socket of that document `4401`
  within ≤ 1 s (a catalog publication: every plan socket of the hub). Evidence, 7800 capture (`s13-w3-state-7800/capture.txt`),
  C13's note document: `server.document.check-in` ok (B, line 3733) → both browsers' sockets and the probe socket (opened at 3712,
  closed after 29 s) close at 3735/3737/3739, browsers redial; again after A's Check In (3771 → 3773/3775). The "~2 min" was the time
  until the journey's next Check In. p24 vs p33: the mechanism is older than p33 — the p24 7800 capture
  (`s13-w3-state-7800-131410`, 00:1x) shows the same closes right after each of its 4 Check Ins; on p24 C13's crafted leg failed
  earlier on the frame-deadline Ack (fixed by (a)), so it never held the probe across a Check In. My B1 policy would do the same
  every 1 024 edits. React worker: link loss (reconnecting,
  child suspended); Rust actors: reconnect.
  **Fix (hub, `wp-h14/h14-plan-socket-live.py`):** live authority = sealed plan, subject binding, descriptor, catalog SELECTION, a
  reached directory revision; `document_plan_bootstrap_current` checks generation + checkpoint once, at grant consumption; an
  outdated plan there answers `409 socket-grant-stale` (grant rejected; every client re-plans — the Rust link maps any upgrade
  failure to a link shortage, the browser redials). Laws: the consume law pins `409` + unchanged live authority for a Check In and a
  catalog generation; new quick law `a_plan_socket_outlives_a_check_in_and_ten_idle_minutes_and_its_next_batch_commits` (published
  checkpoint, 600 authorization ticks on a paused clock, then a batch commits). Worker: no change needed (every close is a
  transient link loss there, `RebootstrapRequired` is handled; the GIS approval's `RebootstrapRequired` broadcast now actually
  reaches plan sockets instead of being pre-empted by the 4401).
- 17:1x coordinator: semio-hub red mid-train (LB2 row 3 before 3b) → hold 11 withdrawn (mutex ticket removed, nothing applied).
  Rust document link check: a `409` at the upgrade and a live `4401` are both transient there already; the real defect is the
  twin of the worker's 09-22 fix — both Rust actors bound a LIVE socket by `plan.expires_at_unix_ms` (≤ 30 s exchange window): the
  native `socket_authority_deadline` and the wasm `pump_socket` check close + requeue + re-plan every wgpu/native document socket
  ~30 s after admission, for ever (mock laws use a 60 s window). Guest set `wp-h14/h14-rust-link-live-socket.py` (kernel
  `🏪️store/🔄️sync`: deadline, `invalidate_socket_authority`, test hook and the unread `socket_authority` fields go; native law's
  expiry tail dropped; mock-hub law `a_live_socket_outlives_its_admission_plan_window`: 3 s window, an edit 3.5 s after Session is
  accepted on the same socket), dry-run clean → L1's next train.
- 17:33–17:47 **hold 11** (`h14-lane-hold11.sh`, `hold11-2.txt`, `hold11-plan-laws.txt`, `hold11-bin-all.txt`; requeued the moment
  L1 wrote T6 round 2 GREEN): applied `h14-plan-socket-live.py` → semio-hub `--lib --bins --tests` EXIT 0 (no warning in the changed
  regions), plan/socket/presence/revocation laws **36/36** (incl. `a_plan_socket_outlives_a_check_in_and_ten_idle_minutes_and_its_next_batch_commits`
  and the consume law's 409 cases), whole os-hub bin **171/171**, integration-fixtures Check In + checkpoint-policy laws **8/8**. Not
  executed: the idle law against the pre-fix code (red-before argued from the removed `CheckpointDiffers` branch + the 7800 capture).
  Row 17 registered in `📓️t6-queue.md` (coordinator). 17:48 hold 12 queued: test-only `.forget()` of three gate permits in my hold-3 Ack
  law (`h14-ack-law-permits.py`; 3 `unused SemaphorePermit` warnings, a dropped permit returns to its gate).
  18:12–18:13 hold 12 green: the three laws 3/3, the Ack law's permit warnings gone (`hold12-laws.txt`).


### Session 14b

Successor agent, 2026-09-28 12:1x (predecessor cut ~20:45 by the usage limit, mid-way through the P1 db credit fix).
Chain launched 12:02:46 (GUEST FREEZE on). `semio-framework-os-kernel-db` has exactly one normal-edge dependent, `semio-hub`
(`cargo tree -i … --workspace -e normal --target all`), so it is chain-compiled only through os-hub (hub prewarm + the
post-publish os-hub build), never through a guest / rebuild-all.

| # | Item | Status |
|---|------|--------|
| 0 | Reconcile the predecessor's in-flight db edit | predecessor's WAL + artifact hunks (20:35–20:41) were auto-committed at 21:54 (`5bcb2da23da`) **red**: `PlannedEntries` undefined + replay still called the removed `apply_one` → chain `hub-prewarm rc=1` 12:03 (`s14-w4-logs/final-hub-prewarm.txt` l.222–676; predecessor's own `lane-hub-1.txt` 21:10 saw the same 4 errors). **Finished forward 12:2x**: `struct PlannedEntries` + replay `plan_one → commit_one`; native check queued (priority stamp) → `generated/lane-db-1.txt` |
| 1 | P1 db credit: a declared-legal batch never refused for capacity | **DONE, native green** (coordinator: kernel-db is hub-closure-only → hub rule): 4 capacity refusals + the replay trap removed, 1 P0 found by the max-batch law and fixed (WAL reader held 64 frames → a committed batch of ≥ 64 envelopes left its document unopenable); db laws 85/85 + 210/210, hub check EXIT 0, hub bin laws 2/2 — see log + landing |
| 2 | Trusted-catalog fixtures → channel 19 + bundle check green (coordinator 13:0x–13:5x; H14 = trusted-catalog owner this session) | **DONE**: 3 fixtures bumped; stdio-gis generation re-derived twice by the hub script's own encoding (→ `bda0b90f…` after ST2's svg pin fix; relayed); compiled-dependencies rawCases re-derived by the Pack codec (15 → 19); census sees Pack-hex + numeric `executionProtocol` versions, 2 fixtures registered, two-author fixture 17 → 19 via `generate`: `channel-version check` **0 findings**; resolver: `trustedBootstrapResolveClosure` (foreign/self/cycle/linked-profile edge) at the root; 3 stale source anchors aligned; `trusted-stdio-gis-bundle-check --source` **rc 0**; hub TS typecheck rc 0 |
| 3 | Rust hub `trusted_catalog` laws 54/57 → root-cause the 3 reds | **DONE**: all three panicked at `plugin().expect(..)` (unit l.940/l.1895) = stdio/gis/vcs assembly failing on the peer's svg `📡️.protocol.semio` change with a stale pinned SHA-256 (fixed by ST2 13:5x); rerun **3/3 PASS** 14:26 (`generated/lane-tc-1.txt`) |
| 4 | Item 7: remove the db artifact "deprecated-in-spirit extension seam" | **DONE 14:28**: `wp-h14/h14-remove-authz-seam.py` applied inside one lane hold (auto-revert armed): `AuthzHook`, `AllowAll`, `SecurityAuthzHook`, `Database::open_with_authz` and the generic `A` of `ArtifactEngineConfig`/`ArtifactEngine`/runner/`Database` removed (9 files); `submit` authorizes through `security` only; db `--lib --tests` EXIT 0, semio-hub `--lib --bins --tests` EXIT 0, db laws **210/210** (`generated/lane-seam-1.txt`); root `📜️script.ts` P1w caller census drops `open_with_authz` (its 16 other findings on the real engine source are pre-existing, identical before/after); TS self-test PASS |
| 5 | Predecessor's unrun hub laws (residency footprint, observability, pair-content Rust twin) | **DONE**: hub lib laws **68/68** (incl. footprint + pair-content twin), bin observability/readiness **6/6** 14:30 (`generated/lane-hub-2.txt`); landing row written for the predecessor's 27th hub set |
| 6 | Post-assembly codec origin (`OwnedRuntime::codec_call`, plugin host = frozen) | prepared patch `wp-h14/h14-codec-origin.py` **dry-run clean on the live tree 14:3x** (plugin-host + owned-instance laws + hub residency hunk: all `replace`, i.e. unchanged since capture); the predecessor's overlay law build died on an overlay-sync gap (`validate_fragment_array_structure`, 27th 20:57) → **window 3**: apply, native check plugin-host + hub, run the laws (no overlay rebuild now: machine reserved for the chain) |
| 7 | ALL catalog on 7800: idle-release/LRU under real pressure (row 2.7), boot/readyz timings | waits for 7800 on ALL (chain in rebuild-all at 14:3x) |
| 8 | `shutdown-drill` / backup-restore / README-metrics parity (row 4.9) | done by the predecessor (drill LIVE PASS 27th, parity law 7/7, spec relayed to R10); backup-restore stays permanent |

#### Session 14b log

- 12:1x read preamble 14 (rules 1–21 + 14b), AGENTS.md, fleet tail, this report. db module: no working-tree diff; the
  predecessor's P1 work = commit `5bcb2da23da` (10 files: `📝️wal/🦀️.rs` commands appended from their own encoded bytes
  inside the one transaction — `SegmentWriter::append_command`, `ArtifactWal::{preflight_submit, submit}(commands, records)`;
  `🗿️artifact/🦀️.rs` submit split into a side-effect-free plan (`plan_one`/`plan_entries`, batch-local last-writer map) →
  WAL preflight → commit (`commit_one`/`apply_entries`), so a WAL refusal leaves state untouched and commands take no DB I/O
  page credit; call sites in cli/cluster/tests pass `&[]`).
- 12:2x finished forward (not reverted — revert would have to restore 10 files from `6b8089dcb21`): `PlannedEntries` struct
  beside `DocumentState`; `open`'s replay → `plan_one(&envelope, &batch_ids, &mut HashMap::new())` then `commit_one`
  (same semantics: replay applies one envelope at a time, conflicts ignored); doc reference `apply_one` → `plan_one`.
  Main told (SendMessage). Native lane: `wp-h14/h14-lane-db.sh` (check db lib+tests, then hub lib+bins+tests),
  `FLEET_TICKET_STAMP=20260928120000`, capture `generated/lane-db-1.txt`; lane held by WG11 from 12:18.
- 12:3x–12:4x hold 0 (`lane-db-1.txt`, priority stamp): db lib EXIT 0; lib-test red on one stale `apply_entries` call (fixed);
  semio-hub red only on ST2's vcs xlsx break (`XlsxSnapshot.workbook`, relayed). Contract `💻️os/📦️packages/🦀️rust/📜️script.ts`
  asserted the old `preflight_submit(&self, records…)` signature → updated.
- 12:4x **P1 root causes** (C12: the post-cut outbox arrives as ONE batch, refused → rolled back → keystrokes lost). A
  declared-legal batch (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_*` = 8192 envelopes / 256 KiB) met four capacity refusals + one replay trap:
  (a) DB I/O per-operation credit when commands were staged as `WalBytes` (predecessor's WAL change: commands appended from their
  own bytes); (b) `DbLimits::default().max_batch_commands` 4 096 < 8 192 → now `protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES`;
  (c) the hub's per-socket DoS budget 240 tokens charged per envelope (a 241-envelope flush refused, permanent) → capacity = the
  declared maximum, whole batch charged at once (never partially), refill 60/s, and an empty budget is `DbError::Unavailable` →
  H13's transient `hub.unavailable` (client resends); (d) the replay guards (hub socket + engine) RECORDED ids at admission, so the
  resend of a batch refused after admission was refused as a replay (permanent) → `ReplayGuard::{check, record}` split,
  `SecurityGate::admit_commands` (batch) + `record_committed` after commit; the engine skips already-committed envelopes of a
  batch (idempotent merged resend) and refuses one id twice in one batch explicitly.
- 13:0x hold 1 (`lane-p1-2.txt`): db check EXIT 0 (226 pre-existing warnings), hub `--lib --bins --tests` EXIT 0 13:16, hub bin laws
  `a_document_socket_admits_a_declared_maximal_batch_and_readmits_an_uncommitted_one` + `a_transiently_refused_batch_…` **2/2 PASS**;
  db laws (security/wal/artifact/engine/sync/cluster) **330 pass / 2 fail**: (1) my `a_declared_legal_byte_maximal_batch_commits_as_one_transaction_and_replays`
  → `ArtifactEngine::open` refused the reopen → **P0**: `WalTransactionGate` (recovery + committed replay) held
  `[Option<WalRecordFrame>; 64]` while the writer bounded a transaction only by the readable segment → a committed batch of ≥ 64
  envelopes left its document unopenable (reachable since `5bcb2da23da`; before it, ≥ 63 envelopes hit the per-op credit instead).
  Fix: `WAL_TRANSACTION_RECORDS_MAX` = declared envelopes + 1 (frontier); `preflight_submit` refuses above it before any I/O; the
  gate is a bounded `Vec<WalRecordFrame>`; WAL law extended (maximal 8 192 × 32 B transaction reopens via `ArtifactWal::open` and
  replays byte for byte; one record over is refused, segment untouched); os contract assertion updated. (2)
  `fs_commits_and_reopen_storms_stay_within_their_throughput_bounds` (process-isolated child) → rerun alone in hold 2.
- 13:3x coordinator item: trusted-catalog fixtures `👥️two-package`, `🧬️stdio-gis-bootstrap`, `🧱️generation-stage` → `appChannelVersion`
  19 (2/2/3 literals). stdio-gis `profile.generationId` derived with the hub script's OWN `trustedBootstrapProfileEncoding` over
  `projectTrustedBootstrapCodecsV1` (loaded from a rewritten copy, `wp-h14/h14-bootstrap-generation.ts --write 19`): 0ca2d589… →
  76d1a92f… (4 occurrences: generationId, rotation initial/current/stale-issued); the derivation only admits the current version, the
  old id was already stale before 18. `channel-version check`: pin 19, 28 consumers, **0 findings** (`generated/channel-version-check-1.txt`).
  Oracles: `trusted-stdio-gis-bundle-check --publication-source` rc 0 (generation-stage 33 cases, publication; `fixture-oracle-2.txt`);
  full stdio+GIS bootstrap oracle **PASS** via `h14-bootstrap-oracle.ts` skipping only `proveTrustedCompiledDependenciesFixture`, which
  is red outside this fixture (`🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json` violates its schema; `fixture-oracle-1.txt`).
  RELAY ST2: `cx1-apply.py` BOOTSTRAP_GENERATION_OLD → 76d1a92f….
- 13:5x coordinator: (1) lease fixture `descriptorHex` → H13 (fixed by H13, census 0 findings 14:0x); (2)+(3) mine.
  (2) `trustedBootstrapResolveDependencies` was generalised to N packages on 09-21 (`50c97b20513`, deliberate) and lost four
  refusals the hub still enforces at load (`validate_bundle`: selected identity = its package record, topological order refuses
  cycles incl. self, profile fence `local-stdio-gis-open-v1` = gis → stdio exactly). Root fix in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`:
  per-package self-dependency refusal + new `trustedBootstrapResolveClosure(profileId, claims, candidates)` (identity = own
  descriptor, acyclic, linked-profile edge set), used by `materializeTrustedCatalogBundle` and `trustedBootstrapPreflightDescriptorsV1`
  (both hold every selected package's claims; `requests` = `selection` 1:1); the compiled-dependencies oracle resolves the whole
  two-package closure → 25/25 cases (probe `wp-h14/🗑️generated/probe-cases.ts`: 0 non-throwing refusals). Further reds of the same
  check, all stale source anchors vs deliberate 09-21/09-26 changes: fresh-component describe docstring (`Shared implementation` →
  `The ONE describe route`), Rust N-target messages (`exactly one` → `at least one` + count equality), `export async function
  materializeTrustedCatalogBundle`; `🌎️hub/🧫️fixtures/🤝️two-author-shell-v1` held `"executionProtocol": 17` unregistered → census
  pattern `"executionProtocol": N` + consumer row, `channel-version generate` wrote 19. **`trusted-stdio-gis-bundle-check --source`
  rc 0** 14:0x (`generated/fixture-oracle-9.txt`, `-11.txt` after the re-derivation); hub `typecheck` rc 0 (`hub-ts-typecheck-2.txt`).
- 14:0x ST2 fixed svg's pinned protocol sha (stdio receipts changed) → `h14-bootstrap-generation.ts --write 19`: 76d1a92f… →
  **bda0b90f3ef9a8e253c4ee54c6e7ea2595b8377330d4044f7e9d6400abbead51** (4 occurrences); full bootstrap oracle PASS with nothing
  skipped (`fixture-oracle-10.txt`); relayed to ST2 (`cx1-apply.py` OLD).
- channel-version vitest: Pack-hex + literal + generator laws 3/3 PASS; the repo-census law's 50 s whole-repo `git grep --untracked`
  (48.8 s old pattern vs 51.0 s new at load 44) exceeds the vitest timeout under load — pre-existing cost, the CLI census is the
  measurement (`channel-version check` 0 findings).

## Session 14

| # | Item | Status |
|---|------|--------|
| 1 | Post-assembly checkpoint per compiled guest (`OwnedRuntime::codec_call`), law: outputs byte-identical (H12's 15 FNV-1a hashes) | plugin-host part WRITTEN in overlay `s14-h14-overlay` (window 3), overlay law build queued (overlay lane #6); hub part (measured residency charge + `footprintBytes`) in tree, native lane queued (#6) |
| 2 | residency-watch "pairs differ" harness defect (id-derived digests) | H12's TS fix (16:00, live PASS 16:15) + my Rust twin under the production decoders; TS oracles **8/8 PASS** 19:07; Rust twin compile+run queued (native lane) |
| 3 | ALL catalog: residency/idle-release under memory pressure (row 2.7) + permanent RSS-watch harness; cold/warm boot + `/readyz`; creation latency per package | waits for 7800 on ALL (chain not launched yet) |
| 4 | Permanent commands: backup/restore drill (row 2.11) + graceful-shutdown drill (📜️script.ts verb + nx target + generated launch row, with R10); fuzz (row 2.13) | backup/restore ALREADY permanent (os-hub-ts `backup-restore-drill` + nx + launch row); **graceful-shutdown drill WRITTEN + LIVE PASS 19:5x** (os-hub-ts verb `shutdown-drill`, acceptance `hub-graceful-shutdown`; SIGTERM → exit **1 025 ms, code 0** with a 2d.puzzle creation interpreting + open socket + edit in flight, load 115); nx target + launch row spec → R10 (rule 17); fuzz = H13 item 6 |
| 5 | Coordinator 18:4x (audit-s14-state row 4.9): os-hub README / metrics-vocabulary parity vs the running hub (`/metrics`, `/readyz` schema), live diff on 7800 + own hub, fix at the root, permanent law or nx target | **live diff done** (7800 + 8161: `/metrics` 404 on both = README's "no scrape endpoint"; liveness/readiness/observability keys = schemas) → 5 README defects + 1 code-first body fixed; **permanent law** `hub README parity` (4 cases, os-hub-ts vitest) **7/7 PASS** 19:4x; liveness schema bin law queued (native lane) |

### Session 14 log

- 18:3x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `📓️wp-h12.md`,
  `📓️wp-h10.md`, `📓️wp-db1.md`, `📓️acceptance-s13.md` §2, `📓️fleet-13-agents.md` from 14:00, `📓️wp-h11.md` (items K/S),
  `📓️wp-h13.md`, `📓️wp-r10.md`. Load 73 (peers' greps + builds), 4 rustc. H12's hub 79070 is alive on **8161**
  (`os-hub-b3`, B3 clone root `s13-h12-hub-8161-b3`, 64 MiB residency).
- Reconciled H12 after its last report line (15:5x): at 16:00 H12 landed the pair-harness fix in the tree (hub TS only):
  `🌎️hub/🧪️tests/🧠️residency/🟦️.ts` (`pairContentDigest` = `semio.hub.pair-content/v1`: pack whole, SPR frame by frame
  without each frame's CRC-32C/`back_len` and a commit's chain hash, own id → `<document>`), new oracle
  `🌎️hub/🧪️tests/🪞️pair-content/🟦️.ts` + fixture `🌎️hub/🧫️fixtures/🪞️pair-content-v1/🔣️.json` (4 captured pairs), vitest config
  entry. Live rerun on 8161 16:15: `[acceptance] hub-residency PASS — 38/38 creations opened … pairs agree`
  (`wp-h12/generated/residency-8161-b3-2.txt`). No landing row. The TS law names a Rust twin
  `the_pair_content_digest_is_the_fixtures_under_the_production_decoder` (`🛰️lag-rebootstrap`, production decoders + `sha2`)
  that does not exist (`git grep` 0 hits) → item 2 = write it + run both.
- 18:4x coordinator: + item 5 (row 4.9); rule 16 (relay via main) + rule 17 (harness contract, reuse os-hub-ts verbs, no
  project.json/launch edits, send R10 the target spec). H13 (via main): no edits in OwnedRuntime / trusted-catalog residency /
  `interpret_off_worker` — mine.
- 18:5x **item 1 design.** `CompiledHandle.owned` becomes `OwnedCompiledGuest { artifact, codec_origin: OwnedCodecOriginCell }`.
  `codec_call` = get-or-assemble the origin, then run the operation on a CLONE of the origin instance (`OwnedSemioInstance`
  is `Clone`: memories, tables, globals; no serialization). Assembly = `codec.pack-schema-hash` for the schema `""` (no app
  declares it: the export runs `__semio_ensure_plugin_runtime`, answers its refusal) on a fresh instance, kept after its
  Deallocate. One assembly per compiled guest: a concurrent caller waits on a Condvar, relays the assembler's fuel as its own
  progress (a hub stall bound sees it move, and sees it stop), observes its own `GuestCallCancellation`; a failed / cancelled /
  panicking assembly leaves the cell `Absent` (drop guard) and a waiter takes over. The call's `budget.fuel` covers assembly +
  operation; progress reports both as one count. `CompiledHandle::codec_origin_bytes()` = origin memory + diagnostics.
  Laws (overlay, owned-instance-open tests, real note component): `codec_calls_answer_from_the_assembled_origin_exactly_what_a_fresh_instance_answers`
  (genesis/pack-schema-hash/print-mirror/apply-ops each == a fresh instance's answer; first call's fuel == assembly + second
  call's fuel exactly; second < fresh; genesis A, B, A → A unchanged; origin shared by clones; foreign schema refused),
  `a_call_waiting_for_another_calls_assembly_relays_its_fuel_honours_its_cancellation_and_takes_over`; third-party oracle:
  `wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter` now also compares wasmtime's fresh instance against the
  owned origin path. Overlay = tracked files + 2 589 gitignored generated inputs cloned (`wp-h14/h14-overlay-sync.py`, APFS
  clonefile); first overlay build 18:56 died on a missing generated file (`🔤️tokens/🦀️.rs`, gitignored) → extras added, relaunched
  19:14 (`wp-h14/generated/overlay-host-laws-2.txt`, overlay lane position 6).
- 19:0x **item 1 hub part (tree, rule 35, hub-only):** the residency ledger charges what a guest holds, not only its
  registration: `GuestResidentFootprintV1` (value-measured bytes; `CompiledHandle` → 0 today, → `codec_origin_bytes()` in window 3),
  slot charge = registered + footprint (admission, `configure`, `state`), `GuestResidencyV1::remeasure(value, context)` after
  every hub codec call (genesis, print-mirror, apply-ops, replay, row verification): a resident guest that grew past what is left
  displaces LRU unheld guests only if its uses outnumber theirs (the same TinyLFU rule), else becomes held (serves its
  operation, dropped with it), and one grown past the whole budget is never resident. Schema-first: `TrustedCatalogGuestResidencyStateV1`
  + required `footprintBytes` (JSON schema, Rust, TS twin, observability fixture ×8, observability unit law, admin page figure
  "Codec origins"/"Codec-Ursprünge", residency-watch round report). Law `a_guest_that_grows_in_a_call_is_charged_what_it_holds`.
- 19:0x **item 2:** Rust twin written: `🛰️lag-rebootstrap` unit law `the_pair_content_digest_is_the_fixtures_under_the_production_decoder`
  (production `decode_canonical_checkpoint_pair` + replication `format::FrameCursor`, own SHA-256; asserts frames == fixture
  `sprFrames`, digest == `contentDigest`, `sameContent` groups share one digest). The hub has no `sha2` dev-dependency (adding one =
  Cargo.toml/lock edit, frozen), so the third-party oracle is the TS law's node:crypto SHA-256; the TS header claimed `sha2` →
  corrected. **TS oracles 8/8 PASS** 19:07 (`bun ./📜️script.ts test long pair-content observability`, pair-content 5 + observability
  Ajv 3 incl. the fixture with `footprintBytes`; `wp-h14/generated/ts-oracles-2.txt`; the `quick` level's 15 s budget killed the
  first run at load 99: `ts-oracles-1.txt`).
- 19:07 native lane hold queued (`wp-h14/h14-lane-hub.sh`, `generated/lane-hub-1.txt`: check hub lib+bins+tests, lib laws
  trusted_catalog/observability/lag_rebootstrap, bin observability/readiness), position 6 behind t14/sh2/h13/lb2/g12.
- 19:2x **item 5, live diff** (`wp-h14/generated/{healthz,readyz,metrics}-{7800,8161}*`, `observability-8161.json` via
  `wp-h14/h14-obs-read.ts`, credentials from env only): `/healthz` 200 `{schema, status, runId, uptimeMs}` both hubs; `/readyz`
  200, keys = `LocalBootstrapReadinessV1` (ready: no `blockedBy`/`startup`); `/metrics` **404 on both** (README: "no Prometheus
  exposition … no scrape endpoint" — holds); `/admin/api/observability` (8161) top-level, `rows[]`, `routes[]` keys = README list =
  `HubObservabilityV1`; 25 `declaredEvents` incl. presence join/expiry/leave, boot, shutdown, catalog publication (README claim holds).
  **Defects found + fixed:** (1) README said `blocked_by`, the body/schema say `blockedBy`; (2) the startup object omitted its
  `catalog`; (3) the Known-gaps list pointed at a section "Health and readiness" that does not exist (→ "Health endpoints");
  (4) `SEMIO_TRACE_LEVEL`/`SEMIO_TRACE_SINK` are read by the hub (`Tracer::from_environment`) but were missing from the env tables
  (prose only) → rows added; (5) `/readyz` documented no field list → full list with `?` for `blockedBy`/`startup`, schema links;
  (6) **the liveness body was code-first** (`HubLivenessV1` struct, no schema) → `LocalBootstrapLivenessV1` in
  `🚀️local-bootstrap/🧬️schema/🔣️.json`, one `hub_liveness()` for the boot server and the router (was two copies), fixture value
  `live` (🚇️pipe-v1) + Ajv checks in the 🤝️integration quick contract, bin law `the_served_liveness_body_is_the_declared_liveness_schema`.
  Also: `OS_HUB_GUEST_RESIDENCY_BYTES` row now says what the budget charges (component + footprint).
  **Permanent law** (existing vitest file `🌎️hub/🧪️tests/📊️observability/🟦️.ts`, no new directory): `hub README parity` —
  every documented health route is registered (`.route("…")` in bootstrap) and its `{…}` field list == its schema's properties
  (`?` exactly on optional ones; observability `rows[]`/`routes[]` by `$ref`); the startup notation == `startupProgress`; the router
  registers no metrics/prometheus/scrape route and the README says so; README trace levels == `TraceLevel::as_str`; env tables ==
  every `"OS_HUB_*"` a non-test hub source reads + the trace/db-driver constants. Served ↔ schema stays with the Rust laws
  (liveness new, readiness + observability existing). Run: vitest verbose **7/7 PASS** (`generated/ts-oracles-4.txt`),
  `test long observability integration` **18 passed / 1 skipped** (`ts-oracles-3.txt`), os-hub-ts `typecheck` EXIT 0,
  admin vitest **21/21** (`admin-vitest-2.txt`, ObservabilityPage laws read the fixture with `footprintBytes`). Red-before for the
  parity law not run (would need the pre-fix README in place; the positive run + each assertion naming a fixed defect is the evidence).
  Noted, not changed: `OS_HUB_ADMIN_TOKEN` / `OS_HUB_TRUSTED_CATALOG_{BUNDLE,PROFILE}` are still scrubbed/passed by 4 TS launchers
  although no Rust reads them (README calls them vestigial) — legacy plumbing, candidate cleanup (not in the parity law's table set).
- 19:5x **item 4, graceful-shutdown drill** (rule 17: existing verb module + existing directory): `runShutdownDrill` in
  `🌎️hub/🧪️tests/💾️backup-restore/🟦️.ts` beside the backup drill (shared `bootHub`, now also answering exit code/signal + output),
  verb `shutdown-drill` in `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` (`withAcceptanceRecord` + `publishAcceptanceCheckResult`,
  check `hub-graceful-shutdown`, en + de summary, `blocked` on missing precondition), probe sessions gain `ended()` (close code +
  clean) in `🌎️hub/🤝️integration-harness/🟦️.ts`. Round: fresh root + catalog copy, cold boot, one document with 20 acknowledged
  edits, socket left open with a 21st edit in flight, a heavy-kind creation in flight → SIGTERM → pass iff exit code 0 within
  `SHUTDOWN_EXIT_BOUND_MS` (10 s = the db shutdown deadline), `server.shutdown … database=closed` recorded, socket ended with a
  close frame (≠ 1006), restart on the same root, head ordinal ≥ 20, next edit accepted, interrupted creation answers typed (< 500).
  **LIVE PASS** 19:5x (`OS_HUB_BINARY` = copy of the 16:08 build-dev `os-hub` = `.🧬semio/🌐hub/s14-h14-bin/os-hub-1608`, catalog B3,
  load 115): SIGTERM → exit **1 025 ms code 0** with the **2d.puzzle creation interpreting**; socket close **1012** (service restart)
  but `wasClean: false` (the hub exits before the closing handshake completes — noted); 7/7 checks (`generated/shutdown-drill-1.txt`).
  The observability read at SIGTERM answered non-200 in development mode (packages-verifying count unknown) → the drill now records
  the status. Hub 8161 (H12's, pid 79070) stopped 19:5x (SIGTERM → 2 871 ms); no hub of mine runs.
