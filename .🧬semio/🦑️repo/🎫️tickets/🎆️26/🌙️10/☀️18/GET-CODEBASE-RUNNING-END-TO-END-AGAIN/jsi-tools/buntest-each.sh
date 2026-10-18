#!/bin/zsh
# usage: buntest-each.sh <label> <repo-relative test file>...  runs every file in its own bun test process with a script-process envelope; writes C/🗑️generated/jsi-each-<label>.txt ("status<TAB>file<TAB>name") and one log per file
REAL=/Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$REAL/$C/🗑️generated"; mkdir -p "$G/jsi-each-$1" "$G/jsi-artifact/$1"
label="$1"; shift
wire="$(cd $REAL && bun "$C/jsi-tools/envelope.ts")"
cd "${JSI_ROOT:-$REAL}"
: > "$G/jsi-each-$label.txt"
n=0
for file in "$@"; do
  n=$((n+1))
  base="$G/jsi-each-$label/$n"
  SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_TEST_ARTIFACT_DIR="$G/jsi-artifact/$label" NODE_PATH=/Users/ueli/.cache/jsi/nm/node_modules nice -n 10 perl -e 'alarm 300; exec @ARGV' -- bun --tsconfig-override=/Users/ueli/.cache/jsi/nm/tsconfig.override.json test --timeout 120000 --reporter=junit --reporter-outfile="$base.xml" "./$file" > "$base.log" 2>&1
  if [ -f "$base.xml" ]; then python3 - "$base.xml" "$file" >> "$G/jsi-each-$label.txt" <<'PY'
import sys, xml.etree.ElementTree as ET
t = ET.parse(sys.argv[1])
rows = []
for tc in t.iter("testcase"):
    st = "failed" if (tc.find("failure") is not None or tc.find("error") is not None) else ("skipped" if tc.find("skipped") is not None else "passed")
    print("\t".join((st, sys.argv[2].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/",""), tc.get("name") or "")))
PY
  else echo "failed\t$file\t(no junit output)" >> "$G/jsi-each-$label.txt"; fi
done
awk -F'\t' '{c[$1]++} END{for(k in c) printf "%s=%d ", k, c[k]; print ""}' "$G/jsi-each-$label.txt"
