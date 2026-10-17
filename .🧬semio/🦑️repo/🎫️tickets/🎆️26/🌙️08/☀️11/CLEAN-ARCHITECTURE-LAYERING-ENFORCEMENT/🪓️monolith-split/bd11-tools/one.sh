#!/bin/zsh
# usage: one.sh <file> <capSeconds> ; prints "pass<TAB>fail<TAB>exit<TAB>file<TAB>firstError"
R=/Users/ueli/Documents/semio; cd $R
f="$2"; cap="$1"
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
if [ -z "$PLAIN" ]; then export SEMIO_TEST_ARTIFACT_DIR="$F/🗑️generated/bd11-artifacts" SEMIO_REPO_ROOT="$R"; fi
out=$( (ulimit -n 65536; nice -n 10 perl -e 'alarm $ARGV[0]; shift; exec @ARGV' -- "$cap" bun test "./$f" 2>&1); echo "EXIT:$?" )
p=$(echo "$out" | /usr/bin/grep -aE '^ *[0-9]+ pass' | awk '{print $1}' | tail -1)
fl=$(echo "$out" | /usr/bin/grep -aE '^ *[0-9]+ fail' | awk '{print $1}' | tail -1)
x=$(echo "$out" | /usr/bin/grep -ao 'EXIT:[0-9]*' | tail -1)
e=$(echo "$out" | /usr/bin/grep -a -m1 -E '^error|ENOENT|Cannot find|SEMIO_|Segmentation|panic' | cut -c1-160)
printf '%s\t%s\t%s\t%s\t%s\n' "${p:-?}" "${fl:-?}" "$x" "$f" "$e"
