#!/bin/sh
# usage: run1.sh <label> <outdir> <cmd...>  (cwd = repo root); writes <label>.{out,err,rc,span} with start/end epoch seconds
label=$1; out=$2; shift 2
start=$(date +%s); echo "$start $(date '+%H:%M:%S')" >"$out/$label.start"
"$@" >"$out/$label.out" 2>"$out/$label.err"
rc=$?
end=$(date +%s); echo "$rc $end $(date '+%H:%M:%S') $((end-start))s" >"$out/$label.rc"
