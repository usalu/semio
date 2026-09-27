# WP-LC — Landing Window: Plugin Handlers, Editors, Preferences, Creation Labels/Progress

Slice LC, session 13 (2026-09-26 19:0x →). Coordinator = main chat. Ports: hubs 8120–8129, serves 6620–6629 (none needed yet).
Private cargo target `.tmp-ticket/wp-lc/target`; captures `wp-lc/generated/` (expendable); durable data `.🧬semio/🌐hub/s13-lc-*`.
Sets landed here come from the handovers `📓️wp-p8.md`, `📓️wp-f1.md`, `📓️wp-u5.md`, `📓️wp-h9.md` (rows L, C).

Status legend: **measured** = ran here, capture named; **written, not run** = honest status; nothing is claimed that did not run.

## Session 13

| # | item | state | evidence |
|---|---|---|---|
| 1 | P8: flow leftovers + six sets (agent-lane, law, flow, cad, space-studio, space-home); `orphan` decision | **all six in the tree, native green** (one combined `--keep-going --lib --tests` check, 12 crates, rc 0, 05:58); laws **PASS**: AJV twin 28 cases, verdict 2/2, cad 2/2, space-studio 1/1, flow lib 257/257, space-home editor P8 laws, reasoning + architect agent-lane laws; `orphan` HELD; wasm32 = W3 fast gate | `generated/combined-check-2.txt`, `s13-lc-p8-land/land-test-3.txt` |
| 2 | F1: typing coalescing (jack + vcs, SDK `typing_run()`, ≥ 1000-char laws) | **in the tree, native green**; laws RED from the test harness (writer, jack after 510 keys, vcs: displaced-owner retirement saturation — the fixture settle never drains pressure) → harness fix PREPARED `wp-lc/patches/f1-law-maintenance.py` | `combined-check-2.txt` |
| 3 | U5: preference lane Rust twin + hub command/page/policy | **in the tree, native green + laws green**: kernel twin 3/3, hub policy 2/2, TS Ajv policy oracle 2/2 (after the auth-schema enum fix) | `u5-kernel-check-1.txt`, `hub-check-2.txt` |
| 4 | H9 L: per-kind localized labels | **prepared for the next window** — dry run clean: 186 literal sites / 94 files + **11/11** exact hunks (5 follow-ups pre-derived from source: `OsArtifactDescriptor.name` → `label: LocalizedLabel` in both definitions (drops `Eq`: `LocalizedLabel` is not `Eq`), the unregistered-kind placeholder, the wfc3d kind test, the host registry law `projected.label == spec.label` + German cell); hub catalog kind lookup matches id AND schema; written, not compiled | `kind-label-patch.py` |
| 5 | H9 C: creation-phase progress | **in the tree, native green**; TS Ajv creation-progress oracle 1/1; H12 told `GuestCompiling` landed | `h9c-kernel-check-1.txt` |

### Log

- 19:06 read AGENTS.md, preambles 13/12, handovers P8/F1/U5/H9. Disk 112 GiB free, 2 rustc, load ~20. All six P8 sets
  dry-run clean on the current tree (`p8-land.py` preflight); `p8-orphan` needs `p8-law` first (its twin file) — expected.
- 19:1x P8 leftovers: the flow law's `[DEBUG]` `p8_scratch_flow_probe_dump` test removed from the tree (test-only); the
  `p8-flow.py` hunk re-anchored to the file without it (the same hunk replaces the `probes.len() == 0` placeholder with the
  28-verb law + pinned agent divergences + the one-scene law); `p8-flow.py --dry-run` clean again. `p8-land.py` adapted to
  session 13 (normal priority, wait above 14 rustc, binaries in `wp-lc/target`, backups `.🧬semio/🌐hub/s13-lc-p8-land/`).
