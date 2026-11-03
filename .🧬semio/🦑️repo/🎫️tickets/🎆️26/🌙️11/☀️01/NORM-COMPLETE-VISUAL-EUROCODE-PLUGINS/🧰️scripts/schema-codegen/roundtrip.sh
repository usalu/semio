#!/usr/bin/env bash
# Round-trip proof on a scratch copy of the norm tree (the sources are never touched): harvest the Rust annotations, generate every twin and struct into the copy,
# then judge the copy with the independent Python checker (shares no code with the generator), the strict twin gate, a TypeScript syntax and type pass and the import check.
# Usage: roundtrip.sh <scratch dir>
set -euo pipefail
scratch="${1:?scratch dir}"
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../../../../../../.." && pwd)"
norm="✏️s/🔌️plugins/📕️norm"
rm -rf "$scratch/ov" && mkdir -p "$scratch/ov/$norm"
nice -n 19 ionice -c3 rsync -a --exclude target --exclude node_modules --exclude 🗑️generated "$root/$norm/" "$scratch/ov/$norm/"
cd "$root"
nice -n 19 repo ./📜️script.ts schema twins annotate --under "$scratch/ov/$norm"
nice -n 19 repo ./📜️script.ts schema twins generate --under "$scratch/ov/$norm" | head -6
nice -n 19 repo ./📜️script.ts schema twins check --under "$scratch/ov/$norm" --strict | head -6
nice -n 19 python3 -I "$scratch/ov/$norm/🧪️tests/🧬️schema-sync/🐍️check.py" --root "$scratch/ov/$norm" --limit 40
nice -n 19 node "$here/syntaxcheck.cjs" "$scratch/ov/$norm"
nice -n 19 node --max-old-space-size=6000 "$here/typecheck.cjs" "$scratch/ov/$norm" ""
nice -n 19 python3 -I "$here/importcheck.py" "$scratch/ov/$norm" | head -5
