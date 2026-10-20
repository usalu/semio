#!/bin/sh
# usage: probe.sh <tasks.txt> <results.tsv>   (AFTER GO; one real `bun repo --tool=bazel run` per task, foreground, one at a time)
# tasks.txt: one `project:target` per line. results.tsv: project<TAB>target<TAB>ok|fail:<reason>|plain<TAB>note.
# ok = the run passed and a RepoHermetic spawn ran; plain = passed but not hermetic (demoted or unselected); fail:<reason> = hermetic and failed (reason from the schema's Reason enum, chosen from the log).
ROOT=${SEMIO_ROOT:-/Users/ueli/Documents/semio}
cd "$ROOT" || exit 1
: > "$2"
while IFS= read -r task; do
  [ -z "$task" ] && continue
  log=$(mktemp "${TMPDIR:-/tmp}/hermetic-probe.XXXXXX")
  perl -e 'alarm 1800; exec @ARGV' bun repo --tool=bazel run "$task" --verbose > "$log" 2>&1
  code=$?
  project=${task%%:*}; target=${task#*:}
  if [ $code -eq 0 ]; then
    if grep -q "RepoHermetic" "$log"; then status=ok; else status=plain; fi
    note=""
  else
    status=fail:undeclared-assets
    if grep -q "Original build caller authority\|native" "$log"; then status=fail:native-authority; fi
    if grep -q "managed store\|\.cache/b/p/m\|lease" "$log"; then status=fail:managed-store; fi
    if grep -q "unbounded\|whole repository" "$log"; then status=fail:unbounded-inputs; fi
    note=$(grep -m1 "\[hermetic\]\|Cannot find\|ENOENT" "$log" | cut -c1-200)
  fi
  printf '%s\t%s\t%s\t%s\n' "$project" "$target" "$status" "$note" >> "$2"
  rm -f "$log"
done < "$1"
