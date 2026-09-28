#!/bin/zsh
# 🧪️ S19 hub-genesis proof in ONE `overlay`-lane hold: generation3d + generation2d genesis laws (and the archive-door laws)
# green on the fixed overlay, then the generation3d genesis law red with the SDK producer UNFIXED (payload 0069 `.old`,
# kernel fixed), restored after. Each cargo capped at 28 min. usage: zsh s19-genesis-proof.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"; H=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%T')" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
run() { perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p \$1 --features component-app-assembly --lib --no-fail-fast -- \${=3} --nocapture; echo \"STEP \$2 rc=\$? \$(date +%T)\"; }
swap() { python3 -c \"
import json, shutil, sys
H, O, side = '$H', '$O', sys.argv[1]
for entry in json.load(open(H + '/payload/manifest.json', encoding='utf-8')):
    if entry['id'] == '0069':
        shutil.copyfile(H + '/payload/0069.' + side, O + '/' + entry['path'])
print('SDK plugin crate ->', side)
\" \$1; }
run semio-s-artifact-procedural-generation3d gen3d-fixed 'a_hub_genesis_pair_is_produced_and_parses_back_without_trapping a_document_archive_loads_into_a_fresh_instance_through_the_import_door'
run semio-s-artifact-procedural-generation2d gen2d-fixed 'a_hub_genesis_pair_is_produced_and_parses_back_without_trapping a_document_archive_loads_into_a_fresh_instance_through_the_import_door'
swap old
run semio-s-artifact-procedural-generation3d gen3d-sdk-unfixed a_hub_genesis_pair_is_produced_and_parses_back_without_trapping
swap new
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
