#!/bin/sh
# replay.sh <tag> <closure.jsonl> <id> <key list file> [--extra file]: re-runs one corpus target under the read oracle and fails when it names an undeclared read that corpus.json does not list (a NEW undeclared read = a new wrong-hit class); after a fix lands, regenerate corpus.json with make-corpus.py.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"
nice bun "$T/target-run.mjs" "$@" || exit 2
python3 - "$G/reports/$1.json" "$T/corpus/corpus.json" "$3" <<'P'
import json, sys
report, corpus, target = sys.argv[1:4]
known = {p for row in json.load(open(corpus, encoding="utf8"))["targets"] if row.get("id") == target for v in row.get("expected", {}).values() for p in v}
new = [p for p in json.load(open(report, encoding="utf8"))["learned"] if p not in known]
print(json.dumps({"target": target, "new_undeclared": new}, ensure_ascii=False))
sys.exit(1 if new else 0)
P
