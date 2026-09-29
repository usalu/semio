#!/bin/zsh
# 🗂️ WG11 s15: overlay proof of wg11-local-catalog-patch.py (guest freeze: never the live tree) — one overlay-lane hold: re-sync the
# overlay (APFS clones), apply the set, `cargo check` the renderer lib + tests, build the lib test binary and run the local-catalog +
# replay laws per process-free filter (RUST_MIN_STACK 128 MiB, the canonical runner's stack), then tsc (ShellHost, lane, engine-contract)
# and the engine-contract vitest describes. PRIVATE build-dir + target inside the overlay. Lines prefixed [wg11-lc].
# usage: setopt no_bg_nice; nohup zsh lc-overlay.sh > <capture> 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-wg11
O="$R/.🧬semio/🌐hub/s14-wg11-overlay"
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg11-lc] queued $(date '+%F %T') pid=$$ load=$(sysctl -n vm.loadavg)"
zsh $M[1] overlay wg11 -- zsh -c '
set -u
R=/Users/ueli/Documents/semio; W=$R/.tmp-ticket/wp-wg11; O="$R/.🧬semio/🌐hub/s14-wg11-overlay"
echo "[wg11-lc] HELD $(date "+%F %T") load=$(sysctl -n vm.loadavg)"
python3 $W/wg11-overlay-sync.py || exit 1
rm -rf "$O/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗂️local-catalog/🔣️.json" "$O/.🧬semio/🌐hub/s14-wg11-backup/local-catalog"
python3 $W/wg11-overlay-apply.py "$O" $W/wg11-local-catalog-patch.py --write || { echo "[wg11-lc] apply FAILED"; exit 1; }
cd "$O" || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$O/.cargo-build" CARGO_TARGET_DIR="$O/target" NX_DAEMON=false CARGO_NET_OFFLINE=true
echo "[wg11-lc] check $(date "+%F %T")"
nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|warning: .*(local_catalog|LocalCatalog|replay)|🗂️|Finished|could not" | head -60
echo "[wg11-lc] check rc=${pipestatus[1]} $(date "+%F %T")"
bin=$(nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format json 2>/dev/null | python3 -c "import json,sys
for line in sys.stdin:
    try: m=json.loads(line)
    except Exception: continue
    if m.get(\"reason\")==\"compiler-artifact\" and m.get(\"executable\") and m[\"target\"][\"name\"]==\"semio_framework_os_renderer_wgpu\": print(m[\"executable\"])" | tail -1)
echo "[wg11-lc] test binary: $bin $(date "+%F %T")"
if [ -n "$bin" ]; then
  export RUST_MIN_STACK=134217728
  for t in local_catalog the_shared_replay_refusal_vocabulary an_unserved_guest_replay a_space_artifact_creation_replay a_kept_studio; do
    echo "[wg11-lc] law $t"; nice -n 15 "$bin" "$t" --test-threads 1 2>&1 | tail -12
  done
fi
echo "[wg11-lc] tsc $(date "+%F %T")"
nice -n 15 bunx tsc -p $W/local-catalog/tsconfig-lc-overlay.json 2>&1 | tail -30; echo "[wg11-lc] tsc rc=${pipestatus[1]}"
cd "$O/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" || exit 1
SEMIO_TEST_LEVEL=standard nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --reporter=verbose "🔬️engine-contract" -t "local catalog vocabulary|replay refusal vocabulary" 2>&1 | tail -25
echo "[wg11-lc] vitest rc=${pipestatus[1]} $(date "+%F %T")"
echo "[wg11-lc] RELEASE $(date "+%F %T")"
'
echo "[wg11-lc] END rc=$? $(date '+%F %T')"
