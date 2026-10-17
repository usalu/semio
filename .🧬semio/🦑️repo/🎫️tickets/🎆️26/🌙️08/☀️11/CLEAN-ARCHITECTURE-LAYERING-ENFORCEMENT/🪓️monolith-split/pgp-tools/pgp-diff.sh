#!/bin/zsh
# usage: pgp-diff.sh <name> <python args...>  -- clones base into py/ts sandboxes, runs both tools, diffs stdout and trees
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pgp
F=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split
name=$1; shift
BASE=${PGP_BASE:-$SC/base}
rm -rf $SC/$name; mkdir -p $SC/$name
cp -Rc $BASE $SC/$name/py; cp -Rc $BASE $SC/$name/ts
D=$SC/$name/py/1/2/3/4/5/6/7; mkdir -p $D; cp $F/pgp-tools/pgp-oracle-wp4c-project-exports.py $D/oracle.py
( cd $SC/$name/py && python3 $D/oracle.py "$@" > $SC/$name/py.out 2>$SC/$name/py.err ); echo "python exit $?"
( cd /Users/ueli/Documents/semio && bun $F/pgp-tools/pgp-run-ts.ts $SC/$name/ts "$@" > $SC/$name/ts.out 2>$SC/$name/ts.err ); echo "ts exit $?"
rm -rf $SC/$name/py/1
echo "--- stdout diff"
if diff -q $SC/$name/py.out $SC/$name/ts.out >/dev/null; then echo "stdout identical ($(wc -l < $SC/$name/ts.out) lines)"
elif diff <(grep -v '^RUST-ABSENT' $SC/$name/py.out | sort) <(grep -v '^RUST-ABSENT' $SC/$name/ts.out | sort) >/dev/null; then echo "stdout identical as a sorted multiset without the RUST-ABSENT rows (summary line, REFUSED and TOUCHED rows equal; the capped RUST-ABSENT excerpt lists documents in directory order in Python and in code-point order in TypeScript)"
else echo "STDOUT DIFFERS"; diff <(grep -v '^RUST-ABSENT' $SC/$name/py.out | sort) <(grep -v '^RUST-ABSENT' $SC/$name/ts.out | sort) | head -20; fi
echo "--- tree diff"; diff -r $SC/$name/py $SC/$name/ts > $SC/$name/tree.diff && echo "trees identical" || { echo "TREES DIFFER"; head -40 $SC/$name/tree.diff; }
echo "--- changed vs base (py)"; diff -rq $BASE $SC/$name/py | wc -l
head -c 600 $SC/$name/py.err; head -c 600 $SC/$name/ts.err
