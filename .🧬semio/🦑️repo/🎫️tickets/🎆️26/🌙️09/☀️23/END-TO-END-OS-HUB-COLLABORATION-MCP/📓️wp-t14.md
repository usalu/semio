# WP-T14: Plugin/Framework Debt for Window 3 (F9 Content Ids + Carriers, P8 Orphan, H9-L Labels, 5b dsl_value!)

Slice T14, session 14 (2026-09-27 18:2x). Coordinator = main chat. Continues T13 (`📓️wp-t13.md` F9, F4, wfc clock, 5b) and
LC (`📓️wp-lc.md` P8 `orphan`, H9-L, F1). Rules: `📓️session-14-preamble.md` (+ 13/12). Ports 8060–8069 / 6560–6569 (none
used). Inputs + captures `wp-t14/` (`generated/` expendable); durable data + overlay `.🧬semio/🌐hub/s14-t14-*`. Native cargo
only via the `native` lane (build-fleet-b, private target `wp-t14/target`); overlay builds only via the `overlay` lane with
a private build-dir inside the overlay. Landing rows: `📓️landing.md` § Session 14.

### Session 14c

Successor T14 agent (2026-09-28 16:5x, after the 14:37 usage cut + app restart). Chain relaunched 16:55:46 (pid 76013), GUEST
FREEZE ON. Overlay `s14-t14-overlay` (+ `.t14-build` 13 GB) survived.

| # | item | state | evidence |
|---|------|-------|----------|
| 0 | reconcile: live tree carries no T14 guest edit beyond the landed rule-22 plugin-test fix | **clean 17:03** — `git grep` finds no `dsl_value!`/`T14_F9_RECORD`/`authoring_seed`/`follow_derivable_children`/`child_member_retirements`/TS `contentId` (the two remodeling `content_id` methods are committed 09-25 code); `land-w3.py --check`: F9 33 files / 0 problems to apply, G12 refuses until F9, class fix + P8 orphan (4 files / 17 hunks) + H9-L (100 sites / 94 files, 13/13 hunks to apply, 0 applied) + 5b A+B1 (21 files) + item 6 (2 files) all unapplied; B2 refuses until A; plugin-tests fix = "nothing to do (applied)" | `wp-t14/generated/s14c-land-check-1.txt` |
| 7 | (14b item 7, finished) LC F1 on the live tree | native hold n2 14:28–14:32: writer **ok**, vcs **ok**, jack **FAILED** — "one undo moves the whole run" (unchanged since session 13) → LC debt, not a T14 set | `s14-t14-logs/s14b-n2-F1.txt` |
| 8 | P9's 2 SDK lib reds (rule 22, test-only) | **LANDED 17:08**, native `--lib --tests` check rc 0 17:16, **both laws ok** (bijection: `cancelTypedOperation` is served by the head-of-`dispatch_action` arm → directly-routed residue + pinned arm; childless surface: `TableScene` now has lanes → `IconRenderScene`) | landing row, `s14c-sdk-reds-1*.txt`, `wp-t14/sdk-reds/` |
| 9 | class fix: genesis hooks for raster + note | **no hook — verified by source**: raster `assets` (`RasterOwnedMap<RasterAssetChild>`) and note `NoteTextChild { handle, paragraphs }` carry no `#[child(kind)]`, so `ArtifactCompositionFields::visit_child_refs` never visits them → `ChildRestoreProjection`, `seed_genesis_children` and `follow_derivable_children` never see them; their child content is EMBEDDED authority in the parent (raster `RasterAssetPackRow.content` = the child's canonical pack; note `paragraphs`), so no declared coordinate can name an unheld child (no orphan). Making them real members = a separate composition migration (declare the slot via a `ChildFieldRefs` impl, move content out of the parent pack, archive/restore), not a genesis hook | source: `🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` 🧩️Composition, raster `📸️snapshot` PackRecord, `🗒️note/🦀️.rs:285`, schema derive `✨️derive/⚙️expansion` |
| 11 | renderer `serde_json` (after 5b) | **census, not in this pass**: B2 already moves the renderer's literal action-arg builders (`scene_action` 29 callers, `block_list_action` 8, `canvas_addressed_action`) onto `DslValue` + `dsl_value!` (T13's "16 sites"); what remains is the renderer's OWN runtime serde_json debt — 16 files, `json!(` 256, `from_str` 167, `to_string` 61, `serde_json::Value` 60, `from_value` 24, `Map` 16, `to_value` 6, `DslValue::from(` 14 (Shell 125 lines, Scenes 104, EngineCanvas 55, renderer 32, Interpreter 23, ProgramBridge 16, HubSignIn 14, …); `serde_json = "1.0.140"` a runtime dep of `semio-framework-os-renderer-wgpu`. Replacing it = scene payloads through their `ToValue`/`FromValue` + `pack::json`, `json!`→`dsl_value!` (NOTE: `json!` maps sort keys, `dsl_value!` keeps written order → payload bytes + lane hashes change → fixtures), Value traversal → `DslValue` accessors: a compile-iterated work package of its own (window 3 or later), routed to main | `generated/s14c-renderer-serde-census-1.txt` |
| 1 | F9 content ids + id map + carriers | **patch final** (33 files). Maps: map-1 (owners), map-2 (fast owners after MAP), map-3/4 (process3d exact old ids with nested brep ids reverted; map-4 = the clean rows of map-3). `apply-map.py` re-sorts id-keyed JSON maps. Live dry run: 1 049 carriers, 4 463 ids, 279 re-sorted maps, 33 uncovered (describe outputs of demonstrator/cad/process, fixture-only remodeling assets never re-minted). Owners after MAP: block-3d, note, din18599, gisterrain green; remodeling diffs fixed by the re-sort; process3d re-verification after map-4 is pending (h8) | `s14c-map-4-apply.txt`, `s14b-s14c-h5/h6/h7-*` |
| 2 | G12 builder pass | applied in the overlay (129/129); compiled with the SDK and owners (green) | `s14c-a1-g12-apply.txt` |
| 3 | class fix (derivable children follow their coordinate) | **script changed + overlay-proven**: close deadlock fixed (retiring-member lease disposer); boot settle in the law harness; SDK green; architect declared-verb law, orphan verdict 2/2 and orphan laws (reasoning/flow/architect) ok; process3d no longer aborts | `s14b-s14c-h6-ORPHAN-*.txt` |
| 4 | P8 orphan | applied; verdict 2/2 + declared-verb laws ok (h6) | same |
| 5 | H9-L labels | 100 sites / 94 files; hub/wfc3d laws ok; app-router label path fixed (21:2x); host law re-run with `os-host-full` pending | `s14b-s14c-h5-*-LAW.txt` |
| 6 | 5b A + B1 + B2 | value law ok; B2 first compile found 5 bugs → fixed (live+A scratch: 55 files / 0 problems); N1 re-check pending (h8+) | `generated/s14c-b2-scratch-2.txt` |
| 10 | full ordered pass in the overlay | **applied 17:18** (sync 666 files; F9 33 + recorder + G12 129 + class fix + orphan + H9-L 100/94 + 5b A 21 + B2 53 + item 6 2, landing order); preflight: every set "nothing to do (applied)" (B2 gained an applied sentinel — it writes all-or-nothing; orphan 16/17 + fixture checked directly: 29 cases vs live 28, one `composedChildOrphaned`), 0 duplicated use/mod/import/fn lines, 0 spliced labels (1 expected: two function-scoped `const ajv`); 305 changed files → 94 crates (+`semio-framework-os-infinite` via B2 → N1). Hold h1 queued 17:23 (4th in the overlay FIFO): KERNEL → SDK → OWNERS-1 (records the id map) → MAP → OWNERS-2 → laws (kernel, host, wfc3d, hub, value, orphan verdict + declared-verb laws, SDK reds, TS oracle) → N1–N3 → W1–W2 | `s14c-apply-1.txt`, `generated/s14c-preflight-2.txt`, `generated/s14b-overlay-changed-6.txt`, `s14c-run-h1.txt` |

