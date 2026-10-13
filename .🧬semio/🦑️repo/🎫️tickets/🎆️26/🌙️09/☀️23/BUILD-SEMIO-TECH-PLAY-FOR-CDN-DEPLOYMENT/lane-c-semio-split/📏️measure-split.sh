#!/bin/bash
# 📏️ Builds the split semio workspace (every lib, wasm32-wasip2) at one profile in the private lane-c dirs and records the
# per-crate rustc peak RSS (2 s ps sampler over the private build dir) and per-unit wall times (--timings), resumably:
# rerun after a deadline exit (124) and the build continues while peaks keep accumulating.
# usage: 📏️measure-split.sh <profile> <seconds>
cd /Users/ueli/Documents/semio || exit 2
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
PROFILE="$1"; LIMIT="$2"; OUT="$T/🗑️generated/lane-c/measure-$PROFILE"; mkdir -p "$OUT"
python3 - "$OUT" <<'PY' &
import subprocess, time, sys, os
out = sys.argv[1]; path = out + "/peaks.tsv"; peaks = {}
if os.path.exists(path):
    for line in open(path):
        size, name = line.rstrip("\n").split("\t"); peaks[name] = int(float(size[:-1]) * 1048576)
while True:
    for line in subprocess.run(["ps", "-Ao", "rss=,command="], capture_output=True, text=True).stdout.splitlines():
        p = line.split(None, 1)
        if len(p) < 2 or "⚡️cache/s/" not in p[1]: continue
        t = p[1].split()
        if t[0].endswith("/rustc") and "--crate-name" in t:
            n = t[t.index("--crate-name") + 1]
            peaks[n] = max(peaks.get(n, 0), int(p[0]))
    with open(path + ".tmp", "w") as f:
        for k, v in sorted(peaks.items(), key=lambda kv: -kv[1]): f.write(f"{v/1048576:.3f}G\t{k}\n")
    os.replace(path + ".tmp", path)
    time.sleep(2)
PY
SAMPLER=$!
s=$(date +%s)
bash "$T/lane-c-semio-split/🦀️cargo.sh" "$LIMIT" build --locked --offline --manifest-path "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml" --workspace --lib --target wasm32-wasip2 --profile "$PROFILE" --timings > "$OUT/build.log" 2>&1
rc=$?
kill $SAMPLER
cp .🧬semio/🦑️repo/⚡️cache/s/target/cargo-timings/cargo-timing.html "$OUT/timing-$(date +%s).html" 2>/dev/null
echo "profile=$PROFILE rc=$rc wall=$(( $(date +%s) - s ))s" | tee -a "$OUT/result.txt"
head -8 "$OUT/peaks.tsv"
