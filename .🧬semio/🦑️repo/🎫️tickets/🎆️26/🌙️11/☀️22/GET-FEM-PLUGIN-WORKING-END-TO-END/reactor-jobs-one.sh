#!/usr/bin/env bash
EXE=$(ls -t /c/t/rj/debug/deps/semio_plugin_reactor_jobs-*.exe | head -1)
export RUST_MIN_STACK=134217728 RUST_BACKTRACE=0
timeout 300 "$EXE" --exact "$1" --nocapture 2>&1 | grep -v "^$" | cut -c1-900 | head -${2:-25}
