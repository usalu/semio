#!/usr/bin/env bash
f="$1"; n="${2:-22}"
for i in $(seq 1 "$n"); do grep -qE "^running [0-9]+ test|^test result|^WATCHDOG|has overflowed its stack|^error: (test failed|could not compile)" "$f" && break; sleep 5; done
grep -nE "^running [0-9]+ test|^test |^test result|^WATCHDOG|has overflowed its stack|^error: (test failed|could not compile)" "$f" | cut -c1-250 | tail -30
