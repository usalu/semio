#!/bin/zsh
# 🧪️ S19 set `gen-archive-load` overlay proof 2 (kernel hydration retire + app lease + edited-document laws) in ONE `overlay`-lane
# hold: generation3d + generation2d archive-door laws green on the fixed overlay, then generation2d red on the UNFIXED lease
# (binary + editor `.old`, kernel fixed) and restored. The private build-dir is seeded by APFS clonefile with REGISTRY units only
# from build-fleet-b (path crates' dep-info holds absolute live-tree paths → a cloned path unit would read false-Fresh).
# Each cargo capped at 28 min. usage: zsh s19-gen-archive-proof-2.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"; H=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19
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
LAW=a_document_archive_loads_into_a_fresh_instance_through_the_import_door
run() { perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p \$1 --features component-app-assembly --lib --no-fail-fast -- \$LAW --nocapture; echo \"STEP \$2 rc=\$? \$(date +%T)\"; }
swap() { python3 -c \"
import json, shutil, sys
H, O, side = '$H', '$O', sys.argv[1]
for entry in json.load(open(H + '/payload/manifest.json', encoding='utf-8')):
    if entry['id'] in ('0050', '0051'):
        shutil.copyfile(H + '/payload/' + entry['id'] + '.' + side, O + '/' + entry['path'])
print('generation2d binary + editor ->', side)
\" \$1; }
run semio-s-artifact-procedural-generation3d gen3d-fixed
run semio-s-artifact-procedural-generation2d gen2d-fixed
swap old
run semio-s-artifact-procedural-generation2d gen2d-lease-unfixed
swap new
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