### Log 14c

- 16:5x read preamble 14 (rules 1–23, 14b, 14c), AGENTS.md, fleet log tail, this report. Lanes 17:03: overlay free, native 2 slots
  held, wasm = chain; load 69, 26 rustc, swap 4.9/6 GB, 84 GiB free.
- 17:03 item 0 reconciled (table). 17:08 item 8 landed live + queued in the native lane (2 slots): check rc 0 17:16; the cargo
  test step's summary was cut by my grep, so the two laws were re-run from the compiled test binary (no cargo): 2 passed.
- 17:1x item 9 (raster/note hooks) settled by source (table). The schema derive now accepts `#[child(kind)]` on ANY field type
  implementing `ChildFieldRefs` (raster's doc still claims it only recognizes bare `ArtifactChild`/`Vec` — stale).
- 17:09–17:18 overlay sync (base 3b2f1181d27) + full pass in landing order (`overlay-apply.sh`, now with `follow` + `b2` by
  default and the SDK-red fix verified); `overlay-preflight.py <n>` (new) = idempotency of every set + duplicate/splice scan +
  the changed-file listing `land-w3.py` compares. `overlay-hold.sh` gains SDK (stop-red), ORPHAN-VERDICT, ORPHAN-LAWS,
  SDK-REDS; state `s14c-state.txt`. Launcher waits for ≤ 14 rustc (rule 6), then the lane.
- 18:33 hold h1: KERNEL ok; **SDK red (B2 bug, never compiled before)**: `semio-framework` root still re-exported
  `action_bus::optional_json_to_dsl` (E0432) → B2 now also drops that re-export + action-bus's then-unused `use dsl::DslValue`.
- 19:00 hold h2 (loop `overlay-loop.sh`, re-queues itself after every hold): semio-framework ok; **SDK red**: B2's two world-3d
  measure-closure signatures used a bare `DslValue` (not in scope in `world3d_host`) → `{DV}` (= `semio_framework::DslValue`).
  Both fixes applied to the overlay and the script; B2 live dry-run needs A first (anchors of the 2 new hunks verified 1× live).
- 19:1x static review of the never-compiled class fix against the overlay SDK: registry/root helpers, field renames, retirement
  registries (`ArtifactFixedRegistry` `can_insert`/`insert_admitted`), `graph_mut().remove_owns`, hook return types
  (`step_framework_reserved_commit`/`advance_typed_operation_publication_one` → `Result<(), Fault>`), module privacy (`pub mod app`) —
  no mismatch found. Hold policy: stop only on KERNEL/SDK/MAP red or an OWNERS compile red; law + N/W steps record and continue
  (one hold shows every red). Loop restarted at h3 (19:10, 4th in FIFO).
- 19:29 h3: **SDK green** (85 warnings = type-checked). OWNERS-1 recorded the id map (4 250 rows) — remodeling's lib tests alone
  take 1 313 s, so the 28-min deadline cut curation; process3d's test binary aborted (panic in Drop during unwind).
  Split: OWNERS-1C (curation), OWNERS-1P (process3d, skipping the aborting test), OWNERS-1X (writer, animate, playbook,
  imperative — owners of carrier ids the first map did not cover), all appending to map-1; OWNERS-2 = the 9 fast owners;
  OWNERS-2R = remodeling's failing tests only. Map dry run: 1 037 carriers, 4 353 ids, 49 uncovered (describe outputs of
  demonstrator/cad/process + fixture-only asset ids never re-minted).
- 19:59 h4: MAP applied; OWNERS-2 after MAP: block-3d 351/0 (was 18 red), note 401/0 (was 3), din18599/gisterrain/raster green;
  architect 6 red, process3d aborted, remodeling 8 red. **Live-tree baselines (native lane)**: process3d's 7 reds all PASS
  live, architect's 5 CSV/TSV round-trip reds FAIL live too (pre-existing peer drift: "unsupported register import:
  audit_events") while its declared-verb law passes live.
- 20:0x **class-fix defect found (my predecessor's never-compiled design)**: close stalls ("never reached its terminal-empty
  witness"; architect declared-verb law: "child member is waiting for its required domain-owned bounded disposer"). Cause: a
  followed child left `children` at once, but the previous content root still lends that member's snapshot; the root's
  retirement needs the member LIVE to dispose the lease, and the member's retirement waits for the lease → deadlock (the
  close ladder also returned the Blocked member step before ever reaching content retirements). Fix in `follow-children.py`
  (live dry run 0 problems, applied to the overlay as a delta): `ChildContentRetirement::close_step` takes the member
  retirement registry and finds a retiring member as the lease disposer (`child_snapshot_owner`); the member retirement step
  waits (Blocked) while its member still lends a snapshot; the close ladder falls through to content retirements when the
  member step is Blocked; the follow pass defers (and repeats) while the named child is still retiring (undo back to it).
- 20:1x **carrier order**: 5 remodeling `produces_committed_diff` reds = the committed diff's `durableArtifacts` (a BTreeMap,
  ascending keys) keeps the OLD order after the id swap. `apply-map.py` now re-sorts every id-keyed JSON map that was ascending
  before the swap (only files that are the exact `indent=2` rendering): 279 maps on the live dry run. Overlay carriers
  re-cloned from live (1 046) so h5's MAP re-applies with the re-sort. Never edit the hold/loop scripts while they run (zsh
  re-reads them: h4 mixed old/new step lists, the loop died on a parse error; restarted as h5).
