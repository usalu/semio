#!/bin/zsh
# carve-C1-run.sh LABEL OUTFILE -- COMMAND...
# Runs one heavy command, samples the process tree (rss via ps, phys_footprint via footprint for the rustc/wasm-ld pids) at 2 Hz,
# starts only when free+inactive+speculative >= 9 GB, kills its own tree when that figure falls below 3 GB.
# Prints one TSV row: label, wall_s, peak_rss_mb (tree sum), peak_footprint_mb (largest single rustc/linker process), min_avail_gb, killed, wasm bytes, sha256.
avail_gb() { vm_stat | awk '/page size/{ps=$8} /Pages (free|inactive|speculative)/{gsub("\\.","",$NF);s+=$NF} END{printf "%.2f", s*ps/1e9}'; }
label=$1; out=$2; shift 3
a=$(avail_gb); if (( ${a%.*} < 9 )); then echo "$label\tREFUSED start avail=${a}GB"; exit 3; fi
start=$(date +%s)
"$@" >/dev/null 2>"$out.stderr" &
root=$!
peak_rss=0; peak_fp=0; min_av=$a; killed=0
descend() { local p; for p in $(pgrep -P $1); do echo $p; descend $p; done; }
while kill -0 $root 2>/dev/null; do
  pids="$root $(descend $root)"
  rss=$(ps -o rss= -p ${=pids// /,} 2>/dev/null | awk '{s+=$1} END{print int(s/1024)}')
  (( rss > peak_rss )) && peak_rss=$rss
  for p in $root $(descend $root); do
    c=$(ps -o comm= -p $p 2>/dev/null)
    case "$c" in *rustc*|*wasm-ld*|*wasm-component-ld*)
      f=$(footprint -p $p 2>/dev/null | awk '/Footprint:/{for(i=1;i<=NF;i++)if($i=="Footprint:"){v=$(i+1);u=$(i+2)}; if(u=="GB")v*=1024; if(u=="KB")v/=1024; print int(v); exit}')
      [[ -n $f ]] && (( f > peak_fp )) && peak_fp=$f ;;
    esac
  done
  a=$(avail_gb); awk -v a=$a -v m=$min_av 'BEGIN{exit !(a<m)}' && min_av=$a
  if awk -v a=$a 'BEGIN{exit !(a<3)}'; then killed=1; for p in $(descend $root) $root; do kill -9 $p 2>/dev/null; done; break; fi
  sleep 1.5
done
wait $root 2>/dev/null; code=$?
end=$(date +%s)
wasm=${CARVE_WASM:-}; size=-; sha=-
[[ -n $wasm && -f $wasm ]] && { size=$(stat -f %z "$wasm"); sha=$(shasum -a 256 "$wasm" | cut -d' ' -f1); }
printf "%s\t%ss\trss=%sMB\tfootprint=%sMB\tminavail=%sGB\tkilled=%s\texit=%s\tbytes=%s\tsha256=%s\n" "$label" $((end-start)) $peak_rss $peak_fp $min_av $killed $code $size $sha
