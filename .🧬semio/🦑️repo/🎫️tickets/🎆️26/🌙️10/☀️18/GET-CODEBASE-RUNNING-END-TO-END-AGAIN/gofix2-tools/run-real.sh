#!/bin/zsh
# usage: run-real.sh <label> [extra test flags]  (real tree, canonical overlay)
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
W=$C/🗑️generated/gofix2
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli"
L=$1; shift
python3 $C/gocomp/overlay.py "$M" $W/plan-test.json --tests >/dev/null || exit 1
cd "$M" || exit 1
export GOWORK=/Users/ueli/Documents/semio/go.work
go vet -overlay=$W/plan-test.json . 2>&1 | head -40
go test -overlay=$W/plan-test.json -c -o $W/$L.test . || exit 1
$W/$L.test -test.count=1 -test.short -test.v -test.timeout 500s "$@" > $W/$L-run.txt 2>&1
/usr/bin/grep -E '^--- FAIL' $W/$L-run.txt | awk '{print $3}' | sort > $W/$L-fail.txt
echo "fails: $(wc -l < $W/$L-fail.txt)  pass: $(/usr/bin/grep -cE '^--- PASS' $W/$L-run.txt) skip: $(/usr/bin/grep -cE '^--- SKIP' $W/$L-run.txt)"
echo "new failures vs after1:"; comm -13 $W/after1-fail.txt $W/$L-fail.txt
echo "fixed vs after1:"; comm -23 $W/after1-fail.txt $W/$L-fail.txt
