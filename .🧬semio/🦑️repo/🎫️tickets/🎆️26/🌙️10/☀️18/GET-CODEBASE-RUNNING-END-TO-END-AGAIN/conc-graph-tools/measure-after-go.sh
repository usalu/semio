#!/bin/sh
# live acceptance of the graph generations, one repo command at a time (niced); prints wall seconds per step
# usage: measure-after-go.sh PROJECT_NAME SOURCE_FILE_OF_ANOTHER_PROJECT [--verify]
#   PROJECT_NAME  an existing project for the closure steps; SOURCE_FILE  a source file of a DIFFERENT project that the script appends a comment line to and removes again
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../../../../../.." && pwd)
cd "$ROOT" || exit 1
step() { label=$1; shift; start=$(date +%s); "$@" >/dev/null 2>"$HERE/measure.err"; code=$?; end=$(date +%s); printf '%-46s %4ss rc=%s  %s\n' "$label" $((end - start)) "$code" "$(grep -c . "$HERE/measure.err") stderr lines"; }
PROJECT=$1; FILE=$2
step "1 cold  show projects"                         nice -n 10 bun repo show projects
step "2 warm  show projects"                         nice -n 10 bun repo show projects
step "3 warm  show projects --graph-policy=current"  nice -n 10 bun repo show projects --graph-policy=current
step "4 warm  show project (closure only)"           nice -n 10 bun repo show project "$PROJECT"
printf '\n// conc-graph probe\n' >> "$FILE"
step "5 edit elsewhere, closure of $PROJECT"         nice -n 10 bun repo show project "$PROJECT"
step "6 edit elsewhere, whole graph (one delta)"     nice -n 10 bun repo show projects
step "7 same again (clean)"                          nice -n 10 bun repo show projects
python3 - "$FILE" <<'PY'
import sys
path = sys.argv[1]
text = open(path, encoding="utf8").read()
marker = "\n// conc-graph probe\n"
open(path, "w", encoding="utf8").write(text.replace(marker, "", 1) if text.endswith(marker) else text)
PY
step "8 probe removed (one delta)"                   nice -n 10 bun repo show projects
if [ "$3" = "--verify" ]; then SEMIO_BUILD_GRAPH_VERIFY=1 step "9 full inference oracle equals the generation" nice -n 10 bun repo show projects; fi
rm -f "$HERE/measure.err"
