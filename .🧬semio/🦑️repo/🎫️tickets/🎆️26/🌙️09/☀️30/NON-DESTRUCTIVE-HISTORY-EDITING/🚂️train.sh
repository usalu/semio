#!/bin/zsh
# 🚂️ Landing train check (rule 67): while builds are slow a `landing` / `puzzle` hold is APPLY ONLY, and this loop is the one shared
# verification. Usage: `🚂️train.sh framework|puzzle` — one lane per process. Whenever `🗑️generated/coord/train.txt` gained a line (one per
# landed wave: `<time> <WP> <wave> <files> <restore command>`) the lane runs ONE `cargo check --lib`: FRAMEWORK = kernel, plugin, wgpu
# renderer, ui; PUZZLE = puzzle 2d in the `✏️s` workspace. Each lane owns the lines of `🗑️generated/coord/train.status` that start with
# its name: `<LANE> GREEN <time> through: <last wave at the pass start>` or `<LANE> RED <time> through: …` followed by the first twelve
# `<LANE>   file:line: error` lines, and `<LANE> CHECKING since <time> through: …` while a pass runs. A cargo that ends without a
# compile error (killed) is no verdict and the pass repeats. Full output of the last pass: `🗑️generated/coord/train-check-<lane>.txt`;
# history: `🗑️generated/coord/train.events`. Each lane builds in its own target + build dir (`🗑️generated/coord/train-target[-puzzle]`),
# so it takes no lock in the shared build dir, cannot join a flock cycle there and needs no gate.
setopt no_bg_nice
root="/Users/ueli/Documents/semio"
ticket="${0:A:h}"
coord="$ticket/🗑️generated/coord"
lane="${1:-framework}"
log="$coord/train.txt"; state="$coord/train.status"; out="$coord/train-check-$lane.txt"; events="$coord/train.events"
touch "$log" "$state"
cd "$root" || exit 2
export CARGO_BUILD_JOBS=4
if [ "$lane" = puzzle ]; then
  name="PUZZLE"; dir="$coord/train-target-puzzle"
  pass() { cargo check --manifest-path "✏️s/Cargo.toml" -p semio-s-artifact-puzzle-2d --lib --message-format=short > "$out" 2>&1 }
else
  name="FRAMEWORK"; dir="$coord/train-target"
  pass() { cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-os-renderer-wgpu -p semio-framework-ui --lib --message-format=short > "$out" 2>&1 }
fi
export CARGO_TARGET_DIR="$dir" CARGO_BUILD_BUILD_DIR="$dir"
# ✍️ publish <keep pattern> <new lines…>: replaces this lane's lines (all, or only its CHECKING line) atomically under a directory lock.
publish() {
  local keep="$1"; shift
  until mkdir "$state.lock" 2>/dev/null; do sleep 0.2; done
  { /usr/bin/grep -v -E "$keep" "$state"; print -r -l -- "$@"; } > "$state.tmp" && mv "$state.tmp" "$state"
  rmdir "$state.lock"
}
seen=-1
while true; do
  lines=$(wc -l < "$log" | tr -d ' ')
  if [ "$lines" -ne "$seen" ]; then
    last="$(tail -1 "$log" | cut -c1-90)"; [ -z "$last" ] && last="baseline"
    publish "^$name CHECKING" "$name CHECKING since $(date '+%T') through: $last"
    pass
    code=$?
    errors=("${(@f)$(/usr/bin/grep -E ': error(\[|:)' "$out" | cut -c1-230 | head -12 | sed "s|^|$name   |")}")
    if [ $code -eq 0 ]; then
      publish "^$name " "$name GREEN $(date '+%T') through: $last"
      seen=$lines
    elif [ -n "${errors[1]}" ]; then
      publish "^$name " "$name RED $(date '+%T') through: $last" "${errors[@]}"
      seen=$lines
    else
      publish "^$name CHECKING" "$name CHECKING since $(date '+%T') (previous cargo exit $code without a compile error — repeating)"
    fi
    print -r -- "$(date '+%F %T') $(/usr/bin/grep -E "^$name (GREEN|RED)" "$state" | head -1 | cut -c1-200)" >> "$events"
  fi
  sleep 15
done
