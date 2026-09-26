#!/bin/zsh
# 🧹️ T12 session 12: the whole Python oracle sweep (every case's reference role at exhaustive), then an immediate copy
# of the run's own per-row results (the cache keeps only its latest report and ~90 result directories).
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-t12-captures
date +%s > "$OUT/oracle-python-start-2.txt"
SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false PYTHONDONTWRITEBYTECODE=1 nice -n 15 bun nx run @semio-tech/repo-test-domain:test-oracle --outputStyle=stream -- --implementation python > "$OUT/oracle-python-all-2.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$OUT/oracle-python-all-2.txt"
cp "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📤️results.jsonl" "$OUT/oracle-python-all-2-results.jsonl"
cp "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📊️summary.json" "$OUT/oracle-python-all-2-summary.json"
echo "ALL_DONE $(date '+%T')" >> "$OUT/oracle-python-all-2.txt"
