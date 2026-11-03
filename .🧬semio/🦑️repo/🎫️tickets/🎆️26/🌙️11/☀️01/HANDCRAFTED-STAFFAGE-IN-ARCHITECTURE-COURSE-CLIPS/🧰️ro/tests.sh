#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
O="$W/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"
PYTHONPATH="$W" PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  "$CACHE/python/Scripts/python.exe" "$T/run-g3-tests.py" "$O/🧪️tests/🐍️.py" "$O/🧬️algorithms/🧪️tests/📈️comparison/🐍️.py" "$O/🧬️algorithms/🧪️tests/🧩️widgets/🐍️.py" "$O/🧪️experiments/🧪️tests/🧊️consumed/🐍️.py" > "$T/🗑️generated/ro/tests.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/ro/tests.log"
