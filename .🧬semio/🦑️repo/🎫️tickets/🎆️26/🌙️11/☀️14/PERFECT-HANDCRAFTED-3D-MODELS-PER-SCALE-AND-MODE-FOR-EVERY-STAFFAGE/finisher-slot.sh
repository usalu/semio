#!/bin/sh
# 🚦 Takes the single tool slot from `finisher-sweep.sh` (`take`: pause it and wait until its running validation ended) or gives it back (`give`).
cd "$(dirname "$0")"
if [ "$1" = take ]; then touch 🗑️generated/finisher-sweep.pause; while [ -f 🗑️generated/finisher-sweep.busy ]; do sleep 2; done; else rm -f 🗑️generated/finisher-sweep.pause; fi
