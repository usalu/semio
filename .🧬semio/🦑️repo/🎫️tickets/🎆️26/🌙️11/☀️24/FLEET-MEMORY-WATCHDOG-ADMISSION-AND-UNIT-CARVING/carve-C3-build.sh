#!/bin/zsh
# 🪓 carve-C3-build.sh LABEL LTO [EXTRA_CARGO_SUBCOMMAND_ARGS...]
# Builds the hub cad component final through the repo's cargo ingress (which owns the profile settings: release-wasm; LTO is a label here, the ingress refuses foreign --config),
# samples the final's rustc (resident and phys_footprint) every 2 s into <out>/<label>.final.tsv, the host's available memory beside it,
# waits for other cargo builds to end before it starts and kills its own tree when the host can give out less than 3 GB.
set -u
label=$1; lto=$2; shift 2
cd /Users/ueli/Documents/semio
out=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️24/FLEET-MEMORY-WATCHDOG-ADMISSION-AND-UNIT-CARVING/🗑️generated"
crate=semio_hub_cad_component
avail_mb() { vm_stat | awk '/page size/{ps=$8} /Pages (free|inactive|speculative)/{gsub("\\.","",$NF);s+=$NF} END{printf "%d", s*ps/1048576}'; }
while pgrep -f "cargo build --locked" >/dev/null 2>&1; do sleep 20; done
echo "$(date +%T) start $label lto=$lto avail=$(avail_mb)MB" >> "$out/$label.log"
start=$(date +%s)
mode=${CARVE_MODE:-build}
repo cargo --store=shared --configuration=release --parallelism=${CARVE_JOBS:-4} -- $mode --locked -Z trim-paths --offline \
  --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-cad-component --lib --target wasm32-wasip2 --profile release "$@" >> "$out/$label.log" 2>&1 &
root=$!
descend() { local p; for p in $(pgrep -P $1); do echo $p; descend $p; done; }
peak_rss=0; peak_fp=0; killed=0
while kill -0 $root 2>/dev/null; do
  a=$(avail_mb)
  pid=$(pgrep -f -- "--crate-name $crate " | head -1)
  if [[ -n "$pid" ]]; then
    rss=$(ps -o rss= -p $pid 2>/dev/null | awk '{print int($1/1024)}')
    fp=$(footprint -p $pid 2>/dev/null | awk '/Footprint:/{for(i=1;i<=NF;i++)if($i=="Footprint:"){v=$(i+1);u=$(i+2)}; if(u=="GB")v*=1024; if(u=="KB")v/=1024; print int(v); exit}')
    echo "$(( $(date +%s)-start ))\t${rss:-0}\t${fp:-0}\t$a" >> "$out/$label.final.tsv"
    (( ${rss:-0} > peak_rss )) && peak_rss=$rss; (( ${fp:-0} > peak_fp )) && peak_fp=$fp
  fi
  if (( a < 3000 )); then killed=1; for p in $(descend $root) $root; do kill -9 $p 2>/dev/null; done; break; fi
  sleep 2
done
wait $root 2>/dev/null; code=$?
wasm=$(find .cache/b/r/m -name "$crate.wasm" -newermt "@$start" 2>/dev/null | head -1)
size=-; sha=-; [[ -n "$wasm" ]] && { size=$(stat -f %z "$wasm"); sha=$(shasum -a 256 "$wasm" | cut -d' ' -f1); }
echo "$label\tlto=$lto\twall=$(( $(date +%s)-start ))s\tfinal_rss=${peak_rss}MB\tfinal_footprint=${peak_fp}MB\tkilled=$killed\texit=$code\twasm=$wasm\tbytes=$size\tsha256=$sha" | tee -a "$out/summary.tsv"
