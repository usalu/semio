#!/bin/zsh
# usage: gocomptest-runall.sh <label>
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
G=$C/🗑️generated/gocomptest-work
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli"
L=$1
cd "$M" || exit 1
bun $C/gocomptest-planner.mjs "$M" $G/overlay.json >/dev/null || exit 1
go test -overlay=$G/overlay.json -c -o $G/$L.test . || exit 1
$G/$L.test -test.list '.*' | /usr/bin/grep -E '^(Test|Benchmark|Fuzz|Example)' | sort > $G/$L-list.txt
cat $G/$L-list.txt | xargs -P 6 -n 1 $C/gocomptest-one.sh $G/$L.test | sort > $G/$L-results.txt
echo done $(wc -l < $G/$L-results.txt)
