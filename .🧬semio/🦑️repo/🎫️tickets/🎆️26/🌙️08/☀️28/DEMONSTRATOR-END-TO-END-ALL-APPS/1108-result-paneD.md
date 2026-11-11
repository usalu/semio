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

STATUS: BLOCKED upstream semio-plugin-instance-dispatch (gen3dfinal2: producers:57 E0283, typed-command:69 key_retirement, emit/original:50 E0283); verfolgen en/de and statik en/de not run
