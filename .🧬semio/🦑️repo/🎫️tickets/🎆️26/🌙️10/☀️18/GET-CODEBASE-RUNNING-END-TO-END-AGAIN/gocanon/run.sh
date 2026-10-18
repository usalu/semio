#!/usr/bin/env bash
# usage: run.sh <moduleDir(relative to repo)> <slug> <before|after> [env GOWORK mode: ws|off]

REPO=/Users/ueli/Documents/semio
C="$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/gocanon-logs"
mod="$REPO/$1"; slug="$2"; phase="$3"; gw="${4:-ws}"
mkdir -p "$G"
cd "$mod" || exit 2
if [ "$gw" = off ]; then export GOWORK=off; else export GOWORK="$REPO/go.work"; fi
export GOFLAGS=-mod=mod
unset GOFLAGS
ov=()
pk=("./...")
if true; then
  python3 "$C/gocanon/plan.py" "$mod" "$G/$slug-overlay.json" > "$G/$slug-packages.txt" 2> "$G/$slug-plan.err" || { echo "plan failed"; cat "$G/$slug-plan.err"; exit 3; }
  ov=("-overlay=$G/$slug-overlay.json")
  pk=($(cat "$G/$slug-packages.txt"))
fi
norm() { sed -E 's/ \([0-9.]+s\)//; s/\t[0-9.]+s$//; s/\(cached\)//; s/[0-9]+\.[0-9]+s$//' ; }
{ go vet "${ov[@]}" "${pk[@]}" ; echo "vet-exit=$?"; } > "$G/$slug-$phase-vet.log" 2>&1
{ go vet "${ov[@]}" -tags exhaustive "${pk[@]}" ; echo "vet-exhaustive-exit=$?"; } > "$G/$slug-$phase-vet-exh.log" 2>&1
go test "${ov[@]}" -list '.*' "${pk[@]}" 2>&1 | /usr/bin/grep -v '^ok' | sort > "$G/$slug-$phase-list.txt"
go test "${ov[@]}" -tags exhaustive -list '.*' "${pk[@]}" 2>&1 | /usr/bin/grep -v '^ok' | sort > "$G/$slug-$phase-list-exh.txt"
go test "${ov[@]}" -count=1 -v "${pk[@]}" > "$G/$slug-$phase-test.log" 2>&1
echo "test-exit=$?" >> "$G/$slug-$phase-test.log"
/usr/bin/grep -E '^\s*--- (PASS|FAIL|SKIP)|^(ok|FAIL|---)\s|^\?|^test-exit' "$G/$slug-$phase-test.log" | norm | sed -E 's/\s+$//' | sort > "$G/$slug-$phase-results.txt"
echo "$slug $phase: vet=$(tail -1 "$G/$slug-$phase-vet.log") list=$(wc -l < "$G/$slug-$phase-list.txt") exh=$(wc -l < "$G/$slug-$phase-list-exh.txt") pass=$(/usr/bin/grep -c -- '--- PASS' "$G/$slug-$phase-results.txt") fail=$(/usr/bin/grep -c -- '--- FAIL' "$G/$slug-$phase-results.txt") $(tail -1 "$G/$slug-$phase-test.log")"
