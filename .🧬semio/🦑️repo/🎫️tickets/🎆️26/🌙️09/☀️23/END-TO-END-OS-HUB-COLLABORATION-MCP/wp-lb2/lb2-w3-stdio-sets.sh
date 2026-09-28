#!/bin/zsh
# 🧾️ LB2 window-3 landing (FIRST window-3 item, coordinator 18:4x): the stdio sets, ONE native-lane hold, in dependency
# order, each set reverted on its own red (p5 has its own script: lb2-p5-land.sh; p3 lands with WG11: lb2-p3-wg11-joint-land.sh):
#   xml  demo assets regenerated through the artifact's own writer (`zzz_write_dsl_and_pack_fixtures`) → xml lib (+editor)
#   p6   `.artifact(…)` declared catalogs published → check SDK + stdio `--lib --tests` → shipped_fleet → json editor lib
#   p1   details panel + SDK windowing + reactor follow-up honour the arena → check SDK + contract → law + details unit
#   p7   structural table honours the arena → check contract + csv + tsv `--lib --tests` → table_arena_headroom → csv/tsv lib
# Capture: wp-lb2/generated/<name>.txt. p8 (host TS) is NOT here: tsc + Interpreter vitest + rule-20 boot, see 📓️wp-lb2.md.
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb2/generated/$1.txt"
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/target"
{ echo "QUEUED $(date '+%H:%M:%S')"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c '
t() { nice -n 15 cargo "$@"; }
echo "START $(date "+%H:%M:%S") xml"
t test -p semio-s-artifact-stdio-xml --lib -- --ignored zzz_write_dsl_and_pack_fixtures; echo "XML-GENERATE rc=$? $(date "+%H:%M:%S")"
t test --no-fail-fast -p semio-s-artifact-stdio-xml --features semio-s-artifact-stdio-xml/component-app-assembly --lib; echo "XML-TEST rc=$? $(date "+%H:%M:%S")"
python3 .tmp-ticket/wp-lb2/lb2-p6-declared-catalogs.py --write || exit 3
t check --message-format short --keep-going -p semio-framework-plugin -p semio-s-plugin-stdio --lib --tests; rc=$?; echo "P6-CHECK rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 .tmp-ticket/wp-lb2/lb2-p6-declared-catalogs.py --revert; echo "P6 REVERTED"; exit $rc; fi
t test --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet; echo "P6-FLEET rc=$? $(date "+%H:%M:%S")"
t test --no-fail-fast -p semio-s-artifact-stdio-json --features semio-s-artifact-stdio-json/component-app-assembly --lib; echo "P6-JSON rc=$? $(date "+%H:%M:%S")"
python3 .tmp-ticket/wp-lb2/lb2-p1-arena-budget.py --write || exit 5
t check --message-format short --keep-going -p semio-framework-plugin -p semio-s-artifact-stdio-contract --lib --tests; rc=$?; echo "P1-CHECK rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 .tmp-ticket/wp-lb2/lb2-p1-arena-budget.py --revert; echo "P1 REVERTED"; exit $rc; fi
t test --no-fail-fast -p semio-s-artifact-stdio-contract --test details_arena_headroom; echo "P1-LAW rc=$? $(date "+%H:%M:%S")"
t test --no-fail-fast -p semio-s-artifact-stdio-contract --lib -- details; echo "P1-UNIT rc=$? $(date "+%H:%M:%S")"
python3 .tmp-ticket/wp-lb2/lb2-p7-table-arena.py --write || exit 4
t check --message-format short --keep-going -p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-csv -p semio-s-artifact-stdio-tsv --features semio-s-artifact-stdio-csv/component-app-assembly,semio-s-artifact-stdio-tsv/component-app-assembly --lib --tests; rc=$?; echo "P7-CHECK rc=$rc $(date "+%H:%M:%S")"
if [ $rc -ne 0 ]; then python3 .tmp-ticket/wp-lb2/lb2-p7-table-arena.py --revert; echo "P7 REVERTED"; exit $rc; fi
t test --no-fail-fast -p semio-s-artifact-stdio-contract --test table_arena_headroom; echo "P7-LAW rc=$? $(date "+%H:%M:%S")"
t test --no-fail-fast -p semio-s-artifact-stdio-csv -p semio-s-artifact-stdio-tsv --features semio-s-artifact-stdio-csv/component-app-assembly,semio-s-artifact-stdio-tsv/component-app-assembly --lib; echo "P7-TEST rc=$? $(date "+%H:%M:%S")"
'; } > "$capture" 2>&1
