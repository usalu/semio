# WP-SH2 — Space-Home IO-Owning Job, Space-Home Reds, Kernel Lib Reds (window 3)

Session 14 slice SH2 (Opus executor, coordinator `main`), continues `📓️wp-sh1.md`. Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP.
Rules: `📓️session-14-preamble.md` (+ 13/12). Inputs `wp-sh2/`, expendable captures `wp-sh2/generated/`, durable data
`.🧬semio/🌐hub/s14-sh2-*`. Ports: hubs 8080–8089, serves 6580–6589. Guest-linked edits: prepared patch + overlay proof
during the freeze, landed compile-atomic in window 3.

## Session 15

Successor (2026-09-29 19:2x; predecessor died ~18:45 without handover). Focus: APPROVED P2 Space set (guest + host) + S18's Space
directory stall at ~28 docs (same set: bounded delta batches). Guest freeze ON until "WINDOW 5 OPEN".

| # | Item | Status |
|---|---|---|
| R | reconcile the predecessor's last step | **tree clean**: `p2-set.py apply` (dry) on live 19:2x **123/123 apply, 0 applied, 0 3-way, 0 conflicts** (every listed tree file = the set's base); payload = stage (130 listed, 123 changed, 0 stale; captured 18:44 after the 18:28 E0277 fix); no SH2 process or lane ticket alive; backups `s14-sh2-p2-backup/*` are overlay-root only |
| 1 | P2 guest proof (overlay) | **p2f 19:40–19:43** (18:44 payload + oracle rebase): kernel `os_directory` **63/63**, kernel `sync,ureq` check 0 errors, space-home caa **123/123**, space-home plain **28/28**, space plain **52/52**, plugin-space **85/1** (`descriptor_is_fresh` = describe regen at landing), renderer-wgpu check 0 errors, 0 warnings in set files; space-space caa **95/17** = 13 store-drop-witness + 2 missing-row code reds (all tree-baseline tests) + 2 P2 lane-assertion reds → fixed in the stage (`p2/p2-registered-fixture.py`), re-proof **p2g queued** (overlay lane 6th). The 2 missing-row laws go green only with T6 row 12 (S20: reducer codes cross the job boundary) |
| 2 | P2 host proof (tsc; ShellHost changed 18:05, space script 17:20 — after the last tsc 17:19) | **tsc clean of new errors** (overlay synced 19:26 + set written): os files 27 = only the overlay dual-module `typedwire` test (0 in set files); space script 38 = tree baseline 38 (TS18046 class); bun TS laws (activity twin, 1 000 check-ins → 15 batches, largest 65 341 B ≤ 64 KiB, +1 check-in = 1 batch); 6 space oracles clean in the overlay (17/31/54/23/11/11). Rule-20 boot = at landing (host part cannot land alone) |
| 3 | t6-queue row (host part NOT safe alone: the host feeds `applyDirectoryEvents`, a verb only the guest part adds) | **row 20 appended 19:3x** (proof status in the row; one landing, guest + host) |
| 4 | live proof on 7800 t6 (after 'T6 HUB READY' + the round lands) | waiting |
| 5 | space oracle `home-directory-identity-rows-check` red on the TREE (pin drift, peer 13:17) | **LANDED (rule 22, test-only)**: 54 clean; set re-based (1 file, 3-way clean), 123/123 |
| 6 | coordinator: dedupe the evening-predecessor section | done: folded under `## Session 14` as `### Session 14a` |

### Log 15

- 19:2x predecessor's last runs (captures `.🧬semio/🌐hub/s14-sh2-captures/`): `p2c` 17:54–18:13 (base space-space caa 89/17 = tree
  baseline, `p2/base-fails.txt`); `p2d` 18:14–18:34 space-space caa **compile red E0277** (`TypedOperationFixtureReceipt` has no `Debug`,
  `.expect_err` in the request-delete / open-artifact unit tests) — fixed in the stage 18:28 (`.err().expect(…)`), never re-proven; same
  run: space plain 52/52, kernel `os_directory` 63/63, space-home caa 123/123, plugin-space 85/1 (`descriptor_is_fresh` = describe regen at
  landing), renderer-wgpu `check --lib --tests` 0 errors (4m21s). `p2e-space.txt` empty (never started). `p2-contract-case.txt` 18:49
  (`contract fundamental --case 🫧️mutate-s-space-1-any-viewer-transient`): for the new case only the repo-wide classes every transient
  lane shares (no runtime inventory; no third-party library, decision records a surveyed negative) — same as P1 / generation3d.
- 19:25 overlay proof queued (`SH2_TAG=p2f sh2-overlay.sh p2-proof`, driver pid 5063, mutex 5066; new job `p2-proof` = sync → clear-created →
  `apply --write` → space-space caa + plain, kernel `os_directory`, kernel `sync,ureq` check, space-home caa + plain, plugin-space,
  renderer-wgpu check; no base run — the base is `p2/base-fails.txt`). Lane: c13 holding, SH2 next.
- 19:26 overlay synced by hand (141 copied) + set written; `tsc` (`p2/tsconfig-p2-os-overlay.json`, `…-root-overlay.json`, new tree
  twin `…-root-tree.json`): captures `s14-sh2-captures/p2f-tsc-{os,root,root-tree}.txt`; `bun p2/p2-ts-laws.ts <overlay>` ok.
- 19:2x space oracles: identity-rows red on the tree AND the overlay (not P2): `home_row_action(…, "manageSpace", &row.id)` pin vs the
  tree's fallible `(…, "manageSpace")?`, and adjacency pins broken by a `RowActionPlacement` import. New `p2/p2-oracle-soft.ts <root>`
  lists every failing pin at once (tree 2, overlay 2 — P2 adds none). Fixed on the tree (landing row), set re-based + re-captured.
- 19:3x t6-queue row 20 appended; overlap scan: my files also appear in C12 hub-order + sdk-t6 (ShellHost, wgpu Shell, os `🟦️.ts`,
  backbone-envelope-io), S19 3b + S20 (wgpu Shell) → rebase + dry run after those land (noted in the row).

- 19:40–19:43 **p2f** (`s14-sh2-captures/p2f-p2-proof.txt`, fails `p2/p2f-fails.txt`): see row 1. The 17 space-space caa reds: 13 are the store
  drop witness (`artifact store reached Drop without its exact terminal-empty shallow-shell witness` — the context's registered app was
  never closed; memory "Store Drop Witness"), 2 are `open_artifact_of_a_missing_row_faults` / `request_delete_of_a_missing_row_faults`
  answering `interactive-job.app-owned-output` instead of the reducer's code — the SDK's MOUNTED terminal fault
  (`terminal_fault = ArtifactBoundedToolFault::from_payload(&fault.detail)`) keeps only the message of the reducer's prose detail
  (`retained command reducer rejected operation: <code> <msg>`), while the agent-lane preview decodes the code; S20's T6 row 12
  (`s20-patch-fault-code.py`) already frames job fault details `<code>\u{1f}<message>` and splits them in `from_payload` → no second
  fix; P2 lands after row 12. 2 are P2's own laws: a settled feed publishes `[Transient, Ui, Terminal]` (render refresh + terminal
  marker), the laws demanded `[Transient]` only.
- 19:4x stage fix `p2/p2-registered-fixture.py` (idempotent, `--write`): `Registered<A>` in the viewer transient tests (bound to
  instance 1, `close_registered_fixture_app` on drop unless panicking; Deref/DerefMut), the editor context's `SpaceIndexApp` +
  `feed`/`settle_typed`/`settle_action`/`table` and both P2 laws use it (explicit closes removed); `publishes_only_transient_state`
  (Transient present, only Transient | Ui | Terminal). Captured (123/123 live dry run), overlay refreshed (3 files), `p2g`
  (`sh2-overlay.sh p2-space`: space-space caa + plain) queued 19:48.
## Session 14

### Session 14c

Successor agent (2026-09-28 17:0x, chain relaunched 16:55:46, guest freeze ON). Predecessor cut ~14:37 (usage limit, app down).

