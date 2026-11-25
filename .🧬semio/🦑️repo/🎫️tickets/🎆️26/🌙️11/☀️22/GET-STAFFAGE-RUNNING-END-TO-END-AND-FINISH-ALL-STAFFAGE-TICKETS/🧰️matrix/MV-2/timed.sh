#!/bin/zsh
# ⏱️ Runs a command under `/usr/bin/time -l` (its peak memory goes to stderr): timed.sh <command> <args…>; a wrapper because `repo capacity run` reads a bare `-l` as its own option.
exec /usr/bin/time -l "$@"
