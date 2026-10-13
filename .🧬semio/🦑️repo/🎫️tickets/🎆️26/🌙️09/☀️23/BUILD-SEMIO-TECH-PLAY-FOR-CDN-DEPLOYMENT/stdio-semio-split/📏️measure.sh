#!/bin/bash
# Measures the stdio semio crate cold at wasm-dev: default features, then --all-features, in a private target+build dir.
# Records cargo wall/maxrss (/usr/bin/time -l), per-unit durations (--timings) and per-crate rustc peak RSS (2 s ps sampler).
# usage: 📏️measure.sh <jobs>
cd /Users/ueli/Documents/semio || exit 2
JOBS="${1:-4}"
ROOT="$PWD/.🧬semio/🦑️repo/⚡️cache/split"; OUT="$ROOT/out"; mkdir -p "$OUT"
export CARGO_TARGET_DIR="$ROOT/target" CARGO_BUILD_BUILD_DIR="$ROOT/target/build" CARGO_INCREMENTAL=0 CARGO_TERM_COLOR=never
M="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml"
python3 - "$OUT" <<'PY' &
import subprocess, time, sys, os
out = sys.argv[1]; peaks = {}
while True:
    for line in subprocess.run(["ps", "-Ao", "rss=,command="], capture_output=True, text=True).stdout.splitlines():
        p = line.split(None, 1)
        if len(p) < 2 or "split/target" not in p[1]: continue
        t = p[1].split()
        if t[0].endswith("/rustc") and "--crate-name" in t:
            n = t[t.index("--crate-name") + 1] + ("+all" if "conversion-video" in p[1] else "")
            peaks[n] = max(peaks.get(n, 0), int(p[0]))
    with open(out + "/peaks.tsv.tmp", "w") as f:
        for k, v in sorted(peaks.items(), key=lambda kv: -kv[1]): f.write(f"{v/1048576:.2f}G\t{k}\n")
    os.replace(out + "/peaks.tsv.tmp", out + "/peaks.tsv")
    time.sleep(2)
PY
SAMPLER=$!
for mode in default all; do
  FLAGS=""; [ "$mode" = all ] && FLAGS="--all-features"
  s=$(date +%s)
  /usr/bin/time -l cargo build --locked --offline --manifest-path "$M" -p semio-s-artifact-stdio-semio --lib --target wasm32-wasip2 --profile wasm-dev --jobs "$JOBS" --timings $FLAGS > "$OUT/$mode.log" 2> "$OUT/$mode.time"
  echo "mode=$mode rc=$? wall=$(( $(date +%s) - s ))s maxrss=$(grep 'maximum resident' "$OUT/$mode.time" | awk '{print $1}')" >> "$OUT/result.txt"
  cp "$CARGO_TARGET_DIR/cargo-timings/cargo-timing.html" "$OUT/$mode-timing.html" 2>/dev/null
done
kill $SAMPLER
echo done >> "$OUT/result.txt"
