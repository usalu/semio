#!/bin/bash
# 📏️ Builds one cut artifact workspace (every lib, wasm32-wasip2) at one profile through the E3b cargo wrapper and records
# the peak RSS of each of its own rustc units (2 s ps sampler keyed by crate name) plus the per-unit wall (--timings).
# usage: 📏️measure-cuts.sh <tag> <artifact dir> <crate-name prefix> <profile>
TAG=$1; ART=$2; PREFIX=$3; PROFILE=$4
ROOT="/Users/ueli/Documents/semio"; E3B="$ROOT/.🧬semio/🦑️repo/⚡️cache/e3b"; OUT="$E3B/measure/$TAG"; mkdir -p "$OUT"
python3 - "$OUT" "$PREFIX" <<'PY' &
import subprocess, time, sys, os
out, prefix = sys.argv[1], sys.argv[2]; path = out + "/peaks.tsv"; peaks = {}
while True:
    for line in subprocess.run(["ps", "-Ao", "rss=,command="], capture_output=True, text=True).stdout.splitlines():
        p = line.split(None, 1)
        if len(p) < 2: continue
        t = p[1].split()
        if t and t[0].endswith("/rustc") and "--crate-name" in t:
            n = t[t.index("--crate-name") + 1]
            if n.startswith(prefix): peaks[n] = max(peaks.get(n, 0), int(p[0]))
    with open(path + ".tmp", "w") as f:
        for k, v in sorted(peaks.items(), key=lambda kv: -kv[1]): f.write(f"{v/1048576:.3f}G\t{k}\n")
    os.replace(path + ".tmp", path)
    time.sleep(2)
PY
SAMPLER=$!
"$(dirname "$0")/../e3b-pdf-cycle/🏗️cargo.sh" "measure-$TAG" build --manifest-path "$ART/Cargo.toml" --workspace --lib --target wasm32-wasip2 --profile "$PROFILE" --timings
rc=$?
kill $SAMPLER
cp "$E3B/target/cargo-timings/cargo-timing.html" "$OUT/timing.html" 2>/dev/null
cat "$OUT/peaks.tsv"
exit $rc
