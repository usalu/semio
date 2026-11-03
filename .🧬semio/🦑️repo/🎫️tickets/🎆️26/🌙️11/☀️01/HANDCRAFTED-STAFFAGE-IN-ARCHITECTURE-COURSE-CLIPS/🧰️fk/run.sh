#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$W" \
  SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/tests SEMIO_TEST_MANIM=1 \
  SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/🧰️fk/${FK_SCRIPT:-loop.py}" "$@"
