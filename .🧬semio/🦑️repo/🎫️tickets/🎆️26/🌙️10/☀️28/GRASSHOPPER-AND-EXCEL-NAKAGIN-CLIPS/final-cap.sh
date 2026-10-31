cd /c/git/semio
T="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
O="$T/🗑️generated/final"
PY="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8
export SEMIO_REPO_BINARY="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/repo-shim.cmd"
lang=$1
while read s t c; do
  TMP="$O/tmp-$lang" TEMP="$O/tmp-$lang" "$PY" "$T/capture.py" "$s" "$t" "$c" $lang > "$O/cap-$c-$lang.log" 2>&1; echo $? > "$O/cap-$c-$lang.exit"
done <<'L'
🦗️grasshopper 🦴️anatomy Beat1_Anatomy
🦗️grasshopper 💊️capsule Beat2_Capsule
🦗️grasshopper 🗼️tower Beat3_Tower
🦗️grasshopper 🫗️casting Beat4_Casting
📊️excel 🦴️anatomy Beat1_Anatomy
📊️excel 📍️references Beat2_References
📊️excel 📋️program Beat3_Program
📊️excel ⚡️energy Beat4_Energy
L
echo done > "$O/cap-$lang.done"
