#!/bin/zsh
# 🏠️ Window-3 joint landing of the TableRow contract (agreed LB2 ↔ WG11, sessions 14b/14c): LB2's SDK half
# `lb2-p3-row-actions.py --write` + WG11's wgpu painter `wp-wg11/wg11-table-row-painter-patch.py --apply` in ONE native-lane hold →
# `cargo check` SDK + ui(wgpu-engine) `--lib --tests` (both halves revert on red) → SDK table laws → WG11's ui laws → renderer-react
# typecheck + Interpreter vitest (p3 drops the React `row-action-` key filter). Rule 20's Home boot follows outside the lane.
# Capture: wp-lb2/generated/<name>.txt
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
backup=.tmp-ticket/wp-lb2/generated/p3-backup; rm -rf $backup; mkdir -p $backup
sdk="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"; tests="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-window-kits/🦀️.rs"
interp="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"
fixture="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity"
cp "$sdk" $backup/sdk.rs; cp "$tests" $backup/tests.rs; cp "$interp" $backup/interp.tsx
revert() { cp $backup/sdk.rs "$sdk"; cp $backup/tests.rs "$tests"; cp $backup/interp.tsx "$interp"; rm -rf "$fixture"; python3 .tmp-ticket/wp-wg11/wg11-table-row-painter-patch.py --revert; echo "REVERTED both halves"; }
echo "START $(date "+%H:%M:%S") apply"
python3 .tmp-ticket/wp-lb2/lb2-p3-row-actions.py --write || exit 3
python3 .tmp-ticket/wp-wg11/wg11-table-row-painter-patch.py --apply || { revert; exit 3; }
nice -n 15 cargo check --message-format short --keep-going -p semio-framework-plugin -p semio-framework-ui --features semio-framework-ui/wgpu-engine --lib --tests; rc=$?
echo "CHECK rc=$rc $(date "+%H:%M:%S")"
[ $rc -eq 0 ] || { revert; exit $rc; }
nice -n 15 cargo test --no-fail-fast -p semio-framework-plugin --lib -- home_shaped_rows table_kit table_window; echo "SDK-LAWS rc=$? $(date "+%H:%M:%S")"
nice -n 15 cargo test --no-fail-fast -p semio-framework-ui --features wgpu-engine --lib -- table_row_grid reconcile::tests flex::tests accessibility; echo "UI-LAWS rc=$? $(date "+%H:%M:%S")"
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && nice -n 15 bun ./📜️script.ts typecheck; echo "TYPECHECK rc=$? $(date "+%H:%M:%S")"; nice -n 15 bun ./📜️script.ts test long "🗣️Interpreter/🟦️.tsx"; echo "VITEST rc=$? $(date "+%H:%M:%S")" )
'; } > "$capture" 2>&1