- 20:15–20:43 h5: SDK green with the deadlock fix; OWNERS-1X + MAP (1 042 carriers, 4 369 ids, 279 maps re-sorted);
  OWNERS-2: block-3d/note/din18599/gisterrain/raster green, process3d NO abort any more; remodeling 2R: only its 2 pre-existing
  reds (live baseline fails them too); **laws ok**: kernel content-id + addressing 3/3, wfc3d kind 1/1, hub linked-kind 1/1,
  value `dsl_value!` 1/1 (HOST-LAW ran 0 tests: needs `--features os-host-full`, fixed for h7).
- 20:3x architect declared-verb law in the overlay: 8 View/Shell verbs "wrote the document" — the fixture boot loads the example
  via `load_document_pack` (no follow hook), so the first verb's follow pass opened the example's derivable children. The law
  harness (`declared_verb_fixture_app`) now settles derivable children after the boot load (what the runtime's next backbone
  tick does) → h6: **architect declared-verb law ok**, only its 5 pre-existing CSV/TSV reds remain (fail live too).
- 20:43–21:11 h6: **ORPHAN-VERDICT 2/2, ORPHAN-LAWS ok (architect 1, flow 2, reasoning 1), SDK-REDS 2/2, TS oracle ok** (re-run
  explicitly: shared addressing + content id vs hashlib/node:crypto, `generated/s14c-ts-oracle-1.txt`). N1 **red** on B2 (first
  compile of B2 outside the SDK): infinite world `scale: Option<serde_json::Value>` + 2 `Vec<serde_json::Value>` targets and 8
  test uses of the deleted `action_args`; Shell `authors: Vec<Value>`; and H9-L wrote `semio_framework_plugin::LocalizedLabel`
  into the plugin-host app-router test (that crate has no plugin-SDK dep). Fixed in `5b/dsl-value-b2.py` (5 hunks; path rule:
  `semio_framework_plugin::` only when the file does not name `semio_framework::` AND its crate depends on the plugin SDK) and
  `h9l/kind-label-patch.py`; B2 re-checked on a live clone + phase A (`s14-t14-scratch`): 55 files / 0 problems; the overlay
  got the same bytes. process3d's remaining 18 reds: its `steps-flow` content embeds the tool brep ids, so the old id can only
  be rebuilt from the content with the nested new brep ids reverted — the overlay-only recorder now does exactly that
  (`record-instrument.py`: brep exact rows + in-process nested map; atomic line writes) → map-3 in h7.
- 21:00 **WINDOW 3 OPEN** (L1 trains; T1 runs my `land-w3.py` SETS). 21:2x RELAY L1 sent: which scripts changed
  (class fix, B2, H9-L, apply-map re-sort), maps 1+2 final, map-3 for process3d pending one overlay turn, N2/N3/W not proven.
  Live baselines (native lane): sequence 2 and curation 12 reds fail on the live tree too (pre-existing, not T14).
- 21:3x–21:53 h7: map-3 recorded with escaped tabs (my recorder literal was double-escaped) → map-4 = its 72 exact rows
  (38 steps-flow incl. the rename-step pair); maps 1–4 on the overlay: 20 more files / 44 ids, 33 uncovered. Recorder fixed.
  OWNERS-V after MAP2: architect 5 (pre-existing CSV), raster 1 NEW red `retained_image_export_completes_or_cancels_without_
  mutating_history` ("media export job faulted"; not yet baselined live), sequence 2 / curation 12 (pre-existing).
