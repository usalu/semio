#!/bin/zsh
# R8 session 12: every nx `typecheck` target, measured per project (direct `bun ./📜️script.ts typecheck` = the target's command).
root=/Users/ueli/Documents/semio
out="$root/.🧬semio/🌐hub/s12-r8-captures/tc"
run=${1:-1}
typeset -a rows
rows=(
  "framework|🧰️framework/📦️packages/🟦️typescript"
  "framework-os|🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript"
  "os-hub-ts|🌎️hub/📦️packages/🟦️typescript"
  "ui-react|🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript"
  "plugin-window-kits|🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits"
  "renderer-react|🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
  "repo-lib|🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript"
)
for row in $rows; do
  name=${row%%|*}; dir=${row#*|}
  cap="$out/$name-$run.txt"
  start=$(date +%s)
  (cd "$root/$dir" && nice -n 15 bun ./📜️script.ts typecheck) > "$cap" 2>&1
  rc=$?
  echo "$name rc=$rc secs=$(( $(date +%s) - start )) errors=$(/usr/bin/grep -c 'error TS' "$cap")" | tee -a "$out/summary-$run.txt"
done
echo DONE >> "$out/summary-$run.txt"