| # | Item | Status |
|---|---|---|
| R | reconcile the predecessor's last step (B1 ShellHost patch) | **tree clean**: no B1 identifier in any of the 43 targets, all 27 new B1 files absent, no `s14-sh2-b1-backup/`; the worker/ShellHost diffs vs the B1 bases are peers' (sync settle, S18/F3/C13 host work). The ShellHost patch itself was finished and captured in the stage before the cut (14:29 / 14:31) |
| 1 | route-B local-catalog set `wp-sh2/b1/` (now 44 files) | **prepared, complete, overlay-proven**: all six ShellHost regions where S18 agreed; composed dry run (space-home set `--write`, then B1) on the live tree after F3's landing **39/39 + 44/44 apply, 0 conflicts**; overlay tsc **0 errors in B1/ShellHost/worker/config/case** (twice, 2nd after F3); os-config **159/159** (17 new leaf laws); case `📇️mutate-os-config-local-catalog` TS subject **6/6**; space-home feature **112/112**, lib **28/28**; 1 compile red (E0521) + 2 warnings found and fixed in the set; plugin-space proof-count law **8/8** with both sets (ov15); overlay build-dir + mirror deleted (rule 23) |
| 2 | space-home set dry run (Cursor peer on stdio/norm) | **39/39 apply, 0 conflicts** on the live tree (18:36 after F3; `📜️script.ts`, ShellHost = clean 3-way merges); grew 38 → 39 by the proof-count law (Home 14 → 17) |
| 5 | plugin-space proof-catalog law stale on the tree | **LANDED (rule 22)** studio 16 → 40, Home 13 → 14; native plugin-space **88/88** (landing row) |
| 3 | W2 hub fix (member.removed then space.deleted) | landed 13:27 + proven native (14b); live proof waits for 7800 on ALL |
| 4 | Home row-action label capitalization | in the space-home set (file 026: Open/Rename/Share/Delete/Manage, de Öffnen/Umbenennen/Teilen/Löschen/Verwalten); harness matches case-insensitively |

#### Log 14c

- 17:0x `b1-apply.py` on the tree: 37 apply, 6 conflict (032/033/036–039) — by design, their base is the 38-file set. Direct scan of every
  B1 target for `local-catalog|LocalCatalog|admitLocalDocument|…`: 0 hits; `git ls-files --others` 0 B1 paths (git diff alone is not
  enough — the 16:29 auto-commit would hide a half-applied hunk). `sh2-apply.py`: 38/38 apply.
- 17:0x new `wp-sh2/b1/b1-compose.py` (scratch root `.🧬semio/🌐hub/s14-sh2-compose`: tree copy of the 72 paths of both sets, 38-set
  `--write`, B1 dry run; `--rebase` re-bases B1 on the composed content): **38/38, then 43/43 apply, 0 conflicts**. Re-based 2 files
  (worker, ShellHost), `b1-capture.py` → 43 files. The peer keeps editing the worker (17:06 `newArtifactState`); the B1 worker hunk
  still merges clean (landing re-runs `b1-compose.py --rebase` first).
- 17:1x overlay `s13-sh1-overlay` re-synced (`sh2-overlay-sync.py`: 838 copied), both sets `--write` into it (B1 then 42 applied +
  worker = clean merge over the 17:06 peer change). `tsc -p wp-sh2/tsc/tsconfig-b1-overlay.json` (ShellHost, `🗂️local-catalog`,
  worker, config mutations TS, case TS): **27 errors, all in `📃️UiDocumentStore/🧪️tests/🧪️typedwire`** (overlay-vs-tree dual module
  identity of the retained contract; untouched file) — **0 in B1 code and 0 in ShellHost** (the predecessor's 3 ShellHost errors were
  the stale overlay's channel 18 vs 19) (`generated/tsc-b1-14c.txt`). Harness tsc (`🎬️studio` + `👥️two-human`) **rc 0**
  (`generated/tsc-harness-b1-14c.txt`).
- 17:2x repo test platform in the overlay: `discover` lists `📇️mutate-os-config-local-catalog [rust,typescript]`;
  `subject exhaustive --case 📇️mutate-os-config-local-catalog --implementation typescript` **6/6 passed** (identity sibling 6/6, same
  shape; `quick` exercises neither — exhaustive-level cases) (`generated/b1-case-ts-*.txt`). The Rust implementation builds the plugin
  host (wasmtime) → deferred to window 3 (rule 23: not worth a cold private build-dir).
- 17:14 overlay Rust proof queued (tag `ov13`, pid 90150, `sh2-overlay.sh b1` = os-config lib tests, space-home
  `component-app-assembly` lib tests, plugin-space lib tests; kernel dropped — B1 does not touch it) behind S19's overlay hold.
- 17:2x review fix in the set: the worker's new VALUE import of `LOCAL_CATALOG_CONFIG_SCHEMA` was extensionless (`…/🟦️`, copied
  from the identity `import type`, which is erased) → `…/🟦️.ts` like every other runtime import of the worker (stage + overlay,
  re-captured).
- **Gap found (not in this set, reported to main):** the wgpu native shell's `handle_replay_shell_command` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)
  serves only `os.directory.*` and `os.open-artifact*`; every other id is a documented no-op — so on wgpu a Home persist/bind/import
  commit's `os.local-catalog.admit` is dropped silently (same today for `os.create-space-artifact`). Needs the native half of Design
  B1 item 5 (the catalog on the native folder backbone + `open_folder_space_backbone`) or at least a named refusal notice.
- 17:38–17:47 **ov13** (one overlay hold): os-config lib **159/159** incl. the 17 new leaf laws (admit ×10, retire ×7: unit, fixture
  vector, canonical JSON, TS-projection parity, undo); space-home **compile red in B1 code**: E0521 in `🗃️apply-local-catalog-document`
  `keep_on_device` (`let refused = |code: &str, …| Fault::new(…, code, …)` — `Fault` needs `'static` codes) → `code: &'static str`;
  2 B1-introduced `unused_qualifications` warnings in the Home unit laws (`semio_framework_plugin::DslValue::as_str` → `DslValue::as_str`)
  fixed too (stage + overlay, re-captured) (`generated/ov13-b1.txt`). Re-run queued as `ov14` (`sh2-overlay.sh b1-guest`: space-home
  feature + plain lib tests, plugin-space; pid 16834; lane 4 deep: s19 holding, st2, t14, s18).
- 18:36 after F3's two landed ShellHost regions (tree ShellHost 18:20:59): `sh2-apply.py` on the live tree **38/38 apply, 0 conflicts**;
  `b1-compose.py` **38/38 then 43/43 apply, 0 conflicts** (ShellHost + worker clean 3-way merges). 19:0x overlay re-synced (368 files
  incl. F3's ShellHost), 39-set + B1 re-applied, `tsc` again **0 errors in B1/ShellHost/worker/config/case** (only the 27 overlay-identity
  errors in the untouched typedwire test) (`generated/tsc-b1-14c-post-f3.txt`).
- 18:52 **ov14** (after the E0521 fix): space-home `component-app-assembly` lib **112/112** (ov11 109 + the B1 guest laws), space-home
  lib **28/28**, plugin-space **86/2** — the same two reds as the tree baseline b14: `descriptor_is_fresh` (needs `describe`, window 3)
  and `every_app_instance_constructs_against_its_registered_proof_catalog` (`generated/ov14-b1-guest.txt`). Only pre-existing warnings.
