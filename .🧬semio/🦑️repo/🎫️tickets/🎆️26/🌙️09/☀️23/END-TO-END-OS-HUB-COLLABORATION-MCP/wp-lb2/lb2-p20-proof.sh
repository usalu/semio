#!/bin/zsh
# 🌐️ LB2 p20 scratch proof (overlay lane, inside the scratch = live 19:45 + p17 + p19; + gis/vcs clones and the hub crate as a
# member so semio-hub builds): write p20, compile every touched crate with its tests, stdio native-codec receipts + projection,
# stdio registry lib, the shipped fleet (LAW (f) + LAW (g)), the framework coordinate / codec laws, hub native provider pins,
# a native descriptor dump and the TS publisher + ownership check over it. `zsh lb2-p9-scratch.sh run <capture> zsh <this> <tag>`.
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
W="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2"
R="/Users/ueli/Documents/semio"
tag="$1"
dumps="$C/$tag-descriptors"
mkdir -p "$dumps"
step() {
  local name="$1"; shift
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)|^PROBLEM|files, |^TS-CHECK|^editors |^FAIL" "$C/$tag-$name.out" | head -40
}
step prep zsh "$W/lb2-p20-scratch-prep.sh" "$PWD"
step write python3 "$W/lb2-p20-hub-openable.py" --write --root "$PWD"
step probe python3 "$W/lb2-p19-dump-probe.py" "$PWD"
roots=(bmp wav epw binary ifc gif semio pdf)
crates=(-p semio-framework-plugin -p semio-s-plugin-stdio -p semio-s-plugin-stdio-image -p semio-s-plugin-stdio-media -p semio-s-plugin-stdio-bim -p semio-s-plugin-stdio-pdf -p semio-s-plugin-stdio-semio -p semio-s-plugin-stdio-binary -p semio-hub)
for r in $roots; do crates+=(-p "semio-s-artifact-stdio-$r"); done
step check cargo check --offline --keep-going --tests $crates
step receipts cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test native_openable_provider
step registry cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --lib
step framework cargo test --offline --no-fail-fast -p semio-framework-plugin --lib -- a_dialect_coordinate codec_calls_construct_no_app a_shared_schema_is_created
step dump env LB2_DESCRIPTOR_OUT="$dumps" cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet -- --ignored --exact lb2_probe_dump_descriptors
step ts node "$W/lb2-p20-ts-check.mjs" "$PWD" "$dumps"
step fleet cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet
step hub cargo test --offline --no-fail-fast -p semio-hub --lib native_openable
step unprobe python3 "$W/lb2-p19-dump-probe.py" "$PWD" --remove
echo "PROOF END $(date '+%T')"
