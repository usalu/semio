#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/cargo-plan-tools/scan"; mkdir -p "$T"
P=( \( -name node_modules -o -name .git -o -name target -o -name .cache -o -name dist -o -name '.🧬semio' -o -name .nx -o -name bazel-out \) -prune -o -type f )
SINCE=${SINCE:-2026-10-19 00:00}; TODAY=${TODAY:-2026-10-20 00:00}
command find . "${P[@]}" -name Cargo.toml -newermt "$SINCE" -print | sed 's|^\./||' > "$T/man-1019.txt"
command find . "${P[@]}" -name Cargo.toml -newermt "$TODAY" -print | sed 's|^\./||' > "$T/man-1020.txt"
command find . "${P[@]}" -name '*.rs' -newermt "$SINCE" -print | sed 's|^\./||' > "$T/rs-1019.txt"
wc -l "$T"/*.txt
