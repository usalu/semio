#!/bin/zsh
# usage: btest.sh <label> <bun test args...> -> G/c4p3-<label>.log, .xml and .set (status<TAB>suite<TAB>title, sorted)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p3-tools/env.sh
label="$1"; shift
cd "$R"
perl -e 'alarm 900; exec @ARGV' -- bun test --reporter=junit --reporter-outfile="$G/c4p3-$label.xml" "$@" > "$G/c4p3-$label.log" 2>&1
echo "exit=$?" >> "$G/c4p3-$label.log"
python3 - "$G/c4p3-$label.xml" "$G/c4p3-$label.set" <<'PY'
import sys, xml.etree.ElementTree as ET
rows = []
try:
    root = ET.parse(sys.argv[1]).getroot()
    def walk(n, path):
        for c in n:
            if c.tag == "testsuite": walk(c, path + [c.get("name", "")])
            elif c.tag == "testcase":
                st = "fail" if c.find("failure") is not None or c.find("error") is not None else ("skip" if c.find("skipped") is not None else "pass")
                rows.append(f"{st}\t{c.get('classname','')}\t{c.get('name','')}")
    walk(root, [])
except Exception as e:
    rows.append(f"error\t\t{e}")
open(sys.argv[2], "w", encoding="utf8").write("\n".join(sorted(rows)) + "\n")
c = {k: sum(1 for r in rows if r.startswith(k + "\t")) for k in ("pass", "fail", "skip", "error")}
print(sys.argv[2].split("/")[-1], c, "total", len(rows))
PY
