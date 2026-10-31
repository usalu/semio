#!/bin/bash
# usage: retry.sh N mode crate...  retries while the build toolchain is mid-edit (peer refactors)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
PAT='native-build/💾️storage|No space left on device|cannot be refreshed|Source refused before compiler|could not compile .semio-framework'
for i in $(seq 1 40); do
 "$C/1030-tools/fem2d/fem.sh" "$@"
 if grep -Eq "$PAT" "$C/🗑️generated/1030/fem2d-$N.txt"; then rm -f "$C/🗑️generated/1030/fem2d-$N.rc"; sleep 240; else break; fi
done
[ -f "$C/🗑️generated/1030/fem2d-$N.rc" ] || echo 99 > "$C/🗑️generated/1030/fem2d-$N.rc"
