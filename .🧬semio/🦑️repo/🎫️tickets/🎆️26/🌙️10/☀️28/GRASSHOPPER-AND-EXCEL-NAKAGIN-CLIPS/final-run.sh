cd /c/git/semio
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
O="$T/🗑️generated/final"
PY="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 TMP="$O/tmp" TEMP="$O/tmp"
export SEMIO_REPO_BINARY="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
A="🎓️teaching/🏛️architecture"
G="$A/🦗️grasshopper/🎬️clip"; X="$A/📊️excel/🎬️clip"
run(){ n=$1; shift; s=$(date +%s); timeout 6000 "$@" > "$O/$n.log" 2>&1; echo $? > "$O/$n.exit"; echo $(( $(date +%s)-s )) > "$O/$n.dur"; }
run nakagin-py "$PY" "$A/🏢️nakagin/🧪️tests/🐍️.py"
run nakagin-cjs node "$A/🏢️nakagin/🧪️tests/🟨️.cjs"
run g-py "$PY" "$G/🧪️tests/🐍️.py"
run g-cjs node "$G/🧪️tests/🟨️.cjs"
run g-registry "$PY" "$G/🧪️tests/📇️registry/🐍️.py"
run x-py "$PY" "$X/🧪️tests/🐍️.py"
run x-cjs node "$X/🧪️tests/🟨️.cjs"
run x-registry "$PY" "$X/🧪️tests/📇️registry/🐍️.py"
run morph "$PY" "$A/🔀️morph/🧪️tests/🐍️.py"
echo done > "$O/fast.done"
(run g-presentation "$PY" "$G/🧪️tests/🎭️presentation/🐍️.py" -v) &
(run x-presentation "$PY" "$X/🧪️tests/🎭️presentation/🐍️.py" -v) &
wait
echo done > "$O/all.done"
