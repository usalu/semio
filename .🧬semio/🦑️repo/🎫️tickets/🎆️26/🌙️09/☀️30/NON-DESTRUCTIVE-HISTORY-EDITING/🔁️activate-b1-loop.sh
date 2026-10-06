#!/bin/zsh
# 🔁️ Build B1 retry loop: runs `🔁️describe-activate-s4.sh <n>` for n = first..last and stops at the first exit 0 or at an activation-stage
# failure (exit 12, not a compile race). A describe-stage failure (exit 11: a peer saved into the closure mid-build) retries. Before every
# attempt a quiet gate waits (at most ten minutes) until no Rust source of the puzzle closure was saved for two minutes, so a build starts
# right after a peer's wave instead of inside it. One event line per attempt in `🗑️generated/e2e/activate-retry.events`, the gate's waits in
# `🗑️generated/e2e/activate-b1-loop.out`, the final code in `🗑️generated/e2e/activate-b1-loop.exit`.
setopt no_bg_nice
dir="${0:A:h}"
root="/Users/ueli/Documents/semio"
first="${1:-10}"; last="${2:-14}"
quiet() {
  [ -z "$(find "$root/🧰️framework" "$root/✏️s/🔌️plugins/🧩️puzzle" "$root/✏️s/🔌️plugins/🗄️stdio" "$root/🌎️hub" \
    \( -name node_modules -o -name target -o -name '🤖️generated' -o -name '.git' \) -prune -o -type f \( -name '*.rs' -o -name 'Cargo.toml' \) -mmin -2 -print -quit 2>/dev/null)" ]
}
code=1
for n in $(seq "$first" "$last"); do
  waited=0
  until quiet || [ "$waited" -ge 600 ]; do sleep 20; waited=$((waited + 20)); done
  echo "$(date '+%F %T') attempt $n starts after a quiet gate of ${waited}s"
  /bin/zsh "$dir/🔁️describe-activate-s4.sh" "$n"
  code=$?
  [ "$code" -eq 0 ] && break
  [ "$code" -eq 12 ] && break
done
echo "$code $n" > "$dir/🗑️generated/e2e/activate-b1-loop.exit"
exit $code
