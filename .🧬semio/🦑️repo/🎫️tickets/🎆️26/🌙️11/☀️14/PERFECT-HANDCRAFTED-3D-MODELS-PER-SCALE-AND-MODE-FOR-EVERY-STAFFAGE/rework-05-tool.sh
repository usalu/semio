#!/bin/sh
# 🧷️ Runs a copy of a round 3 tool (sheet, project2d) on a candidate asset document instead of the catalog asset: rework-05-tool.sh <sheet|project2d> <candidate 🔣️.json> <tool arguments>
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
tool=$1; file=$2; shift 2
python3 - "$tool" <<'P'
import sys
t = sys.argv[1]
s = open(f"r3-{t}.ts", encoding="utf8").read()
s = s.replace("const { entry, asset } = load(spec, USAGE);", "const { entry, asset } = loadFile(spec, USAGE);").replace('readFileSync(entry.path, "utf8")', 'readFileSync(process.env.R05_FILE!, "utf8")')
i = s.index("\nimport ")
open(f"rework-05-x-{t}.ts", "w", encoding="utf8").write(s[:i] + '\nimport { loadFile } from "./rework-05-load.ts";' + s[i:])
P
R05_FILE="$file" "$BUN" "rework-05-x-$tool.ts" "$@"
