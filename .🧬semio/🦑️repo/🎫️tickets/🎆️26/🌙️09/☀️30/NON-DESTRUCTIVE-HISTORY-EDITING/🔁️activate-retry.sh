#!/bin/zsh
# 🔁️ Activates puzzle 2d for React (6012) and wgpu (6112) once the framework root compiles and both plugin lockfiles resolve:
# every 10 min a gated precheck (`cargo metadata --locked` of every top-level workspace, `cargo check` of the kernel, schema and
# infinite-dag crates); when green, one `nx run-many` of both activation targets; stops at the first successful activation. The gate
# only refuses a swap storm (rustc ≥ 30), never ordinary fleet load. Events in `🗑️generated/e2e/activate-retry.events`.
setopt no_bg_nice
root="/Users/ueli/Documents/semio"
dir="${0:A:h}/🗑️generated/e2e"
mkdir -p "$dir"
for attempt in $(seq 1 48); do
  until [ "$(pgrep -x rustc | wc -l | tr -d ' ')" -lt 30 ]; do sleep 30; done
  pre="$dir/activate-s3-precheck-$attempt.log"
  if (cd "$root" && for w in Cargo.toml ✏️s/Cargo.toml 🌎️hub/Cargo.toml 🎓️teaching/Cargo.toml; do cargo metadata --locked --format-version 1 --manifest-path "$w" >/dev/null || exit 1; done && cargo check --message-format=short -p semio-framework-os-kernel -p semio-framework-schema -p semio-framework-artifact-infinite-dag --lib) > "$pre" 2>&1; then
    log="$dir/activate-s3-retry-$attempt.log"
    (cd "$root" && bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts nx run-many -t activate-puzzle2d-react-dev activate-puzzle2d-wgpu-dev -p @semio-tech/framework-os-dev --parallel=2) > "$log" 2>&1
    code=$?
    echo "$(date '+%F %T') s3 attempt=$attempt precheck=ok activation exit=$code log=${log:t}" >> "$dir/activate-retry.events"
    [ $code -eq 0 ] && exit 0
  else
    echo "$(date '+%F %T') s3 attempt=$attempt precheck=red log=${pre:t}" >> "$dir/activate-retry.events"
  fi
  sleep 600
done
exit 1
