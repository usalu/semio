# WP-LC — Landing Window: Plugin Handlers, Editors, Preferences, Creation Labels/Progress

Slice LC, session 13 (2026-09-26 19:0x →). Coordinator = main chat. Ports: hubs 8120–8129, serves 6620–6629 (none needed yet).
Private cargo target `.tmp-ticket/wp-lc/target`; captures `wp-lc/generated/` (expendable); durable data `.🧬semio/🌐hub/s13-lc-*`.
Sets landed here come from the handovers `📓️wp-p8.md`, `📓️wp-f1.md`, `📓️wp-u5.md`, `📓️wp-h9.md` (rows L, C).

Status legend: **measured** = ran here, capture named; **written, not run** = honest status; nothing is claimed that did not run.

## Session 13

| # | item | state | evidence |
|---|---|---|---|
| 1 | P8: flow leftovers + `p8-land.py --write --test` (agent-lane, law, flow, cad, space-studio, space-home) + decide `orphan` | in progress | — |
| 2 | F1: `typing-coalescing/apply.py` (jack + vcs coalesce, SDK `typing_run()`, ≥ 1000-char laws) | queued | — |
| 3 | U5: `u5-preference-lane-apply.py` (Rust twin + hub command/page/policy) | queued | — |
| 4 | H9 L: per-kind localized labels (`kind-label-patch.py`) | queued | — |
| 5 | H9 C: creation-phase progress (`codemods/creation-progress/`) | queued | — |

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
