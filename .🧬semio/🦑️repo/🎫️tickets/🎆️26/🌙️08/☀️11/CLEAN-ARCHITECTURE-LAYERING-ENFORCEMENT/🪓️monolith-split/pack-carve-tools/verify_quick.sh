#!/bin/zsh
# 🔎️ Re-runs the checks that need the final stage (no baseline refresh); output goes to the log given as $1.
cd "$(dirname "$0")"
{
echo "### union"; python3 union.py native native-nodeflate test wasm wasm-test wasip2 corruption corruption-test 2>&1 | /usr/bin/grep -v 'aborting\|cannot find type .ByteSpan\|^  -split'
echo "### unused"; python3 unused.py 2>&1 | head -20
echo "### orphan"; python3 orphan.py
echo "### seeds"; python3 seed.py 2>&1 | head -12
echo "### fmt"; python3 fmtcheck.py 2>&1 | tail -3
echo "### census"; python3 census_stage.py
echo "### lossless"; python3 lossless.py
echo "### done"
} > "$1" 2>&1
