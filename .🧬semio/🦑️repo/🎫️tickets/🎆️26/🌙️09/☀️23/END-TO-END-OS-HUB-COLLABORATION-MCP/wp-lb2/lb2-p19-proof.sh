#!/bin/zsh
# 💬️ LB2 p19 scratch proof (overlay lane, inside the scratch = live 19:45 + p17): write p19, compile every touched crate with
# its tests, the stdio contract lib tests, the touched verbs' unit tests, dump every stdio package's natively described
# descriptor (scratch-only probe test) and run the census oracle over the dumps, then the whole shipped fleet.
# `zsh lb2-p9-scratch.sh run <capture> zsh <this> <tag>`. Captures: s14-lb2-captures/<tag>-<step>.out
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
W="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2"
tag="$1"
dumps="$C/$tag-descriptors"
mkdir -p "$dumps"
step() {
  local name="$1"; shift
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)|^TOTAL|^stdio|^PROBLEM|files, " "$C/$tag-$name.out" | head -40
}
step write python3 "$W/lb2-p19-agent-verb-descriptions.py" --write --root "$PWD"
step probe python3 "$W/lb2-p19-dump-probe.py" "$PWD"
crates=(-p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-wav -p semio-s-artifact-stdio-semio -p semio-s-artifact-stdio-pdf -p semio-s-plugin-stdio -p semio-s-plugin-stdio-media -p semio-s-plugin-stdio-semio -p semio-s-plugin-stdio-pdf)
step check cargo check --offline --keep-going --tests $crates
step contract cargo test --offline --no-fail-fast -p semio-s-artifact-stdio-contract --lib
step units cargo test --offline --no-fail-fast -p semio-s-artifact-stdio-pdf -p semio-s-artifact-stdio-wav -p semio-s-artifact-stdio-semio --lib -- page edit_audio set_vertex
step dump env LB2_DESCRIPTOR_OUT="$dumps" cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet -- --ignored --exact lb2_probe_dump_descriptors
step census python3 "$W/lb2-p19-census.py" $dumps/*.json
step fleet cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet
step unprobe python3 "$W/lb2-p19-dump-probe.py" "$PWD" --remove
echo "PROOF END $(date '+%T')"