- 19:14 `p8-land.py --write --test` launched detached (pid 13547, log `.🧬semio/🌐hub/s13-lc-p8-land/land-run-1.txt`).
- 19:1x dry runs of the other sets on the current tree: F1 `apply.py --dry-run` clean (5 files + fixture; the jack
  `text-edit` full-file write differs from the tree only by the intended coalesce key — checked by diff); U5 `--check`
  failed on 2 hub anchors (H9's Qd rewrote the event page: visibility is now a sync member-set predicate shared with
  `GET /directory/events`) → re-derived: `DirectoryEventLaneV1` + `directory_event_lane_v1(event)` (a preference event
  rides only the preference lane) checked in the page builder's loop; `GET /directory/events` and the socket keep
  delivering the caller's own space-less events as before (U5's design: the lane wakes on its own socket's events);
  original kept as `wp-lc/u5-preference-lane-apply.orig.py`; `--check` → all anchors in 12 files. H9 L: one new kind
  without German (`3D Terrain`, gis terrain) → `3D-Gelände` added; dry run 186 literal sites / 94 files, 6/6 hunks. H9 C:
  `patch -p1 --dry-run` + `hub-creation-progress.py --dry-run` clean.
- 20:1x rule 22 (memory) acknowledged: LC runs no server/hub/serve/browser (nothing listens on 8120–8129 / 6620–6629;
  F1's old serve 6620 is gone), one cargo at a time (the landing script serializes its gates), `cargo check` before test
  builds. Load ~88, swap 21/22.5 GB: agent-lane gates slow (SDK --lib --tests 651 s, flow 463 s, cad 786 s), all green.
- 19:1x–20:5x agent-lane gates (default build-dir, load 80–113, swap full): framework 93 s, SDK `--lib --tests` 651 s,
  flow 463 s, cad 786 s, sequence 2054 s — all exit 0 but recorded before 20:14 → **suspect under rule 24** (Z3's poisoned
  fingerprints); the architect `--lib --tests` gate was stopped from outside at 21:05 (exit −15 after 2190 s; the coordinator's
  21:1x stop of stuck default-build-dir cargos) → the script's restore put 3 of the 4 agent-lane files back, but the SDK
  `🔌️plugin/🦀️.rs` had meanwhile got a peer edit (`ensure_plugin_initialized` → `install_guest_panic_report`, not LC's) and
  was left as is → tree half-landed for ~1 min. 21:06 re-landed the 3 files byte-exactly from `land-backup/agent-lane/*.landed`
  (they were byte-equal to their `.orig`, so nobody else had touched them); SDK kept with both edits.
- 21:1x rule 27: `p8-land.py` now passes `CARGO_BUILD_BUILD_DIR=…/build-landing`; each set's gates are ONE cargo invocation
  (all its crates, `--lib --tests`, so one cargo parallelizes a cold build); new `--gates-only` (re-check a landed set).
  Chain launched detached (pid 13517, log `s13-lc-p8-land/land-run-2.txt`): `--gates-only --only agent-lane` →
  `--write --from law --test` → `--test --only agent-lane`.
- 04:5x resumed after the overnight cut (rule 28). Reconciled: agent-lane in the tree (3 files == landed, SDK changed by peers
  with P8's hunks intact), law/flow/cad/space-studio/space-home dry-run clean (not written). The 21:1x chain's re-gate had failed
  only on architect `🧭️trace/🦀️.rs:91` (a peer's 26/09 19:51 edit passing the crate-root `SurfaceKind` to `scene_surface`).
- 05:10 U5 applied (`--check` clean) → kernel green; 05:13 H9-C applied (kernel diff's `waiting` copy hunk dropped: a peer had
  landed it; H12's 2 CONFIG anchors re-derived) → kernel green. Hub: lib needed the `os_directory` re-exports + the
  `guest-compiling` code arm (compiler-named) → lib green; bin: 1 missing arm (fixed) + Send/`&u8` errors (triage).
- 05:0x–05:3x SDK red twice from peers (G11's carrier `EmitWire` mid-write; `AppFactory.codec` of another slice) — waited.
- 05:34 P8 law/flow/cad/space-studio/space-home written with backups (`p8-land.py --write --no-gates --from law`) and F1 applied
  (extended: vcs retained work + vcs law) → ONE combined native check; first run stopped at the architect break (cargo without
  `--keep-going` stops scheduling) → fixed that one line (`plugin_app_close_prelude::SurfaceKind`) → **rc 0 in 441 s** (05:58).
- 05:59 P8 laws launched (`p8-land.py --test --only law,cad,space-studio,space-home,flow`, pid 38688) — then F1 laws, U5/H9-C laws.
- Orphan set: HELD. Its rule is correct (clone-measured: reasoning wires orphan `content/wires-content-*` children), but landing
  it without the per-surface content-addressed-child fix turns the reasoning law red, and that fix touches 12 plugins — not
  possible before the 07:00 guest cut-off. Routed as P8 wrote it (coordinator + 12 owners).
- 09:5x resumed (rule 30, REBUILD START 09:53). P8 laws of the 05:59 run: AJV twin 28 cases, `declared_verb_verdict` 2/2, cad
  `import_cad_file` 2/2, space `retained_command_catalog` 1/1 — PASS; flow lib stopped −15 (idle overnight in build-landing, not
  measured); space-home's `retained` filter matched 0 tests (not measured). Remaining laws re-queued through the native lane
  (`zsh wp-lc/lc-laws.sh`, detached via `w2-detach.py`, pid 73869, logs `.🧬semio/🌐hub/s13-lc-laws/`): os-hub bin check, flow
  lib (all), space-home lib (all), writer/jack/vcs typing laws, reasoning + architect declared-verb laws, kernel twin laws
  (preference record, creation progress, event page), hub access-policy vectors.
- 09:5x H9-L prepared for the next window (see row 4); original kept as `wp-lc/kind-label-patch.orig.py`.
- 10:14 `cargo check -p semio-hub --bin os-hub --tests` (native lane, build-fleet-b) **rc 0** — the 05:3x Send/`&u8` errors were a peer's in-flight state. U5 + H9-C rows updated in `📓️landing.md`.
- 10:14–10:49 native-lane laws (`s13-lc-laws/`): os-hub bin + tests check **rc 0**; flow lib **257 passed / 0 failed** (incl.
  `every_declared_flow_verb_honours_its_declaration` + `the_content_child_is_the_one_scene…`); space-home lib 27/28 — the 1 red
  is `structural_correspondence…::direct_owner_descriptor_surfaces_and_catalog_correspond` (asserts outcomeClasses
  `[applied, warning]`, descriptor says `[applied, no-op]` since T12's manifest-align; not P8's); space-home editor
  (`--features component-app-assembly`, the editor module is feature-gated) 55/56 — P8's `every_migrated_home_route_has_an_exact_scalar_boundary`
  + both `rename_space` tests **ok**; the 1 red is `🎚️config` `retained_config_cancel_and_cleanup_respect_the_production_grant`
  (config one-item preparation close with a 1-byte grant no longer `Blocked`; config module untouched by P8 — peer area).
- **F1 writer law RED (measured, 10:41):** `a_typing_run_longer_than_the_edit_ledger_saves_and_undoes_as_one_step` →
  `artifact store displaced-owner fixed retirement authority is saturated` during the run. Cause: every amend displaces the
  edit's envelope/snapshot/dag; the live runtime drains a pressured queue in one-item bursts in the same turn
  (`plugin_runtime`, `maintenance_under_pressure`), the fixture settle never does. Fix PREPARED (guest-linked files → next
  window): `wp-lc/patches/f1-law-maintenance.py` (dry run clean, 5 hunks / 4 files): SDK `artifact_app_laws::drain_maintenance_pressure`
  (+ `RUNTIME_CLOSE_BYTES_PER_STEP` → `pub(crate)`), called after every keystroke in the writer, jack and vcs laws. Written,
  not compiled.
- jack's editor module is gated behind `component-app-assembly`: the 05:58 combined check did NOT type-check F1's jack
  hunks (`--lib --tests` without the feature); re-queued the jack law WITH the feature (it compiles the editor natively).
- 10:56–11:13 remaining laws: jack (with `component-app-assembly`, which also type-checked F1's jack hunks) red at keystroke
  511 — the same saturation (vcs too, `vcs.txt`); reasoning + architect declared-verb laws 1/1 + 1/1; kernel twin laws 3/3
  (`user_preference_record_v1_matches_the_shared_fixture`, 2 event-page laws); hub access-policy 2/2. TS: Ajv access-policy
  oracle RED (the hub auth schema's `HubAccessActionV1` enum lacked U5's two actions) → added `preference.record`,
  `preference.read` to `🌎️hub/🔐️auth/🧬️schema/🔣️.json` (hub-native JSON, read only by TS) → access-policy 2/2 +
  creation-progress oracle 1/1 (`generated/hub-ts-oracles-2.txt`).
