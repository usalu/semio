#!/bin/zsh
# 🧪️ S19 T6 row 3b proof in ONE `overlay`-lane hold on `s14-s19-overlay-t6` (live tree + LB2 p9 + p15 + `s19-hosted-surfaces.py`):
# hub trusted-catalog laws, demonstrator assembly law, framework typegen, describe crate, stdio shipped-fleet, SDK check --tests.
# Private build-dir seeded with REGISTRY units only (path units' dep-info holds absolute live-tree paths). Each cargo is capped
# at 28 min and re-run while the cap fires (finished units stay cached), at most 4 times.
# usage: zsh s19-t6-proof.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay-t6"
B="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b/debug/build"
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%T')" > "$capture"
if [ ! -d "$O/.s19-build/debug/build" ]; then
  mkdir -p "$O/.s19-build/debug/build"
  seeded=0
  for d in "$B"/*(/N); do
    [[ "${d:t}" == semio* ]] && continue
    cp -c -R "$d" "$O/.s19-build/debug/build/" && seeded=$((seeded + 1))
  done
  echo "SEED $seeded registry packages cloned $(date '+%T')" >> "$capture"
fi
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
step() {
  local label=\$1; shift
  local attempt=0 rc=142
  while [ \$rc -eq 142 ] && [ \$attempt -lt 4 ]; do
    attempt=\$((attempt + 1))
    perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo \"\$@\"
    rc=\$?
    echo \"STEP \$label attempt=\$attempt rc=\$rc \$(date +%T)\"
  done
}
step hub-trusted-catalog test -p semio-hub --lib --no-fail-fast -- trusted_catalog
step demonstrator test -p semio-s-plugin-demonstrator --lib --no-fail-fast
step framework test -p semio-framework --lib --no-fail-fast
step describe test -p semio-framework-plugin-describe --lib --no-fail-fast
step sdk-check check -p semio-framework-plugin --lib --tests --message-format short
step stdio-shipped-fleet test -p semio-s-plugin-stdio --test shipped_fleet --no-fail-fast
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
