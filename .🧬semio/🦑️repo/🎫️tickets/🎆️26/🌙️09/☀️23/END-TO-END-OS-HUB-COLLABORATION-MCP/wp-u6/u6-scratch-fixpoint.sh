#!/bin/zsh
# 🧾️ U6 set B, ONE native-lane hold on the scratch copy (APFS clones of 🧰️framework + stdio, a docx+xlsx-only workspace; no symlinks —
# the mutation derive refuses symlinked source paths; PRIVATE build-dir + target inside it — never build-fleet-b: the copy keeps the
# repo's relative paths and would collide with the shared units):
#   round k: `cargo check --lib` WITH `component-app-assembly` (every non-test use) + `cargo check --lib --profile test` WITHOUT it
#   (the lib-test unit) → `u6-dead-docx-xlsx.py count` → stop on an error, stop when no docx/xlsx dead_code/unused_imports is left,
#   else `analyze --round k` edits the scratch and appends round k to the payload;
#   proof: restore the pristine (p5-applied) docx/xlsx copies, apply the WHOLE payload with the set's own `--write`, check both units
#   again → `COUNT errors=0 dead=0` is the set's proof.
# usage: zsh u6-scratch-fixpoint.sh <scratch-root> <capture-prefix> [max-rounds] [first-round]   (first-round > 1 resumes on the scratch's current state)
setopt no_bg_nice
SC="${1:A}"; out="${2:A}"; max="${3:-5}"; first="${4:-1}"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-u6
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$SC/.u6-build" CARGO_TARGET_DIR="$SC/.u6-target"
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
cd "$SC" || exit 2
[ -d "$SC/.pristine" ] || { mkdir -p "$SC/.pristine" && cp -R "$SC/$A/📜️docx" "$SC/$A/📕️xlsx" "$SC/.pristine/"; }
echo "QUEUED fixpoint $(date '+%F %T')" > "$out.log"
zsh "$W/../📜️fleet-mutex.sh" native u6 -- nice -n 15 zsh -c '
  SC="$1"; out="$2"; max="$3"; W="$4"; A="$5"; first="$6"
  units() {
    s=$(date +%s)
    cargo check --offline --keep-going --message-format json-diagnostic-short --lib -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx --features semio-s-artifact-stdio-docx/component-app-assembly,semio-s-artifact-stdio-xlsx/component-app-assembly > "$1.lib.json" 2> "$1.lib.err"
    l=$?
    cargo check --offline --keep-going --message-format json-diagnostic-short --lib --profile test -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx > "$1.tests.json" 2> "$1.tests.err"
    echo "UNITS $1 lib rc=$l tests rc=$? wall=$(( $(date +%s) - s ))s $(date "+%T")"
    python3 "$W/u6-dead-docx-xlsx.py" count "$1"
  }
  round=$first; state=open
  while [ $round -le $max ]; do
    c=$(units "$out.r$round"); echo "$c"
    case "$c" in *"lib rc=0 tests rc=0"*"COUNT errors=0 dead=0 "*) state=converged; break ;; *"COUNT errors=0 dead=-"*) state=error; break ;; *"COUNT errors=0 "*) ;; *) state=error; break ;; esac
    python3 "$W/u6-dead-docx-xlsx.py" analyze "$out.r$round" --root "$SC" --round $round || { state=analyze-failed; break; }
    round=$(( round + 1 ))
  done
  echo "FIXPOINT state=$state rounds=$(( round - 1 )) $(date "+%T")"
  [ "$state" = converged ] || exit 3
  [ -n "$SC" ] && [ -d "$SC/.pristine/📜️docx" ] && [ -d "$SC/.pristine/📕️xlsx" ] || exit 6
  rm -rf "$SC/$A/📜️docx" "$SC/$A/📕️xlsx" && cp -R "$SC/.pristine/📜️docx" "$SC/.pristine/📕️xlsx" "$SC/$A/"
  python3 "$W/u6-dead-docx-xlsx.py" --dry-run --root "$SC" && python3 "$W/u6-dead-docx-xlsx.py" --write --root "$SC" --backup "$SC/.set-backup" || exit 4
  c=$(units "$out.proof"); echo "$c"
  case "$c" in *"lib rc=0 tests rc=0"*"COUNT errors=0 dead=0 "*) echo "PROOF green"; exit 0 ;; *) echo "PROOF red"; exit 5 ;; esac' u6 "$SC" "$out" "$max" "$W" "$A" "$first" >> "$out.log" 2>&1
rc=$?
du -sh "$SC/.u6-build" "$SC/.u6-target" >> "$out.log" 2>/dev/null
echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out.log"
exit $rc
