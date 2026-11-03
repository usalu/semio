#!/bin/bash
# probe.sh <clip file relative to repo> <Scene>: dry-run, logs cast time reached per motion to generated/fr/<Scene>.probe.json
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/fr/out" PYTHONUTF8=1
mkdir -p "$T/🗑️generated/fr"
export PROBE_OUT="$T/🗑️generated/fr/$2.probe.json"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/🧰️fr/probe.py" -ql --fps 15 --dry_run --disable_caching --media_dir "$T/🗑️generated/fr/media" "$1" "$2" > "$T/🗑️generated/fr/$2.log" 2>&1
echo "$2 exit $?"
tail -3 "$T/🗑️generated/fr/$2.log"