- 18:5x root cause of the proof-catalog red: the law pins each app's `bounded_first_step_tool_proofs!` list length and is stale on the
  TREE (studio 16 → 40, Home 13 → 14 since `presenceHeartbeat`; index 14 ok) — test-only, so **landed under rule 22** (counts + the
  provenance moved from an in-body comment into the docstring; backup `.🧬semio/🌐hub/s14-sh2-backup/plugin-space-proof-count/`);
  native lane `test -p semio-s-plugin-space --lib` (tag `c14`, 18:54–18:56) **88/88** incl. `descriptor_is_fresh` (the tree's descriptor is
  current; the overlay's is stale only because the sets change it) (`generated/c14-plugin.txt`, landing row). The file joined both sets: space-home set Home
  14 → 17 (`bindSpaceFile`, `importSpace`, `deleteVirtualFileSystemNode`), B1 17 → 18 (`applyLocalCatalogDocument`) → sets are now
  **39** and **44** files; `b1-compose.py` **39/39 then 44/44 apply, 0 conflicts**. Overlay re-check queued (`ov15 plugin-proofs`,
  pid 58563).
- 19:29 **ov15** (both sets applied, overlay): `test -p semio-s-plugin-space --lib interactive_job_catalog` **8/8** incl. the proof-count law
  at studio 40 / Home 18 / index 14 (`generated/ov15-plugin-proofs.txt`).
- 19:3x rule 23: overlay proofs recorded → deleted my overlay build-dir `s13-sh1-build` (3.7 GB) + `s13-sh1-target`, and (disk 53 GiB
  and falling under the chain) the overlay mirror `s13-sh1-overlay` (8.7 GB) and the compose scratch; disk 61 GiB after.
  `b1-compose.py` recreates its scratch per run; a later overlay needs a full `sh2-overlay-sync.py` (≈ 80 k files).
- W2 live proof still waits: chain in rebuild-all (hub-prewarm rc 0 17:13), 7800 down.

### Session 14c-2 — P1 Home directory projection → transient lane (2026-09-28 23:2x →)

Coordinator P1 (S18, 7800/p24 user1): `directory-bootstrap.apply-refused` at seq 808 — `Space Home config publication exceeds its
complete retained envelope` (config lane sealed the WHOLE projection as one item + inverse copy). Design (approved 23:4x): the directory
projection is derived hub state → the app TRANSIENT lane (`✏️editor/🫧️transient`, the facet dir already registered, scaffold
`📌️.empty.md` retired): typed `Arc` rows folded by the canonical `store::os_directory::fold` one row at a time (copy-on-write of the
touched rows only), each page ONE bounded non-invertible item (`📬️apply-directory-page`, priced by the page, never the projection), no
edit/ledger rows; `HomeConfig` = local-studio tombstones only; rename/share/remove read the ONE row they need from the job's captured
transient; both surfaces share one page route (`HomeDirectoryPageWork`); receipt protocol + host unchanged (host bootstraps `after=0`).

- 23:2x both sets re-based onto the post-T1 tree (`sh2-rebase.py` new: 5 files; `b1-compose.py --rebase`: 4) → 39/39 + 44/44 clean.
- 01:2x reconcile after the cut/panics/sweep: all P1 work lives in the hub stage `.🧬semio/🌐hub/s14-sh2-stage` (survived); the live tree
  carries none of it (`git grep HomeTransient|apply-directory-page` 0, `git status` space plugin clean). Lost in the sweep: `wp-sh2/generated/`,
  `wp-sh2/target/` (captures only). Captures now go to `.🧬semio/🌐hub/s14-sh2-captures/`.

- 01:3x–02:4x P1 fix written into the space-home set (now **80 files**; B1 re-based on it, **44 files**, one conflict in the editor
  publication table / work routing resolved by hand, `wp-sh2/b1/b1-overlay-refresh.py` new):
  - `✏️editor/🫧️transient/` — `HomeTransient` (`Arc<HomeDirectoryProjection>`: `BTreeMap<Arc<str>, Arc<DirectorySpace>>` rows, `Arc` user
    table, frontier), `admit_page` / `fold_page` (canonical `store::os_directory::fold` over a one-row read model), receipt, wire
    (ToValue/FromValue over the framework `DirectoryReadModel` wire), text/pack codec (tooling only), `HomeTransientRetirementFactory`
    (byte estimate, never encodes the root); leaf `📬️apply-directory-page` (non-invertible, applied/no-op/rejected), JSON schema
    (+ receipt def moved from the config), 3 handcrafted vectors + no-oracle decision (`wp-sh2/sh2-p1-vectors.py`), fixture
    `📇️projection-wire-v1` + TS Ajv law moved from `🎚️config`, laws (10k bootstrap, page bound, captured-root isolation + CoW sharing,
    origin re-bootstrap after a drop, wire round trip, vectors through the bridge, retirement grants).
  - `HomeConfig` = tombstones only (schema leaves ×5 + viewer projection ×5: `surface-schema --plugin space --check` **0 drifted**);
    `configValueBytes` 4096; `applyDirectoryEventPage` lane Config → **Transient** (fixture + subset schema).
  - one page route for both surfaces: `HomeDirectoryPageWork<A>` (`CompleteWithEphemeral`: receipt event + ≤ 1 item); rename/share/
    remove read the ONE row they need from the job context (`handle_with_row`; the direct lane refuses by name).
  - language-neutral case `✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient` (Rust adapter via `law::vector`; subject features
    `component-app-assembly` declared on the subset oracles), registered e2e laws for editor and viewer (the generic
    `assert_viewer_never_mutates` admits only lane-less viewers → the plugin-space surface case for Home dropped, replaced by the
    registered viewer law).
  - space TS oracles re-stated (`home-directory-projection-persistence-check`, `home-directory-event-page-owner-check`: transient lane,
    no directory in config, non-invertible leaf; native law selectors with exact `…::component::tests::…` paths).
- 02:07 / 02:18 / 02:30 overlay `s14-sh2-overlay` (full mirror 80 302 files, APFS clones 1.5 GB): `check` space-home + plugin-space
  `--lib --tests`: 1st run a stray brace in the Home test helper + an unreachable arm in `HomeConfig::diff`; 2nd run: the generic viewer
  law's `NoTransient` bound → replaced; **3rd run rc 0** (type-checked, 0 errors; own warnings cleaned). tsc of the P1 TS (root
  tsconfig): TS law + schema TS 0 errors; space `📜️script.ts` only the pre-existing `JSON.parse → unknown` class (tree 36, overlay 38).
  Live-tree dry run 02:4x: **80/80 + 44/44 apply, 0 conflicts**.
- 02:31–03:27 laws in the overlay (`p1` job, captures `.🧬semio/🌐hub/s14-sh2-captures/ov23…ov26-p1.txt`): ov23 (gated by load
  until ~03:00) **7 red**, all one root cause: the new refusals were built with `Fault::from("<code>")`, whose code is `app.message`
  (the code sat in the message) → every new refusal is now a coded `Fault::new(FaultOrigin::App, "<code>", "<message>")`
  (direct-lane `…requires-retained-job` ×4, page `-invalid` / `-rebootstrap-required` / `-frontier-race` / `.work-repeated` /
  `.context-missing` / `-input-missing`; the space TS owner oracle's source assertion follows); B1's re-hydration law now removes
  the kept studio through `handle_with_row(…, None)`; the registered viewer law feeds every fixture page (the first page is an
  empty cursor advance) and asserts every folded space is listed; the viewer fold law compares `fault.code`. ov24 1 red (that
  last law) → ov25 / **ov26 green: space-home `component-app-assembly` 122/122, space-home plain 28/28, plugin-space 87/87,
  space-space check clean**; own warnings cleaned (the one left, create-studio's unused `PluginApp`, and the core / engine
  qualifications are on the tree already). The coordinator's two checks: (1) `a_captured_root_never_sees_a_later_page_and_untouched_rows_stay_shared`
  (a job's captured `Arc` root never sees a later page; rename/share/remove read ONE row via `context.transient.directory().space(id)`,
  the root is shared, never copied); (2) `a_ten_thousand_space_directory_bootstraps_page_by_page_under_the_one_item_bound` +
  `a_dropped_transient_rebootstraps_from_the_origin_without_history` + `a_page_item_never_exceeds_the_one_item_bound` +
  `a_dispatched_page_publishes_one_transient_item_and_no_history_row` — all green.
- 03:0x case contract (`test contract --case 🫧️mutate-s-home-1-any-editor-transient`, repo-wide rules, `p1-contract-case.txt`): the
  case's 3 breaches (kind uncovered / inverse uncovered / undeclared scenarios) are gone (contract 561 → 558); left for Home only the
  repo-wide classes every transient lane shares (no runtime inventory yet; no third-party library, decision records the survey).
  The case's Rust subject run was not executed (a standalone host workspace = a cold full build); the same three vectors run through
  the production bridge in `the_committed_vectors_hold_through_the_production_bridge` (green).
- 03:2x `b1-overlay-refresh.py` now leaves identical files untouched (a rewrite with a new mtime had rebuilt `semio-framework-os*`:
  the re-runs took ~1 min instead of ~50). tsc of the space script: only the pre-existing TS18046 class (38). Final live-tree dry
  runs **80/80** (2 via 3-way over peers: os host 🦀️.rs, ShellHost) and **B1 44/44** on top (2 via 3-way), 0 conflicts. Overlay
  build/target dirs deleted (rule 23, 3.0 GB); the overlay mirror is kept for a fast re-proof.
- 03:3x RELAY sent to L1 (crates, lanes, T3 regen/taxonomy extras) and main. Nothing of P1 landed on the tree (guest freeze → T3 via L1); no landing rows.

### Session 14c-3 — P2 Space activity: a check-in moves the Space row for every member (2026-09-29 16:5x →)

Coordinator (C12 collab STEP 6, `📓️wp-c12.md` row 28): the hub publishes `artifact.checkpoint-published` to both users but the
Space table's "Updated" never moves. Root cause (host delivery was fine — worker space lane → `collectSpaceDirectoryEvents` →
`foldDirectoryEvents`): (1) the kernel fold `os_directory::fold` (Rust + TS twin) ignored checkpoint events; (2)
`SpaceIndexConfig::indexed_artifact_from_directory` hard-coded `updated = created`; (3) `touchArtifact` wrote the retired
`SSpaceSnapshot.artifacts` lane the table never renders, relayed by BOTH shells through a background index-document session
per checkpoint. Latent, same family (S18 sweep: the directory stalls at ~28 documents with `interactive-job.publication-stalled`):
(a) the Space index kept the folded directory + presence in the undoable CONFIG lane (whole-record snapshot + inverse copy per
fold — the P1 fault class); (b) the host re-sent the space's FULL history on every event (128 KiB retained cap).
Design (approved 17:0x): ONE guest+host set `wp-sh2/p2/` (`p2-set.py`: stage/base `.🧬semio/🌐hub/s14-sh2-p2-*`, three-way
`rebase`, capture, dry-run/`--write`/`revert`):
- kernel read model: `DirectorySpace.document_activity` (`DirectoryDocumentActivity {documentId, updatedAtMs, updatedBy}`) folded
  from a HUMAN's checkpoint publication of an ANNOUNCED document (admin/system/unannounced/unknown-space ignored, latest wins in
  place); Rust + TS twin + read-model JSON schema; `DirectorySpace`/`DirectoryReadModel` carry their own camel-case value codec
  (Home's private wire structs removed — Home's transient wire IS the kernel codec); language-neutral cases
  `🧫️fixtures/📇️directory/🕒️document-activity-v1.json` authored by an independent Python reading (`p2-activity-fixture.py`).
