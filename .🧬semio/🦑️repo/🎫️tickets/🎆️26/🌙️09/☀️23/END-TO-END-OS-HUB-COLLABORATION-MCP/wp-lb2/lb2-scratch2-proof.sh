#!/bin/zsh
# 🏠️ LB2 p15 proof in scratch2 (scratch sources with p9–p14 + 🌎️hub/gis/vcs + p15): compile every crate p15 touches with its tests,
# the manifest TS projection law, the hub trusted-catalog laws and the stdio shipped-fleet laws. Run inside the overlay lane:
# `LB2_SCRATCH=<scratch2> zsh lb2-p9-scratch.sh run <capture> zsh <this> <tag> [step…]`. Captures: s14-lb2-captures/<tag>-<step>.out
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
tag="$1"; shift
only=("$@")
step() {
  local name="$1"; shift
  (( ${#only} == 0 || ${only[(Ie)$name]} )) || return 0
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)" "$C/$tag-$name.out" | head -40
}
crates=(semio-framework semio-framework-plugin semio-framework-plugin-host semio-framework-os-kernel semio-hub semio-s-plugin-stdio)
crate_args=(); for c in $crates; do crate_args+=(-p "$c"); done
step check cargo check --offline --tests $crate_args --features semio-framework/typegen
step typegen cargo test --offline -p semio-framework --features typegen --lib exports_typescript_bindings
step hub cargo test --offline --no-fail-fast -p semio-hub --lib trusted_catalog
step fleet cargo test --offline --no-fail-fast -p semio-s-plugin-stdio --test shipped_fleet
echo "PROOF END $(date '+%T')"
