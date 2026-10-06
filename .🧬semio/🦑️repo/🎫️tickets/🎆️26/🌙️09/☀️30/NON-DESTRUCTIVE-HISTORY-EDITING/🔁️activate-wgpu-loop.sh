#!/bin/zsh
# 🔁️ wgpu-lane-only activation retry (build B2 and later): the React lane of an activation can pass while the wgpu lane dies on a peer's
# save mid-build. Runs `activate-puzzle2d-wgpu-dev` up to `${1:-4}` times, each attempt behind the quiet gate (no `*.rs` / `Cargo.toml` of
# the framework saved for two minutes, at most ten minutes of waiting) and four minutes after a failed one. Log per attempt `🗑️generated/e2e/activate-wgpu-<n>.log`, final
# code in `🗑️generated/e2e/activate-wgpu-loop.exit`, one line per attempt in `🗑️generated/e2e/activate-retry.events`.
setopt no_bg_nice
dir="${0:A:h}"; root="/Users/ueli/Documents/semio"; out="$dir/🗑️generated/e2e"
nx=(bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx)
quiet() {
  [ -z "$(find "$root/🧰️framework" "$root/✏️s/🔌️plugins/🧩️puzzle" \( -name node_modules -o -name target -o -name '🤖️generated' -o -name '.git' \) -prune -o -type f \( -name '*.rs' -o -name 'Cargo.toml' \) -mmin -2 -print -quit 2>/dev/null)" ]
}
code=1
for n in $(seq 1 "${1:-4}"); do
  waited=0
  until quiet || [ "$waited" -ge 600 ]; do sleep 20; waited=$((waited + 20)); done
  (cd "$root" && $nx run @semio-tech/framework-os-dev:activate-puzzle2d-wgpu-dev) > "$out/activate-wgpu-$n.log" 2>&1
  code=$?
  echo "$(date '+%F %T') wgpu-lane activation attempt=$n exit=$code quiet-gate=${waited}s" >> "$out/activate-retry.events"
  [ "$code" -eq 0 ] && break
  sleep 240
done
echo "$code" > "$out/activate-wgpu-loop.exit"
exit $code
