#!/bin/zsh
# 📈️ W2: samples a running trusted-catalog publish every 20 s: packages completed, current stage, and the crates rustc is compiling under it
# (which units the bootstrap recompiles, measured). Exits when the publish capture reports the task end. usage: zsh w2-publish-sampler.sh <capture> <out>
CAP="$1"; OUT="$2"
while true; do
  clean=$(sed 's/\x1b\[[0-9;]*m//g' "$CAP" 2>/dev/null)
  done_n=$(print -r -- "$clean" | /usr/bin/grep -c 'bootstrap complete: 8/8')
  stage=$(print -r -- "$clean" | /usr/bin/grep -o 'trusted-stdio-gis-bootstrap [a-z-]*: [0-9]*/[0-9]*' | tail -1)
  crates=$(ps -axo command | /usr/bin/grep '[r]ustc --crate-name' | /usr/bin/grep 'wasm32-wasip2' | /usr/bin/grep -o 'crate-name [a-z0-9_]*' | cut -d' ' -f2 | sort | tr '\n' ' ')
  echo "$(date '+%T') complete=$done_n stage=[$stage] wasm-rustc=[$crates]" >> "$OUT"
  print -r -- "$clean" | /usr/bin/grep -q 'Successfully ran target\|Running target trusted-catalog-bootstrap for project os-hub failed' && break
  sleep 20
done
