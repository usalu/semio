#!/usr/bin/env python3
# 📊️ Summarize hard-coded prose hits: per-module counts and a deterministic top-80 user-facing selection.
# Usage: python3 summarize.py <hits.jsonl> <out-dir>
import json
import os
import re
import sys
from collections import Counter, defaultdict

CLI = re.compile(
    r"^(🎛️commands/|🏃️process/|🏗️build/⚙️operations/|🏗️build/🧭️invocation/|🏗️build/📜️script\.ts|🏗️build/\.profiled-script\.ts|"
    r"🏗️build/📊️reporting/|🏗️build/🧾️ci-report/|🏗️build/🌡️host-capacity/|🏗️build/🔒️leases/|🏗️build/🧹️cache-gc/|"
    r"🏗️build/📏️incremental/|🏗️build/🪞️parity/|🏗️build/🔧️tools/|🏗️authoring/🎮️|🏗️build/🛠️cmake/|⚡️caching/🚀️bootstrap/|"
    r"🎮️playground/|🚨️policy/|🧼️workspace-cleanup/🎮️|🧹️normalization/🎮️|🤖️agent-clients/🎮️|.*⌨️command/|.*📜️script\.ts|"
    r".*🏃️runner/|.*🚪️launcher/|⚖️laws/)"
)
DEV_ONLY = re.compile(r"^⚖️laws/")


def main():
    hits_path, out_dir = sys.argv[1], sys.argv[2]
    os.makedirs(out_dir, exist_ok=True)
    rows = [json.loads(line) for line in open(hits_path, encoding="utf-8")]
    per_module = defaultdict(Counter)
    for row in rows:
        top = row["file"].split("/")[0]
        row["cli"] = bool(CLI.match(row["file"]))
        bucket = "console" if row["kind"] in ("console", "stream") else row["kind"]
        per_module[top][bucket] += 1
        per_module[top]["cli-" + bucket] += int(row["cli"])
    with open(os.path.join(out_dir, "per-module.tsv"), "w", encoding="utf-8") as fh:
        fh.write("module\tconsole\tcli-console\terror\tcli-error\tprop\ttotal\n")
        for module, c in sorted(per_module.items(), key=lambda kv: -sum(kv[1].values())):
            total = c["console"] + c["error"] + c["prop"]
            fh.write(f"{module}\t{c['console']}\t{c['cli-console']}\t{c['error']}\t{c['cli-error']}\t{c['prop']}\t{total}\n")
    console = [r for r in rows if r["kind"] in ("console", "stream") and r["cli"] and not DEV_ONLY.match(r["file"])]
    errors = [r for r in rows if r["kind"] == "error" and r["cli"] and not DEV_ONLY.match(r["file"])
              and re.match(r"^(🎛️commands/|🏃️process/|🏗️build/🧭️invocation/|🏗️build/📜️script\.ts|🏗️build/📏️incremental/|🏗️build/🪞️parity/|🎮️playground/)", r["file"])]
    console.sort(key=lambda r: (r["file"], r["line"]))
    errors.sort(key=lambda r: (r["file"], r["line"]))
    # deterministic spread: every k-th row so that 80 rows cover many modules
    def spread(items, n):
        if len(items) <= n:
            return items
        step = len(items) / n
        return [items[int(i * step)] for i in range(n)]
    selection = spread(console, 48) + spread(errors, 32)
    with open(os.path.join(out_dir, "top80.tsv"), "w", encoding="utf-8") as fh:
        for index, row in enumerate(selection, 1):
            fh.write(f"{index}\t{row['file']}\t{row['line']}\t{row['kind']}\t{row['text'][:110]}\n")
    print("console-cli", len(console), "error-cli-command", len(errors), "selected", len(selection))


if __name__ == "__main__":
    main()
