#!/bin/sh
# usage: sh rebuild.sh [MIRROR]   fresh mirror (round-5 closure batch), this set applied, ratchet allowlist re-cut from the result and applied
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
M=${1:-/private/tmp/pm-seam/m}
cd "$HERE"
nice -n 10 python3 build-plan.py
nice -n 10 python3 apply.py mirror "$M" --lock-sync | tail -1
nice -n 10 python3 apply.py apply --root "$M" --write --ignore-running --group seam --group bootstrap --group graph --group storage --group register | tail -1
nice -n 10 python3 mkratchet.py "$M" --write
nice -n 10 python3 build-plan.py
nice -n 10 python3 apply.py apply --root "$M" --write --ignore-running --group ratchet | tail -1
