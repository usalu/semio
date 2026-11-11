# 1108 result paneD (verfolgen/gis + statik panes, en+de)

State 00:25 (11/11): W-paneD-g1 (gis `--lib -- verfolgen`) queued at the capacity door behind other lanes; no edits made. statik next.

Rerun gis: `SLOT_HANG_SECS=100 $C/1105b-tools/runtime/tst.sh W-paneD-g<n> ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1`
Rerun statik: `SLOT_HANG_SECS=100 $C/1105b-tools/statik/retry.sh <n>` (fem3d pane crate semio-s-artifact-fem-3d-editor, test statik_pane_smoke).

## 00:55 (11/11)
- W-paneD-g1 (gis, native, rc 1): waited ~40 min at the capacity door, then compiled up to `semio-plugin-instance-dispatch` lib: 3 errors (staging/🎟️producers:57 E0283, staging/⌨️typed-command:69 missing `key_retirement`, staging/📤️emit/🎟️original:50 E0283). Claimed by gen3dfinal2 (edited 00:37, compiling W-gen3dfinal2-2). No edits by paneD.
- statik not started: fem3d pane crate depends on the same chain. Waiting for gen3dfinal2 FIXED, then rerun gis (g2) and statik (retry.sh).

## 01:05 (11/11)
- W-gen3dfinal2-2 (rc 1) still shows the same 3 instance-dispatch errors, so gis and statik cannot link. paneD made no source edits and holds no claim. Capacity queue wait was ~40 min before compile, so a rerun after FIXED needs a long foreground wait.
- Rerun order after gen3dfinal2 FIXED: (1) gis: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-paneD-g2 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1`; (2) statik: `SLOT_HANG_SECS=100 "$C/1105b-tools/statik/retry.sh" 300` (check the fem receive fix from matcher/fem2dsplit first: fem2dsplit open item was 2 receive tests failing heap receipt +225/+69 B, see 1108-result-fem2dsplit.md).
- Open: gis crates beyond instance-dispatch were not reached; gis static port is unverified (1107-result-gis.md). No [DEBUG] logs added.

Verdicts: verfolgen en NOT RUN, verfolgen de NOT RUN, statik en NOT RUN, statik de NOT RUN (the pane test loops en and de itself via PANE_SMOKE_LOCALES).


## 01:55 (11/11)
gen3dfinal2 FIXED-verified instance-dispatch (W-gen3dfinal2-3). W-paneD-g2 (gis) started 01:46 but sat in the capacity queue (7 ahead) when the turn ended; the child dies with the turn, so rerun it (command above, label g3) and then statik/retry.sh.

STATUS: BLOCKED capacity queue only; upstream instance-dispatch now fixed; verfolgen en/de and statik en/de not run

## 04:35 (11/11) update
- g3/g4 compile reds were upstream and peers' (job construction duplicate, stash/ff artifacts), then semio-framework-value-snapshot-projection owned/🦀️.rs (closeshared). Per coordinator, queued g5 and statik-400 (and the leftover statik-301 retry chain) were cancelled so closeshared's fix runs first. No edits by paneD.
- Rerun after `closeshared FIXED-verified`: gis `W-paneD-g6`, then `statik/retry.sh 500`.


## 06:30 (11/11) edits
closeshared fix landed; W-paneD-g6 (gis) and W-statik-500 now both red only in semio-plugin-instance-render (unclaimed after gen3dfinal released). Edits (claimed):
- instance-render staging `🪟️projections/🦀️.rs`: removed the duplicated `use semio_plugin_instance_extension::InstanceExtension;` (E0252).
- instance-jobs `👷️jobs/📥️staging/🏷️reserved/🦀️.rs`: added `ReservedPreparation::handoff(output, cx)` (moves job + completion + wire admission out in one paid step with a grant check; caller `reserved-jobs/🦀️.rs:191` and dispatch producers:90 already expect it).
Check: `W-paneD-c1` (`chk.sh`, native, instance-jobs + instance-render) queued. Then rerun gis `W-paneD-g7` and `statik/retry.sh 600`.


## 07:55 (11/11)
W-paneD-c1 still in the cargo capacity door (6 ahead, host load 20-30) after ~90 min; W-statik-501 retry chain queued behind it. Edits are on disk and claimed (see above). On rc0 of c1: gis `W-paneD-g7`, then `statik/retry.sh 600` (kill the stale statik-50x chain first by pid tree).

STATUS: UNVERIFIED W-paneD-c1 queued (instance-render projections import + ReservedPreparation::handoff); verfolgen en/de and statik en/de not run