- Home P1 transient: the checkpoint event now addresses its row (one-row fold ≡ kernel fold, new law over the activity cases);
  wire/schema/fixtures/vectors/TS law/space-script oracle carry `documentActivity` (+2 malformed cases).
- Space index: directory projection + presence → the TRANSIENT lane, owned by the viewer facet (`👁️viewer/🫧️transient`, so
  the viewer never imports `::editor::`) and reused by the editor; `SpaceIndexTransient` (kernel read model of the space's
  events + presence rows behind `Arc`s), verbs `📥️apply-directory-events` (bounded batch ≤ 1 hub page, frontier
  `afterSeqExclusive`, `0` = origin) and `👥️set-artifact-presence`, both non-invertible; shared route `SpaceIndexTransientWork`
  (admits in O(batch), folds once at publication); editor `Config = NoConfig` (config lane retired, NoConfig projections),
  opening reads the ONE row from the captured transient; viewer gets the retained feed route + renders directory rows (it
  rendered the retired snapshot lane); `touchArtifact` verb retired (13 editor tools, viewer 2); vectors + case
  `🫧️mutate-s-space-1-any-viewer-transient` (`p2-space-vectors.py`), retained catalog fixture + subset schema narrowing
  re-stated (`p2-space-limits.py`).
- host: ShellHost feeds each mounted Space surface bounded delta batches (`spaceDirectoryBatchesV1`, sequential, frontier per
  instance, origin replay on remount / not-applied, 1 s refeed on a retryable refusal); the `touchArtifact` relays of the
  React shell (background index sessions) and the wgpu shell removed (`p2-host-edits.py` re-applies onto the live shells).
- Laws: one check-in event moves Updated (en `2027-01-15 08:05 UTC` / de `15.01.2027, 08:05 UTC`) + Updated By for the author's
  editor AND the spectator's viewer, feeds write only the transient lane; 1 000 check-ins fold in bounded batches, root size
  independent of check-ins, last check-in = Updated; 1 000 documents + 1 000 check-ins list every document, the newest in the
  one batch carrying it, table stamps total 1 002 en/de (S18 stall); frontier refusals by name; origin replay no-op; wire
  corruption refused; vectors through the production bridge; TS: activity twin parity + 1 000-check-in batching (15 batches,
  ≤ 64 KiB each).
- 17:1x–17:4x overlay `p2a` (fresh private build-dir, overlay lane): space crates check clean (own warnings then cleaned);
  kernel `os_directory` 62/62 incl. the activity law; space-home caa 123/123 (incl. kernel-parity law); space plain 52/52;
  plugin-space 85/1 (`descriptor_is_fresh` = the describe regen); renderer-wgpu check 0 errors; tsc os 0 in P2 files (27 =
  overlay dual-module artifact in UiDocumentStore), root tsc only the pre-existing TS18046 class; space script oracles
  (projection-persistence 17, page-owner 31, identity-rows 54 (+surface-law count 3 → 2), job-catalog 23, data-class 11,
  plugin-identity 11) clean; bun TS laws clean. space-space caa 92/19: my 4 (bridge report keys, document loads with empty
  ops = framework now requires an ops header → laws no longer load a document) fixed; the other 15 (`dispatch_typed` of a
  retained verb answers only the operation handle; `load_document_text` without an ops header) → baseline run queued.

#### Window-3 landing runbook (SH2, both sets, in this order)

1. `python3 wp-sh2/sh2-apply.py` (dry) → `--write` (80 files).
2. `python3 wp-sh2/b1/b1-apply.py` (dry, on the tree after step 1;
   every file must say `apply`, worker/ShellHost may say 3-way merge) → `--write` (44 files).
3. Native lane: `test -p semio-framework-os-config --lib`, `test -p semio-s-artifact-space-home --features component-app-assembly
   --lib`, `test -p semio-s-artifact-space-home --lib`, `test -p semio-s-plugin-space --lib` (expect `descriptor_is_fresh` red
   until 4; the proof-count law expects Home 18), and the host case through the test platform: `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts
   subject exhaustive --case 📇️mutate-os-config-local-catalog` (rust + typescript; rust builds the plugin host).
