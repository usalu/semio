#!/usr/bin/env bash
W=/c/git/semio
cd "$W" && PYTHONUTF8=1 PYTHONPATH="$W" SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/tests SEMIO_TEST_MANIM=1 SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m unittest "$@"
