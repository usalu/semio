#!/bin/sh
d=$(dirname "$0")
while pgrep -f "native-cargo.sh" >/dev/null 2>&1 && pgrep -f "cargo-bin\|bin/cargo " >/dev/null 2>&1; do sleep 20; done
sh "$d/native-cargo.sh" test -p "$1" -- pointer_down page_window_declares
echo "exit=$?"