4. wasm lane: space plugin `describe` regen (`✏️s/🔌️plugins/🪐️space/🔣️.json` flips `bindSpaceFile` to migrated + adds
   `applyLocalCatalogDocument`), `surface-schema --plugin space --check`, space TS oracles (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`).
5. `tsc` of the os package files touched (ShellHost, `🗂️local-catalog`, worker, config TS), rule-20 boot to Home, then
   `zsh wp-sh2/sh2-verify-home.sh --serve <6580> [--hub …] --locale en|de` (steps incl. import-kept-studio + reopen keeps it).
6. R10: taxonomy rows for the new dirs (config leaves `📥️admit-local-document`, `📤️retire-local-document`, host case
   `📇️mutate-os-config-local-catalog`, `🏛️ShellHost/🗂️local-catalog`, home command `🗃️apply-local-catalog-document`).
   P1 adds: `✏️editor/🫧️transient/` (`🧬️schema/🧬️mutations/📬️apply-directory-page`, `🧫️fixtures/📬️apply-directory-page/*`,
   `🧫️fixtures/📇️projection-wire-v1`, `🧬️schema/📇️directory-projection/🧪️tests/🔬️unit`, `🔮️oracles`, `🧪️tests/🔬️unit`), case
   `✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient`; the emptied `🎚️config/🧬️schema/📇️directory-projection/` dir is removed;
   schema-catalog regen (the receipt `$def` moved from the Home config schema to the transient schema).

### Session 14b

Successor agent (2026-09-28 12:0x, guest freeze ON since 12:02:46).

| # | Item | Status |
|---|---|---|
| 1 | set dry run on live tree, overlay tests, land in window 3 | tree clean of SH2 edits (38/38 `apply`, no backup dir); 2 files rebased; set now 38 files incl. the envelope/catalog fixes + capitalized row labels; overlay native `check` (`--lib --tests`, component-app-assembly) **rc 0** 12:33 (`ov8-check.txt`); overlay tests queued (overlay lane ~6 deep); wasm32 = chain only during the freeze |
| 2 | space-home IO-owning job + reachable controls | in the set; the 4 ov6 law reds root-caused + fixed in the set (host envelope drops ×5, per-call catalog port) — re-measure queued (ov8) |
| 3 | kernel lib reds + space-home reds | space-home tree reds unchanged (27/1, 95/2 — fixed in the set); kernel lib-test compile red: paged-list fixture include path fixed (test-only, landed); kernel check queued (ov9) |
| 4 | Home windowed-table bug W1 | S18 fixed; confirmed live with the permanent harness (create-hub-space + reopen PASS en + de) |
| W2 | deleted hub space stays on the live Home | **root cause hub** (member-only event filter hides `space.deleted`); fix + 2 laws LANDED 13:27; native check rc 0 + laws 4/4 (13:31); regression batch queued; live proof after 7800 on ALL |
| 5 | host-owned local-studio catalog design + prepared patch | design written (section "Design B1 (session 14b)"); prepared patch pending |
| 6 | Home e2e probe as permanent harness (rule 17) | `verify home` written, tsc 0, run live en + de (4/7 each, expected pre-landing); spec relayed to R10 |

#### Log 14b

- 12:08 `sh2-apply.py` dry run on the live tree: **files=38 apply=38 applied=0 conflicts=0** (36 targets equal `.old`; `📜️script.ts` and
  ShellHost `🟦️.tsx` 3-way-merge clean over peer changes) and `.🧬semio/🌐hub/s14-sh2-backup/` does not exist → the tree carries no
  half-applied SH2 edit. Rebased those 2 (base := tree, stage := merge) and re-captured: 38/38 plain `apply`.
- 12:10 overlay pins/drops cleared (the peer's pinned set is now committed: every pinned file equals HEAD, dropped paths are tracked);
  `sh2-overlay-sync.py` copied 2 367 changed files; payload applied to the overlay (38 written).
- 12:11 overlay `check` launched detached (tag `ov7`, pid 48806, `generated/ov7-*.txt`); native re-measure on the tree queued (tag `b14`,
  pid 48890: space-home lib, space-home `component-app-assembly`, plugin-space, kernel `sync,ureq`).
- 12:12 ov7 red in `semio-framework-ui`: `colors::DIFF_ADDED` missing — the overlay mirror never copied the gitignored generated token
  table `🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` (`.gitignore` names it explicitly, it is not under a `🤖️generated/` dir). `sh2-overlay-sync.py`
  now also mirrors `.gitignore`'s explicit generated files (`overlay-extra-files.txt`).
- 12:1x root causes of the 4 overlay-ov6 law reds (read in code, re-measure in ov8):
  - **envelope Drop witness (3 laws)**, all in `🖥️host/🦀️.rs` (set file 025): `import_os_space_from_dsl` built its vcs from a temporary
    `create_document_envelope(..).vcs.clone()` (shell dropped undetached — every studio import aborted, also on the tree) → now
    `create_backbone_document`; the file backbone write (`SpaceBackbonePort::write`, File kind — the bind commit) read
    `parsed.envelope.vcs.initial_snapshot` and dropped the parsed shell → `parsed.into_snapshot()` (the `.os` mirror now prints the
    HEAD projection, not the genesis); the dsl-only file read built an envelope just to print its pack → `create_backbone_document` +
    `export_backbone_pack`; `decode_backbone_payload` dropped the shell on a schema mismatch → `into_envelope()` + `retire_unadopted()`;
    `import_os_space_from_pack` partial-moved the envelope → `into_envelope()`.
  - **persist listing 0 (1 law)**: `catalog_port()` minted a NEW `LocalStorageBackbonePort` per call, while the catalog tracks studio uris
    per port `Arc` address and the port keeps its fallback bytes per instance → anything bound/persisted/imported was gone on the next
    listing (and every call re-parsed + re-seeded the demo studio and leaked a tracking key). Space core (set file 019):
    `catalog_port_concrete` is now one process singleton (`OnceLock`, seeded once).
- 12:20 ov8 queued (overlay lane behind EN2): check, home-feature, home, plugin, space.
- 12:2x item 6 harness (rule 17, EXISTING dirs only, open during the freeze: host TS not under `🔌️plugin/`):
  `verify home` in os-dev (`🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts` route) → `runHomeE2eCli` / `runHomeE2e` in region `🔖️HomeE2e` of
  `🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts` (steps boot, import-control, [hub: sign-in, create-hub-space], import-studio, remove-from-home,
  reopen, [hub: delete-hub-space]; every step also fails on fault lines; `withAcceptanceRecord` + `publishAcceptanceCheckResult` check
  `home-e2e`, en + de, `blocked` on no serve / missing hub credential env `OS_HUB_PROBE_EMAIL`/`OS_HUB_PROBE_PASSWORD`). Session helpers
  are reused, not forked: 14 helpers of `👥️two-human/🟦️.ts` gained `export` (no body change). `tsc` over script.ts + the 3 files
  (1 170 files) **rc 0, 0 lines** (`wp-sh2/tsc/tsconfig-harness.json`, `generated/tsc-harness.txt`). **Not run yet** (needs the landed
  guests; one serve + browser deferred to window 3 to keep the machine free for the chain).
- 12:3x native tree re-measure (b14): space-home lib **27/1** (`structural_correspondence`), space-home `component-app-assembly` **95/2**
  (`retained_config_cancel_and_cleanup…`, `structural_correspondence`) — the same two reds the set fixes (`generated/b14-home*.txt`).
- 12:4x kernel lib-test red (relayed from WG11): `🧬️retained-clone/📋️paged-list` unit test `include_str!("../../../🧫️fixtures/📦️copy/…")`
  one dir too high (fixture lives in `📋️paged-list/🧫️fixtures/`) → `../../` (test-only, landed in the tree, copied into the overlay);
  kernel `check --lib --tests` queued in the overlay (`ov9`) to see whether the reported E0618 on `RetainedCloneGrant` is real or a
  follow-on of the missing include (no `let grant` shadowing found by reading).
- 12:4x **W1 re-run (S18's fix) with the permanent harness** on S18's serve 6540 → 7800 B3 (`zsh wp-sh2/sh2-verify-home.sh --serve
  http://127.0.0.1:6540/ --hub http://127.0.0.1:7800 --locale en|de`; credential env via `sh2-hub-env.sh`, never argv): en **4/7**, de
  **4/7** — boot, sign-in, create-hub-space (row now found — W1 fixed), reopen PASS; import-control + import-studio FAIL as expected
  (pre-landing guests); delete-hub-space FAIL → finding W2 (`generated/home-e2e/s14b-w1-{en,de}/report.json`). Harness fixes: an
  unhandled `filechooser` rejection when the control is missing; the delete action pattern anchored (`^delete\b`, the row name could
  contain the word); a failed delete reloads and records `goneAfterReload`.
- 13:0x–13:2x **W2 root cause** (probes `sh2-probe-delete-live.ts` with a DOM timeline recorder, `sh2-probe-hub-delete.ts` hub-only):
  after "Delete Space" the live Home keeps the row (75 s … >240 s; a reload drops it). The hub accepts `delete-space` in 21 ms with
  `space.deleted` in the reply, but `/directory/event-page/v1?after=631` answers `throughSeqInclusive 633` with **0 events** — raw
  directory events are member-only and judged at READ time; after the deletion nobody is a member, so `space.deleted` reaches no page
  and no socket (and `directory_access_change_for_reader` owes `access-changed` only for `member.removed`). Not the 30 s settle
  deadline. **Fix landed 13:27** (hub, open during the freeze): `decide(DeleteSpace)` emits `member.removed` for every member (owner
  included) before `space.deleted` → each member's global socket owes the existing `access-changed: revoked` → the worker re-reads from
  the origin, whose member-only pages no longer carry the space. Laws: decider `delete_space_revokes_every_membership_before_the_space`,
  live socket `a_member_of_a_deleted_space_is_told_its_access_was_revoked` (`sh2-w2-patch.py`, backup `s14-sh2-backup/w2/`).
  Native `check -p semio-hub --lib --tests --bins` + both laws in ONE hold (coordinator slot `FLEET_TICKET_STAMP=20260928120003`),
  13:28–13:31: **check rc 0** (83 warnings, type-checked), **decider laws 2/2, socket laws 2/2** (`generated/w2-hub-all.txt`). Regression
  batch around deletion (invite redemption, admin shutdown, CAS space-delete, access-changed) 13:46–13:49 **all ok** (lib 19, bin 7;
  `generated/w2-hub-regress.txt`). Live proof waits
  for 7800 on ALL (the chain builds os-hub after the publish).
- 13:3x serve 6580 stopped (idle; `serve-hold` log shows `stopped`).
- 13:2x Home row-action labels were lowercase localized words ("delete: <name>", de "löschen: …"), not raw ids (my first relay said
  ids — corrected) → set file 026 capitalizes them en + de ("Open/Rename/Share/Delete/Manage", "Öffnen/Umbenennen/Teilen/Löschen/
  Verwalten"); Home window laws match case-insensitively.
- 13:1x own serve for the probes: `serve-hold --serve http://127.0.0.1:6580/ --hub http://127.0.0.1:7800` (w2-detach pid 49307, log
  `.🧬semio/🌐hub/s14-sh2-serve/serve-hold-6580.log`), ready in 3 s.

### Session 14a — evening predecessor (2026-09-27 18:2x →)

| # | Item | Status |
|---|---|---|
| 1 | space-home `bindSpaceFile` / `importSpace` / `deleteVirtualFileSystemNode`: IO-owning job, event-sourced, progress + cancel, en+de, laws + oracle, reachable control | **prepared, overlay native check green** (35-file set `wp-sh2/payload/`, `sh2-apply.py` dry run 35/35 clean on the tree; `generated/ov5-check.txt` rc 0 19:19); overlay laws queued (overlay lane 4 deep); bind cannot work in any real shell → blocker B1 (design below) |
| 2 | space-home reds (`structural_correspondence` outcomeClasses, config `retained_config_cancel_and_cleanup…`, all other lib reds) | tree baseline measured: space-home lib **27/28** (`generated/base-home.txt`); fixes in the set (2 stale tests + 4 stale TS oracles/schemas found on the way: Home retained-limits schema, SpacePlay schema, event-page oracle, identity-rows oracle) |
| 3 | kernel lib reds (directory client ×5, open-plan fixture, en1993 grammar) | pending re-measure (SH1 overlay 15:34: 1233/1234, only en1993 left) |
| 4 | space/home e2e in `s` (create, bind/import, delete, reopen) locally + hub 7800 | probe written (`wp-sh2/sh2-home-e2e.mjs`); **baseline (pre-SH2 guests, serve 6580 → 7800 B3):** no import control (expected), **create-space row never appears — Home table never streams past row 27** (finding W1, routed to main); full run after landing |

#### Log 14a

- 18:2x read AGENTS.md, preambles 14/13/12, fleet-14 log (no CHAIN LAUNCHED yet), `📓️wp-sh1.md` + `wp-sh1/`, `📓️wp-lc.md`,
  `📓️wp-ld.md`, inventory row SH1, fleet-13 log 14:00→16:0x.
- 18:3x `python3 wp-sh1/sh1-apply.py` (dry run) on the live tree: **files=25 apply=25 conflicts=0** (every target still equals
  SH1's `.old`, no peer changes since 15:56).
- 18:37 baseline native-lane runs queued detached (`wp-sh2/sh2-native.sh`, pid 44771, tag `base`: space-home lib, space-home
  `component-app-assembly`, space-space, plugin-space, kernel `sync,ureq` lib) → `generated/base-*.txt` (native lane 6 deep).
- 18:3x overlay = SH1's `.🧬semio/🌐hub/s13-sh1-overlay` refreshed by `wp-sh2/sh2-overlay-sync.py` (tracked + untracked files,
  size/mtime diff copy, 64 s, 329 files). Overlay runs: `wp-sh2/sh2-overlay.sh` via `fleet-mutex.sh overlay`, PRIVATE
  build/target dirs `s13-sh1-{build,target}`.
- 18:40–18:52 overlay check of SH1's payload blocked twice by a PEER's in-flight edit (not ours, not staged): the
  COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE work (`🌱️value/✨️derive/⚙️expansion` + `🔁️codec` + `🧬️schema/✅️validator`
  + `🗣️dsl/🧬️schema` + stdio contract `✏️editing/🩹️patch/` (untracked) …): the worktree fails in `semio-framework-actor`
  (8× E0282 in derive expansions, `generated/ov1-check.txt`); pinning only the value files to the index broke stdio contract
  (`generated/ov3-check.txt`). Overlay now pins that peer's whole set to HEAD (`wp-sh2/overlay-pin-head.txt`) and drops its
  untracked files (`overlay-drop.txt`). The live tree is red in the same place until that peer finishes — my native-lane
  baseline will measure whatever state the tree has when its turn comes.
- 18:5x design review of SH1 (read the code paths, not assumed):
  - Every real shell runs guests as wasm32 (browser jco, native wgpu via wasmtime); the local studio catalog in a guest is
    process-global memory (`LocalStorageBackbonePort` = memory fallback on wasip2; `set_host_backbone_port` never called),
    and `createStudio kind=folder` / `persistLocally` / the old `bindSpaceFile` are silent no-ops under `cfg(wasm32)`. So
    local studios never survive a reload in any shell, and a guest can never bind a file. The root fix is a host-owned local
    studio catalog service (React: store worker + dev folder/file endpoint `writeBackbonePayload`; wgpu native: file IO)
    reached by a `ReplayShellCommand` channel like `os.directory.*` / `os.create-space-artifact` (which already carries
    progress + cancel). Out of this slice's window-3 scope → recorded as blocker B1 below.
  - Host defect found: ShellHost intercepted `importSpace` on the landing controller, clicked a hidden input and re-dispatched
    `importSpace {json}` — which the same intercept caught again (a picker loop), with an arg the guest never reads (`dsl` /
    `payload`), and routed `.pack` to `importSpacePackPayload` on Home (only the studio app declares it). The guest already
    answers a bare `importSpace` with `RequestFileOpen{accept ".os", importAction "importSpace"}`, which the generic host path
    serves chunked (`{payload, name, chunk, chunkCount}`, 32 KiB chunks).
  - Host defect found: `import_os_space_from_dsl` admitted the studio with an EMPTY document name (the catalog lists by
    document name) → SH1's import law would have been red (`catalog_entries_named(name) == 1`).
- 19:0x SH2 payload = SH1's 25 files + 4 (`wp-sh2/sh2-files.txt`, stage `.🧬semio/🌐hub/s14-sh2-stage`, base
  `s14-sh2-base`, `sh2-capture.py` → `wp-sh2/payload/`, `sh2-apply.py` dry run **29/29 apply, 0 conflicts** on the tree):
  - import: decode refuses `chunkCount > 1` by name (`s.home.import-space.oversized`, bound = `IMPORT_CHUNK_BYTES`),
    reads `payload` via `kernel::IMPORT_ARGUMENT_PAYLOAD`; validate refuses an unnamed manifest (`…unnamed`); host
    `import_os_space_from_dsl` keeps the manifest's name.
  - controls: Home toolbar row `#s-home-toolbar` = `#s-home-create-space` + `#s-home-import-studio` ("Import Studio" /
    "Studio importieren", `importSpace` without args); every local row gets "Remove from Home" / "Aus Home entfernen"
    (`deleteVirtualFileSystemNode`, icon eye-off).
  - ShellHost: the landing `importSpace` intercept, its hidden input and the now unused `landingControllerId` removed.
  - laws added: decode (1 chunk / 2 chunks / bare), studio `.os` export → import round trip, unnamed refusal; Home window
    tests re-stated (local row = open + remove; ephemeral = open + promote + persist + remove; toolbar buttons by key;
    German toolbar/removal labels).
- 19:2x surface-schema: the new `HomeConfig.retiredLocalStudioIds` made the viewer's projected config lane drift (`bun …/📇️registry/📜️script.ts
  surface-schema --plugin space --check`: tree 0 drift, overlay 1 drift) → regenerated in the overlay, the 5 viewer projection leaves joined the set.
- 19:3x space TS oracles (`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`), measured on tree AND overlay:
  `home-directory-projection-persistence-check` 11 clean both; `plugin-identity-check` 11 clean both; `persistence-data-class-check` 11
  clean both; **reds on the tree (pre-existing, not SH1's):**
  - `home-directory-event-page-owner-check`: the Home retained-limits schema (`HomeRetainedCommandLimits`) is self-contradictory since
    the shared `RetainedCommandLimits` shape changed 09-15 (owner narrowing demands `scalarBytes`, shared byte budget forbids it and
    requires `configValueBytes`) and still pinned the pre-S4 4 MiB / 16 MiB step budgets (code: 1 MiB). Fixed: fixture + schema →
    shared byte budget (`configValueBytes` = new `HOME_CONFIG_VALUE_BYTES` = base + 136, the literal the config admission used);
    Rust law pins all seven limits to code constants (was 2). Then its source-string proof was stale since the page emission moved
    into `HomeConfig::directory_event_page_emit` (09-26) → re-stated against the config source (hostile mutations `replaceAll`).
    Overlay: **27 checks clean**.
  - `home-directory-identity-rows-check`: 3 stale source strings (manageSpace import line, `spaceEngine` read the same file twice →
    count 6 ≠ 3, `render_rows_wrapped` gained `&TreeWindows::unhosted()`) → fixed; overlay **54 checks clean**.
  - `interactive-job-catalog-check`: studio `SpacePlayRetainedCommandLimits` narrowing stated 16 contracts / 15 bounded / 25 batch;
    the fixture (Rust-pinned to code) has 40 / 40 / 0 → schema re-stated from the fixture by the one-off `wp-sh2/sh2-spaceplay-schema.py`.
    Overlay: now stops only at `descriptor publishes s.space.home@1/*#editor:bindSpaceFile as batchOnlyPendingRewrite, source says
    migrated` — the generated `✏️s/🔌️plugins/🪐️space/🔣️.json` flips only with `describe` (window 3, wasm lane), as SH1 noted.
- 19:48 native-lane baseline on the tree: space-home lib (no feature) **27 passed / 1 failed** (`generated/base-home.txt`, the
  `structural_correspondence` red); other baselines still queued.
- 19:5x asked main: land pre-freeze (like H13) or hold for window 3; default hold.

#### Blocker B1 — local studios cannot persist or bind in any real shell (design, not in this slice's scope)

Measured in the code: every shell runs guests as wasm32 (browser jco; native wgpu shell via wasmtime), the guest's local studio catalog
is process-global memory (`LocalStorageBackbonePort` = memory fallback on wasip2, `store::set_host_backbone_port` has no caller), and
`createStudio kind=folder`, `persistLocally` and (before this set) `bindSpaceFile` were silent `cfg(wasm32)` no-ops answering success.
So local studios (ephemeral drafts, imports) vanish on reload and no guest can bind a file; hub spaces are unaffected. Root fix: a
host-owned, event-sourced local studio catalog — React: one `os.local-studios` document in the store worker bound to the existing
folder lane (`${S_DATA_DIR}/os`, dev endpoint `writeBackbonePayload` already writes `file://`/`folder://`), wgpu native: the same
document on its native folder backbone; guests reach it through `ReplayShellCommand os.local-studio.{persist,bind,retire}` (the
`os.directory.*` / `os.create-space-artifact` pattern, which already carries progress + cancel) and the host re-seeds the guest
catalog on Home open. This set makes bind refuse by name (`s.home.bind-space-file.filesystem-unavailable`) instead of lying.
- 20:1x persistLocally + folder `createStudio` had the same lie as bind (IO in `handle`, `let _ =`, success on wasm32): persist is now
  a third `HomeCatalogWork` route (dialog without a folder / validate / commit via the new public host
  `admit_os_space_document`, which also fixes bind + persist listing the studio with an EMPTY name — `import_os_space_from_pack`
  drops the document name); folder `createStudio` refuses by name (`folder-path-required`, `io-failed`, `filesystem-unavailable`).
  Laws: persist job validate→commit once (folder written, listed once under its name, draft retired, repeat refused), persist
  refusals by name, folder create without folder refused, bind keeps its name. Set = 38 files, dry run clean.
- 20:1x baseline e2e on the orphan session-13 serve 6580 (F2's, `S_HUB_URL` 7800 B3, pre-SH2 guests; one headless browser,
  closed): `bun wp-sh2/sh2-home-e2e.mjs sh2-base-en http://127.0.0.1:6580/` → boot PASS, sign-in PASS, toolbar has only
  Create Space (no Import Studio) FAIL-as-expected, **create hub space FAIL**: `POST /directory/commands` 202, but the row never
  appears (`generated/sh2-base-en-report.json`). Root of that: **finding W1** — Home's windowed table stamps
  `offset=0 length=27 total=41` and never moves when scrolled or wheeled (`sh2-probe-window.mjs`,
  `generated/sh2-probe-window-1.txt`); every space past row 27 is unreachable. Suspect (read, not proven): the ShellHost
  tree-window scheduler decides window-vs-panel body from the PRIMARY session's app, Home is the landing app → refreshed as a
  panel body. Routed to main (S18?).

#### Design B1 (session 14b) — host-owned, event-sourced local studio catalog

Measured facts it builds on (code read 12:1x–12:3x, 09-28): the dev serve's `/semio-backbone` endpoint (`🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
`readBackbonePayload`/`writeBackbonePayload`) already reads/writes `file://<path>` (raw archive) and `folder://<dir>` (sqlite
`.semio/documents.db`, `documentId` default `studio`, the same convention as native `vcs::FolderSqliteStorage`); payloads are
canonical document archives (`encodeDocumentArchiveBytes` = `{parent_pack, parent_spr, members[]}`). The store worker already opens
persisted-local-only facets through `PersistenceBinding {kind:"folder", dataClass:"persistedLocalOnly", path:`${dataDir}/os`}`
(`identityActorConfig`), `dataDir` = `VITE_S_DATA_DIR`; without it the facet is ephemeral-local-only. Guests reach host IO only through
`Effect::ReplayShellCommand` (ShellHost routes `os.directory.*`, `os.create-space-artifact` to worker requests with request ids, progress
UI and `*-cancel`), and the host feeds Home through receipt-sealed, job-routed actions (`applyDirectoryEventPage`,
`ShellHost/📇️directory-bootstrap/`).

1. **Authority.** One host-owned event log `os.local-studios` (schema `os.config.local-studios`, singleton by schema id like
   `os.config.identity`), data class persisted-local-only (folder lane `${dataDir}/os`; no data dir → ephemeral-local-only and Home says
   so). Events (no CRUD): `studio-admitted {spaceId, name, storage: folder|file, target, archiveSha256}`, `studio-bound {spaceId, file}`,
   `studio-retired {spaceId}`. Each studio's own document lives beside it: `folder://${dataDir}/os/studios/<spaceId>` (persist/import) or
   `file://<path>` (bind), written with `writeBackbonePayload` semantics (dev endpoint in React, native fs in wgpu).
2. **Guest → host (commands).** Home's retained jobs keep validate/commit; on a host without a filesystem (every wasm32 guest) the commit
   stage emits `ReplayShellCommand os.local-studio.{admit,bind,retire}` with `{spaceId, name, target?, archive}` (archive = the studio's
   `export_backbone_pack` wrapped as a document archive, ≤ the retained raw budget) instead of refusing, and answers progress
   `space-home.catalog.requested`; import additionally admits in memory so the row shows at once (pending chip until the host echo).
3. **Host job.** ShellHost routes `os.local-studio.*` to worker request `local-studio-command {requestId, command}`; the worker validates,
   writes the studio document (progress = bytes written / archive bytes, cancellable until the write commits via
   `local-studio-command-cancel`), then appends the catalog event (the write-then-append order makes a cancelled or failed job leave no
   event). Refusals are named `os.local-studio.*` with en + de notice texts (replay refusal table).
4. **Host → guest (projection).** The worker streams `local-studio-page {throughSeqInclusive, events}`; ShellHost delivers it to Home as the
   job-routed `applyLocalStudioPage {pageJson}` (same receipt/settle protocol as `applyDirectoryEventPage`, owner module beside
   `📇️directory-bootstrap`). The guest folds the page into its (now process-singleton, set file 019) catalog port idempotently by
   `spaceId` (admit = `admit_os_space_document` of the decoded archive, retire = config tombstone already in this set), bumps
   `catalog_generation` once per page. On every Home mount the worker replays the log from seq 0 → **reopen after reload lists every
   persisted/bound/imported studio**.
5. **wgpu native shell.** Same route and page in its Rust host (`os-shell` native): the log on its native folder backbone, the studio
   documents via `open_folder_space_backbone`/`open_file_space_backbone` (host-side, where they compile).
6. **Laws.** Guest: commit on a no-filesystem host emits exactly one `os.local-studio.*` command and writes nothing; `applyLocalStudioPage`
   admits once per id (replay idempotent), retire hides, generation bumps once per page. Host (vitest): write-then-append ordering,
   cancel before commit leaves no event and no file, refusal codes, seq-0 replay on remount. Oracle: the written folder is read back by
   `bun:sqlite` (third party) and the archive by the TS archive codec; e2e: `verify home` gains `bind-studio-file` + `persist-studio` and
   `reopen` asserts they survive the reload.
7. **Scope for window 3+.** Guest part (space-home + space core) is a second prepared set after the current 38-file set lands; host part
   touches ShellHost + store worker + os wire schema (`BackboneWorkerRequest/Response`) → owner of the React shell lane (S18) or SH2 with
   a new `ShellHost/🏠️local-studio-catalog/` dir (R10 taxonomy registration needed).
8. **Alternative C found 13:4x (decide before the patch).** Guests already have host-owned async storage: kernel
   `Effect::StorageRead/StorageWrite/StorageDelete {req, key[, bytes]}` (wit `storage-read/-write/-delete`, browser bundle
   `hostAsync.storageWrite` → `effect-request`), executed by the plugin host's `dispatch_storage` against `services.storage_backend`
   with per-package byte quotas, a deadline and `ctx.cancel` (cancellation before dispatch answers `capability-revoked`) — i.e. the IO,
   quota and cancellation of step 3 exist. With it the guest's catalog port could persist through the host (Home's retained jobs
   await the `Respond` of their `StorageWrite` in the commit stage; Home open reads the catalog key) and no new shell route, worker
   request or page protocol is needed. Open questions (not verified): which storage backend the React shard host binds
   (persisted-local-only or memory), whether app-plugin guests (not only actor packages) may emit these effects under their
   capability grants, and how the retained job observes the `Respond`. B is proven-pattern but ~5 new host pieces; C is smaller if
   the backend persists. **Prepared patch not started** — the choice between B and C needs the React-host / plugin-host owner.
9. **Coordinator decision 13:4x: route B**, SH2 owns both halves. Because the event log is a new schema-first OS config vocabulary with
   Rust twins in `💻️os/🎚️config` (frozen), library taxonomy/schema-catalog rows and a plugin-host exhaustive case, the whole route
   lands as ONE window-3 set (`wp-sh2/b1/`, overlay-proven), modelled 1:1 on the opening-preferences vocabulary (`set-default-app`
   upsert keyed → `admit-local-document` keyed on `documentId`; `clear-default-app` → `retire-local-document`):
   - **vocabulary** `os.config.local-catalog` (`LocalCatalog { documents: [LocalDocument] }`, `LocalDocument { documentId, schema, name,
     storage: folder|file, target, admittedAtMs }`): leaves `📥️admit-local-document`, `🗑️retire-local-document` (manifest 🔣️.json,
     payload 🧬️schema, 🦀️.rs, 🟦️.ts, unit + fixture-vector tests, fixtures), dispatch in `🧬️mutations/{🦀️.rs,🟦️.ts}`, plugin-host case
     `🔌️plugin/🖥️host/🧪️tests/📇️mutate-os-config-local-catalog/`, oracle no-oracle decision, taxonomy + schema-catalog rows (R10).
   - **host TS**: `localCatalogActorConfig(actor, dataDir)` (worker, identity pattern: folder lane `${dataDir}/os`, else ephemeral);
     ShellHost region `🔖️LocalCatalog` (own module `🏛️ShellHost/📇️local-catalog/` — new dir, R10): open + fold at boot (local-first,
     no hub needed), replay route `os.local-catalog.admit {documentId, schema, name, storage, target, pack, spr}` = job (stage
     "writing": worker `open` with the folder/file binding + `send localDocumentArchive` + `close`; stage "recording": facet mutation
     `admitLocalDocument`; cancellable until the archive is sent; en + de notices), `os.local-catalog.retire {documentId}`,
     `os.local-catalog.open {documentId}` (openDocument with the entry's binding, as the sync card does); feed Home with
     `applyLocalCatalogPage {pageJson}` on Home mount and after every committed job.
   - **guest**: Home persist/bind/import commit stages on a host without a filesystem emit `os.local-catalog.admit` (pack/spr of the
     studio's `export_backbone_pack`) instead of refusing; `applyLocalCatalogPage` → Home config `localCatalogJson` projection → rows
     (origin local, `persistedLocalOnly`), their open → `os.local-catalog.open`; Remove-from-Home stays the config tombstone.
   - **laws**: leaf vectors (Rust + TS, same fixtures), plugin-host exhaustive case, worker/ShellHost vitest for the job (write before
     record, cancel before write leaves nothing), Home guest laws (commit emits exactly one admit; page apply idempotent), e2e
     `verify home` gains persist-studio + reopen-survives-reload.
10. **B1 build log (14:0x–):** stage `.🧬semio/🌐hub/s14-sh2-b1-stage`, base `…-b1-base`, list `wp-sh2/b1/b1-files.txt` (helpers
    `b1-stage.py add|rebase`, `b1-capture.py`, `b1-apply.py`; ShellHost's B1 base = the 38-file set's staged ShellHost, so B1 lands
    after it). Written so far (not compiled yet — overlay lane busy): the vocabulary (2 leaves × manifest, payload schema, Rust, TS,
    unit + vector tests, fixture quintet; dispatch Rust/TS; crate module decls + descriptor registration; oracle decision + catalog +
    manifest), plugin-host exhaustive case `📇️mutate-os-config-local-catalog` (feature, Rust, TS adapters), worker
    `localCatalogActorConfig`, host module `🏛️ShellHost/🗂️local-catalog/🟦️.ts` (request validation + target/binding resolution,
    whole-record envelope, archive codec, Home page arguments, en + de notices). Open path decided: the host keeps the durable copy;
    `os.local-catalog.open` re-hydrates the guest from the folder archive (pack/spr through the import lane) and then routes to the
    studio — later edits persist by persisting again (continuous folder sync of the studio app is a separate step).
11. **B1 state 14:4x (prepared, overlay-applied, verification queued):** 42 files (`wp-sh2/b1/payload/manifest.json`; dry run on
    the overlay 42/42 `apply`; on the tree 5 files conflict by design — their base is the 38-file set, so B1 lands right after it).
    Final shape: guest commits on wasm32 (persist → storage `folder` + the dialog's folder, bind → `file` + the path, import → `folder`
    + the device default) emit `os.local-catalog.admit {documentId, schema, name, storage, target, pack, spr}` (the studio's own
    `export_backbone_pack` pair, base64); the host validates (`localCatalogAdmissionV1`), writes through a worker actor with the
    folder binding and VERIFIES the write by reading the lane back (externalChanged → `documentArchiveReplaced` byte-equal), then
    records `admitLocalDocument` in the facet (envelope + archive) and hands the pair back as `applyLocalCatalogDocument` — a new
    chrome-audience retained Home job (validate: base64 + the pair is the named `s.space` document; commit:
    `import_os_space_from_pack` under its own id and manifest name — idempotent — retires the ephemeral draft, bumps the generation;
    never re-emits an admission). On every landing-app mount the host re-hydrates each kept document it has not handed that
    instance yet (read-back from its lane). Remove from Home of a persisted studio also emits `os.local-catalog.retire` (files stay
    on disk). Also fixed on the way (set 1, host 025): `import_os_space_from_pack` keeps the head manifest's name. Laws added
    (guest): re-hydration lists once under its name + idempotent + removal unkeeps; malformed/foreign pair refused by name; import
    commit emits exactly one admission whose pair decodes to the imported studio. `tsc` of the B1 host TS in the overlay: 0 errors in
    B1 code (the 3 remaining ShellHost errors are the overlay's dual-module-identity/stale channel-version class that the untouched
    identity `open` shares). Queued: overlay hold `ov12 b1` (config crate, space-home component-app-assembly, plugin-space, kernel
    lib tests). Not yet: generated test-host run of `📇️mutate-os-config-local-catalog`, the space plugin descriptor regen
    (`describe`, wasm lane, window 3), a live e2e.
- 14:4x **item 1 overlay proof (ov11, one hold 14:21–14:25):** space-home `component-app-assembly` **109/109**, space-home lib
  **28/28** (all four ov6 reds fixed); kernel lib-test compile red #2 found: `🧬️retained-clone/🧪️tests/🔬️unit/🦀️.rs:311` E0618
  — `let grant = grant(&fixture)` called a local `grant` binding shadowing the fixture fn (line 291) → the first binding renamed
  `copy_grant` (test-only, landed in the tree 14:28, backup `s14-sh2-backup/kernel-fixture/`).
