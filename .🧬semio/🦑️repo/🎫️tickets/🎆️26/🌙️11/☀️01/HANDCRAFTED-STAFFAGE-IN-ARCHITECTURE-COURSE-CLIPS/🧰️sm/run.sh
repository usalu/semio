#!/usr/bin/env bash
# run.sh <file rel to repo> [scenes] [skip]
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
G="$T/🗑️generated/sm"; mkdir -p "$G"
cd "$W"
SM_MEDIA="$G/media" PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=${SEMIO_TEACHING_LANGUAGE:-en} SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  timeout ${SM_TIMEOUT:-2400} uv run --project "$P" --python 3.12 python "$T/🧰️sm/worker.py" "$W/$1" "$G/results.jsonl" "$2" "$3" > "$G/last.log" 2>&1
echo "rc=$? $1"
tail -3 "$G/last.log"
