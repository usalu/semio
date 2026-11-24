#!/bin/sh
cd "$(dirname "$0")"
mkdir -p 🗑️generated
log="${1:-shard-run}"; shift
sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture > "🗑️generated/$log.log" 2>&1
