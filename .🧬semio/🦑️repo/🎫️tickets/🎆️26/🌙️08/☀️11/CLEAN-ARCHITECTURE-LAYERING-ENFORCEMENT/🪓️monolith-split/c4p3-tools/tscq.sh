#!/bin/zsh
# usage: tscq.sh <label> <scope>...   -> G/c4p3-<label>.diag.tsv and a diff against the baseline
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/env.sh
label="$1"; shift
args=()
for s in "$@"; do args+=(--scope "$s"); done
cd "$R"
bun ./📜️script.ts verify typescript check "${args[@]}" --out "$G/c4p3-$label.tsconfig.json" --max-diagnostics 1000000 > "$G/c4p3-tsc-$label.run.log" 2>&1
python3 - "$G/c4p3-$label.tsconfig.tsc.txt" "$G/c4p3-$label.diag.tsv" <<'PY'
import re,sys
t=open(sys.argv[1],encoding="utf8").read()
rows=[]
for l in t.split("\n"):
    m=re.match(r"^(\S.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",l)
    if m: rows.append((m.group(1),m.group(4),m.group(5)[:120]))
open(sys.argv[2],"w",encoding="utf8").write("\n".join("\t".join(r) for r in sorted(rows))+"\n")
print(len(rows))
PY
python3 - "$G/c4p3-before.diag.tsv" "$G/c4p3-$label.diag.tsv" "$G/c4p3-$label.diag.diff" "$@" <<'PY'
import sys, collections
before, after, out, *scopes = sys.argv[1:]
def load(p):
    c = collections.Counter()
    for l in open(p, encoding="utf8"):
        f = l.rstrip("\n").split("\t")
        if len(f) >= 2 and any(f[0].startswith(s) for s in scopes): c[(f[0], f[1], f[2] if len(f) > 2 else "")] += 1
    return c
b, a = load(before), load(after)
new = a - b; gone = b - a
with open(out, "w", encoding="utf8") as fh:
    for k, v in sorted(new.items()): fh.write(f"NEW  {v} {k[0]} {k[1]} {k[2]}\n")
    for k, v in sorted(gone.items()): fh.write(f"GONE {v} {k[0]} {k[1]} {k[2]}\n")
print(f"scope rows: before={sum(b.values())} after={sum(a.values())} new={sum(new.values())} gone={sum(gone.values())}")
PY