- 21:4x RELAY S19: no T14 set touches its symbols; `ChildContentRetirement::close_step` gains a `retiring` argument.
- 22:3x S18 relay (kind picker shows "Editor" for every kind): Rust `artifact_kind_choices` already reads the kind label; the TS
  twin `artifactKindChoices` (`🛂️manifest/🟦️.ts`, host TS, open) read the app label → now the kind label (app's
  `artifactKinds`, then the manifest's, by schema) + law "every choice carries its kind's own label, no two share one" in
  `🧪️hostresolvedargs/🟦️.ts`; framework `tsc` shows only 4 pre-existing errors (none in manifest); the vitest run did not match
  the file filter (not run yet). The hub `artifact_creation_catalog` already resolves `kind.label` (Native, en + de). Source
  scan: no two distinct kinds of one plugin share an en label; the current generated descriptors carry no kind `label` yet
  (describe output predates H9-L) → regen below.
- 23:5x **T1 hotfix (L1, authorized in-tree)**: T1 native + wasm32 reds from h9l — host `seed_builtin_artifact_kinds` 4×
  `name:` → `label: LocalizedLabel::native(…)` (Parameter/Parameter, Workflow/Workflow, Space/Space, Collection/Sammlung);
  semio-framework root now re-exports `LocalizedLabel` (+ `Locale`, `Terminology`) from its canonical home
  `semio_framework_os_kernel` (was `ui_wgpu::wgpu`, a re-export of the kernel type); stdio `native_openable_provider` test
  `"name"` mutation → `"label"` (+ fixture case `foreign-label`). Check `-p semio-framework-os -p semio-framework -p
  semio-framework-plugin-host --lib --tests` **rc 0 00:00** (`s14c-hotfix-1.txt`); gate 2 (re-check with the kernel-home
  re-export + the stdio test) queued with priority stamp (`s14c-hotfix-2.txt`). Descriptor regen = `bun nx run-many -t
  describe --exclude @semio-tech/os-plugin-describe-rs --parallel=2` (describe → component-dev: wasm lane), when L1 frees it.
- 00:12–00:16 gate 2: re-check with the kernel-home re-export **rc 0**; stdio `native_openable_provider` **7/7** (`s14c-hotfix-2.txt`).
  T1 GREEN (L1). Descriptor regen done by L1 (not duplicated).
- 02:4x (after kernel panics + 01:14 sweep; my scripts survived, captures live under `s14-t14-logs`) coordinator items:
  (1) **picker "Editor" root cause**: all 75 editor apps carry the SDK default app label "Editor"; the pre-T1 hub
  `artifact_creation_catalog` labelled choices by `app.label` (git 5bcb2da23d) → H9-L (T1, 27d829d8a5) reads `kind.label`; TS
  twin `artifactKindChoices` fixed (22:3x) — **vitest 2/2** (existing + new law, in-source suite via a scratch config
  `s14c-manifest-vitest.config.ts`, `s14c-kindchoices-vitest-2.txt`); rule 20: os tsc 0 + Home boots 0 faults since (G12/S18 02:1x–02:2x).
  Live confirmation needs a hub from this tree + a catalog republished from regenerated descriptors (p24 descriptors carry no
  kind `label`); check `wp-t14/kind-label-check.py` (02:4x mid-regen: 28/105 kinds labelled). (2) **gen2d/3d hub create crash**:
  S19's `gen-archive-load` (genesis owners → `retire_unadopted` retiring snapshot + edits) is live after T1; 7800's p24 procedural
  component predates it. Native proof = S19's genesis + archive laws on the live tree, queued behind wasm-idle + load < 32
  (`s14c-genesis-laws-1.txt`).
- 03:25–03:29 **gen2d/3d create path proven natively on the live tree** (after T1): gen2d `a_hub_genesis_pair_is_produced_and_
  parses_back_without_trapping` + archive-door law **2/2 ok**, gen3d **2/2 ok** (`s14c-genesis-laws-1.txt`) → S19's set covers
  the hub crash; 7800 needs a procedural package published from this tree (the all-34 chain after T2–T4).
- 03:3x kind-label-check after L1's regen (L1: rc 1, 88 "no label"): the CHECK's rule was too narrow — viewers declare no kind
  (they share the editor's). Package-wide lookup → 51 left, all "kind schema ≠ app io schema" = exactly W4's 9 packages (+ demonstrator
  via sourcing) fixed by `wp-w4/w4-kind-spec-identity.py`, plus space Home (`s.home`, no kind: not creatable). W4's set re-diffed:
  its forms anchor still said `name:` (H9-L made it `label:`) → fixed; **dry run 42 pending / 0 bad**; no extension needed.
  New T3 set **`wp-t14/kind-choices/kind-choices.py`** (`--dry-run|--write|--revert`, backups `s14-t14-land/kind-choices/`, dry run
  **8 pending / 0 bad**), lands WITH W4's set: TS `artifactKindChoices` + Rust twin `artifact_kind_choices` offer exactly the
  package's DECLARED kinds (package-wide by schema), labelled by the kind, no app-label fallback; TS law extended (viewer shares its
  editor's kind, no undeclared kind, unique labels) — **vitest 2/2 with the set applied** (`s14c-kindchoices-vitest-3.txt`), then
  reverted live to the booted 22:3x state (vitest 2/2, `-4.txt`); the Rust hunk is written, not compiled (unused twin; T3 check).
  `kind-label-check.py` now mirrors that rule: **rc 0, 95 picker kinds, 0 problems, 25 editor apps "not creatable" today** (the W4
  packages until T3 + Home) (`s14c-kind-label-check-3.txt`).
- 03:38 rule 23: deleted my overlay build-dir `.t14-build` (25 GB), `.t14-target` and the scratch clone `s14-t14-scratch`
  (disk 118 → 135 GiB free); overlay sources kept. Hold h8 (22:18) died in the 22:42 kernel panic before a step finished; no
  T14 process runs. Stopped (coordinator: free a slot). Open for T3: W4 identity set + `kind-choices` (one train), then
  `kind-label-check.py` after the all-34 chain; raster `retained_image_export_completes_or_cancels_without_mutating_history`
  (overlay-only red, not baselined live) and process3d fixtures (maps 1–4) to be confirmed by T1's owner tests.
- 05:0x **T3 revert (L1 04:58)**: `w4-kind-spec-identity.py`'s fem `document_artifact_kind()` template still wrote `name:` → now
  `label: LocalizedLabel::native("FEM 2D Model", "FEM-2D-Modell")` (3D alike; distinct from the results kinds). Sweep of all three
  scripts: no other `name:` on `ArtifactKindSpec`/`OsArtifactDescriptor` (the gate fixture's `"name"` keys are case names). All three
  scripts gain `--root <tree>`; kind-choices backups split `live/`/`scratch/`. Live dry runs: 42/0 bad, 8/0 bad, 8/0 bad. Scratch
  `s14-t14-kindsets` (APFS clone of the live tree) with all three applied (29 crates + demonstrator); proof
  `wp-t14/kindsets-proof.sh` (private seeded build-dir, `--lib --tests` check, then `artifact_kind`/`kind_identity` laws) queued behind
  wasm-idle + native-idle + load < 32 (rule 25), captures `s14c-kindsets-*.txt`.
- 05:55–06:10 (coordinator: native lane, load < 16, priority stamp) scratch proof **GREEN**: `--lib --tests` check of the 29 touched
  crates + demonstrator (fem app features) **rc 0** (4 m 32 s, private seeded build-dir); laws **14 passed / 0 failed**
  (`kind_identity_law_matches_the_fixture` + every `artifact_kind_names_the_store_schema`); kind-choices TS law vitest 2/2 (earlier).
  Live re-dry-runs 42/0, 8/0, 8/0 bad. Scratch clone + build-dir deleted (disk 106 GiB free). RELAY L1: re-land in order
  identity → gate → kind-choices. Stopped.
- 10:2x **CHAIN FIX (critical path; freeze lifted for it by the coordinator)**: the all-34 chain failed twice at components → describe,
  only flow: "none of its 1 declared artifact kinds is owned … flow.host_snapshot: artifact codec schema has no structural record
  specification" — W4's identity set keyed flow's kind on `FLOW_DOCUMENT_SCHEMA`, whose hand-rolled `ArtifactPack` answered
  `record_spec() = None`. Root fix (precedent: writer's snapshot, stdio `txt`): `FlowSnapshot` derives `dsl::DslRecord`
  (`#[dsl(extension = "flow")]`) and `record_spec()` returns `Some(Self::__dsl_spec())` — the hash fingerprints the snapshot record
  (`schema` + composed `content` handle); the JSON pack body is unchanged. File: flow `🧬️schema/📸️snapshot/🦀️.rs`. **Native check
  `--lib --tests` of flow + 9 extensions + describe crate rc 0 (10:28), flow codec/pack laws 5/5** (`s14c-flow-spec-1.txt`);
  **`bun nx run @semio-tech/flow-plugin:describe` rc 0 10:29** (gate passed; descriptor declares `computation.flow` =
  `flow.host_snapshot`, label Flow/Fluss; `s14c-flow-describe-1.txt`). Relayed: relaunch `--from components`. Landing row added.

## Session 14b

Successor T14 agent (2026-09-28 12:0x, after the usage cut + app restart). Guest freeze ON since 12:02:46 (chain pid 45604).

| # | item | state | evidence |
|---|------|-------|----------|
| 0 | reconcile: no T14 edit in the live tree | **clean** 12:1x — `git grep` finds no `content_id`/`dsl_value!`/`contentId`/`T14_F9_RECORD`/`authoring_seed`; every prepared set dry-runs as not-yet-applied on the live tree after the peer's overnight ~1 870 edits: F9 33 files / 0 problems, H9-L clean (186 sites), 5b A+B1 21 files / 0 problems (19 B1, **53** B2 sites, was 51), P8 orphan 4 files / 17 hunks, G12 refuses until F9 (130 files) | `wp-t14/generated/s14b-*-dry-1.txt` |
| 1 | F9 content ids + carrier map (one pass) | patch dry-run clean on live (33 files) and applied in the overlay; kernel check green with all base sets (13:46); TS content-id oracle ok (hashlib + node:crypto); id-map recording hold queued (16th) | `s14b-h3-TS-ORACLE.txt`, `s14b-h3b-KERNEL.txt` |
| 2 | G12 builder pass (same gate as F9) | applied in the overlay (129 literal files); compile proof in the same hold | `s14b-a4-g12-apply.txt` |
| 3 | class fix + P8 orphan (one pass after G12) | SDK route written (`p8class/follow-children.py`, dry run 0 problems); orphan applied in the overlay; proof = hold 4 (class fix + 5b B2 + orphan/declared-verb laws) | `wp-t14/p8class/` |
| 4 | H9-L kind labels (native + wasm32 proof) | patch FIXED (double edit inside `-> ArtifactKindSpec` fns): 100 sites / 94 files, dry run clean; proof in the queued hold (N1–N3, W1–W2, host/hub/wfc3d laws) | `generated/s14b-h9l-dry-2.txt` |
| 5 | 5b `dsl_value!` + B2 codemod; renderer `serde_json` 16 | A+B1 applied in the overlay, **value law ok** (vs serde_json incl. borrowed values); B2 written (53 files, dry run clean) → hold 4 | `s14b-h3-VALUE-LAW.txt`, `generated/s14b-5b2-diff-3.txt` |
| 6 | remove `🔌️plugin/🦀️.rs` per-app fallback wrappers | only puzzle2d's dead `selection_ids` (0 callers) remains → `item6/fallback-wrappers.py` (2 files, dry run clean), in the overlay proof | `wp-t14/item6/` |
| 7 | verify LC F1 / T13 F4 / wfc clock laws (native lane) | **F4 ok, clock ok, wfc inferences 72/0, wfc fill 41/0**; F1 re-queued (writer test red at 14:01 from a peer docx change, fixed live 14:06) | `s14b-n1-*.txt` |
| 8 | rule-22 test-only fix: plugin lib tests (H13) | **LANDED 12:49, native `--lib --tests` rc 0 14:01** | landing row, `s14b-n1-PLUGIN-TESTS.txt` |

### Log 14b

- 12:0x read preamble 14 (rules 1–21 + 14b), AGENTS.md, fleet log, this report. Predecessor's last state (from captures, the
  log above ends 20:0x): hold 1's 105-crate native check never ran (zsh: an empty `-p` before `--features` broke the line,
  `hold1-native.txt`); hold 2 (21:12–21:27) applied F9 + recorder + G12 + orphan + H9-L in the overlay, **kernel content-id +
  addressing laws 3/3 ok** (`hold2-kernel-laws.txt`), everything else rc 101 upstream of T14 — the synced snapshot had a peer's
  half-edit (`semio-framework-ui` E0432 `kernel_3d_scene::projection_spec_*`, `semio-framework-os-kernel-db` `PlannedEntries`),
  so no id map was recorded (`hold2-map-apply.txt`: map file missing). Lanes at 12:12: overlay sh2, native g12 (+ en2, sh2 queued),
  wasm = chain; load 43, swap 4.5/6 GB, 100 GiB free.
- 12:1x reconcile (item 0 above). `overlay.py sync` gains `<base-commit>`: tracked files git deleted since the base are deleted
  in the overlay too (the mtime walk cannot see a deletion of an older file). Overlay sync 3 (base 6b8089dcb21 = HEAD before
  yesterday 19:20) running outside the lane (file clones only).
- 12:1x–12:29 sync 3: 1 969 files re-cloned (0 deleted), 700 s (`hold3-sync.txt`). Applied in the overlay (no lane):
  F9 33 files / 0 problems, recorder, G12, P8 orphan, H9-L, 5b A+B1 (21 files), item 6. **G12 finding:** its walk skips every
  directory whose path contains `/.🧬semio` — with `--root <overlay>` (which lives under `.🧬semio/🌐hub/`) it silently patched 0
  of its 129 literal files (hold 2 too: "2 file(s) … 0 with literals"). Through a symlinked root (`wp-t14/generated/ov`) it
  patches 129 files / 0 problems (`hold3-g12-apply-2.txt`). The live tree is unaffected (root has no `.🧬semio` prefix).
- 12:2x item 6 source: the per-app fallback wrappers are gone except puzzle2d's `selection_ids` (singular `id` fallback, **0
  callers**); procedural's `transforms::selection_ids(ids, fallback)` is the current-selection default, not a key fallback.
  Patch `wp-t14/item6/fallback-wrappers.py` (deletes the wrapper, rewrites the SDK doc's "keep their own fallback wrapper for
  now"): dry run 2 files / 0 problems.
- 12:2x lane tooling: `deadline.py` (absolute deadline, kills the whole process group incl. rustc, exit 124),
  `overlay-hold.sh <tag>` (resumable 28-min overlay hold: OWNERS-1 recording the id map → MAP → OWNERS-2 → kernel/host/wfc3d/hub/
  value laws → orphan verdict + laws → TS oracle → native N1 core 8 / N2 plugins 63 / N3 stdio 37 → wasm32 W1/W2 103 guest crates;
  state `s14b-state.txt`, a killed step resumes next hold on its kept units), `native-hold.sh` (item 7, same pattern, live tree,
  build-fleet-b + private target), `overlay-apply.sh` (sync + all sets), `s14b-groups.zsh` (crate groups from the 108 crates the
  applied sets touch, `generated/s14b-crates-3.txt`). Queued 12:29 overlay (5th) and 12:30 native (7th).
- 12:3x–12:4x **5b phase B2 written** (`wp-t14/5b/dsl-value-b2.py`, dry run on the overlay 53 files / 0 problems, diff
  `generated/s14b-5b2-diff-2.txt`): deletes `action-bus::optional_json_to_dsl`; app wrappers (generation3d ×2, gis2d, lowpoly,
  note, puzzle2d + its menu-row closure, layout menu item) and the SDK world-3d measure closures take `Option<DslValue>`; literal
  callers → `dsl_value!`, lowpoly's 7 DslValue→JSON→DslValue round trips and puzzle3d/5d's 4 closure round trips dropped; layout
  interaction args + infinite world action args (23 literals, scale, brush-mesh pages) built as DslValue (JSON text via
  `pack::json::to_json_string`); wfc ×5 fill effects literal. Renderer: `scene_action` (29 callers) / `block_list_action` (8) /
  `canvas_addressed_action` take DslValue with `dsl_value!` callers; its JSON-sourced args (parsed operations, world3d cancel maps,
  effective staged maps, tutorial history, fixture vectors in tests) convert where they enter the descriptor via the value module's
  `DslValue::from(serde_json::Value)` — the renderer's wider serde_json use (scene payload parsing) stays a separate debt. Macro
  change: the fallback arm converts by method call (`(&expr).to_value()`), so `&String`/`&[String]`/`&&str` auto-deref like
  `json!` accepts them; the law gains those cases. B2 is NOT in hold 3's overlay (a compile error in the SDK would sink the F9 map
  recording); it goes into the next hold after the map is recorded.
- 12:4x coordinator: T14 owns the content-addressed-child class fix too (prove F9 + class fix + P8 orphan together, reasoning
  law green, one window-3 pass after G12). G12 fixed its `--root` walk (overlay-apply.sh uses the overlay root directly again).
- 12:49 **LANDED (rule 22, test-only, H13 relay):** `semio-framework-plugin` lib tests compile fix — 3 files
  (`wp-t14/plugin-tests/plugin-lib-tests.py`, landing row). Not compiled yet: native hold `n1` step PLUGIN-TESTS (6th in FIFO;
  coordinator: no priority stamp). Also applied in the overlay.
- 12:5x **class fix design = SDK route** (approved): `wp-t14/p8class/follow-children.py` (dry run 0 problems, live + overlay).
  `VcsArtifactApp::follow_derivable_children` after every parent-lane change (`handle_action`, each typed-publication turn incl.
  framework reserved commits, each backbone tick; gated by the parent store generation, deferred while a child admission is in
  flight): opens each declared-but-unheld child `ArtifactApp::genesis_child_pack` derives (the `seed_genesis_children` restore
  path) and retires each held child whose slot now names another derived child — member map (new backward-shift `remove`),
  ownership graph (`remove_owns`), child-content root (new copy-on-write `without_member`; previous root → bounded content
  retirement) → bounded member retirement (the admission-abort registry renamed to the ONE `child_member_retirements`, same
  maintenance stage 20 + close drain). Genesis hooks: every declaring plugin of the 12 already has one; raster (`assets` map) and
  note (`NoteTextChild` wraps the handle inside an enum variant) declare NO child in the restore projection, so the rule and the
  orphan law do not see them — no hook needed (correcting my 12:5x message). Laws: the P8 orphan verdict case + AJV twin and the
  reasoning/flow/architect declared-verb laws (reasoning `addNode`/`addRelationship` must stop orphaning). Not in hold 3 (an SDK
  compile error would sink the F9 map); hold 4 = class fix + 5b B2 + orphan laws.
- 12:5x overlay value module refreshed and 5b A re-applied (the overlay had the pre-12:40 macro without method-call conversion).
- 13:0x B2 pre-fixes for values `ToValue` cannot take (serde `Value`s): infinite world hover `targets`, layout preflight
  `issue_value`, renderer table `selectRow` `row` (→ `DslValue::from(row)`); B2 dry run 53 files / 0 problems
  (`generated/s14b-5b2-diff-3.txt`).
- 13:21 **lost overlay turn:** h3 got the lane 13:21:26 and ended in 0 s — `STEPS=(${@:-a b c})` expands the default as ONE word
  in zsh ("unknown step", rc 2). Fixed in `overlay-hold.sh` + `native-hold.sh` (`STEPS=(a b c); (( $# )) && STEPS=("$@")`,
  verified), state reset, re-queued 13:21:51 (~13th); asked main for a priority stamp.
- 13:27–13:38 h3 (restored stamp 20260928122949): **red from step 1 by my own re-apply** — re-running `5b/dsl-value.py` in the
  overlay (after the 12:5x value refresh) inserted the kernel re-export twice (`pub use protocol::dsl_value;` ×2 → E0252 in
  `semio-framework-os-kernel`), so every crate above the kernel failed (captures `s14b-h3a-*`). Measured regardless:
  **VALUE-LAW ok** (`dsl_value_literal_matches_serde_json_and_keeps_written_order`, incl. borrowed `&String`/`&[String]`/`&&str`,
  `s14b-h3-VALUE-LAW.txt`), **TS-ORACLE ok** (`verify shared-artifact-addressing oracle`: `testContentIdOracle` — hashlib
  vectors, `ContentId` schema, first-party SHA-256 == `node:crypto` incl. padding boundaries, `s14b-h3-TS-ORACLE.txt`).
  Stopped my hold 13:38 (own pids) to free the lane. Fixed: overlay duplicate removed; `exact()` in `dsl-value.py` and
  `dsl-value-b2.py` now treats an insertion whose full new text is present as applied. Re-queued 13:39 (14th).
- 13:4x coordinator granted the stamp ONE last time with conditions, all met before requeueing
  (`generated/s14b-overlay-idempotency-1.txt`, `s14b-duplicate-check-1.txt`): every set reports its applied state on the
  overlay — F9 (new sentinel check: content_id + TS contentId + schema ContentId + TS oracle + runner → "nothing to do"), G12
  (129/129 applied), H9-L (13/13 hunks applied, 0 literal sites; its insertion hunks no longer re-apply), 5b (0 files), item 6,
  plugin-tests (both had the same insertion re-apply bug → fixed), P8 orphan (`wp-t14/verify-applied-p8.py`: 16/17 hunks
  applied; the 17th is the fixture rewrite, which re-reads the fixture — the fixture holds the orphan case and its `why` rule
  exactly once); duplicate check over the 271 touched sources vs the live tree: no `use`/`mod`/`import`/macro line and no
  fn/struct/const definition duplicated (only expected JSON-key repeats). Hold script: KERNEL check first; stops at the first
  compile-red step (test failures in crates that compiled continue as data); control flow simulated with a stub runner.
  Re-queued 13:4x with stamp 20260928122949 (head of the FIFO).
- 13:45–13:48 h3: **KERNEL ok** (`semio-framework-os-kernel` check with F9 + G12 + H9-L + 5b A/B1 + item 6 + orphan,
  `s14b-h3-KERNEL.txt`); OWNERS-1 **compile-red → STOP-RED (by design)**: `semio-s-artifact-stdio-binary` corrupted by MY H9-L
  patch — `LITERAL` also matched the fn signature `-> ArtifactKindSpec {`, so the function body and the struct literal inside it
  both produced an edit at the same `name:` site; applied back-to-front they spliced the label twice
  (`label: …native("Binary", "Binärdatei"),_plugin::LocalizedLabel::native(…),`). Every file whose kind spec lives in an
  `-> ArtifactKindSpec` fn was hit (the predecessor's "186 sites / 94 files" was this double count). Fixed: `->` contexts
  skipped + edits deduplicated with an overlap guard → **100 sites / 94 files**, 13/13 hunks, dry run clean on the live tree
  (`generated/s14b-h9l-dry-2.txt`). Per the coordinator's condition I wait my turn now (no stamp). Overlay re-sync + base-set
  re-apply running (`overlay-apply.sh s14b-a4`, detached, log `s14b-apply-4.txt`); B2 + class fix are opt-in extra sets now
  (`overlay-apply.sh <tag> <base> [b2] [follow]`) and stay out of the F9-map hold.
- 13:54 overlay re-sync (782 files) + base sets re-applied (`s14b-apply-4.txt`); verified: every set "nothing to do (applied)"
  (`generated/s14b-overlay-idempotency-2.txt`), no duplicated use/mod/import/macro/fn line vs live and no spliced label line
  among the 102 new `label:` lines (`generated/s14b-duplicate-check-2.txt`); readiness gate `s14b-overlay-ready` written; hold
  queued 13:49 (16th, no stamp).
- 13:57–14:16 **native hold n1 (live tree, item 7 + rule-22 gate):** PLUGIN-TESTS **rc 0** (plugin lib test type-checked, 440
  warnings); T13 F4 **ok** (dag + raster `viewer_never_mutates`); wfc solve clock **ok** (`the_logical_clock_advances_…`);
  wfc ×5 `inferences` **72 passed / 0 failed**; wfc ×5 `fill` with `component-app-assembly` **41 passed / 0 failed**; LC F1
  **not measurable at 14:01**: writer lib test red (`DocxSnapshot.document` gone — peer docx change; fixed live by another slice
  14:06:42, `project_document()`), so jack/vcs never ran. F1 re-queued alone (hold n2, 14:16). Captures `s14b-n1-*.txt`.

## Session 14

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | F9 content ids + carrier regeneration plan (one pass) | **patch extended + dry-run clean** (33 files: 29 minting sites = T13's 23 + wires, cad, raster ×2, remodeling asset, din18599; TS sha256/contentId twin; child schema `ContentId`; Rust + TS laws vs hashlib); **carrier census 1 085 files** (architect 537, remodeling 281, note 81, block3d 80, process3d 42, …; 30 = plugin 🔣️.json/descriptor, regenerated by describe); no owner has a regen switch → mapping plan (below); overlay proof queued | `wp-t14/f9/content-id.py`, `generated/f9-carrier-census-1.txt` |
| 2 | P8 `orphan` after F9 | **F9 is NOT its precondition** (source): the orphan is wires re-minting its content-addressed child on every parent edit — F9 changes the hash, not the re-mint; P8's own precondition is the content-addressed-child CLASS fix (child keeps its coordinate, edits on the child lane — flow's P8 pattern / gismap's stable admitted ids). Orphan set dry-run clean (4 files, 17 hunks); overlay measurement (verdict + reasoning/flow/architect laws with F9 + orphan) queued in hold 2 | `wp-p8/patches/p8-orphan.py` |
| 3 | H9-L per-kind labels: overlay proof (native + wasm32), land before S19 1b | **patch fixed** (T14 copy: fixture needs both terminology rows — the hub law `to_value == fixture` was red-by-construction; manifest schema `ArtifactKindFormatsFixture` `name`→`label`); dry run 186 sites / 94 files + 13/13 hunks; overlay native check of 105 crates running (hold 1) | `wp-t14/h9l/kind-label-patch.py` |
| 4 | 5b `dsl_value!` + codemod; renderer `serde_json` 16 sites | **phase A + B1 written, dry-run clean** (21 files): `dsl_value!` (json!-grammar tt-muncher over `ToValue`, written entry order) in the value module + law vs `serde_json::json!`, kernel/`semio_framework` re-exports, 19 literal sites; **51 B2 sites** need type changes (wrapper params `Option<serde_json::Value>` → `Option<DslValue>` with 5–32 callers each, puzzle's internal `Value` plumbing, renderer 16 programmatic maps, fixture-driven tests) — only compile-iterable on the live tree (window 3) | `wp-t14/5b/dsl-value.py`, `generated/5b-dry-1.txt` |
| 5 | verify window-2 landings (LC F1 helper, T13 F4, wfc solve clock) on the current tree | **blocked: tree red upstream** (Codex peer 18:35–19:07: `semio-framework-schema` E0658/E0308, then `semio-framework-plugin` E0004 `NoPresenceMutation`); my run stopped to free the native lane; re-run when green | `s14-t14-logs/item5-*.txt` |

### Log

- 18:2x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED line yet; W4 unblocking),
  `📓️wp-t13.md`, `📓️wp-lc.md`, `📓️audit-s13-window3-inventory.md`, session-13 coordinator log 14:00→16:0x. Lanes free
  (native/wasm/overlay queues empty), 0 rustc, 141 GiB free, swap 7.6/9.2 GiB.
- 18:3x found in the session-13 captures (after both reports ended): T13's window-2 gate `wp-t13/generated/lw2-check-2.log`
  had the wfc `component-app-assembly` test targets red (grid2d/grid3d lib test, 2 errors each) and the bitmap relay timing
  law red; T13's follow-up `lw2-check-3.log` (16:30–16:32) shows both green after a fix T13 never reported. LC's F1 run 9
  (`.🧬semio/🌐hub/s13-lc-laws/f1-laws-2.txt`, 16:27): writer **ok**, vcs **ok**, jack **FAILED** ("one undo moves the whole
  run": after the run, one undo leaves the typed text instead of restoring the query). Re-measured on the current tree in item 5.
- 18:37 item 5 run 1 launched (native lane, `wp-t14/item5-verify.sh`, log `.🧬semio/🌐hub/s14-t14-logs/item5-run-1.txt`); lane
  18:58. Check **rc 101 19:04**: `semio-framework-schema` `✅️validator/🦀️.rs` E0658 `str_as_str` ×2 + E0308 ×2 (Codex peer's
  18:35–19:07 edits); wfc-app check + F1-law build rc 101 on `semio-framework-plugin` E0004 (`NoPresenceMutation`,
  `NoTransientMutation`, `NoConfigMutation` "non-empty" — the peer's value-derive change). Nothing of item 5 is measurable on
  this tree; stopped my run 19:35 (own pids 3835, 41943 + 3 rustc, 44597, 44593) to free the lane. Re-run when green.
- 19:0x F9 re-census: T13's list misses 6 persisted-id sites whose `format!` sits in a helper fed a `u64` — reasoning wires
  `wires-content` (the P8 orphan's own child!), cad `<pane>-model`, raster `raster-asset` (raw + canonical handle), remodeling
  `remodeling-asset` (+ a TypeScript SipHash-1-3 transliteration of `DefaultHasher` in the remodeling mutation twin), norm
  din18599 `din18599-climate`. Extended patch `wp-t14/f9/content-id.py` (`--root` for overlays): 29 sites, content id =
  `<prefix>-` + 16 hex of SHA-256 (`semio_framework_hash::Sha256`), first-party synchronous TS SHA-256 in `🔏️hash/🟦️.ts`
  (twin of Rust `Sha256`; Web Crypto is async), TS `contentId` beside `parseArtifactChild`, child schema `$defs.ContentId`,
  Rust law + TS law (`verify shared-artifact-addressing oracle`) against 7 Python-hashlib vectors + node:crypto for the TS
  SHA-256 (padding boundaries 55/56/64/…). Dry run on the live tree: **33 files, 0 problems**. Not in scope (not ids):
  block3d `world_fit_revision`, cad/puzzle edit-mode revisions, inference `content_digest` values, draw element ids.
- 19:1x H9-L: LC's prepared patch would have failed its own law — `LocalizedLabel` serializes EVERY terminology
  (`native`, `reuse`), the committed fixture hunk wrote only `native`, and the hub unit law asserts
  `serde_json::to_value(&kind) == fixture`; the manifest schema's `ArtifactKindFormatsFixture` still required `name` (hub TS
  validates the fixture against it). T14 copy `wp-t14/h9l/kind-label-patch.py` fixes both (+ `--root`); dry run 186 sites /
  94 files + 13/13 hunks. File→crate map (`wp-t14/crate-of.py`): every F9 and H9-L `.rs` file is in LC's 105-crate list.
- 19:1x overlay `.🧬semio/🌐hub/s14-t14-overlay` = APFS `clonefile(2)` of the working tree (`wp-t14/overlay.py`, 44 s, no
  data copied); private build-dir `<overlay>/.t14-build` seeded with copy-on-write clones of REGISTRY units only
  (`wp-t14/overlay-build-seed.py`: 584 native + 55 wasm32 units kept, every path-package unit left out so no overlay unit can
  be judged Fresh against the live tree).
- 19:1x RELAY G12 (via main): one primitive = F9's `store::content_id`; G12's `wp-g12/g12-authoring-seed.py` (SDK
  `AppOperationContext.authoring_seed` = `content_id("authoring-seed", actor ␟ HLC)`, 125 literals, 126 files) applies after
  F9 (it refuses without it); its crates add `semio-s-artifact-stdio-contract` + `semio-s-plugin-playbook-procedural` to my
  proof list. It goes into overlay hold 2 (hold 1 had already started with F9 + H9-L only).
- 19:26 overlay hold 1 started (`wp-t14/overlay-hold-1.sh`, log `s14-t14-logs/hold1-*.txt`): refresh overlay → F9 → H9-L →
  schema baseline → ONE native `--keep-going --lib --tests` check of the 105 crates. Load 104.
- 19:4x F9 carrier census (`wp-t14/f9/carrier-census.py`, `generated/f9-carrier-census-1.txt` + `f9-carriers-1.json`):
  **1 085 files** — fixtures 1 013, assets/examples 23, test sources 16, plugin `🔣️.json` 15 + `🛂️.descriptor.semio` 15
  (describe output → regenerated by the chain), production text-codec doc examples 2 (process3d, curation), 1 TS story, 1
  os-mcp fixture. By crate: architect 537, remodeling 281, note 81, block3d 80, process3d 42, din18599 11, sequence 5, raster
  3, curation 3, rest ≤ 2. **No owner has a fixture generator or `*_REGEN_*` switch** for these (searched every owner + the
  test platform) → regeneration plan = exact old→new id MAP, recorded by the production code itself: an overlay-only
  recorder in `content_id` computes the DefaultHasher candidates of the same bytes, the owners' lib tests run in the
  overlay, the map rewrites every carrier (same length: 16 hex → 16 hex, so binary packs keep their framing).
- 19:5x overlay refresh took 35 min (rmtree of the old clone: `🌎️hub/📦️packages/🦀️rust/🗑️generated` alone holds ~140 k
  gitignored files) → `overlay.py` now renames a replaced tree aside (`.t14-trash`) and has an incremental `sync` (files newer
  than the overlay stamp on either side, generated/dist/target/node_modules pruned) used from hold 2 on.
- 19:5x RELAY G12 builder pass + P8 orphan + wasm32 folded into overlay hold 2 (`wp-t14/overlay-hold-2.sh`, queued 19:50,
  9th in the overlay FIFO): sync → F9 + id recorder + G12 + orphan + H9-L → owners' lib tests recording the id map → map
  applied → owners' tests again → kernel content-id/addressing laws, host registry law, wfc3d kind law, hub linked-kind law,
  P8 verdict + reasoning/flow declared-verb laws, TS content-id oracle → ONE wasm32-wasip2 `--lib` check of the 103 guest
  crates.
- 20:0x Codex peer's value-derive set (`🌱️value/✨️derive`, `🔁️codec`, `✅️validator`) is uncommitted and was red at 19:1x
  (W4 `wp-w4/generated/wasm-check-layout-1.txt`: 6×E0004 empty-enum `match`); it also edits `🌱️value/🦀️.rs`, where 5b's
  macro goes (5b anchors on `//#region 🔖️SerDe`, untouched by the peer).
- 20:0x 5b phase A + B1 (`wp-t14/5b/dsl-value.py`, dry run `generated/5b-dry-1.txt`: 21 files, 19 literal sites, 51 B2
  sites listed). Decision: B2 (wrapper signatures `Option<serde_json::Value>` → `Option<DslValue>` + their 5–32 callers each,
  puzzle2d/5d internal `Value` plumbing, the renderer's programmatic `serde_json::Map` args, fixture-driven tests) cannot be
  compile-iterated before window 3 (overlay lane FIFO ≥ 1 h per turn, guest freeze on the live tree) → window 3 on the live
  tree after items 1–3, else routed with the exact list.
- 20:01 hold 1 overlay ready (F9 33 files / 0 problems, H9-L 186 sites + 13/13 hunks applied in the overlay);
  **schema-crate baseline green 20:09** (the peer's validator red is gone in this snapshot); 105-crate native check running.
- 20:0x window-3 landing tool `wp-t14/land-w3.py`: dry-runs every set on the live tree; `--write <set>` takes a
  copy-on-write snapshot of the source roots, applies, records the exact changed files (mtime > stamp) with before/after
  bytes under `.🧬semio/🌐hub/s14-t14-land/<set>/`; `--restore <set>` puts back only files still byte-equal to what it wrote.
