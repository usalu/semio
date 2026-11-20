#!/bin/sh
set -u

ticket=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd) || exit 1
root=$ticket
while [ ! -f "$root/Cargo.toml" ] || [ ! -f "$root/.cargo/config.toml" ]; do
  parent=$(dirname -- "$root")
  if [ "$parent" = "$root" ]; then
    printf '%s\n' 'Could not locate the repository root.' >&2
    exit 1
  fi
  root=$parent
done

generated="$ticket/🗑️generated"
mkdir -p "$generated" || exit 1
cd "$root" || exit 1
report="$generated/native-rerun.txt"
: > "$report" || exit 1
result=0

run_crate() {
  crate=$1
  manifest="$2/Cargo.toml"
  log="$generated/native-$crate.txt"
  printf '[DEBUG] Running %s\n' "$crate"
  if repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path "$manifest" > "$log" 2>&1; then
    status=0
  else
    status=$?
    result=1
  fi
  printf '%s (exit %s)\n' "$crate" "$status" >> "$report"
  if ! grep -aE '^test result:|panicked at|^error|low-space|No space left|^Internal error|^FAILED' "$log" >> "$report"; then
    tail -n 12 "$log" >> "$report"
  fi
  if [ "$status" -ge 128 ]; then
    exit "$status"
  fi
}

ui="🧰️framework/🔨️modules/🖱️ui"
wgpu="$ui/🎯️targets/🧊️wgpu"
tui="$ui/⌨️tui/🧪️tests"
os="🧰️framework/🛍️products/💻️os"
rust="📦️packages/🦀️rust"

run_crate semio-ui-border-presentation "$ui/🔨️modules/📏️border-presentation/$rust"
run_crate semio-ui-draw-contract "$wgpu/🖍️draw/🏷️types/$rust"
run_crate semio-ui-wgpu-paint-laws "$wgpu/🖌️paint/🧪️tests/🥣️companion/$rust"
run_crate semio-ui-wgpu-events-laws "$wgpu/⚡️events/🧪️tests/🥣️companion/$rust"
run_crate semio-ui-tui-unit-laws "$tui/🔬️unit/$rust"
run_crate semio-ui-tui-laws "$tui/🥣️companion/$rust"
run_crate semio-renderer-wgpu-dock-laws "$os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️laws/$rust"

cat "$report"
exit "$result"
