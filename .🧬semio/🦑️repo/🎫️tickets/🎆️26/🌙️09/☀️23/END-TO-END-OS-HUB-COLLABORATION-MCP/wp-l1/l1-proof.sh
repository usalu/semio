#!/bin/zsh
# 🧾️ L1 combined train proof, one step after another (each lane released before the next is queued — session-14 rule 28):
#   native `check --keep-going --lib --tests` (full framework + plugin union, build-fleet-b, private target)
#   → wasm32-wasip2 `check --lib` (guest union, default build-dir) → wasm32-unknown-unknown (renderer + os-kernel wasm actors)
#   → tsc os / ui-react / renderer-react / hub → one `serve s react dev` boot to Home (port 6700).
# Crate/feature lists = the T6R3a union (228 native, 59 features, 178 wasm, 2 uu), copied per round so a round can extend them.
# usage: zsh l1-proof.sh <ROUND> [steps…]   (steps default: native wasm uu tsc boot)   captures .🧬semio/🌐hub/s14-l1-logs/<ROUND>-*
setopt no_bg_nice
R=/Users/ueli/Documents/semio; W=$R/.tmp-ticket/wp-l1; L="$R/.🧬semio/🌐hub/s14-l1-logs"
round="$1"; shift; steps=(${@:-native wasm uu tsc boot})
for kind in native-crates native-features wasm-crates uu-crates; do
  [ -f "$L/$round-$kind.txt" ] || cp "$L/T6R3a-$kind.txt" "$L/$round-$kind.txt"
done
say() { echo "[l1-proof] $round $* $(date '+%F %T')" >> "$L/$round-proof.txt"; }
say "START steps: $steps"
for step in $steps; do
  case "$step" in
    native) zsh $W/l1-check.sh native "$L/$round-native-1.txt" "$L/$round-native-crates.txt" "$L/$round-native-features.txt"; say "native rc=$?" ;;
    wasm) zsh $W/l1-check.sh wasm "$L/$round-wasm-1.txt" "$L/$round-wasm-crates.txt" wasm32-wasip2; say "wasm rc=$?" ;;
    uu) zsh $W/l1-check.sh wasm "$L/$round-renderer-1.txt" "$L/$round-uu-crates.txt" wasm32-unknown-unknown; say "uu rc=$?" ;;
    tsc)
      for pair in "os:🧰️framework/🛍️products/💻️os" "ui-react:🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript" \
                  "renderer-react:🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" \
                  "hub:🌎️hub/📦️packages/🟦️typescript"; do
        name="${pair%%:*}"; dir="${pair#*:}"
        (cd "$R/$dir" && nice -n 5 bunx tsc --noEmit -p tsconfig.json > "$L/$round-tsc-$name-1.txt" 2>&1); rc=$?
        say "tsc $name rc=$rc errors=$(/usr/bin/grep -c 'error TS' "$L/$round-tsc-$name-1.txt")"
      done ;;
    boot) (cd $W && bun l1-boot-probe.ts --port 6700 > "$L/$round-boot-1.txt" 2>&1); say "boot rc=$? $(tail -1 "$L/$round-boot-1.txt" | cut -c1-200)" ;;
  esac
done
say "END"
