#!/bin/zsh
# 🧪️ S19 set `gen-archive-load` overlay proof in ONE `overlay`-lane hold: the archive-door law green on the fixed overlay
# (generation3d, generation2d), then red on the UNFIXED binary + editor (payload `.old` restored for the run, the fixed
# `.new` put back after), each cargo capped at 28 min. usage: zsh s19-gen-archive-proof.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"; H=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%T')" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
LAW=a_document_archive_loads_into_a_fresh_instance_through_the_import_door
run() { perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p \$1 --features component-app-assembly --lib --no-fail-fast -- \$LAW --nocapture; echo \"STEP \$2 rc=\$? \$(date +%T)\"; }
run semio-s-artifact-procedural-generation3d gen3d-fixed
run semio-s-artifact-procedural-generation2d gen2d-fixed
python3 - <<'PY'
import json, shutil
H = '$H'; O = '$O'
for entry in json.load(open(H + '/payload/manifest.json', encoding='utf-8')):
    if entry['id'] in ('0047', '0048'):
        shutil.copyfile(H + '/payload/' + entry['id'] + '.old', O + '/' + entry['path'])
print('unfixed binary + editor restored for the red run')
PY
run semio-s-artifact-procedural-generation3d gen3d-unfixed
python3 - <<'PY'
import json, shutil
H = '$H'; O = '$O'
for entry in json.load(open(H + '/payload/manifest.json', encoding='utf-8')):
    if entry['id'] in ('0047', '0048'):
        shutil.copyfile(H + '/payload/' + entry['id'] + '.new', O + '/' + entry['path'])
print('fixed binary + editor put back')
PY
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
