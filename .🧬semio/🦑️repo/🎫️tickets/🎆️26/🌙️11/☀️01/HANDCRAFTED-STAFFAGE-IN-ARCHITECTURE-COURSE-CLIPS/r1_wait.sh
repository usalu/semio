#!/usr/bin/env bash
T="$(dirname "$0")"
G="$T/🗑️generated/r1"
until [ "$(tail -1 "$G/chain.log" | grep -c 'chain complete')" = 1 ]; do sleep 10; done
bash "$T/r1_chain.sh" "$T/r1_list3.txt"; bash "$T/r1_chain.sh" "$T/r1_list4.txt"
