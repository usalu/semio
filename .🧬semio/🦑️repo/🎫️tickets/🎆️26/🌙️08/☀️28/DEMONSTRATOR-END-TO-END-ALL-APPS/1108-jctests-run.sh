#!/bin/bash
# usage: 1108-jctests-run.sh <label> <cargo test args...>   (foreground wait <=500 s)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; L=$1; shift
rm -f "$G/$L.rc"; nohup "$C/1105b-tools/runtime/tst.sh" "$L" 🧰️framework/Cargo.toml native -p semio-framework-job-context --features testkit "$@" >/dev/null 2>&1 &
for i in $(seq 1 50); do [ -f "$G/$L.rc" ] && break; sleep 10; done
echo "rc=$(cat $G/$L.rc 2>/dev/null)"
awk '/^error/{e=1;m=$0;next} e&&/^ *--> /{print $2" "m;e=0}' "$G/$L.txt" | sed 's/^[^ ]*\/\.\.\/\.\.\///' | cut -c1-220 | head -${LINES_MAX:-60}
grep -E "^error: could not compile|test result|^test .*(FAILED|failed)|panicked" "$G/$L.txt" | head -40
