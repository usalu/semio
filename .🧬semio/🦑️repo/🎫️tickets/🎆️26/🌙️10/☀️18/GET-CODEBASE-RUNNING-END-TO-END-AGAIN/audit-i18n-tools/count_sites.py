#!/usr/bin/env python3
# 🔢️ Read-only census of Error / console / stream sites in the repo build library by first-argument shape.
# Usage: python3 count_sites.py <file-list> <repo-root> <out.tsv>
import re
import sys
from collections import Counter, defaultdict

LIB_PREFIX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SITE = re.compile(r"(?:\bError|\bconsole\.(?:log|error|warn|info|debug|table|group)|\b(?:stdout|stderr)\.write)\(\s*(.)?")
CLI_CHANNEL = re.compile(
    r"^(🎛️commands/|🏃️process/|🏗️build/⚙️operations/|🏗️build/🧭️invocation/|🏗️build/📜️script\.ts|🏗️build/📊️reporting/|"
    r"🏗️build/🧾️ci-report/|🏗️build/🌡️host-capacity/|🏗️build/🔒️leases/|🏗️build/🧹️cache-gc/|🏗️build/📏️incremental/|"
    r"🏗️build/🪞️parity/|🏗️build/🔧️tools/|⚖️laws/|🎮️|🏗️authoring/🎮️|⚡️caching/🚀️bootstrap/|🚪️launcher|🏃️runner|⌨️command|📜️script\.ts|🏭️generator)"
)
LOCALIZED_CALL = re.compile(r"(both|cacheTextV1|concurrentTextV1|text|label|localized|message|renderMessage|laneMessageV1|t|fill|msg)\(")


def classify(first):
    if first is None or first == "":
        return "no-argument"
    if first in "\"'`":
        return "literal"
    if first.isalpha() or first == "_":
        return "identifier-or-call"
    return "other"


def main():
    files_list, root, out = sys.argv[1], sys.argv[2], sys.argv[3]
    paths = [line.strip() for line in open(files_list, encoding="utf-8") if line.strip()]
    table = defaultdict(Counter)
    totals = Counter()
    with open(out, "w", encoding="utf-8") as fh:
        fh.write("module\tkind\tshape\tcli_channel\tcount\n")
        for rel in paths:
            if not rel.endswith((".ts", ".mts", ".mjs", ".cjs")):
                continue
            try:
                text = open(root + "/" + rel, encoding="utf-8").read()
            except (OSError, UnicodeDecodeError):
                continue
            short = rel[len(LIB_PREFIX):] if rel.startswith(LIB_PREFIX) else rel
            module = "/".join(short.split("/")[:2]) if "/" in short else short
            cli = bool(CLI_CHANNEL.match(short))
            for match in SITE.finditer(text):
                head = match.group(0)
                kind = "error" if head.startswith(("Error", "\bError")) or "Error(" in head else ("stream" if "write" in head else "console")
                shape = classify(match.group(1))
                totals[(kind, shape, cli)] += 1
                table[module][(kind, shape)] += 1
                fh.write(f"{module}\t{kind}\t{shape}\t{int(cli)}\t1\n")
    print("kind shape cli count")
    for key, value in sorted(totals.items()):
        print(*key, value)
    print("TOP MODULES (error literal/other, console/stream)")
    rows = []
    for module, counter in table.items():
        lit = counter[("error", "literal")]
        con = counter[("console", "literal")] + counter[("stream", "literal")]
        rows.append((lit + con, module, lit, con, sum(v for k, v in counter.items())))
    for row in sorted(rows, reverse=True)[:60]:
        print(row[1], "error-literal", row[2], "console-literal", row[3], "all", row[4])


if __name__ == "__main__":
    main()
